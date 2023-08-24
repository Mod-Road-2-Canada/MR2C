use super::image_constant::{get_sprite_size_map};

use crate::errorwrap::{Error, ModScriptError};

fn extract_ranges(input: &str) -> Result<Vec<u32>, Error> {
	let mut result = Vec::new();
	
	for part in input.split(',') {
		let range_parts: Vec<&str> = part.split('-').collect();
		match range_parts.len() {
			1 => {
				if let Ok(num) = range_parts[0].parse::<u32>() {
					result.push(num);
				} else {
					return ModScriptError::ExtractRange(
						format!("Invalid positive number {}", range_parts[0])
					);
				}
			},
			2 => {
				if let Ok(start) = range_parts[0].parse::<u32>() {
					if let Ok(end) = range_parts[1].parse::<u32>() {
						if start <= end {
							result.extend(start..=end);
							continue;
						}
					}
				}
				
				return ModScriptError::ExtractRange(
					format!("Cannot parse range format {}", part)
				);
			},
			_ => return ModScriptError::ExtractRange(
				format!("Invalid range format {}", part)
			),
		}
	}
	
	Ok(result)
}

fn remove_duplicates(vec: &mut Vec<u32>) {
    let mut seen = HashSet::new();

    for value in vec.iter_mut() {
        if seen.contains(value) {
            vec.remove(value);
        } else {
            seen.insert(*value);
        }
    }
}

pub fn overlap_in_images(mod_img_path: &str, vanilla_img_path: &str, ranges: &str) -> Result<(), Error> {
	let van_img = image::open(&vanilla_img_path).with_context(|| format!("Failed to open img: {}", vanilla_img_path))?;
	let mod_img = image::open(&mod_img_path).with_context(|| format!("Failed to open img: {}", mod_img_path))?;

	if mod_img.width() != van_img.width() || mod_img.height() != van_img.height() {
		custombail!("Modded image {} is not same dimension with the original image {}", mod_img_path, vanilla_img_path);
	}

	let replace_range = extract_ranges(ranges)?;

	for pos in &replace_range {
		if let Some(dim) = get_sprite_size_map().get(search_name) {
			if dim[0] > 0 {

			} else {
				return ModImgError::SpriteMapNotFound(search_name);
			}
		} else {

		}

		let mod_img_view = mod_img.view();
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