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
			let tempMods = $MODS;

			for (var i = 0; i < modRawArray.length; i+=2) {
				const new_mod = JSON.parse(modRawArray[i + 1]);

				if (!new_mod.hasOwnProperty('path')) {
					toast.error("JSON: Mod with no path: " + modRawArray[i]);
					continue;
				}

				if (!new_mod.hasOwnProperty('version')) {
					toast.error("JSON: Mod with no version: " + modRawArray[i]);
					continue;
				}

				if (!new_mod.hasOwnProperty('name')) {
					toast.error("JSON: Mod with no name: '" + modRawArray[i] + "'");
					new_mod.name = modRawArray[i];
				}

				if (!new_mod.hasOwnProperty('tag')) {
					new_mod.tag = modRawArray[i];
				}

				let existmod = tempMods.find(m => m.name === new_mod.name);
				if (existmod !== undefined) {
					existmod = Object.assign(existmod, new_mod);
				} else {
					let existtag = tempMods.find(m => m.tag === new_mod.tag)
					if (existtag !== undefined) {
						toast.error("JSON: Conflicting mod tags: '" + modRawArray[i] + "' and '" + m.name + "'");
					}

					tempMods.push(new_mod);		
				}
			}

			MODS.set(tempMods);
			toast.success("Mod list refreshed.");
		} catch (err) {
			toast.error(err);
			console.error(err);
		}
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

</script>


<div class="grid grid-cols-12 gap-3 h-full p-3">
	<div class="col-span-8 border rounded-xl p-2" style="overflow: overlay;">
		{#each $MODS as mod}
			<div class="form-control">
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
				<button class="btn" on:click={getJsons} >Load Mods</button>
			{/if}
			<div class="border rounded-xl p-2 grow">
				<h6>{modcount}</h6>
			</div>
			<a  class="btn btn-primary" href="steam://launch/252610">Run From Steam</a>
		{/if}
	</div>
</div>


<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}
</style>
