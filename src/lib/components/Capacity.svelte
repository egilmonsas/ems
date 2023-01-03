<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import DataProperty from '../units/DataProperty.svelte';
	import type { CapacityResponse } from '$lib/types/capacityResponse';
	import type { MaterialResponse } from '$lib/types/materialResponse';

	export let crsKind: string;

	let want_to_show_stiffness: boolean = true;
	let want_to_show_capacity: boolean = true;
	let want_to_show_material: boolean = true;

	let have_data_to_show: boolean = false;
	let json_res: CapacityResponse = {};
	let material_res: MaterialResponse = {};

	let d: {
		N_pl: number | undefined;
		V_pl_y: number | undefined,
		V_pl_z: number | undefined,
		M_el_y: number | undefined;
		M_pl_y: number | undefined;
		M_el_z: number | undefined;
		M_pl_z: number | undefined;
	};
	async function toggle_material_view() {
		want_to_show_material = !want_to_show_material;
	}
	async function toggle_cross_section_stiffness_view() {
		want_to_show_stiffness = !want_to_show_stiffness;
	}
	async function toggle_cross_section_capacity_view() {
		want_to_show_capacity = !want_to_show_capacity;
	}
	export async function execute(
		crsKind: string,
		crsType: string,
		material: string,
		icludeSafetyFactor: boolean
	) {
		json_res = await invoke('get_capacity', {
			crstype: crsKind,
			name: crsType,
			material: material
		});
		d = {
			N_pl: icludeSafetyFactor ? json_res.N_pl_d : json_res.N_pl_k,
			V_pl_y: icludeSafetyFactor ? json_res.V_pl_y_d : json_res.V_pl_y_k,
			V_pl_z: icludeSafetyFactor ? json_res.V_pl_z_d : json_res.V_pl_z_k,
			M_el_y: icludeSafetyFactor ? json_res.M_el_y_d : json_res.M_el_y_k,
			M_pl_y: icludeSafetyFactor ? json_res.M_pl_y_d : json_res.M_pl_y_k,
			M_el_z: icludeSafetyFactor ? json_res.M_el_z_d : json_res.M_el_z_k,
			M_pl_z: icludeSafetyFactor ? json_res.M_pl_z_d : json_res.M_pl_z_k
		};

		material_res = await invoke('get_material_properties', {
			material: material
		});
		have_data_to_show = true;
	}

	export async function forget() {
		json_res = {};
		material_res = {};
		have_data_to_show = false;
	}
</script>

<div id="cross-section-properties" class="overflow">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<h1 on:click={toggle_material_view}>Material</h1>
	{#if want_to_show_material && have_data_to_show}
		<div>
			<DataProperty props={{ id: 'E', val: material_res.E?.toFixed(0), unit: 'MPa' }} />
			<DataProperty props={{ id: '\\rho', val: material_res.rho?.toFixed(0), unit: 'kg/m^3' }} />
			<DataProperty props={{ id: 'f_y', val: material_res.f_y?.toFixed(0), unit: 'MPa' }} />
			<DataProperty props={{ id: 'f_{y,d}', val: material_res.f_y_d?.toFixed(0), unit: 'MPa' }} />
			<DataProperty props={{ id: 'f_u', val: material_res.f_u?.toFixed(0), unit: 'MPa' }} />
			<DataProperty props={{ id: 'f_{u,d}', val: material_res.f_u_d?.toFixed(0), unit: 'MPa' }} />
			<DataProperty
				props={{ id: '\\gamma _{M0}', val: material_res.gamma_m0?.toFixed(2), unit: '' }}
			/>
			<DataProperty
			props={{ id: '\\gamma _{M1}', val: material_res.gamma_m1?.toFixed(2), unit: '' }}
		/>
		</div>
	{/if}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<h1 on:click={toggle_cross_section_stiffness_view}>Stivhet</h1>
	{#if want_to_show_stiffness && have_data_to_show}
		<div>
			<DataProperty props={{ id: 'EA', val: json_res.EA?.toExponential(2), unit: 'N' }} />
			{#if crsKind == 'HEB'}
				<DataProperty
					props={{ id: 'EI_y', val: json_res.EI_y?.toExponential(2), unit: 'N mm^2' }}
				/>
				<DataProperty
					props={{ id: 'EI_z', val: json_res.EI_z?.toExponential(2), unit: 'N mm^2' }}
				/>
			{/if}
			{#if crsKind == 'CHS'}
				<DataProperty props={{ id: 'EI', val: json_res.EI_y?.toExponential(2), unit: 'N mm^2' }} />
			{/if}
		</div>
	{/if}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<h1 on:click={toggle_cross_section_capacity_view}>Kapasitet</h1>
	{#if want_to_show_capacity && have_data_to_show}
		<div>
			<DataProperty
				props={{
					id: 'N_{pl}',
					val: d.N_pl ? (d.N_pl / 1000).toFixed(0) : '0',
					unit: 'kN '
				}}
			/>
			{#if crsKind == 'HEB'}
			<DataProperty
			props={{
				id: 'V_{pl,y}',
				val: d.V_pl_y ? (d.V_pl_y / 1000).toFixed(0) : '0',
				unit: 'kN'
			}}
		/>
		<DataProperty
		props={{
			id: 'V_{pl,z}',
			val: d.V_pl_z ? (d.V_pl_z / 1000).toFixed(0) : '0',
			unit: 'kN'
		}}
	/>
				<DataProperty
					props={{
						id: 'M_{el,y}',
						val: d.M_el_y ? (d.M_el_y / 1000000).toFixed(0) : '0',
						unit: 'kN m'
					}}
				/>
				<DataProperty
					props={{
						id: 'M_{pl,y}',
						val: d.M_pl_y ? (d.M_pl_y / 1000000).toFixed(0) : '0',
						unit: 'kN m'
					}}
				/>
				<DataProperty
					props={{
						id: 'M_{el,z}',
						val: d.M_el_z ? (d.M_el_z / 1000000).toFixed(0) : '0',
						unit: 'kN m'
					}}
				/>
				<DataProperty
					props={{
						id: 'M_{pl,z}',
						val: d.M_pl_z ? (d.M_pl_z / 1000000).toFixed(0) : '0',
						unit: 'kN m'
					}}
				/>
			{/if}
			{#if crsKind == 'CHS'}
			<DataProperty
			props={{
				id: 'V_{pl}',
				val: d.V_pl_y ? (d.V_pl_y / 1000).toFixed(0) : '0',
				unit: 'kN m'
			}}
		/>
				<DataProperty
					props={{
						id: 'M_{el}',
						val: d.M_el_y ? (d.M_el_y / 1000000).toFixed(0) : '0',
						unit: 'kN m'
					}}
				/>
				<DataProperty
					props={{
						id: 'M_{pl}',
						val: d.M_pl_y ? (d.M_pl_y / 1000000).toFixed(0) : '0',
						unit: 'kN m'
					}}
				/>
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
