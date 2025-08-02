// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hook;
mod rhai_engine;

use hook::{
    check_file_in_cwd, create_backup_gfx, create_dir_if_not_exist, crop_all_img_to_gfx, get_cwd,
    get_jsons, load_cookies, restore_backup_gfx, save_cookies, set_cwd,
};
use rhai_engine::load_mod;

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    println!("{}", current_dir.display());

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Folder {
                    path: current_dir,
                    file_name: None,
                },
            )).build()
        )
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            set_cwd,
            get_cwd,
            save_cookies,
            load_cookies,
            get_jsons,
            check_file_in_cwd,
            create_dir_if_not_exist,
            create_backup_gfx,
            restore_backup_gfx,
            crop_all_img_to_gfx,
            load_mod
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
