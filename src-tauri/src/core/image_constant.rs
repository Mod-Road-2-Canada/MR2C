pub const SPRITE_TOTAL: usize = 70;

/// Contains loading order of a spritesheet and size of each sprite in that spritesheet
/// E.g. "dr2_zombie_bodies.png" is the 2nd spritesheet loaded and has size of each sprite is 24x24
pub fn sheet_to_idx_size(search: &str) -> Option<(usize, [u32; 2])> {
match search {
		//  gfx\fonts
	"font6x8" => Some((0, [0, 256])),		// [0, ...] Because it cannot be indexed. (each sprite has different size)

		// gfx\ui
	"dr2c_title_small" => Some((1, [0, 1])),

		// gfx\chars
	"dr2c_zombie_bodies"  => Some((2, [24, 24])),
	"dr2c_zombie_heads"   => Some((3, [24, 24])),
	"dr2c_skeletons_24px" => Some((4, [24, 24])),
	
		// head hair glass beard hat body
	"dr2c_male_heads"          => Some((5, [24, 24])),
	"dr2c_male_heads_extra"    => Some((6, [24, 24])),
	"dr2c_male_hair"           => Some((7, [24, 24])),
	"dr2c_male_hair_extra"     => Some((8, [24, 24])),
	"dr2c_male_glasses"        => Some((9, [24, 24])),
	"dr2c_male_glasses_extra"  => Some((10, [24, 24])),
	"dr2c_male_beards"         => Some((11, [24, 24])),
	"dr2c_male_beards_extra"   => Some((12, [24, 24])),
	"dr2c_male_hats"           => Some((13, [24, 24])),
	"dr2c_male_hats_extra"     => Some((14, [24, 24])),
	"dr2c_male_bodies"         => Some((15, [24, 24])),
	"dr2c_male_bodies_extra"   => Some((16, [24, 24])),

	"dr2c_female_heads"        => Some((17, [24, 24])),
	"dr2c_female_heads_extra"  => Some((18, [24, 24])),
	"dr2c_female_hair"         => Some((19, [24, 24])),
	"dr2c_female_hair_extra"   => Some((20, [24, 24])),
	"dr2c_female_glasses"      => Some((21, [24, 24])),
	"dr2c_female_glasses_extra"=> Some((22, [24, 24])),
	"dr2c_female_hats"         => Some((23, [24, 24])),
	"dr2c_female_hats_extra"   => Some((24, [24, 24])),
	"dr2c_female_bodies"       => Some((25, [24, 24])),
	"dr2c_female_bodies_extra" => Some((26, [24, 24])),

	"dr2c_special_chars"   => Some((27, [24, 24])),
	"dr2c_special_chars48" => Some((28, [48, 48])),

	"superpets24" => Some((29, [24, 24])),
	"pets24" => Some((30, [24, 24])),
	"pets48" => Some((31, [48, 48])),
	// "hairpalette" => Some((?, [1, 1])), // ?
	// "skinpalette" => Some((?, [1, 1])), // ?

		// gfx\items
	"dr2c_weapons"         => Some((32, [8, 32])),
	"dr2c_weapons_16x32px" => Some((33, [16, 32])),
	"dr2c_loot16"          => Some((34, [16, 16])),

	"dr2c_cityfurniture_16px"  => Some((35, [16, 16])),
	"dr2c_housefurniture_32px" => Some((36, [32, 32])),
	// "dr2c_cityfurniture_32px" => Some((?, [32, 32])),   // ?? Need verification
	// "trash16" => Some((?, [16, 16])),   // ??
	// "trash32" => Some((?, [32, 32])),   // ??

		// gfx\tiles
	"buildings"      => Some((37, [16, 16])),
	"city_generic"   => Some((38, [16, 16])),
	"city_florida"   => Some((39, [16, 16])),
	"city_carolina"  => Some((40, [16, 16])),
	"houses"         => Some((41, [16, 16])),
	"buildings_misc" => Some((42, [16, 16])),

	"street"         => Some((43, [16, 16])),
	"walls"          => Some((44, [16, 16])),
	"floors"         => Some((45, [16, 16])),
	"doors"          => Some((46, [16, 16])),
	"shelves"        => Some((47, [16, 16])),

	// m.insert("custom" => Some((? ,[16, 16])),        // ??

		// gfx\items
	"special16" => Some((48, [16, 16])),
	"special32" => Some((49, [32, 32])),
	"special48" => Some((50, [48, 48])),
	"special64" => Some((51, [64, 64])),
	"special96" => Some((52, [96, 96])),

		// gfx\misc
	"dr2c_particles"   => Some((53, [16, 16])),
	"dr2c_particles32" => Some((54, [32, 32])),
	"icons16x16"       => Some((55, [16, 16])),
	"dr2c_grain64x64"  => Some((56, [64, 64])),
	"eventcards80x45"  => Some((57, [80, 45])),

		// gfx\road
	"dr2c_roadview_florida_64px"    => Some((58, [64, 64])),
	"dr2c_roadview_carolina_64px"   => Some((59, [64, 64])),
	"dr2c_roadview_virginia_64px"   => Some((60, [64, 64])),
	"dr2c_roadview_all_layers_64px" => Some((61, [64, 64])),
	"dr2c_roadview_canada_64px"     => Some((62, [64, 64])),
	"trees_96px" => Some((63, [96, 96])),
	// "dr2c_roadview_otherstuff_16px" => Some((?, [16, 16])), // ??

		// gfx\cars
	"dr2c_cars110x96" => Some((64, [110, 96])),
	// "cars_unique_110x96" => Some((?, [110, 96])),   // NOT LOADED
	"car-wrecks124x96" => Some((65, [124, 96])),

		// gfx\misc
	"rocketcat" => Some((66, [0, 1])),
	"gnomes"    => Some((67, [64, 64])),
	"skeletonbits" => Some((68, [48, 48])),

		// gfx\ui
	"madgarden" => Some((69, [0, 1])),   // final 70
	// END OF DEFINITION
	_ => None
}
}

pub const IDX_TO_SHEET: [&str; SPRITE_TOTAL] = [
	"font6x8",
	"dr2c_title_small",
	"dr2c_zombie_bodies" ,
	"dr2c_zombie_heads"  ,
	"dr2c_skeletons_24px",
	"dr2c_male_heads"         ,
	"dr2c_male_heads_extra"   ,
	"dr2c_male_hair"          ,
	"dr2c_male_hair_extra"    ,
	"dr2c_male_glasses"       ,
	"dr2c_male_glasses_extra" ,
	"dr2c_male_beards"        ,
	"dr2c_male_beards_extra"  ,
	"dr2c_male_hats"          ,
	"dr2c_male_hats_extra"    ,
	"dr2c_male_bodies"        ,
	"dr2c_male_bodies_extra"  ,
	"dr2c_female_heads"       ,
	"dr2c_female_heads_extra" ,
	"dr2c_female_hair"        ,
	"dr2c_female_hair_extra"  ,
	"dr2c_female_glasses"     ,
	"dr2c_female_glasses_extra",
	"dr2c_female_hats"        ,
	"dr2c_female_hats_extra"  ,
	"dr2c_female_bodies"      ,
	"dr2c_female_bodies_extra",
	"dr2c_special_chars",
	"dr2c_special_chars48",
	"superpets24",
	"pets24",
	"pets48",
	"dr2c_weapons"        ,
	"dr2c_weapons_16x32px",
	"dr2c_loot16"         ,
	"dr2c_cityfurniture_16px" ,
	"dr2c_housefurniture_32px",
	"buildings"     ,
	"city_generic"  ,
	"city_florida"  ,
	"city_carolina" ,
	"houses"        ,
	"buildings_misc",
	"street"        ,
	"walls"         ,
	"floors"        ,
	"doors"         ,
	"shelves"       ,
	"special16",
	"special32",
	"special48",
	"special64",
	"special96",
	"dr2c_particles"  ,
	"dr2c_particles32",
	"icons16x16"      ,
	"dr2c_grain64x64" ,
	"eventcards80x45" ,
	"dr2c_roadview_florida_64px"   ,
	"dr2c_roadview_carolina_64px"  ,
	"dr2c_roadview_virginia_64px"  ,
	"dr2c_roadview_all_layers_64px",
	"dr2c_roadview_canada_64px"    ,
	"trees_96px",
	"dr2c_cars110x96",
	"car-wrecks124x96",
	"rocketcat",
	"gnomes"   ,
	"skeletonbits",
	"madgarden"
];


// Special files are the files that special words are based on.
// E.g. .specialbody! .zombiehead!
// 		dr2c_special_chars has 7 sprites each unit (.specialbody! .specialtype!)
//		dr2c_zombie_heads has 3 sprites each head (.zombiehead!)
pub fn match_special_file(search: &str, sprite_count: u32) -> Option<(String, u32)>  {
	let mut calc_offset: u32 = 0;
	let special_name: &str = match search {
		"dr2c_special_chars" => { calc_offset = sprite_count/7+1; "MOD_SC" },	// Specialtype count starts at 1;
		"dr2c_zombie_bodies" => { calc_offset = sprite_count/5; "MOD_ZBODY" } ,
		"dr2c_zombie_heads"  => { calc_offset = sprite_count/3; "MOD_ZHEAD" },
		_ => ""
	};

	if !special_name.is_empty() {
		println!("SPECIAL: {}: {} ~ {}", special_name, calc_offset, sprite_count);
		return Some((special_name.to_string(), calc_offset));
	} else {
		return None;
	}
}

// use indexmap::IndexMap;
// use once_cell::sync::Lazy; // Import Lazy

// // Define your static variable using Lazy
// // This map is used for getting the size of each sprite in a whole spritesheet
// //   [width, height]
// // OR
// //   [0, sprite_count]
// static SPRITE_NAME_SIZE_MAP: Lazy<IndexMap<&'static str, [u32; 2]>> = Lazy::new(|| {
// 	let mut map = IndexMap::new();

// 	// NOTE: THIS IS ALSO ORDER OF LOADING SPRITES IN THE GAME. 
// 	// DO NOT CHANGE THE ORDER!

// 		//  gfx\fonts
// 	map.insert("font6x8", [0, 256]);

// 		// gfx\ui
// 	map.insert("dr2c_title_small", [0, 1]);

// 		// gfx\chars
// 	map.insert("dr2c_zombie_bodies",  [24, 24]);
// 	map.insert("dr2c_zombie_heads",   [24, 24]);
// 	map.insert("dr2c_skeletons_24px", [24, 24]);

// 		// head hair glass beard hat body
// 	map.insert("dr2c_male_heads",          [24, 24]);
// 	map.insert("dr2c_male_heads_extra",    [24, 24]);
// 	map.insert("dr2c_male_hair",           [24, 24]);
// 	map.insert("dr2c_male_hair_extra",     [24, 24]);
// 	map.insert("dr2c_male_glasses",        [24, 24]);
// 	map.insert("dr2c_male_glasses_extra",  [24, 24]);
// 	map.insert("dr2c_male_beards",         [24, 24]);
// 	map.insert("dr2c_male_beards_extra",   [24, 24]);
// 	map.insert("dr2c_male_hats",           [24, 24]);
// 	map.insert("dr2c_male_hats_extra",     [24, 24]);
// 	map.insert("dr2c_male_bodies",         [24, 24]);
// 	map.insert("dr2c_male_bodies_extra",   [24, 24]);

// 	map.insert("dr2c_female_heads",        [24, 24]);
// 	map.insert("dr2c_female_heads_extra",  [24, 24]);
// 	map.insert("dr2c_female_hair",         [24, 24]);
// 	map.insert("dr2c_female_hair_extra",   [24, 24]);
// 	map.insert("dr2c_female_glasses",      [24, 24]);
// 	map.insert("dr2c_female_glasses_extra",[24, 24]);
// 	map.insert("dr2c_female_hats",         [24, 24]);
// 	map.insert("dr2c_female_hats_extra",   [24, 24]);
// 	map.insert("dr2c_female_bodies",       [24, 24]);
// 	map.insert("dr2c_female_bodies_extra", [24, 24]);

// 	map.insert("dr2c_special_chars", [24, 24]);
// 	map.insert("dr2c_special_chars48", [48, 48]);

// 	map.insert("superpets24", [24, 24]);
// 	map.insert("pets24", [24, 24]);
// 	map.insert("pets48", [48, 48]);
// 	// m.insert("hairpalette", [1, 1]); // ?
// 	// m.insert("skinpalette", [1, 1]); // ?

// 		// gfx\items
// 	map.insert("dr2c_weapons", [8, 32]);
// 	map.insert("dr2c_weapons_16x32px", [16, 32]);
// 	map.insert("dr2c_loot16", [16, 16]);

// 	map.insert("dr2c_cityfurniture_16px", [16, 16]);
// 	map.insert("dr2c_housefurniture_32px", [32, 32]);
// 	// m.insert("dr2c_cityfurniture_32px", [32, 32]);   // ?? Need verification
// 	// m.insert("trash16", [16, 16]);   // ??
// 	// m.insert("trash32", [32, 32]);   // ??

// 		// gfx\tiles
// 	map.insert("buildings",      [16, 16]);
// 	map.insert("city_generic",   [16, 16]);
// 	map.insert("city_florida",   [16, 16]);
// 	map.insert("city_carolina",  [16, 16]);
// 	map.insert("houses",         [16, 16]);
// 	map.insert("buildings_misc", [16, 16]);

// 	map.insert("street",         [16, 16]);
// 	map.insert("walls",          [16, 16]);
// 	map.insert("floors",         [16, 16]);
// 	map.insert("doors",          [16, 16]);
// 	map.insert("shelves",        [16, 16]);

// 	// m.insert("custom", [16, 16]);        // ??

// 		// gfx\items
// 	map.insert("special16", [16, 16]);
// 	map.insert("special32", [32, 32]);
// 	map.insert("special48", [48, 48]);
// 	map.insert("special64", [64, 64]);
// 	map.insert("special96", [96, 96]);

// 		// gfx\misc
// 	map.insert("dr2c_particles",   [16, 16]);
// 	map.insert("dr2c_particles32", [32, 32]);
// 	map.insert("icons16x16", [16, 16]);
// 	map.insert("dr2c_grain64x64", [64, 64]);
// 	map.insert("eventcards80x45", [80, 45]);

// 		// gfx\road
// 	map.insert("dr2c_roadview_florida_64px",    [64, 64]);
// 	map.insert("dr2c_roadview_carolina_64px",   [64, 64]);
// 	map.insert("dr2c_roadview_virginia_64px",   [64, 64]);
// 	map.insert("dr2c_roadview_all_layers_64px", [64, 64]);
// 	map.insert("dr2c_roadview_canada_64px",     [64, 64]);
// 	map.insert("trees_96px", [96, 96]);
// 	// m.insert("dr2c_roadview_otherstuff_16px", [16, 16]); // ??

// 		// gfx\cars
// 	map.insert("dr2c_cars110x96", [110, 96]);
// 	// m.insert("cars_unique_110x96", [110, 96]);   // NOT LOADED
// 	map.insert("car-wrecks124x96", [124, 96]);

// 		// gfx\misc
// 	map.insert("rocketcat", [0, 1]);
// 	map.insert("gnomes", [64, 64]);
// 	map.insert("skeletonbits", [48, 48]);

// 		// gfx\ui
// 	map.insert("madgarden", [0, 1]);   // final 70
// 	// END OF DEFINITION

// 	map
// });

// // Export the index map for use in other files
// pub fn summon_sprite_size_map() -> &'static IndexMap<&'static str, [u32; 2]> {
// 	&*SPRITE_NAME_SIZE_MAP
// }



