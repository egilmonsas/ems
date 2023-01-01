<script>
	import { VERSION } from '../../stores/appver.js';
	import { AUTHOR } from '../../stores/author.js';

	// import { time, elapsed } from '../../stores/time.js';

	// const formatter = new Intl.DateTimeFormat('en-NO', {
	// 	hour12: false,
	// 	hour: 'numeric',
	// 	minute: '2-digit',
	// 	second: '2-digit'
	// });
</script>

<footer>
	<div>v{$VERSION}</div>
	<div class="author">{$AUTHOR}</div>

	<!-- <div>{formatter.format($time)}</div> -->
</footer>

<style>
	footer {
		position: absolute;
		bottom: 0;
		left: 0;
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
