<script>
  import { Highlight } from "svelte-highlight";
  import typescript from "svelte-highlight/languages/typescript";
  import "svelte-highlight/styles/monokai-sublime.css";

  const code = "const add = (a: number, b: number) => a + b;";
  const whyyy = "<code>shit</code>1";
</script>


<div class="p-3">
	<h1>about</h1>
	<p>this is the about page.</p>
<Highlight class="my-3" language={typescript} {whyyy} />
	

</div>


