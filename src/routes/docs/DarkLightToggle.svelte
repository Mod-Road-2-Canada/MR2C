<script>
	import { THEME } from '$lib/stores';

	import dayicon from '$lib/images/day_icon.png';
	import nighticon from '$lib/images/night_icon.png';

	function toggleTheme(event) {
		$THEME = (event.target.checked) ? "cupcake" : "apocalypse";
	}
</script>


<label class="swap swap-flip fixed right-0 bottom-0 mb-4 mr-4 z-10">
	<!-- this hidden checkbox controls the state -->
	<input type="checkbox" on:change={toggleTheme} checked={$THEME === "cupcake"}/>
	
	<img class="pixelart swap-on w-14 h-14 border-[3px] border-accent" 
		title="Light mode" src={dayicon} alt="Light mode button"/>

	<img class="pixelart swap-off w-14 h-14 border-[3px] border-base-100"
		title="Dark mode" src={nighticon} alt="Dark mode button"/>
</label>


<style>
	.pixelart {
		image-rendering: pixelated;
	}
</style>