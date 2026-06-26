use std::{
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM}, UI::{
        Input::KeyboardAndMouse::{
            GetKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY, VK_CAPITAL, VK_LSHIFT, VK_RSHIFT,
        }, WindowsAndMessaging::{CallNextHookEx, HC_ACTION, KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_KEYUP},
    },
};

use crate::detekti::estas_esperanta;

enum Ago {
    Pasi,
    Konsumi,
    Ignori,
}

static LASTO: Mutex<Option<(char, bool, Instant)>> = Mutex::new(None);

static SHIFT_MALSUPREN: AtomicBool = AtomicBool::new(false);

pub unsafe extern "system" fn hoko_proc(n_kodo: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        // Trakti klavojn nur en Esperanta reĝimo.
        if n_kodo != HC_ACTION as i32 {
            return CallNextHookEx(None, n_kodo, w_param, l_param);
        }

        if estas_esperanta() {
            const KLAVO_MALSUPREN: usize = WM_KEYDOWN as usize;
            const KLAVO_SUPREN: usize = WM_KEYUP as usize;

            // Klavara malalt-nivela hoka strukturo.
            let kmn = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            // VK‑kodo.
            let vk = kmn.vkCode as u32;

            // Filtri for 231.
            if vk == 231 {
                return CallNextHookEx(None, n_kodo, w_param, l_param);
            }

            match w_param.0 {
                KLAVO_MALSUPREN => {
                    // Ĝisdatigi_Shift-staton.
                    if vk == VK_LSHIFT.0 as u32 || vk == VK_RSHIFT.0 as u32 {
                        SHIFT_MALSUPREN.store(true, Ordering::Relaxed);
                        return CallNextHookEx(None, n_kodo, w_param, l_param);
                    }

                    // Trakti klavojn nur en Esperanta reĝimo.
                    if estas_esperanta() {
                        // Kontroli Esperantan prefikson sen malhelpi.
                        if let Some(s) = vk_al_signo(vk) {
                            match trakti_klavon(s) {
                                Ago::Pasi => {
                                    return CallNextHookEx(None, n_kodo, w_param, l_param);
                                }
                                Ago::Konsumi => return LRESULT(1),
                                Ago::Ignori => {}
                            }
                        }
                    }
                }
                KLAVO_SUPREN => {
                    // Ĝisdatigi_Shift-staton.
                    if vk == VK_LSHIFT.0 as u32 || vk == VK_RSHIFT.0 as u32 {
                        SHIFT_MALSUPREN.store(false, Ordering::Relaxed);
                        return CallNextHookEx(None, n_kodo, w_param, l_param);
                    }
                }
                _ => {}
            }
        } else {
            // Nur kiam la stato estas vera, agordi ĝin al falsa.
            if SHIFT_MALSUPREN.load(Ordering::Relaxed) {
                SHIFT_MALSUPREN.store(false, Ordering::Relaxed);
            }
            // Se en alia lingvo lasto enhavas datumon, tiam forigu ĝin.
            if let Ok(mut lasto) = LASTO.lock() {
                *lasto = None;
            }
        }

        CallNextHookEx(None, n_kodo, w_param, l_param)
    }
}

fn vk_al_signo(vk: u32) -> Option<char> {
    let s: char = match vk {
        0x41..=0x5A => (vk as u8 as char).to_ascii_lowercase(), // A-Z
        _ => return None,
    };
    Some(s)
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

fn trakti_klavon(s: char) -> Ago {
    // Akiri la antaŭe konservitan signon.
    let mut lasto = LASTO.lock().unwrap();
    // Post ĉiu klavopremo, forigi lasto post 3 sekundoj da neaktiveco.
    if let Some((_, _, tempo)) = *lasto {
        if tempo.elapsed().as_secs() >= 3 {
            *lasto = None;
        }
    }

    match s {
        // Registri la prefiksan literon.
        'c' | 'g' | 'h' | 'j' | 's' | 'u' => {
            // Legi_staton.
            let shift = SHIFT_MALSUPREN.load(Ordering::Relaxed);
            // Realteme kontroli la staton de CapsLock.
            let caps = unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 0x0001 != 0 };

            let ĉu_majuskle = shift ^ caps;

            *lasto = Some((s, ĉu_majuskle, Instant::now()));
            Ago::Pasi
        }

        // Nur ĉe “x” okazas anstataŭigo.
        'x' => {
            if let Some((prefikso, ĉu_majuskle, _tempo)) = *lasto {
                let eligo = match prefikso {
                    'c' => {
                        if ĉu_majuskle {
                            'Ĉ'
                        } else {
                            'ĉ'
                        }
                    }
                    'g' => {
                        if ĉu_majuskle {
                            'Ĝ'
                        } else {
                            'ĝ'
                        }
                    }
                    'h' => {
                        if ĉu_majuskle {
                            'Ĥ'
                        } else {
                            'ĥ'
                        }
                    }
                    'j' => {
                        if ĉu_majuskle {
                            'Ĵ'
                        } else {
                            'ĵ'
                        }
                    }
                    's' => {
                        if ĉu_majuskle {
                            'Ŝ'
                        } else {
                            'ŝ'
                        }
                    }
                    'u' => {
                        if ĉu_majuskle {
                            'Ŭ'
                        } else {
                            'ŭ'
                        }
                    }
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
