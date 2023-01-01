<script lang="ts">
	// @ts-nocheck

	import { scaleLinear, scaleOrdinal } from 'd3-scale';
	import { extent } from 'd3-array';
	import type { IrisEntry } from '$lib/types/iris';

	export let show;
	export let data: IrisEntry[];
	export let xDimension: string;
	export let yDimension: string;

	const height = 500;
	const width = 1.5 * height;
	const axisSpace = 50;
	const tickLength = 10;
	const legendWidth = 100;
	const buffer = 10;
	const margin = 0;
	const drawTime = 1000;
	const scaleMargin = 0.1;
	const animationTime = drawTime / 10;
	const colors = ['red', 'green', 'blue'];

	$: xExtent = extent(data, (d: IrisEntry) => d[xDimension]);
	$: yExtent = extent(data, (d: IrisEntry) => d[yDimension]);
	$: xBuffer = (xExtent[1] - xExtent[0]) * scaleMargin;
	$: yBuffer = (yExtent[1] - yExtent[0]) * scaleMargin;

	let species = Array.from(new Set(data.map((d) => d.species)));

	let colorScale = scaleOrdinal().domain(Array.from(species)).range(colors);

	$: xScale = scaleLinear()
		.domain([xExtent[0] - xBuffer, xExtent[1] + xBuffer])
		.range([axisSpace + buffer, width - buffer - legendWidth]);
	$: yScale = scaleLinear()
		.domain([yExtent[0] - yBuffer, yExtent[1] + yBuffer])
		.range([height - axisSpace - buffer, buffer + margin]);
</script>

{#if show}
	<svg {height} {width}>
		<!-- Plot background -->
		<g transform={`translate(${axisSpace + buffer},${buffer})`}>
			<rect
				height={height - axisSpace - 2 * buffer}
				width={width - axisSpace - 2 * buffer - legendWidth}
				fill="white"
				stroke="black"
			/>
		</g>
		<!-- Plot data -->
		{#each data as item, i}
			<circle
				r="3"
				cx={xScale(item[xDimension])}
				cy={yScale(item[yDimension])}
				fill={colorScale(item.species)}
			/>
		{/each}
		<!-- Draw xAxis -->
		{#each xScale.ticks() as tick, i}
			<g transform={`translate(${xScale(tick) + buffer} ${height - axisSpace - buffer})`}>
				<line y1={-tickLength} y2={tickLength} stroke="black" />
				<text y={15 + tickLength} text-anchor="middle">{tick}</text>
			</g>
		{/each}
		<!-- Draw yAxis -->
		{#each yScale.ticks() as tick, i}
			<g transform={`translate(${axisSpace + buffer} ${yScale(tick)})`}>
				<line x1={-tickLength} x2={tickLength} stroke="black" />
				<text x="-25" dominant-baseline="middle" text-anchor="end">{tick.toFixed(1)}</text>
			</g>
		{/each}
		<!-- Legend -->
		<g transform={`translate(${width - legendWidth},${buffer})`}>
			{#each species as species, i}
				<g transform={`translate(0 ${i * 20})`}>
					<rect height="10" width="10" fill={colorScale(species)} />
					<text dominant-baseline="middle" y="5" x="20">{species}</text>
				</g>
			{/each}
		</g>
	</svg>
{/if}

<style>
	svg {
		background: rgba(255, 255, 255, 0.9);
	}
</style>
