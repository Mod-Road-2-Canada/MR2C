use indexmap::IndexMap;
use once_cell::sync::Lazy; // Import Lazy

pub const SPRITE_TOTAL: usize = 70;

// Define your static variable using Lazy
// This map is used for getting the size of each sprite in a whole spritesheet
//   [width, height]
// OR
//   [0, sprite_count]
static SPRITE_NAME_SIZE_MAP: Lazy<IndexMap<&'static str, [u32; 2]>> = Lazy::new(|| {
	let mut map = IndexMap::new();

	// NOTE: THIS IS ALSO ORDER OF LOADING SPRITES IN THE GAME. 
	// DO NOT CHANGE THE ORDER!

		//  gfx\fonts
	map.insert("font6x8.png", [0, 256]);

		// gfx\ui
	map.insert("dr2c_title_small.png", [0, 1]);

		// gfx\chars
	map.insert("dr2c_zombie_bodies.png",  [24, 24]);
	map.insert("dr2c_zombie_heads.png",   [24, 24]);
	map.insert("dr2c_skeletons_24px.png", [24, 24]);

		// head hair glass beard hat body
	map.insert("dr2c_male_heads.png",          [24, 24]);
	map.insert("dr2c_male_heads_extra.png",    [24, 24]);
	map.insert("dr2c_male_hair.png",           [24, 24]);
	map.insert("dr2c_male_hair_extra.png",     [24, 24]);
	map.insert("dr2c_male_glasses.png",        [24, 24]);
	map.insert("dr2c_male_glasses_extra.png",  [24, 24]);
	map.insert("dr2c_male_beards.png",         [24, 24]);
	map.insert("dr2c_male_beards_extra.png",   [24, 24]);
	map.insert("dr2c_male_hats.png",           [24, 24]);
	map.insert("dr2c_male_hats_extra.png",     [24, 24]);
	map.insert("dr2c_male_bodies.png",         [24, 24]);
	map.insert("dr2c_male_bodies_extra.png",   [24, 24]);

	map.insert("dr2c_female_heads.png",        [24, 24]);
	map.insert("dr2c_female_heads_extra.png",  [24, 24]);
	map.insert("dr2c_female_hair.png",         [24, 24]);
	map.insert("dr2c_female_hair_extra.png",   [24, 24]);
	map.insert("dr2c_female_glasses.png",      [24, 24]);
	map.insert("dr2c_female_glasses_extra.png",[24, 24]);
	map.insert("dr2c_female_hats.png",         [24, 24]);
	map.insert("dr2c_female_hats_extra.png",   [24, 24]);
	map.insert("dr2c_female_bodies.png",       [24, 24]);
	map.insert("dr2c_female_bodies_extra.png", [24, 24]);

	map.insert("dr2c_special_chars.png", [24, 24]);
	map.insert("dr2c_special_chars48.png", [48, 48]);

	map.insert("superpets24.png", [24, 24]);
	map.insert("pets24.png", [24, 24]);
	map.insert("pets48.png", [48, 48]);
	// m.insert("hairpalette.png", [1, 1]); // ?
	// m.insert("skinpalette.png", [1, 1]); // ?

		// gfx\items
	map.insert("dr2c_weapons.png", [8, 32]);
	map.insert("dr2c_weapons_16x32px.png", [16, 32]);
	map.insert("dr2c_loot16.png", [16, 16]);

	map.insert("dr2c_cityfurniture_16px.png", [16, 16]);
	map.insert("dr2c_housefurniture_32px.png", [32, 32]);
	// m.insert("dr2c_cityfurniture_32px.png", [32, 32]);   // ?? Need verification
	// m.insert("trash16.png", [16, 16]);   // ??
	// m.insert("trash32.png", [32, 32]);   // ??

		// gfx\tiles
	map.insert("buildings.png",      [16, 16]);
	map.insert("city_generic.png",   [16, 16]);
	map.insert("city_florida.png",   [16, 16]);
	map.insert("city_carolina.png",  [16, 16]);
	map.insert("houses.png",         [16, 16]);
	map.insert("buildings_misc.png", [16, 16]);

	map.insert("street.png",         [16, 16]);
	map.insert("walls.png",          [16, 16]);
	map.insert("floors.png",         [16, 16]);
	map.insert("doors.png",          [16, 16]);
	map.insert("shelves.png",        [16, 16]);

	// m.insert("custom.png", [16, 16]);        // ??

		// gfx\items
	map.insert("special16.png", [16, 16]);
	map.insert("special32.png", [32, 32]);
	map.insert("special48.png", [48, 48]);
	map.insert("special64.png", [64, 64]);
	map.insert("special96.png", [96, 96]);

		// gfx\misc
	map.insert("dr2c_particles.png",   [16, 16]);
	map.insert("dr2c_particles32.png", [32, 32]);
	map.insert("icons16x16.png", [16, 16]);
	map.insert("dr2c_grain64x64.png", [64, 64]);
	map.insert("eventcards80x45.png", [80, 45]);

		// gfx\road
	map.insert("dr2c_roadview_florida_64px.png",    [64, 64]);
	map.insert("dr2c_roadview_carolina_64px.png",   [64, 64]);
	map.insert("dr2c_roadview_virginia_64px.png",   [64, 64]);
	map.insert("dr2c_roadview_all_layers_64px.png", [64, 64]);
	map.insert("dr2c_roadview_canada_64px.png",     [64, 64]);
	map.insert("trees_96px.png", [96, 96]);
	// m.insert("dr2c_roadview_otherstuff_16px.png", [16, 16]); // ??

		// gfx\cars
	map.insert("dr2c_cars110x96.png", [110, 96]);
	// m.insert("cars_unique_110x96.png", [110, 96]);   // NOT LOADED
	map.insert("car-wrecks124x96.png", [124, 96]);

		// gfx\misc
	map.insert("rocketcat.png", [0, 1]);
	map.insert("gnomes.png", [64, 64]);
	map.insert("skeletonbits.png", [48, 48]);

		// gfx\ui
	map.insert("madgarden.png", [0, 1]);   // final 70
	// END OF DEFINITION

	map
});

// Export the index map for use in other files
pub fn summon_sprite_size_map() -> &'static IndexMap<&'static str, [u32; 2]> {
	&*SPRITE_NAME_SIZE_MAP
}



