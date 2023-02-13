<script lang="ts">
	export let options: Array<string> = [''];
	export let selected: string;
	export let onChange: any;
	export let hover: boolean = false;
	export let label: string;
</script>

<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div>
	{label}
	<select
		bind:value={selected}
		on:change={onChange}
		on:mouseenter={() => {
			hover = true;
		}}
		on:mouseleave={() => {
			hover = false;
		}}
	>
		{#each options as item}
			<option>{item}</option>
		{/each}
	</select>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		width: 100%;
		margin-bottom: 0.5em;
	}
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
