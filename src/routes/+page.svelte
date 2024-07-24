<script lang='ts'>
	import { COOKIES_LOADED } from '$lib/stores';

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
			toast.error("setCWD: " + err);
			console.error(err);
		}
	}

	import SaverLoader from './SaverLoader.svelte';
	let saverloader;

	async function backupGFX () {

		try {
			await invoke('create_backup_gfx');
			toast.success("GFX copied");

			// Only if CWD is available
			await saverloader.loadData();

			// If cookies cannot be loaded, create new
			if (!$COOKIES_LOADED) {
				await invoke('create_dir_if_not_exist', {name: "mods"});
				await saverloader.refreshJsons();
				COOKIES_LOADED.set(true);
			}
		} catch (err) {
			toast.error("backupGFX: " + err);
			console.error(err);
		}
	}

	onMount(() => {
		saverloader.loadData();
	});

</script>


<SaverLoader bind:this={saverloader} />

<div class="p-3">
	<button class="btn join-item btn-primary" on:click={backupGFX}>Initialize (Back up gfx folder)</button>
</div>