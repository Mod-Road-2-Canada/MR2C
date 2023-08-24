<script>
	import '$lib/app.css'

	import CustomTab from './CustomTab.svelte';
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
</script>

<!-- <img src={banner} alt="Banner for mod installer" style="width: 100%; height: 35vh; object-fit: cover;"/> -->
<div class="tabs align-items-end" style="background: url({banner}) no-repeat center; background-size: cover; height: 30vh;">
	<Toaster toastOptions={toastNewDefault}/>

	<CustomTab href='/'>Home</CustomTab>
	{#if $COOKIES_LOADED}
		<CustomTab href='/mod'>Install Mods</CustomTab>
		<CustomTab href='/tools'>Help</CustomTab>
	{/if}
</div>

<div style="height: 70vh;">
	<slot />
</div>