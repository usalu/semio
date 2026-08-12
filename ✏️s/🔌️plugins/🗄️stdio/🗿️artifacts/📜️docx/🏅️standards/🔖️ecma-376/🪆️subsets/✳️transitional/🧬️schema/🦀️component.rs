//! 🧬️ DocxSnapshot schema (ecma-376/✳️transitional) — reuses the ✳️any subset's `DocxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.docx` schema id). ISO/IEC 29500-4:2016 Transitional
//! is a validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️transitional/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::docx::schema::snapshot::{DocxParagraph, DocxRun, DocxStyle, DocxTable};
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::DocxBuilderConstruction as DocxAnyBuilder;
    use crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct DocxTransitionalBuilderConstruction {
        inner: DocxAnyBuilder,
    }

    impl DocxTransitionalBuilderConstruction {
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

    impl ArtifactBuilder for DocxTransitionalBuilderConstruction {
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
            DocxTransitionalBuilderConstruction::empty().build().expect("empty transitional builder must be conformant");
        }

        #[test]
        fn add_paragraph_stays_transitional_conformant() {
            let snapshot = DocxTransitionalBuilderConstruction::empty().add_text_paragraph("Hello, transitional world!").build().expect("must build");
            assert_eq!(snapshot.document.body.len(), 1);
        }

        #[test]
        fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = DocxTransitionalBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
            snapshot.opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
            let (mutated, _diff) = DocxTransitionalBuilderConstruction::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("mixed-in strict namespace must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::schema::CODE_STRICT_NS_PRESENT));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::{DocxAnalyzer as DocxAnyAnalyzer, DocxParts};
    use crate::artifacts::docx::DocxSnapshot;
    use crate::artifacts::zip::opc::{resolve_relationship_target, OpcPackage, OpcPart};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Namespaces
    pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    /// 🔎️ Every ISO/IEC 29500-1 Strict namespace URI (markup AND relationship) shares this prefix --
    /// see `✳️strict`'s `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with it.
    pub const STRICT_NS_FAMILY_PREFIX: &str = "purl.oclc.org/ooxml";
    //#endregion 🔖️Namespaces

    //#region 🔖️Conformance
    pub const CODE_MAIN_NS_MISSING: &str = "stdio.docx.transitional.main-ns-missing";
    pub const CODE_STRICT_NS_PRESENT: &str = "stdio.docx.transitional.strict-ns-present";
    pub const CODE_CONFORMANCE_ATTR: &str = "stdio.docx.transitional.conformance-attr-invalid";

    /// 🔎️ Resolves the main document part via the root officeDocument relationship -- matched by
    /// relationship-type SUFFIX (`/officeDocument`) so this resolves for either conformance class; see
    /// `✳️strict::analyzer::main_document_part`'s doc comment for the full rationale.
    fn main_document_part<'a>(opc: &'a OpcPackage) -> Option<(&'a OpcPart, String)> {
        let rel = opc.relationships_for("").iter().find(|r| r.rel_type.ends_with("/officeDocument"))?;
        let path = resolve_relationship_target("", &rel.target);
        opc.part(&path).map(|p| (p, path))
    }

    fn part_contains(bytes: &[u8], needle: &str) -> bool {
        !needle.is_empty() && bytes.windows(needle.len()).any(|w| w == needle.as_bytes())
    }

    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO/IEC 29500-4:2016 Transitional conformance checks against one already-decoded
    /// `DocxSnapshot`. Shared single source of truth: `DocxTransitionalComposer::compose` hard-gates
    /// on this (pre-serialization, authoritative), `DocxTransitionalBuilder::build` hard-gates on this
    /// too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    pub fn check_transitional_conformance(snapshot: &DocxSnapshot) -> Vec<Diagnostic> {
        let opc = &snapshot.opc;
        let mut out = Vec::new();

        match main_document_part(opc) {
            Some((part, path)) => {
                if !part_contains(&part.bytes, TRANSITIONAL_MAIN_NS) {
                    out.push(hard(CODE_MAIN_NS_MISSING, format!("main document part {path} does not declare the transitional WordprocessingML namespace {TRANSITIONAL_MAIN_NS}")));
                }
                if part_contains(&part.bytes, "conformance=\"strict\"") {
                    out.push(soft(CODE_CONFORMANCE_ATTR, format!("main document part {path} root element declares conformance=\"strict\" -- transitional documents must leave it absent or =\"transitional\"")));
                }
            }
            None => out.push(hard(
                CODE_MAIN_NS_MISSING,
                "package has no root officeDocument relationship -- cannot locate the main document part to check the transitional namespace on".into(),
            )),
        }

        for part in &opc.parts {
            if part_contains(&part.bytes, STRICT_NS_FAMILY_PREFIX) {
                out.push(hard(
                    CODE_STRICT_NS_PRESENT,
                    format!("part {} contains a strict-family namespace ({STRICT_NS_FAMILY_PREFIX}) -- transitional conformance forbids mixed namespaces", part.path),
                ));
            }
        }

        let mut owners: Vec<&String> = opc.relationships.keys().collect();
        owners.sort();
        for owner in owners {
            for rel in &opc.relationships[owner] {
                if rel.rel_type.contains(STRICT_NS_FAMILY_PREFIX) {
                    out.push(hard(
                        CODE_STRICT_NS_PRESENT,
                        format!("relationship {} owned by {owner:?} uses a strict-family relationship base ({STRICT_NS_FAMILY_PREFIX}) -- transitional conformance forbids it", rel.id),
                    ));
                }
            }
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.docx` (ecma-376/✳️transitional): delegates the real parse to the ✳️any
    /// subset's analyzer (same `DocxSnapshot`), then folds real ISO/IEC 29500-4 Transitional
    /// conformance diagnostics on top.
    pub struct DocxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for DocxTransitionalAnalyzerAnalysis {
        type Parts = DocxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            DocxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = DocxAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_transitional_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

        fn transitional_document_bytes() -> Vec<u8> {
            format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}"><w:body/></w:document>"#).into_bytes()
        }

        fn snapshot_with_main_part(rel_type: &str, doc_bytes: Vec<u8>) -> DocxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", doc_bytes);
            opc.add_relationship("", "rId1", rel_type, "word/document.xml");
            DocxSnapshot::from_parts(opc, Default::default())
        }

        #[test]
        fn conforming_transitional_document_has_no_hard_diagnostics() {
            let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, transitional_document_bytes());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_CONFORMANCE_ATTR), "got {diagnostics:?}");
        }

        #[test]
        fn missing_transitional_namespace_is_hard() {
            let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, b"<w:document><w:body/></w:document>".to_vec());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn strict_namespace_anywhere_is_hard() {
            let mut snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, transitional_document_bytes());
            snapshot.opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn strict_relationship_base_anywhere_is_hard() {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            opc.content_types.set_default("xml", "application/xml");
            opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", transitional_document_bytes());
            opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
            opc.add_relationship("word/document.xml", "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/image", "media/image1.png");
            let snapshot = DocxSnapshot::from_parts(opc, Default::default());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn conformance_strict_attribute_present_is_soft() {
            let doc = format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}" conformance="strict"><w:body/></w:document>"#).into_bytes();
            let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, doc);
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DocxTransitionalBuilderFacets {
        construction: derived_construction::DocxTransitionalBuilderConstruction,
        analysis: derived_analysis::DocxTransitionalAnalyzerAnalysis,
        composition: super::io::derived_composition::DocxTransitionalComposerComposition,
    }
    builder: DocxTransitionalBuilder,
    analyzer: DocxTransitionalAnalyzer,
    composer: DocxTransitionalComposer,
);
//#endregion 🧬️DerivedArtifactFacets
