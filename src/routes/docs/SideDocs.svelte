<script lang='ts'>
	// import { Drawer } from 'vaul-svelte';

	import modderdocs from '$lib/README_MODDER.md?raw';
	import playerdocs from '$lib/README_USER.md?raw';
	import aboutdocs from '$lib/README_ABOUT.md?raw';


	
	import toast from 'svelte-french-toast';

	import DarkLightToggle from './DarkLightToggle.svelte';
	import DocsCollapse from './DocsCollapse.svelte';

	let { showDocs = $bindable(false) } = $props();

	import bezier from 'bezier-easing'
	import { fly } from 'svelte/transition';
</script>

<button class="btn btn-primary absolute right-0 top-0 m-4"
	onclick={() => { showDocs = !showDocs; console.log(showDocs); } }>
	Help
</button>

{#if showDocs}
	<div class="fixed md:w-[50%] w-[90%] right-0 top-0 flex bg-base-300 rounded-2xl border-secondary/50 border-l-2" 
		transition:fly={{ duration: 500, x: '100%', easing: bezier(0.32, 0.72, 0, 1) }}>
		<DarkLightToggle />

		<!-- Tab on the side -->


		<div class="w-full max-w-full overflow-y-auto overflow-x-hidden h-screen">
			<DocsCollapse title="How to install mods" source={playerdocs}/>
			<DocsCollapse title="Modder Documentations" source={modderdocs} codeFormat={true}/>
			<DocsCollapse title="About" source={aboutdocs} />
		</div>
	</div>
{/if}
<!-- 
<Drawer.Root bind:open={toggle} direction="right"
		onOutsideClick={(e: Event) => e.preventDefault() }>
	<Drawer.Trigger
	class=
	
	>
		Help
	</Drawer.Trigger>

	<Drawer.Portal>
-->
			<!-- <Drawer.Overlay class="fixed inset-0 bg-black/40"/>
		<Drawer.Content 
			


		</Drawer.Content>

	</Drawer.Portal>
</Drawer.Root> -->



