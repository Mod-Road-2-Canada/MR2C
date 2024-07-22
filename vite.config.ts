import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	// Something something
	// optimizeDeps: {
	// 	include: ["highlight.js", "highlight.js/lib/core"],
	// },
});
