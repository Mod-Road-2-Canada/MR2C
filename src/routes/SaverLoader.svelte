<script lang='ts'>
	import { GFX_FOLDER, MODS, THEME, COOKIES_LOADED } from '$lib/stores';
	import { COOKIE_FILE, MOD_FOLDER } from '$lib/consts';

	import type { ModInfo, ModCookies } from '$lib/types';
	import { ModStatus } from '$lib/types';

	import { invoke } from '@tauri-apps/api/tauri';
	import toast from 'svelte-french-toast';


	function validateJson(jsonString: string, fileName: string): ModInfo | null {
		const parsed = JSON.parse(jsonString);

		if (!parsed.path) {
			toast.error("JSON: Mod with no path: " + fileName);
			return null;
		}

		if (!parsed.version) {
			toast.error("JSON: Mod with no version: " + fileName);
			return null;
		}

		if (!parsed.name) {
			toast.error("JSON: Mod with no name: '" + fileName + "'");
			parsed.name = fileName;
		}

		if (!parsed.tag) {
			parsed.tag = fileName;
		}

		if (!parsed.creator) {
			toast.error("JSON: Mod with no creator: '" + fileName + "'");
			parsed.creator = "unknown";
		}

		return {
			name: parsed.name,
			tag: parsed.tag,
			path: parsed.path,
			version: parsed.version,
			creator: parsed.creator,
			description: parsed.description,

			checked: false,
			status: ModStatus.Normal
		}
	}

	function processJsons (modRawArray: string[]): ModInfo[] {
		let tempMods: ModInfo[] = [];
		let existmod: ModInfo;

		for (var i = 0; i < modRawArray.length; i += 2) {
			const new_mod = validateJson(modRawArray[i + 1], modRawArray[i]);

			if (!new_mod) { continue; }

			// Check same tag
			existmod = tempMods.find(m => m.tag === new_mod.tag);
			if (existmod !== undefined) { // Same tag name
				toast.error("JSON: Conflicting mod tags with: '" + existmod.name + "' and '" + new_mod.name + "'" 
					+ "\n(" + new_mod.tag + ")");
				continue;
			}

			// Check same name
			existmod = tempMods.find(m => m.name === new_mod.name);
			if (existmod !== undefined) {
				toast.error("Same mod name for different mods, added -alt: '" + new_mod.name + "'");
				new_mod.name += "-alt";
			}

			// Only add when tag is not the same
			tempMods.push(new_mod);
		}

		return tempMods;
	}

	export async function refreshJsons () {
		try {
			const modRawArray = await invoke('get_jsons', {modFolder: MOD_FOLDER});

			let tempMods = processJsons(modRawArray);

			// Add checked status from old mods
			// Should do nothing if first loaded.
			for (const newMod of tempMods) {
				let oldMod = $MODS.find(m => m.tag === newMod.tag);
				if (oldMod !== undefined) {
					newMod.checked = oldMod.checked;

					if (newMod.version !== oldMod.version) {
						newMod.status = ModStatus.Updated;
					} else {
						newMod.status = oldMod.status;
					}
				} else {
					newMod.status = ModStatus.New;
				}
			}

			$MODS = tempMods;
			toast.success("Mod list refreshed.");
		} catch (err) {
			toast.error("get_jsons: " + err);
			console.error(err);
		}

		await saveData();
	}


	export async function saveData () {
		try {
			const cookies = {
				theme: $THEME,
				gfx_dir: $GFX_FOLDER,
				modlist: $MODS,
			}
			await invoke('save_cookies', {data: JSON.stringify(cookies), file: COOKIE_FILE});
			toast.success("Session saved.", { position: "top-left" });
		} catch (err) {
			toast.error("Session not saved: " + err);
			console.error(err);
		}
	}

	export async function loadData () {
		if (!$COOKIES_LOADED) {
			try {
				const cookies_str = await invoke('load_cookies', {file: COOKIE_FILE});

				if (cookies_str) {
					const cookies = JSON.parse(cookies_str);

					GFX_FOLDER.set(cookies.gfx_dir);
					MODS.set(cookies.modlist);
					$THEME = cookies.theme;

					COOKIES_LOADED.set(true);
					toast.success("Session loaded.", { position: "top-left" });				
				} else {
					// First time openning the app
					// Do nothing?
					console.warn("DR2C: FIRST TIME. COOKIES NOT LOADED.");
				}

			} catch (err) {
				// MAY CAUSE ERRORS LATER ON
				toast.error("Session loading error: " + err);
				console.error(err);
			}
		}
	}

</script>
