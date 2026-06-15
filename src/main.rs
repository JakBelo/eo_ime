use windows::Win32::{
    UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, HHOOK, MSG, SetWindowsHookExW, TranslateMessage,
        WH_KEYBOARD_LL,
    },
};

use crate::hoko::hoko_proc;

mod detekti;
mod esperanto;
mod hoko;

static mut HOKO: Option<HHOOK> = None;

fn main() {
    instali_hokon();

    unsafe {
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

pub fn instali_hokon() {
    unsafe {
        let hoko = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hoko_proc), None, 0)
            .expect("Malsukcesis instali la hokan.");

        HOKO = Some(hoko);
    }
}
