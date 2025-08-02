<script lang="ts">
	import { page } from '$app/state'
	let { href = '', children } = $props()

	let tabIsActive = $derived(
		href == '/' ? page.url.pathname == '/' : page.url.pathname.includes(href)
	)
</script>

<a
	class="tab font-semibold"
	class:text-gray-400={!tabIsActive}
	class:tab-active={tabIsActive}
	{href}>
	{@render children?.()}
</a>
