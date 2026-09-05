use std::env;
use std::fs;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 🧭️ Collects source SVGs by public identity while retaining their exact domain paths.
fn catalog_sources(root: &Path, directory: &Path, sources: &mut BTreeMap<String, PathBuf>) {
    for entry in fs::read_dir(directory).expect("read icon source directory") {
        let entry = entry.expect("read icon source entry");
        if directory == root && entry.file_name() == "🤖️generated" {
            continue;
        }
        let kind = entry.file_type().expect("read icon source kind");
        assert!(!kind.is_symlink(), "linked icon source is not admitted");
        let path = entry.path();
        if kind.is_dir() {
            catalog_sources(root, &path, sources);
        } else if kind.is_file() && path.extension().and_then(|value| value.to_str()) == Some("svg") {
            let stem = path.file_stem().and_then(|value| value.to_str()).expect("UTF-8 icon stem");
            let id = stem.trim_start_matches(|value: char| !value.is_ascii_lowercase());
            assert!(!id.is_empty() && id.chars().all(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || value == '-' || value == '_'), "invalid public icon identity");
            assert!(sources.insert(id.to_owned(), path.strip_prefix(root).expect("icon owner path").to_owned()).is_none(), "duplicate public icon identity");
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let icons_dir = manifest_dir.join("../../../../../../../../../🔨️modules/🖼️assets/🔣️icons");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let out_icons = out_dir.join("🔣️icons");
    let _ = fs::remove_dir_all(&out_icons);
    fs::create_dir_all(&out_icons).expect("create OUT_DIR/icons");

    println!("cargo:rerun-if-changed={}", icons_dir.display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("../../../../🔣️.svg").display());
    fs::copy(manifest_dir.join("../../../../🔣️.svg"), out_dir.join("🪧️semio_logo.svg")).expect("copy semio logo");

    let mut entries = BTreeMap::new();
    catalog_sources(&icons_dir, &icons_dir, &mut entries);
    assert!(!entries.is_empty(), "icon source catalog must not be empty");
    for path in entries.values() {
        let dest = out_icons.join(path);
        fs::create_dir_all(dest.parent().expect("icon parent")).expect("create icon output group");
        fs::copy(icons_dir.join(path), dest).expect("copy icon svg");
    }

    let mut out = String::from("// @emoji 🖼️ Auto-generated icon SVG embeds — do not edit by hand.\n\n");
    out.push_str("pub const SEMIO_LOGO_SVG: &str = include_str!(\"🪧️semio_logo.svg\");\n\n");
    out.push_str("pub static ICON_SVGS: &[(&str, &str)] = &[\n");
    for (id, path) in &entries {
        let path = path.to_str().expect("UTF-8 icon source path").replace('\\', "/");
        out.push_str(&format!("    (\"{id}\", include_str!(\"🔣️icons/{path}\")),\n"));
    }
    out.push_str("];\n");
    fs::write(out_dir.join("🧩️icons.rs"), out).expect("write icon bindings");
}
