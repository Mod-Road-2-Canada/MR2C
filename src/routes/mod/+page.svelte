<script lang='ts'>
	import { GFX_FOLDER, BACKUP_GFX_FOLDER, DATA_FILE, MODS } from '$lib/stores.ts';

	import { invoke } from '@tauri-apps/api/tauri';

	import toast from 'svelte-french-toast';

	const MOD_FOLDER = "mods";

	$: modcount = $MODS.length;
	$: selectedMods = $MODS.filter(m => m.checked === true);
	$: { console.log(selectedMods); }

	async function getJsons () {
		try {
			const modRawArray = await invoke('get_jsons', {modFolder: MOD_FOLDER});
			let tempMods = [];

			for (var i = 0; i < modRawArray.length; i += 2) {
				const new_mod = JSON.parse(modRawArray[i + 1]);
				new_mod.filename = modRawArray[i];

				if (!new_mod.hasOwnProperty('path')) {
					toast.error("JSON: Mod with no path: " + new_mod.filename);
					continue;
				}

				if (!new_mod.hasOwnProperty('version')) {
					toast.error("JSON: Mod with no version: " + new_mod.filename);
					continue;
				}

				let existmod = tempMods.find(m => m.tag === new_mod.tag);
				if (existmod !== undefined) { // Same tag name
					toast.error("JSON: Conflicting mod tags with: '" + existmod.name + "' and '" + new_mod.name + "'" 
						+ "\n(" + new_mod.tag + ")");
					continue;
				}

				if (!new_mod.hasOwnProperty('name')) {
					toast.error("JSON: Mod with no name: '" + new_mod.filename + "'");
					new_mod.name = new_mod.filename;
				}

				if (!new_mod.hasOwnProperty('tag')) {
					new_mod.tag = new_mod.filename;
				}

				let existname = tempMods.find(m => m.name === new_mod.name);
				if (existname !== undefined) {
					toast.error("Warning: JSON: Same mod name for different mods, added -alt: '" + new_mod.name + "'");
					new_mod.name += "-alt";
				}

				// Add checked
				let loadedmod = $MODS.find(m => m.tag === new_mod.tag);
				if (loadedmod !== undefined) {
					new_mod.checked = loadedmod.checked;
				}

				// Only add when tag is not the same
				tempMods.push(new_mod);
			}

			$MODS = tempMods;
			toast.success("Mod list refreshed.");
		} catch (err) {
			toast.error(err);
			console.error(err);
		}

		await saveData();
	}

	export async function saveData () {
		try {
			const cookies = {
				gfx_dir: $GFX_FOLDER,
				modlist: $MODS,
			}
			await invoke('save_cookies', {data: JSON.stringify(cookies), file: DATA_FILE});
			toast.success("Session saved.");
		} catch (err) {
			toast.error("Session not saved." + err);
			console.error(err);
		}
	}



	async function loadMods () {
		try {	// Clear / Reset the modded spritesheets to vanillas
			console.time('copydir');
			let count = await invoke('copy_dir_all', {src: BACKUP_GFX_FOLDER, dst: $GFX_FOLDER, overwrite: true});
			console.timeEnd('copydir');
			toast.success(count + " file(s) copied");
		} catch (err) {
			toast.error(err);
		}

		for (const mod of $MODS) {
			try {
				console.time(mod.name);
				let modArgs = { 
					modFile: mod.path, 
					modTag: mod.tag,
					gfxModded: $GFX_FOLDER,
					gfxVanilla: BACKUP_GFX_FOLDER
				};

				await invoke('load_mod', {...modArgs, modInstallState: 2});
				if (mod.checked) {
					await invoke('load_mod', {...modArgs, modInstallState: 1});
					toast.success("Done: " + mod.name, { position: "bottom-right"});
				}
				
				console.timeEnd(mod.name);
			} catch (err) {
				toast.error("In: " + mod.name + "\nError: " + err);
			}
		}	

		await saveData();
	}

  const myPromise = () => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        resolve();
      }, 5000);
    });
  };

	let isLoadingMods = false;
	function promiseMods () {
		isLoadingMods = true;

		toast.promise(
			loadMods(),
			{
				loading: "Applying Mods",
				success: "All Finished.",
				error: "Mod loading failed",
			},
			{ position: "top-right" }
		);

		setTimeout(() => {
			isLoadingMods = false;
		}, 5000);
	}

	let modfocus = undefined;
	function handleMouseOver(i) {
		modfocus = $MODS.at(i);
	}
</script>


<div class="grid grid-cols-12 gap-3 h-full p-3">
	<div class="col-span-8 border rounded-xl p-2" style="overflow: overlay;">
		{#each $MODS as mod, i}
			<div class="form-control hover:bg-neutral-focus rounded-xl" on:mouseover={() => handleMouseOver(i)}>
				<label class="label justify-normal">
					<input class="checkbox me-3 checkbox-primary"
						type=checkbox 
						bind:checked={mod.checked} value={mod}
					/>
					<span class="label-text">{mod.name}</span>
				</label>
			</div>		
		{/each}
	</div>

	<div class="col-span-4 flex flex-col justify-between gap-3">
		{#if $GFX_FOLDER != ""}
			{#if modcount > 0}
				<button class="btn btn-accent btn-outline btn-sm sm:btn-md" on:click={getJsons} >Refresh List</button>
				<button class="btn btn-primary" on:click={promiseMods} disabled={isLoadingMods} >Load Selected Mods</button>
			{:else}
				<p>Please follow the instructions in the Home tab to proceed.</p>
			{/if}
			<div class="border rounded-xl p-2 grow">
				{#if modfocus !== undefined}
					<h3 class="text-lg text-center mb-3">{modfocus.name}</h3>
					<p><b>Version: </b>{modfocus.version}</p>
					<p><b>Creator: </b>{modfocus.creator}</p>
				{/if}
				<!-- <h6 class="inline-block align-text-bottom">Mod count: {modcount}</h6> -->
			</div>
			<a  class="btn btn-secondary btn-outline" href="steam://launch/252610">Run From Steam</a>
		{/if}
	</div>
</div>


<style>
</style>
