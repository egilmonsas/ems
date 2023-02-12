import { sveltekit } from '@sveltejs/kit/vite';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';

const file = fileURLToPath(new URL('package.json', import.meta.url));
const json = readFileSync(file, 'utf8');
const pkg = JSON.parse(json);
/** @type {import('vite').UserConfig} */
const config = {
	define: {
		'__APP_VERSION__': JSON.stringify(pkg.version),
		'__AUTHOR__': JSON.stringify(pkg.author),

	},
	plugins: [sveltekit()]
};

export default config;
