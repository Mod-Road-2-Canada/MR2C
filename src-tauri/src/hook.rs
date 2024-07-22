use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use anyhow::Context;



use app::errorwrap::Error;
use app::custombail;


fn crop_modded_image<P: AsRef<Path>>(modded_path: P, backup_path: P, dest_path: P) -> Result<(), Error> {

	let modded = image::open(&modded_path).with_context(|| format!("Failed to open img 1: {}", modded_path.as_ref().display()))?;
	let original = image::open(&backup_path).with_context(|| format!("Failed to open img 2: {}", backup_path.as_ref().display()))?;
	
	let height = modded.height() - original.height();

	if height == 0 {
		println!("Warning: Skipped {} due to same height.", modded_path.as_ref().display());
		return Ok(())
	}

	let width = original.width();

	let cropped = modded.crop_imm(0, original.height(), width, height);
	cropped.save(&dest_path).with_context(|| format!("Failed to save imgs"))?;

	Ok(())
}

#[tauri::command]
pub fn crop_all_img_to_gfx(modded_dir: &str, backup_dir: &str, dest_dir: &str) -> Result<u16, Error> {
	let mut countcropped: u16 = 0;

	for entry in WalkDir::new(modded_dir) {
		let entry = entry?;
		let modded_path = entry.path();
		
		let filename = modded_path.strip_prefix(modded_dir)?;
		let backup_path = Path::new(backup_dir).join(filename);
		let dest_path = Path::new(dest_dir).join(filename);

		if !backup_path.try_exists()? { continue;	}	// Skip files that don't exist in original gfx

		if let (Some(ex1), Some(ex2)) = (modded_path.extension(), backup_path.extension()) {
			if ex1 == "png" && ex2 == "png" {
				crop_modded_image(&modded_path, &backup_path.as_path(), &dest_path.as_path())?;
				countcropped += 1;
			}
		}
	}

	Ok(countcropped)
}




fn check_two_dir_name(src: &str, dst: &str) -> Result<(), Error> {
	let src_path = Path::new(src);
	let dst_path = Path::new(dst);
	if let (Some(src_name), Some(dst_name)) = (src_path.file_name(), dst_path.file_name()) {
		if src_name != dst_name {
			custombail!("Chosen folder name is not '{}'", dst_name.to_string_lossy().into_owned());
		}
	} else {
		custombail!("Unrecoverable error?");
	}

	Ok(())
}

#[tauri::command]
pub fn copy_dir_all(src: &str, dst: &str, overwrite: bool) -> Result<u16, Error> {

	// Check if name is the same
	check_two_dir_name(src, dst)?;
	// Create folder
	fs::create_dir_all(dst)?;

	let mut countdir = 0;
	let mut countcopied: u16 = 0;

	for entry in WalkDir::new(src) {
		let entry = entry?;
		let path = entry.path();
		let destination = Path::new(dst).join(path.strip_prefix(src)?);

		if destination.try_exists()? && !overwrite { 
			println!("Already exists. {}", destination.display());
			continue; 
		} else {
			println!("Copy to: {}", destination.display());
		}

		if path.is_dir() {
			fs::create_dir_all(&destination)?;
			countdir += 1;
		} else {
			match path.extension() {
				Some(ex) if ex == "png" => {
					fs::copy(&path, &destination)?;
					countcopied += 1; 
				},
				_ => continue,
			}
		}
	}

	println!("Directory created: {}\nCopied: {}", countdir, countcopied);
	Ok(countcopied)
}



#[tauri::command]
pub fn get_jsons(mod_folder: &str) -> Result<Vec<String>, Error> {
	let mut vec_jsons = Vec::new();

	// println!("file path: {}", std::env::current_dir()?.display());

	for entry in fs::read_dir(mod_folder)? {
		let entry = entry?;
		let path = entry.path();
		let json_name = path.file_name().ok_or(Error::Other("Cannot get file name.".to_string()))?.to_string_lossy().into_owned();

		match path.extension() {
			Some(ex) if ex == "json" => {
				let contents = fs::read_to_string(&path)?;

				vec_jsons.push(json_name);
				vec_jsons.push(contents);
			},
			_ => println!("Skipped file/folder {}", json_name),
		}
	}

	if vec_jsons.len() % 2 != 0 {
		custombail!("Something went wrong (vec_jsons is not even)");
	}

	Ok(vec_jsons)
}

#[tauri::command]
pub fn create_dir_if_not_exist(name: &str) -> Result<(), Error> {
	let path = Path::new(name);
	if !path.exists() {
		fs::create_dir(path)?;
	}

	Ok(())
}

// General fs
#[tauri::command]
pub fn check_file_in_cwd(file_path: &str) -> bool {
	if let Ok(current_dir) = std::env::current_dir() {
		if let Some(path) = Path::new(file_path).parent() {
			return path == current_dir;
		}
	}

	false
}

#[tauri::command]
pub fn get_cwd() -> Result<PathBuf, Error> {
	let path = std::env::current_dir()?;
	Ok(path)
}

#[tauri::command]
pub fn set_cwd(path_cwd: &str) -> Result<(), Error> {
	std::env::set_current_dir(path_cwd)?;
	Ok(())
}

#[tauri::command]
pub fn load_cookies(file: &str) -> Result<String, Error> {
	// Return json string if found
	// empty string otherwise

	match fs::read_to_string(file) {
		Ok(cookies) => Ok(cookies),
		Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
		Err(e) => Err(e.into()),
	}
}

#[tauri::command]
pub fn save_cookies(data: &str, file: &str) -> Result<(), Error> {
	fs::write(file, data)?;
	println!("Writen {} {}", 	std::env::current_dir().unwrap().display(), file);
	Ok(())
}


