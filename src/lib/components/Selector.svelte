<script lang="ts">
	export let options: Array<string> = [''];
	export let selected: string;
	export let onChange: any;
</script>

<select bind:value={selected} on:change={onChange}>
	{#each options as item}
		<option>{item}</option>
	{/each}
</select>

<style>
	select {
		width: 100%;

		font-family: inherit;
		font-size: 0.9em;
		cursor: inherit;
		line-height: inherit;
		outline: none;
		cursor: pointer;
		border: 1px solid var(--select-border);

		color: inherit;
		background-color: rgb(40, 40, 40);
	}
	select:focus {
		border: 1px solid var(--focus-border);
	}
	option {
		color: inherit;
		background-color: rgb(40, 40, 40);
	}
</style>
