<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import DataProperty from '../units/DataProperty.svelte';
	import type { CrossSectionResponse } from '$lib/types/crossSectionResponse';
	import { afterUpdate } from 'svelte';

	export let crsKind: string;

	let want_to_show: boolean = true;
	let have_data_to_show: boolean = false;
	let json_res: CrossSectionResponse = {};

	async function toggle_cross_section_data_view() {
		want_to_show = !want_to_show;
	}

	export async function execute(crsKind: string, crsType: string) {
		json_res = await invoke('get_area', { crstype: crsKind, name: crsType });
		have_data_to_show = true;
	}

	export async function forget() {
		json_res = {};
		have_data_to_show = false;
	}
</script>

<div id="cross-section-properties" class="overflow">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<h1 on:click={toggle_cross_section_data_view}>Egenskaper</h1>
	{#if want_to_show && have_data_to_show}
		<div>
			{#if crsKind == 'HEB'}
				<DataProperty props={{ id: 'b', val: json_res.width?.toString(), unit: 'mm' }} />
				<DataProperty props={{ id: 'h', val: json_res.height?.toString(), unit: 'mm' }} />
			{/if}
			{#if crsKind == 'CHS'}
				<DataProperty props={{ id: 'd', val: json_res.height?.toString(), unit: 'mm' }} />
			{/if}
			<DataProperty props={{ id: 'A', val: json_res.area?.toExponential(2), unit: 'mm^2' }} />

			{#if crsKind == 'HEB'}
			<DataProperty props={{ id: 'A_{v,pl,y}', val: json_res.A_v_y?.toExponential(2), unit: 'mm^2' }} />
			
			<DataProperty
			props={{ id: 'w_{el,y}', val: json_res.w_el_y?.toExponential(2), unit: 'mm^3' }}
			/>
			<DataProperty
			props={{ id: 'w_{pl,y}', val: json_res.w_pl_y?.toExponential(2), unit: 'mm^3' }}
			/>
			<DataProperty props={{ id: 'A_{v,pl,z}', val: json_res.A_v_z?.toExponential(2), unit: 'mm^2' }} />
				<DataProperty
					props={{ id: 'w_{el,z}', val: json_res.w_el_z?.toExponential(2), unit: 'mm^3' }}
				/>
				<DataProperty
					props={{ id: 'w_{pl,z}', val: json_res.w_pl_z?.toExponential(2), unit: 'mm^3' }}
				/>
				<DataProperty props={{ id: 'I_y', val: json_res.I_y?.toExponential(2), unit: 'mm^4' }} />
				<DataProperty props={{ id: 'I_z', val: json_res.I_z?.toExponential(2), unit: 'mm^4' }} />
			{/if}
			{#if crsKind == 'CHS'}
			<DataProperty props={{ id: 'A_{v,pl}', val: json_res.A_v_y?.toExponential(2), unit: 'mm^2' }} />

				<DataProperty
					props={{ id: 'w_{el}', val: json_res.w_el_y?.toExponential(2), unit: 'mm^3' }}
				/>
				<DataProperty
					props={{ id: 'w_{pl}', val: json_res.w_pl_y?.toExponential(2), unit: 'mm^3' }}
				/>
				<DataProperty props={{ id: 'I', val: json_res.I_y?.toExponential(2), unit: 'mm^3' }} />
			{/if}
		</div>
	{/if}
</div>

<style>
	:root {
		--select-border: rgb(100, 100, 100);
		--focus-border: rgb(150, 150, 150);

		--select-focus: blue;
		--select-arrow: var(--select-border);
	}

	.overflow {
		display: inline-block;
		overflow: hidden;
		vertical-align: bottom;
	}
</style>
