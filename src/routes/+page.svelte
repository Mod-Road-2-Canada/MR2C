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

<h1>Home</h1>
<p>this is the HMEEEEEEE page.</p>
<b>{$GFX_FOLDER}</b><br/>
<b>{BACKUP_GFX_FOLDER}</b><br/>

<button class="btn" on:click={setCWD} >CWD</button>
<button class="btn" on:click={getGFX} >Select original gfx folder</button>