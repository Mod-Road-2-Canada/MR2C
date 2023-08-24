export enum ModStatus {
  Normal = 0,
  New = 1,
  Updated = 2,
}

type ModInfo = {
	name: string;
	tag: string;
	path: string;
	version: string;
	creator: string;

	checked: boolean;
	status: ModStatus;
};

type ModCookies = {
	gfx_dir: string;
	modlist: ModInfo[];
}