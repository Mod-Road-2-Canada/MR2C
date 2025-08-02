<script lang="ts">
	import '$lib/app.css'
	import { onMount } from 'svelte';

	import CustomTab from './CustomTab.svelte';

	import SideDocs from './docs/SideDocs.svelte';

	import banner from '$lib/images/banner.png';
	import { Toaster } from 'svelte-french-toast';

	import { MR2C_GLOBAL } from '$lib/stores.svelte';

	
	let { children } = $props();

	// Automatic change theme
	$effect(() => {
		console.log(MR2C_GLOBAL.THEME)
		document.documentElement.setAttribute("data-theme", MR2C_GLOBAL.THEME);
	});

	// Toast default options
	let toastNewDefault = {
		success: {
			position: "top-right",
		},
		error: {
			duration: 5000,
			position: "bottom-right",
		},	
	}

	let showDocs = $state(false);
</script>

<div class:halfw={showDocs} class="container-transition w-full">
	<div class="flex align-items-end relative"  
		style="background: url({banner}) no-repeat center; background-size: cover; height: 30vh;">

		<Toaster toastOptions={toastNewDefault}/>

		<SideDocs bind:showDocs/>

		<div role="tablist" class="tabs tabs-lifted tabs-md">
			<CustomTab href='/'>Home</CustomTab>
			{#if MR2C_GLOBAL.COOKIES_LOADED}
				<CustomTab href='/mod'>Install Mods</CustomTab>
			{/if}
		</div>
	</div>

	<div style="height: 70vh;">
		{@render children?.()}
	</div>

</div>

<style>
	.halfw {
		width: 50%;
	}

	.container-transition {
		transition: all 0.5s cubic-bezier(0.32, 0.72, 0, 1);
	}
</style>
