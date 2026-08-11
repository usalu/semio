//! 🏗️ PdfEBuilder (1.7/✳️e) — a typed builder whose ergonomic path can only produce a
//! PDF/E-1-conforming `PdfSnapshot` BY CONSTRUCTION: there is no `set_encryption`/`set_action`/
//! `set_javascript`/`add_movie_annotation`/`add_sound_annotation` method — the only mutating
//! methods are `add_page`/`set_info`, matching PDF/E's forbidden vocabulary (Movie/Sound
//! annotations forbidden, `/Subtype /3D` allowed but out of this builder's ergonomic scope
//! anyway since no 3D-annotation constructor exists here either).
//!
//! `build()` re-runs the SAME `check_e_conformance` used by `PdfEComposer`, unconditionally.
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::e::analyzer::check_e_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct PdfEBuilder {
    snapshot: PdfSnapshot,
}

impl PdfEBuilder {
    pub fn new() -> Self {
        Self { snapshot: PdfSnapshot::default() }
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

impl Default for PdfEBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactBuilder for PdfEBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;

    fn empty() -> Self {
        Self::new()
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
        let hard: Vec<Diagnostic> = check_e_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject};

    #[test]
    fn empty_builder_builds_clean() {
        let snapshot = PdfEBuilder::new().add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("An E Test".into()), ..PdfInfo::default() }).build().expect("no hard violations by default");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let violating = PdfIndirectObject {
            id: ObjRef { num: 99, gen: 0 },
            value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Movie".into()) }]),
        };
        let mut snapshot = PdfEBuilder::new().add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
        snapshot.objects.push(violating);
        let (mutated, _diff) = PdfEBuilder::from_snapshot(PdfSnapshot::default()).mutate(PdfMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a Movie annotation must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::e::analyzer::CODE_MOVIE_OR_SOUND));
    }
}
