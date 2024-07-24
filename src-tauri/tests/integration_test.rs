use std::path::PathBuf;
use app::core::image_merging::merge_all_img_to_gfx;


#[test]
fn test_merge_images() {
	// env::set_var("RUST_BACKTRACE", "1");
	let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

	let options = fs_extra::dir::CopyOptions::new().overwrite(true).depth(5);

	fs_extra::copy_items(&[base_path.join("tests/,moddedcompare/gfx")], base_path.join("tests"), &options).unwrap();

	let mod_gfx = base_path.join("tests/mods/testmod/gfx");
	let index_file = base_path.join("tests/mods/testmod/index.df").to_string_lossy().into_owned();
	let game_gfx = base_path.join("tests/gfx").to_string_lossy().into_owned();
	let mod_tag = "test_merge";

	let result = merge_all_img_to_gfx(mod_gfx, &game_gfx, &index_file, mod_tag);
	assert!(result.is_ok(), "Test failed: {}", result.err().unwrap());

	
}