<script lang="ts">
	import Katex from 'svelte-katex';
	interface DataProp {
		id?: string;
		val?: string;
		unit?: string;
		tooltip?: String;
	}

	export let props: DataProp = {};

	$: show = props.val !== 'NaN' && props.val !== undefined;
</script>

{#if show}
	<div class="wrapper">
		<div id="row">
			<span class="tooltiptext">{props.tooltip ? props.tooltip : ''}</span>
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
	</div>
{/if}

<style>
	#row {
		display: flex;
		width: 100%;
		flex-direction: row;
		font-size: 0.8em;
		height: 1.5em;
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
		width: 4em;
		margin-left: 1em;
	}
	.wrapper {
		position: relative;
		display: inline-block;
		border-bottom: 1px dotted black; /* If you want dots under the hoverable text */
	}

	/* Tooltip text */
	.tooltiptext {
		visibility: hidden;
		left: 3em;
		top: 0;
		background-color: black;
		color: #fff;
		text-align: center;
		padding: 5px 0;
		border-radius: 6px;
		/* Position the tooltip text - see examples below! */
		position: absolute;
		z-index: 1;
	}
	#row:hover .tooltiptext {
		visibility: visible;
	}
</style>
