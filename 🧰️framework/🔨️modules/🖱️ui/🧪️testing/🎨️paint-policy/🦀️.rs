//! 🎨️ Shared test mechanism for owned renderer paint policies.

use std::fs;
use std::path::{Path, PathBuf};

const COLOR_CONSTRUCTORS: &[&str] = &["Rgba::new(", "Rgba::from_srgb8("];

fn channels_are_hand_written(arguments: &str) -> bool {
    let channels: Vec<&str> = arguments.split(',').take(3).collect();
    if channels.len() < 3 {
        return false;
    }
    channels.iter().all(|argument| {
        let argument = argument.trim().trim_end_matches("_f32").trim_end_matches("_u8");
        !argument.is_empty() && argument.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '_' || c == '-')
    })
}

/// 🔍️ Argument text of the constructor call starting at `open` (the index just past its `(`), or
/// `None` when the call spans past the end of the line.
fn call_arguments(line: &str, open: usize) -> Option<&str> {
    let rest = &line[open..];
    let mut depth = 1_i32;
    for (index, character) in rest.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&rest[..index]);
                }
            }
            _ => {}
        }
    }
    None
}

fn rust_sources(directory: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name == "node_modules" || name == "target" || name.starts_with("🗑️") || name.starts_with('.') {
                continue;
            }
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

/// 🧭️ Nearest preceding `const`/`fn`/`static` item name, so an allowlist entry can name a symbol
/// instead of a line number that every neighbouring edit invalidates.
fn enclosing_item<'a>(lines: &'a [&'a str], index: usize) -> &'a str {
    for line in lines[..=index].iter().rev() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") || trimmed.starts_with("const ") || trimmed.starts_with("pub const ") || trimmed.starts_with("pub(crate) fn ") || trimmed.starts_with("static ") {
            return trimmed;
        }
    }
    ""
}

fn is_allowlisted(relative: &str, line: &str, item: &str, allowlist: &[(&str, &str, &str)]) -> bool {
    allowlist.iter().any(|(path, marker, _)| relative.ends_with(path) && (line.contains(marker) || item.starts_with(marker)))
}

/// 🔎️ Scans only the explicitly supplied owner roots and paint exceptions.
pub fn scan(root: &Path, scan_roots: &[&str], allowlist: &[(&str, &str, &str)]) -> (usize, Vec<String>) {
    let mut violations: Vec<String> = Vec::new();
    let mut scanned = 0_usize;
    for scan_root in scan_roots {
        let absolute = root.join(scan_root);
        assert!(absolute.is_dir(), "scan root {scan_root} is missing — the law would pass vacuously");
        let mut files = Vec::new();
        rust_sources(&absolute, &mut files);
        for file in files {
            let relative = file.strip_prefix(&root).unwrap_or(&file).to_string_lossy().to_string();
            if !relative.contains("🧊️wgpu") {
                continue;
            }
            if relative.contains("🧪️tests") {
                continue;
            }
            scanned += 1;
            let text = fs::read_to_string(&file).expect("wgpu source reads as utf8");
            let lines: Vec<&str> = text.lines().collect();
            for (index, line) in lines.iter().enumerate() {
                if line.trim_start().starts_with("//") {
                    continue;
                }
                for constructor in COLOR_CONSTRUCTORS {
                    let Some(at) = line.find(constructor) else { continue };
                    let Some(arguments) = call_arguments(line, at + constructor.len()) else { continue };
                    if !channels_are_hand_written(arguments) {
                        continue;
                    }
                    let item = enclosing_item(&lines, index);
                    if is_allowlisted(&relative, line, item, allowlist) {
                        continue;
                    }
                    violations.push(format!("{relative}:{}: {}", index + 1, line.trim()));
                }
            }
        }
    }
    (scanned, violations)
}
