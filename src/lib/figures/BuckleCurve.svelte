<script lang="ts">
	// @ts-nocheck
	import type { BuckleResponse } from '$lib/types/buckleResponse.ts';
	import Plotly from 'plotly.js-dist';
	import { onMount } from 'svelte';

	export let data: BuckleResponse[];
	export let parentWidth, parentHeight;
	export let crsType = '';

	const fig_color = 'rgba(255,255,255,0)';
	const axis_y_color = '#a0cbe8';
	const axis_z_color = '#4e79a7';
	const section_cap_color = '#f28e2b';
	const grid_color = 'rgb(50,50,50)';
	const tickwidth = 2;
	const ticklen = 10;
	let diagonal = Math.pow(Math.pow(parentWidth, 2) + Math.pow(parentHeight, 2), 0.5);
	onMount(() => {
		var N_pl = {
			x: data.map((d) => d['L_k'] / 1000),
			y: data.map((d) => d['N_pl'] / 1000),
			mode: 'lines',
			name: 'N_pl',
			line: {
				color: section_cap_color
			}
		};
		var ymax = data[0]['N_pl'];
		var N_rd_y = {
			x: data.map((d) => d['L_k'] / 1000),
			y: data.map((d) => d['N_rd_y'] / 1000),
			mode: 'lines',
			line: {
				shape: 'spline',
				color: axis_y_color
			},
			name: 'N_b_rd_y'
		};
		var N_rd_z = {
			x: data.map((d) => d['L_k'] / 1000),
			y: data.map((d) => d['N_rd_z'] / 1000),
			mode: 'lines',
			line: {
				shape: 'spline',
				color: axis_z_color
			},
			name: 'N_b_rd_z'
		};
		var N_eu_y = {
			x: data.map((d) => d['L_k'] / 1000),
			y: data.map((d) => d['N_eu_y'] / 1000),
			mode: 'lines',
			line: {
				dash: 'dash',
				shape: 'spline',
				color: axis_y_color
			},
			name: 'N_eu_y'
		};
		var N_eu_z = {
			x: data.map((d) => d['L_k'] / 1000),
			y: data.map((d) => d['N_eu_z'] / 1000),
			mode: 'lines',
			line: {
				dash: 'dash',
				shape: 'spline',
				color: axis_z_color
			},
			name: 'N_eu_z'
		};
		var dataToPlot = [N_pl, N_rd_y, N_rd_z, N_eu_y, N_eu_z];

		var layout = {
			width: parentWidth,
			height: parentHeight,
			title: crsType,
			xaxis: {
				title: 'Knekklengde [m]',
				range: [0, 20],
				gridcolor: grid_color,
				nticks: 20,
				ticks: 'inside',
				linecolor: 'black',
				tickcolor: 'black',
				mirror: true,
				ticklen: ticklen,
				tickwidth: tickwidth,
				minor: {
					ticks: 'inside',
					nticks: 2,
					ticklen: ticklen * 0.75,
					tickwidth: tickwidth
				},
				automargin: true
			},
			yaxis: {
				title: 'Kapasitet [kN]',
				range: [0, (1.1 * ymax) / 1000],
				gridcolor: grid_color,
				nticks: 20,
				tickcolor: 'black',
				ticks: 'inside',
				linecolor: 'black',
				mirror: true,
				ticklen: ticklen,
				tickwidth: tickwidth,
				minor: {
					ticks: 'inside',
					nticks: 2,
					ticklen: ticklen * 0.75,
					tickwidth: tickwidth
				},
				automargin: true
			},
			paper_bgcolor: fig_color,
			plot_bgcolor: fig_color,
			font: {
				size: diagonal / 125
			},
			modebar: {
				orientation: 'v',
				bgcolor: 'rgba(100,100,100,0.0)',
				color: 'rgb(0, 122, 204)',
				remove: ['editInChartStudio', 'editinchartstudio', 'autoscale']
			}
		};
		let plotDiv = document.getElementById('plotDiv');

		var config = {
			showSendToCloud: true,
			displaylogo: false,
			editable: true,
			displayModeBar: true
		};

		// @ts-ignore
		let Plot = Plotly.newPlot(plotDiv, dataToPlot, layout, config);
	});
</script>

<div id="plotly">
	<div id="plotDiv" />
</div>
