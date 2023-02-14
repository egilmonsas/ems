<script lang="ts">
	// @ts-nocheck
	import { line } from 'd3-shape';
	export let xScale, yScale, point, ang, slide, Zoom;
	const side_length = 1 * Zoom;
	interface Vec2 {
		x: number;
		y: number;
	}

	$: ang1 = ang + Math.PI / 6;
	$: ang2 = ang - Math.PI / 6;
	function step_along(p0: Vec2, stepLength: number, stepAngle: number): Vec2 {
		let x = p0.x - Math.cos(stepAngle) * stepLength;
		let y = p0.y - Math.sin(stepAngle) * stepLength;

		return { x: x, y: y };
	}
	$: pathLine = line()
		.x((d: Vec2) => xScale(d.x))
		.y((d: Vec2) => yScale(d.y));
</script>

<path
	d={pathLine([
		point,
		step_along(point, side_length, ang1),
		step_along(point, side_length, ang2),
		point
	])}
	stroke-width="3"
/>

{#if slide}
	<path
		d={pathLine([
			step_along(point, side_length * 1.3, ang1),
			step_along(point, side_length * 1.3, ang2)
		])}
		stroke="black"
		stroke-width="10"
	/>
{:else}
	{#each [0, 0.25, 0.5, 0.75, 1.0] as x}
		<path
			d={pathLine([
				step_along(step_along(point, side_length * 1, ang1), side_length * x, ang - Math.PI / 2),
				step_along(
					step_along(step_along(point, side_length * 1, ang1), side_length * x, ang - Math.PI / 2),
					side_length * 0.25,
					ang1 - Math.PI / 3
				)
			])}
			stroke="black"
			stroke-width="3"
		/>
	{/each}
{/if}
<circle r="10" stroke="black" fill="black" cx={xScale(point.x)} cy={yScale(point.y)} />
