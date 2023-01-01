import { readable } from 'svelte/store';

export const AUTHOR = readable(new String(), function start(set) {
	const ver = set('egil.monsas@ngi.no');

	return function stop() {};
});
