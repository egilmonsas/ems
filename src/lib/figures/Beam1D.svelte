<script lang="ts">
	// @ts-nocheck
	import type { CapacityResponse } from '$lib/types/capacityResponse';
	import { scaleLinear } from 'd3-scale';
	import { curveNatural, line } from 'd3-shape';
	import BeamForceArrow from '../components/BeamForceArrow.svelte';
	import BeamGrid from '../components/BeamGrid.svelte';
	import BeamLoadCurve from '../components/BeamLoadCurve.svelte';
	import BeamMomentArrow from '../components/BeamMomentArrow.svelte';
	import BeamOpplager from '../components/BeamOpplager.svelte';

	export let parentWidth: number, parentHeight: number, res: CapacityResponse;
	// Config
	const x_norms = [
		0, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8, 0.85,
		0.9, 0.95, 1.0
	];
	const margin = 0.15;

	// User input
	export let beam: Arrow;
	let self_weight = res.self_weight_kN_pr_meter ? res.self_weight_kN_pr_meter : 1;
	export let F0;
	export let M0;
	export let M1;

	// Opplager
	$: ang = beam_ang(beam);

	// Load calculations
	$: lineload = x_norms.map((d) => ({ x: d, y: self_weight, ang: (3 / 2) * Math.PI }));
	$: moment = x_norms.map((d) => ({
		x: d,
		y: compute_moment_at_x(d, beam, self_weight, M0, M1),
		ang: beam_ang(beam) + (1 / 2) * Math.PI
	}));
	$: shear = x_norms.map((d) => ({
		x: d,
		y: -compute_shear_at_x(d, beam, self_weight, M0, M1),
		ang: beam_ang(beam) + (1 / 2) * Math.PI
	}));
	$: axial = x_norms.map((d) => ({
		x: d,
		y: -compute_self_weight_axial_at_x(d, beam, self_weight) - F0,
		ang: beam_ang(beam) - (1 / 2) * Math.PI
	}));

	function compute_moment_at_x(
		x_norm: number,
		beam: Arrow,
		self_weight: number,
		M0: number,
		M1: number
	) {
		return -(
			0.5 *
				x_norm *
				Math.pow(beam_len(beam), 2) *
				self_weight *
				(1 - x_norm) *
				Math.cos(beam_ang(beam)) +
			M0 * (1 - x_norm) -
			M1 * x_norm
		);
	}

	function compute_shear_at_x(
		x_norm: number,
		beam: Arrow,
		self_weight: number,
		M0: number,
		M1: number
	) {
		return (
			beam_len(beam) * self_weight * (0.5 - x_norm) * Math.cos(beam_ang(beam)) -
			(M1 + M0) / beam_len(beam)
		);
	}
	function compute_self_weight_axial_at_x(x_norm: number, beam: Arrow, self_weight: number) {
		return beam_len(beam) * self_weight * (1 - x_norm) * Math.sin(beam_ang(beam));
	}

	// Draw

	$: xExtent = [beam.start_node.x, beam.end_node.x];
	$: yExtent = [beam.start_node.y, beam.end_node.y];
	$: xSpan = xExtent[1] - xExtent[0];
	$: ySpan = yExtent[1] - yExtent[0];

	$: xMidPoint = xExtent[0] + xSpan / 2;
	$: yMidPoint = yExtent[0] + ySpan / 2;

	$: xUtil = xSpan / parentWidth;
	$: yUtil = ySpan / parentHeight;
	$: Span = xUtil > yUtil ? xSpan : ySpan;

	$: vAspect = parentHeight / parentWidth;

	$: xZoom = xUtil > yUtil ? 1 : 1 / vAspect;
	$: yZoom = xUtil > yUtil ? vAspect : 1;
	$: Zoom = Math.sqrt(xZoom * yZoom);

	$: xDomain = [xMidPoint - (xZoom * Span) / 2, xMidPoint + (xZoom * Span) / 2];
	$: yDomain = [yMidPoint - (yZoom * Span) / 2, yMidPoint + (yZoom * Span) / 2];

	$: xScale = scaleLinear()
		.domain(xDomain)
		.range([parentWidth * margin, parentWidth * (1 - margin)]);
	$: yScale = scaleLinear()
		.domain(yDomain)
		.range([parentHeight * (1 - margin), parentHeight * margin]);

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
	<BeamOpplager {Zoom} {xScale} {yScale} point={beam.start_node} {ang} slide={false} />
	<BeamOpplager
		{Zoom}
		{xScale}
		{yScale}
		point={beam.end_node}
		ang={ang + Math.PI / 2}
		slide={true}
	/>

	<!-- Draw loads -->
	<BeamLoadCurve
		unit={'kN/m'}
		label={'q'}
		load={lineload}
		{xScale}
		{yScale}
		{beam}
		color={'pink'}
	/>
	<BeamLoadCurve unit={'kN'} label={'N'} load={axial} {xScale} {yScale} {beam} color={'red'} />
	<BeamLoadCurve unit={'kN'} label={'V'} load={shear} {xScale} {yScale} {beam} color={'green'} />
	<BeamLoadCurve unit={'kNm'} label={'M'} load={moment} {xScale} {yScale} {beam} color={'blue'} />

	<!-- Draw force arrow -->
	<BeamForceArrow
		{Zoom}
		{xScale}
		{yScale}
		point={beam.end_node}
		ang={ang - 2 * Math.PI}
		bind:F={F0}
	/>
	<BeamMomentArrow {Zoom} {xScale} {yScale} point={beam.start_node} {ang} bind:M={M0} />
	<BeamMomentArrow {Zoom} {xScale} {yScale} point={beam.end_node} {ang} bind:M={M1} />
</svg>

<style>
	svg {
		background: rgba(255, 255, 255, 0.1);
	}
</style>
