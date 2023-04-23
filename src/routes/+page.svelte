<script lang='ts'>
	import { GFX_FOLDER, BACKUP_GFX_FOLDER, DATA_FILE, MODS, COOKIES_LOADED } from '$lib/stores.ts';
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

	async function backupGFX () {
		try {
			let count = await invoke('copy_dir_all', {src: $GFX_FOLDER, dst: BACKUP_GFX_FOLDER, overwrite: false});
			toast.success(count + " file(s) copied");
		} catch (err) {
			toast.error(err);
			console.error(err);
		}
	}


	export async function loadData () {
		if (!$COOKIES_LOADED) {
			try {
				const cookies = JSON.parse(await invoke('load_cookies', {file: DATA_FILE}));
				GFX_FOLDER.set(cookies.gfx_dir);
				MODS.set(cookies.modlist);

				COOKIES_LOADED.set(true);
				toast.success("Session loaded.");
			} catch (err) {
				toast.error("Session not found." + err);
				console.error(err);
			}
		}
	}

	onMount(loadData);

</script>

<div class="p-3">
	<div class="mb-3">
		<label class="label">
			<span class="label-text">Select original gfx folder:</span>	
		</label>
		<div class="input-group">
			<input type="button" value="CHOOSE" class="btn file-input file-input-bordered" on:click={getGFX} />
			<span class="w-full file-input file-input-bordered">{$GFX_FOLDER}</span>
		</div>
	</div>
	{#if $GFX_FOLDER != ""}
		<button class="btn btn-primary mr-2" on:click={backupGFX}>Back up</button>
	{/if}
	<button class="btn" on:click={setCWD} >CWD</button>
</div>