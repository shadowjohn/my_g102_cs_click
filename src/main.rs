#![windows_subsystem = "windows"]

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

use windows::Win32::Foundation::{HANDLE, GetLastError, ERROR_ALREADY_EXISTS, CloseHandle};
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
                std::mem::forget(handle);
                
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
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    
    unsafe { UnhookWindowsHookEx(hook); }
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