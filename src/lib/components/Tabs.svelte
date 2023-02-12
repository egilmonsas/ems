<script lang="ts">
	// @ts-nocheck
	import { onMount } from 'svelte';

	export let items = [];
	export let activeTabValue;
	onMount(() => {
		// Set default tab value
		if (Array.isArray(items) && items.length && items[0].value) {
			activeTabValue = items[0].value;
		}
	});

	const handleClick = (tabValue) => () => (activeTabValue = tabValue);
</script>

<header>
	<ul>
		{#if Array.isArray(items)}
			{#each items as item}
				<li class={activeTabValue === item.value ? 'active' : ''}>
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<span on:click={handleClick(item.value)}>{item.label}</span>
				</li>
			{/each}
		{/if}
	</ul>
</header>

<style>
	ul {
		display: flex;
		flex-wrap: wrap;
		padding-left: 0;
		margin-bottom: 0;
		list-style: none;
		border-bottom: 1px solid gray;
	}

	li {
		color: #495057;
		margin-left: 10px;
		cursor: pointer;
	}
	li.active > span {
		color: white;
		background-color: rgba(0, 0, 0, 0);
		border-bottom: rgba(0, 0, 0, 0);
		border-top: 1px solid gray;
		border-right: 1px solid gray;
		border-left: 1px solid gray;
		border-radius: 5px 5px 0px 0px;
	}
	li:hover {
		background: rgba(255, 255, 255, 0.1);
	}
</style>
