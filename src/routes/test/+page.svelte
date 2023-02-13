<script lang="ts">
	import Beam1D from '$lib/components/Beam1D.svelte';
	let viewPortWidth, viewPortHeight;

	let x1: number = 0,
		y1: number = 0,
		x2: number = 10,
		y2: number = 5;

	$: beam = { start_node: { x: x1, y: y1 }, end_node: { x: x2, y: y2 } };
</script>

<main>
	<sidebar>
		<input type="number" bind:value={x1} step="0.1" />
		<input type="number" bind:value={x2} step="0.1" />
		<input type="number" bind:value={y1} step="0.1" />
		<input type="number" bind:value={y2} step="0.1" />
	</sidebar>

	<div class="container" bind:clientWidth={viewPortWidth} bind:clientHeight={viewPortHeight}>
		<Beam1D parentWidth={viewPortWidth} parentHeight={viewPortHeight} {beam} />
	</div>
</main>

<style>
	.container {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
		flex-grow: 1;
		display: flex;
		flex-direction: column;
	}
</style>
