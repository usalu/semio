//! 🧬️ DocxSnapshot schema (ecma-376/✳️strict) — reuses the ✳️any subset's `DocxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.docx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's Tier-1
//! "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf
//! exists so `🪆️subsets/✳️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`,
//! without duplicating the schema definition.

pub use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::docx::schema::snapshot::{DocxDocument, DocxParagraph, DocxRun};
    use crate::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::{check_strict_conformance, STRICT_REL_BASE};
    use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};
    use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
    use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

    //#region 🔖️Namespaces
    const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
    const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
    const MAIN_DOCUMENT_PART: &str = "word/document.xml";
    //#endregion 🔖️Namespaces

    //#region 🔖️Seed
    /// 🌱️ Assembles a fresh, minimal-but-strict-conformant OPC package around `document` -- real
    /// construction (mirrors the ✳️any subset's `build_minimal_docx` shape), just with the strict
    /// namespace, root `conformance="strict"` attribute, and strict officeDocument relationship base
    /// written from the start instead of the transitional ones.
    fn build_minimal_strict_docx(document: DocxDocument) -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let bytes = xml_document_to_text(&document_to_strict_xml(&document)).into_bytes();
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, bytes);
        opc.add_relationship("", "rId1", &format!("{STRICT_REL_BASE}/officeDocument"), MAIN_DOCUMENT_PART);
        DocxSnapshot::from_parts(opc, document)
    }

    /// ✍️ Same paragraph/run -> XML shape as the ✳️any subset's `engine::document_to_xml`, just with
    /// the strict `xmlns:w` value and an added `conformance="strict"` root attribute.
    /// ✍️ Renders only the `Paragraph` blocks of `doc.body` (strict conformance's ergonomic
    /// construction path is paragraph/run-only, same scope as before this ticket's table/style
    /// enrichment; a `Table` block reaching this builder via `SetSnapshot`/raw `mutate` still survives
    /// losslessly through the shared `✳️any` engine's `document_to_xml`, this fn is only the TYPED
    /// convenience path for `add_paragraph`/`add_text_paragraph`/`add_runs`).
    fn document_to_strict_xml(doc: &DocxDocument) -> XmlDocument {
        let body_children = doc
            .body
            .iter()
            .filter_map(|block| match block {
                crate::artifacts::docx::schema::snapshot::DocxBlock::Paragraph(p) => Some(p),
                _ => None,
            })
            .map(|p| {
                let run_children = p
                    .runs
                    .iter()
                    .map(|r| {
                        let mut rc = Vec::new();
                        if r.bold || r.italic || r.underline {
                            let mut rpr = Vec::new();
                            if r.bold {
                                rpr.push(XmlNode::Element { name: "w:b".into(), attrs: vec![], children: vec![] });
                            }
                            if r.italic {
                                rpr.push(XmlNode::Element { name: "w:i".into(), attrs: vec![], children: vec![] });
                            }
                            if r.underline {
                                rpr.push(XmlNode::Element { name: "w:u".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: "single".into() }], children: vec![] });
                            }
                            rc.push(XmlNode::Element { name: "w:rPr".into(), attrs: vec![], children: rpr });
                        }
                        rc.push(XmlNode::Element {
                            name: "w:t".into(),
                            attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }],
                            children: vec![XmlNode::Text { text: r.text.clone() }],
                        });
                        XmlNode::Element { name: "w:r".into(), attrs: vec![], children: rc }
                    })
                    .collect();
                XmlNode::Element { name: "w:p".into(), attrs: vec![], children: run_children }
            })
            .collect();
        XmlDocument {
            prolog: Vec::new(),
            root: Some(XmlNode::Element {
                name: "w:document".into(),
                attrs: vec![
                    XmlAttr { name: "xmlns:w".into(), value: STRICT_MAIN_NS.into() },
                    XmlAttr { name: "conformance".into(), value: "strict".into() },
                ],
                children: vec![XmlNode::Element { name: "w:body".into(), attrs: vec![], children: body_children }],
            }),
            doctype: None,
            declaration: None,
        }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct DocxStrictBuilderConstruction {
        snapshot: DocxSnapshot,
    }

    impl DocxStrictBuilderConstruction {
        /// ➕️ Appends a paragraph, re-serializing the strict-namespaced `word/document.xml` part.
        pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
            self.snapshot.document.body.push(crate::artifacts::docx::schema::snapshot::DocxBlock::Paragraph(paragraph));
            self.snapshot = build_minimal_strict_docx(self.snapshot.document);
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
    }

    impl ArtifactBuilder for DocxStrictBuilderConstruction {
        type Snapshot = DocxSnapshot;
        type Mutation = DocxMutation;
        type Diff = DocxDiff;

        fn empty() -> Self {
            Self { snapshot: build_minimal_strict_docx(DocxDocument::default()) }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::docx::schema::mutations::apply_docx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&diff, &self.snapshot);
            self
        }

        /// 🛡️ The real construction gate: re-runs `check_strict_conformance` unconditionally,
        /// regardless of which path produced the in-flight snapshot (typed `add_paragraph`,
        /// `from_binary`, a raw `SetSnapshot` mutation) -- a hard violation can never leave `build()`
        /// as `Ok`.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_strict_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
        fn empty_builder_is_strict_conformant() {
            let snapshot = DocxStrictBuilderConstruction::empty().build().expect("empty strict builder must be conformant");
            assert!(snapshot.opc.part_bytes(MAIN_DOCUMENT_PART).is_some());
        }

        #[test]
        fn add_paragraph_stays_strict_conformant() {
            let snapshot = DocxStrictBuilderConstruction::empty().add_text_paragraph("Hello, strict world!").build().expect("must build");
            assert_eq!(snapshot.document.body.len(), 1);
            let bytes = snapshot.opc.part_bytes(MAIN_DOCUMENT_PART).unwrap();
            assert!(String::from_utf8_lossy(bytes).contains(STRICT_MAIN_NS));
        }

        #[test]
        fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = DocxStrictBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
            snapshot.opc.set_part("word/legacyDrawing.xml", "application/xml", b"<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\"/>".to_vec());
            let (mutated, _diff) = DocxStrictBuilderConstruction::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("VML content must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::CODE_VML_PRESENT));
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
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

    //#region 🔖️Namespaces
    pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
    pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    pub const STRICT_REL_BASE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    pub const TRANSITIONAL_REL_BASE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    pub const VML_NS: &str = "urn:schemas-microsoft-com:vml";
    //#endregion 🔖️Namespaces

    //#region 🔖️Conformance
    pub const CODE_MAIN_NS_MISSING: &str = "stdio.docx.strict.main-ns-missing";
    pub const CODE_TRANSITIONAL_NS_PRESENT: &str = "stdio.docx.strict.transitional-ns-present";
    pub const CODE_VML_PRESENT: &str = "stdio.docx.strict.vml-present";
    pub const CODE_REL_BASE: &str = "stdio.docx.strict.non-strict-relationship-base";
    pub const CODE_CONFORMANCE_ATTR: &str = "stdio.docx.strict.conformance-attr-missing";
    pub const CODE_ALTERNATE_CONTENT: &str = "stdio.docx.strict.alternate-content-present";

    /// 🔎️ Resolves the main document part via the root officeDocument relationship -- matched by
    /// relationship-type SUFFIX (`/officeDocument`) rather than the transitional-shaped
    /// `REL_TYPE_OFFICE_DOCUMENT` constant verbatim, since a genuinely strict package's root
    /// relationship carries the SAME suffix under the strict base namespace (that swap is exactly what
    /// `CODE_REL_BASE` below checks for) -- matching by suffix here keeps this lookup honest for both
    /// conformance classes instead of silently failing to find the main part on any strict document.
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

    /// 🛡️ Real ISO/IEC 29500-1:2016 Strict conformance checks against one already-decoded
    /// `DocxSnapshot`. Shared single source of truth: `DocxStrictComposer::compose` hard-gates on
    /// this (pre-serialization, authoritative), `DocxStrictBuilder::build` hard-gates on this too, and
    /// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    pub fn check_strict_conformance(snapshot: &DocxSnapshot) -> Vec<Diagnostic> {
        let opc = &snapshot.opc;
        let mut out = Vec::new();

        match main_document_part(opc) {
            Some((part, path)) => {
                if !part_contains(&part.bytes, STRICT_MAIN_NS) {
                    out.push(hard(CODE_MAIN_NS_MISSING, format!("main document part {path} does not declare the strict WordprocessingML namespace {STRICT_MAIN_NS}")));
                }
                if !part_contains(&part.bytes, "conformance=\"strict\"") {
                    out.push(soft(CODE_CONFORMANCE_ATTR, format!("main document part {path} root element does not declare conformance=\"strict\"")));
                }
            }
            None => out.push(hard(
                CODE_MAIN_NS_MISSING,
                "package has no root officeDocument relationship -- cannot locate the main document part to check the strict namespace on".into(),
            )),
        }

        for part in &opc.parts {
            if part_contains(&part.bytes, TRANSITIONAL_MAIN_NS) {
                out.push(hard(
                    CODE_TRANSITIONAL_NS_PRESENT,
                    format!("part {} contains the transitional WordprocessingML namespace {TRANSITIONAL_MAIN_NS} -- strict conformance forbids mixed namespaces", part.path),
                ));
            }
            if part_contains(&part.bytes, VML_NS) {
                out.push(hard(
                    CODE_VML_PRESENT,
                    format!("part {} contains the VML namespace {VML_NS} -- VML is transitional-only markup, forbidden under strict conformance", part.path),
                ));
            }
            if part_contains(&part.bytes, "mc:AlternateContent") {
                out.push(soft(CODE_ALTERNATE_CONTENT, format!("part {} contains mc:AlternateContent compatibility markup", part.path)));
            }
        }

        let mut owners: Vec<&String> = opc.relationships.keys().collect();
        owners.sort();
        for owner in owners {
            for rel in &opc.relationships[owner] {
                if rel.rel_type.starts_with(TRANSITIONAL_REL_BASE) {
                    out.push(hard(
                        CODE_REL_BASE,
                        format!("relationship {} owned by {owner:?} uses the transitional relationship base {TRANSITIONAL_REL_BASE} -- strict conformance requires {STRICT_REL_BASE}", rel.id),
                    ));
                }
            }
        }

        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.docx` (ecma-376/✳️strict): delegates the real parse to the ✳️any subset's
    /// analyzer (same `DocxSnapshot`), then folds real ISO/IEC 29500-1 Strict conformance diagnostics
    /// on top.
    pub struct DocxStrictAnalyzerAnalysis;

    impl ArtifactAnalysis for DocxStrictAnalyzerAnalysis {
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
                let checks = check_strict_conformance(snapshot);
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

        fn strict_document_bytes() -> Vec<u8> {
            format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}" conformance="strict"><w:body/></w:document>"#).into_bytes()
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
        fn conforming_strict_document_has_no_hard_diagnostics() {
            let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
            let snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn missing_strict_namespace_is_hard() {
            let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
            let snapshot = snapshot_with_main_part(&rel_type, b"<w:document><w:body/></w:document>".to_vec());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn transitional_namespace_anywhere_is_hard() {
            let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
            let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
            snapshot.opc.set_part("word/styles.xml", "application/xml", format!(r#"<w:styles xmlns:w="{TRANSITIONAL_MAIN_NS}"/>"#).into_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRANSITIONAL_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn vml_namespace_anywhere_is_hard() {
            let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
            let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
            snapshot.opc.set_part("word/header1.xml", "application/xml", format!(r#"<w:hdr xmlns:v="{VML_NS}"/>"#).into_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn transitional_relationship_base_is_hard() {
            let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, strict_document_bytes());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_REL_BASE && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn missing_conformance_attr_is_soft() {
            let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
            let doc = format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}"><w:body/></w:document>"#).into_bytes();
            let snapshot = snapshot_with_main_part(&rel_type, doc);
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn alternate_content_anywhere_is_soft() {
            let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
            let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
            snapshot.opc.set_part("word/document2.xml", "application/xml", b"<mc:AlternateContent/>".to_vec());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ALTERNATE_CONTENT && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn missing_officedocument_relationship_is_hard() {
            let mut opc = OpcPackage::empty();
            opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
            let snapshot = DocxSnapshot::from_parts(opc, Default::default());
            let diagnostics = check_strict_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DocxStrictBuilderFacets {
        construction: derived_construction::DocxStrictBuilderConstruction,
        analysis: derived_analysis::DocxStrictAnalyzerAnalysis,
        composition: super::io::derived_composition::DocxStrictComposerComposition,
    }
    builder: DocxStrictBuilder,
    analyzer: DocxStrictAnalyzer,
    composer: DocxStrictComposer,
);
//#endregion 🧬️DerivedArtifactFacets
