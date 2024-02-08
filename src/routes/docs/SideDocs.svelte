<script>/*
	import { Highlight } from "svelte-highlight";
	import typescript from "svelte-highlight/languages/typescript";
	import "svelte-highlight/styles/monokai-sublime.css";
*/
  // import Markdown from '@magidoc/plugin-svelte-marked';
	import { Drawer } from 'vaul-svelte';
	import SvelteMarkdown from 'svelte-markdown';
  // import hljs from 'highlight.js/lib/core';
  // import 'highlight.js/styles/atom-one-dark.css';

	import modderdocs from '$lib/README.md?raw';
	import playerdocs from '$lib/README_USER.md?raw';
//hljs.registerLanguage('javascript', require('highlight.js/lib/languages/javascript'));

	export let toggle = false;
	import toast from 'svelte-french-toast';

	import DarkLightToggle from './DarkLightToggle.svelte';
	import DocsCollapse from './DocsCollapse.svelte';
</script>

<Drawer.Root bind:open={toggle} direction="right"
		onOutsideClick={(e) => e.preventDefault() }>
	<Drawer.Trigger
	class="btn btn-primary absolute right-0 top-0 m-4"
	on:click={() => { if(toggle){ toggle = false;} } }
	>
		Help
	</Drawer.Trigger>

	<Drawer.Portal>

			<!-- <Drawer.Overlay class="fixed inset-0 bg-black/40"/> -->
		<Drawer.Content 
			class="fixed md:w-[50%] w-[90%] right-0 top-0 flex bg-base-300 rounded-2xl border-accent/50 border-l-2">
			<DarkLightToggle />

			<!-- Tab on the side -->
			<div class="p-2 -left-8 absolute bottom-[45%]">
			<div class="rounded-l-2xl h-24 w-6 bg-base-300 flex justify-center items-center border-accent/50 border-2 border-r-0 gap-1">
				<div class="rounded-full h-16 w-1 bg-base-content/40" />
				<div class="rounded-full h-16 w-1 bg-base-content/40" />
			</div>
			</div>

			<div class="w-full max-w-full overflow-y-auto overflow-x-hidden h-screen">
				<DocsCollapse title="How to install mods">
					<div class="prose select-text">
						<SvelteMarkdown source={playerdocs} />
					</div>				
				</DocsCollapse>

				<DocsCollapse title="Documentations">
					<div class="prose select-text">
						<SvelteMarkdown source={modderdocs} />
					</div>				
				</DocsCollapse>
			</div>

		</Drawer.Content>

	</Drawer.Portal>
</Drawer.Root>
