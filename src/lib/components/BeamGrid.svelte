<script lang="ts">
	// @ts-nocheck
	export let xScale;
	export let yScale;
	export let beam;
	$: x1 = beam.start_node.x;
	$: y1 = beam.start_node.y;
	$: x2 = beam.end_node.x;
	$: y2 = beam.end_node.y;

	$: xTicks = interval_grid(x1, x2, 1);
	$: yTicks = interval_grid(y1, y2, 1);

	function interval_grid(x1: number, x2: number, step) {
		let Ticks = [];

		if (x2 >= x1) {
			for (let i = x1; i < x2; i += step) {
				Ticks.push(i);
			}
			Ticks.push(x2);
		}
		if (x2 < x1) {
			for (let i = x1; i > x2; i += -step) {
				Ticks.push(i);
			}
			Ticks.push(x2);
		}
		return Ticks;
	}
</script>

{#each xTicks as tick}
	<line
		r="3"
		x1={xScale(tick)}
		y1={yScale(yTicks[0])}
		x2={xScale(tick)}
		y2={yScale(yTicks[yTicks.length - 1])}
		stroke="rgba(0,0,0,0.5)"
	/>
{/each}>
{#each yTicks as tick}
	<line
		r="3"
		x1={xScale(xTicks[0])}
		y1={yScale(tick)}
		x2={xScale(xTicks[xTicks.length - 1])}
		y2={yScale(tick)}
		stroke="rgba(0,0,0,0.5)"
	/>
{/each}>
{#if Math.abs(x2 - x1) > 1}
	<text
		font-size="20"
		fill="white"
		x={xScale((x1 + x2) / 2)}
		y={yScale(yTicks[0]) + 20}
		text-anchor="middle"
		dominant-baseline="middle">{Math.abs(x2 - x1).toFixed(1)} m</text
	>
{/if}
{#if Math.abs(y2 - y1) > 1}
	<text
		font-size="20"
		text-anchor="middle"
		dominant-baseline="middle"
		transform="rotate(270 {xScale(xTicks[xTicks.length - 1]) + 20} {yScale((y1 + y2) / 2)})"
		fill="white"
		x={xScale(xTicks[xTicks.length - 1]) + 20}
		y={yScale((y1 + y2) / 2)}>{Math.abs(y2 - y1).toFixed(1)} m</text
	>
{/if}
<line
	stroke-width="2"
	x1={xScale(x1)}
	y1={yScale(y1)}
	x2={xScale(x2)}
	y2={yScale(y1)}
	stroke="black"
/>
<line
	stroke-width="2"
	x1={xScale(x2)}
	y1={yScale(y1)}
	x2={xScale(x2)}
	y2={yScale(y2)}
	stroke="black"
/>
<line
	r="3"
	x1={xScale(x1)}
	y1={yScale(y2)}
	x2={xScale(x1 + 1)}
	y2={yScale(y2)}
	stroke="red"
	stroke-width="5"
/>
<line
	r="3"
	x1={xScale(x1)}
	y1={yScale(y2 - (1 * (y2 - y1)) / (x2 - x1))}
	x2={xScale(x1 + 1)}
	y2={yScale(y2)}
	stroke="red"
	stroke-width="5"
/>
