//! ⚡️ Semio text artifact — hand-rolled `OpText` for `SemioTextMutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text codec stays handcrafted here, one
//! keyword per semantic verb, grammar `keyword:arg1,arg2,...` (`✳️image`'s own hex/bracket-encoded
//! value convention, reused so this facet's grammar can lean on the shared `hex` macro instead of
//! a quoted-string production).

pub use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::{
    add_mark::mutation::AddMark, change_run_language::mutation::ChangeRunLanguage, edit_run::mutation::EditRun, insert_run::mutation::InsertRun, remove_mark::mutation::RemoveMark, remove_run::mutation::RemoveRun, reorder_runs::mutation::ReorderRuns,
};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

async fn enc_mark_kind(k: SemioTextMarkKind) -> char {
    match k {
        SemioTextMarkKind::Bold => 'b',
        SemioTextMarkKind::Italic => 'i',
        SemioTextMarkKind::Code => 'c',
        SemioTextMarkKind::Link => 'l',
    }
}
async fn dec_mark_kind(s: &str) -> Result<SemioTextMarkKind, String> {
    match s {
        "b" => Ok(SemioTextMarkKind::Bold),
        "i" => Ok(SemioTextMarkKind::Italic),
        "c" => Ok(SemioTextMarkKind::Code),
        "l" => Ok(SemioTextMarkKind::Link),
        other => Err(format!("bad mark kind {other:?}")),
    }
}
async fn enc_mark(m: &SemioTextMark) -> String {
    format!("[{},{}]", enc_mark_kind(m.kind), enc_str(&m.href))
}
async fn dec_mark(s: &str) -> Result<SemioTextMark, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [kind, href] = parts.as_slice() else { return Err(format!("mark: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTextMark { kind: dec_mark_kind(kind)?, href: dec_str(href)? })
}
async fn enc_run(r: &SemioTextRun) -> String {
    let marks = r.marks.iter().map(enc_mark).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}]]", enc_str(&r.language), enc_str(&r.content), marks)
}
async fn dec_run(s: &str) -> Result<SemioTextRun, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [language, content, marks] = parts.as_slice() else { return Err(format!("run: expected 3 fields, got {}", parts.len())) };
    let marks = split_top_level(strip_brackets(marks)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_mark).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioTextRun { language: dec_str(language)?, content: dec_str(content)?, marks })
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
async fn print_text_mutation(m: &SemioTextMutation) -> String {
    match m {
        SemioTextMutation::InsertRun(p) => format!("insertRun:{},{}", p.index, enc_run(&p.run)),
        SemioTextMutation::RemoveRun(p) => format!("removeRun:{}", p.index),
        SemioTextMutation::EditRun(p) => format!("editRun:{},{}", p.index, enc_str(&p.new_content)),
        SemioTextMutation::ChangeRunLanguage(p) => format!("changeRunLanguage:{},{}", p.index, enc_str(&p.new_language)),
        SemioTextMutation::ReorderRuns(p) => format!("reorderRuns:{},{}", p.from, p.to),
        SemioTextMutation::AddMark(p) => format!("addMark:{},{},{}", p.run_index, p.index, enc_mark(&p.mark)),
        SemioTextMutation::RemoveMark(p) => format!("removeMark:{},{}", p.run_index, p.index),
    }
}

async fn parse_text_mutation(line: &str) -> Result<SemioTextMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("text mutation: missing ':' in {line:?}"))?;
    match tag {
        "insertRun" => {
            let (idx, run) = rest.split_once(',').ok_or_else(|| "insertRun: missing comma".to_string())?;
            Ok(SemioTextMutation::InsertRun(InsertRun { index: parse_usize(idx)?, run: dec_run(run)? }))
        }
        "removeRun" => Ok(SemioTextMutation::RemoveRun(RemoveRun { index: parse_usize(rest)? })),
        "editRun" => {
            let (idx, content) = rest.split_once(',').ok_or_else(|| "editRun: missing comma".to_string())?;
            Ok(SemioTextMutation::EditRun(EditRun { index: parse_usize(idx)?, new_content: dec_str(content)? }))
        }
        "changeRunLanguage" => {
            let (idx, lang) = rest.split_once(',').ok_or_else(|| "changeRunLanguage: missing comma".to_string())?;
            Ok(SemioTextMutation::ChangeRunLanguage(ChangeRunLanguage { index: parse_usize(idx)?, new_language: dec_str(lang)? }))
        }
        "reorderRuns" => {
            let parts = split_top_level(rest, ',');
            let [from, to] = parts.as_slice() else { return Err(format!("reorderRuns: expected 2 fields, got {}", parts.len())) };
            Ok(SemioTextMutation::ReorderRuns(ReorderRuns { from: parse_usize(from)?, to: parse_usize(to)? }))
        }
        "addMark" => {
            let parts = split_top_level(rest, ',');
            let [run_index, index, mark] = parts.as_slice() else { return Err(format!("addMark: expected 3 fields, got {}", parts.len())) };
            Ok(SemioTextMutation::AddMark(AddMark { run_index: parse_usize(run_index)?, index: parse_usize(index)?, mark: dec_mark(mark)? }))
        }
        "removeMark" => {
            let parts = split_top_level(rest, ',');
            let [run_index, index] = parts.as_slice() else { return Err(format!("removeMark: expected 2 fields, got {}", parts.len())) };
            Ok(SemioTextMutation::RemoveMark(RemoveMark { run_index: parse_usize(run_index)?, index: parse_usize(index)? }))
        }
        other => Err(format!("text mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioTextMutation {
    async fn print_op(&self) -> String {
        print_text_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_text_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for `ops_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️component.rs` and this file's own round-trip test.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<SemioTextMutation> {
    vec![
        SemioTextMutation::InsertRun(InsertRun { index: 1, run: SemioTextRun { language: "en".into(), content: "hi".into(), marks: vec![] } }),
        SemioTextMutation::RemoveRun(RemoveRun { index: 0 }),
        SemioTextMutation::EditRun(EditRun { index: 0, new_content: "greetings".into() }),
        SemioTextMutation::ChangeRunLanguage(ChangeRunLanguage { index: 0, new_language: "fr".into() }),
        SemioTextMutation::ReorderRuns(ReorderRuns { from: 0, to: 1 }),
        SemioTextMutation::AddMark(AddMark { run_index: 0, index: 0, mark: SemioTextMark { kind: SemioTextMarkKind::Link, href: "https://semio.tech".into() } }),
        SemioTextMutation::RemoveMark(RemoveMark { run_index: 1, index: 0 }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;

    #[test]
    async fn op_text_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioTextMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
        }
    }
}
//#endregion 🧪️Tests
