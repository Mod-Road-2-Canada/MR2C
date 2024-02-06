<script>
	import '$lib/app.css'

	import CustomTab from './CustomTab.svelte';

	import SideWrapper from './docs/SideWrapper.svelte';
	import DarkLightToggle from './DarkLightToggle.svelte'
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
</script>

<SideWrapper>

<div class="flex align-items-end"  
	style="background: url({banner}) no-repeat center; background-size: cover; height: 30vh;">

	<Toaster toastOptions={toastNewDefault}/>

	<DarkLightToggle />

	<div role="tablist" class="tabs tabs-lifted tabs-lg ">
		<CustomTab href='/'>Home</CustomTab>
		<!-- {#if $COOKIES_LOADED} -->
			<CustomTab href='/mod'>Install Mods</CustomTab>
			<CustomTab href='/tools'>Help</CustomTab>
		<!-- {/if} -->
	</div>
</div>

<div style="height: 70vh;">
	<slot />
</div>

</SideWrapper>