<script lang='ts'>
	import { COOKIES_LOADED } from '$lib/stores';

	import { onMount } from 'svelte';
	import toast from 'svelte-french-toast';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';

	async function setCWD () {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				defaultPath: await invoke('get_cwd')
			});

			// const selected = "D:\\Steam\\steamapps\\common\\DeathRoadToCanada";

			if (selected == null) {
				throw "No folder selected";
			} else {
				await invoke('set_cwd', {pathCwd: selected});
				await loadData();
			}

			toast.success("Changed cwd to " + selected);
		} catch (err) {
			toast.error("setCWD: " + err);
			console.error(err);
		}
	}

	import { loadData, refreshJsons } from './SaverLoader';

	async function backupGFX () {

		try {
			await invoke('create_backup_gfx');
			toast.success("GFX copied");

			// Only if CWD is available
			await loadData();

			// If cookies cannot be loaded, create new
			if (!$COOKIES_LOADED) {
				await invoke('create_dir_if_not_exist', {name: "mods"});
				await refreshJsons();
				COOKIES_LOADED.set(true);
			}
		} catch (err) {
			toast.error("backupGFX: " + err);
			console.error(err);
		}
	}

	onMount(() => {
		loadData();
	});

</script>

<div class="p-3">
	<button class="btn btn-primary" on:click={backupGFX}>Initialize (Back up gfx)</button>

	<!-- <button class="btn btn-secondary" on:click={setCWD}>CWD</button> -->
</div>