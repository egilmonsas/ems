<script lang="ts">
	// @ts-nocheck
	import { curveNatural, line } from 'd3-shape';
	export let xScale, yScale, point, ang, M, Zoom;

	const side_length = 1 * Zoom;
	interface Vec2 {
		x: number;
		y: number;
	}
	const theta_norms = [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

	$: angStart = ang - Math.PI * 0.25;
	$: angStop = ang + Math.PI;
	$: dAng = angStop - angStart;
	function draw_curve(p0: Vec2, stepLength: number, stepAngle: number): Vec2 {
		let x = p0.x + Math.cos(stepAngle) * stepLength;
		let y = p0.y + Math.sin(stepAngle) * stepLength;

		return { x: x, y: y };
	}
	$: points = theta_norms.map((d) => draw_curve(point, side_length, angStart + d * dAng));
	$: pathLine = line()
		.x((d: Vec2) => xScale(d.x))
		.y((d: Vec2) => yScale(d.y))
		.curve(curveNatural);
	function step_along(p0: Vec2, stepLength: number, stepAngle: number): Vec2 {
		let x = p0.x - Math.cos(stepAngle) * stepLength;
		let y = p0.y - Math.sin(stepAngle) * stepLength;

		return { x: x, y: y };
	}
</script>

<path d={pathLine(points)} stroke="blue" stroke-width="3" fill="none" />
<path
	d={pathLine([
		points[0],
		step_along(points[0], -side_length / 4, angStart + Math.PI * (1 / 2 - 1 / 4))
	])}
	stroke="blue"
	stroke-width="3"
/>
<path
	d={pathLine([
		points[0],
		step_along(points[0], -side_length / 4, angStart + Math.PI * (1 / 2 + 1 / 4))
	])}
	stroke="blue"
	stroke-width="3"
/>

<foreignObject
	width="50"
	height="23"
	x={xScale(step_along(points[0], -side_length / 3, angStart + Math.PI * (1 / 2 - 1 / 4)).x)}
	y={yScale(step_along(points[0], -side_length / 3, angStart + Math.PI * (1 / 2 - 1 / 4)).y)}
>
	<input id="input" class="input-real" type="number" bind:value={M} />
</foreignObject>

<style>
	input {
		background: rgba(0, 0, 0, 0);
		border: 0;
		color: blue;
		font-size: 1.5em;
	}
</style>
