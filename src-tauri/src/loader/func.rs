use std::fs::File;
use std::io::{Read, Write, BufRead, BufReader, BufWriter};

use crate::errorwrap::Error;


pub const INSTALL: u8 = 1;	// Bitwise value
pub const REMOVE: u8 = 2;


fn count_between_substr_newline(string: &str, substring: &str) -> usize {
	if let Some(substring_index) = string.find(substring) {
		let substring_end = substring_index + substring.len();

		println!("---Checking ::{} ::{}", &string, &substring);

		if let Some(newline_index) = string[substring_end..].find('\n') {
			let count = string[substring_end..substring_end + newline_index]
				// .trim()
				.chars()
				.count();

			return count;
		}
	}

	0
}

pub fn save(file_name: &str, str_add: &str, str_search: &str, str_above: bool, mod_tag_o: &str, mod_install_state: u8) -> Result<(), Error> {
	// println!("{}\n{}\n{}\n{}\n{}\n", file_name, str_add, str_search, str_above, mod_tag_o); // Debug
	let file_temp = format!("{}.temp", file_name);
	let mod_tag_newline = format!("\n(* {} nl *) ", mod_tag_o);
	let mod_tag = format!("(* {} *) ", mod_tag_o);

	{	// file only exist within scope
		let file = File::open(file_name)?;
		let out_file = File::create(&file_temp)?;

		let mut reader = BufReader::new(file);
		let mut writer = BufWriter::new(out_file);
		let mut contents = String::new();

		if mod_install_state == REMOVE {

			contents = reader.lines()
				.map(|line| line.unwrap())
				.filter(|line| !line.contains(&mod_tag))
				.collect::<Vec<_>>()
				.join("\n");

			contents = contents.replace(&mod_tag_newline, "");
			// Remove the inserted newline to separate from the mod (if inserted)

		} else if mod_install_state == INSTALL {
			reader.read_to_string(&mut contents)?;
			let str_replace: String;
			let mut str_add = concat_string!(mod_tag, str_add);
			str_add = str_add.replace("\n", &concat_string!("\n", mod_tag)); // Add tag for each additional multilines

			if str_above {
				// Above go above all else (no pun intended)
				str_replace = concat_string!(str_add, "\n", str_search);
			} else {
				if count_between_substr_newline(&contents, &str_search) > 0 {
					// Case when want to insert
					str_replace = concat_string!(str_search, "\n", str_add, mod_tag_newline);
				} else {
					str_replace = concat_string!(str_search, "\n", str_add);	
				}
			}

			contents = contents.replace(str_search, &str_replace);
			// println!("Replace: {}", str_search);
			// println!("With: {}", str_replace);
		}

		writer.write_all(contents.as_bytes())?;
	}

	std::fs::rename(&file_temp, file_name)?;

	println!("Saved.");
	Ok(())
}



use std::path::Path;
use walkdir::WalkDir;
use anyhow::Context;
use image::GenericImage;

#[derive(Debug)]
struct Indices {
	arr: [u32; SPRITE_TOTAL],
}

impl Indices {
	fn new() -> Self {
		Indices { 
			arr: [0; SPRITE_TOTAL],
		}
	}

	fn get_changed_indices(&mut self) -> Vec<usize> {
		(1..self.arr.len()).filter(|&i| self.arr[i] != 0).collect()
	}

	fn map_modded_indices(&mut self, all: &mut Indices) {
		for i in 2..self.arr.len() {
			self.arr[i] += all.arr[i-1];
			all.arr[i] += all.arr[i-1];
		}
	}

	fn map_from_changed_indices_to_df_file(&self, filtered: &Vec<usize> , output_file: &str) -> Result<(), Error> {
		let file = File::create(output_file)?;
		let mut writer = BufWriter::new(file);

		for i in filtered.iter() {
			let line = format!("{} constant {}\n", self.arr[*i], Self::map_index_to_name(*i));
			writer.write_all(line.as_bytes())?;
		}

		writer.flush()?;

		let last_index = self.arr.len() - 1; 
		if self.arr[last_index] > 15200 {
			custombail!("WARNING: TOO MANY SPRITES {} (GAME MIGHT BREAK)", self.arr[last_index]);
		}

		Ok(())
	}

	fn map_index_to_name(index: usize) -> &'static str {
		match index {
			1 => "font6x8_mod_id",
			2 => "dr2c_title_small_mod_id",

			3 => 	"dr2c_zombie_bodies_mod_id",
			4 => 	"dr2c_zombie_heads_mod_id",
			5 => 	"dr2c_skeletons_24px_mod_id",
			6 => 	"dr2c_male_heads_mod_id",
			7 => 	"dr2c_male_heads_extra_mod_id",
			8 => 	"dr2c_male_hair_mod_id",
			9 => 	"dr2c_male_hair_extra_mod_id",
			10 => 	"dr2c_male_glasses_mod_id",
			11 => 	"dr2c_male_glasses_extra_mod_id",
			12 => 	"dr2c_male_beards_mod_id",
			13 => 	"dr2c_male_beards_extra_mod_id",
			14 => 	"dr2c_male_hats_mod_id",
			15 => 	"dr2c_male_hats_extra_mod_id",
			16 => 	"dr2c_male_bodies_mod_id",
			17 => 	"dr2c_male_bodies_extra_mod_id",

			18 => 	"dr2c_female_heads_mod_id",
			19 => 	"dr2c_female_heads_extra_mod_id",
			20 => 	"dr2c_female_hair_mod_id",
			21 => 	"dr2c_female_hair_extra_mod_id",
			22 => 	"dr2c_female_glasses_mod_id",
			23 => 	"dr2c_female_glasses_extra_mod_id",
			24 => 	"dr2c_female_hats_mod_id",
			25 => 	"dr2c_female_hats_extra_mod_id",
			26 => 	"dr2c_female_bodies_mod_id",
			27 => 	"dr2c_female_bodies_extra_mod_id",

			28 => 	"dr2c_special_chars_mod_id",
			29 => 	"dr2c_special_chars48_mod_id",

			30 => 	"superpets24_mod_id",
			31 => 	"pets24_mod_id",
			32 => 	"pets48_mod_id",
			// => 	"hairpalette_mod_id",
			// => 	"skinpalette_mod_id",

			33 => 	"dr2c_weapons_mod_id",
			34 => 	"dr2c_weapons_16x32px_mod_id",
			35 => 	"dr2c_loot16_mod_id",

			36 => 	"dr2c_cityfurniture_16px_mod_id",
			37 => 	"dr2c_housefurniture_32px_mod_id",

			// => 	"dr2c_cityfurniture_32px_mod_id",
			// => 	"trash16_mod_id",
			// => 	"trash32_mod_id",

			38 => 	"buildings_mod_id",
			39 => 	"city_generic_mod_id",
			40 => 	"city_florida_mod_id",
			41 => 	"city_carolina_mod_id",
			42 => 	"houses_mod_id",
			43 => 	"buildings_misc_mod_id",

			44 => 	"street_mod_id",
			45 => 	"walls_mod_id",
			46 => 	"floors_mod_id",
			47 => 	"doors_mod_id",
			48 => 	"shelves_mod_id",

			// 49 => 	"custom_mod_id",

			49 => 	"special16_mod_id",
			50 => 	"special32_mod_id",
			51 => 	"special48_mod_id",
			52 => 	"special64_mod_id",
			53 => 	"special96_mod_id",

			54 => 	"dr2c_particles_mod_id",
			55 => 	"dr2c_particles32_mod_id",
			56 => 	"icons16x16_mod_id",
			57 => 	"dr2c_grain64x64_mod_id",
			58 => 	"eventcards80x45_mod_id",

			59 => 	"dr2c_roadview_florida_64px_mod_id",
			60 => 	"dr2c_roadview_carolina_64px_mod_id",
			61 => 	"dr2c_roadview_virginia_64px_mod_id",
			62 => 	"dr2c_roadview_all_layers_64px_mod_id",
			63 => 	"dr2c_roadview_canada_64px_mod_id",
			64 => 	"trees_96px_mod_id",
			// => 	"dr2c_roadview_otherstuff_16px_mod_id",

			65 => 	"dr2c_cars110x96_mod_id",
			// 66 => 	"cars_unique_110x96_mod_id",
			66 => 	"car-wrecks124x96_mod_id",
			67 => 	"rocketcat_mod_id",
			68 => 	"gnomes_mod_id",
			69 => 	"skeletonbits_mod_id",
			70 => 	"madgarden_mod_id",
			_ => "bugged_mod_id"
		}
	}

	fn map_file_to_indices(&mut self, file: &str, sheet_area: u32) -> () {
		let (sprite_num, sprite_dimensions): (usize, [u32; 2])
		= match file {
		//	gfx\
		//	gfx\fonts
			"font6x8.png"			
			=> (1, [0,256]),
		//	gfx\ui
			"dr2c_title_small.png"	
			=> (2, [0,1]),
		//	gfx\chars
			"dr2c_zombie_bodies.png"
			=> (3, [24,24]),
			"dr2c_zombie_heads.png"	
			=> (4, [24,24]),
			"dr2c_skeletons_24px.png"	
			=> (5, [24,24]),
			
				// head hair glass beard hat body
			"dr2c_male_heads.png"
			=> (6, [24,24]),
			"dr2c_male_heads_extra.png"
			=> (7, [24,24]),
			"dr2c_male_hair.png"
			=> (8, [24,24]),
			"dr2c_male_hair_extra.png"
			=> (9, [24,24]),
			"dr2c_male_glasses.png"		
			=> (10, [24,24]),
			"dr2c_male_glasses_extra.png"
			=> (11, [24,24]),
			"dr2c_male_beards.png"		
			=> (12, [24,24]),
			"dr2c_male_beards_extra.png"
			=> (13, [24,24]),
			"dr2c_male_hats.png"		
			=> (14, [24,24]),
			"dr2c_male_hats_extra.png"	
			=> (15, [24,24]),
			"dr2c_male_bodies.png"		
			=> (16, [24,24]),
			"dr2c_male_bodies_extra.png"
			=> (17, [24,24]),

			"dr2c_female_heads.png"		
			=> (18, [24,24]),
			"dr2c_female_heads_extra.png"
			=> (19, [24,24]),
			"dr2c_female_hair.png"		
			=> (20, [24,24]),
			"dr2c_female_hair_extra.png"
			=> (21, [24,24]),
			"dr2c_female_glasses.png"	
			=> (22, [24,24]),
			"dr2c_female_glasses_extra.png"
			=> (23, [24,24]),
			"dr2c_female_hats.png"		
			=> (24, [24,24]),
			"dr2c_female_hats_extra.png"
			=> (25, [24,24]),
			"dr2c_female_bodies.png"	
			=> (26, [24,24]),
			"dr2c_female_bodies_extra.png"
			=> (27, [24,24]),

			"dr2c_special_chars.png"	
			=> (28, [24,24]),
			"dr2c_special_chars48.png"	
			=> (29, [48,48]),

			"superpets24.png"		
			=> (30, [24,24]),
			"pets24.png"			
			=> (31, [24,24]),
			"pets48.png"			
			=> (32, [48,48]),
			"hairpalette.png"		
				=> (0, [1,1]),	// ?
			"skinpalette.png"		
				=> (0, [1,1]),	// ?

		//	gfx\items
			"dr2c_weapons.png"		
			=> (33, [8,32]),
			"dr2c_weapons_16x32px.png"
			=> (34, [16,32]),
			"dr2c_loot16.png"		
			=> (35, [16,16]),

			"dr2c_cityfurniture_16px.png"
			=> (36, [16,16]),
			"dr2c_housefurniture_32px.png"
			=> (37, [32,32]),

			"dr2c_cityfurniture_32px.png"
				=> (0, [32,32]),	// ??
			"trash16.png"	
				=> (0, [16,16]),	// ??
			"trash32.png"	
				=> (0, [32,32]),	// ??
							
		//	gfx\tiles
			"buildings.png"		
			=> (38, [16,16]),
			"city_generic.png"	
			=> (39, [16,16]),
			"city_florida.png"	
			=> (40, [16,16]),
			"city_carolina.png"	
			=> (41, [16,16]),
			"houses.png"		
			=> (42, [16,16]),
			"buildings_misc.png"
			=> (43, [16,16]),

			"street.png"	
			=> (44, [16,16]),
			"walls.png"		
			=> (45, [16,16]),
			"floors.png"	
			=> (46, [16,16]),
			"doors.png"		
			=> (47, [16,16]),
			"shelves.png"	
			=> (48, [16,16]),

			"custom.png"
				=> (0, [16,16]),	// ??
			// gfx\items
			"special16.png"
			=> (49, [16,16]),
			"special32.png"	
			=> (50, [32,32]),
			"special48.png"	
			=> (51, [48,48]),
			"special64.png"	
			=> (52, [64,64]),
			"special96.png"	
			=> (53, [96,96]),
		//	gfx\misc
			"dr2c_particles.png"	
			=> (54, [16,16]),
			"dr2c_particles32.png"	
			=> (55, [32,32]),
			"icons16x16.png"		
			=> (56, [16,16]),
			"dr2c_grain64x64.png"	
			=> (57, [64,64]),
			"eventcards80x45.png"	
			=> (58, [80,45]),
		//	gfx\road
			"dr2c_roadview_florida_64px.png"	
			=> (59, [64,64]),
			"dr2c_roadview_carolina_64px.png"	
			=> (60, [64,64]),
			"dr2c_roadview_virginia_64px.png"	
			=> (61, [64,64]),
			"dr2c_roadview_all_layers_64px.png"	
			=> (62, [64,64]),
			"dr2c_roadview_canada_64px.png"		
			=> (63, [64,64]),
			"trees_96px.png"					
			=> (64, [96,96]),
			"dr2c_roadview_otherstuff_16px.png"	
				=> (0, [16,16]),		// ???
		//	gfx\cars
			"dr2c_cars110x96.png"		
			=> (65, [110,96]),
			"cars_unique_110x96.png"	// NOT LOADED
				=> (0, [110,96]),
			"car-wrecks124x96.png"		
			=> (66, [124,96]),
			// gfx\misc
			"rocketcat.png"			
			=> (67, [0,1]),
			"gnomes.png"			
			=> (68, [64,64]),
			"skeletonbits.png"
			=> (69, [48,48]),
		//	gfx\ui
			"madgarden.png"
			=> (70, [0,1]),		// final 70
			_ => (0, [0,0])
		};

		if sprite_dimensions[0] > 0 {
			let sprite_size = sprite_dimensions[0] * sprite_dimensions[1];
			self.arr[sprite_num] = (sheet_area) / sprite_size;
		} else {
			// The whole spritesheet counts as 1 / special count can't divide (font) (USUALLY CANNOT BE CHANGED)
			// TODO: Put a warning here that they're editing something that can't be indexed
			self.arr[sprite_num] = sprite_dimensions[1];
		}

		()
	}

	fn map_default(&mut self, dir: &str) -> Result<(), Error>{
		for entry in WalkDir::new(dir) {
			let entry = entry?;
			let path = entry.path();

			if let Some(ex1) = path.extension() {
				if ex1 == "png" {
					if let Some(file_name) = path.file_name() {
						let file_name = file_name.to_string_lossy().into_owned();
						let img = image::open(&path).with_context(|| format!("Failed to open img 1: {}", path.display()))?;

						let width = img.width();
						let height = img.height();

						self.map_file_to_indices(&file_name, width * height);
					}
				}
			}
		}

		Ok(())
	}
}

const SPRITE_TOTAL: usize = 70 + 1;

pub fn overlap_in_images(img1_path: &str, img2_path: &str) -> Result<(), Error> {
	let img1 = image::open(&img1_path).with_context(|| format!("Failed to open img 1: {}", img1_path))?;
	let img2 = image::open(&img2_path).with_context(|| format!("Failed to open img 2: {}", img2_path))?;

	if img2.height() > img1.height() {
		custombail!("Second image {} is larger the original image {}", img2_path, img1_path);
	}

	if img2.width() != img1.width() {
		custombail!("Second image {} is not same width with the original image {}", img2_path, img1_path);
	}

	let mut buffer = image::ImageBuffer::new(img1.width(), img1.height());
	buffer.copy_from(&img2, 0, 0)?;

	if img1.height() > img2.height() {
		let rest1 = img1.crop_imm(0, 0, img1.width(), img1.height() - img2.height());
		buffer.copy_from(&rest1, 0, img2.height())?;
	}

	buffer.save(&img1_path).with_context(|| format!("Failed to save imgs"))?;

	Ok(())
}

fn merge_two_images<P: AsRef<Path>>(img1_path: P, img2_path: P, mod_indices: &mut Indices, all_indices: &mut Indices) -> Result<(), Error> {
	println!("Merge to: {}", img1_path.as_ref().display());

	let img1 = image::open(&img1_path).with_context(|| format!("Failed to open img 1: {}", img1_path.as_ref().display()))?;
	let img2 = image::open(&img2_path).with_context(|| format!("Failed to open img 2: {}", img2_path.as_ref().display()))?;
	
	let width = std::cmp::max(img1.width(), img2.width());
	let height = img1.height() + img2.height();

	let mut buffer = image::ImageBuffer::new(width, height);

	buffer.copy_from(&img1, 0, 0)?;
	buffer.copy_from(&img2, 0, img1.height())?;
	buffer.save(&img1_path).with_context(|| format!("Failed to save imgs"))?;
	// image::save_buffer(&img1_path, &vec1, width, height, img1.color()).unwrap();


	let path_str = img1_path.as_ref().file_name().ok_or(Error::Other("Cannot get file name.".to_string()))?.to_string_lossy().into_owned();

	mod_indices.map_file_to_indices(&path_str, width * img1.height());
	all_indices.map_file_to_indices(&path_str, width * height);

	Ok(())
}

pub fn merge_all_img_to_gfx<P: AsRef<Path>>(mod_gfx: P, game_gfx: &str, backup_gfx: &str, index_file: &str) -> Result<u16, Error> {
	let mut countmerged: u16 = 0;

	if !mod_gfx.as_ref().is_dir() { custombail!("Not a directory: {}", mod_gfx.as_ref().display()); }

	let mut mod_indices = Indices::new();
	let mut all_indices = Indices::new();
	all_indices.map_default(backup_gfx)?;	// Start index all from vanilla

	for entry in WalkDir::new(&mod_gfx) {
		let entry = entry?;
		let path = entry.path();
		let destination = Path::new(game_gfx).join(path.strip_prefix(&mod_gfx)?);

		if !destination.try_exists()? { continue;	}	// Skip files that don't exist in game gfx

		if let (Some(ex1), Some(ex2)) = (path.extension(), destination.extension()) {
			if ex1 == "png" && ex2 == "png" {
				merge_two_images(&destination.as_path(), &path, &mut mod_indices, &mut all_indices)?;
				countmerged += 1;
			}
		}
	}

	let changed_indices = mod_indices.get_changed_indices();
	mod_indices.map_modded_indices(&mut all_indices);
	mod_indices.map_from_changed_indices_to_df_file(&changed_indices, index_file)?;

	Ok(countmerged)
}