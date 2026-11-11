// 打包为 Windows 发布版时隐藏控制台窗口（debug 保留以便看日志）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    umbra_desktop_lib::run()
}
