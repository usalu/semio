use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let icons_dir = manifest_dir.join("../../../../../../../../🔨module/🖱️ui/🖼️asset/⚡️implementation/🟦typescript/🔣icon");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let out_icons = out_dir.join("icons");
    let _ = fs::remove_dir_all(&out_icons);
    fs::create_dir_all(&out_icons).expect("create OUT_DIR/icons");

    println!("cargo:rerun-if-changed={}", icons_dir.display());
    fs::copy(manifest_dir.join("🔣semio_logo.svg"), out_dir.join("🔣semio_logo.svg")).expect("copy semio logo");

    let mut entries: Vec<String> = Vec::new();
    if let Ok(read_dir) = fs::read_dir(&icons_dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("svg") {
                continue;
            }
            let Some(id) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            if id.starts_with('.') {
                continue;
            }
            let dest = out_icons.join(format!("{id}.svg"));
            fs::copy(&path, &dest).expect("copy icon svg");
            entries.push(id.to_string());
        }
    }
    entries.sort();

    let mut out = String::from("// @emoji 🖼 Auto-generated icon SVG embeds — do not edit by hand.\n\n");
    out.push_str("pub const SEMIO_LOGO_SVG: &str = include_str!(\"🔣semio_logo.svg\");\n\n");
    out.push_str("pub static ICON_SVGS: &[(&str, &str)] = &[\n");
    for id in &entries {
        out.push_str(&format!("    (\"{id}\", include_str!(\"icons/{id}.svg\")),\n"));
    }
    out.push_str("];\n");
    fs::write(out_dir.join("icons_🤖generated.rs"), out).expect("write icons_🤖generated.rs");
}
