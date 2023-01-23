<script lang="ts">
	import Katex from 'svelte-katex';
	interface DataProp {
		id?: string;
		val?: string;
		unit?: string;
	}

	export let props: DataProp = {};

	$: show = props.val !== 'NaN' && props.val !== undefined;
</script>
{#if show}
	
<div id="row">	
	<div id="name">
		<Katex>{props.id}</Katex>
	</div>
	<div id="data">
		<Katex>{props.val}</Katex>
	</div>
	<div id="unit">
		<Katex>{props.unit}</Katex>
	</div>
</div>
{/if}
<style>
	#row {
		display: flex;
		flex-grow: 1;
		width: 100%;
		flex-direction: row;
		font-size: 0.8em;
	}
	#name {
		text-align: left;
		width: 3.5em;
	}
	#data {
		text-align: right;
		width: 7em;
	}
	#unit {
		text-align: left;
		width: 3.5em;
		margin-left: 1em;
	}
</style>
