use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use anyhow::Context;



/*
pub fn load_mod(mod_path: &str, mod_tag: &str, gfx_modded: &str, gfx_vanilla: &str, install_state: u8) -> Result<(), Error> {
const INSTALL: u8 = 1;	// Bitwise value
const REMOVE: u8 = 2;
// const UPDATE: i8 = INSTALL | REMOVE;

	// println!("File path: {}", std::env::current_dir()?.display());
	println!("File: '{}'", mod_path);
	let file = File::open(mod_path).with_context(|| format!("Path: {}", mod_path))?;

	// if mod_tag.is_empty() { custombail!("Mod tag not defined for {}", mod_path); }	// Checked in front-end
	let mod_tag = format!("(* {} *) ", mod_tag);

	let mut command = String::new();

	let mut file_path = String::new();
	let mut str_search = String::new();
	let mut str_replace = String::new();
	let mut contents = String::new();

	let mut img_first_path = String::new();

	let reader = BufReader::new(file);
	let mut lines = reader.lines().map(|l| l.unwrap()).peekable();

	while let Some(line) = lines.next() {
		let v: Vec<&str> = line.split_whitespace().collect();

		if v.len() == 0 {
			continue;	// Skip empty lines
		}

		command.clear();

		if v.len() == 2 {
			if v[0] == MOD_CMD {	// Starting of a command
				command = v[1].to_string();
			}
		}

		if install_state & INSTALL != 0 {
			match command.as_str() {
				"File:" => {
					if let Some(line) = lines.next() {
						println!("Filing: {}", line);
						file_path = line.trim().replace("\"", "");			
					}
				},
				"Line:" => {
					while let Some(line) = lines.next() {
						println!("Seeing: {}", line);
						if !str_search.is_empty() { str_search.push('\n'); }
						str_search.push_str(&line);

						// Iterate 'til next command
						if let Some(line) = lines.peek() { if line.contains(MOD_CMD) { break; } }
					}
				},
				"Add:" => {
					while let Some(line) = lines.next() {
						println!("Replacing: {}", line);
						if !str_replace.is_empty() {
							str_replace.push('\n');
						}
						str_replace.push_str(&mod_tag);
						str_replace.push_str(&line);

						// Iterate 'til next command
						if let Some(line) = lines.peek() { if line.contains(MOD_CMD) { break; } }
					}
				},
				"Save" => {
					let file_path_temp = file_path.clone() + ".temp";

					{
						let file = File::open(&file_path)?;
						let out_file = File::create(&file_path_temp)?;

						let mut reader = BufReader::new(file);
						let mut writer = BufWriter::new(out_file);

						if install_state & REMOVE != 0 {

							contents = reader.lines()
								.map(|line| line.unwrap())
								.filter(|line| !line.contains(&mod_tag))
								.collect::<Vec<_>>()
								.join("\n");
						} else if install_state & INSTALL != 0 {
							reader.read_to_string(&mut contents)?;
						}

						if install_state & INSTALL != 0 {
							str_replace = str_search.clone() + "\n" + &str_replace;
							contents = contents.replace(&str_search, &str_replace);
						}

						writer.write_all(contents.as_bytes())?;
					}

					fs::rename(&file_path_temp, &file_path)?;

					contents.clear();
					// file_path.clear(); // Multiple edits in a file
					str_search.clear();
					str_replace.clear();

					println!("Saved.");
				},
				/*
				"TakeImg:" => {
					if let Some(line) = lines.next() {
						println!("Taking: {}", line);
						img_path = line.trim().replace("\"", "");			
					}
				},
				"MergeImg:" => {
					if let Some(line) = lines.next() {
						println!("Merging: {}", line);
						let img_merge_path = line.trim().replace("\"", "");

						merge_two_images(&img_path, &img_merge_path)?;
						img_path.clear();	
					}
				}, */

				"MergeAllIn:" => {
					if let Some(line) = lines.next() {
						if install_state != REMOVE {
							println!("Merging All: {}", line);

							let img_merge_path = line.trim().replace("\"", "");
							merge_all_img_to_gfx(&img_merge_path, gfx_modded, gfx_vanilla)?;
						}
					}
				},

				"OverlapImage:" => {
					if let Some(line) = lines.next() {
						img_first_path = line.trim().replace("\"", "");
					}
				},

				"WithImage:" => {
					if let Some(line) = lines.next() {
						if install_state != REMOVE {
							println!("Replacing: {}", line);

							let img_overlap_path = line.trim().replace("\"", "");
							overlap_in_images(&img_first_path, &img_overlap_path)?;
						}
					}
				},

				"" => {
					custombail!("No command for argument: {}", line);
				},
				_ => {
					custombail!("Invalid command: '{}'", command);
				}
			}
		} 

	}

	// std::thread::sleep(std::time::Duration::from_secs(2));
	Ok(())
} */


use crate::errorwrap::Error;


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




#[tauri::command]
pub fn copy_dir_all(src: &str, dst: &str, overwrite: bool) -> Result<u16, Error> {
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
			_ => println!("Skipped file {}", json_name),
		}
	}

	if vec_jsons.len() % 2 != 0 {
		custombail!("Something went wrong (vec_jsons is not even)");
	}

	Ok(vec_jsons)
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
	let cookies = fs::read_to_string(file)?;
	Ok(cookies)
}

#[tauri::command]
pub fn save_cookies(data: &str, file: &str) -> Result<(), Error> {
	fs::write(file, data)?;
	println!("Writen {} {}", 	std::env::current_dir().unwrap().display(), file);
	Ok(())
}
