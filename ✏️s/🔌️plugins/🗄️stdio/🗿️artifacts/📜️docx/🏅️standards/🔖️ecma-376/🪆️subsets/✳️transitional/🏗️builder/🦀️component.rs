//! 🏗️ DocxTransitionalBuilder (ecma-376/✳️transitional) — a thin wrapper around the ✳️any
//! subset's `DocxBuilder`: that builder's shared `engine::build_minimal_docx`/`encode_docx`
//! ALREADY hardcodes the transitional WordprocessingML namespace and the transitional
//! officeDocument relationship base (see `⚙️engine/🦀️component.rs`'s `W_NS`/`REL_TYPE_OFFICE_DOCUMENT`),
//! so ergonomic construction here is transitional-conformant by construction with no separate
//! seeding step needed -- unlike `✳️strict`, which has to override that shared engine's namespace
//! choice. `build()` still re-runs `check_transitional_conformance` unconditionally (the SAME
//! single source of truth `DocxTransitionalComposer` hard-gates on), so a hard violation reaching
//! this builder via the generic `SetSnapshot`/`mutate` escape hatch can never leave `build()` as
//! `Ok`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::docx::schema::snapshot::{DocxParagraph, DocxRun, DocxStyle, DocxTable};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::builder::DocxBuilder as DocxAnyBuilder;
use crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::analyzer::check_transitional_conformance;
use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct DocxTransitionalBuilder {
    inner: DocxAnyBuilder,
}

impl DocxTransitionalBuilder {
    /// ➕️ Appends a paragraph.
    pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
        self.inner = self.inner.add_paragraph(paragraph);
        self
    }

    /// ➕️ Appends a single-run plain-text paragraph.
    pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
        self.add_paragraph(DocxParagraph::text(text.into()))
    }

    /// ➕️ Appends a paragraph made of the given runs (basic bold/italic/underline formatting).
    pub fn add_runs(self, runs: Vec<DocxRun>) -> Self {
        self.add_paragraph(DocxParagraph { runs, style: None, extra_paragraph_properties: Vec::new() })
    }

    /// ➕️ Appends a table.
    pub fn add_table(mut self, table: DocxTable) -> Self {
        self.inner = self.inner.add_table(table);
        self
    }

    /// ➕️ Appends (or replaces, by `id`) a named style.
    pub fn add_style(mut self, style: DocxStyle) -> Self {
        self.inner = self.inner.add_style(style);
        self
    }
}

impl ArtifactBuilder for DocxTransitionalBuilder {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Diff = DocxDiff;

    fn empty() -> Self {
        Self { inner: DocxAnyBuilder::empty() }
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { inner: DocxAnyBuilder::from_snapshot(snapshot) }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self { inner: DocxAnyBuilder::from_text(text)? })
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self { inner: DocxAnyBuilder::from_binary(bytes)? })
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let (inner, diff) = self.inner.mutate(mutation);
        self.inner = inner;
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.inner = self.inner.absorb(diff);
        self
    }

    /// 🛡️ The real construction gate: re-runs `check_transitional_conformance` unconditionally,
    /// regardless of which path produced the in-flight snapshot -- a hard violation can never
    /// leave `build()` as `Ok`. Syncs `opc`'s main part from `document` first (mirrors what
    /// `encode_docx` does at real encode time) — `check_transitional_conformance` needs a
    /// materialized `word/document.xml`/relationship to find at all, and the shared `DocxAnyBuilder`
    /// this wraps doesn't materialize either until actual encode.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let mut snapshot = self.inner.build()?;
        crate::artifacts::docx::standards::v_ecma_376::engine::sync_main_part(&mut snapshot);
        let hard: Vec<Diagnostic> = check_transitional_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(snapshot)
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
    fn empty_builder_is_transitional_conformant() {
        DocxTransitionalBuilder::empty().build().expect("empty transitional builder must be conformant");
    }

    #[test]
    fn add_paragraph_stays_transitional_conformant() {
        let snapshot = DocxTransitionalBuilder::empty().add_text_paragraph("Hello, transitional world!").build().expect("must build");
        assert_eq!(snapshot.document.body.len(), 1);
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = DocxTransitionalBuilder::empty().add_text_paragraph("clean").build().unwrap();
        snapshot.opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
        let (mutated, _diff) = DocxTransitionalBuilder::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("mixed-in strict namespace must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::analyzer::CODE_STRICT_NS_PRESENT));
    }
}
