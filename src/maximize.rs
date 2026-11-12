use crate::window::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

pub trait WindowExt {
    fn maximize(&self);
}

impl WindowExt for Window {
    fn maximize(&self) {
        // Change `self.window` below to match your `minifb::Window` field name in `src/window.rs`
        let Ok(handle) = self.window.window_handle() else {
            return;
        };

        match handle.as_raw() {
            #[cfg(windows)]
            RawWindowHandle::Win32(h) => {
                use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_MAXIMIZE};
                unsafe {
                    ShowWindow(h.hwnd.get() as *mut _, SW_MAXIMIZE);
                }
            }

            #[cfg(target_os = "macos")]
            RawWindowHandle::AppKit(h) => {
                use objc2::msg_send;
                use objc2::runtime::AnyObject;
                unsafe {
                    let ns_view = h.ns_view.as_ptr() as *mut AnyObject;
                    let ns_window: *mut AnyObject = msg_send![ns_view, window];
                    if !ns_window.is_null() {
                        let is_zoomed: bool = msg_send![ns_window, isZoomed];
                        if !is_zoomed {
                            let _: () = msg_send![ns_window, zoom: std::ptr::null::<AnyObject>()];
                        }
                    }
                }
            }

            #[cfg(target_os = "linux")]
            RawWindowHandle::Xlib(h) => {
                use x11rb::connection::Connection;
                use x11rb::protocol::xproto::{
                    ClientMessageEvent, ConnectionExt, EventMask, CLIENT_MESSAGE_EVENT,
                };

                if let Ok((conn, screen_num)) = x11rb::connect(None) {
                    let screen = &conn.setup().roots[screen_num];
                    let wm_state = conn.intern_atom(false, b"_NET_WM_STATE").unwrap().reply().unwrap().atom;
                    let max_horz = conn.intern_atom(false, b"_NET_WM_STATE_MAXIMIZED_HORZ").unwrap().reply().unwrap().atom;
                    let max_vert = conn.intern_atom(false, b"_NET_WM_STATE_MAXIMIZED_VERT").unwrap().reply().unwrap().atom;

                    let event = ClientMessageEvent {
                        response_type: CLIENT_MESSAGE_EVENT,
                        format: 32,
                        sequence: 0,
                        window: h.window as u32,
                        type_: wm_state,
                        data: [1, max_horz, max_vert, 1, 0].into(), // 1 = _NET_WM_STATE_ADD
                    };

                    let _ = conn.send_event(
                        false,
                        screen.root,
                        EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                        event,
                    );
                    let _ = conn.flush();
                }
            }

            _ => {}
        }
    }
}
