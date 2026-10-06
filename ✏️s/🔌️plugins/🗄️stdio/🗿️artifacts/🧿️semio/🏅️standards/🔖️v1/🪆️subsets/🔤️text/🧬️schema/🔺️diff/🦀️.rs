//! 🔺️ SemioTextDiff — sparse per-field diff over `SemioTextSnapshot`. `text` has exactly one
//! mutable field (`runs`, an intrinsically ordered, anonymous collection with no stable id per
//! `📓️taxonomy.md`'s addressing rule #3), so the diff carries a single `runs: Option<…>` slot: a
//! whole-list-wrapper rebuilt POSITIONALLY from `base` by each mutation triad's own `🔺️diff` leaf
//! (never a generic `between()` re-derivation) — the same shape
//! `SEMANTIC-MUTATIONS-OVERHAUL`'s `din4108` facet (this ticket's binding reference,
//! `📌️important.md`'s "Authoring a 🧬️mutations facet" section) uses for its own id-less `layers`
//! collection. No `snapshot: Option<SemioTextSnapshot>` full-replace slot anywhere — whole-
//! document replace is `ArtifactStore::reset`, outside history.

use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️RunList
/// 📋 Whole-list wrapper for the `runs` field diff — every mutation triad rebuilds the full
/// ordered `values` vec from `base` and wraps it here (`din4108::Din4108LayerList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioTextRunList {
    pub values: Vec<SemioTextRun>,
}
//#endregion 🔖️RunList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text.diff")]
pub struct SemioTextDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<SemioTextRunList>,
}

impl SemioTextDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.runs.is_none()
    }
}

impl MutationDiff<SemioTextSnapshot> for SemioTextDiff {
    fn apply(&self, base: &SemioTextSnapshot) -> protocol::MutationApplyResult<SemioTextSnapshot> {
        let mut next = base.clone();
        if let Some(list) = &self.runs {
            next.runs = list.values.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.runs.is_some() {
            self.runs = other.runs;
        }
    }
}

/// 🧮️ `text`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch (`SemioDiff`
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







use crate::standards::v1::subsets::text::schema::snapshot::SemioTextMark;














//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioTextDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioTextDiff> {
    use crate::standards::v1::subsets::text::schema::snapshot::{demo_text_snapshot, SemioTextMarkKind};
    vec![
        SemioTextDiff::default(),
        SemioTextDiff { runs: Some(SemioTextRunList { values: demo_text_snapshot().runs }) },
        SemioTextDiff { runs: Some(SemioTextRunList { values: vec![SemioTextRun { language: "fr".into(), content: "bonjour".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Italic, href: String::new() }] }] }) },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests




























