<script lang="ts">
	// @ts-nocheck
	export let xScale, yScale;
	export let load;
	export let beam;
	export let color;
	export let label;
	import { curveNatural, line } from 'd3-shape';

	$: momentLine = line()
		.x((d) => xScale(d.x))
		.y((d) => yScale(d.y))
		.curve(curveNatural);

	function find_extrema(force_vectors: Array<Vec3>): [Vec3, Vec3, number] {
		let force_min = force_vectors[0];
		let force_max = force_vectors[0];

		for (let i = 0; i < force_vectors.length; i++) {
			if (force_vectors[i].y < force_min.y) {
				force_min = force_vectors[i];
			}
			if (force_vectors[i].y > force_max.y) {
				force_max = force_vectors[i];
			}
		}
		let scale_factor = Math.max(1, Math.abs(force_min.y), Math.abs(force_max.y));

		return [force_min, force_max, scale_factor];
	}
	function get_node_along_beam(beam: Arrow, moment: Vec3): Vec2 {
		let x0 = beam.start_node.x;
		let y0 = beam.start_node.y;
		let dx = beam_dx(beam);
		let dy = beam_dy(beam);
		return { x: x0 + moment.x * dx, y: y0 + moment.x * dy };
	}
	function get_node_along_curve(beam: Arrow, moment: Vec3, max_load: number): Vec2 {
		let p0: Vec2 = get_node_along_beam(beam, moment);
		let p1: Vec2 = {
			x: p0.x + (Math.cos(moment.ang) * moment.y) / max_load,
			y: p0.y + (Math.sin(moment.ang) * moment.y) / max_load
		};
		return p1;
	}
	function beam_dx(beam: Arrow) {
		let dx = beam.end_node.x - beam.start_node.x;
		return dx;
	}
	function beam_dy(beam: Arrow) {
		let dy = beam.end_node.y - beam.start_node.y;
		return dy;
	}
	function get_moment_arrow(beam: Arrow, moment: Vec3, max_load: number): Arrow {
		let p0 = get_node_along_beam(beam, moment);
		let p1 = get_node_along_curve(beam, moment, max_load);
		let arrow: Arrow = { start_node: p0, end_node: p1 };
		return arrow;
	}
	interface Vec2 {
		x: number;
		y: number;
	}
	interface Vec3 {
		x: number;
		y: number;
		ang: number;
	}
	interface Arrow {
		start_node: Vec2;
		end_node: Vec2;
	}
	$: extrema = find_extrema(load);
	$: load_arrows = load.map((d) => get_moment_arrow(beam, d, extrema[2]));
	$: points_along_curve = load.map((d) => get_node_along_curve(beam, d, extrema[2]));

	let show = false;
	function mouseEnter() {
		show = true;
	}
	function mouseExit() {
		show = false;
	}
</script>

{#each load_arrows as arrow}
	<line
		r="3"
		x1={xScale(arrow.start_node.x)}
		y1={yScale(arrow.start_node.y)}
		x2={xScale(arrow.end_node.x)}
		y2={yScale(arrow.end_node.y)}
		stroke={color}
	/>
{/each}>
<path
	d={momentLine(points_along_curve)}
	stroke={color}
	stroke-width="5"
	on:mouseenter={mouseEnter}
	on:mouseleave={mouseExit}
/>

<!-- Annotate, show max and min only on hover -->
{#if show}
	<text
		x={xScale(get_node_along_curve(beam, extrema[1], extrema[1].y).x)}
		y={yScale(get_node_along_curve(beam, extrema[1], extrema[1].y).y) - 20}
		>{label}_max={extrema[1].y.toFixed(2)} kNm</text
	>

	<circle
		r="6"
		stroke={color}
		fill={color}
		cx={xScale(get_node_along_curve(beam, extrema[1], extrema[1].y).x)}
		cy={yScale(get_node_along_curve(beam, extrema[1], extrema[1].y).y)}
	/>
	<!-- Refrain from annotating both if value is the same along length -->
	{#if extrema[1].y != extrema[0].y}
		<circle
			r="6"
			stroke={color}
			fill={color}
			cx={xScale(get_node_along_curve(beam, extrema[0], extrema[1].y).x)}
			cy={yScale(get_node_along_curve(beam, extrema[0], extrema[1].y).y)}
		/>
		<text
			x={xScale(get_node_along_curve(beam, extrema[0], extrema[1].y).x)}
			y={yScale(get_node_along_curve(beam, extrema[0], extrema[1].y).y) - 20}
			>{label}_min={extrema[0].y.toFixed(2)} kN</text
		>
	{/if}
{/if}

<style>
	path {
		fill: none;
	}
	text {
		font-size: 20;
		fill: white;
	}
</style>
