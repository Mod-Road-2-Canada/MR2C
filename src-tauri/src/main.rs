// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use(concat_string)]
extern crate concat_string;


#[macro_use]
mod errorwrap;
mod hook;
mod loader;

use loader::load_mod;
use hook::{
	set_cwd, 
	get_cwd, 
	save_cookies, 
	load_cookies, 
	get_jsons, 
	check_file_in_cwd,

	copy_dir_all,
	crop_all_img_to_gfx
};

fn main() {
	// std::env::set_current_dir("../../").unwrap();
	println!("{}", std::env::current_dir().unwrap().display());

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			set_cwd, 
			get_cwd, 
			save_cookies, 
			load_cookies, 
			get_jsons, 
			check_file_in_cwd,

			copy_dir_all,
			crop_all_img_to_gfx,

			load_mod
		])
		.plugin(tauri_plugin_window_state::Builder::default().build())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
