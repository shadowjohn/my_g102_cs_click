#![windows_subsystem = "windows"]
use tray_icon::{
    TrayIconBuilder,
    menu::{Menu, MenuItem, MenuEvent},
};
use once_cell::sync::Lazy;
use windows::core::PCWSTR;
use base64::Engine;
use image::load_from_memory;
use winit::event_loop::{EventLoop, ControlFlow};
//use winit::event::Event;


use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use windows::{
    core::w,
    Win32::{
        Foundation::*,
        UI::{
            Input::KeyboardAndMouse::*,
            WindowsAndMessaging::*,
        },
    },
};

use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS}; // ,HANDLE, CloseHandle
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONWARNING};

fn single_instance() -> bool {
    unsafe {
        let mutex_name = w!("MY_G102_CS_CLICK");
        
        // 1. CreateMutexW 回傳的是 Result，需要解包
        // 如果建立失敗 (例如系統資源不足)，這裡會直接 panic 或回傳錯誤，這裡簡化處理
        let result = CreateMutexW(None, false, mutex_name);

        match result {
            Ok(handle) => {
                // 2. 檢查 GetLastError 是否為 ERROR_ALREADY_EXISTS
                // 即使 CreateMutexW 回傳 Ok，如果是開啟現有的 Mutex，也會設定這個 Error Code
                if GetLastError() == ERROR_ALREADY_EXISTS {
                    // 已有實例在執行，這裡讓 handle 自動 Drop (CloseHandle) 即可
                    return false;
                }

                // 3. 【關鍵修正】
                // 為了讓 Mutex 在整個程式執行期間都有效，
                // 我們必須告訴 Rust "忘記" 釋放這個 handle。
                // 否則函式一結束，handle 被 drop，Mutex 就消失了。
                //std::mem::forget(handle);
                let _ = handle;
                
                true
            }
            Err(_) => {
                // 建立 Mutex 發生系統錯誤
                false
            }
        }
    }
}

// 修正 1: HHOOK 初始化時，0 必須轉型為指標 (0 as _)
//static mut H_HOOK: HHOOK = HHOOK(0 as _);
static mut H_HOOK: HHOOK = HHOOK(0 as _);

static CLICKING: AtomicBool = AtomicBool::new(false);


// ---------- Static text ----------
const VERSION: &str = "0.0.2";

static STR_ABOUT: Lazy<String> = Lazy::new(|| {
    format!(
        r#"
============================
G102 側鍵連點工具 V{}
作者: 羽山秋人 (https://3wa.tw)
版權所有 © 2025
============================

功能：
- 側鍵快速連點
- 適用於 CS 系列遊戲
- 高性能，低延遲

使用說明：
按下滑鼠側鍵即可開始/停止連點。

感謝使用本工具！"#,
        VERSION
    )
});

static TOOLTIP: Lazy<String> = Lazy::new(|| format!(
    "G102 CS Click By 羽山秋人 V{}",
    VERSION
));
// ---------- menu IDs ----------
const ABOUT_ID: &str = "關於";
const QUIT_ID: &str = "結束";

// ---------- Base64 icon ----------
const ICON_BASE64: &str = "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"; 
fn load_icon_from_base64(b64: &str) -> tray_icon::Icon {
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64).unwrap();
    let img = load_from_memory(&bytes).unwrap().into_rgba8();
    let (w, h) = img.dimensions();
    tray_icon::Icon::from_rgba(img.into_raw(), w, h).unwrap()
}

// ---------- Win32 helper ----------
fn to_pcwstr(s: &str) -> (Vec<u16>, PCWSTR) {
    let mut wide: Vec<u16> = s.encode_utf16().collect(); // UTF-16
    wide.push(0); // NUL 結尾
    let pcw = PCWSTR(wide.as_ptr());
    (wide, pcw)
}
fn hiword(val: u32) -> u16 {
    ((val >> 16) & 0xFFFF) as u16
}

fn main() {

    if !single_instance() {
        // 顯示 alert
        unsafe {
            MessageBoxW(None, w!("程式已經在執行中！"), w!("提示"), MB_OK | MB_ICONWARNING);
        }
        return;
    }
    // ---------- menu ----------
    let menu = Menu::new();
    menu.append(&MenuItem::with_id("關於", ABOUT_ID,true,None)).unwrap();
    menu.append(&MenuItem::with_id("結束", QUIT_ID,true, None)).unwrap();
    
    let icon = load_icon_from_base64(ICON_BASE64);
    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip(TOOLTIP.to_string())
        .build()
        .unwrap();

    // 一定要保留 tray，避免被 drop
    std::mem::forget(tray);
    MenuEvent::set_event_handler(Some(Box::new(|ev: MenuEvent| {
        match ev.id().0.as_str() {
            QUIT_ID => std::process::exit(0),
            ABOUT_ID => {
            let title_str = format!("About G102 CS Click v{}", VERSION);
                let (_title_buf, title) = to_pcwstr(&title_str);    
                let (_msg_buf, msg) = to_pcwstr(&*STR_ABOUT);                
                unsafe { MessageBoxW(None, msg, title, MB_OK); }
            }
            _ => {}
        }
    })));

    // 一定要有 event loop
    let event_loop = EventLoop::new();
    event_loop.run(move |_, _, control| {
        *control = ControlFlow::Wait;
    });

    let hook = unsafe {
        SetWindowsHookExW(
            WH_MOUSE_LL,
            Some(mouse_proc),
            None,
            0,
        )
    }.expect("Hook failed");

    unsafe { H_HOOK = hook; }

    thread::spawn(move || {
        loop {
            if CLICKING.load(Ordering::Relaxed) {
                send_left_click();
            }
            thread::sleep(Duration::from_millis(20));
        }
    });

    let mut msg = MSG::default();
    unsafe {
        // 修正 2: HWND 初始化時，0 也必須轉型為指標 (0 as _)
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    
    unsafe { let _ = UnhookWindowsHookEx(hook); }
}

extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        unsafe {
            let mouse = *(lparam.0 as *const MSLLHOOKSTRUCT);
            let is_injected = (mouse.flags & 0x01) != 0;
            
            if !is_injected {
                match wparam.0 as u32 {
                    WM_XBUTTONDOWN => {
                        let button = hiword(mouse.mouseData);
                        // 印出按下的按鍵代號，方便你確認
                        println!("偵測到側鍵按下，代號 (XBUTTON): {}", button);

                        // 修改這裡：判斷 XBUTTON1 或 XBUTTON2
                        // 如果你想兩顆鍵都能觸發，就用 || (OR)
                        if button == XBUTTON1 as u16  {
                            //|| button == XBUTTON2 as u16
                            println!("-> 啟動連點");
                            CLICKING.store(true, Ordering::Relaxed);
                        }
                    }
                    WM_XBUTTONUP => {
                        let button = hiword(mouse.mouseData);
                        
                        // 同樣監聽放開事件
                        if button == XBUTTON1 as u16  {
                            //|| button == XBUTTON2 as u16
                            println!("-> 停止連點");
                            CLICKING.store(false, Ordering::Relaxed);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

fn send_left_click() {
    unsafe {
        let inputs = [
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTDOWN,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}