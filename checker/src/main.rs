//! Checks every file in `palettes/` with Cinder's own rules, the way the player checks them at boot.
//!
//!     cargo run --manifest-path checker/Cargo.toml             check, change nothing
//!     cargo run --manifest-path checker/Cargo.toml -- --write  also rebuild the index and previews
//!
//! `--write` rebuilds `palettes/index.txt` (what Flint's "Download shared" reads), one preview per
//! palette in `previews/`, and `GALLERY.md`. CI runs it on every push to main, so nobody has to.
//! Exits 1 if any palette would be refused, with the player's own reasons.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use flint_core::palette::{check_file, palette_stem, Palette, Shown, MAX_BYTES};

const DIR: &str = "palettes";
const INDEX: &str = "index.txt";

fn main() -> ExitCode {
    let write = std::env::args().any(|a| a == "--write");
    let mut names: Vec<String> = match fs::read_dir(DIR) {
        Ok(rd) => rd.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect(),
        Err(e) => {
            eprintln!("{DIR}/: {e} — run this from the repository's root");
            return ExitCode::FAILURE;
        }
    };
    names.sort();

    let mut good: Vec<Palette> = Vec::new();
    let mut bad = 0;
    for name in &names {
        if name == INDEX {
            continue;
        }
        let mut refuse = |why: &str| {
            println!("REFUSED  {name}: {why}");
            bad += 1;
        };
        if palette_stem(name).is_none() {
            refuse("only .palette files belong in palettes/");
            continue;
        }
        if *name != name.to_ascii_lowercase() {
            refuse("file names are lowercase: the name is the palette's id");
            continue;
        }
        let path = Path::new(DIR).join(name);
        if fs::metadata(&path).map(|m| m.len() > MAX_BYTES).unwrap_or(true) {
            refuse(&format!("larger than {MAX_BYTES} bytes, which the player skips"));
            continue;
        }
        let body = match fs::read_to_string(&path) {
            Ok(b) => b,
            Err(e) => {
                refuse(&format!("not readable as UTF-8 text: {e}"));
                continue;
            }
        };
        match check_file(name, &body) {
            Some((_, Ok(p))) => {
                println!("ok       {name}  ({})", p.name);
                good.push(p);
            }
            Some((_, Err(errs))) => refuse(&errs.join("; ")),
            None => refuse("not a palette file name"),
        }
    }
    println!("\n{} palette(s) pass, {bad} refused", good.len());

    if write {
        if let Err(e) = write_all(&good) {
            eprintln!("writing: {e}");
            return ExitCode::FAILURE;
        }
        println!("wrote {DIR}/{INDEX}, previews/ and GALLERY.md");
    }
    if bad > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn write_all(good: &[Palette]) -> std::io::Result<()> {
    let index: String = good.iter().map(|p| format!("{}.palette\n", p.id)).collect();
    fs::write(Path::new(DIR).join(INDEX), index)?;

    fs::create_dir_all("previews")?;
    for e in fs::read_dir("previews")?.flatten() {
        if e.path().extension().is_some_and(|x| x == "svg") {
            fs::remove_file(e.path())?;
        }
    }
    let mut gallery = String::from(
        "# Gallery\n\n*Rebuilt by CI from `palettes/` on every push to main — do not edit by hand.*\n\n\
         Each preview is drawn from the file with the colours the player draws: day on the left, night \
         (dimmed, as the panel shows it) on the right. A palette without an accent of its own is shown \
         with Amber, and works with all six of Cinder's accents.\n\n| | Palette | File |\n|---|---|---|\n",
    );
    for p in good {
        fs::write(format!("previews/{}.svg", p.id), preview(p))?;
        let own = if p.tokens.accent.is_some() { "own accent" } else { "any accent" };
        let _ = writeln!(
            gallery,
            "| ![{0}](previews/{1}.svg) | **{0}**<br>{own} | [`{1}.palette`](palettes/{1}.palette) |",
            p.name, p.id
        );
    }
    fs::write("GALLERY.md", gallery)
}

fn hex(c: u32) -> String {
    format!("#{c:06x}")
}

/// A small Now Playing row in `s`: the panel strip, a title, an artist, a caption, the rail.
fn half(x: i32, s: &Shown, label: &str) -> String {
    format!(
        r##"<g transform="translate({x} 0)">
  <rect width="160" height="96" fill="{bg}"/>
  <rect width="160" height="18" fill="{panel}"/>
  <rect y="18" width="160" height="1" fill="{line}"/>
  <text x="8" y="13" font-size="9" fill="{faint}">{label}</text>
  <rect x="8" y="26" width="30" height="30" fill="{panel}" stroke="{line}"/>
  <text x="46" y="38" font-size="11" font-weight="600" fill="{ink}">Title</text>
  <text x="46" y="52" font-size="10" fill="{dim}">Artist</text>
  <rect x="8" y="66" width="144" height="3" rx="1.5" fill="{line}"/>
  <rect x="8" y="66" width="62" height="3" rx="1.5" fill="{acc}"/>
  <text x="8" y="86" font-size="8" fill="{faint}">FLAC 24/96</text>
  <circle cx="140" cy="84" r="8" fill="{acc}"/>
  <path d="M137 80 L145 84 L137 88 Z" fill="{acc_ink}"/>
</g>"##,
        bg = hex(s.bg),
        panel = hex(s.panel),
        line = hex(s.line),
        faint = hex(s.faint),
        ink = hex(s.ink),
        dim = hex(s.dim),
        acc = hex(s.acc),
        acc_ink = hex(s.acc_ink),
    )
}

fn preview(p: &Palette) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"320\" height=\"96\" viewBox=\"0 0 320 96\" \
         font-family=\"Segoe UI, Helvetica, Arial, sans-serif\">\n{}\n{}\n</svg>\n",
        half(0, &p.tokens.shown(false, 0), "DAY"),
        half(160, &p.tokens.shown(true, 0), "NIGHT"),
    )
}
