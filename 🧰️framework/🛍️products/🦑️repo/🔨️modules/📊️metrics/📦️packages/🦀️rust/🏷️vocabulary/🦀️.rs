//! 🏷️ Vocabulary: the default branch, the language table and the counting vocabulary every other part names.

use super::GitIgnore;
use serde::Serialize;
use std::collections::BTreeSet;

//#region 🏷️Vocabulary

/// ⛳️ The development branch a `loc --history` walk defaults to.
pub const DEFAULT_BRANCH: &str = "⛳️wip";

/// 🧮️ Synthetic row label summing every enabled code language.
pub const AGG_CODE: &str = "Code";

/// 🧮️ Synthetic row label for every markup file (HTML, Markdown and friends).
pub const AGG_MARKUP: &str = "Markup";

/// 🧮️ Synthetic row label for every structured-data file (JSON, YAML, TOML and friends).
pub const AGG_DATA: &str = "Data";

/// 🧮️ Synthetic row label summing code, markup and data.
pub const AGG_TOTAL: &str = "Total";

/// 🗣️ The code buckets a `loc` run counts unless `--languages` narrows them, in report order.
pub const DEFAULT_CODE_LANGUAGES: [&str; 5] = ["TypeScript", "Go", "C#", "Python", "Rust"];

/// 🖨️ The pretty format the numstat walk asks git for; every commit line starts with `COMMIT`.
pub const NUMSTAT_PRETTY: &str = "COMMIT%x09%H%x09%aN%x09%aE%x09%at";

/// 📜️ The exact `git log` argument vector the numstat walk uses, optionally pinned to one ref.
pub fn numstat_log_args(git_ref: &str) -> Vec<String> {
    let mut args: Vec<String> = ["log", "--no-merges", "--numstat", "--first-parent", "--reverse"].iter().map(|value| (*value).to_string()).collect();
    args.push(format!("--pretty=format:{NUMSTAT_PRETTY}"));
    if !git_ref.trim().is_empty() {
        args.push(git_ref.trim().to_string());
    }
    args
}

/// 📜️ The exact `git log` argument vector one chunk of the numstat walk uses.
///
/// A chunk states a contiguous segment of the first-parent chain as `<older>..<newer>`, or a single
/// commit for the segment that reaches the root. Every other argument is [`numstat_log_args`]', so a
/// chunked walk and a whole-history walk ask git the same question about every commit.
pub fn numstat_log_range_args(range: &str) -> Vec<String> {
    let mut args = numstat_log_args("");
    args.push(range.to_string());
    args
}

/// 🧵️ Splits a first-parent chain, newest commit first, into contiguous `<older>..<newer>` ranges
/// that together cover it exactly once, oldest range first.
///
/// The ranges are what makes the walk parallel: `git log --numstat` reads both blobs of every
/// changed file, which is the whole cost of a `loc` run over a large repository, and that cost is
/// per commit and therefore divisible. Concatenating the ranges' outputs in the returned order
/// reproduces the single walk byte for byte, because `--reverse` orders each range internally and
/// the ranges are already in history order.
pub fn numstat_log_ranges(chain: &[String], chunks: usize) -> Vec<String> {
    if chain.is_empty() {
        return Vec::new();
    }
    let chunks = chunks.clamp(1, chain.len());
    let size = chain.len().div_ceil(chunks);
    let mut ranges = Vec::new();
    let mut start = 0usize;
    while start < chain.len() {
        let end = (start + size - 1).min(chain.len() - 1);
        ranges.push(match chain.get(end + 1) {
            Some(base) => format!("{base}..{}", chain[start]),
            None => chain[start].clone(),
        });
        start = end + 1;
    }
    ranges.reverse();
    ranges
}

/// 🧵️ Joins the outputs of [`numstat_log_ranges`] back into one stream.
///
/// `--pretty=format:` writes no trailing newline and separates commits with a blank line, so the
/// seam between two ranges is exactly one newline more than what each range carries on its own.
pub fn join_numstat_chunks(chunks: &[String]) -> String {
    let mut joined = String::new();
    for chunk in chunks.iter().filter(|chunk| !chunk.is_empty()) {
        if !joined.is_empty() {
            joined.push('\n');
        }
        joined.push_str(chunk);
    }
    joined
}

/// 🧾️ Normalises a repository-relative path to forward slashes without a leading `./`.
pub fn normalize_repo_path(path: &str) -> String {
    let slashed = path.replace('\\', "/");
    slashed.strip_prefix("./").unwrap_or(&slashed).to_string()
}

/// 🏷️ Maps a path extension to a `loc` bucket (a code language, `Markup` or `Data`).
pub fn classify_loc_bucket(path: &str) -> Option<&'static str> {
    match extension_of(path).as_str() {
        ".ts" | ".tsx" | ".cts" | ".mts" | ".mtsx" => Some("TypeScript"),
        ".go" => Some("Go"),
        ".cs" => Some("C#"),
        ".py" => Some("Python"),
        ".rs" => Some("Rust"),
        ".html" | ".htm" | ".xhtml" | ".md" | ".markdown" | ".mdown" | ".mkd" | ".mdx" | ".mdc" | ".svx" | ".svxtheme" => Some(AGG_MARKUP),
        ".json" | ".jsonc" | ".yaml" | ".yml" | ".toml" | ".csv" | ".xml" | ".ini" | ".cfg" | ".conf" | ".properties" | ".editorconfig" | ".gitattributes" | ".gitmodules" => Some(AGG_DATA),
        _ => None,
    }
}

/// 🏷️ Maps a path to the fine-grained per-language bucket the TypeScript library reports.
///
/// This is the second projection of the same tree: `classify_loc_bucket` folds every markup and
/// data format into two aggregate rows because the `loc` table has a fixed shape, while the
/// library's repository dashboard names each format. Both are computed from the same extension
/// table, and both feed the SAME per-file counter [`count_unified_loc_for_file`].
pub fn classify_language_fine(path: &str) -> Option<&'static str> {
    let rel = normalize_repo_path(path);
    let base = rel.rsplit('/').next().unwrap_or(&rel).to_lowercase();
    if base == "dockerfile" || base.starts_with("dockerfile.") {
        return Some("Dockerfile");
    }
    if base == "makefile" || base == "justfile" {
        return Some("Makefile");
    }
    match extension_of(&rel).as_str() {
        ".ts" | ".tsx" | ".cts" | ".mts" | ".mtsx" => Some("TypeScript"),
        ".js" | ".mjs" | ".cjs" => Some("JavaScript"),
        ".go" => Some("Go"),
        ".cs" => Some("C#"),
        ".py" => Some("Python"),
        ".rs" => Some("Rust"),
        ".sh" | ".bash" | ".zsh" => Some("Shell"),
        ".ps1" => Some("PowerShell"),
        ".css" | ".scss" | ".sass" => Some("CSS"),
        ".sql" => Some("SQL"),
        ".html" | ".htm" | ".xhtml" => Some("HTML"),
        ".md" | ".markdown" | ".mdown" | ".mkd" | ".mdx" | ".mdc" | ".svx" => Some("Markdown"),
        ".tex" | ".sty" | ".cls" | ".ltx" | ".bib" => Some("TeX"),
        ".json" | ".jsonc" => Some("JSON"),
        ".yaml" | ".yml" => Some("YAML"),
        ".toml" => Some("TOML"),
        ".csv" => Some("CSV"),
        ".xml" => Some("XML"),
        _ => None,
    }
}

/// 🧩️ The lowercase extension of a path's last segment, `""` when it has none.
pub(super) fn extension_of(path: &str) -> String {
    let base = path.rsplit(['/', '\\']).next().unwrap_or(path);
    match base.rfind('.') {
        Some(index) if index > 0 || base.len() > 1 => base[index..].to_lowercase(),
        _ => String::new(),
    }
}

/// 🧾️ The enabled code-language set, trimmed exactly as the CLI flag delivers it.
pub fn make_lang_set(languages: &[String]) -> BTreeSet<String> {
    languages.iter().map(|value| value.trim().to_string()).collect()
}

/// 🧾️ The enabled code languages plus the two aggregate buckets git deltas are classified into.
pub fn make_numstat_lang_set(languages: &[String]) -> BTreeSet<String> {
    let mut set = make_lang_set(languages);
    set.insert(AGG_MARKUP.to_string());
    set.insert(AGG_DATA.to_string());
    set
}

/// 🏷️ The bucket of a path, but only when the active weight set enables it.
pub fn classify_for_numstat(path: &str, weights: &BTreeSet<String>) -> Option<String> {
    let bucket = classify_loc_bucket(path)?;
    if weights.contains(bucket) {
        Some(bucket.to_string())
    } else {
        None
    }
}

/// 🫥️ Whether any path segment starts with a dot (`.` and `..` do not count).
pub fn path_has_hidden_segment(rel: &str) -> bool {
    normalize_repo_path(rel).split('/').any(|segment| !segment.is_empty() && segment != "." && segment != ".." && segment.starts_with('.'))
}

/// 🧬️ Whether a path is the repository's own `.🧬semio` meta tree.
pub fn path_is_repo_meta(rel: &str) -> bool {
    let rel = normalize_repo_path(rel);
    rel == semio_framework_repo_workspace::SEMIO_DIR_NAME || rel.starts_with(&format!("{}/", semio_framework_repo_workspace::SEMIO_DIR_NAME))
}

/// 🗂️ Whether a path must not be counted: empty, repo meta, ignored, or under a dot directory.
pub fn path_skipped_for_loc(rel: &str, ignore: Option<&GitIgnore>) -> bool {
    let rel = normalize_repo_path(rel);
    if rel.is_empty() || path_is_repo_meta(&rel) {
        return true;
    }
    if ignore.is_some_and(|rules| rules.matches_path(&rel)) {
        return true;
    }
    path_has_hidden_segment(&rel)
}

/// 🔣️ Serialises any metrics value to compact JSON.
///
/// The test hosts of this repository are deliberately dependency-free and carry no serialization
/// crate of their own, so the crate that owns a shape is also the crate that renders it.
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}

//#endregion 🏷️Vocabulary
