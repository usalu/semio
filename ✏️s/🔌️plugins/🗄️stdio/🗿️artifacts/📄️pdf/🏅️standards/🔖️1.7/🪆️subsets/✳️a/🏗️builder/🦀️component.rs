//! 🏗️ PdfABuilder (1.7/✳️a) — a typed builder whose ergonomic path can only produce a
//! PDF/A-conforming `PdfSnapshot` BY CONSTRUCTION (D5 requirement #3):
//! - `new()` REQUIRES an `OutputIntent` condition identifier -- there is no way to reach a
//!   built snapshot without one via the recommended path.
//! - There is no `set_encryption`/`set_action`/`set_javascript` method anywhere on this type --
//!   the only mutating methods are `add_page`/`set_info`, the same restricted vocabulary
//!   PDF/A actually allows content-wise.
//!
//! `ArtifactBuilder` (the SDK trait every builder facet implements for generic UI/mutation
//! dispatch) still mandates a no-arg `empty()` plus the general escape hatches
//! (`from_binary`/`from_text`/arbitrary `mutate`/`absorb`) -- those exist here too, because the
//! trait requires them and other machinery (D4's `MigrateDialect`, generic mutation replay)
//! depends on every builder implementing it uniformly. What makes construction genuinely
//! conforming-only is `build()` itself: it re-runs the SAME `check_pdf_a_conformance` used by
//! `PdfAComposer`, unconditionally, regardless of which path produced the in-flight snapshot --
//! so a hard PDF/A violation can never leave this builder as an `Ok(PdfSnapshot)`, no matter
//! which method put it there.
//!
//! W2 restructure (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES): renamed from
//! `PdfA2bBuilder`/`✳️a-2b`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::a::analyzer::check_pdf_a_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfInfo, PdfIndirectObject, PdfObject, PdfPage, PdfSnapshot};

//#region 🔖️Seed
/// 🌱️ Seeds a fresh snapshot with a real `/Root /OutputIntents` → `OutputIntent` object pair
/// (`/S /GTS_PDFA1`, ISO 19005-2/-3's own conformance marker) -- a genuine, well-formed PDF/A
/// OutputIntent, not a placeholder value that merely satisfies string equality.
fn seeded_snapshot(output_intent_condition: String) -> PdfSnapshot {
    let objects = vec![
        PdfIndirectObject {
            id: ObjRef { num: 1, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
            ]),
        },
        PdfIndirectObject {
            id: ObjRef { num: 2, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFA1".into()) },
                PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(output_intent_condition.into_bytes()) },
            ]),
        },
    ];
    PdfSnapshot { objects, ..PdfSnapshot::default() }
}
//#endregion 🔖️Seed

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct PdfABuilder {
    snapshot: PdfSnapshot,
}

impl PdfABuilder {
    /// ➕ The recommended entry point: REQUIRES an OutputIntent condition identifier
    /// (e.g. `"sRGB IEC61966-2.1"`) up front -- there is no variant of `new` that omits it.
    pub fn new(output_intent_condition: impl Into<String>) -> Self {
        Self { snapshot: seeded_snapshot(output_intent_condition.into()) }
    }

    pub fn add_page(mut self, page: PdfPage) -> Self {
        let index = self.snapshot.pages.len();
        apply_pdf_mutation(&mut self.snapshot, &PdfMutation::InsertPage { index, page });
        self
    }

    pub fn set_info(mut self, info: PdfInfo) -> Self {
        apply_pdf_mutation(&mut self.snapshot, &PdfMutation::SetInfo { info });
        self
    }
}

impl ArtifactBuilder for PdfABuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;

    /// ⚠️ `ArtifactBuilder::empty()` is mandated no-arg by the SDK trait (generic UI/mutation
    /// dispatch needs every builder facet uniform) -- it falls back to a generic sRGB condition
    /// rather than omitting the OutputIntent entirely, since `build()` requires one to pass clean
    /// regardless. Prefer `PdfABuilder::new(condition)` directly wherever the real condition is
    /// known.
    fn empty() -> Self {
        Self::new("sRGB IEC61966-2.1")
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ The real construction gate: however `self.snapshot` got here (`new`+`add_page`,
    /// `from_binary`, a raw `mutate(SetSnapshot { .. })`), a hard PDF/A violation fails
    /// `build()` -- soft/info diagnostics (missing OutputIntent, non-embedded font, the detected
    /// level) pass through as advisory `Diagnostic`s; the `Err` path is NOT taken for those, only
    /// hard ones block.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_pdf_a_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_requires_output_intent_and_builds_clean() {
        let snapshot = PdfABuilder::new("sRGB IEC61966-2.1")
            .add_page(PdfPage::new(200.0, 200.0))
            .set_info(PdfInfo { title: Some("A Test".into()), ..PdfInfo::default() })
            .build()
            .expect("conforming construction must build");
        assert_eq!(snapshot.pages.len(), 1);
        assert_eq!(snapshot.info.title.as_deref(), Some("A Test"));
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let violating = PdfIndirectObject {
            id: ObjRef { num: 99, gen: 0 },
            value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) }]),
        };
        let mut snapshot = PdfABuilder::new("sRGB IEC61966-2.1").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
        snapshot.objects.push(violating);
        // Even routed back in via the generic `SetSnapshot` escape hatch, `build()` still catches it.
        let (mutated, _diff) = PdfABuilder::from_snapshot(PdfSnapshot::default()).mutate(PdfMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a /Launch action must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::a::analyzer::CODE_LAUNCH));
    }
}
