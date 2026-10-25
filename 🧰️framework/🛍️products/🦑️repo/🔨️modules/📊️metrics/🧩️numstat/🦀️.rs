//! 🧩️ Numstat: the pure `git log --numstat` parser.

use super::{classify_for_numstat, path_skipped_for_loc, GitIgnore, LineMetrics};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

//#region 🧩️Numstat

/// 📄️ One file entry of a numstat commit block.
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
    /// 🗂️ Every file line of the block, skipped ones included, in stream order.
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

/// 🔪️ Splits a numstat path field into `(new path, rename source)`.
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

/// 🐙️ Breaks a `git log --numstat` stream into per-commit records with weighted bucket sums.
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
