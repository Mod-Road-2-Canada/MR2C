# Documentation for Modders
## Requirements:
File structure:
<pre><code>└📁 DeathRoadToCanada
	└📁 data
	└📁 deathforth
	└📁 gfx
	└📁 mods
		└📂 <b><em>Example_mod_folder</em></b>  ✔️
			📑 <em>example.dfmod</em>    ✔️
		📄 <em>example.json</em>         ✔️
	🐴 DR2C MOD INSTALLER
</code></pre>


Each mod should contains 2 files:
- `📄 example.json` : Contains info for the mods.
- `📄 example.dfmod` : Contains instructions for **MR2C** to install your mod.

## `📄.json` file specifications
The json file for your mod should be directly inside the `📁 mods` folder.<br/>
This file will contains:
- "name": Name of your mod
-	"tag": The tag that **MR2C** will use to identify your mod. This need to be a unique value.<br/> 		Default is the name of this `📄.json` file to avoid conflict.
-	"path": Path to your `📄.dfmod` file (relative to the location of **MR2C**).
-	"version": Your mod version
-	"creator": Mod's creator(s)

Example:
```json
{
	"name": "Head Swap Doctor Reveal",
	"tag": "headdoc",
	"path": "./mods/mod-doc-head/doc.dfmod",
	"version": "0.3-NERVE",
	"creator": "Hwang"
}
```

## `📄.dfmod` file specifications

### 1. Variables
| Name        | Type    | Use                                                                    |
|-------------|---------|:-----------------------------------------------------------------------|
| `File`      | string  | File path to the vanilla file you want to edit <br/> (Relative to `📁 DeathRoadToCanada` folder) |
| `Search`    | string  | Line(s) to search for                                                  |
| `Add`       | string  | Line(s) to insert above/below the `Search`                             |
| `Above`     | boolean | Set _true_ to insert above `Search` string.<br/>Default is _false_ (insert below `Search` string).  |
| `Bottom`    | boolean | Set _true_ to insert at bottom of `File`. Ignore `Above` and `Search`.<br/>Default is _false_. |
| `GfxFolder` | string  | File path to **your mod** gfx folder                                   |
| `IndexFile` | string  | File path to generate the file containing index of your modded sprites |

* **Every variable is required**, except for `Above` (Default is false).
The modloader will `Add` your content to a new line above or below the line containing `Search` string so you won't need to worry about adding extra lines.

* Empty lines are skipped.

### 2. Commands
`Save_This.`: For editing text files.<br/>
`Merge_This.`: For combining images.<br/>
`Overlap_This.`: For overlaying images.<br/>



### 3. Examples

* Injecting code into vanilla file:
```ts
File	= "./deathforth/decks.df";
Search	= "uses don/events/don-common.df";
Add	  = "uses ../mods/mod-doc-head/docevents.df";
Above	= false;

Save_This.
```

* Add code into bottom of file:
```ts
File	= "./deathforth/boot.df";
Add	  = "uses ../mods/mod-new/boot.df";
Bottom	= true;

Save_This.
```

* Adding multiple lines:
```ts
File		= "./deathforth/events/common/cyoa-k-newbie.df";
Search	= "cyoa: starthintme";
Add		= `
1 WEAPON_CURSEDHEAD trunk.weapon+
1 WEAPON_CURSEDHEAD trunk.weapon+
`;

Save_This.
```

* Merging spritesheets:
```ts
GfxFolder	= "./mods/Mod Name/gfx";
IndexFile	= "./mods/Mod Name/alex.df";

Merge_This.
```

* Overlapping file for individual spritesheet:
```ts
File	= "./gfx/tiles/street.png";
Add		= "./mods/Clean Road/gfx/tiles/street.png";

Overlap_This.
```