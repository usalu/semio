//! 🏗️ PdfVtBuilder (1.7/✳️vt) — a typed builder whose ergonomic path can only produce a
//! PDF/VT-1/-2-conforming `PdfSnapshot` BY CONSTRUCTION: `new()` REQUIRES an output-condition
//! identifier (the same X-4 `/GTS_PDFX` OutputIntent requirement `✳️x` has) and additionally
//! seeds a minimal `/DPartRoot` → one `/DPart` node carrying `/DPM` -- there is no way to reach a
//! built snapshot without both via this path.
//!
//! `build()` re-runs the SAME `check_vt_conformance` used by `PdfVtComposer` (which itself layers
//! on `✳️x::check_x_conformance`), unconditionally. Ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::vt::analyzer::check_vt_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfInfo, PdfIndirectObject, PdfObject, PdfPage, PdfSnapshot};

//#region 🔖️Seed
/// 🌱️ Seeds a fresh snapshot with a real `/GTS_PDFX` OutputIntent (same shape `✳️x` seeds) plus
/// a minimal `/DPartRoot` → `/DParts` → one `/DPart` node carrying `/DPM`.
fn seeded_snapshot(output_condition: String) -> PdfSnapshot {
    let objects = vec![
        PdfIndirectObject {
            id: ObjRef { num: 1, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
                PdfDictEntry { key: "DPartRoot".into(), value: PdfObject::Ref(ObjRef { num: 10, gen: 0 }) },
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
        PdfIndirectObject {
            id: ObjRef { num: 10, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("DPartRoot".into()) },
                PdfDictEntry { key: "DParts".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 11, gen: 0 })]) },
            ]),
        },
        PdfIndirectObject {
            id: ObjRef { num: 11, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("DPart".into()) },
                PdfDictEntry { key: "DPM".into(), value: PdfObject::Dict(vec![]) },
            ]),
        },
    ];
    PdfSnapshot { objects, ..PdfSnapshot::default() }
}
//#endregion 🔖️Seed

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct PdfVtBuilder {
    snapshot: PdfSnapshot,
}

impl PdfVtBuilder {
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

impl ArtifactBuilder for PdfVtBuilder {
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
        let hard: Vec<Diagnostic> = check_vt_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
        let snapshot = PdfVtBuilder::new("FOGRA39").add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("A VT Test".into()), ..PdfInfo::default() }).build().expect("conforming construction must build");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = PdfVtBuilder::new("FOGRA39").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
        if let Some(catalog_obj) = snapshot.objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut catalog_obj.value {
                d.retain(|e| e.key != "DPartRoot");
            }
        }
        let (mutated, _diff) = PdfVtBuilder::from_snapshot(PdfSnapshot::default()).mutate(PdfMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a Catalog missing /DPartRoot must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::vt::analyzer::CODE_DPART_ROOT));
    }
}
