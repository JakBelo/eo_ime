use std::sync::Mutex;

use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{
            INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
            SendInput, VIRTUAL_KEY,
        },
        WindowsAndMessaging::{
            CallNextHookEx, HC_ACTION, KBDLLHOOKSTRUCT, PostQuitMessage, WM_KEYDOWN,
        },
    },
};

use crate::{detekti::estas_esperanta, esperanto::anstatauxigi};

static STATO: Mutex<String> = Mutex::new(String::new());

fn prilabori_enigon(c: char) -> Option<char> {
    let mut stato = STATO.lock().unwrap();

    if estas_esperanta() {
        anstatauxigi(c, &mut stato)
    } else {
        stato.clear();
        Some(c)
    }
}

pub unsafe extern "system" fn hoko_proc(n_kodo: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        if n_kodo == HC_ACTION as i32 && w_param.0 == WM_KEYDOWN as usize {
            // Klavara malalt-nivela hoka strukturo.
            let kmn = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            // VK‑kodo.
            let vk = kmn.vkCode as u32;

            if vk == 0x1B {
                PostQuitMessage(0);
            }
            // Filtri for 231.
            if vk == 231 {
                return CallNextHookEx(None, n_kodo, w_param, l_param);
            }

            // trakti klavojn nur en Esperanta reĝimo.
            if !estas_esperanta() {
                // Ne‑Esperanta reĝimo: lasi la klavojn normale pasi al la sistemo.
                return CallNextHookEx(None, n_kodo, w_param, l_param);
            }

            // Konverti al signo (nur literojn).
            if let Some(ch) = vk_al_signo(vk) {
                match prilabori_enigon(ch) {
                    Some(out) => {
                        sendi_signon(out);
                        return LRESULT(1); // Interkapti la originalan klavopremon.
                    }
                    None => {
                        return LRESULT(1);
                    }
                }
            }
        }

        CallNextHookEx(None, n_kodo, w_param, l_param)
    }
}

fn vk_al_signo(vk: u32) -> Option<char> {
    let s = match vk {
        0x41..=0x5A => (vk as u8 as char).to_ascii_lowercase(), // A-Z
        _ => return None,
    };
    Some(s)
}

pub fn sendi_signon(s: char) {
    let mut input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: s as u16,
                dwFlags: KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);

        input.Anonymous.ki.dwFlags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
