use rhai::{Engine, Scope};

use crate::errorwrap::Error;
mod func;
use func::{save, merge_all_img_to_gfx, overlap_in_images, REMOVE};

fn wrapper_save(file_name: &str, str_add: &str, str_search: &str, str_above: bool, mod_tag: &str, mod_install_state: u8) -> String {
	let result = save(file_name, str_add, str_search, str_above, mod_tag, mod_install_state);
	match result {
		Ok(()) => "".to_string(),
		Err(e) => format!("{}", e),
	}
}

fn wrapper_merge(mod_gfx: &str, game_gfx: &str, backup_gfx: &str, index_file: &str, mod_install_state: u8) -> String {
	if mod_install_state != REMOVE {
		let result = merge_all_img_to_gfx(mod_gfx, game_gfx, backup_gfx, index_file);
		match result {
			Ok(_) => "".to_string(),
			Err(e) => format!("{}", e),
		}
	} else {
		"".to_string()
	}
}

fn wrapper_overlap(original_img: &str, on_top_img: &str, mod_install_state: u8) -> String {
	if mod_install_state != REMOVE {
		let result = overlap_in_images(original_img, on_top_img);
		match result {
			Ok(_) => "".to_string(),
			Err(e) => format!("{}", e),
		}
	} else {
		"".to_string()
	}
}

/*
fn wrapper_save(file_name: &str, str_add: &str, str_search: &str, str_above: bool, mod_tag: &str, mod_install_state: u8) -> String {
	let result = save(file_name, str_add, str_search, str_above, mod_tag, mod_install_state);
	match result {
		Ok(()) => "".to_string(),
		Err(e) => format!("{}", e),
	}
} */



macro_rules! throw_on_err {
	($s:expr) => {
		concat!("RESULT = ", $s, " if RESULT != \"\" {throw RESULT;}")
	}
}

#[tauri::command]
pub fn load_mod(mod_file: &str, mod_tag: &str, gfx_modded: &str, gfx_vanilla: &str, mod_install_state: u8) -> Result<(), Error>{
	// Create a new person

	// Create a new Rhai Engine and Scope
	let mut engine = Engine::new();
	let mut scope = Scope::new();

	// Register the Functions with the Rhai Engine
	engine
		.set_strict_variables(true)
		.set_allow_looping(false)
		.disable_symbol("let")
		.disable_symbol("const")
		.register_fn("Savefile", wrapper_save)
		.register_fn("Mergefile", wrapper_merge)
		.register_fn("Overlapfile", wrapper_overlap)
	;


	// Add the person object to the Rhai scope

	scope.push("File", "");
	scope.push("Add", "");
	scope.push("Search", "");
	scope.push("Above", false);

	scope.push("GfxFolder", "");
	scope.push("IndexFile", "");

	scope.push("RESULT", "");

	scope.push_constant("mod_tag", format!("(* {} *) ", mod_tag));
	scope.push_constant("mod_install_state", mod_install_state);
	scope.push_constant("gfx_modded", gfx_modded.to_string());
	scope.push_constant("gfx_vanilla", gfx_vanilla.to_string());

	// Read the script
	println!("{:?}", std::env::current_dir()?.display());
	let mut script = std::fs::read_to_string(mod_file)?;
	script = script.replace("Save_This.", throw_on_err!("Savefile(File, Add, Search, Above, mod_tag, mod_install_state);") );
	script = script.replace("Merge_This.", throw_on_err!("Mergefile(GfxFolder, gfx_modded, gfx_vanilla, IndexFile, mod_install_state);") );
	script = script.replace("Overlap_This.", throw_on_err!("Overlapfile(File, Add, mod_install_state);") );

	script.push_str("RESULT");

	// Evaluate a Rhai script
	let reslut = engine.eval_with_scope::<String>(&mut scope, &script);

	match reslut {
		Ok(_) => Ok(()),	// Code error
		Err(e) => Err(Error::Script(format!("{}", e))),							// Script parsing error
	}
}
/*
fn main() {
	let result = load_mod("./src/doc.dfmod","haha", "", "", 1);
	match result {
		Ok(()) => println!("Result: Good "),
		Err(error) => println!("{}", error),
	}
} */
