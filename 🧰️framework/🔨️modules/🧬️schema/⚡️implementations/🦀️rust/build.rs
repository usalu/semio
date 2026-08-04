//! 🔧️ Runs entity-catalog codegen when `🤖️generated.rs` is missing.

use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let generated = manifest_dir.join("🤖️generated.rs");
    println!("cargo:rerun-if-changed={}", generated.display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("🔣️entity-kinds.json").display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("📜️script.ts").display());
    if !generated.is_file() {
        let status = std::process::Command::new("bun").args(["./📜️script.ts", "generate"]).current_dir(&manifest_dir).status()?;
        if !status.success() || !generated.is_file() {
            return Err(format!("missing {} — run `bun nx run @semio-tech/framework-schema:generate` first", generated.display()).into());
        }
    }
    Ok(())
}
