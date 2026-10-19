//! 👪️ Source family reader for source-text laws: lists and reads every authored `.rs` file below a folder as one text, in path order, for laws about a carved crate family whose members mount each other through Cargo rather than `#[path]`.
//!
//! 🧭️ The independent oracle is the TypeScript `readSourceFamilySource` of `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree`; both read the same closed fixture. Canonical `🧪️tests` and `🧪️testing` folders and `node_modules` stay out, names order by UTF-16 code unit exactly like the TypeScript string comparison.

use std::path::{Path, PathBuf};

const EXCLUDED_DIRECTORIES: [&str; 3] = ["🧪️tests", "🧪️testing", "node_modules"];

/// 👪️ Lists every authored `.rs` file below a folder in path order, directories and files interleaved by name.
pub fn list_source_family(directory: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    visit(directory, &mut files)?;
    Ok(files)
}

/// 📖️ The whole family as one text: every listed file, each piece newline-terminated.
pub fn read_source_family_source(directory: &Path) -> Result<String, String> {
    let mut text = String::new();
    for file in list_source_family(directory)? {
        let piece = std::fs::read_to_string(&file).map_err(|error| format!("Source family file unreadable: {} ({error})", file.display()))?;
        text.push_str(&piece);
        if !piece.ends_with('\n') {
            text.push('\n');
        }
    }
    Ok(text)
}

fn visit(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let unreadable = |error: std::io::Error| format!("Source family folder unreadable: {} ({error})", directory.display());
    let mut entries = std::fs::read_dir(directory).map_err(unreadable)?.collect::<Result<Vec<_>, _>>().map_err(unreadable)?;
    entries.sort_by_cached_key(|entry| entry.file_name().to_string_lossy().encode_utf16().collect::<Vec<u16>>());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let kind = entry.file_type().map_err(unreadable)?;
        if kind.is_dir() {
            if !EXCLUDED_DIRECTORIES.contains(&name.as_str()) {
                visit(&entry.path(), files)?;
            }
        } else if kind.is_file() && entry.path().extension().is_some_and(|extension| extension == "rs") {
            files.push(entry.path());
        }
    }
    Ok(())
}
