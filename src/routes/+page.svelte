<script lang="ts">
	import table6_2 from '$lib/assets/infographics/table6_2.png';
	import CapacitySelect from '$lib/components/CapacitySelect.svelte';
	import CrsSelect from '$lib/components/CrsSelect.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Header from '$lib/components/Header.svelte';
	import Selector from '$lib/components/Selector.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import BuckleCurve from '$lib/figures/BuckleCurve.svelte';
	import type { BuckleResponse } from '$lib/types/buckleResponse';
	import { invoke } from '@tauri-apps/api/tauri';
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


	let buckleCurveKinds = ['A0', 'A', 'B','C','D'];

	function handle() {
		get_crs_data(crsKind, crsType);
		get_cmb_capacity(crsKind, crsType, material, icludeSafetyFactor);
		data = get_buckledata();
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
    let viewPortWidth, viewPortHeight
	let consideringBuckleCurve:boolean;
</script>

<Header links={[{ display: 'Test', route: '/test' }]} />
<main>
	<sidebar>
		Material: <Selector bind:selected={material} options={names.MAT} onChange={handle}/>
		Materialfaktor? <Switch bind:active={icludeSafetyFactor} onChange={handle}/>
		Tverrsnittstype: <Selector bind:selected={crsKind} options={crsKinds} onChange={handle}/>
		Tverrsnittsvariant: <Selector bind:selected={crsType} options={crsOptions} onChange={handle} />
		Knekkurve,y: <Selector bind:selected={curveY} options={buckleCurveKinds} onChange={handle} bind:hover={consideringBuckleCurve}/>
		Knekkurve,z: <Selector bind:selected={curveZ} options={buckleCurveKinds} onChange={handle} bind:hover={consideringBuckleCurve}/>
		<CrsSelect bind:execute={get_crs_data} {crsKind}/>
		<CapacitySelect  bind:execute={get_cmb_capacity} {icludeSafetyFactor} />
	</sidebar>
	{#await data}
	<p>Loading...</p>
	{:then data}
	<div class = "container" bind:clientWidth={viewPortWidth} bind:clientHeight={viewPortHeight}>
		<!-- <BuckleCurve {data} parentWidth={viewPortWidth} parentHeight={viewPortHeight}/> -->
		<img src={table6_2} alt="Eurokode 3-1, tabell 6.3" hidden={!consideringBuckleCurve}/> 
		<BuckleCurve {data} {crsType} parentWidth={viewPortWidth} parentHeight={viewPortHeight}/>
		
	</div>
	{/await}
</main>
<Footer />

<style>
	.container{
		margin:0;
		padding:0;
		width:100%;
		flex-grow: 1;
		display:flex;
		flex-direction: column;
		background: rgba(255, 255, 255, 0);
	}

	img{
		position: absolute;
		height: 100%;
		z-index: 2
	}
</style>