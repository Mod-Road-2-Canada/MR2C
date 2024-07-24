<script lang='ts'>
	import { MODS } from '$lib/stores';
	
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
			await invoke('restore_backup_gfx');
			console.timeEnd('copydir');
			toast.success("GFX restored.");
		} catch (err) {
			toast.error("copy_dir_all: " + err);
		}

		for (const mod of $MODS) {
			try {
				console.time(mod.name);
				let modArgs = { 
					modFile: mod.path, 
					modTag: mod.tag,
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
						<div class="badge badge-accent badge-outline">Updated!</div>
					{:else if mod.status === ModStatus.New}
						<div class="badge badge-info badge-outline">New!</div>
					{/if}
				</label>
			</div>		
		{/each}
	</div>

	<div class="col-span-4 flex flex-col justify-between gap-3">
		<button class="btn btn-accent btn-outline btn-sm sm:btn-md" 
			on:click={() => saverloader.refreshJsons()} >Refresh List</button>

		{#if modcount > 0}
			<button class="btn btn-primary" on:click={promiseMods} disabled={isLoadingMods} >Load Selected Mods</button>
		{:else}
			<p>No mod found in mods folder.</p>
		{/if}

		<div class="border rounded-xl p-2 grow overflow-y-auto max-h-[35vh]">
			{#if modfocus !== undefined}
				<h3 class="text-lg text-center mb-3">{modfocus.name}</h3>
				<p><b>Version: </b>{modfocus.version}</p>
				<p><b>Creator: </b>{modfocus.creator}</p>
				<!-- {#if modfocus.description} -->
					<p class="whitespace-pre-line"><b>Description: </b>{modfocus.description}</p>
				<!-- {/if} -->
			{/if}
			<!-- <h6 class="inline-block align-text-bottom">Mod count: {modcount}</h6> -->
		</div>
		<a  class="btn btn-secondary btn-outline" href="steam://launch/252610">Run DR2C from Steam</a>
	</div>
</div>


<style>
</style>
