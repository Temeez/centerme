///////////////
// Modification of: https://old.reddit.com/r/rust/comments/kt4kts/enumerate_displays_windows/gik7jcq/
///////////////
use std::ops::Deref;

use winapi::shared::minwindef::{BOOL, DWORD, TRUE};
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HDC;
use winapi::shared::windef::HMONITOR;
use winapi::shared::windef::RECT;
use winapi::um::shellscalingapi::SetProcessDpiAwareness;
use winapi::um::shellscalingapi::PROCESS_PER_MONITOR_DPI_AWARE;
use winapi::um::winuser::MONITORINFO;
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW};

pub fn enumerate_monitors() -> Vec<MONITORINFO> {
    struct CallbackResultWrapper(Vec<Option<MONITORINFO>>);

    unsafe extern "system" fn enum_monitors_callback(
        hdc: HMONITOR,
        _lprc_clip: HDC,
        _lpfn_enum: *mut RECT,
        dw_data: isize,
    ) -> BOOL {
        let mut mi: MONITORINFO = std::mem::zeroed();
        mi.cbSize = std::mem::size_of::<MONITORINFO>() as DWORD;

        let monitor = if GetMonitorInfoW(hdc, &mut mi) == 0 {
            None
        } else {
            Some(mi)
        };

        let monitors_ptr = dw_data as *mut CallbackResultWrapper;
        (*monitors_ptr).0.push(monitor);

        TRUE
    }

    let monitors = unsafe {
        SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
        let mut monitors = CallbackResultWrapper { 0: vec![] };
        let monitors_ptr: *mut CallbackResultWrapper = &mut monitors;

        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null(),
            Some(enum_monitors_callback),
            monitors_ptr as isize,
        );

        monitors.0
    };

    let valid_monitors = monitors
        .iter()
        .filter_map(|monitor| *monitor)
        .collect::<Vec<_>>();

    let found_invalid = monitors.len() != valid_monitors.len();

    if monitors.is_empty() || valid_monitors.is_empty() || found_invalid {
        println!("Warning: Couldn't determine display position");
    }

    valid_monitors
}

pub struct VirtualScreen(Vec<MonitorInfo>);

impl VirtualScreen {
    pub fn new() -> Self {
        Self(enumerate_monitors().into_iter().map(MonitorInfo).collect())
    }

    // pub fn monitor_count(&self) -> usize {
    //     self.iter().len()
    // }

    pub fn monitor_bounds(&self, index: usize) -> RECT {
        let top = self.iter().nth(index).unwrap().get_display_monitor().top;
        let left = self.iter().nth(index).unwrap().get_display_monitor().left;
        let right = self.iter().nth(index).unwrap().get_display_monitor().right;
        let bottom = self.iter().nth(index).unwrap().get_display_monitor().bottom;

        RECT {
            top,
            left,
            right,
            bottom,
        }
    }
}

impl Deref for VirtualScreen {
    type Target = Vec<MonitorInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Rectangle {
    fn width(&self) -> LONG;

    fn height(&self) -> LONG;
}

impl Rectangle for RECT {
    fn width(&self) -> LONG {
        self.right - self.left
    }

    fn height(&self) -> LONG {
        self.bottom - self.top
    }
}

pub struct MonitorInfo(MONITORINFO);

impl MonitorInfo {
    pub fn get_display_monitor(&self) -> &RECT {
        &self.0.rcMonitor
    }
}

impl Deref for MonitorInfo {
    type Target = MONITORINFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
