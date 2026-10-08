//! 📝️ Text representation codec surface for `stdio.semio.text` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");



#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::text::schema::diff::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_opt, enc_indexed_triple, enc_opt};
use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextMark;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_text_diff(d: &SemioTextDiff) -> String {
    match &d.runs {
        Some(runs) => format!("runs={}", enc_runs(runs)),
        None => String::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_text_diff(line: &str) -> Result<SemioTextDiff, String> {
    if line.is_empty() {
        return Ok(SemioTextDiff::default());
    }
    let rest = line.strip_prefix("runs=").ok_or_else(|| format!("text diff: unknown token {line:?}"))?;
    Ok(SemioTextDiff { runs: Some(dec_runs(rest)?) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_runs(runs: &IndexedTripleDiff<SemioTextRunDiff, SemioTextRun>) -> String {
    format!("[{}]", enc_indexed_triple(runs, enc_run_diff, enc_run))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_runs(s: &str) -> Result<IndexedTripleDiff<SemioTextRunDiff, SemioTextRun>, String> {
    dec_indexed_triple(strip_brackets(s)?, dec_run_diff, dec_run)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run_diff(d: &SemioTextRunDiff) -> String {
    let marks = enc_opt(d.marks.as_ref(), |marks| enc_indexed_triple(marks, |m| enc_mark(&m.value), enc_mark));
    format!("[{},{},{}]", enc_opt(d.language.as_ref(), |v| enc_str(v)), enc_opt(d.content.as_ref(), |v| enc_str(v)), marks)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run_diff(s: &str) -> Result<SemioTextRunDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [language, content, marks] = parts.as_slice() else { return Err(format!("run diff: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTextRunDiff {
        language: dec_opt(language, dec_str)?,
        content: dec_opt(content, dec_str)?,
        marks: dec_opt(marks, |triple| dec_indexed_triple(triple, |m| dec_mark(m).map(|value| Replace { value }), dec_mark))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run(r: &SemioTextRun) -> String {
    let marks = r.marks.iter().map(enc_mark).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}]]", enc_str(&r.language), enc_str(&r.content), marks)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run(s: &str) -> Result<SemioTextRun, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [language, content, marks] = parts.as_slice() else { return Err(format!("run: expected 3 fields, got {}", parts.len())) };
    let marks = split_top_level(strip_brackets(marks)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_mark).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTextRun { language: dec_str(language)?, content: dec_str(content)?, marks })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mark(m: &SemioTextMark) -> String {
    format!("[{},{}]", enc_mark_kind(m.kind), enc_str(&m.href))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mark(s: &str) -> Result<SemioTextMark, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, href] = parts.as_slice() else { return Err(format!("mark: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTextMark { kind: dec_mark_kind(kind)?, href: dec_str(href)? })
}

/// 🧪️ Hand-rolled `protocol::DiffCodec` — `text`'s single collection field prints as
/// `runs=[<run>,...]` (empty string = no-op diff), reusing the snapshot facet's own real
/// hex/bracket run/mark encoders (duplicated locally, same convention every sibling subset's
/// `🔺️diff` facet already establishes — see that facet's own doc comment for why).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mark_kind(k: crate::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind) -> char {
    crate::standards::v1::subsets::text::io::text::snapshot::enc_mark_kind(k)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mark_kind(s: &str) -> Result<crate::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind, String> {
    crate::standards::v1::subsets::text::io::text::snapshot::dec_mark_kind(s)
}

impl protocol::DiffText for SemioTextDiff {
fn print_diff(&self) -> String {
    print_text_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_text_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
