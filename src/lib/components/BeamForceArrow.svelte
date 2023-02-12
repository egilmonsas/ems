<script lang="ts">
	// @ts-nocheck
	import { curveNatural, line } from 'd3-shape';
	export let xScale, yScale, point, ang, F;
	const side_length = 1;
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
		.y((d: Vec2) => yScale(d.y))
		.curve(curveNatural);
</script>

<path
	d={pathLine([
		step_along(point, -side_length / 3, ang),
		step_along(step_along(point, -side_length / 3, ang), -side_length / 2, ang1)
	])}
	stroke="red"
	stroke-width="3"
/>
<path
	d={pathLine([
		step_along(point, -side_length / 3, ang),
		step_along(step_along(point, -side_length / 3, ang), -side_length / 2, ang2)
	])}
	stroke="red"
	stroke-width="3"
/>
<path
	d={pathLine([
		step_along(point, -side_length / 3, ang),
		step_along(point, -1.5 * side_length, ang)
	])}
	stroke="red"
	stroke-width="3"
/>

<text
	font-size="20"
	x={xScale(step_along(point, -1.5 * side_length, ang).x) + 10}
	y={yScale(step_along(point, -1.5 * side_length, ang).y)}
	stroke="red">F_0 = {F} kN</text
>
