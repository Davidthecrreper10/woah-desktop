// No console window on Windows in the finished app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    woah_lib::run()
}
