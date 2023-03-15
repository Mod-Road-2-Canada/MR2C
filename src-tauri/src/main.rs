#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod hook;
use hook::{get_mods, load_mod, test, test1};




fn main() {
	std::env::set_current_dir("../../").unwrap();
	println!("{}", std::env::current_dir().unwrap().display());
	test1();

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_mods, load_mod, test])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
