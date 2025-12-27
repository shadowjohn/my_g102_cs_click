my_g102_cs_click

🖱️ My G102 CS Clicker

輕量級滑鼠連點工具，專為 Counter-Strike (CS) 設計。
主要目的是取代龐大的 Logitech G Hub 軟體，提供更低延遲、更節省資源的側鍵連點方案。

---
✨ 特色

極致輕量：使用 Rust 編寫，無須安裝肥大的驅動程式。

針對 G102 優化：專門監聽 Logitech G102 的側鍵訊號。

隱藏式執行：背景運作，不佔用視窗空間。

單一執行檔：編譯後僅需一個 .exe 即可在任何 Windows 電腦上執行。

---
📦 最新版本

V0.0.2 (2025-12-28)


---

🛠️ 前置需求

作業系統：Windows 10 / 11
Rust 環境：可至官網安裝 rustup.rs

---
🚀 快速開始

1. 下載專案
git clone https://github.com/shadowjohn/my_g102_cs_click
cd my_g102_cs_click

2. 編譯專案
建議使用 release 模式以獲得最佳效能與最小體積：

cargo build --release

或可直接使用提供的 release.bat 編譯。

3. 執行程式
編譯完成後的執行檔位於 target/release/ 資料夾中：

.\target\release\my_g102_cs_click.exe

---
🎮 使用方式

按住滑鼠左側靠後的側鍵 XBUTTON1

功能：自動以 10ms 間隔 觸發左鍵點擊（適合手槍局或連點槍枝）

關閉程式：開啟工作管理員，結束 my_g102_cs_click.exe 工作

---
⚠️ 常見問題與故障排除

Q: 按下側鍵沒有反應？

A: 請檢查電腦是否安裝了 Logitech G Hub。

G Hub 可能會攔截側鍵訊號並轉為鍵盤訊號。

建議完全關閉 G Hub，或使用 Logitech Onboard Memory Manager 將側鍵恢復為預設。

---
🛑 免責聲明 (Disclaimer)

本程式僅供學習 Rust 與 Windows API 互動使用。
在多人線上遊戲（如 VAC 保護的伺服器）中使用自動腳本可能違反遊戲服務條款並導致帳號被封鎖。
使用者需自行承擔使用風險。

---
👨‍💻 作者

羽山秋人 – https://3wa.tw


---
📝 Todo List

(Done 2025-12-28) 1. 增加右下角系統列圖示，方便使用者知道程式正在運行中。
(Done 2025-12-28) 2. 增加右下角系統列「關於」，並可以提示目前版本。
