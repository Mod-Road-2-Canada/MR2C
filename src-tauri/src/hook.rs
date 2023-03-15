use std::fs::{File, read_dir, rename};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
	S: serde::ser::Serializer,
  {
	serializer.serialize_str(self.to_string().as_ref())
  }
}

pub fn test1() {
	let img1 = image::open("data/in1.png").expect("Opening image 1 failed");
	let img2 = image::open("data/in2.png").expect("Opening image 2 failed");
		
	let buf1 = img1

		filtered.save("data/out.png").expect("Saving image failed");
}

// const UPDATE: i8 = INSTALL | REMOVE;
#[tauri::command]
pub fn load_mod(mod_path: &str, install_state: i8) -> Result<(), Error> {
const INSTALL: i8 = 1;
const REMOVE: i8 = 2;

	println!("File: '{}'", mod_path);
	let file = File::open(mod_path)?;
	let reader = BufReader::new(file);

	let mut contents = String::new();

	let mut dfstate = 0;

	let mut mod_tag = String::new();
	let mut file_path = String::new();
	let mut str_search = String::new();
	let mut str_replace = String::new();

	for arg in reader.lines() {
		let arg = arg.unwrap();

		if arg.contains("> File:") {
			dfstate = 1;
		} else if arg.contains("> Line:") {
			dfstate = 2;
		} else if arg.contains("> Add:") {
			dfstate = 3;
		} else if arg.contains("> Save") {
			let file_path_temp = file_path.clone() + ".temp";
			
			{
				let file = File::open(&file_path)?;
				let out_file = File::create(&file_path_temp)?;

				let mut reader = BufReader::new(file);
				let mut writer = BufWriter::new(out_file);

				if install_state & REMOVE != 0 {
					assert!(!mod_tag.is_empty(), "Cannot uninstall NULL");

					contents = reader.lines()
						.map(|line| line.unwrap())
						.filter(|line| !line.contains(&mod_tag))
						.collect::<Vec<_>>()
						.join("\n");
				} else if install_state & INSTALL != 0 {
					reader.read_to_string(&mut contents).unwrap();
				}

				if install_state & INSTALL != 0 {
					str_replace = str_search.clone() + "\n" + &str_replace;
					contents = contents.replace(&str_search, &str_replace);
				}

				writer.write_all(contents.as_bytes())?;
			}

			rename(&file_path_temp, &file_path)?;

			contents.clear();
			str_search.clear();
			str_replace.clear();

			println!("Saved.");
			dfstate = 0;
		} else {
			match dfstate {
				0 => {},
				1 => { 
					println!("Filing: {}", arg);
					file_path = arg.trim().replace("\"", "");
					mod_tag = format!("(* {} *) ", file_path); 
					dfstate = 0;
				},
				2 => { 
					println!("Seeing: {}", arg);
					if !str_search.is_empty() {
						str_search.push('\n');
					}
					str_search.push_str(&arg);
				},
				3 => { 
					println!("Replacing: {}", arg);
					if !str_replace.is_empty() {
						str_replace.push('\n');
					}
					str_replace.push_str(&mod_tag);
					str_replace.push_str(&arg);
				},
				_ => println!("This is not a valid command: {}", arg)
			}
		}
	}

	Ok(())
}

#[tauri::command]
pub fn get_mods() -> Result<Vec<String>, Error> {
	let mut vec_paths = Vec::new();

	println!("File path: {}", std::env::current_dir().unwrap().display());

	let paths = read_dir("./Mods")?;


	for path in paths {
		let path = path.unwrap().path();
		let path_str = String::from(path.to_string_lossy());
		
		// ; 

		match path.extension() {
			Some(ex) if ex == "dfmod" => { vec_paths.push(path_str);  },
			_ => println!("Skipped file {}", path_str),
		}
	}

	Ok(vec_paths)
}

#[tauri::command]
pub fn test() -> Result<(), Error> {

	println!("{:?}", get_mods());

// 3 unwrap, 7 ?
	// print!("{}", contents);
	Ok(())
}