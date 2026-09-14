//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 📊️ Repository metrics: the pure `git log --numstat` parser, the unified-LOC counters, the
//! `LocReport` aggregation, UTC time bucketing and the benchmark timing model.
//!
//! Behaviour twin of `github.com/usalu/semio/repo/metrics`. Everything in this crate is a pure
//! function over data; the only door to the outside world is [`GitLogSource`], which the CLI
//! satisfies with [`SystemGit`] and the tests satisfy with a recorded [`GitTranscript`].
//!
//! Rendering the `loc` CLI verb (flags, engine wiring, colour) is NOT here — it belongs to
//! `⌨️cli`. What this crate exposes instead is the report model plus the two pure table renderers
//! [`render_markdown`] and [`render_text`].
//!
//! See `🧬️schema/🔣️.json` for the wire shapes and `🧪️tests/` for the language-agnostic cases.

//#endregion 🧲️Header

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

pub use semio_framework_repo_model::LineMetrics;
pub use semio_framework_repo_workspace::{find_repo_root, GitIgnore};

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
fn extension_of(path: &str) -> String {
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

//#region 📏️Counting

/// 📏️ Newline-terminated line count, `wc -l` style: a non-empty file has at least one line.
pub fn physical_line_count(data: &[u8]) -> usize {
    if data.is_empty() {
        return 0;
    }
    data.iter().filter(|byte| **byte == b'\n').count() + 1
}

/// 🧮️ Recursively counts the object keys of a JSON value; arrays contribute only their members.
pub fn count_json_keys(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Object(map) => map.len() + map.values().map(count_json_keys).sum::<usize>(),
        serde_json::Value::Array(items) => items.iter().map(count_json_keys).sum(),
        _ => 0,
    }
}

/// 🧮️ Object-key count of a JSON document, `0` when the bytes are blank or not valid JSON.
pub fn json_key_count(data: &[u8]) -> usize {
    let text = std::str::from_utf8(data).unwrap_or("").trim();
    if text.is_empty() {
        return 0;
    }
    serde_json::from_str::<serde_json::Value>(text).map_or(0, |value| count_json_keys(&value))
}

/// 📏️ Unified LOC of one tracked file body: JSON keys for `.json`/`.jsonc`, physical lines else.
///
/// This is the repository-wide definition of "one line of code". It is identical to the
/// TypeScript library's `countUnifiedLocForFile`, which the language-agnostic
/// `loc-aggregation` case uses as its cross-implementation reference.
pub fn count_unified_loc_for_file(rel: &str, data: &[u8]) -> usize {
    match extension_of(rel).as_str() {
        ".json" | ".jsonc" => {
            let keys = json_key_count(data);
            if keys > 0 {
                return keys;
            }
            if std::str::from_utf8(data).unwrap_or("").trim().is_empty() {
                return 0;
            }
            physical_line_count(data)
        }
        _ => physical_line_count(data),
    }
}

//#endregion 📏️Counting

//#region 🧩️Numstat

/// 🧩️ One file entry of a numstat commit block.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileDelta {
    /// 🧾️ The path the change lands on; for a rename this is the NEW path.
    pub path: String,
    /// ↩️ The path the change came from, present only for a rename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_from: Option<String>,
    pub added: i64,
    pub removed: i64,
    /// 🧱️ A binary change, which git reports as `-` / `-` and which carries no line counts.
    pub binary: bool,
    /// 🏷️ The `loc` bucket the path classified into, absent when the path is skipped or unweighted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
}

/// 💿️ One parsed commit and the per-bucket line deltas it contributes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CommitDelta {
    pub sha: String,
    pub when_unix: i64,
    pub author: String,
    pub author_mail: String,
    /// 🧾️ Every file line of the block, skipped ones included, in stream order.
    pub files: Vec<FileDelta>,
    /// 🧮️ Weighted per-bucket sums, which is what every aggregation reads.
    pub delta: BTreeMap<String, LineMetrics>,
}

/// 🔤️ Decodes a path the way git quotes it when `core.quotepath` is on: `"\360\237..."`.
///
/// git escapes every non-ASCII byte as a three-digit octal sequence inside a double-quoted
/// string. Leaving that encoded would classify `"🧬️x.ts"` under no bucket at all, so a numstat
/// parser that means to count this repository's own emoji-named tree has to decode it.
pub fn unquote_git_path(raw: &str) -> String {
    let trimmed = raw.trim();
    if !(trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"')) {
        return raw.to_string();
    }
    let inner: Vec<char> = trimmed[1..trimmed.len() - 1].chars().collect();
    let mut bytes: Vec<u8> = Vec::with_capacity(inner.len());
    let mut index = 0;
    while index < inner.len() {
        if inner[index] != '\\' {
            let mut buffer = [0_u8; 4];
            bytes.extend_from_slice(inner[index].encode_utf8(&mut buffer).as_bytes());
            index += 1;
            continue;
        }
        index += 1;
        if index >= inner.len() {
            break;
        }
        let escape = inner[index];
        index += 1;
        match escape {
            'n' => bytes.push(b'\n'),
            't' => bytes.push(b'\t'),
            'r' => bytes.push(b'\r'),
            'a' => bytes.push(7),
            'b' => bytes.push(8),
            'f' => bytes.push(12),
            'v' => bytes.push(11),
            '"' => bytes.push(b'"'),
            '\\' => bytes.push(b'\\'),
            digit if digit.is_digit(8) => {
                let mut value = digit.to_digit(8).unwrap_or(0);
                let mut taken = 1;
                while taken < 3 && index < inner.len() && inner[index].is_digit(8) {
                    value = value * 8 + inner[index].to_digit(8).unwrap_or(0);
                    index += 1;
                    taken += 1;
                }
                bytes.push((value & 0xff) as u8);
            }
            other => {
                let mut buffer = [0_u8; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buffer).as_bytes());
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// ↩️ Splits a numstat path field into `(new path, rename source)`.
///
/// git writes a rename either as `old => new` or, when the two paths share a prefix and a suffix,
/// as `shared/{old => new}/tail`. Counting the raw field would classify `a.ts => b.rs` by the
/// extension of the whole string, which is a bucket that does not exist.
pub fn resolve_numstat_path(raw: &str) -> (String, Option<String>) {
    let field = raw.trim_end_matches('\r');
    if let (Some(open), Some(close)) = (field.find('{'), field.find('}')) {
        if open < close {
            let middle = &field[open + 1..close];
            if let Some(arrow) = middle.find(" => ") {
                let prefix = &field[..open];
                let suffix = &field[close + 1..];
                let old = format!("{prefix}{}{suffix}", &middle[..arrow]);
                let new = format!("{prefix}{}{suffix}", &middle[arrow + 4..]);
                return (unquote_git_path(&collapse_slashes(&new)), Some(unquote_git_path(&collapse_slashes(&old))));
            }
        }
    }
    if let Some(arrow) = field.find(" => ") {
        return (unquote_git_path(field[arrow + 4..].trim()), Some(unquote_git_path(field[..arrow].trim())));
    }
    (unquote_git_path(field), None)
}

/// 🧹️ Collapses the empty segment a `{ => new}` rename leaves behind.
fn collapse_slashes(path: &str) -> String {
    let mut out = path.replace("//", "/");
    while out.contains("//") {
        out = out.replace("//", "/");
    }
    out
}

/// 🧩️ Breaks a `git log --numstat` stream into per-commit records with weighted bucket sums.
///
/// The parser is total: a malformed line is skipped, never fatal, because a log stream is
/// produced by a tool this crate does not control. `ignore` is the repository's ignore rule set
/// when one is available; passing `None` counts everything the log names.
pub fn parse_numstat_log(stdout: &str, weights: &BTreeSet<String>, ignore: Option<&GitIgnore>) -> Vec<CommitDelta> {
    let mut out: Vec<CommitDelta> = Vec::new();
    let mut current: Option<CommitDelta> = None;
    let mut buckets: HashMap<String, Option<String>> = HashMap::new();
    for raw in stdout.split('\n') {
        let line = raw.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("COMMIT") {
            let fields: Vec<&str> = rest.splitn(5, '\t').collect();
            if fields.len() < 5 {
                continue;
            }
            if let Some(finished) = current.take() {
                out.push(finished);
            }
            current = Some(CommitDelta {
                sha: fields[1].to_string(),
                author: fields[2].to_string(),
                author_mail: fields[3].to_string(),
                when_unix: fields[4].trim().parse::<i64>().unwrap_or(0),
                files: Vec::new(),
                delta: BTreeMap::new(),
            });
            continue;
        }
        let Some(commit) = current.as_mut() else { continue };
        let parts: Vec<&str> = line.splitn(3, '\t').collect();
        if parts.len() < 3 {
            continue;
        }
        let binary = parts[0] == "-" || parts[1] == "-";
        let (path, rename_from) = resolve_numstat_path(parts[2]);
        let added = parts[0].parse::<i64>().unwrap_or(0);
        let removed = parts[1].parse::<i64>().unwrap_or(0);
        if !binary && (parts[0].parse::<i64>().is_err() || parts[1].parse::<i64>().is_err()) {
            continue;
        }
        let bucket = if binary {
            None
        } else {
            buckets
                .entry(path.clone())
                .or_insert_with(|| if path_skipped_for_loc(&path, ignore) { None } else { classify_for_numstat(&path, weights) })
                .clone()
        };
        if let Some(name) = bucket.clone() {
            let entry = commit.delta.entry(name).or_default();
            entry.added += added;
            entry.removed += removed;
        }
        commit.files.push(FileDelta { path, rename_from, added: if binary { 0 } else { added }, removed: if binary { 0 } else { removed }, binary, bucket });
    }
    if let Some(finished) = current.take() {
        out.push(finished);
    }
    out
}

//#endregion 🧩️Numstat

//#region 📊️Report

/// 🔢️ Serializes a float the way Go's `encoding/json` does: a value with no fractional part
/// is written as an integer, so `0` and not `0.0` reaches a report a reader diffs against the
/// reference.
pub fn go_float<S: serde::Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 9.007_199_254_740_992e15 {
        return serializer.serialize_i64(*value as i64);
    }
    serializer.serialize_f64(*value)
}

/// 🔢️ [`go_float`] for an optional float; `None` never reaches this, the field skips it.
pub fn go_float_option<S: serde::Serializer>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error> {
    match value {
        Some(inner) => go_float(inner, serializer),
        None => serializer.serialize_none(),
    }
}

/// 💿️ One row of a `loc` table: tree LOC, its share, branch churn share and the git delta sums.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocLangStats {
    #[serde(rename = "loc")]
    pub loc: i64,
    #[serde(rename = "percent", serialize_with = "go_float")]
    pub percent: f64,
    #[serde(rename = "since_prev_loc_percent", skip_serializing_if = "Option::is_none", serialize_with = "go_float_option")]
    pub since_prev_loc_percent: Option<f64>,
    #[serde(rename = "wip_percent", serialize_with = "go_float")]
    pub wip_percent: f64,
    #[serde(rename = "edited")]
    pub edited: i64,
    #[serde(rename = "added")]
    pub added: i64,
    #[serde(rename = "removed")]
    pub removed: i64,
}

/// 💿️ One commit step of the history series; `languages` and `by_contributors` are exclusive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocHistoryEntry {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "author", skip_serializing_if = "String::is_empty", default)]
    pub author: String,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<BTreeMap<String, LocLangStats>>,
    #[serde(rename = "byContributors", skip_serializing_if = "Option::is_none")]
    pub by_contributors: Option<BTreeMap<String, BTreeMap<String, LocLangStats>>>,
}

/// 💿️ The whole `loc` payload: the snapshot table plus the optional breakdowns.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocReport {
    #[serde(rename = "snapshot")]
    pub snapshot: BTreeMap<String, LocLangStats>,
    #[serde(rename = "byContributors", skip_serializing_if = "Option::is_none")]
    pub by_contributors: Option<BTreeMap<String, BTreeMap<String, LocLangStats>>>,
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<LocHistoryEntry>>,
    #[serde(rename = "branch", skip_serializing_if = "String::is_empty", default)]
    pub branch: String,
}

/// 👤️ The default contributor alias: `Name <mail>`, or `unknown` when git recorded neither.
pub fn default_contributor_alias(name: &str, email: &str) -> String {
    let author = name.trim();
    let mail = email.trim();
    let combined = if mail.is_empty() { author.to_string() } else { format!("{author} <{mail}>") };
    if combined.trim().is_empty() {
        "unknown".to_string()
    } else {
        combined
    }
}

/// 🧮️ Running added/removed sums keyed by bucket.
pub type Cumulative = BTreeMap<String, LineMetrics>;

/// 🧮️ Running sums keyed by contributor alias and then by bucket.
pub type CumulativeByContributor = BTreeMap<String, Cumulative>;

/// 🧮️ Folds a commit stream into cumulative sums, optionally split by contributor and filtered.
pub fn cumulative_from_raw(commits: &[CommitDelta], languages: &[String], by_contributor: bool, contributor_filter: &str, alias: &dyn Fn(&str, &str) -> String) -> (Cumulative, Option<CumulativeByContributor>) {
    let weights = make_numstat_lang_set(languages);
    let mut cumulative: Cumulative = weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect();
    let mut by_contributors: Option<CumulativeByContributor> = if by_contributor { Some(BTreeMap::new()) } else { None };
    let filter = contributor_filter.trim();
    for commit in commits {
        let who = alias(&commit.author, &commit.author_mail);
        if !filter.is_empty() && !filter.eq_ignore_ascii_case(&who) {
            continue;
        }
        if let Some(map) = by_contributors.as_mut() {
            map.entry(who.clone()).or_insert_with(|| weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect());
        }
        for (bucket, pair) in &commit.delta {
            if !weights.contains(bucket) {
                continue;
            }
            let total = cumulative.entry(bucket.clone()).or_default();
            total.added += pair.added;
            total.removed += pair.removed;
            if let Some(map) = by_contributors.as_mut() {
                let row = map.entry(who.clone()).or_default().entry(bucket.clone()).or_default();
                row.added += pair.added;
                row.removed += pair.removed;
            }
        }
    }
    (cumulative, by_contributors)
}

/// 🧩️ Builds one table row out of the tree scan and the cumulative git pair for a bucket.
pub fn stat_from_pair_and_scan(cumulative: &Cumulative, scanned: Option<&BTreeMap<String, i64>>, key: &str) -> LocLangStats {
    let mut stat = LocLangStats { loc: scanned.and_then(|map| map.get(key).copied()).unwrap_or(0), ..LocLangStats::default() };
    if let Some(pair) = cumulative.get(key) {
        stat.added = pair.added;
        stat.removed = pair.removed;
        stat.edited = pair.added + pair.removed;
    }
    stat
}

/// 🧮️ Stamps `percent` with `Total.loc` as denominator; the `Total` row is 100 % by construction.
pub fn apply_percents(rows: &mut BTreeMap<String, LocLangStats>) {
    let denominator = rows.get(AGG_TOTAL).map_or(0, |row| row.loc);
    if denominator <= 0 {
        for row in rows.values_mut() {
            row.percent = 0.0;
        }
        return;
    }
    for (key, row) in rows.iter_mut() {
        row.percent = if key == AGG_TOTAL { 100.0 } else { (10000.0 * row.loc as f64 / denominator as f64).round() / 100.0 };
    }
}

/// 📊️ Sums added + removed over every weighted bucket: the branch's whole edited-line churn.
pub fn sum_edited_pairs(cumulative: &Cumulative, weights: &BTreeSet<String>) -> i64 {
    weights.iter().filter_map(|bucket| cumulative.get(bucket)).map(|pair| pair.added + pair.removed).sum()
}

/// 🧮️ Stamps `wip_percent` from edited churn; a zero global denominator falls back to `Total`.
pub fn apply_wip_percents(rows: &mut BTreeMap<String, LocLangStats>, global_denominator: i64) {
    let mut denominator = global_denominator;
    if denominator <= 0 {
        denominator = rows.get(AGG_TOTAL).map_or(0, |row| row.edited);
    }
    if denominator <= 0 {
        for row in rows.values_mut() {
            row.wip_percent = 0.0;
        }
        return;
    }
    for row in rows.values_mut() {
        row.wip_percent = (10000.0 * row.edited as f64 / denominator as f64).round() / 100.0;
    }
}

/// 📊️ Merges tree scan and git deltas into the full row set with its `Code`/`Total` aggregates.
pub fn compose_loc_report_snapshot(cumulative: &Cumulative, scanned: Option<&BTreeMap<String, i64>>, languages: &[String], wip_global_denominator: i64) -> BTreeMap<String, LocLangStats> {
    let weights = make_numstat_lang_set(languages);
    let mut rows: BTreeMap<String, LocLangStats> = BTreeMap::new();
    let mut code = LocLangStats::default();
    for language in DEFAULT_CODE_LANGUAGES {
        if !weights.contains(language) {
            continue;
        }
        let stat = stat_from_pair_and_scan(cumulative, scanned, language);
        code.loc += stat.loc;
        code.added += stat.added;
        code.removed += stat.removed;
        code.edited += stat.edited;
        rows.insert(language.to_string(), stat);
    }
    let markup = stat_from_pair_and_scan(cumulative, scanned, AGG_MARKUP);
    let data = stat_from_pair_and_scan(cumulative, scanned, AGG_DATA);
    let total = LocLangStats {
        loc: code.loc + markup.loc + data.loc,
        edited: code.edited + markup.edited + data.edited,
        added: code.added + markup.added + data.added,
        removed: code.removed + markup.removed + data.removed,
        ..LocLangStats::default()
    };
    rows.insert(AGG_MARKUP.to_string(), markup);
    rows.insert(AGG_DATA.to_string(), data);
    rows.insert(AGG_CODE.to_string(), code);
    rows.insert(AGG_TOTAL.to_string(), total);
    apply_percents(&mut rows);
    apply_wip_percents(&mut rows, wip_global_denominator);
    rows
}

/// 📶️ Row names ordered by tree LOC descending, ties broken by name, with `Total` last.
pub fn sorted_row_keys(rows: &BTreeMap<String, LocLangStats>) -> Vec<String> {
    sorted_rows_by(rows, |row| row.loc)
}

/// 📶️ Row names ordered by edited churn descending, ties broken by name, with `Total` last.
pub fn sorted_row_keys_churn(rows: &BTreeMap<String, LocLangStats>) -> Vec<String> {
    sorted_rows_by(rows, |row| row.edited)
}

/// 📶️ Shared ordering: one numeric key descending, name ascending, `Total` pinned last.
fn sorted_rows_by(rows: &BTreeMap<String, LocLangStats>, key: impl Fn(&LocLangStats) -> i64) -> Vec<String> {
    let mut names: Vec<String> = rows.keys().filter(|name| *name != AGG_TOTAL).cloned().collect();
    names.sort_by(|left, right| {
        let value_left = rows.get(left).map_or(0, &key);
        let value_right = rows.get(right).map_or(0, &key);
        value_right.cmp(&value_left).then_with(|| left.cmp(right))
    });
    if rows.contains_key(AGG_TOTAL) {
        names.push(AGG_TOTAL.to_string());
    }
    names
}

/// 🧾️ Whether a table has real tree LOC, which is what decides between the wide and churn layouts.
pub fn use_full_tree_table(rows: &BTreeMap<String, LocLangStats>) -> bool {
    rows.get(AGG_TOTAL).is_some_and(|row| row.loc > 0)
}

/// 🔧️ Turns per-contributor cumulative pairs into per-contributor tables with zero tree LOC.
pub fn by_contributors_to_snapshot(source: &CumulativeByContributor, languages: &[String], branch_wip_denominator: i64) -> BTreeMap<String, BTreeMap<String, LocLangStats>> {
    let zero: BTreeMap<String, i64> = make_numstat_lang_set(languages).into_iter().map(|bucket| (bucket, 0)).collect();
    source.iter().map(|(alias, pairs)| (alias.clone(), compose_loc_report_snapshot(pairs, Some(&zero), languages, branch_wip_denominator))).collect()
}

/// 📉️ Percent change in tree LOC against the previous history row; `0 -> n` is defined as 100 %.
pub fn pct_loc_since_prev(previous: i64, current: i64) -> f64 {
    let value = match (previous, current) {
        (0, 0) => 0.0,
        (0, _) => 100.0,
        _ => 100.0 * (current - previous) as f64 / previous as f64,
    };
    (100.0 * value).round() / 100.0
}

/// 📉️ The language rows of one history step: the merged tree, else the single contributor row.
pub fn history_entry_stats(entry: &LocHistoryEntry) -> Option<&BTreeMap<String, LocLangStats>> {
    if let Some(languages) = entry.languages.as_ref() {
        if !languages.is_empty() {
            return Some(languages);
        }
    }
    entry.by_contributors.as_ref().and_then(|rows| rows.values().next())
}

/// 📉️ Stamps every history row's `since_prev_loc_percent`; the first row keeps `None`.
pub fn apply_history_loc_since_prev(history: &mut [LocHistoryEntry]) {
    let mut previous: Option<BTreeMap<String, i64>> = None;
    for entry in history.iter_mut() {
        let rows = match (entry.languages.as_mut(), entry.by_contributors.as_mut()) {
            (Some(languages), _) if !languages.is_empty() => languages,
            (_, Some(by_contributor)) => match by_contributor.values_mut().next() {
                Some(rows) => rows,
                None => continue,
            },
            _ => continue,
        };
        if let Some(prior) = previous.as_ref() {
            for (name, row) in rows.iter_mut() {
                row.since_prev_loc_percent = Some(pct_loc_since_prev(prior.get(name).copied().unwrap_or(0), row.loc));
            }
        } else {
            for row in rows.values_mut() {
                row.since_prev_loc_percent = None;
            }
        }
        previous = Some(rows.iter().map(|(name, row)| (name.clone(), row.loc)).collect());
    }
}

/// 📉️ Maps the common spellings of the development branch onto its emoji id.
pub fn display_history_branch(git_ref: &str) -> String {
    let trimmed = git_ref.trim();
    if trimmed.is_empty() {
        return trimmed.to_string();
    }
    let short = trimmed.strip_prefix("refs/heads/").unwrap_or(trimmed);
    if short.eq_ignore_ascii_case("wip") || short == DEFAULT_BRANCH {
        return DEFAULT_BRANCH.to_string();
    }
    trimmed.to_string()
}

/// 📉️ The short checkpoint id of a commit, matching the repository tree's checkpoint nodes.
pub fn history_checkpoint_label(sha: &str) -> String {
    let trimmed = sha.trim();
    let short: String = trimmed.chars().take(7).collect();
    format!("🔀️{short}")
}

/// 📉️ The contributor entity id of an alias, matching the repository tree's contributor nodes.
pub fn contributor_emoji_id(alias: &str) -> String {
    format!("🧑️‍💻️{}", alias.trim().to_lowercase().replace(' ', ""))
}

//#endregion 📊️Report

//#region 🕰️Time

/// 🕰️ The granularity a commit stream is grouped at.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TimeBucket {
    /// 🔀️ No grouping: one bucket per commit, keyed by its RFC 3339 UTC timestamp.
    Commit,
    Hour,
    Day,
    /// 📆️ ISO-8601 weeks, Monday-started, keyed `YYYY-Www`.
    Week,
    Month,
    Year,
}

/// 🗓️ `(year, month, day)` of a Unix second in UTC, by the proleptic Gregorian civil algorithm.
pub fn civil_from_unix(unix: i64) -> (i64, u32, u32) {
    let days = unix.div_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_prime + 2) / 5 + 1) as u32;
    let month = if month_prime < 10 { month_prime + 3 } else { month_prime - 9 } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// 🗓️ Days since the Unix epoch for a civil UTC date, the inverse of [`civil_from_unix`].
pub fn unix_days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year.rem_euclid(400);
    let month_prime = if month > 2 { month - 3 } else { month + 9 } as i64;
    let day_of_year = (153 * month_prime + 2) / 5 + day as i64 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// 🗓️ ISO weekday of a Unix second, Monday = 1 … Sunday = 7.
pub fn iso_weekday(unix: i64) -> i64 {
    unix.div_euclid(86_400).rem_euclid(7) + 4
}

/// ⏱️ RFC 3339 UTC rendering of a Unix second, exactly what the history rows carry.
pub fn format_rfc3339_utc(unix: i64) -> String {
    let (year, month, day) = civil_from_unix(unix);
    let seconds_of_day = unix.rem_euclid(86_400);
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z", seconds_of_day / 3600, (seconds_of_day % 3600) / 60, seconds_of_day % 60)
}

/// 🕰️ The bucket key of a Unix second at one granularity.
pub fn bucket_key(unix: i64, bucket: TimeBucket) -> String {
    let (year, month, day) = civil_from_unix(unix);
    match bucket {
        TimeBucket::Commit => format_rfc3339_utc(unix),
        TimeBucket::Hour => format!("{year:04}-{month:02}-{day:02}T{:02}Z", unix.rem_euclid(86_400) / 3600),
        TimeBucket::Day => format!("{year:04}-{month:02}-{day:02}"),
        TimeBucket::Week => {
            let weekday = ((iso_weekday(unix) - 1) % 7) + 1;
            let thursday = unix.div_euclid(86_400) - (weekday - 1) + 3;
            let (iso_year, _, _) = civil_from_unix(thursday * 86_400);
            let january_first = unix_days_from_civil(iso_year, 1, 1);
            format!("{iso_year:04}-W{:02}", (thursday - january_first) / 7 + 1)
        }
        TimeBucket::Month => format!("{year:04}-{month:02}"),
        TimeBucket::Year => format!("{year:04}"),
    }
}

/// 🕰️ The first Unix second of the bucket a timestamp falls in.
pub fn bucket_start(unix: i64, bucket: TimeBucket) -> i64 {
    let (year, month, _) = civil_from_unix(unix);
    match bucket {
        TimeBucket::Commit => unix,
        TimeBucket::Hour => unix - unix.rem_euclid(3600),
        TimeBucket::Day => unix.div_euclid(86_400) * 86_400,
        TimeBucket::Week => {
            let weekday = ((iso_weekday(unix) - 1) % 7) + 1;
            (unix.div_euclid(86_400) - (weekday - 1)) * 86_400
        }
        TimeBucket::Month => unix_days_from_civil(year, month, 1) * 86_400,
        TimeBucket::Year => unix_days_from_civil(year, 1, 1) * 86_400,
    }
}

/// 🕰️ One time bucket: its key, its first second, the commits in it and their summed deltas.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TimeBucketGroup {
    pub key: String,
    pub start_unix: i64,
    pub commits: Vec<String>,
    pub delta: BTreeMap<String, LineMetrics>,
}

/// 🕰️ Groups a commit stream into ordered time buckets, summing every weighted bucket delta.
pub fn bucket_commits(commits: &[CommitDelta], bucket: TimeBucket) -> Vec<TimeBucketGroup> {
    let mut order: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, TimeBucketGroup> = BTreeMap::new();
    for commit in commits {
        let key = bucket_key(commit.when_unix, bucket);
        let group = groups.entry(key.clone()).or_insert_with(|| {
            order.push(key.clone());
            TimeBucketGroup { key: key.clone(), start_unix: bucket_start(commit.when_unix, bucket), commits: Vec::new(), delta: BTreeMap::new() }
        });
        group.commits.push(commit.sha.clone());
        for (name, pair) in &commit.delta {
            let total = group.delta.entry(name.clone()).or_default();
            total.added += pair.added;
            total.removed += pair.removed;
        }
    }
    order.into_iter().filter_map(|key| groups.remove(&key)).collect()
}

//#endregion 🕰️Time

//#region 🔌️Git Source

/// 🔌️ The only door this crate has to git. Everything else is a pure function over its output.
pub trait GitLogSource: Sync {
    /// 📜️ The raw `git log --numstat` stream for a ref (`""` meaning `HEAD`).
    fn numstat_log(&self, git_ref: &str) -> Result<String, String>;
    /// 🧾️ The repository-relative tracked paths at a ref (`""` meaning the working tree).
    fn tracked_paths(&self, git_ref: &str) -> Result<Vec<String>, String>;
    /// 📄️ The bytes of one tracked path at a ref (`""` meaning the working tree).
    fn tracked_bytes(&self, git_ref: &str, rel: &str) -> Result<Vec<u8>, String>;
}

/// 🖥️ The real git, reached through `std::process`.
#[derive(Clone, Debug)]
pub struct SystemGit {
    repo: PathBuf,
}

impl SystemGit {
    /// 🖥️ Binds the source to one repository root.
    pub fn new(repo: impl AsRef<Path>) -> Self {
        SystemGit { repo: repo.as_ref().to_path_buf() }
    }

    /// 🖥️ Runs git in the bound repository and returns stdout, or the trimmed stderr as an error.
    fn run(&self, args: &[String]) -> Result<Vec<u8>, String> {
        let output = Command::new("git").args(args).current_dir(&self.repo).output().map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!("git {}: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim()));
        }
        Ok(output.stdout)
    }
}

impl SystemGit {
    /// 🧵️ The first-parent chain of a ref, newest commit first, or nothing when git cannot name one.
    fn first_parent_chain(&self, git_ref: &str) -> Vec<String> {
        let mut args = vec!["rev-list".to_string(), "--first-parent".to_string()];
        args.push(if git_ref.trim().is_empty() { "HEAD".to_string() } else { git_ref.trim().to_string() });
        let Ok(stdout) = self.run(&args) else { return Vec::new() };
        String::from_utf8_lossy(&stdout).lines().map(str::trim).filter(|line| !line.is_empty()).map(str::to_string).collect()
    }

    /// 🧵️ How many ranges the chain is split into: enough to keep every core busy on a chain whose
    /// per-commit cost is heavily skewed towards the newest commits, and one for a short history.
    fn numstat_chunks(commits: usize) -> usize {
        if commits <= MINIMUM_CHUNKED_COMMITS {
            return 1;
        }
        let workers = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).clamp(1, MAXIMUM_NUMSTAT_WORKERS);
        (workers * NUMSTAT_CHUNKS_PER_WORKER).min(commits)
    }
}

/// 🧵️ A history shorter than this is walked in one `git log`; the split would cost more than it saves.
const MINIMUM_CHUNKED_COMMITS: usize = 32;

/// 🧵️ The most concurrent `git log` processes a numstat walk starts.
const MAXIMUM_NUMSTAT_WORKERS: usize = 16;

/// 🧵️ Ranges per worker, so a worker that draws a cheap range picks up another instead of idling.
const NUMSTAT_CHUNKS_PER_WORKER: usize = 4;

impl GitLogSource for SystemGit {
    fn numstat_log(&self, git_ref: &str) -> Result<String, String> {
        let chain = self.first_parent_chain(git_ref);
        let chunks = Self::numstat_chunks(chain.len());
        if chunks < 2 {
            return Ok(String::from_utf8_lossy(&self.run(&numstat_log_args(git_ref))?).into_owned());
        }
        let ranges = numstat_log_ranges(&chain, chunks);
        let outputs: Vec<Mutex<Option<Result<String, String>>>> = ranges.iter().map(|_| Mutex::new(None)).collect();
        let next = AtomicUsize::new(0);
        let workers = ranges.len().min(MAXIMUM_NUMSTAT_WORKERS);
        std::thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| loop {
                    let index = next.fetch_add(1, Ordering::SeqCst);
                    let Some(range) = ranges.get(index) else { return };
                    let answer = self.run(&numstat_log_range_args(range)).map(|stdout| String::from_utf8_lossy(&stdout).into_owned());
                    *outputs[index].lock().expect("numstat chunk") = Some(answer);
                });
            }
        });
        let mut collected = Vec::with_capacity(outputs.len());
        for slot in outputs {
            collected.push(slot.into_inner().expect("numstat chunk").expect("every chunk ran")?);
        }
        Ok(join_numstat_chunks(&collected))
    }

    fn tracked_paths(&self, git_ref: &str) -> Result<Vec<String>, String> {
        let args: Vec<String> = if git_ref.trim().is_empty() {
            ["ls-files", "-z"].iter().map(|value| (*value).to_string()).collect()
        } else {
            ["ls-tree", "-r", "--name-only", "-z", git_ref].iter().map(|value| (*value).to_string()).collect()
        };
        Ok(String::from_utf8_lossy(&self.run(&args)?).split('\0').filter(|value| !value.is_empty()).map(|value| value.replace('\\', "/")).collect())
    }

    fn tracked_bytes(&self, git_ref: &str, rel: &str) -> Result<Vec<u8>, String> {
        if git_ref.trim().is_empty() {
            return std::fs::read(self.repo.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR))).map_err(|error| error.to_string());
        }
        self.run(&["show".to_string(), format!("{git_ref}:{rel}")])
    }
}

/// 🎞️ A recorded git session: exactly what the real CLI answered, keyed by ref.
///
/// A transcript is what makes every metrics test hermetic and cross-language: the same JSON file
/// drives the Rust subject, the Go subject and the TypeScript oracle, and the oracle additionally
/// replays the recipe against the real `git` binary to prove the recording is still faithful.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitTranscript {
    /// 🆔️ The transcript's own identifier, used in test projections.
    #[serde(default)]
    pub id: String,
    /// 📜️ `git log --numstat` stdout keyed by the ref it was recorded for (`""` = `HEAD`).
    #[serde(default)]
    pub logs: BTreeMap<String, String>,
    /// 🧾️ Tracked paths keyed by ref.
    #[serde(default)]
    pub tracked: BTreeMap<String, Vec<String>>,
    /// 📄️ File bodies keyed by `"<ref>\u{1}<path>"`, stored as UTF-8 text.
    #[serde(default)]
    pub blobs: BTreeMap<String, String>,
}

impl GitTranscript {
    /// 🔑️ The blob key for a `(ref, path)` pair.
    pub fn blob_key(git_ref: &str, rel: &str) -> String {
        format!("{git_ref}\u{1}{rel}")
    }

    /// 📥️ Parses a transcript from its JSON encoding.
    pub fn from_json(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|error| error.to_string())
    }
}

impl GitLogSource for GitTranscript {
    fn numstat_log(&self, git_ref: &str) -> Result<String, String> {
        self.logs.get(git_ref).cloned().ok_or_else(|| format!("transcript has no log for ref {git_ref:?}"))
    }

    fn tracked_paths(&self, git_ref: &str) -> Result<Vec<String>, String> {
        self.tracked.get(git_ref).cloned().ok_or_else(|| format!("transcript has no tracked path list for ref {git_ref:?}"))
    }

    fn tracked_bytes(&self, git_ref: &str, rel: &str) -> Result<Vec<u8>, String> {
        self.blobs.get(&GitTranscript::blob_key(git_ref, rel)).map(|body| body.as_bytes().to_vec()).ok_or_else(|| format!("transcript has no blob for {git_ref:?}:{rel}"))
    }
}

//#endregion 🔌️Git Source

//#region 🧮️Pipeline

/// 🧮️ Walks the tracked tree at a ref and sums unified LOC per `loc` bucket.
pub fn snapshot_loc_counts(source: &dyn GitLogSource, git_ref: &str, languages: &[String], ignore: Option<&GitIgnore>) -> Result<BTreeMap<String, i64>, String> {
    let weights = make_numstat_lang_set(languages);
    let mut counts: BTreeMap<String, i64> = weights.iter().map(|bucket| (bucket.clone(), 0)).collect();
    let considered: Vec<(String, String)> = source
        .tracked_paths(git_ref)?
        .into_iter()
        .filter(|rel| !path_skipped_for_loc(rel, ignore))
        .filter_map(|rel| classify_for_numstat(&rel, &weights).map(|bucket| (rel, bucket)))
        .collect();
    let next = AtomicUsize::new(0);
    let partials: Vec<Mutex<BTreeMap<String, i64>>> = (0..snapshot_scan_workers(considered.len())).map(|_| Mutex::new(BTreeMap::new())).collect();
    std::thread::scope(|scope| {
        for partial in &partials {
            scope.spawn(|| {
                let mut local: BTreeMap<String, i64> = BTreeMap::new();
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some((rel, bucket)) = considered.get(index) else { break };
                    let Ok(data) = source.tracked_bytes(git_ref, rel) else { continue };
                    if data.contains(&0) {
                        continue;
                    }
                    *local.entry(bucket.clone()).or_insert(0) += count_unified_loc_for_file(rel, &data) as i64;
                }
                *partial.lock().expect("snapshot partial") = local;
            });
        }
    });
    for partial in partials {
        for (bucket, lines) in partial.into_inner().expect("snapshot partial") {
            *counts.entry(bucket).or_insert(0) += lines;
        }
    }
    Ok(counts)
}

/// 🧵️ How many threads read the tracked tree: the scan is one file read per entry and nothing else,
/// so it is bounded by the disk rather than by the host's cores.
fn snapshot_scan_workers(files: usize) -> usize {
    if files < 64 {
        return 1;
    }
    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).clamp(1, MAXIMUM_NUMSTAT_WORKERS)
}

/// 📅️ Assembles the per-commit history series, scanning the tree at every commit.
pub fn build_history(source: &dyn GitLogSource, languages: &[String], by_contributor: bool, commits: &[CommitDelta], contributor_filter: &str, ignore: Option<&GitIgnore>, alias: &dyn Fn(&str, &str) -> String) -> Result<Vec<LocHistoryEntry>, String> {
    let weights = make_numstat_lang_set(languages);
    let mut running: Cumulative = weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect();
    let mut running_branch: Cumulative = running.clone();
    let mut running_by_contributor: CumulativeByContributor = BTreeMap::new();
    let mut history: Vec<LocHistoryEntry> = Vec::new();
    let filter = contributor_filter.trim();
    for commit in commits {
        for (bucket, pair) in &commit.delta {
            if !weights.contains(bucket) {
                continue;
            }
            let branch = running_branch.entry(bucket.clone()).or_default();
            branch.added += pair.added;
            branch.removed += pair.removed;
        }
        let who = alias(&commit.author, &commit.author_mail);
        if !filter.is_empty() && !filter.eq_ignore_ascii_case(&who) {
            continue;
        }
        if by_contributor {
            running_by_contributor.entry(who.clone()).or_insert_with(|| weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect());
        }
        for (bucket, pair) in &commit.delta {
            if !weights.contains(bucket) {
                continue;
            }
            let total = running.entry(bucket.clone()).or_default();
            total.added += pair.added;
            total.removed += pair.removed;
            if by_contributor {
                let row = running_by_contributor.entry(who.clone()).or_default().entry(bucket.clone()).or_default();
                row.added += pair.added;
                row.removed += pair.removed;
            }
        }
        let scan = snapshot_loc_counts(source, &commit.sha, languages, ignore)?;
        let wip_denominator = sum_edited_pairs(&running_branch, &weights);
        let mut entry = LocHistoryEntry { sha: commit.sha.clone(), date: format_rfc3339_utc(commit.when_unix), author: who.clone(), ..LocHistoryEntry::default() };
        if by_contributor {
            let rows = compose_loc_report_snapshot(running_by_contributor.get(&who).unwrap_or(&BTreeMap::new()), Some(&scan), languages, wip_denominator);
            entry.by_contributors = Some(BTreeMap::from([(who.clone(), rows)]));
        } else {
            entry.languages = Some(compose_loc_report_snapshot(&running, Some(&scan), languages, wip_denominator));
        }
        history.push(entry);
    }
    Ok(history)
}

/// 🎛️ Everything a `loc` run decides before any git call happens.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocOptions {
    /// 🗣️ The enabled code languages; markup and data are always counted when matched.
    pub languages: Vec<String>,
    /// 📅️ Whether to assemble the per-commit history series.
    pub history: bool,
    /// 👥️ Whether to break the cumulative deltas down by first author.
    pub by_contributors: bool,
    /// ⛳️ The ref the history walk logs; empty means `HEAD` and no branch label.
    pub branch: String,
    /// 👤️ Restricts deltas and history rows to one contributor alias, case-insensitively.
    pub contributor: String,
}

impl Default for LocOptions {
    fn default() -> Self {
        LocOptions { languages: DEFAULT_CODE_LANGUAGES.iter().map(|value| (*value).to_string()).collect(), history: false, by_contributors: false, branch: String::new(), contributor: String::new() }
    }
}

/// 📊️ The whole `loc` pipeline: log, fold, scan, compose — with git behind [`GitLogSource`].
pub fn build_loc_report(source: &dyn GitLogSource, options: &LocOptions, ignore: Option<&GitIgnore>, alias: &dyn Fn(&str, &str) -> String) -> Result<LocReport, String> {
    let weights = make_numstat_lang_set(&options.languages);
    let log_ref = if options.history { if options.branch.trim().is_empty() { DEFAULT_BRANCH.to_string() } else { options.branch.trim().to_string() } } else { String::new() };
    let commits = parse_numstat_log(&source.numstat_log(&log_ref)?, &weights, ignore);
    let (cumulative_all, by_contributor) = cumulative_from_raw(&commits, &options.languages, options.by_contributors, "", alias);
    let filter = options.contributor.trim();
    let cumulative_for_snapshot = if filter.is_empty() { cumulative_all.clone() } else { cumulative_from_raw(&commits, &options.languages, false, filter, alias).0 };
    let scan = snapshot_loc_counts(source, "", &options.languages, ignore)?;
    let wip_denominator = sum_edited_pairs(&cumulative_all, &weights);
    let mut report = LocReport { snapshot: compose_loc_report_snapshot(&cumulative_for_snapshot, Some(&scan), &options.languages, wip_denominator), branch: log_ref, ..LocReport::default() };
    if options.by_contributors {
        let selected = by_contributor.map(|rows| if filter.is_empty() { rows } else { rows.into_iter().filter(|(alias, _)| alias.eq_ignore_ascii_case(filter)).collect() });
        report.by_contributors = selected.map(|rows| by_contributors_to_snapshot(&rows, &options.languages, wip_denominator));
    }
    if options.history {
        let mut history = build_history(source, &options.languages, options.by_contributors, &commits, filter, ignore, alias)?;
        apply_history_loc_since_prev(&mut history);
        report.history = Some(history);
    }
    Ok(report)
}

//#endregion 🧮️Pipeline

//#region 📤️Render

/// 📤️ A GitHub-flavoured pipe table; `since_prev` adds the Δ % column of a history row.
pub fn markdown_table(title: &str, rows: &BTreeMap<String, LocLangStats>, full_tree: bool, since_prev: bool) -> String {
    let names = if full_tree { sorted_row_keys(rows) } else { sorted_row_keys_churn(rows) };
    let mut out = String::new();
    if !title.is_empty() {
        out.push_str(&format!("### {title}\n\n"));
    }
    if full_tree && since_prev {
        out.push_str("| Category | loc | Δ% | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    } else if full_tree {
        out.push_str("| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    } else {
        out.push_str("| Category | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    }
    for name in names {
        let row = rows.get(&name).cloned().unwrap_or_default();
        if full_tree && since_prev {
            let delta = row.since_prev_loc_percent.map_or_else(|| "—".to_string(), |value| format!("{value:+.2}%"));
            out.push_str(&format!("| {name} | {} | {delta} | {:.2}% | {:.2}% | {} | {} | {} |\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else if full_tree {
            out.push_str(&format!("| {name} | {} | {:.2}% | {:.2}% | {} | {} | {} |\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else {
            out.push_str(&format!("| {name} | {:.2}% | {} | {} | {} |\n", row.wip_percent, row.edited, row.added, row.removed));
        }
    }
    out
}

/// 📤️ The whole report as GitHub-flavoured markdown.
pub fn render_markdown(report: &LocReport, with_history: bool, by_contributor: bool) -> String {
    let mut out = String::from("## LOC\n");
    out.push_str(&markdown_table("Snapshot", &report.snapshot, use_full_tree_table(&report.snapshot), false));
    out.push('\n');
    if by_contributor {
        if let Some(rows) = report.by_contributors.as_ref().filter(|rows| !rows.is_empty()) {
            out.push_str("\n## By contributor\n");
            for (alias, table) in rows {
                out.push_str(&format!("\n### {}\n\n", contributor_emoji_id(alias)));
                out.push_str(&markdown_table("", table, use_full_tree_table(table), false));
                out.push('\n');
            }
        }
    }
    if with_history {
        if let Some(history) = report.history.as_ref().filter(|entries| !entries.is_empty()) {
            out.push_str(&format!("\n## History ( {} )\n", display_history_branch(&report.branch)));
            for entry in history {
                if by_contributor {
                    out.push_str(&format!("\n#### {}  {}  {}\n", history_checkpoint_label(&entry.sha), entry.date, contributor_emoji_id(&entry.author)));
                    for (alias, table) in entry.by_contributors.iter().flatten() {
                        out.push_str(&format!("\n- **{}**\n\n", contributor_emoji_id(alias)));
                        out.push_str(&markdown_table("", table, use_full_tree_table(table), true));
                        out.push('\n');
                    }
                } else if let Some(table) = entry.languages.as_ref() {
                    out.push_str(&format!("\n#### {}  {}\n\n", history_checkpoint_label(&entry.sha), entry.date));
                    out.push_str(&markdown_table("", table, use_full_tree_table(table), true));
                    out.push('\n');
                }
            }
        }
    }
    out
}

/// 📤️ A fixed-width plain-text table; `since_prev` adds the Δ % column of a history row.
pub fn text_table(title: &str, rows: &BTreeMap<String, LocLangStats>, full_tree: bool, since_prev: bool) -> String {
    let names = if full_tree { sorted_row_keys(rows) } else { sorted_row_keys_churn(rows) };
    let mut out = String::new();
    if !title.is_empty() && !title.starts_with("  ") {
        out.push_str(&format!("{title}\n"));
    }
    if names.is_empty() {
        return out;
    }
    if full_tree && since_prev {
        out.push_str(&format!("{:<14}{:>8}{:>8}{:>7}{:>7}{:>8}{:>8}{:>8}\n", "Category", "loc", "Δ%", "%", "wip", "edited", "added", "removed"));
    } else if full_tree {
        out.push_str(&format!("{:<14}{:>8}{:>7}{:>7}{:>8}{:>8}{:>8}\n", "Category", "loc", "%", "wip", "edited", "added", "removed"));
    } else {
        out.push_str(&format!("{:<14}{:>7}{:>8}{:>8}{:>8}\n", "Category", "wip", "edited", "added", "removed"));
    }
    for name in names {
        let row = rows.get(&name).cloned().unwrap_or_default();
        if full_tree && since_prev {
            let delta = row.since_prev_loc_percent.map_or_else(|| "    —".to_string(), |value| format!("{value:+6.1}%"));
            out.push_str(&format!("{name:<14}{:>8}{delta:>8}{:>6.1}%{:>6.1}%{:>8}{:>8}{:>8}\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else if full_tree {
            out.push_str(&format!("{name:<14}{:>8}{:>6.1}%{:>6.1}%{:>8}{:>8}{:>8}\n", row.loc, row.percent, row.wip_percent, row.edited, row.added, row.removed));
        } else {
            out.push_str(&format!("{name:<14}{:>6.1}%{:>8}{:>8}{:>8}\n", row.wip_percent, row.edited, row.added, row.removed));
        }
    }
    out
}

/// 📤️ The whole report as plain text, without any terminal colour.
pub fn render_text(report: &LocReport, with_history: bool, by_contributor: bool) -> String {
    let mut out = text_table("Snapshot", &report.snapshot, use_full_tree_table(&report.snapshot), false);
    if by_contributor {
        for (alias, table) in report.by_contributors.iter().flatten() {
            out.push_str(&format!("\nContributor: {}\n", contributor_emoji_id(alias)));
            out.push_str(&text_table("", table, use_full_tree_table(table), false));
        }
    }
    if with_history {
        if let Some(history) = report.history.as_ref().filter(|entries| !entries.is_empty()) {
            out.push_str(&format!("\nHistory: {}\n", display_history_branch(&report.branch)));
            for entry in history {
                if by_contributor {
                    out.push_str(&format!("{}  {} {}\n", history_checkpoint_label(&entry.sha), entry.date, contributor_emoji_id(&entry.author)));
                    for (alias, table) in entry.by_contributors.iter().flatten() {
                        out.push_str(&format!("  {}\n", contributor_emoji_id(alias)));
                        out.push_str(&text_table("  ", table, use_full_tree_table(table), true));
                    }
                } else if let Some(table) = entry.languages.as_ref() {
                    out.push_str(&format!("{}  {}\n", history_checkpoint_label(&entry.sha), entry.date));
                    out.push_str(&text_table("", table, use_full_tree_table(table), true));
                }
            }
        }
    }
    out
}

//#endregion 📤️Render

//#region ⏱️Benchmark

/// ⏱️ The ecosystems a benchmark sweep reports, in report-column order.
pub const BENCHMARK_LANGUAGES: [&str; 5] = ["Typescript", "Python", "Go", "C#", "Rust"];

/// 🔶️ One timing a benchmark run reported: which case, in which ecosystem, how long.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchmarkResult {
    #[serde(rename = "test")]
    pub test: String,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "time")]
    pub time: String,
}

/// 🔬️ Extracts `name,time` timing lines from one ecosystem's benchmark stdout.
///
/// A benchmark process also prints warnings and paths, so only two-field lines whose first field
/// carries no `warning`, no `:` and no path separator are timings.
pub fn parse_benchmark_output(lang: &str, output: &str) -> Vec<BenchmarkResult> {
    output
        .split('\n')
        .filter_map(|raw| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return None;
            }
            let parts: Vec<&str> = trimmed.split(',').collect();
            if parts.len() != 2 || parts[0].contains("warning") || parts[0].contains(':') || parts[0].contains('/') || parts[0].contains('\\') {
                return None;
            }
            Some(BenchmarkResult { test: parts[0].to_string(), lang: lang.to_string(), time: parts[1].to_string() })
        })
        .collect()
}

/// ⏱️ Parses a timing into seconds; accepts a bare number or an `s`/`ms`/`us`/`ns` suffix.
pub fn parse_duration_seconds(raw: &str) -> Option<f64> {
    let text = raw.trim();
    if text.is_empty() {
        return None;
    }
    let (number, factor) = if let Some(head) = text.strip_suffix("ms") {
        (head, 1e-3)
    } else if let Some(head) = text.strip_suffix("us").or_else(|| text.strip_suffix("µs")) {
        (head, 1e-6)
    } else if let Some(head) = text.strip_suffix("ns") {
        (head, 1e-9)
    } else if let Some(head) = text.strip_suffix('s') {
        (head, 1.0)
    } else {
        (text, 1.0)
    };
    number.trim().parse::<f64>().ok().map(|value| value * factor)
}

/// 📊️ One summary row: a case, its timing per ecosystem and the fastest ecosystem for it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchmarkRow {
    pub test: String,
    pub timings: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fastest: Option<String>,
}

/// 📊️ The whole benchmark table: the case names, the ecosystem columns and the rows.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchmarkSummary {
    pub tests: Vec<String>,
    pub languages: Vec<String>,
    pub rows: Vec<BenchmarkRow>,
}

/// 📊️ Folds raw timings into the report table; the first timing of a `(test, lang)` pair wins.
pub fn summarize_benchmarks(results: &[BenchmarkResult]) -> BenchmarkSummary {
    let tests: Vec<String> = results.iter().map(|entry| entry.test.clone()).collect::<BTreeSet<String>>().into_iter().collect();
    let languages: Vec<String> = BENCHMARK_LANGUAGES.iter().map(|value| (*value).to_string()).collect();
    let rows = tests
        .iter()
        .map(|test| {
            let mut timings: BTreeMap<String, String> = BTreeMap::new();
            for language in &languages {
                if let Some(found) = results.iter().find(|entry| entry.test == *test && entry.lang == *language) {
                    timings.insert(language.clone(), found.time.clone());
                }
            }
            let fastest = timings
                .iter()
                .filter_map(|(language, time)| parse_duration_seconds(time).map(|seconds| (language.clone(), seconds)))
                .min_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.0.cmp(&right.0)))
                .map(|(language, _)| language);
            BenchmarkRow { test: test.clone(), timings, fastest }
        })
        .collect();
    BenchmarkSummary { tests, languages, rows }
}

/// ✏️ The benchmark report as CSV, one row per case and one column per ecosystem.
pub fn benchmark_csv(results: &[BenchmarkResult]) -> String {
    let summary = summarize_benchmarks(results);
    let mut out = String::new();
    out.push_str("Test,");
    out.push_str(&summary.languages.join(","));
    out.push('\n');
    for row in &summary.rows {
        out.push_str(&row.test);
        for language in &summary.languages {
            out.push(',');
            out.push_str(row.timings.get(language).map_or("", String::as_str));
        }
        out.push('\n');
    }
    out
}

//#endregion ⏱️Benchmark
