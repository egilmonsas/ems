<script lang="ts">
	// @ts-nocheck
	import ResponseTable from '$lib/components/ResponseTable.svelte';
	import DataProperty from '$lib/units/DataProperty.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let DCToShow: Array<DataProperty> = [];
	let res = '';
	export async function execute(
		crsKind: string,
		crsType: string,
		material: String,
		N_kN: number,
		My_kNm: number,
		Mz_kNm: number,
		length: number,
		beta_ky: number,
		beta_kz: number,
		beta_kltb: number,
		buckleCurveY: String,
		buckleCurveZ: String,
		ltbCurve: String
	) {
		let DcResponse: designCheckResponse = await invoke('perform_design_check', {
			crstype: crsKind,
			name: crsType,
			material: material,
			n: N_kN * 1000,
			my: My_kNm * 1000000,
			mz: Mz_kNm * 1000000,
			cMy: 0.95,
			cMz: 0.95,
			muCr: 1.5,
			length: length * 1000,
			betaKy: beta_ky,
			betaKz: beta_kz,
			betaKltb: beta_kltb,
			buckleCurveY: buckleCurveY,
			buckleCurveZ: buckleCurveZ,
			ltbCurve: ltbCurve
		});
		DCToShow = [
			{
				id: '6.2',
				val: DcResponse.util_6_2 ? DcResponse.util_6_2.toFixed(3) : 'NaN',
				unit: '%',
				tooltip: 'Tverrsnitskapasitet'
			},
			{
				id: '6.46_y',
				val: DcResponse.util_6_46_y ? DcResponse.util_6_46_y.toFixed(3) : 'NaN',
				unit: '%',
				tooltip: 'Bøyeknekking om sterk akse'
			},
			{
				id: '6.46_z',
				val: DcResponse.util_6_46_z ? DcResponse.util_6_46_z.toFixed(3) : 'NaN',
				unit: '%',
				tooltip: 'Bøyeknekking om svak akse'
			},
			{
				id: '6.61',
				val: DcResponse.util_6_61 ? DcResponse.util_6_61.toFixed(3) : 'NaN',
				unit: '%',
				tooltip: 'Samvirke, bøyeknekking om sterk akse og vipping'
			},
			{
				id: '6.62',
				val: DcResponse.util_6_62 ? DcResponse.util_6_62.toFixed(3) : 'NaN',
				unit: '%',
				tooltip: 'Samvirke, bøyeknekking om svak akse og vipping'
			}
		];
		test = '';
	}
</script>

<ResponseTable title={'DC'} dataToShow={DCToShow} />
