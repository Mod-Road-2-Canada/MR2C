# Documentation
```
└📁 DeathRoadToCanada
  └📁 data
  └📁 deathforth
  └📁 gfx
  └📁 mods
  🐴 DR2C MOD INSTALLER
```
## Syntax:

### Variables
| Name        | Type    | Use                                                                    |
|-------------|---------|------------------------------------------------------------------------|
| `File`      | string  | File path to the vanilla file you want to edit                         |
| `Search`    | string  | Line(s) to search for                                                  |
| `Add`       | string  | Line(s) to insert above/below the `Search`                             |
| `Above`     | boolean | true for above. Default is false (below).                                      |
| `GfxFolder` | string  | File path to **your mod** gfx folder                                   |
| `IndexFile` | string  | File path to generate the file containing index of your modded sprites |

### Commands
`Save_This.`: For editing text files.

`Merge_This.`: For combining images.

`Overlap_This.`: For overlaying images.

## Examples
<pre><code>└📁 DeathRoadToCanada
  └📁 data
  └📁 deathforth
  └📁 gfx
  └📁 mods
    <strong>└📂 Example
      📑 example.dfmod
    📄 example.json</strong>
  🐴 DR2C MOD INSTALLER
</code></pre>

Editing files:
```ts
Search	= "uses don/events/don-common.df";
Add		= "uses ../mods/mod-doc-head/docevents.df";
Above	= false;

Save_This.
```

```ts
File	= "./deathforth/events/common/cyoa-k-newbie.df";
Search	= "cyoa: starthintme";
Add		= `
1 WEAPON_CURSEDHEAD trunk.weapon+
1 WEAPON_CURSEDHEAD trunk.weapon+
`;
Save_This.
```

Merging spritesheets:
```ts
GfxFolder = "./mods/gfx";
IndexFile = "./mods/At Me/alex.df";

Merge_This.
```

Overlapping file for individual spritesheet:
```ts
File	= "./gfx/tiles/street.png";
Add		= "./mods/Clean Road/gfx/tiles/street.png";

Overlap_This.
```