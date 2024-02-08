<script>
	import github from "svelte-highlight/styles/monokai";
	import Highlight from "svelte-highlight";
	import typescript from "svelte-highlight/languages/typescript";

	export let text = "No code?";
	export let lang = "";
</script>


<svelte:head>
	<style>
		.ts {background: oklch(var(--n));}
		.ts .hljs-title {font-weight: 600;}
		.ts .hljs-string {color: oklch(var(--a));}
		.ts .hljs-literal {color: oklch(var(--p));}
	</style>
</svelte:head>

<!-- <div>'{text}' + '{infostring}'</div> -->
<Highlight language={typescript} code={text} class={lang}/>