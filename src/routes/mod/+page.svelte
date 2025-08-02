<script lang='ts'>
	import { MR2C_GLOBAL } from '$lib/stores.svelte';
	
	import { ModStatus, type ModInfo } from '$lib/types';

	import { invoke } from '@tauri-apps/api/core';
	import toast from 'svelte-french-toast';

	import { saveData, refreshJsons } from '../SaverLoader';

	let ALL_MODS = MR2C_GLOBAL.MODS;
	let selectedMods = $derived(ALL_MODS.filter(m => m.checked === true));

	async function loadMods () {
		try {	// Clear / Reset the modded spritesheets to vanillas
			console.time('copydir');
			await invoke('restore_backup_gfx');
			console.timeEnd('copydir');
			// toast.success("GFX restored.");
		} catch (err) {
			toast.error("copy_dir_all: " + err);
		}

		for (const mod of ALL_MODS) {
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

		await saveData();
	}

	let isLoadingMods = $state(false);
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

	let modfocus: ModInfo | undefined = $state(undefined);
	function handleMouseOver(index: number) {
		modfocus = ALL_MODS.at(index);
	}
</script>

<div class="grid grid-cols-12 gap-3 h-full p-3">
	<div class="col-span-8 border border-base-content rounded-xl p-2" style="overflow: overlay;">
		{#each ALL_MODS as mod, i}
			<div class="form-control hover:bg-base-300 rounded-xl" 
				role="presentation"
				onmouseover={() => handleMouseOver(i)} 
				onfocus={() => handleMouseOver(i)} >

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
			onclick={() => refreshJsons()} >Refresh List</button>

		{#if ALL_MODS.length > 0}
			<button class="btn btn-primary uppercase" onclick={promiseMods} disabled={isLoadingMods} >Load Selected Mods</button>
		{:else}
			<p>No mod found in mods folder.</p>
		{/if}

		<div class="border border-base-content rounded-xl p-2 grow overflow-y-auto max-h-[35vh]">
			{#if modfocus !== undefined}
				<h3 class="text-lg text-center mb-3">{modfocus.name}</h3>
				<p><b>Version:</b> {modfocus.version}</p>
				<p><b>Creator:</b> {modfocus.creator}</p>
				{#if modfocus.description}
					<p class="whitespace-pre-line"><b>Description: </b>{modfocus.description}</p>
				{/if}
			{/if}
			<!-- <h6 class="inline-block align-text-bottom">Mod count: {modcount}</h6> -->
		</div>
		<a  class="btn btn-secondary btn-outline" href="steam://launch/252610">Run DR2C from Steam</a>
	</div>
</div>

