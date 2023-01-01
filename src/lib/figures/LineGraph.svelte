<script>
	// @ts-nocheck
	import { draw } from 'svelte/transition';
	import { extent } from 'd3-array';
	import { scaleLinear } from 'd3-scale';
	import { line, curveBasis } from 'd3-shape';

	export let data;
	export let show;

	const height = 500;
	const width = 1.5 * height;
	const axisSpace = 50;
	const tickLength = 10;
	const legendWidth = 100;
	const buffer = 10;
	const margin = 10;

	const xScale = scaleLinear()
		.domain(extent(data.map((d) => d.x)))
		.range([axisSpace, width - buffer - legendWidth]);

	const yScale = scaleLinear()
		.domain(extent(data.map((d) => d.y)))
		.range([height - axisSpace - buffer, buffer + margin]);

	// the path generator
	const pathLine = line()
		.x((d) => xScale(d.x))
		.y((d) => yScale(d.y))
		.curve(curveBasis);
</script>

<svg {height} {width}>
	{#if show}
		<path transition:draw={{ duration: 100 }} d={pathLine(data)} />
	{/if}
</svg>

<style>
	path {
		stroke: purple;
		stroke-width: 2;
		fill: none;
		stroke-linecap: round;
	}
</style>
