import { writable, readable } from 'svelte/store';
import type { ModInfo } from '$lib/types';

export const MODS = writable<ModInfo>([]);
export const COOKIES_LOADED = writable(false);

export const THEME = writable("apocalypse");	// Default is dark theme