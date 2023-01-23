<script lang="ts">
	import DataProperty from "$lib/units/DataProperty.svelte";
	export let dataToShow:Array<DataProp>;
	export let title:string;

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
<div id="cross-section-properties" class="overflow">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<h1 on:click={toggleShow}>{title}</h1>
	{#if want_to_show_capacity}
	<div>
		{#each dataToShow as props}
			<DataProperty {props}/>
		{/each}
	</div>
	{/if}
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
		width: 100%;
    }

	.overflow {
		display: inline-block;
		overflow: hidden;
		vertical-align: bottom;
	}
</style>
