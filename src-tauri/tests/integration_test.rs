use std::path::{PathBuf, Path};
use app::core::image_merging::merge_all_img_to_gfx;


fn assert_two(path1: &Path, path2: &Path, exclude: &Vec<String>) {
	let result = folder_compare::FolderCompare::new(path1, path2, exclude).unwrap();

	assert!(result.changed_files.is_empty(), "\nDifferent files: [\n{}\n]", 
		result.changed_files.into_iter().map(|x| x.to_string_lossy().into_owned())
       .collect::<Vec<String>>()
       .join("\n\t"));

	assert!(result.new_files.is_empty(), "\nNew files: [\n{}\n]", 
		result.new_files.into_iter().map(|x| x.to_string_lossy().into_owned())
       .collect::<Vec<String>>()
       .join("\n\t"));
}

#[test]
fn test_merge_images() {
	// env::set_var("RUST_BACKTRACE", "1");
	let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

	let options = fs_extra::dir::CopyOptions::new().overwrite(true).depth(5);

	fs_extra::copy_items(&[base_path.join("tests/,moddedcompare/gfx")], base_path.join("tests"), &options).unwrap();

	let mod_gfx = base_path.join("tests/mods/testmod/gfx");
	let game_gfx = base_path.join("tests/gfx");
	let index_file = base_path.join("tests/mods/testmod/index.df");
	let mod_tag = "test_merge";

	merge_all_img_to_gfx(mod_gfx, &game_gfx.to_string_lossy().into_owned(), &index_file.to_string_lossy().into_owned(), mod_tag).unwrap();

	let expected_game_gfx = base_path.join("tests/,moddedcompare/expected/gfx");

	assert_two(&game_gfx, &expected_game_gfx, &Vec::new());
	assert_two(&base_path.join("tests/mods/testmod/"), &base_path.join("tests/,moddedcompare/expected/"), &vec![".png".to_string(), ".dfmod".to_string()]);
}