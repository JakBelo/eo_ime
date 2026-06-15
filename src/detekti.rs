use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;

pub fn estas_esperanta() -> bool {
    unsafe {
        // Klavara pritraktilo.
        let kp = GetKeyboardLayout(0);
        let lin_id = kp.0 as isize;
        lin_id == 67706880 // Esperanta lingvo-identigilo.
    }
}
