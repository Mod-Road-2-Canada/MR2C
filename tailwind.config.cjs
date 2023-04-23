/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},
	},
	daisyui: {
		themes: [
			{
				mytheme: {
					"primary": "#8C0327",	
					"secondary": "#D85251",		
					"accent": "#9ca3af",		
					"neutral": "#110E0E",		
					"base-100": "#171212",			
					"info": "#42ADBB",			
					"success": "#499380",			
					"warning": "#E97F14",				
					"error": "#DF1A2F",
				},
			},
		"cupcake"],
	},
	plugins: [
		require('daisyui'),
	],
}

