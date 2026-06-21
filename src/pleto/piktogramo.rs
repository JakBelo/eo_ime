use windows::Win32::{
    Foundation::HWND,
    UI::{
        Shell::{
            NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW, Shell_NotifyIconW,
        },
        WindowsAndMessaging::{IDI_APPLICATION, LoadIconW, WM_USER},
    },
};

const WM_SISTEMPLETA_IKONO: u32 = WM_USER + 1;

pub unsafe fn aldoni_sistempletan_ikonon(hwnd: HWND) {
    let ikono = unsafe { LoadIconW(None, IDI_APPLICATION).unwrap_or_default() };

    let mut spd = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
        uCallbackMessage: WM_SISTEMPLETA_IKONO,
        hIcon: ikono,
        ..Default::default()
    };

    // agorda teksto "EO".
    let indiko: Vec<u16> = "EO\0".encode_utf16().collect();
    spd.szTip[..indiko.len()].copy_from_slice(&indiko);

    unsafe {
        let _ = Shell_NotifyIconW(NIM_ADD, &spd);
    }
}

pub unsafe fn forigi_sistempletan_ikonon(hwnd: HWND) {
    let spd = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        ..Default::default()
    };

    unsafe {
        let _ = Shell_NotifyIconW(NIM_DELETE, &spd);
    }
}
