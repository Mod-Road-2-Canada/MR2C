<script>
	export let title = "Title"
</script>

<div class="border-t-2 border-accent collapse collapse-arrow">
	<input type="checkbox"/>
	<h2 class="collapse-title text-xl font-bold">{title}</h2>
	<div class="collapse-content px-4 min-w-0">
		<slot/>
	</div>
</div>