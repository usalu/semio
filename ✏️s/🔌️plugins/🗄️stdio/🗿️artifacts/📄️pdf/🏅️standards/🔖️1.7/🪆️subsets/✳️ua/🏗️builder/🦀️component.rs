//! 🏗️ PdfUaBuilder (1.7/✳️ua) — a typed builder whose ergonomic path can only produce a
//! PDF/UA-1-conforming `PdfSnapshot` BY CONSTRUCTION:
//! - `new()` REQUIRES a language tag up front, seeding `/Root/MarkInfo/Marked true`,
//!   `/StructTreeRoot`, `/Root/Lang`, and `/Root/ViewerPreferences/DisplayDocTitle true` -- there
//!   is no way to reach a built snapshot without a tagged, structured Catalog via this path.
//!
//! `build()` re-runs the SAME `check_ua_conformance` used by `PdfUaComposer`, unconditionally.
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::ua::analyzer::check_ua_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfInfo, PdfIndirectObject, PdfObject, PdfPage, PdfSnapshot};

//#region 🔖️Seed
/// 🌱️ Seeds a fresh snapshot with a real tagged-PDF Catalog: `/MarkInfo/Marked true`, a
/// (minimal) `/StructTreeRoot`, `/Lang`, and `/ViewerPreferences/DisplayDocTitle true`.
fn seeded_snapshot(lang: String) -> PdfSnapshot {
    let objects = vec![
        PdfIndirectObject {
            id: ObjRef { num: 1, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                PdfDictEntry { key: "MarkInfo".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                PdfDictEntry { key: "StructTreeRoot".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                PdfDictEntry { key: "Lang".into(), value: PdfObject::Str(lang.into_bytes()) },
                PdfDictEntry { key: "ViewerPreferences".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
            ]),
        },
        PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Marked".into(), value: PdfObject::Bool(true) }]) },
        PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("StructTreeRoot".into()) }]) },
        PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "DisplayDocTitle".into(), value: PdfObject::Bool(true) }]) },
    ];
    PdfSnapshot { objects, ..PdfSnapshot::default() }
}
//#endregion 🔖️Seed

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct PdfUaBuilder {
    snapshot: PdfSnapshot,
}

impl PdfUaBuilder {
    /// ➕ The recommended entry point: REQUIRES a language tag (e.g. `"en-US"`) up front.
    pub fn new(lang: impl Into<String>) -> Self {
        Self { snapshot: seeded_snapshot(lang.into()) }
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

impl ArtifactBuilder for PdfUaBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;

    fn empty() -> Self {
        Self::new("en")
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

    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_ua_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    fn new_requires_lang_and_builds_clean() {
        let snapshot = PdfUaBuilder::new("en-US")
            .add_page(PdfPage::new(200.0, 200.0))
            .set_info(PdfInfo { title: Some("An Accessible Doc".into()), ..PdfInfo::default() })
            .build()
            .expect("conforming construction must build");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = PdfUaBuilder::new("en-US").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
        // Strip the seeded /StructTreeRoot to simulate a stripped-down document reaching the
        // builder via the generic `SetSnapshot` escape hatch.
        if let Some(catalog_obj) = snapshot.objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut catalog_obj.value {
                d.retain(|e| e.key != "StructTreeRoot");
            }
        }
        let (mutated, _diff) = PdfUaBuilder::from_snapshot(PdfSnapshot::default()).mutate(PdfMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a Catalog missing /StructTreeRoot must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::ua::analyzer::CODE_STRUCT_TREE_ROOT));
    }
}
