mod monitors;

use std::sync::{Arc, RwLock};
use std::{thread, time};

use crate::monitors::{Rectangle, VirtualScreen};
use clap::Parser;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::GetWindowRect;
use winapi::um::winuser::{
    GetForegroundWindow, GetWindowTextW, SetWindowPos, HWND_TOP, SWP_NOSIZE,
};

// https://stackoverflow.com/a/30860086
fn get_window_title(window: HWND) -> String {
    let mut v: Vec<u16> = Vec::with_capacity(255);
    unsafe {
        let read_len = GetWindowTextW(window, v.as_mut_ptr(), v.capacity().try_into().unwrap());
        v.set_len(read_len.try_into().unwrap()); // this is undefined behavior if read_len > v.capacity()
        String::from_utf16_lossy(&v)
    }
}

fn center_window(window_title: &str, already_did: &Arc<RwLock<i8>>, delay: u64) {
    let vs = VirtualScreen::new();
    let primary_monitor = vs.monitor_bounds(0);
    let w = primary_monitor.width();
    let h = primary_monitor.height();

    let hwnd = unsafe { GetForegroundWindow() };
    let found_title = get_window_title(hwnd);

    if found_title.as_str() != window_title && *already_did.read().unwrap() == 1 {
        let mut i = already_did.write().unwrap();
        *i = 0;
    }

    if (window_title.is_empty() || found_title.as_str() == window_title)
        && *already_did.read().unwrap() == 0
    {
        // Wait x milliseconds before getting the target window size and centering it.
        if delay > 0 {
            thread::sleep(time::Duration::from_millis(delay));
        }

        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            GetWindowRect(hwnd, &mut rect);
        }

        let ww = rect.right - rect.left;
        let wh = rect.bottom - rect.top;

        let x = (w / 2) - (ww / 2);
        let y = (h / 2) - (wh / 2);

        unsafe {
            SetWindowPos(hwnd, HWND_TOP, x, y, 100, 100, SWP_NOSIZE);
            let mut i = already_did.write().unwrap();
            *i = 1;
        }
    }
}

/// Simple program to center a window on the primary monitor.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Title of the window to center when it is active (foreground).
    #[arg(short, long)]
    title: Option<String>,

    /// How long to wait in milliseconds before centering the window.
    /// Some windows may change in size shortly after appearing so
    /// this delay will allow more flexibility.
    #[arg(short, long)]
    delay: Option<u64>,

    /// Print the title of the currently active (foreground) window after 1 second.
    #[arg(short, long)]
    print: bool,
}

fn main() {
    let args = Args::parse();

    if args.print {
        // Wait one second.
        thread::sleep(time::Duration::from_millis(1000));

        // Get the current foreground window.
        let hwnd = unsafe { GetForegroundWindow() };
        // Get the title of the current foreground window.
        let found_title = get_window_title(hwnd);
        // Print the title for users viewing (copy paste) pleasure.
        println!("{}", found_title);
    }

    if let Some(title_to_center) = args.title {
        // Delay in milliseconds before centering.
        let delay = if let Some(d) = args.delay { d } else { 0 };
        // Use this crazy thing to center the correct window only once as long
        // as it is on the foreground.
        let already_did = Arc::new(RwLock::new(0));

        // Loop till manually killed!
        loop {
            center_window(&title_to_center, &already_did, delay);

            thread::sleep(time::Duration::from_millis(200));
        }
    }
}
