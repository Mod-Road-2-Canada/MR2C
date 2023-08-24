<script lang='ts'>
	import { GFX_FOLDER, BACKUP_GFX_FOLDER, COOKIES_LOADED } from '$lib/stores.ts';
	import { onMount } from 'svelte';
	import toast from 'svelte-french-toast';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';

	async function setCWD () {
		try {
			// const basedir = await executableDir();
			// console.log(basedir);
			const selected = await open({
				directory: true,
				multiple: false,
				defaultPath: await invoke('get_cwd')
			});

			if (selected == null) {
				throw "No folder selected";
			} else {
				await invoke('set_cwd', {pathCwd: selected});
			}

			toast.success("Changed cwd to " + selected);
		} catch (err) {
			toast.error(err);
			console.error(err);
		}
	}

	async function getGFX () {
		try {
			// const basedir = await executableDir();
			// console.log(basedir);
			const selected = await open({
				directory: true,
				multiple: false,
				defaultPath: await invoke('get_cwd')
			});

			if (selected == null) {
				throw "No folder selected";
			} else {
				GFX_FOLDER.set(selected);
			}
		} catch (err) {
			toast.error(err);
			console.error(err);
		}
	}

	import SaverLoader from './SaverLoader.svelte';
	let saverloader;

	async function backupGFX () {
		let correctExePlacement = await invoke('check_file_in_cwd', {filePath: $GFX_FOLDER});
		if (!correctExePlacement) {
			toast.error("This .exe is placed in the wrong folder. Please follow Step 0 correctly in \"How to install mods\" section.");
			return;
		}

		try {
			let count = await invoke('copy_dir_all', {src: $GFX_FOLDER, dst: BACKUP_GFX_FOLDER, overwrite: false});
			toast.success(count + " file(s) copied");

			// First time since cookies cannot be loaded
			if (!$COOKIES_LOADED) {
				await saverloader.refreshJsons();
				COOKIES_LOADED.set(true);
			}
		} catch (err) {
			toast.error(err);
			console.error(err);
		}
	}

	onMount(() => {
		saverloader.loadData(); 
	});

	import SvelteMarkdown from 'svelte-markdown';
	import readme from '$lib/README_USER.md?raw';
</script>


<SaverLoader bind:this={saverloader} />

<div class="p-3">
	<label class="label" for="selectGFX">
		<span class="label-text">Select original gfx folder:</span>	
	</label>
	<div class="flex mb-3 join border border-black">
		<input id="selectGFX" type="button" value="CHOOSE" class="btn join-item file-input-bordered" on:click={getGFX} />
		<span class="px-3 join-item w-full no-scrollbar overflow-auto self-center">{$GFX_FOLDER}</span>
		{#if $GFX_FOLDER != ""}
			<button class="btn join-item btn-primary" on:click={backupGFX}>Back up</button>
		{/if}
		<!-- For testing only -->
		<button class="btn join-item" on:click={setCWD} >CWD</button>
		<!-- For testing only -->
	</div>
</div>

<div class="overflow-auto border-t-2 border-accent collapse collapse-arrow hover:bg-neutral-focus" >
	<input type="checkbox" />
	<h1 class="collapse-title text-xl font-bold">How to install mods</h1>
	<div class="prose collapse-content overflow-auto " style="max-width: none; height: 45vh">
		<SvelteMarkdown source={readme} />
	</div>
</div>

<style>
	/* Hide scrollbar for Chrome, Safari and Opera */
	.no-scrollbar::-webkit-scrollbar {
		display: none;
	}

	/* Hide scrollbar for IE, Edge and Firefox */
	.no-scrollbar {
		-ms-overflow-style: none;  /* IE and Edge */
		scrollbar-width: none;  /* Firefox */
	}
</style>