<script lang="ts">
	import CHS from '$lib/assets/crossSections/CHS.png';
	import HEB from '$lib/assets/crossSections/HEB.png';

	import type { CrossSectionResponse } from '$lib/types/crossSectionResponse';
	import { invoke } from '@tauri-apps/api/tauri';
	import ResponseTable from './ResponseTable.svelte';
	interface DataProp {
		id?: string;
		val?: string;
		unit?: string;
	}
	let crossSectionDataToShow: Array<DataProp> = [];
	let image_to_show: any;
	export let crsKind: string;
	$: switch (crsKind) {
		case 'HEB':
			image_to_show = HEB;
			break;
		case 'CHS':
			image_to_show = CHS;
			break;
	}
	export async function execute(crsKind: string, crsType: string) {
		let CrossSectionResponse: CrossSectionResponse = await invoke('get_area', {
			crstype: crsKind,
			name: crsType
		});
		crossSectionDataToShow = [
			{
				id: 'b',
				val: CrossSectionResponse.width ? CrossSectionResponse.width.toFixed(0) : 'NaN',
				unit: 'mm'
			},
			{
				id: 'h',
				val: CrossSectionResponse.height ? CrossSectionResponse.height.toFixed(0) : 'NaN',
				unit: 'mm'
			},
			{
				id: 'd',
				val: CrossSectionResponse.diameter ? CrossSectionResponse.diameter.toFixed(1) : 'NaN',
				unit: 'mm'
			},
			{
				id: 't',
				val: CrossSectionResponse.thickness_wall
					? CrossSectionResponse.thickness_wall.toFixed(1)
					: 'NaN',
				unit: 'mm'
			},
			{
				id: 's',
				val: CrossSectionResponse.thickness_web
					? CrossSectionResponse.thickness_web.toFixed(1)
					: 'NaN',
				unit: 'mm'
			},
			{
				id: 't',
				val: CrossSectionResponse.thickness_flange
					? CrossSectionResponse.thickness_flange.toFixed(1)
					: 'NaN',
				unit: 'mm'
			},
			{
				id: 'r',
				val: CrossSectionResponse.radius1 ? CrossSectionResponse.radius1.toFixed() : 'NaN',
				unit: 'mm'
			},
			{
				id: 'A',
				val: CrossSectionResponse.area ? CrossSectionResponse.area.toExponential(2) : 'NaN',
				unit: 'mm^2'
			},
			{
				id: 'A_v',
				val: CrossSectionResponse.A_v ? CrossSectionResponse.A_v.toExponential(2) : 'NaN',
				unit: 'mm^2'
			},
			{
				id: 'A_{v,y}',
				val: CrossSectionResponse.A_v_y ? CrossSectionResponse.A_v_y.toExponential(2) : 'NaN',
				unit: 'mm^2'
			},
			{
				id: 'A_{v,z}',
				val: CrossSectionResponse.A_v_z ? CrossSectionResponse.A_v_z.toExponential(2) : 'NaN',
				unit: 'mm^2'
			}
		];
	}
</script>

<div id="cross-section-properties" class="overflow">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<img src={image_to_show} alt={crsKind} />
	<ResponseTable title={'Tverrsnitt'} dataToShow={crossSectionDataToShow} />
</div>

<style>
	:root {
		--select-border: rgb(100, 100, 100);
		--focus-border: rgb(150, 150, 150);

		--select-focus: blue;
		--select-arrow: var(--select-border);
	}
	div {
		width: 100%;
		display: flex;
		flex-grow: 1;
	}
	.overflow {
		display: inline-block;
		overflow: hidden;
		vertical-align: bottom;
	}
	img {
		margin-top: 0.5em;
		width: 100%;
	}
</style>
