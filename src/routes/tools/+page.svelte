<script>/*
	import { Highlight } from "svelte-highlight";
	import typescript from "svelte-highlight/languages/typescript";
	import "svelte-highlight/styles/monokai-sublime.css";
*/
  // import Markdown from '@magidoc/plugin-svelte-marked';
  import SvelteMarkdown from 'svelte-markdown';
  // import hljs from 'highlight.js/lib/core';
  // import 'highlight.js/styles/atom-one-dark.css';

	import readme from '$lib/README.md?raw';
//hljs.registerLanguage('javascript', require('highlight.js/lib/languages/javascript'));

</script>


<div class="p-3 overflow-auto" style="height: 60vh;">
	<h1>TEST</h1>
	<div class="prose">
		<SvelteMarkdown source={readme} />
	</div>
</div>

