import { writable, readable } from 'svelte/store';

export const GFX_FOLDER = writable("");
export const MODS = writable([]);
export const COOKIES_LOADED = writable(false);

export const BACKUP_GFX_FOLDER = "mr2c/gfx";
export const DATA_FILE = "mr2c/mr2c.data";