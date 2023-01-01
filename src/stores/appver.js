import { readable } from 'svelte/store';

export const VERSION = readable(new String(), function start(set) {
	const ver = set('0.1.0');

	return function stop() {};
});
