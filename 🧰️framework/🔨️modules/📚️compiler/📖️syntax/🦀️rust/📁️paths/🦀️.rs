//! 📁️ Inspects whole first literal path arguments with caller-owned neutral roots.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustPathLiteralContext {
    Method,
    Qualified,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustPathLiteral {
    pub value: String,
    pub path: String,
    pub root: String,
    pub call: String,
    pub context: RustPathLiteralContext,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RustPathRootError;

impl RustPathRootError {
    pub fn code(&self) -> &'static str {
        "invalid-root"
    }
}

impl std::fmt::Display for RustPathRootError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Rust path root must be unique and canonical relative")
    }
}

impl std::error::Error for RustPathRootError {}

use super::tokens::{Kind, tokens, string_value, token_pairs};

fn qualified_call(call: &str) -> bool {
    matches!(call, "std::path::Path::new" | "std::path::PathBuf::from" | "std::fs::File::open" | "std::fs::File::create")
        || call.strip_prefix("std::fs::").is_some_and(|name| matches!(name, "read" | "read_to_string" | "read_dir" | "write" | "metadata" | "symlink_metadata" | "canonicalize" | "create_dir" | "create_dir_all" | "remove_file" | "remove_dir" | "remove_dir_all"))
}

/// 🧭️ Returns syntactic whole first-literal references; receiver and alias types remain unresolved.
pub fn inspect_rust_path_literals(source: &str, roots: &[&str]) -> Result<Vec<RustPathLiteral>, RustPathRootError> {
    let mut seen = std::collections::HashSet::new();
    for root in roots {
        if !seen.insert(*root) || root.chars().any(|value| matches!(value, '\\' | ':' | '\0' | '\r' | '\n')) || root.split('/').any(|part| matches!(part, "" | "." | "..")) { return Err(RustPathRootError); }
    }
    let tokens = tokens(source);
    let pairs = token_pairs(&tokens);
    let mut output = Vec::new();
    for open in 1..tokens.len() {
        if tokens[open].text != "(" || tokens[open - 1].kind != Kind::Identifier { continue; }
        let Some(close) = pairs[open] else { continue; };
        let name = tokens[open - 1].text.strip_prefix("r#").unwrap_or(tokens[open - 1].text);
        let method = open >= 2 && tokens[open - 2].text == ".";
        let mut first = open - 1;
        if !method { while first >= 2 && tokens[first - 1].text == "::" && tokens[first - 2].kind == Kind::Identifier { first -= 2; } }
        let call = if method { name.to_owned() } else { tokens[first..open].iter().map(|token| token.text).collect::<String>() };
        if !(if method { matches!(name, "join" | "push") } else { qualified_call(&call) }) { continue; }
        let Some(argument) = tokens.get(open + 1) else { continue; };
        if open + 2 != close && tokens.get(open + 2).is_none_or(|token| token.text != ",") { continue; }
        let Some(value) = string_value(argument) else { continue; };
        let normalized = value.replace('\\', "/");
        let mut path = normalized.as_str();
        while let Some(next) = path.strip_prefix("./").or_else(|| path.strip_prefix("../")) { path = next; }
        let Some(root) = roots.iter().filter(|root| path == **root || path.strip_prefix(**root).is_some_and(|suffix| suffix.starts_with('/'))).max_by_key(|root| root.encode_utf16().count()) else { continue; };
        if path.chars().any(|value| matches!(value, '\0' | '\r' | '\n')) { continue; }
        output.push(RustPathLiteral { value, path: path.to_owned(), root: (*root).to_owned(), call, context: if method { RustPathLiteralContext::Method } else { RustPathLiteralContext::Qualified }, start: argument.start, end: argument.end, line: argument.line });
    }
    output.sort_by_key(|value| value.start);
    Ok(output)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
