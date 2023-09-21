<script>
	// @ts-nocheck
</script>

<footer>
	<!-- svelte-ignore missing-declaration -->
	<div>v{__APP_VERSION__}</div>
	<!-- svelte-ignore missing-declaration -->
	<div class="author">{__AUTHOR__}</div>

	<!-- <div>{formatter.format($time)}</div> -->
</footer>

<style>
	footer {
		bottom: 0;
		left: 0;
		margin-top: auto;
		width: 100%;
		padding-top: 0.15em;
		padding-bottom: 0.15em;
		background: rgb(0, 122, 204);
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		font-size: 0.9em;
	}

	footer div {
		padding-right: 1em;
		padding-left: 1em;
		text-orientation: sideways;
	}

	footer .author {
		font-style: italic;
	}
</style>
