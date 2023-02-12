<script lang="ts">
	import DataProperty from '$lib/units/DataProperty.svelte';
	export let dataToShow: Array<DataProp>;
	export let title: string;

	let want_to_show_capacity: boolean = true;

	interface DataProp {
		id?: string;
		val?: string;
		unit?: string;
	}

	async function toggleShow() {
		want_to_show_capacity = !want_to_show_capacity;
	}
</script>

<div id="cross-section-properties">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<h1 on:click={toggleShow}>{title}</h1>
</div>
{#if want_to_show_capacity}
	<div>
		{#each dataToShow as props}
			<DataProperty {props} />
		{/each}
	</div>
{/if}

<style>
	#cross-section-properties {
		display: inline-block;
		height: fit-content;
	}
	div {
		width: 100%;
		height: fit-content;
		display: flex;
		flex-direction: column;
	}
</style>
