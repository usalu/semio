//! 🔺️ SemioTextDiff — sparse per-run diff over `SemioTextSnapshot`. `text` has exactly one mutable field (`runs`, an
//! intrinsically ordered, anonymous collection with no stable id per `📓️taxonomy.md`'s addressing rule #3), so the diff
//! carries a single index-keyed `runs` triple: removed base indices, modified rows (a sparse [`SemioTextRunDiff`] per run) and
//! added rows with their final position. Every row-local change (content, language, one mark) is its own sparse field, never a
//! rebuilt list. No `snapshot: Option<SemioTextSnapshot>` full-replace slot anywhere — whole-document replace is
//! `ArtifactStore::reset`, outside history.

use crate::standards::v1::subsets::base::schema::triples::{absorb_indexed_rows, absorb_indexed_slot, apply_indexed_rows, inverse_indexed_rows, validate_indexed_triple, IndexedRow, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextRun, SemioTextSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️RunDiff
/// 🏃️ Sparse diff of one run: each present field is the new value; `marks` nests an index-keyed triple of whole-mark rows.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioTextRunDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub marks: Option<IndexedTripleDiff<Replace<SemioTextMark>, SemioTextMark>>,
}

impl IndexedRow<SemioTextRun> for SemioTextRunDiff {
    fn apply_row(&self, base: &SemioTextRun) -> SemioTextRun {
        let mut next = base.clone();
        if let Some(language) = &self.language {
            next.language = language.clone();
        }
        if let Some(content) = &self.content {
            next.content = content.clone();
        }
        if let Some(marks) = &self.marks {
            next.marks = apply_indexed_rows(marks, &base.marks);
        }
        next
    }
    fn inverse_row(&self, base: &SemioTextRun) -> Self {
        Self { language: self.language.as_ref().map(|_| base.language.clone()), content: self.content.as_ref().map(|_| base.content.clone()), marks: self.marks.as_ref().map(|marks| inverse_indexed_rows(marks, &base.marks)) }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.language.is_some() {
            self.language = other.language;
        }
        if other.content.is_some() {
            self.content = other.content;
        }
        absorb_indexed_slot(&mut self.marks, other.marks);
    }
    fn row_is_empty(&self) -> bool {
        self.language.is_none() && self.content.is_none() && self.marks.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
}
//#endregion 🔖️RunDiff

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text.diff")]
pub struct SemioTextDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<IndexedTripleDiff<SemioTextRunDiff, SemioTextRun>>,
}

impl SemioTextDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.runs.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
}

impl MutationDiff<SemioTextSnapshot> for SemioTextDiff {
    fn apply(&self, base: &SemioTextSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioTextSnapshot> {
        let mut next = base.clone();
        if let Some(runs) = &self.runs {
            validate_indexed_triple(runs, base.runs.len(), ["runs"])?;
            for modified in &runs.modified {
                let run = &base.runs[modified.index];
                if let Some(marks) = &modified.diff.marks {
                    validate_indexed_triple(marks, run.marks.len(), ["runs".to_string(), modified.index.to_string(), "marks".to_string()])?;
                }
            }
            next.runs = apply_indexed_rows(runs, &base.runs);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        absorb_indexed_slot(&mut self.runs, other.runs);
    }
}

/// 🧮️ `text`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch. `inverse` is the concrete negative diff of
/// the keyed rows; `between` is the positional sync/import delta, never used by mutation leaves.
impl protocol::command::DiffAlgebra<SemioTextSnapshot> for SemioTextDiff {
    fn inverse(&self, base: &SemioTextSnapshot) -> Self {
        SemioTextDiff { runs: self.runs.as_ref().map(|runs| inverse_indexed_rows(runs, &base.runs)) }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec





















//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioTextDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioTextDiff> {
    use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified};
    use crate::standards::v1::subsets::text::schema::snapshot::{demo_text_snapshot, SemioTextMarkKind};
    let bold = SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() };
    vec![
        SemioTextDiff::default(),
        SemioTextDiff { runs: Some(IndexedTripleDiff { added: demo_text_snapshot().runs.into_iter().enumerate().map(|(index, item)| IndexAdded { index, item }).collect(), ..Default::default() }) },
        SemioTextDiff { runs: Some(IndexedTripleDiff { removed: vec![0], ..Default::default() }) },
        SemioTextDiff {
            runs: Some(IndexedTripleDiff {
                modified: vec![IndexModified { index: 1, diff: SemioTextRunDiff { language: Some("fr".into()), content: Some("bonjour".into()), marks: Some(IndexedTripleDiff { removed: vec![0], added: vec![IndexAdded { index: 0, item: bold }], ..Default::default() }) } }],
                ..Default::default()
            }),
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests




























