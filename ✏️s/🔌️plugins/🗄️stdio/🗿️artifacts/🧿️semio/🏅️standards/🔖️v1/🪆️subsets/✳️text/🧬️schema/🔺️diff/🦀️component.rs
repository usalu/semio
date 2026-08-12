//! 🔺️ SemioTextDiff — sparse per-field diff over `SemioTextSnapshot`. `text` has exactly one
//! mutable field (`runs`, an intrinsically ordered, anonymous collection with no stable id per
//! `📓️taxonomy.md`'s addressing rule #3), so the diff carries a single `runs: Option<…>` slot: a
//! whole-list-wrapper rebuilt POSITIONALLY from `base` by each mutation triad's own `🔺️diff` leaf
//! (never a generic `between()` re-derivation) — the same shape
//! `SEMANTIC-MUTATIONS-OVERHAUL`'s `din4108` facet (this ticket's binding reference,
//! `📌️important.md`'s "Authoring a 🧬️mutations facet" section) uses for its own id-less `layers`
//! collection. No `snapshot: Option<SemioTextSnapshot>` full-replace slot anywhere — whole-
//! document replace is `ArtifactStore::reset`, outside history.

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️RunList
/// 📋 Whole-list wrapper for the `runs` field diff — every mutation triad rebuilds the full
/// ordered `values` vec from `base` and wraps it here (`din4108::Din4108LayerList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SemioTextRunList {
    pub values: Vec<SemioTextRun>,
}
//#endregion 🔖️RunList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text.diff")]
pub struct SemioTextDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<SemioTextRunList>,
}

impl SemioTextDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.runs.is_none()
    }
}

impl MutationDiff<SemioTextSnapshot> for SemioTextDiff {
    fn apply(&self, base: &SemioTextSnapshot) -> SemioTextSnapshot {
        let mut next = base.clone();
        if let Some(list) = &self.runs {
            next.runs = list.values.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.runs.is_some() {
            self.runs = other.runs;
        }
    }
}

/// 🧮️ `text`'s own `DiffAlgebra` — required by the `✳️any` envelope's own dispatch (`SemioDiff`
/// delegates `between`/`inverse`/`is_empty` straight through to every wrapped subset's own impl).
/// Whole-list `between`/`inverse` are honest here (not apply-then-capture): `text` has exactly one
/// mutable field, so a change is fully described by "the new/old `runs` value", same shape every
/// mutation triad's own `🔺️diff` leaf already produces.
impl protocol::command::DiffAlgebra<SemioTextSnapshot> for SemioTextDiff {
    fn between(base: &SemioTextSnapshot, other: &SemioTextSnapshot) -> Self {
        SemioTextDiff { runs: (base.runs != other.runs).then(|| SemioTextRunList { values: other.runs.clone() }) }
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Self {
        SemioTextDiff { runs: self.runs.as_ref().map(|_| SemioTextRunList { values: base.runs.clone() }) }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `text`'s single collection field prints as
/// `runs=[<run>,...]` (empty string = no-op diff), reusing the snapshot facet's own real
/// hex/bracket run/mark encoders (duplicated locally, same convention every sibling subset's
/// `🔺️diff` facet already establishes — see that facet's own doc comment for why).
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 { return Err(format!("odd hex length: {s:?}")); }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String { hex_encode(s.as_bytes()) }
fn dec_str(s: &str) -> Result<String, String> { String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string()) }

use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextMark;

fn enc_mark_kind(k: crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind) -> char {
    crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::enc_mark_kind(k)
}
fn dec_mark_kind(s: &str) -> Result<crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind, String> {
    crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::dec_mark_kind(s)
}
fn enc_mark(m: &SemioTextMark) -> String { format!("[{},{}]", enc_mark_kind(m.kind), enc_str(&m.href)) }
fn dec_mark(s: &str) -> Result<SemioTextMark, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, href] = parts.as_slice() else { return Err(format!("mark: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTextMark { kind: dec_mark_kind(kind)?, href: dec_str(href)? })
}
fn enc_run(r: &SemioTextRun) -> String {
    let marks = r.marks.iter().map(enc_mark).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}]]", enc_str(&r.language), enc_str(&r.content), marks)
}
fn dec_run(s: &str) -> Result<SemioTextRun, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [language, content, marks] = parts.as_slice() else { return Err(format!("run: expected 3 fields, got {}", parts.len())) };
    let marks = split_top_level(strip_brackets(marks)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_mark).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTextRun { language: dec_str(language)?, content: dec_str(content)?, marks })
}
fn enc_runs(list: &SemioTextRunList) -> String {
    format!("[{}]", list.values.iter().map(enc_run).collect::<Vec<_>>().join(","))
}
fn dec_runs(s: &str) -> Result<SemioTextRunList, String> {
    let values = split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_run).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTextRunList { values })
}

fn print_text_diff(d: &SemioTextDiff) -> String {
    match &d.runs {
        Some(list) => format!("runs={}", enc_runs(list)),
        None => String::new(),
    }
}
fn parse_text_diff(line: &str) -> Result<SemioTextDiff, String> {
    if line.is_empty() {
        return Ok(SemioTextDiff::default());
    }
    let rest = line.strip_prefix("runs=").ok_or_else(|| format!("text diff: unknown token {line:?}"))?;
    Ok(SemioTextDiff { runs: Some(dec_runs(rest)?) })
}

impl protocol::DiffCodec for SemioTextDiff {
    fn print_diff(&self) -> String { print_text_diff(self) }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_text_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`runs`) are two REAL fixed
    /// fields; when present, `runs` follows as a real varint count + per-run binary encoding
    /// (reusing the snapshot facet's own `write_run`/`read_run`) rather than a text-blob-in-binary
    /// shortcut — `text`'s diff has exactly one collection field, so no opaque multi-field payload
    /// chain is needed.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::write_run;
        let presence: u8 = if self.runs.is_some() { 0b0000_0001 } else { 0 };
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(list) = &self.runs {
            store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
            for r in &list.values {
                write_run(&mut out, r);
            }
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::read_run;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let runs = if presence & 0b0000_0001 != 0 {
            let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff runs count", offset: 2, detail: e.to_string() })?;
            let mut values = Vec::with_capacity(count as usize);
            for _ in 0..count {
                values.push(read_run(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff run", offset: 2, detail: e })?);
            }
            Some(SemioTextRunList { values })
        } else {
            None
        };
        Ok(SemioTextDiff { runs })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioTextDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioTextDiff> {
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMarkKind, demo_text_snapshot};
    vec![
        SemioTextDiff::default(),
        SemioTextDiff { runs: Some(SemioTextRunList { values: demo_text_snapshot().runs }) },
        SemioTextDiff {
            runs: Some(SemioTextRunList {
                values: vec![SemioTextRun { language: "fr".into(), content: "bonjour".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Italic, href: String::new() }] }],
            }),
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMarkKind, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

    #[test]
    fn apply_replaces_runs_wholesale() {
        let base = SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: vec![SemioTextRun { language: "en".into(), content: "a".into(), marks: vec![] }] };
        let diff = SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "en".into(), content: "b".into(), marks: vec![] }] }) };
        let next = diff.apply(&base);
        assert_eq!(next.runs[0].content, "b");
    }

    #[test]
    fn absorb_last_write_wins() {
        let mut d1 = SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "en".into(), content: "a".into(), marks: vec![] }] }) };
        let d2 = SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "en".into(), content: "b".into(), marks: vec![] }] }) };
        d1.absorb(d2.clone());
        assert_eq!(d1, d2);
    }

    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioTextDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioTextDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }

    #[test]
    fn mark_kind_helper_smoke() {
        assert_eq!(dec_mark_kind("l").unwrap(), SemioTextMarkKind::Link);
    }
}
//#endregion 🔖️Tests
