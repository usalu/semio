//! 🔧 Build-time codegen for universal icon shortcode lookup (emoji, UI catalog, metabolism).

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize)]
struct GeneratedShortcodes {
    emoji: BTreeMap<String, String>,
    catalog: Vec<String>,
}

fn repo_ui_assets(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("../../../../../../../🔨module/🖱️ui/🖼️asset/⚡️implementation/🟦typescript")
}

fn metabolism_icons_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("../../../../../../../🔨module/🖼️asset/⚡️implementation/🟦typescript/🌱metabolism/🔣icon")
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let ui_assets = repo_ui_assets(&manifest_dir);
    let shortcodes_path = ui_assets.join("🔣icon/🤖generated/shortcodes.json");
    let icons_dir = ui_assets.join("🔣icon");
    let metabolism_dir = metabolism_icons_dir(&manifest_dir);

    println!("cargo:rerun-if-changed={}", shortcodes_path.display());
    println!("cargo:rerun-if-changed={}", icons_dir.display());
    println!("cargo:rerun-if-changed={}", metabolism_dir.display());

    let raw = fs::read_to_string(&shortcodes_path).unwrap_or_else(|e| panic!("read {}: {e}. Run `bun nx run @ui/asset:generate` first.", shortcodes_path.display()));
    let spec: GeneratedShortcodes = serde_json::from_str(&raw).expect("parse shortcodes.json");

    let mut emoji_arms = String::new();
    for (code, emoji) in &spec.emoji {
        emoji_arms.push_str(&format!("        {code:?} => Some(ShortcodeResolved::Emoji({emoji:?})),\n", code = code, emoji = emoji));
    }

    let mut catalog_arms = String::new();
    for id in &spec.catalog {
        let svg_path = icons_dir.join(format!("{id}.svg"));
        if !svg_path.is_file() {
            continue;
        }
        let safe = id.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect::<String>();
        let dest = out_dir.join(format!("shortcode_catalog_{safe}.svg"));
        fs::copy(&svg_path, &dest).unwrap_or_else(|e| panic!("copy {:?} -> {:?}: {e}", svg_path, dest));
        catalog_arms.push_str(&format!("        {id:?} => Some(ShortcodeResolved::SvgPlain(include_str!(concat!(env!(\"OUT_DIR\"), \"/shortcode_catalog_{safe}.svg\")))),\n", id = id, safe = safe));
    }

    let mut metabolism_arms = String::new();
    if metabolism_dir.is_dir() {
        for ent in fs::read_dir(&metabolism_dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", metabolism_dir.display())) {
            let ent = ent.expect("dir entry");
            let path = ent.path();
            if path.extension().and_then(|x| x.to_str()) != Some("svg") {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).expect("svg stem");
            let safe = stem.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect::<String>();
            let dest = out_dir.join(format!("shortcode_metabolism_{safe}.svg"));
            fs::copy(&path, &dest).unwrap_or_else(|e| panic!("copy {:?} -> {:?}: {e}", path, dest));
            metabolism_arms.push_str(&format!("        {stem:?} => Some(ShortcodeResolved::SvgThemed(include_str!(concat!(env!(\"OUT_DIR\"), \"/shortcode_metabolism_{safe}.svg\")))),\n", stem = stem, safe = safe));
        }
    }

    let out = format!(
        r#"// Generated shortcode resolver (emoji, UI catalog, metabolism).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortcodeResolved {{
    Emoji(&'static str),
    SvgPlain(&'static str),
    SvgThemed(&'static str),
}}

pub fn icon_shortcode_resolve(code: &str) -> Option<ShortcodeResolved> {{
    let c = code.trim();
    if c.is_empty() {{
        return None;
    }}
    if let Some(v) = resolve_emoji(c) {{
        return Some(v);
    }}
    if let Some(v) = resolve_metabolism(c) {{
        return Some(v);
    }}
    resolve_catalog(c)
}}

fn resolve_emoji(code: &str) -> Option<ShortcodeResolved> {{
    match code {{
{emoji_arms}        _ => None,
    }}
}}

fn resolve_catalog(code: &str) -> Option<ShortcodeResolved> {{
    match code {{
{catalog_arms}        _ => None,
    }}
}}

fn resolve_metabolism(code: &str) -> Option<ShortcodeResolved> {{
    match code {{
{metabolism_arms}        _ => None,
    }}
}}
"#
    );

    fs::write(out_dir.join("icon_shortcode_match.rs"), out).expect("write icon_shortcode_match.rs");
}
