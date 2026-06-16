use windows::Win32::UI::{
    Input::KeyboardAndMouse::{GetKeyState, GetKeyboardLayout, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN},
    WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
};

pub fn estas_esperanta() -> bool {
    unsafe {
        // Akiri la antaŭan fenestron.
        let fg_fenestro = GetForegroundWindow();

        // Akiri la fadenan identigilon de la antaŭa fenestro.
        let fadena_id = GetWindowThreadProcessId(fg_fenestro, None);

        // Klavara pritraktilo.
        let kp = GetKeyboardLayout(fadena_id);
        let lin_id = kp.0 as isize;
        lin_id == 67706880 // Esperanta lingvo-identigilo.
    }
}

// Kontroli ĉu modifa klavo estas premita.
pub fn havas_modifilon() -> bool {
    unsafe {
        let ctrl = (GetKeyState(VK_CONTROL.0 as i32) & (0x8000u16 as i16)) != 0;
        let alt  = (GetKeyState(VK_MENU.0 as i32)    & (0x8000u16 as i16)) != 0;
        let lwin = (GetKeyState(VK_LWIN.0 as i32)    & (0x8000u16 as i16)) != 0;
        let rwin = (GetKeyState(VK_RWIN.0 as i32)    & (0x8000u16 as i16)) != 0;
        
        ctrl || alt || lwin || rwin
    }
}
