<script lang="ts">
	export let active: boolean;
	export let onChange: any;
	export let label:string;

</script>

<label class="switch">
	{label}
	<input type="checkbox" bind:checked={active} on:change={onChange}>
	<span class="slider round" />
</label>
