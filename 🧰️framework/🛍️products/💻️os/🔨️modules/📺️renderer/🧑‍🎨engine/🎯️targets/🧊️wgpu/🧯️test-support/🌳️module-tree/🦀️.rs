//! 🌳️ Module tree reader for source-text laws: lists and reads a Rust file with every `#[path]` file module and `include!("literal")` item below it, at run time instead of through `include_str!` embeds.
//!
//! 🧭️ The independent oracle is the TypeScript `readModuleTreeSource` of `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree`; both read the same closed fixture.

use crate::rust_tokens::{rust_token_pairs, rust_tokens, RustToken};
use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};

const TESTS_DIRECTORY: &str = "🧪️tests";

/// 🪢️ How a source pulls a file into its module tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleVia {
    Path,
    Include,
}

/// 🌳️ One `#[path = "…"] mod name;` declaration or `include!("…")` item, with the inline module names that enclose it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleChild {
    pub path: String,
    pub modules: Vec<String>,
    pub via: ModuleVia,
}

/// 🧭️ Lists every file module and literal include of a Rust source in declaration order; cfg attributes are ignored, inline module nesting is kept.
/// @see <https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute>
pub fn inspect_module_children(source: &str) -> Result<Vec<ModuleChild>, String> {
    let tokens = rust_tokens(source);
    let pairs = rust_token_pairs(&tokens);
    let mut children = Vec::new();
    scope(&tokens, &pairs, 0, tokens.len(), &[], &mut children)?;
    Ok(children)
}

/// 📚️ The root file and every file reachable through its module mounts below it, parent first, children in declaration order, each once; `🧪️tests` mounts are excluded.
pub fn read_module_tree_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    let start = normalize(root);
    let scope_directory = start.parent().map(Path::to_path_buf).unwrap_or_default();
    visit(&start, &scope_directory, &mut files, &mut seen, &mut |_, _| {})?;
    Ok(files)
}

/// 📖️ The whole module tree as one text: the parent first, then every child in declaration order, each piece newline-terminated.
pub fn read_module_tree_source(root: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    let start = normalize(root);
    let scope_directory = start.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut text = String::new();
    visit(&start, &scope_directory, &mut files, &mut seen, &mut |_, piece| {
        text.push_str(piece);
        if !piece.ends_with('\n') {
            text.push('\n');
        }
    })?;
    Ok(text)
}

/// ✂️ Slices one marker-delimited item out of the tree: inside the first file that holds `from`, up to the first `to` after it, or to the end of that file.
pub fn slice_module_tree_source(root: &Path, from: &str, to: Option<&str>) -> Result<String, String> {
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    let start = normalize(root);
    let scope_directory = start.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut pieces: Vec<String> = Vec::new();
    visit(&start, &scope_directory, &mut files, &mut seen, &mut |_, piece| pieces.push(piece.to_owned()))?;
    for piece in pieces {
        let Some(begin) = piece.find(from) else { continue };
        let end = to.and_then(|marker| piece[begin + from.len()..].find(marker).map(|offset| begin + from.len() + offset));
        return Ok(match end {
            Some(end) => piece[begin..end].to_owned(),
            None => piece[begin..].to_owned(),
        });
    }
    Err(format!("Module tree marker missing: {from}"))
}

fn visit(file: &Path, scope_directory: &Path, files: &mut Vec<PathBuf>, seen: &mut HashSet<PathBuf>, on_text: &mut dyn FnMut(&Path, &str)) -> Result<(), String> {
    if !seen.insert(file.to_path_buf()) {
        return Ok(());
    }
    let text = std::fs::read_to_string(file).map_err(|error| format!("Module tree file unreadable: {} ({error})", file.display()))?;
    files.push(file.to_path_buf());
    on_text(file, &text);
    let directory = file.parent().map(Path::to_path_buf).unwrap_or_default();
    for child in inspect_module_children(&text)? {
        let included = child.via == ModuleVia::Include;
        let mut target = directory.clone();
        if !included {
            for module in &child.modules {
                target.push(module);
            }
        }
        target.push(&child.path);
        let target = normalize(&target);
        let relative = relative_segments(scope_directory, &target);
        let is_test = relative.iter().any(|segment| segment == TESTS_DIRECTORY);
        if is_test || relative.first().is_some_and(|segment| segment == "..") {
            continue;
        }
        if !target.exists() {
            return Err(format!("Rust module tree mount missing: {} (declared in {})", target.display(), file.display()));
        }
        let child_scope = if included { scope_directory.to_path_buf() } else { target.parent().map(Path::to_path_buf).unwrap_or_default() };
        visit(&target, &child_scope, files, seen, on_text)?;
    }
    Ok(())
}

fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn relative_segments(from: &Path, to: &Path) -> Vec<String> {
    let from: Vec<_> = from.components().collect();
    let to: Vec<_> = to.components().collect();
    let common = from.iter().zip(to.iter()).take_while(|(left, right)| left == right).count();
    let mut segments: Vec<String> = from[common..].iter().map(|_| "..".to_owned()).collect();
    segments.extend(to[common..].iter().map(|component| component.as_os_str().to_string_lossy().into_owned()));
    segments
}

fn scope(tokens: &[RustToken], pairs: &[Option<usize>], start: usize, end: usize, modules: &[String], out: &mut Vec<ModuleChild>) -> Result<(), String> {
    let mut index = start;
    while index < end {
        if tokens[index].is_punct('#') && tokens.get(index + 1).is_some_and(|token| token.is_punct('!')) && tokens.get(index + 2).is_some_and(|token| token.is_punct('[')) {
            let close = pairs[index + 2].ok_or("Invalid inner Rust attribute")?;
            index = close + 1;
            continue;
        }
        let (after_attributes, paths) = attributes(tokens, pairs, index, end)?;
        let at = visibility(tokens, pairs, after_attributes);
        if tokens.get(at).is_some_and(|token| token.is_identifier("include"))
            && tokens.get(at + 1).is_some_and(|token| token.is_punct('!'))
            && tokens.get(at + 2).is_some_and(|token| token.is_punct('(') || token.is_punct('['))
            && pairs[at + 2] == Some(at + 4)
        {
            if let Some(RustToken::Str(Some(literal))) = tokens.get(at + 3) {
                out.push(ModuleChild { path: literal.clone(), modules: modules.to_vec(), via: ModuleVia::Include });
            }
        }
        if tokens.get(at).is_some_and(|token| token.is_identifier("mod")) {
            if let Some(RustToken::Identifier(name)) = tokens.get(at + 1) {
                let name = name.strip_prefix("r#").unwrap_or(name).to_owned();
                if paths.len() > 1 || paths.iter().any(Option::is_none) {
                    return Err(format!("Ambiguous Rust path mount: {name}"));
                }
                let mount = paths.first().cloned().flatten();
                let boundary = at + 2;
                if tokens.get(boundary).is_some_and(|token| token.is_punct('{')) {
                    let close = pairs[boundary].ok_or("Unbalanced inline module")?;
                    let mut nested = modules.to_vec();
                    nested.push(mount.unwrap_or(name));
                    scope(tokens, pairs, boundary + 1, close, &nested, out)?;
                    index = close + 1;
                    continue;
                }
                if tokens.get(boundary).is_some_and(|token| token.is_punct(';')) {
                    if let Some(path) = mount {
                        out.push(ModuleChild { path, modules: modules.to_vec(), via: ModuleVia::Path });
                    }
                    index = boundary + 1;
                    continue;
                }
            }
        }
        let Some(boundary) = find_top_level(tokens, pairs, at, end) else { break };
        index = pairs[boundary].map_or(boundary, |close| if close > boundary { close } else { boundary }) + 1;
    }
    Ok(())
}

fn attributes(tokens: &[RustToken], pairs: &[Option<usize>], start: usize, end: usize) -> Result<(usize, Vec<Option<String>>), String> {
    let mut index = start;
    let mut paths = Vec::new();
    while index + 1 < end && tokens[index].is_punct('#') && tokens[index + 1].is_punct('[') {
        let close = pairs[index + 1].ok_or("Invalid Rust attribute")?;
        if tokens.get(index + 2).is_some_and(|token| token.is_identifier("path")) {
            let value = match (tokens.get(index + 3), tokens.get(index + 4)) {
                (Some(equals), Some(RustToken::Str(Some(literal)))) if equals.is_punct('=') && index + 5 == close => Some(literal.clone()),
                _ => None,
            };
            paths.push(value);
        }
        index = close + 1;
    }
    Ok((index, paths))
}

fn visibility(tokens: &[RustToken], pairs: &[Option<usize>], at: usize) -> usize {
    if tokens.get(at).is_some_and(|token| token.is_identifier("pub")) {
        if tokens.get(at + 1).is_some_and(|token| token.is_punct('(')) {
            return pairs[at + 1].map_or(at + 1, |close| close + 1);
        }
        return at + 1;
    }
    at
}

fn find_top_level(tokens: &[RustToken], pairs: &[Option<usize>], start: usize, end: usize) -> Option<usize> {
    let mut index = start;
    while index < end {
        if tokens[index].is_punct(';') || tokens[index].is_punct('{') {
            return Some(index);
        }
        if (tokens[index].is_punct('(') || tokens[index].is_punct('[')) && pairs[index].is_some_and(|close| close > index) {
            index = pairs[index].unwrap_or(index);
        }
        index += 1;
    }
    None
}
