import type { ModInfo } from '$lib/types'

export const MR2C_GLOBAL: {
	MODS: ModInfo[]
	COOKIES_LOADED: boolean
	THEME: string
} = $state({
	MODS: [],
	COOKIES_LOADED: false,
	THEME: 'apocalypse', // Default is dark theme
})
