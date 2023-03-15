<script> /*
	import Counter from './Counter.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcome_fallback from '$lib/images/svelte-welcome.png'; */
	import { invoke } from "@tauri-apps/api/tauri";
	import { appDataDir } from '@tauri-apps/api/path';
	const appDataDirPath = await appDataDir();

	let greetMsg = '';
	let selectedMods = [];

	let mods = [];
	$: modcount = mods.length;

	async function getMods () {
		try {
			mods = await invoke('get_mods');
			console.log(mods);
		} catch (err) {
			console.error(err.message);
		}
	}
		let fillings = [];
		$: { console.log(selectedMods); }
</script>

<section>
<input type="checkbox" bind:group={fillings} value="Rice">
<input type="checkbox" bind:group={fillings} value="Beans">
<input type="checkbox" bind:group={fillings} value="Cheese">
<input type="checkbox" bind:group={fillings} value="Guac (extra)">

	<div class="container">
		
		<h1>
			<span class="welcome">

			</span>

			to your new<br />SvelteKit app
		</h1>

		<h2>
			try editing <strong>src/routes/+page.svelte</strong>
		</h2>
		<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

	</div>


	<div class="container text-start">
					
		{#each mods as mod, i}
			<div class="form-check">
						<label class="form-check-label">
							<input class="form-check-input me-3" type=checkbox bind:group={selectedMods} value={mod} />
							{mod}
						</label>
			</div>		
		{/each}

	</div>


	<div class="container text-center my-5">
		<div class="row">
			<h1>Welcome to Tauri!</h1>
		</div>
			
		<p>Click on the Tauri, Vite, and React logos to learn more.</p>

		<div class="row">
			<div class="col">
				<button class="" on:click={getMods} >Wei</button>
			</div>
		</div>
		<p>{greetMsg} {modcount}</p>
	</div>
</section>

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

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
