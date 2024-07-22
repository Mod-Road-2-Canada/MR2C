# Documentation for Modders
## 1. Requirements:
File structure:
<pre><code>└📁 DeathRoadToCanada
	└📁 data
	└📁 deathforth
	└📁 gfx
	└📁 mods
		└📂 <b><em>Example_mod_folder</em></b>  ✔️
			📑 <em>example.dfmod</em>    ✔️
		📄 <em>example.json</em>         ✔️
	🐴 Mod Road 2 Canada
</code></pre>


Each mod should have at least these 2 files:
- `📄 example.json` : Contains info for the mods.
- `📄 example.dfmod` : Contains instructions for **MR2C** to install your mod.

## 2. `📄.json` file specifications
The json file for your mod should be directly inside the `📁 mods` folder. This file will include the metadata **MR2C** needed for your mod:
- "name": Name of your mod
- "tag": The tag that **MR2C** will use to identify your mod. Must be a unique value.   		
Default value is the name of this `📄.json` file to avoid tag name conflict with other mods.
- "path": Path to your `📄.dfmod` file (relative to the location of **MR2C**).
- "version": Your mod version
- "creator": The mod's creator(s)
Optionals:
- "description": Short description for your mod. Recommended to be <160 characters.

For example:
```json
{
	"name": "Head Swap Doctor Reveal",
	"tag": "headdoc",
	"path": "./mods/mod-doc-head/doc.dfmod",
	"version": "0.3-NERVE",
	"creator": "Hwang"
}
```

## 3. `📄.dfmod` file specifications

This file is where you write instructions for **MR2C** to insert new code into the game's vanilla code. Whitespaces are ignored, so feel free to add extra line breaks to improve readability.

### 3.1. Text file manipulation
| Name        | Type    | Use                                                                    |
|-------------|---------|:-----------------------------------------------------------------------|
| `File`      | string  | File path to the vanilla file you want to edit <br/> (Relative to `📁 DeathRoadToCanada` folder) |
| `Search`    | string  | Line(s) to search for                                                  |
| `Add`       | string  | Line(s) to insert above/below the `Search`                             |
| `Above`     | boolean | Set _true_ to insert above `Search` string.<br/>Default is _false_ (insert below `Search` string). |
| `Bottom`    | boolean | Set _true_ to insert at bottom of `File`. Ignore `Above` and `Search`.<br/>Default is _false_. |
| **`Save_This.`** | **command** | Start insertion/replacement with the provided arguments.      |

* **Every variable is required**, except for `Above` (Default is false).  
**MR2C** automatically `Add` your content to a new line above or below the line containing `Search` term so you won't need to worry about adding extra lines.

* Note: Always use a complete line for your `Search` term. If you don't, the loader will insert new code mid-line, which could cause issues.  
Here's an example of what could go wrong:

`epilogue-check.df` before MR2C:
```
...
	<- char pig? if "was widely recognized in Canada as being SOME PIG!*" then;
	<- char ispet? if "was finally a free animal again, and vanished into the wilderness.*" then;

	<- char .trait@ "CARL!" $= if "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARRRRRRRRRRRRRL!" then;
...
```
`📄.dfmod`:
```ts
File	= "./deathforth/events/finale/epilogue-check.df";
Search	= `<- char ispet? if "was finally`;
Add		= `<- char .trait@ "Cat lover" $= if "ENDING 1" then;`;
Save_This.
```
The line will be inserted in the middle of the `Search` term right after "was finally", breaking the existing string.
```
...
	<- char pig? if "was widely recognized in Canada as being SOME PIG!*" then;
	<- char ispet? if "was finally
(* modtag *) <- char .trait@ "Cat lover" $= if "ENDING 1" then;
(* modtag nl *)  a free animal again, and vanished into the wilderness.*" then;

	<- char .trait@ "CARL!" $= if "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARRRRRRRRRRRRRL!" then;
...
```

### 3.2. Spritesheet extension
| Name        | Type    | Use                                                                    |
|-------------|---------|:-----------------------------------------------------------------------|
| `GfxFolder` | string  | File path to **your mod's gfx folder** |
| `IndexFile` | string  | File path to generate the file containing index of your modded sprites |
| **`Merge_This.`** | **command** | Start combining images. |

When executing this command, **MR2C** automatically merges all of your mod custom spritesheets in `GfxFolder` with the vanilla ones that shares the same filename. Then, it generates a `.df` file that contains the starting indexes of your spritesheets. You can then import this file into your code to reference your new sprites.

For example, assuming your custom weapon sprites is located at `./mods/Mod Name/dr2c_weapons.png`
In `📄.dfmod` file:
```ts
GfxFolder  = "./mods/Mod Name/gfx";
IndexFile  = "./mods/Mod Name/myindex.df";

Merge_This.
```

### 3.3. Examples

* Injecting code into vanilla file:
```ts
File    = "./deathforth/decks.df";
Search  = "uses don/events/don-common.df";
Add     = "uses ../mods/mod-doc-head/docevents.df";
Above   = false;

Save_This.
```

* Add code into bottom of file:
```ts
File    = "./deathforth/boot.df";
Add     = "uses ../mods/mod-new/boot.df";
Bottom  = true;

Save_This.
```

* Adding multiple lines:
```ts
File    = "./deathforth/events/common/cyoa-k-newbie.df";
Search  = "cyoa: starthintme";
Add     = `
1 WEAPON_CURSEDHEAD trunk.weapon+
1 WEAPON_CURSEDHEAD trunk.weapon+
`;

Save_This.
```

* Merging spritesheets:
```ts
GfxFolder  = "./mods/Mod Name/gfx";
IndexFile  = "./mods/Mod Name/alex.df";

Merge_This.
```

* Overlapping file for individual spritesheet:
```ts
File  = "./gfx/tiles/street.png";
Add   = "./mods/Clean Road/gfx/tiles/street.png";

Overlap_This.
```