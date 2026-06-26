#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::{
    Win32::{
        Foundation::{ERROR_ALREADY_EXISTS, GetLastError, HANDLE},
        System::Threading::CreateMutexW,
        UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, HHOOK, MB_ICONERROR, MB_OK, MSG, MessageBoxW,
            SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL,
        },
    },
    core::{PCWSTR, w},
};

use crate::{
    hoko::hoko_proc,
    pleto::{krei_pleton, pritrakti_tray_eventojn},
};

mod detekti;
mod hoko;
mod pleto;

static mut HOKO: Option<HHOOK> = None;

// Difini statikan variablon por teni la pritraktilon de la Mutex, por malebligi ĝian fruan liberigon dum la rulado de main.
static mut MUTEX_PRITRAKTILO: Option<HANDLE> = None;

fn main() {
    // Certigi, ke nur unu instanco ruliĝu.
    if !certigi_unikan_instancon() {
        // // Se jam ekzistas ruliĝanta instanco, montri averton kaj tuj eliri la programon.
        unsafe {
            MessageBoxW(
                None,
                w!("La programo jam rulas!"),
                w!("Averto"),
                MB_OK | MB_ICONERROR,
            );
        }
        return;
    }

    let _tray = krei_pleton();

    instali_hokon();

    unsafe {
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            pritrakti_tray_eventojn();

            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        //malinstali la hokon post la mesaĝa buklo.
        malinstali_hokon();

        // Liberigi la Mutex‑pritraktilon.
        liberigi_mutekson();
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

// Kontroli kaj certigi, ke nur unu programinstanco ruliĝas nun.
// Se ĝi estas la sola instanco, redoni true; se jam ekzistas alia ruliĝanta instanco, redoni false.
fn certigi_unikan_instancon() -> bool {
    unsafe {
        // Uzi unikan nomon por identigi vian aplikaĵon.
        let mutex_nomo = "Global\\{019f038e-90fe-729b-aaaf-9bb72e9a61f5}_UnikaKlavaraHokoApoMutex\0";

        // Konverti la ĉenon al larĝsigna tabelo (UTF‑16) por uzo en Win32 API.
        let mutex_nomo_u16: Vec<u16> = mutex_nomo.encode_utf16().collect();
        let pcwstr_nomo = PCWSTR::from_raw(mutex_nomo_u16.as_ptr());

        // Krei nomitan mutexon.
        let trakti_rezulton = CreateMutexW(None, true, pcwstr_nomo);

        match trakti_rezulton {
            Ok(pritraktilo) => {
                // Se la kreado sukcesis, ni devas kontroli ĉu la mutexo jam ekzistis antaŭ la voko.
                if GetLastError() == ERROR_ALREADY_EXISTS {
                    // La mutexo jam ekzistas, kio signifas ke alia instanco jam ruliĝas.
                    return false;
                }
                // Konservi la pritraktilon tiel, ke ĝi restu valida dum la tuta programrulado.
                MUTEX_PRITRAKTILO = Some(pritraktilo);
                true
            }
            Err(_) => {
                // Se la kreado malsukcesas pro iu kialo, por sekureco oni kutime permesas la programon plu ruliĝi, kvankam eblas ankaŭ elekti eliron.
                true
            }
        }
    }
}

// Liberigi la muteksan pritraktilon.
unsafe fn liberigi_mutekson() {
    unsafe {
        if let Some(handle) = MUTEX_PRITRAKTILO {
            use windows::Win32::Foundation::CloseHandle;
            let _ = CloseHandle(handle);
            MUTEX_PRITRAKTILO = None;
        }
    }
}
