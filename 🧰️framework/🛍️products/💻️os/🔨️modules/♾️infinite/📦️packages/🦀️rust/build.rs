//! 🔧️ Cargo-owned build-time codegen for universal icon shortcode lookup.

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
    manifest_dir.join("../../../../../../🔨️modules/🖼️assets")
}

fn metabolism_icons_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("../../../../../../🔨️modules/🖼️assets/🌱️metabolism/🔣️icons")
}

/// 🧭️Indexes exact icon paths without changing public IDs or their letter case.
fn icon_sources(root: &Path, directory: &Path, sources: &mut BTreeMap<String, PathBuf>) {
    for entry in fs::read_dir(directory).expect("read icon source directory") {
        let entry = entry.expect("read icon source entry");
        if directory == root && entry.file_name() == "🤖️generated" {
            continue;
        }
        let kind = entry.file_type().expect("read icon source kind");
        assert!(!kind.is_symlink(), "linked icon source is not admitted");
        let path = entry.path();
        if kind.is_dir() {
            icon_sources(root, &path, sources);
        } else if kind.is_file() && path.extension().and_then(|value| value.to_str()) == Some("svg") {
            let stem = path.file_stem().and_then(|value| value.to_str()).expect("UTF-8 icon stem");
            let (_, id) = stem.rsplit_once(|value: char| !value.is_ascii_alphanumeric() && value != '-' && value != '_').expect("emoji-prefixed icon stem");
            assert!(!id.is_empty(), "empty icon identity");
            assert!(sources.insert(id.to_owned(), path.strip_prefix(root).expect("icon owner path").to_owned()).is_none(), "duplicate icon identity");
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let ui_assets = repo_ui_assets(&manifest_dir);
    let shortcodes_path = ui_assets.join("🔣️icons/🤖️generated/🔣️shortcodes.json");
    let icons_dir = ui_assets.join("🔣️icons");
    let metabolism_dir = metabolism_icons_dir(&manifest_dir);

    println!("cargo:rerun-if-changed={}", shortcodes_path.display());
    println!("cargo:rerun-if-changed={}", icons_dir.display());
    println!("cargo:rerun-if-changed={}", metabolism_dir.display());

    let raw = fs::read_to_string(&shortcodes_path).unwrap_or_else(|e| panic!("read {}: {e}. Run `bun nx run @semio-tech/assets:build` first.", shortcodes_path.display()));
    let spec: GeneratedShortcodes = serde_json::from_str(&raw).expect("parse 🔣️shortcodes.json");

    let mut emoji_arms = String::new();
    for (code, emoji) in &spec.emoji {
        emoji_arms.push_str(&format!("        {code:?} => Some(ShortcodeResolved::Emoji({emoji:?})),\n", code = code, emoji = emoji));
    }

    let mut catalog_arms = String::new();
    let mut catalog_sources = BTreeMap::new();
    icon_sources(&icons_dir, &icons_dir, &mut catalog_sources);
    assert_eq!(catalog_sources.len(), spec.catalog.len(), "catalog identity count differs from shortcode authority");
    for id in &spec.catalog {
        let path = catalog_sources.get(id).expect("catalog identity must resolve");
        let dest = out_dir.join("🖼️catalog").join(path);
        fs::create_dir_all(dest.parent().expect("catalog output parent")).expect("create catalog output directory");
        fs::copy(icons_dir.join(path), &dest).expect("copy catalog source");
        let path = path.to_str().expect("UTF-8 catalog path").replace('\\', "/");
        catalog_arms.push_str(&format!("        {id:?} => Some(ShortcodeResolved::SvgPlain(include_str!(\"🖼️catalog/{path}\"))),\n"));
    }

    let mut metabolism_arms = String::new();
    let mut metabolism_sources = BTreeMap::new();
    icon_sources(&metabolism_dir, &metabolism_dir, &mut metabolism_sources);
    assert!(!metabolism_sources.is_empty(), "metabolism catalog must not be empty");
    for (id, path) in metabolism_sources {
        let dest = out_dir.join("🌱️metabolism").join(&path);
        fs::create_dir_all(dest.parent().expect("metabolism output parent")).expect("create metabolism output directory");
        fs::copy(metabolism_dir.join(&path), &dest).expect("copy metabolism source");
        let path = path.to_str().expect("UTF-8 metabolism path").replace('\\', "/");
        metabolism_arms.push_str(&format!("        {id:?} => Some(ShortcodeResolved::SvgThemed(include_str!(\"🌱️metabolism/{path}\"))),\n"));
    }

    let out = format!(
        r#"// Generated shortcode resolver (emoji, UI catalog, metabolism).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
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

    fs::write(out_dir.join("🔎️shortcodes.rs"), out).expect("write shortcode bindings");
}
