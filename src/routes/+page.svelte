<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import CrsSelect from '$lib/components/CrsSelect.svelte';
	import Capacity from '$lib/components/Capacity.svelte';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Selector from '$lib/components/Selector.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import BuckleCurve from '$lib/figures/BuckleCurve.svelte';
	import type { BuckleResponse } from '$lib/types/buckleResponse';

	let icludeSafetyFactor: boolean;
	let material = 'S355';
	let crsKind = 'HEB';
	let crsType = 'HEB 100';
	let crsKinds = ['HEB', 'CHS'];

	interface Names {
		CHS: Array<string>;
		HEB: Array<string>;
		MAT: Array<string>;
	}
	let names: Names = {
		CHS: [],
		HEB: [],
		MAT: []
	};
	get_names();
	let crsOptions: Array<string> = names.HEB;
	let curveY='A';
	let curveZ='A';
	$: switch (crsKind) {
		case 'HEB':
			crsOptions = names.HEB;
			if (!crsOptions.includes(crsType)) {
				crsType = crsOptions[0];
			}
			break;
		case 'CHS':
			crsOptions = names.CHS;
			if (!crsOptions.includes(crsType)) {
				crsType = crsOptions[0];
			}
			break;
		default:
			crsOptions = [''];
	}
	let get_crs_data: any;
	let get_cmb_capacity: any;
	let forget1: any;
	let forget2: any;


	let buckleCurveKinds = ['A0', 'A', 'B','C','D'];

	function handle() {
		get_crs_data(crsKind, crsType);
		get_cmb_capacity(crsKind, crsType, material, icludeSafetyFactor);
		data = get_buckledata();
	}
	function forget() {
		forget1();
		forget2();
	}

	async function get_names() {
		names.CHS = await invoke('get_section_names', { crstype: 'CHS' });
		names.HEB = await invoke('get_section_names', { crstype: 'HEB' });
		names.MAT = await invoke('get_steel_variants');
	}

	async function get_buckledata() {
		let data = (await invoke(`get_buckle_curve`, {
			crstype: crsKind,
			name: crsType,
			material: material,
			curveY,
			curveZ,
			limitstate: icludeSafetyFactor? "D" : "K"
		})) as BuckleResponse[];
		return data;
	}
	let data = get_buckledata();

</script>

<Header links={[{ display: 'Test', route: '/test' }]} />
<main>
	<sidebar>
		Material: <Selector bind:selected={material} options={names.MAT} onChange={handle} />
		Materialfaktor? <Switch bind:active={icludeSafetyFactor} onChange={handle} />
		Tverrsnittstype: <Selector bind:selected={crsKind} options={crsKinds} onChange={handle} />
		Tverrsnittsvariant: <Selector bind:selected={crsType} options={crsOptions} onChange={handle} />
		Knekkurve,y: <Selector bind:selected={curveY} options={buckleCurveKinds} onChange={handle} />
		Knekkurve,z: <Selector bind:selected={curveZ} options={buckleCurveKinds} onChange={handle} />
		<CrsSelect {crsKind} bind:execute={get_crs_data} bind:forget={forget1} />
		<Capacity {crsKind} bind:execute={get_cmb_capacity} bind:forget={forget2} />
	</sidebar>
	{#await data}
		<p>Loading...</p>
	{:then data}
		<BuckleCurve {data} xDimension={'L_k'} yDimension={'N_pl'} />
	{/await}
</main>
<Footer />
