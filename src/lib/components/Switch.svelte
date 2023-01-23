<script lang="ts">
	export let active: boolean;
	export let onChange: any;
</script>

<label class="switch">
	<input type="checkbox" bind:checked={active} on:change={onChange}>
	<span class="slider round" />
</label>
