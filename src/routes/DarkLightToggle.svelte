<script>
	import dayicon from '$lib/images/day_icon.png';
	import nighticon from '$lib/images/night_icon.png';
</script>


<label class="swap swap-flip absolute l-0 r-0 p-4">
	<!-- this hidden checkbox controls the state -->
	<input type="checkbox" class="theme-controller" value="cupcake" />
	
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