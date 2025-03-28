use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::GetCapture,
        WindowsAndMessaging::{CallNextHookEx, PostMessageW, WM_KEYDOWN},
    },
};

#[no_mangle]
unsafe extern "system" fn keyboard_hook_proc(
    ncode: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if ncode >= 0 {
        let capture_window = unsafe { GetCapture() };

        unsafe { PostMessageW(Some(capture_window), WM_KEYDOWN, wparam, lparam).unwrap() };
        return LRESULT(1);
    }
    CallNextHookEx(None, ncode, wparam, lparam)
}

#[no_mangle]
unsafe extern "system" fn mouse_hook_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if ncode >= 0 {
        let capture_window = unsafe { GetCapture() };

        let _ =
            unsafe { PostMessageW(Some(capture_window), wparam.0 as u32, WPARAM(0), LPARAM(0)) };
    };

    CallNextHookEx(None, ncode, wparam, lparam)
}
