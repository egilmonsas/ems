<script lang="ts">
	// @ts-nocheck
	import Selector from '$lib/components/Selector.svelte';
	import linepath from '../../../src/stores/data/linepath.js';
	import LineGraph from '$lib/figures/LineGraph.svelte';
	import Scatter from '$lib/figures/Scatter.svelte';
	import type { IrisEntry } from '$lib/types/iris';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	let show = false;
	let xDimension = 'petalWidth';
	let yDimension = 'petalLength';
	let options = ['petalWidth', 'petalLength', 'sepalWidth', 'sepalLength'];
	let data = loadData();

	async function loadData() {
		let data = await fetch('../../../src/stores/data/iris.json');
		let json = (await data.json()) as IrisEntry[];
		return json;
	}
</script>

<Header links={[{ display: 'Home', route: '/' }]} />
<main>
	<sidebar>
		<div>
			<label for="show" style="display: inline;">Show Line:</label>
			<input id="show" type="checkbox" bind:checked={show} />
		</div>
		<p>X Dimension: <Selector bind:selected={xDimension} {options} /></p>
		<p>Y Dimension: <Selector bind:selected={yDimension} {options} /></p>
	</sidebar>
	{#await data}
		<p>Loading...</p>
	{:then iris}
		<Scatter {show} data={iris} {xDimension} {yDimension} />
	{/await}
	<LineGraph data={linepath} {show} />
</main>
<Footer />

<style>
	div {
		display: flex;
		flex-direction: row;
	}

	input {
		width: 100%;
	}
</style>
