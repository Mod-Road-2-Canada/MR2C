export enum ModStatus {
	Normal = 0,
	New = 1,
	Updated = 2,
}

export type ModInfo = {
	name: string
	tag: string
	path: string
	version: string
	creator: string
	description: string

	checked: boolean
	status: ModStatus
}

export type ModCookies = {
	gfx_dir: string
	modlist: ModInfo[]
}
