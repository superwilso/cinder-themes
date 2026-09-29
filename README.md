# Cinder themes

Palettes for [Cinder](https://github.com/superwilso/Cinder), the replacement firmware UI for Sony's
NW-A50 series Walkmans. Anyone can share one here, and anyone can download them.

A palette is a small text file that recolours Cinder. It sets six neutral colours for day and six
for night, and optionally an accent of its own. Layout, type and icons stay the same. Every
palette here passed the same readability rules the player applies when it loads one, so anything
you download will load.

**[See them all in the gallery →](GALLERY.md)**

## Get palettes

**With [Flint](https://github.com/superwilso/flint):**

1. Open Flint and go to the **Palettes** page.
2. Click **Choose…** and pick a folder for your palettes.
3. Click **Download shared**. Flint downloads every palette here that is not already in your folder.
   It never replaces a file you already have.
4. Click **Check**, then **Send to the player**.

**By hand:**

1. Open [`palettes/`](palettes) and download the `.palette` file you want. Click the file, then
   *Download raw file*.
2. Connect the player over USB.
3. In the root of its internal memory, make a folder called `cinder_palettes` if there isn't one.
4. Copy the file into it and unplug.
5. On the player, open **Settings ▸ Display ▸ Palette** and pick it.

## Make one

**With Flint:** click **Palettes ▸ New palette**. Pick a starting point (Cinder, Slate, Paper or
Sony), name it and change the colours. The preview and the player's verdict update as you type.
Then click **Save to folder**.

**By hand:**

1. Copy [`template/cinder.palette`](template/cinder.palette), which is Cinder's own colours written
   out.
2. Rename the copy. The file name is the palette's id, and `cinder` is taken.
3. Edit the colours.

The rules, and what each key colours, are in Cinder's
[`docs/PALETTES.md`](https://github.com/superwilso/Cinder/blob/main/docs/PALETTES.md). In short:

- Text has to stay readable on the background, by day and by night.
- Night colours must be dark enough for a bedside table.
- A light palette needs an accent of its own, because Cinder's six accents are made for dark
  backgrounds.

## Share one

- **From Flint:** click **Share…** in the palette editor. It opens this repository's form with
  your palette already filled in. Submit it.
- **Without Flint:** [open a "Share a palette" issue](../../issues/new?template=palette.yml) and
  paste the file.
- **With git:** open a pull request that adds `palettes/<id>.palette`. CI checks it with the
  player's rules and prints the player's reason if it would be refused.

The gallery, the previews and `palettes/index.txt` (the list Flint downloads from) are rebuilt by
CI. Don't edit them by hand.

### For maintainers

To accept a shared palette, label its issue **`accepted`**.

- If the palette passes, a bot opens a pull request that adds it. The PR closes the issue when it
  is merged.
- If it would be refused, the bot comments with the reason and removes the label.

To check the palettes locally:

```sh
cargo run --manifest-path checker/Cargo.toml             # check, change nothing
cargo run --manifest-path checker/Cargo.toml -- --write  # also rebuild the index, previews and gallery
```

## Now Playing layouts

Custom Now Playing screens are planned, but not built yet. They would be shared the same way,
under [`layouts/`](layouts). See Cinder's
[`docs/PLAN_now_playing_layouts.md`](https://github.com/superwilso/Cinder/blob/main/docs/PLAN_now_playing_layouts.md).

## Licence

MIT. By sharing a palette here, you agree it can be shared under the same licence.
