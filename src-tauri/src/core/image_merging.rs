use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use walkdir::WalkDir;

use anyhow::Context;		// with_context
use image::GenericImage;	// buffer.copy_from

use crate::errorwrap::{Error, ModImgError};
use super::image_constant::{SPRITE_TOTAL, IDX_TO_SHEET, sheet_to_idx_size, match_special_file};


type SpriteCountArray = [u32; SPRITE_TOTAL];

#[derive(Debug)]
struct Indices {
	sprite_count: SpriteCountArray,
}

impl Indices {
	fn new() -> Self {
		Indices { 
			sprite_count: [0; SPRITE_TOTAL],
		}
	}

	fn map_size_to_index(&mut self, search_png: &str, total_area: u32) -> Result<(), Error> {
		if let Some((i, dim)) = sheet_to_idx_size(search_png) {
			if dim[0] > 0 {
				let single_sprite_size = dim[0] * dim[1];
				self.sprite_count[i] = total_area / single_sprite_size;
			} else {
				// When the first dimension is 1, that means that the whole spritesheet can't be divide (such as the font.png) 
				// TODO: Put a warning here that they're editing something that can't be indexed
				self.sprite_count[i] = 1;
			}
		} else {
			return Err(Error::ModImgError(
				ModImgError::SpriteMapNotFound(search_png.to_string())
			));
		}

		Ok(())
	}




	fn has_reached_max(&self) -> Result<(), Error> {
		let last_index = self.sprite_count.len() - 1; 
		if self.sprite_count[last_index] > 15200 {
			return Err(Error::ModImgError(
				ModImgError::TooManySprites(self.sprite_count[last_index])
			));
		}

		Ok(())
	}

	fn map_default(&mut self, dir: &str) -> Result<(), Error>{
		for entry in WalkDir::new(dir) {
			let entry = entry?;
			let path = entry.path();

			if let Some(ex1) = path.extension() {
				if ex1 == "png" {
					if let Some(filename) = path.file_stem() {
						let filename = filename.to_string_lossy().into_owned();

						let img = image::open(&path).with_context(|| format!("Mapping: Failed to open img 1: {}", path.display()))?;

						let width = img.width();
						let height = img.height();

						match self.map_size_to_index(&filename, width * height) {
							// Do nothing if error since some of the default sprite doesn't load
							Err(err) => println!("Not loaded: {}", err),
							_ => ()
						}
						// !!! No error here?
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
	println!("Height: {} > {}", height, img2.height());

	Ok((width, height, img2.height()))
}

fn merge_all_imgs_in_folder<P: AsRef<Path>>(mod_gfx: P, game_gfx: &str, mod_index: &mut Indices, all_index: &mut Indices) -> Result<u16, Error> {
	let mut countmerged: u16 = 0;

	for entry in WalkDir::new(&mod_gfx) {
		let entry = entry?;
		let path = entry.path();
		let destination = Path::new(game_gfx).join(path.strip_prefix(&mod_gfx)?);

		if !destination.try_exists()? { continue;	}	// Skip files that don't exist in game gfx

		if let (Some(ex1), Some(ex2)) = (path.extension(), destination.extension()) {
			if ex1 == "png" && ex2 == "png" {
				let (width, total_height, mod_height) = merge_two_images(&destination.as_path(), &path)?;

				let png_name = destination.file_stem().ok_or(Error::Other("2. Cannot get file name (without extension).".to_string()))?.to_string_lossy().into_owned();

				mod_index.map_size_to_index(&png_name, width * mod_height)?;
				all_index.map_size_to_index(&png_name, width * total_height)?;

				countmerged += 1;
			}
		}
	}

	Ok(countmerged)
}


// !!! Mathematical documentation comes later
fn calc_indices(modded: &SpriteCountArray, all: &SpriteCountArray) -> Vec<(String, u32)> {
	let mut result: Vec<(String, u32)> = Vec::new();

	let mut cumu: SpriteCountArray = [0; SPRITE_TOTAL];
	cumu[0] = all[0];

	for i in 1..SPRITE_TOTAL {
		// Cumulative sum index
		cumu[i] = cumu[i-1] + all[i];

		if modded[i] > 0 {
			// If there's new file
			let idxname = IDX_TO_SHEET[i];
			result.push((idxname.to_string(), cumu[i] - modded[i]));

			if let Some((newname, calc_offset)) = match_special_file(idxname, all[i] - modded[i]) {
				result.push((newname, calc_offset));
			}			
		}
	}	

	result
}

fn export_to_df(named_indices: &Vec<(String, u32)>, output_file: &str, mod_tag: &str) -> Result<(), Error> {
	let file = File::create(output_file)?;
	let mut writer = BufWriter::new(file);

	for (name, value) in named_indices.iter() {
		let line = format!("{} constant {}.{}\n", value, mod_tag, name);
		writer.write_all(line.as_bytes())?;
	}

	writer.flush()?;

	Ok(())
}


pub fn merge_all_img_to_gfx<P: AsRef<Path>>(mod_gfx: P, game_gfx: &str, index_file: &str, mod_tag: &str) -> Result<u16, Error> {
	if !mod_gfx.as_ref().is_dir() { custombail!("Not a directory: {}", mod_gfx.as_ref().display()); }

	let mut mod_index = Indices::new();
	let mut all_index = Indices::new();
	all_index.map_default(game_gfx)?;	// Fill empty index first with the current game

	let countmerged = merge_all_imgs_in_folder(mod_gfx, game_gfx, &mut mod_index, &mut all_index)?;
	
	let toexport = calc_indices(&mod_index.sprite_count, &all_index.sprite_count);

	export_to_df(&toexport, index_file, mod_tag)?;

	// Add a final warning if too many sprites loaded (the game might not work correctly)
	all_index.has_reached_max()?;

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