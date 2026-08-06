//! 🔧️ Guarantees the graph-manifest codegen output exists and re-runs it whenever a manifest source changes.

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// 🌳️ Mirrors `📜️script.ts`'s own discovery: a manifest source is tagged by its `🛂️manifest*.json`
/// filename, never by a directory convention, and dot-directories hold parallel worktree checkouts.
fn watch_manifest_sources(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if name.starts_with('.') || matches!(name, "node_modules" | "target" | "🤖️generated") {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            watch_manifest_sources(&path);
        } else if name.starts_with("🛂️manifest.json") && name.ends_with(".json") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = manifest_dir.join("../../../../..");
    let generated = manifest_dir.join("../../🤖️generated/🦀️registry.rs");
    println!("cargo:rerun-if-changed={}", generated.display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("📜️script.ts").display());
    watch_manifest_sources(&repo_root);
    if !generated.is_file() {
        let status = std::process::Command::new("bun").args(["./📜️script.ts", "generate"]).current_dir(&manifest_dir).status()?;
        if !status.success() || !generated.is_file() {
            return Err(format!("missing {} — run `bun nx run @semio-tech/framework-math:generate` first", generated.display()).into());
        }
    }
    Ok(())
}
