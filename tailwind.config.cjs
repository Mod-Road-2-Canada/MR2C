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
					"accent": "#86786A",		
					"neutral": "#3B1F22",		
					"base-100": "#110E0E",			
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

