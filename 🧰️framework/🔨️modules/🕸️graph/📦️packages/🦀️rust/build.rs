//! 🔧️ Guarantees the graph-manifest codegen output exists and re-runs it whenever a manifest source changes.

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// ⏱️ Records the most recent modification time seen across every watched source.
async fn note_newest(path: &Path, newest: &mut Option<SystemTime>) {
    let Ok(modified) = fs::metadata(path).and_then(|meta| meta.modified()) else { return };
    if newest.map_or(true, |current| modified > current) {
        *newest = Some(modified);
    }
}

/// 🌳️ Mirrors `📜️script.ts`'s own discovery: a manifest source is tagged by its `🛂️manifest*.json`
/// filename, never by a directory convention, and dot-directories hold parallel worktree checkouts.
async fn watch_manifest_sources(dir: &Path, newest: &mut Option<SystemTime>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if name.starts_with('.') || matches!(name, "node_modules" | "target" | "🤖️generated") {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            watch_manifest_sources(&path, newest);
        } else if name.starts_with("🛂️manifest.json") && name.ends_with(".json") {
            println!("cargo:rerun-if-changed={}", path.display());
            note_newest(&path, newest);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = manifest_dir.join("../../../../..");
    let generated = manifest_dir.join("../../🤖️generated/🦀️registry.rs");
    println!("cargo:rerun-if-changed={}", generated.display());
    let script = manifest_dir.join("📜️script.ts");
    println!("cargo:rerun-if-changed={}", script.display());

    let mut newest_source = None;
    note_newest(&script, &mut newest_source);
    watch_manifest_sources(&repo_root, &mut newest_source);

    let stale = match fs::metadata(&generated).and_then(|meta| meta.modified()) {
        Ok(generated_at) => newest_source.map_or(false, |source_at| source_at > generated_at),
        Err(_) => true,
    };

    if stale {
        let status = std::process::Command::new("bun").args(["./📜️script.ts", "generate"]).current_dir(&manifest_dir).status()?;
        if !status.success() || !generated.is_file() {
            return Err(format!("missing {} — run `bun nx run @semio-tech/framework-graph:generate` first", generated.display()).into());
        }
    }
    Ok(())
}
