//! 🏗️ PdfXBuilder (1.7/✳️x) — a typed builder whose ergonomic path can only produce a
//! PDF/X-4-conforming `PdfSnapshot` BY CONSTRUCTION:
//! - `new()` REQUIRES an output-condition identifier for its seeded `/GTS_PDFX` OutputIntent.
//! - There is no `set_encryption`/`set_action`/`set_javascript` method — the only mutating
//!   methods are `add_page`/`set_info`, matching PDF/X's restricted content vocabulary.
//!
//! `build()` re-runs the SAME `check_x_conformance` used by `PdfXComposer`, unconditionally, so a
//! hard PDF/X violation can never leave this builder as an `Ok(PdfSnapshot)`. Ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::x::analyzer::check_x_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfInfo, PdfIndirectObject, PdfObject, PdfPage, PdfSnapshot};

//#region 🔖️Seed
/// 🌱️ Seeds a fresh snapshot with a real `/Root /OutputIntents` → `OutputIntent` object pair
/// (`/S /GTS_PDFX` + `/DestOutputProfile`, ISO 15930-7's own conformance marker).
fn seeded_snapshot(output_condition: String) -> PdfSnapshot {
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
                PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(output_condition.into_bytes()) },
                PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
            ]),
        },
    ];
    PdfSnapshot { objects, ..PdfSnapshot::default() }
}
//#endregion 🔖️Seed

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct PdfXBuilder {
    snapshot: PdfSnapshot,
}

impl PdfXBuilder {
    /// ➕ The recommended entry point: REQUIRES an output-condition identifier up front.
    pub fn new(output_condition: impl Into<String>) -> Self {
        Self { snapshot: seeded_snapshot(output_condition.into()) }
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

impl ArtifactBuilder for PdfXBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;

    fn empty() -> Self {
        Self::new("FOGRA39")
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
        let hard: Vec<Diagnostic> = check_x_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    fn new_requires_output_condition_and_builds_clean() {
        let snapshot = PdfXBuilder::new("FOGRA39").add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("An X Test".into()), ..PdfInfo::default() }).build().expect("conforming construction must build");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let violating = PdfIndirectObject {
            id: ObjRef { num: 99, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
            ]),
        };
        let mut snapshot = PdfXBuilder::new("FOGRA39").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
        snapshot.objects.push(violating);
        let (mutated, _diff) = PdfXBuilder::from_snapshot(PdfSnapshot::default()).mutate(PdfMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("an /Encrypt dict must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::x::analyzer::CODE_ENCRYPT));
    }
}
