//! 📤️ Exports the genuine schema registry from a fresh production process.

use semio_framework_schema::{register_framework_schema_exports, SchemaExportEntries};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = match (args.next(), args.next(), args.next()) {
        (Some(flag), Some(path), None) if flag == "--out" => path,
        _ => return Err("schema-export-entries --out <path>".into()),
    };
    register_framework_schema_exports()?;
    let dump = SchemaExportEntries::from_catalog("semio-framework-schema schema-export-entries");
    let output = Path::new(&output);
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(output, dump.to_json())?;
    println!("[DEBUG] production schema registry entries={}", dump.entries.len());
    Ok(())
}
