<script lang="ts">
	import table6_2 from '$lib/assets/infographics/table6_2.png';
	import table6_4 from '$lib/assets/infographics/table6_4.png';
	import tableB_3 from '$lib/assets/infographics/tableB_3.png';

	import CapacitySelect from '$lib/components/CapacitySelect.svelte';
	import Beam1D from '$lib/figures/Beam1D.svelte';

	import CrsSelect from '$lib/components/CrsSelect.svelte';
	import DesignCheckTable from '$lib/components/DesignCheckTable.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Selector from '$lib/components/Selector.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import Header from '$lib/components/Tabs.svelte';
	import BuckleCurve from '$lib/figures/BuckleCurve.svelte';
	import type { BuckleResponse } from '$lib/types/buckleResponse';
	import { BaseDirectory, createDir } from '@tauri-apps/api/fs';
	import { invoke } from '@tauri-apps/api/tauri';
	let M0 = 0;
	let M1 = 0;
	let F0 = 0;
	let icludeSafetyFactor: boolean = true;
	let material = 'S355';
	let crsKind = 'HEB';
	let crsType = 'HEB 100';
	let crsKinds = ['HEB', 'CHS'];
	let res: any = '';

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
	let curveY = 'A';
	let curveZ = 'A';
	let curveLTB = 'A';

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
	let perform_design_check: any;
	let buckleCurveKinds = ['A0', 'A', 'B', 'C', 'D'];
	let LTBCurveKinds = ['A', 'B', 'C', 'D'];

	let cMy: number = 0.95,
		cMz: number = 0.95,
		muCr: number = 1.145;
	function handle() {
		get_crs_data(crsKind, crsType);
		perform_design_check(
			crsKind,
			crsType,
			material,
			N_kN ? N_kN : 0,
			My_kNm ? My_kNm : 0,
			Mz_kNm ? Mz_kNm : 0,
			length,
			beta_ky,
			beta_kz,
			beta_kltb,
			curveY,
			curveZ,
			curveLTB,
			icludeSafetyFactor ? 'D' : 'K',
			cMy,
			cMz,
			muCr
		);
		res = get_cmb_capacity(crsKind, crsType, material, icludeSafetyFactor);
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
			limitstate: icludeSafetyFactor ? 'D' : 'K'
		})) as BuckleResponse[];
		return data;
	}
	let data = get_buckledata();
	let viewPortWidth, viewPortHeight;
	let consideringBuckleCurve: boolean;
	let consideringLTBCurve: boolean;
	let consideringCm: Boolean;
	let tabItems = [
		{ label: 'Stavmodell', value: 1 },
		{ label: 'Knekkurve', value: 2 },
		{ label: 'Kapasitetssjekk', value: 3 }
	];
	let currentTab: any;

	let x1: number = 0,
		y1: number = 0,
		x2: number = 10,
		y2: number = 0,
		N_kN: Number,
		My_kNm: Number,
		Mz_kNm: Number;
	$: beam = { start_node: { x: x1, y: y1 }, end_node: { x: x2, y: y2 } };
	let beta_ky = 1.0;
	let beta_kz = 1.0;
	let beta_kltb = 1.0;

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
	async function increment() {
		await invoke('increment_count', {});
		count = get_count();
	}
	async function get_count() {
		let res = await invoke('get_count', {}).then();
		return res;
	}
	$: count = get_count();
	$: length = beam_len(beam);

	const createDataFolder = async () => {
		try {
			await createDir('NGI_STÅL', {
				dir: BaseDirectory.Document,
				recursive: true
			});
		} catch (e) {
			console.error(e);
		}
	};
	createDataFolder();
</script>

<main>
	<sidebar>
		<sidebarContent>
			<p>Dim. Last</p>
			<div class="row">
				<input type="number" on:change={handle} bind:value={N_kN} step="1" placeholder="N,ed" />
				<input
					type="number"
					on:change={handle}
					bind:value={My_kNm}
					step="0.1"
					placeholder="My,ed"
				/>
				<input
					type="number"
					on:change={handle}
					bind:value={Mz_kNm}
					step="0.1"
					placeholder="Mz,ed"
				/>
			</div>
			<p>Geometri</p>
			<div class="row">
				<input type="number" on:change={handle} bind:value={x1} step="0.1" placeholder="x1" />
				<input type="number" on:change={handle} bind:value={x2} step="0.1" placeholder="x2" />
				<input type="number" on:change={handle} bind:value={y1} step="0.1" placeholder="y1	" />
				<input type="number" on:change={handle} bind:value={y2} step="0.1" placeholder="y2" />
				{length.toFixed(1)}
			</div>
			<p>Div.</p>
			<div class="row">
				<input
					type="number"
					on:change={handle}
					bind:value={cMy}
					step="0.01"
					placeholder="cMy"
					on:mouseenter={() => {
						consideringCm = true;
					}}
					on:mouseleave={() => {
						consideringCm = false;
					}}
				/>
				<input
					type="number"
					on:change={handle}
					bind:value={cMz}
					step="0.01"
					placeholder="cMz"
					on:mouseenter={() => {
						consideringCm = true;
					}}
					on:mouseleave={() => {
						consideringCm = false;
					}}
				/>
				<input type="number" on:change={handle} bind:value={muCr} step="0.01" placeholder="muCr" />
			</div>
			<p>Material</p>
			<div class="row">
				<Selector
					label={'Klasse: '}
					bind:selected={material}
					options={names.MAT}
					onChange={handle}
				/>
				<Switch label={'Faktor? '} bind:active={icludeSafetyFactor} onChange={handle} />
			</div>
			<p>Tverrsnitt</p>
			<div class="row">
				<Selector label={'Type'} bind:selected={crsKind} options={crsKinds} onChange={handle} />
				<Selector
					label={'Variant '}
					bind:selected={crsType}
					options={crsOptions}
					onChange={handle}
				/>
			</div>
			<p>Knekkurve</p>
			<div class="row">
				<Selector
					label={'Y-akse'}
					bind:selected={curveY}
					options={buckleCurveKinds}
					onChange={handle}
					bind:hover={consideringBuckleCurve}
				/>
				<Selector
					label={'Z-akse'}
					bind:selected={curveZ}
					options={buckleCurveKinds}
					onChange={handle}
					bind:hover={consideringBuckleCurve}
				/>
				<Selector
					label={'LTB'}
					bind:selected={curveLTB}
					options={LTBCurveKinds}
					onChange={handle}
					bind:hover={consideringLTBCurve}
				/>
			</div>
			<div class="row">
				<input
					type="number"
					on:change={handle}
					bind:value={beta_ky}
					step="0.1"
					max="2"
					min="0"
					placeholder="beta,y"
				/>
				<input
					type="number"
					on:change={handle}
					bind:value={beta_kz}
					step="0.1"
					max="2"
					min="0"
					placeholder="beta,z"
				/>
				<input
					type="number"
					on:change={handle}
					bind:value={beta_kltb}
					step="0.1"
					max="2"
					min="0"
					placeholder="beta,ltb"
				/>
			</div>
			<CrsSelect bind:execute={get_crs_data} {crsKind} />
			<CapacitySelect bind:execute={get_cmb_capacity} {icludeSafetyFactor} />
			<DesignCheckTable bind:execute={perform_design_check} />
		</sidebarContent>
	</sidebar>

	<div class="container" bind:clientWidth={viewPortWidth} bind:clientHeight={viewPortHeight}>
		<Header items={tabItems} bind:activeTabValue={currentTab} />
		<img src={table6_2} alt="Eurokode 3-1, tabell 6.3" hidden={!consideringBuckleCurve} />
		<img src={table6_4} alt="Eurokode 3-1, tabell 6.4" hidden={!consideringLTBCurve} />
		<img src={tableB_3} alt="Eurokode 3-1, tabell B.3" hidden={!consideringCm} />

		{#await data then data}
			{#await res then res}
				{#if 1 === currentTab}
					<Beam1D
						{F0}
						{M0}
						{M1}
						parentWidth={viewPortWidth}
						parentHeight={viewPortHeight}
						{beam}
						{res}
					/>
				{/if}
			{/await}
			{#if 2 === currentTab}
				<BuckleCurve {data} {crsType} parentWidth={viewPortWidth} parentHeight={viewPortHeight} />
			{/if}
			{#if 3 === currentTab}
				<button type="button" on:click={increment}>Click Me!</button>
				{#await count then count}
					{count}
				{/await}
			{/if}
		{/await}
	</div>
</main>
<Footer />

<style>
	.container {
		margin: 0;
		padding: 0;
		width: 100%;
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		background: rgba(255, 255, 255, 0);
	}

	img {
		position: absolute;
		width: 800px;
		z-index: 2;
	}
	.row {
		display: flex;
		flex-direction: row;
		width: 100%;
	}

	p {
		margin: 0;
		font-size: 1.1em;
		width: 100%;
		color: rgb(200, 200, 255);
	}
	input {
		width: 100%;
	}
	input[type='number']::-webkit-inner-spin-button,
	input[type='number']::-webkit-outer-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
</style>
