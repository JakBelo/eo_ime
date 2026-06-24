use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, HHOOK, MSG, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL
};

use crate::{hoko::hoko_proc, pleto::krei_pleton};

mod detekti;
mod esperanto;
mod hoko;
mod pleto;

static mut HOKO: Option<HHOOK> = None;

fn main() {
    let _tray = krei_pleton();

    instali_hokon();

    unsafe {
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        //malinstali la hokon post la mesaĝa buklo.
        malinstali_hokon();
    }
}

pub fn instali_hokon() {
    unsafe {
        let hoko = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hoko_proc), None, 0)
            .expect("Malsukcesis instali la hokan.");

        HOKO = Some(hoko);
    }
}

pub fn malinstali_hokon() {
    unsafe {
        if let Some(hoko) = HOKO {
            let _ = UnhookWindowsHookEx(hoko);
            HOKO = None;
        }
    }
}

pub fn pritrakti_tray_eventojn() {
    
}
