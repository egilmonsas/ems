<script lang="ts">
	import type { CapacityResponse } from '$lib/types/capacityResponse';
	import type { MaterialResponse } from '$lib/types/materialResponse';
	import { invoke } from '@tauri-apps/api/tauri';
	import ResponseTable from './ResponseTable.svelte';

	interface DataProp {
		id?: string;
		val?: string;
		unit?: string;
	}
	let materialDataToShow:	 	Array<DataProp> = [] ;
	let stiffnessDataToShow:	Array<DataProp> = [] ;
	let capacityDataToShow:		Array<DataProp> = [] ;
	let CrsClassDataToShow:		Array<DataProp> = [] ;

	export let icludeSafetyFactor:boolean;

	export async function execute(
		crsKind: string,
		crsType: string,
		material: string,
	) {
		let capacityResponse:CapacityResponse = await invoke('get_capacity', {
			crstype: crsKind,
			name: crsType,
			material: material
		});

		let materialResponse:MaterialResponse = await invoke('get_material_properties', {
			material: material
		});
		materialDataToShow=[
			{id: 'E', val: materialResponse.E.toFixed(0), unit: 'Nmm^2'},
			{id: '\\rho', val: materialResponse.rho.toFixed(0), unit: 'kg/m^3'},
			{id: 'f_{y,k}', val: materialResponse.f_y.toFixed(0), unit: 'MPa'},
			{id: 'f_{y,d}', val: materialResponse.f_y_d.toFixed(0), unit: 'MPa'},
			{id: 'f_{u,k}', val: materialResponse.f_u.toFixed(0), unit: 'MPa'},
			{id: 'f_{u,d}', val: materialResponse.f_u_d.toFixed(0), unit: 'MPa'},
			{id: '\\gamma_{M0}', val: materialResponse.gamma_m0.toFixed(2), unit: ''},
			{id: '\\gamma_{M1}', val: materialResponse.gamma_m1.toFixed(2), unit: ''},
		]
		stiffnessDataToShow=[
			// All sections
			{id: 'w_{kg}', val: ((capacityResponse.self_weight_kg_pr_meter)).toFixed(1), unit: 'kg/m'},
			{id: 'w_{kN}', val: ((capacityResponse.self_weight_kN_pr_meter)).toFixed(1), unit: 'kN/m'},
			{id: 'EA', val: (capacityResponse.EA).toExponential(2), unit: 'N'},

			// Symmetric sections
			{id: 'EI', val: capacityResponse.EI?capacityResponse.EI.toExponential(2):"NaN", unit: 'Nmm^2'},
			
			// // Assymetric sections
			{id: 'EI_y', val: capacityResponse.EI_y?capacityResponse.EI_y.toExponential(2):"NaN", unit: 'Nmm^2'},
			{id: 'EI_z', val: capacityResponse.EI_y?capacityResponse.EI_z.toExponential(2):"NaN", unit: 'Nmm^2'},
		]
		CrsClassDataToShow=[
			{id: '', val: capacityResponse.Cross_section_class, unit: ''},
			{id: 'Steg, bøyning', val: capacityResponse.Cross_section_class_web_bending, unit: ''},
			{id: 'Steg, trykk', val: capacityResponse.Cross_section_class_web_compression , unit: ''},
			{id: 'Flens, trykk', val: capacityResponse.Cross_section_class_flange_compression, unit: ''},
		]
		capacityDataToShow=[			
			// All sections
			{id: 'N_{pl}', val: ((icludeSafetyFactor? capacityResponse.N_pl_d:capacityResponse.N_pl_k) / 1000).toFixed(0), unit: 'kN'},
			
			// Symmetric sections
			{id: 'V_{pl}', val: ((icludeSafetyFactor? capacityResponse.V_pl_d:capacityResponse.V_pl_k) / 1000).toFixed(0), unit: 'kN'},
			{id: 'M_{el}', val: ((icludeSafetyFactor? capacityResponse.M_el_d:capacityResponse.M_el_k) / 1000000).toFixed(0), unit: 'kNm'},
			{id: 'M_{pl}', val: ((icludeSafetyFactor? capacityResponse.M_pl_d:capacityResponse.M_pl_k) / 1000000).toFixed(0), unit: 'kNm'},
			
			// Assymetric sections
			{id: 'V_{pl,y}', val: ((icludeSafetyFactor? capacityResponse.V_pl_y_d:capacityResponse.V_pl_y_k) / 1000).toFixed(0), unit: 'kN'},
			{id: 'V_{pl,z}', val: ((icludeSafetyFactor? capacityResponse.V_pl_z_d:capacityResponse.V_pl_z_k) / 1000).toFixed(0), unit: 'kN'},
			{id: 'M_{el,y}', val: ((icludeSafetyFactor? capacityResponse.M_el_y_d:capacityResponse.M_el_y_k) / 1000000).toFixed(0), unit: 'kNm'},
			{id: 'M_{pl,y}', val: ((icludeSafetyFactor? capacityResponse.M_pl_y_d:capacityResponse.M_pl_y_k) / 1000000).toFixed(0), unit: 'kNm'},
			{id: 'M_{el,z}', val: ((icludeSafetyFactor? capacityResponse.M_el_z_d:capacityResponse.M_el_z_k) / 1000000).toFixed(0), unit: 'kNm'},
			{id: 'M_{pl,z}', val: ((icludeSafetyFactor? capacityResponse.M_pl_z_d:capacityResponse.M_pl_z_k) / 1000000).toFixed(0), unit: 'kNm'},
		]
	}

</script>

<div id="cross-section-properties" class="overflow">
	<ResponseTable title={"Material"} dataToShow={materialDataToShow}/>
	<ResponseTable title={"Stivhet"} dataToShow={stiffnessDataToShow}/>
	<ResponseTable title={"Tverrsnittsklasse"} dataToShow={CrsClassDataToShow}/>
	<ResponseTable title={"Kapasitet"} dataToShow={capacityDataToShow}/>
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
