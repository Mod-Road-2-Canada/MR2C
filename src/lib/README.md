# Installing mods:
```
└📁 DeathRoadToCanada
  └📁 data
  └📁 deathforth
  └📁 gfx
  └📁 mods
  🐴 DR2C MOD INSTALLER
```
To install mods (that are compatible with this installer):
- Step 0: **Make sure** you've already put this`.exe`in`📁 DeathRoadToCanada`folder.
- Step 1: Home > Choose the`📁 gfx`folder
- Step 2: Press the **\[BACK UP\]** button
- Step 3: Install Mods > Load Mods
- Step 4: Choose the mod you want to install and then press **\[LOAD SELECTED MODS\]**


If there's any bug, problems or suggestions for this app, feel free to reach out to me: `@Hwang#2760` on Discord

---
# Documentation for Modders
## Requirements:
<pre><code>└📁 DeathRoadToCanada
	└📁 data
	└📁 deathforth
	└📁 gfx
	└📁 mods
		└📂 <b><em>Example</em></b>
			📑 <em>example.dfmod</em>
		📄 <em>example.json</em>
	🐴 DR2C MOD INSTALLER
</code></pre>

By default, the installer will try to find json files from the`📁 mods`folder
Each mod should have 2 files:
1. 📄 *example.json* : Contains info for the mods:
- "name": Name of your mod
-	"tag": The tag that installer will refers to as your mod. Default is the *.json* file name
-	"path": Path to your *.dfmod* file
-	"version": Your mod version
-	"creator": Mod's creators
2. 📄 *example.dfmod* : Contains instructions for the installer to install your mod.

### Variables
| Name        | Type    | Use                                                                    |
|-------------|---------|------------------------------------------------------------------------|
| `File`      | string  | File path to the vanilla file you want to edit <br/> (Relative to `📁 DeathRoadToCanada`folder)                         |
| `Search`    | string  | Line(s) to search for                                                  |
| `Add`       | string  | Line(s) to insert above/below the `Search`                             |
| `Above`     | boolean | true for above insert. Default is false (below).                                      |
| `GfxFolder` | string  | File path to **your mod** gfx folder                                   |
| `IndexFile` | string  | File path to generate the file containing index of your modded sprites |
### Commands
`Save_This.`: For editing text files.

`Merge_This.`: For combining images.

`Overlap_This.`: For overlaying images.

## Examples
### 1. *.json* file:
```json
{
	"name": "Head Swap Doctor Reveal",
	"tag": "headdoc",
	"path": "./mods/mod-doc-head/doc.dfmod",
	"version": "0.3-NERVE",
	"creator": "Hwang"
}
```
### 2. *.dfmod* file:
Injecting code into vanilla file:
```ts
Search	= "uses don/events/don-common.df";
Add	  = "uses ../mods/mod-doc-head/docevents.df";
Above	= false;

Save_This.
```

Adding multiple lines:
```ts
File		= "./deathforth/events/common/cyoa-k-newbie.df";
Search	= "cyoa: starthintme";
Add		= `
1 WEAPON_CURSEDHEAD trunk.weapon+
1 WEAPON_CURSEDHEAD trunk.weapon+
`;
Save_This.
```

Merging spritesheets:
```ts
GfxFolder	= "./mods/gfx";
IndexFile	= "./mods/At Me/alex.df";

Merge_This.
```

Overlapping file for individual spritesheet:
```ts
File	= "./gfx/tiles/street.png";
Add		= "./mods/Clean Road/gfx/tiles/street.png";

Overlap_This.
```