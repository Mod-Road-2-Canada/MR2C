use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use walkdir::WalkDir;

use anyhow::Context;		// with_context
use image::GenericImage;	// buffer.copy_from

use crate::errorwrap::Error;

const SPRITE_TOTAL: usize = 70;

const SPRITE_SIZE_NAME_ARRAY: [([u32; 2], &str); SPRITE_TOTAL] = [
		// NOTE: THIS IS ALSO ORDER OF LOADING SPRITES IN THE GAME. DO NOT CHANGE!
		//  gfx\fonts
	([0, 256], "font6x8.png"),

		// gfx\ui
	([0, 1], "dr2c_title_small.png"),

		// gfx\chars
	([24, 24], "dr2c_zombie_bodies.png"),
	([24, 24], "dr2c_zombie_heads.png"),
	([24, 24], "dr2c_skeletons_24px.png"),

		// head hair glass beard hat body
	([24, 24], "dr2c_male_heads.png"),
	([24, 24], "dr2c_male_heads_extra.png"),
	([24, 24], "dr2c_male_hair.png"),
	([24, 24], "dr2c_male_hair_extra.png"),
	([24, 24], "dr2c_male_glasses.png"),
	([24, 24], "dr2c_male_glasses_extra.png"),
	([24, 24], "dr2c_male_beards.png"),
	([24, 24], "dr2c_male_beards_extra.png"),
	([24, 24], "dr2c_male_hats.png"),
	([24, 24], "dr2c_male_hats_extra.png"),
	([24, 24], "dr2c_male_bodies.png"),
	([24, 24], "dr2c_male_bodies_extra.png"),

	([24, 24], "dr2c_female_heads.png"),
	([24, 24], "dr2c_female_heads_extra.png"),
	([24, 24], "dr2c_female_hair.png"),
	([24, 24], "dr2c_female_hair_extra.png"),
	([24, 24], "dr2c_female_glasses.png"),
	([24, 24], "dr2c_female_glasses_extra.png"),
	([24, 24], "dr2c_female_hats.png"),
	([24, 24], "dr2c_female_hats_extra.png"),
	([24, 24], "dr2c_female_bodies.png"),
	([24, 24], "dr2c_female_bodies_extra.png"),

	([24, 24], "dr2c_special_chars.png"),
	([48, 48], "dr2c_special_chars48.png"),

	([24, 24], "superpets24.png"),
	([24, 24], "pets24.png"),
	([48, 48], "pets48.png"),
	// ([1, 1], "hairpalette.png"),	// ?
	// ([1, 1], "skinpalette.png"),	// ?

		// gfx\items
	([8, 32], "dr2c_weapons.png"),
	([16, 32], "dr2c_weapons_16x32px.png"),
	([16, 16], "dr2c_loot16.png"),

	([16, 16], "dr2c_cityfurniture_16px.png"),
	([32, 32], "dr2c_housefurniture_32px.png"),
	// ([32, 32], "dr2c_cityfurniture_32px.png"),	// ?? Need verification
	// ([16, 16], "trash16.png"),	// ??
	// ([32, 32], "trash32.png"),	// ??

		// gfx\tiles
	([16, 16], "buildings.png"),
	([16, 16], "city_generic.png"),
	([16, 16], "city_florida.png"),
	([16, 16], "city_carolina.png"),
	([16, 16], "houses.png"),
	([16, 16], "buildings_misc.png"),

	([16, 16], "street.png"),
	([16, 16], "walls.png"),
	([16, 16], "floors.png"),
	([16, 16], "doors.png"),
	([16, 16], "shelves.png"),

	// ([16, 16], "custom.png"),		// ??

		// gfx\items
	([16, 16], "special16.png"),
	([32, 32], "special32.png"),
	([48, 48], "special48.png"),
	([64, 64], "special64.png"),
	([96, 96], "special96.png"),

		// gfx\misc
	([16, 16], "dr2c_particles.png"),
	([32, 32], "dr2c_particles32.png"),
	([16, 16], "icons16x16.png"),
	([64, 64], "dr2c_grain64x64.png"),
	([80, 45], "eventcards80x45.png"),

		// gfx\road
	([64, 64], "dr2c_roadview_florida_64px.png"),
	([64, 64], "dr2c_roadview_carolina_64px.png"),
	([64, 64], "dr2c_roadview_virginia_64px.png"),
	([64, 64], "dr2c_roadview_all_layers_64px.png"),
	([64, 64], "dr2c_roadview_canada_64px.png"),
	([96, 96], "trees_96px.png"),
	// ([16, 16], "dr2c_roadview_otherstuff_16px.png"),	// ??

		// gfx\cars
	([110, 96], "dr2c_cars110x96.png"),
	// ([110, 96], "cars_unique_110x96.png"),	// NOT LOADED
	([124, 96], "car-wrecks124x96.png"),

		// gfx\misc
	([0, 1], "rocketcat.png"),
	([64, 64], "gnomes.png"),
	([48, 48], "skeletonbits.png"),

		// gfx\ui
	([0, 1], "madgarden.png")	// final 70
];



#[derive(Debug)]
struct Indices {
	sprite_count: [u32; SPRITE_TOTAL],
}

impl Indices {
	fn new() -> Self {
		Indices { 
			sprite_count: [0; SPRITE_TOTAL],
		}
	}

	fn map_file_to_indices(&mut self, search_name: &str, total_area: u32) -> () /* Result<(), Error> */ {
		if let Some((index, (dim, _))) = SPRITE_SIZE_NAME_ARRAY.iter().enumerate().find(|(_, (_, sprite_name))| sprite_name == &search_name) {
			if dim[0] > 0 {
				let single_sprite_size = dim[0] * dim[1];
				self.sprite_count[index] = total_area / single_sprite_size;
			} else {
				// The whole spritesheet counts as 1 / special count can't divide (font) (USUALLY CANNOT BE CHANGED)
				// TODO: Put a warning here that they're editing something that can't be indexed
				self.sprite_count[index] = 1;
			}
		} else {
			// TODO: WARNING
			println!("Value not found {}", search_name);
		}

		()
	}

	fn get_changed_spritesheets(&mut self) -> Vec<usize> {
		(1..self.sprite_count.len()).filter(|&i| self.sprite_count[i] != 0).collect()
	}

	fn accumulate_other_to_me(&mut self, other: &mut Indices) {
		for i in 2..self.sprite_count.len() {
			self.sprite_count[i] += other.sprite_count[i-1];
			other.sprite_count[i] += other.sprite_count[i-1];
		}
	}

	fn print_filtered_to_df_file(&self, filtered: &Vec<usize> , output_file: &str) -> Result<(), Error> {
		let file = File::create(output_file)?;
		let mut writer = BufWriter::new(file);

		for i in filtered.iter() {
			let (_, sprite_name) = SPRITE_SIZE_NAME_ARRAY[*i];
			let line = format!("{} constant {}\n", self.sprite_count[*i], sprite_name);
			writer.write_all(line.as_bytes())?;
		}

		writer.flush()?;

		// Add a final warning
		let last_index = self.sprite_count.len() - 1; 
		if self.sprite_count[last_index] > 15200 {
			custombail!("WARNING: TOO MANY SPRITES {} (GAME MIGHT BREAK)", self.sprite_count[last_index]);
		}

		Ok(())
	}

	fn map_default(&mut self, dir: &str) -> Result<(), Error>{
		for entry in WalkDir::new(dir) {
			let entry = entry?;
			let path = entry.path();

			if let Some(ex1) = path.extension() {
				if ex1 == "png" {
					if let Some(file_name) = path.file_name() {
						let file_name = file_name.to_string_lossy().into_owned();
						let img = image::open(&path).with_context(|| format!("Mapping: Failed to open img 1: {}", path.display()))?;

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


fn merge_two_images<P: AsRef<Path>>(img1_path: P, img2_path: P) 
	-> Result<(u32, u32, u32), Error> {

	let img1 = image::open(&img1_path).with_context(|| format!("MFailed to open img 1: {}", img1_path.as_ref().display()))?;
	let img2 = image::open(&img2_path).with_context(|| format!("MFailed to open img 2: {}", img2_path.as_ref().display()))?;
	
	let width = std::cmp::max(img1.width(), img2.width());
	let height = img1.height() + img2.height();

	let mut buffer = image::ImageBuffer::new(width, height);

	buffer.copy_from(&img1, 0, 0)?;
	buffer.copy_from(&img2, 0, img1.height())?;
	buffer.save(&img1_path).with_context(|| format!("Failed to save imgs"))?;
	// image::save_buffer(&img1_path, &vec1, width, height, img1.color()).unwrap();

	println!("Merged {} to: {}", img2_path.as_ref().display(), img1_path.as_ref().display());

	Ok((width, img1.height(), img2.height()))
}



pub fn merge_all_img_to_gfx<P: AsRef<Path>>(mod_gfx: P, game_gfx: &str, backup_gfx: &str, index_file: &str) -> Result<u16, Error> {
	let mut countmerged: u16 = 0;

	if !mod_gfx.as_ref().is_dir() { custombail!("Not a directory: {}", mod_gfx.as_ref().display()); }

	let mut mod_indices = Indices::new();
	let mut all_indices = Indices::new();
	all_indices.map_default(backup_gfx)?;	// Fill empty index first with vanilla

	for entry in WalkDir::new(&mod_gfx) {
		let entry = entry?;
		let path = entry.path();
		let destination = Path::new(game_gfx).join(path.strip_prefix(&mod_gfx)?);

		if !destination.try_exists()? { continue;	}	// Skip files that don't exist in game gfx

		if let (Some(ex1), Some(ex2)) = (path.extension(), destination.extension()) {
			if ex1 == "png" && ex2 == "png" {
				let (width, mod_start_height, mod_end_height) = merge_two_images(&destination.as_path(), &path)?;

				let png_name = destination.file_name().ok_or(Error::Other("Cannot get file name.".to_string()))?.to_string_lossy().into_owned();

				mod_indices.map_file_to_indices(&png_name, width * mod_start_height);
				all_indices.map_file_to_indices(&png_name, width * mod_end_height);

				countmerged += 1;
			}
		}
	}

	let filtered_sheets = mod_indices.get_changed_spritesheets();
	mod_indices.accumulate_other_to_me(&mut all_indices);

	mod_indices.print_filtered_to_df_file(&filtered_sheets, index_file)?;

	Ok(countmerged)
}



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