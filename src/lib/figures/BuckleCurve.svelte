<script lang="ts">
	// @ts-nocheck

	import { scaleLinear } from 'd3-scale';
	import { min, max } from 'd3-array';
	import type { BuckleResponse } from '$lib/types/buckleResponse.ts';
	import { line, curveBasis } from 'd3-shape';

	export let data: BuckleResponse[];
	export let xDimension: string;
	export let yDimension: string;

	const height = 950;
	const width = 1700;
	const axisSpace = 75;
	const tickLength = 10;
	const legendWidth = 150;
	const buffer = 25;
	const margin = 0;
	const NumScale = 1 / 1000;

	$: x = data.map((d) => d[xDimension] * NumScale);
	$: y = data.map((d) => d[yDimension] * NumScale);

	$: xExtent = [0, max(x)];
	$: yExtent = [0, max(y)];

	$: xMin = axisSpace + buffer;
	$: xMax = width - buffer - legendWidth;
	$: vWidth = xMax - xMin;
	$: yMax = height - axisSpace - buffer;
	$: yMin = buffer + margin;
	$: vHeight = yMax - yMin;

	$: xScale = scaleLinear().domain([xExtent[0], xExtent[1]]).range([xMin, xMax]).nice();
	$: yScale = scaleLinear().domain([yExtent[0], yExtent[1]]).range([yMax, yMin]).nice();

	const pathLine_y = line()
		.x((d) => xScale(d[xDimension] * NumScale))
		.y((d) => yScale(d['N_rd_y'] * NumScale))
		.curve(curveBasis);
	const pathLine_z = line()
		.x((d) => xScale(d[xDimension] * NumScale))
		.y((d) => yScale(d['N_rd_z'] * NumScale))
		.curve(curveBasis);
	const eulerCurve_y = line()
		.x((d) => xScale(d[xDimension] * NumScale))
		.y((d) => yScale(d['N_eu_y'] * NumScale))
		.curve(curveBasis);
	const eulerCurve_z = line()
		.x((d) => xScale(d[xDimension] * NumScale))
		.y((d) => yScale(d['N_eu_z'] * NumScale))
		.curve(curveBasis);
	const sectionLine = line()
		.x((d) => xScale(d[xDimension] * NumScale))
		.y((d) => yScale(d['N_pl'] * NumScale))
		.curve(curveBasis);
</script>

<svg {height} {width}>
	<!-- Plot background -->
	<g transform={`translate(${xMin} ${yMin})`}>
		<rect height={vHeight} width={vWidth} stroke="black" />
	</g>

	<!-- Draw xAxis -->
	{#each xScale.ticks() as tick}
		<g transform={`translate(${xScale(tick)} ${yMax})`}>
			<line y1={-tickLength} y2={tickLength} stroke="black" />
			<text y={15 + tickLength} text-anchor="middle">{tick}</text>
		</g>
	{/each}
	<!-- Draw yAxis -->
	{#each yScale.ticks() as tick}
		<g transform={`translate(${xMin} ${yScale(tick)})`}>
			<line x1={-tickLength} x2={tickLength} stroke="black" />
			<text x="-20" dominant-baseline="middle" text-anchor="end">{tick.toFixed(0)}</text>
		</g>
	{/each}
	<!-- Draw vertical grid -->
	{#each xScale.ticks() as tick}
		<g transform={`translate(${xScale(tick)} )`}>
			<line y1={yMax} y2={yMin} stroke="black" />
		</g>
	{/each}
	<!-- Draw horisontal grid -->
	{#each yScale.ticks() as tick}
		<g transform={`translate(0 ${yScale(tick)} )`}>
			<line x1={xMin} x2={xMax} stroke="black" />
		</g>
	{/each}
	<!-- Plot data -->
	<g>
		<path class={'buckle-curve axis-y'} d={pathLine_y(data)} />
		<path class={'buckle-curve axis-z'} d={pathLine_z(data)} />
		<path class={'euler-curve axis-y'} d={eulerCurve_y(data)} />
		<path class={'euler-curve axis-z'} d={eulerCurve_z(data)} />
		<path class={'section-curve'} d={sectionLine(data)} />
	</g>
</svg>

<style>
	svg {
		background: transparent;
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
		flex-grow: 1;
	}
	text {
		fill: rgba(240, 230, 220);
		stroke: transparent;
	}
	rect {
		fill: rgba(255, 255, 255, 0.1);
		stroke-width: 2;
	}

	path {
		stroke-width: 3;
		fill: none;
		stroke-linecap: round;
	}
	.buckle-curve {
		stroke-width: 3;
	}
	.euler-curve {
		stroke-dasharray: 10;
	}
	.section-curve {
		stroke: #f28e2b;
	}
	.axis-y {
		stroke: #a0cbe8;
	}
	.axis-z {
		stroke: #4e79a7;
	}
</style>
