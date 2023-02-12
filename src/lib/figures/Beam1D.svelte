<script lang="ts">
	// @ts-nocheck
	import type { CapacityResponse } from '$lib/types/capacityResponse';
	import { scaleLinear } from 'd3-scale';
	import { curveNatural, line } from 'd3-shape';
	import BeamForceArrow from '../components/BeamForceArrow.svelte';
	import BeamGrid from '../components/BeamGrid.svelte';
	import BeamLoadCurve from '../components/BeamLoadCurve.svelte';
	import BeamOpplager from '../components/BeamOpplager.svelte';

	export let parentWidth: number, parentHeight: number, res: CapacityResponse;
	// Config
	const x_norms = [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
	const margin = 300;

	// User input
	export let beam: Arrow;
	let self_weight = res.self_weight_kN_pr_meter;
	let F0 = 700;

	// Opplager
	$: ang = beam_ang(beam);

	// Load calculations
	$: lineload = x_norms.map((d) => ({ x: d, y: self_weight, ang: (3 / 2) * Math.PI }));
	$: moment = x_norms.map((d) => ({
		x: d,
		y: compute_self_weight_moment_at_x(d, beam, self_weight),
		ang: beam_ang(beam) - (1 / 2) * Math.PI
	}));
	$: shear = x_norms.map((d) => ({
		x: d,
		y: compute_self_weight_shear_at_x(d, beam, self_weight),
		ang: beam_ang(beam) + (1 / 2) * Math.PI
	}));
	$: axial = x_norms.map((d) => ({
		x: d,
		y: compute_self_weight_axial_at_x(d, beam, self_weight) + F0,
		ang: beam_ang(beam) + (1 / 2) * Math.PI
	}));

	function compute_self_weight_moment_at_x(x_norm: number, beam: Arrow, self_weight: number) {
		return (
			0.5 *
			x_norm *
			Math.pow(beam_len(beam), 2) *
			self_weight *
			(1 - x_norm) *
			Math.cos(beam_ang(beam))
		);
	}
	function compute_self_weight_shear_at_x(x_norm: number, beam: Arrow, self_weight: number) {
		return beam_len(beam) * self_weight * (0.5 - x_norm) * Math.cos(beam_ang(beam));
	}
	function compute_self_weight_axial_at_x(x_norm: number, beam: Arrow, self_weight: number) {
		return beam_len(beam) * self_weight * (1 - x_norm) * Math.sin(beam_ang(beam));
	}

	// Draw
	$: aspect = (parentHeight - 2 * margin) / (parentWidth - 2 * margin);
	$: xExtent = [beam.start_node.x, beam.end_node.x];
	$: xSpan = xExtent[1] - xExtent[0];
	$: xMidPoint = xExtent[0] + xSpan / 2;

	$: yExtent = [beam.start_node.y, beam.end_node.y];
	$: ySpan = yExtent[1] - yExtent[0];
	$: yMidPoint = yExtent[0] + ySpan / 2;

	$: xScale = scaleLinear()
		.domain([xMidPoint - xSpan / 2, xMidPoint + xSpan / 2])
		.range([margin, parentWidth - margin]);
	$: yScale = scaleLinear()
		.domain([yMidPoint - (xSpan * aspect) / 2, yMidPoint + (xSpan * aspect) / 2])
		.range([parentHeight - margin, margin]);

	interface Vec2 {
		x: number;
		y: number;
	}

	interface Arrow {
		start_node: Vec2;
		end_node: Vec2;
	}

	function beam_dx(beam: Arrow) {
		return beam.end_node.x - beam.start_node.x;
	}
	function beam_dy(beam: Arrow) {
		return beam.end_node.y - beam.start_node.y;
	}
	export function beam_len(beam: Arrow) {
		return Math.sqrt(Math.pow(beam_dx(beam), 2) + Math.pow(beam_dy(beam), 2));
	}
	function beam_ang(beam: Arrow) {
		return Math.atan2(beam_dy(beam), beam_dx(beam));
	}

	// the path generator
	$: pathLine = line()
		.x((d: Vec2) => xScale(d.x))
		.y((d: Vec2) => yScale(d.y))
		.curve(curveNatural);
</script>

<svg width={parentWidth} height={parentHeight}>
	<!-- Grid -->
	<BeamGrid {beam} {xScale} {yScale} />
	<!-- Draw beam -->
	<path d={pathLine([beam.start_node, beam.end_node])} stroke="black" stroke-width="10" />

	<!-- Draw opplager -->
	<BeamOpplager {xScale} {yScale} point={beam.start_node} {ang} slide={false} />
	<BeamOpplager {xScale} {yScale} point={beam.end_node} ang={ang + Math.PI / 2} slide={true} />

	<!-- Draw loads -->
	<BeamLoadCurve label={'q'} load={lineload} {xScale} {yScale} {beam} color={'pink'} />
	<BeamLoadCurve label={'N'} load={axial} {xScale} {yScale} {beam} color={'red'} />
	<BeamLoadCurve label={'M'} load={moment} {xScale} {yScale} {beam} color={'blue'} />
	<BeamLoadCurve label={'S'} load={shear} {xScale} {yScale} {beam} color={'green'} />

	<!-- Draw force arrow -->
	<BeamForceArrow {xScale} {yScale} point={beam.end_node} ang={ang - 2 * Math.PI} F={F0} />
</svg>

<style>
	svg {
		background: rgba(255, 255, 255, 0.1);
	}
</style>
