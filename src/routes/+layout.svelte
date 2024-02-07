<script>
	import '$lib/app.css'

	import CustomTab from './CustomTab.svelte';

	import SideDocs from './docs/SideDocs.svelte';

	import banner from '$lib/images/banner.png';
	import { Toaster } from 'svelte-french-toast';

	import { COOKIES_LOADED } from '$lib/stores';

	let toastNewDefault = {
		success: {
			position: "top-right",
		},
		error: {
			duration: 5000,
			position: "bottom-right",
		},	
	}

	import { page } from '$app/stores';
	$: console.log($page);

	let toggle = false;
</script>

<div class:halfw={toggle} class="container-transition w-full">
	<div class="flex align-items-end relative"  
		style="background: url({banner}) no-repeat center; background-size: cover; height: 30vh;">

		<Toaster toastOptions={toastNewDefault}/>

		<SideDocs bind:toggle/>

		<div role="tablist" class="tabs tabs-lifted tabs-lg ">
			<CustomTab href='/'>Home</CustomTab>
			<!-- {#if $COOKIES_LOADED} -->
				<CustomTab href='/mod'>Install Mods</CustomTab>
			<!-- {/if} -->
		</div>
	</div>

	<div style="height: 70vh;">
		<slot />
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
