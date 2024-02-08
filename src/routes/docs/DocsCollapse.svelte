<script>
	import SvelteMarkdown from 'svelte-markdown';
	import Highlighter from "./Highlighter.svelte";
	
	export let title = "Title"
	export let source;
	export let codeFormat = false;
</script>

<div class="border-t-2 border-secondary collapse collapse-arrow">
	<input type="checkbox"/>
	<h2 class="collapse-title text-xl font-bold">{title}</h2>
	<div class="collapse-content px-4 min-w-0 prose select-text max-w-full">
		{#if codeFormat}
			<SvelteMarkdown source={source} 
			renderers={{ code: Highlighter }}
			/>
		{:else}
			<SvelteMarkdown source={source} />
		{/if}
	</div>
</div>