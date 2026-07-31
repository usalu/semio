//! 🔧 Includes codegen output from `bun ./📜script.ts generate`.

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let generated = manifest_dir.join("../../../../🕸️graph/🛂manifest/⚡️implementation/🦀rust/🤖generated/🦀registry.rs");
    println!("cargo:rerun-if-changed={}", generated.display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("../../../../🕸️graph/🛂manifest/⚡️implementation/🦀rust/📜script.ts").display());
    for entry in fs::read_dir(manifest_dir.join("../../..")).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.file_name().and_then(|n| n.to_str()) == Some("🛂manifest") {
            if let Ok(read) = fs::read_dir(&path) {
                for file in read.flatten() {
                    let p = file.path();
                    if p.extension().and_then(|e| e.to_str()) == Some("json") && p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".manifest.json")) {
                        println!("cargo:rerun-if-changed={}", p.display());
                    }
                }
            }
        }
    }
    for rel in [
        "../../../../../../s/plugin/trinity/manifest",
        "../../../../../../s/plugin/puzzle/app/2d/manifest",
        "../../../../../../s/plugin/puzzle/app/3d/manifest",
        "../../../../../../s/plugin/puzzle/app/5d/manifest",
        "../../../../../../s/plugin/flow/manifest",
        "../../../../../../s/plugin/draw/manifest",
        "../../../../../../s/plugin/writer/manifest",
        "../../../../../../s/plugin/reasoning/app/wires/manifest",
    ] {
        let dir = manifest_dir.join(rel);
        if dir.is_dir() {
            if let Ok(read) = fs::read_dir(&dir) {
                for file in read.flatten() {
                    let p = file.path();
                    if p.extension().and_then(|e| e.to_str()) == Some("json") {
                        println!("cargo:rerun-if-changed={}", p.display());
                    }
                }
            }
        }
    }
    if !generated.is_file() {
        let status = std::process::Command::new("bun")
            .args(["./📜script.ts", "generate"])
            .current_dir(&manifest_dir)
            .status()?;
        if !status.success() || !generated.is_file() {
            return Err(format!(
                "missing {} — run `bun nx run @semio-tech/graph-manifest:generate` first",
                generated.display()
            )
            .into());
        }
    }
    Ok(())
}
