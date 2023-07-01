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
| `File`      | string  | File path to the vanilla file you want to edit <br/> (Relative to `📁 DeathRoadToCanada`folder) |
| `Search`    | string  | Line(s) to search for                                                  |
| `Add`       | string  | Line(s) to insert above/below the `Search`                             |
| `Bottom`    | boolean | Set _true_ to insert at bottom of `File`. Will skip `Above` and `Search` if _true_. Default is _false_. |
| `Above`     | boolean | Set _true_ to insert above `Search` string. Default is _false_ (below).  |
| `GfxFolder` | string  | File path to **your mod** gfx folder                                   |
| `IndexFile` | string  | File path to generate the file containing index of your modded sprites |

* Every variable is **mandatory**, except for `Above` (Default is false).
The modloader will `Add` your content to a new line above or below the line containing `Search` string so you won't need to worry about adding extra lines.

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
File	= "./deathforth/decks.df";
Search	= "uses don/events/don-common.df";
Add	  = "uses ../mods/mod-doc-head/docevents.df";
Above	= false;

Save_This.
```

Add code into bottom of file:
```ts
File	= "./deathforth/boot.df";
Add	  = "uses ../mods/mod-new/boot.df";
Bottom	= true;

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