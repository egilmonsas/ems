<script lang="ts">
	export let links: NavLink[];

	interface NavLink {
		display: string;
		route: string;
	}
</script>

<footer>
	EMs ståldesign
	{#each links as link}
		<a href={link.route}>{link.display}</a>
	{/each}
</footer>

<style>
	footer {
		top: 0;
		left: 0;
		width: 100%;
		padding-top: 0.5em;
		padding-bottom: 0.5em;
		padding-right: 1em;
		padding-left: 1em;
		margin: 0 15px 0 0;
		background: rgb(60, 60, 60);
		text-align: start;
	}
</style>
