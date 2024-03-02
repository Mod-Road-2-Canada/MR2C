<script lang='ts'>
	import { GFX_FOLDER, MODS } from '$lib/stores';
	import { BACKUP_GFX_FOLDER } from '$lib/consts';
	
	import { ModStatus } from '$lib/types';

	import { invoke } from '@tauri-apps/api/tauri';
	import toast from 'svelte-french-toast';

	import SaverLoader from '../SaverLoader.svelte';
	let saverloader;

	$: modcount = $MODS.length;
	$: selectedMods = $MODS.filter(m => m.checked === true);
	$: { console.log(selectedMods); }


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
				};

				await invoke('load_mod', {...modArgs, modInstallState: 2});
				if (mod.checked) {
					await invoke('load_mod', {...modArgs, modInstallState: 1});
					toast.success("Loaded: " + mod.name, { position: "bottom-right"});
				}
				
				console.timeEnd(mod.name);
			} catch (err) {
				toast.error("In: " + mod.name + "\nError: " + err);
			}
		}	

		await saverloader.saveData();
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


<SaverLoader bind:this={saverloader} />

<div class="grid grid-cols-12 gap-3 h-full p-3">
	<div class="col-span-8 border rounded-xl p-2" style="overflow: overlay;">
		{#each $MODS as mod, i}
			<div class="form-control hover:bg-base-300 rounded-xl" 
				role="presentation"
				on:mouseover={() => handleMouseOver(i)} 
				on:focus={() => handleMouseOver(i)} >

				<label class="label justify-between">
					<div class="flex items-center">
						<input class="checkbox me-3 checkbox-primary"
							type=checkbox 
							bind:checked={mod.checked} value={mod}
						/>
						<span class="label-text">{mod.name}</span>
					</div>
					{#if mod.status === ModStatus.Updated}
						<div class="badge badge-secondary badge-outline">Updated!</div>
					{:else if mod.status === ModStatus.New}
						<div class="badge badge-info badge-outline">New!</div>
					{/if}
				</label>
			</div>		
		{/each}
	</div>

	<div class="col-span-4 flex flex-col justify-between gap-3">
		{#if $GFX_FOLDER != ""}
			{#if modcount > 0}
				<button class="btn btn-accent btn-outline btn-sm sm:btn-md" on:click={() => saverloader.refreshJsons()} >Refresh List</button>
				<button class="btn btn-primary" on:click={promiseMods} disabled={isLoadingMods} >Load Selected Mods</button>
			{:else}
				<p>There must be an error somewhere. Please report back to Hwang on Discord.</p>
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
