use std::sync::Mutex;

use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{
            GetKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT,
            KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY, VK_CAPITAL, VK_SHIFT,
        },
        WindowsAndMessaging::{CallNextHookEx, HC_ACTION, KBDLLHOOKSTRUCT, WM_KEYDOWN},
    },
};

use crate::detekti::estas_esperanta;

enum Ago {
    Pasi,
    Konsumi,
    Ignori,
}

static LASTO: Mutex<Option<char>> = Mutex::new(None);

pub unsafe extern "system" fn hoko_proc(n_kodo: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        if n_kodo == HC_ACTION as i32 && w_param.0 == WM_KEYDOWN as usize {
            // Klavara malalt-nivela hoka strukturo.
            let kmn = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            // VK‑kodo.
            let vk = kmn.vkCode as u32;

            // Filtri for 231.
            if vk == 231 {
                return CallNextHookEx(None, n_kodo, w_param, l_param);
            }

            // trakti klavojn nur en Esperanta reĝimo.
            if estas_esperanta() {
                // Kontroli Esperantan prefikson sen malhelpi.
                if let Some((s, ĉu_majuskle)) = vk_al_signo(vk) {
                    match trakti_klavon(s, ĉu_majuskle) {
                        Ago::Pasi => return CallNextHookEx(None, n_kodo, w_param, l_param),
                        Ago::Konsumi => return LRESULT(1),
                        Ago::Ignori => {},
                    }
                }
            }
        }

        CallNextHookEx(None, n_kodo, w_param, l_param)
    }
}

fn vk_al_signo(vk: u32) -> Option<(char, bool)> {
    let shift = unsafe { GetKeyState(VK_SHIFT.0 as i32) } < 0;
    let caps = unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 1 } != 0;

    let ĉu_majuskle = shift ^ caps;

    let s: char = match vk {
        0x41..=0x5A => (vk as u8 as char).to_ascii_lowercase(), // A-Z
        _ => return None,
    };
    Some((s, ĉu_majuskle))
}

pub fn sendi_signon(s: char) {
    let mut enigo = INPUT {
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
        SendInput(&[enigo], std::mem::size_of::<INPUT>() as i32);

        enigo.Anonymous.ki.dwFlags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
        SendInput(&[enigo], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn sendi_retropaŝon() {
    let mut enigo = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0x08), // VK_RETROPAŜO.
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[enigo], std::mem::size_of::<INPUT>() as i32);

        // klavo supren.
        enigo.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
        SendInput(&[enigo], std::mem::size_of::<INPUT>() as i32);
    }
}

fn trakti_klavon(s: char, ĉu_majuskle: bool) -> Ago {
    // Akiri la antaŭe konservitan signon.
    let mut lasto = LASTO.lock().unwrap();

    match s {
        // Registri la prefiksan literon.
        'c' | 'g' | 'h' | 'j' | 's' | 'u' => {
            *lasto = Some(s);
            Ago::Pasi
        }

        // Nur ĉe “x” okazas anstataŭigo.
        'x' => {
            if let Some(prefikso) = *lasto {
                let eligo = match prefikso {
                    'c' => if ĉu_majuskle { 'Ĉ' } else { 'ĉ' },
                    'g' => if ĉu_majuskle { 'Ĝ' } else { 'ĝ' },
                    'h' => if ĉu_majuskle { 'Ĥ' } else { 'ĥ' },
                    'j' => if ĉu_majuskle { 'Ĵ' } else { 'ĵ' },
                    's' => if ĉu_majuskle { 'Ŝ' } else { 'ŝ' },
                    'u' => if ĉu_majuskle { 'Ŭ' } else { 'ŭ' },
                    _ => return Ago::Ignori,
                };

                sendi_retropaŝon();
                sendi_signon(eligo);
                *lasto = None;
                return Ago::Konsumi;
            }

            *lasto = None;
            Ago::Pasi
        }

        // Aliaj signoj: forigi staton kaj pasi normale.
        _ => {
            *lasto = None;
            Ago::Pasi
        }
    }
}
