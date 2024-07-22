/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},
	},
	daisyui: {
		themes: [
			{
				apocalypse: {
					"primary": "#D85251",	
					"secondary": "#8C0327",
					"accent": "#B387FA",		
					"neutral": "#2A323C",		
					"base-100": "#292524",			
					"info": "#42ADBB",			
					"success": "#499380",			
					"warning": "#E97F14",				
					"error": "#DF1A2F",
				},
			},
		"cupcake"],
	},
	plugins: [
		require("@tailwindcss/typography"),
		require('daisyui'),
	],
}

