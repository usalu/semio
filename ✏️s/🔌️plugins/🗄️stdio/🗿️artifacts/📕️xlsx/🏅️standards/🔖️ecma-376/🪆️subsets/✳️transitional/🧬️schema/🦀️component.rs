//! 🧬️ XlsxSnapshot schema (ecma-376/✳️transitional) — reuses the ✳️any subset's `XlsxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.xlsx` schema id). ISO/IEC 29500-4 Transitional
//! conformance is a validation-gated dialect STAMP on top of that existing schema, not a new one
//! (D4's Tier-1 "same snapshot type, subset moves" semantics — `ArtifactCommand::MigrateDialect`).
//! This leaf exists so `🪆️subsets/✳️transitional/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxSnapshot, XlsxWorkbook};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::{check_transitional_conformance, TRANSITIONAL_R_NS, TRANSITIONAL_SML_NS};
    use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation};
    use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlNode};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    const WORKBOOK_PART: &str = "xl/workbook.xml";
    const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

    //#region 🔖️Stamp
    /// 🖋️ Real-rewrites `snapshot.opc`'s `xl/workbook.xml` root attrs to explicit Transitional shape.
    pub async fn stamp_transitional_namespace(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
        if let Some(bytes) = snapshot.opc.part_bytes(WORKBOOK_PART) {
            if let Ok(text) = std::str::from_utf8(bytes) {
                if let Ok(mut doc) = xml_document_from_text(text) {
                    if let Some(XmlNode::Element { attrs, .. }) = &mut doc.root {
                        set_attr(attrs, "xmlns", TRANSITIONAL_SML_NS);
                        set_attr(attrs, "xmlns:r", TRANSITIONAL_R_NS);
                        set_attr(attrs, "conformance", "transitional");
                    }
                    let bytes = xml_document_to_text(&doc).into_bytes();
                    snapshot.opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, bytes);
                }
            }
        }
        snapshot
    }

    async fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: &str) {
        if let Some(existing) = attrs.iter_mut().find(|a| a.name == name) {
            existing.value = value.into();
        } else {
            attrs.push(XmlAttr { name: name.into(), value: value.into() });
        }
    }
    //#endregion 🔖️Stamp

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct XlsxTransitionalBuilderConstruction {
        snapshot: XlsxSnapshot,
    }

    impl XlsxTransitionalBuilderConstruction {
        /// ➕️ The recommended entry point: builds a minimal package from `workbook` via the shared
        /// ecma-376 engine, then stamps it explicitly Transitional.
        pub async fn new(workbook: XlsxWorkbook) -> Self {
            Self { snapshot: stamp_transitional_namespace(crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(workbook)) }
        }
    }

    impl ArtifactBuilder for XlsxTransitionalBuilderConstruction {
        type Snapshot = XlsxSnapshot;
        type Mutation = XlsxMutation;
        type Diff = XlsxDiff;

        async fn empty() -> Self {
            Self::new(XlsxWorkbook::default())
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard Transitional
        /// violation (Strict-shaped namespace/conformance attribute, or an unparsable workbook.xml)
        /// fails `build()`; the soft diagnostic passes through as advisory.
        async fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_transitional_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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

        #[semio_framework_async_macros::async_test]
        async fn new_stamps_transitional_and_builds_clean() {
            let snapshot = XlsxTransitionalBuilderConstruction::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
            assert!(check_transitional_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_transitional_conformance(&snapshot));
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = XlsxTransitionalBuilderConstruction::new(XlsxWorkbook::default()).build().unwrap();
            snapshot.opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, br#"<workbook xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict"/>"#.to_vec());
            let (mutated, _diff) = XlsxTransitionalBuilderConstruction::from_snapshot(XlsxSnapshot::default()).mutate(XlsxMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("a Strict-declared workbook.xml must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::CODE_CONFORMANCE_ATTRIBUTE), "got {err:?}");
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxSnapshot;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxAnalyzer as XlsxAnyAnalyzer;
    pub use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxParts;
    use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlNode};
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Conformance
    pub const CODE_NAMESPACE_MISMATCH: &str = "stdio.xlsx.transitional.namespace-mismatch";
    pub const CODE_RELATIONSHIPS_NAMESPACE_MISMATCH: &str = "stdio.xlsx.transitional.relationships-namespace-mismatch";
    pub const CODE_CONFORMANCE_ATTRIBUTE: &str = "stdio.xlsx.transitional.conformance-attribute";
    pub const CODE_WORKSHEET_CONTENT_TYPE: &str = "stdio.xlsx.transitional.worksheet-content-type-missing";

    /// 🏷️ ISO/IEC 29500-4 Transitional SpreadsheetML main namespace (same value the shared
    /// `⚙️engine`'s private `SML_NS` uses -- duplicated here as a `pub` constant since the engine's
    /// copy isn't exported).
    pub const TRANSITIONAL_SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
    /// 🔗️ ISO/IEC 29500-4 Transitional officeDocument relationships (markup) namespace.
    pub const TRANSITIONAL_R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
    const WORKBOOK_PART: &str = "xl/workbook.xml";

    /// 🔎️ Real scan of `xl/workbook.xml`'s root element attrs -- `(xmlns, xmlns:r, conformance)`,
    /// each `None` when absent. `None` overall only when the part is missing or unparsable as XML.
    async fn workbook_root_attrs(snapshot: &XlsxSnapshot) -> Option<(Option<String>, Option<String>, Option<String>)> {
        let bytes = snapshot.opc.part_bytes(WORKBOOK_PART)?;
        let text = std::str::from_utf8(bytes).ok()?;
        let doc = xml_document_from_text(text).ok()?;
        let XmlNode::Element { name, attrs, .. } = doc.root? else { return None };
        if name != "workbook" {
            return None;
        }
        let get = |n: &str| attrs.iter().find(|a| a.name == n).map(|a| a.value.clone());
        Some((get("xmlns"), get("xmlns:r"), get("conformance")))
    }

    async fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🩺️ Real worksheet content-type scan -- same check as ✳️strict's own copy, duplicated (small
    /// enough, CODE_* consts stay subset-namespaced) rather than a cross-subset dependency.
    async fn worksheet_content_type_gaps(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
        snapshot
            .opc
            .parts
            .iter()
            .filter(|p| p.path.starts_with("xl/worksheets/") && p.path.ends_with(".xml") && p.content_type != WORKSHEET_CONTENT_TYPE)
            .map(|p| soft(CODE_WORKSHEET_CONTENT_TYPE, format!("worksheet part {} resolves content type {:?}, expected {WORKSHEET_CONTENT_TYPE:?} (ECMA-376 Part 1 §12.3.24)", p.path, p.content_type)))
            .collect()
    }

    /// 🛡️ Real ISO/IEC 29500-4 (Transitional) conformance checks against one already-decoded
    /// `XlsxSnapshot`. Same single-source-of-truth role as ✳️strict's `check_strict_conformance`:
    /// `XlsxTransitionalComposer::compose` and `XlsxTransitionalBuilder::build` hard-gate on this, and
    /// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    pub async fn check_transitional_conformance(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        let Some((xmlns, xmlns_r, conformance)) = workbook_root_attrs(snapshot) else {
            out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} is missing or unparsable as XML -- cannot verify ISO/IEC 29500-4 Transitional conformance")));
            return out;
        };
        if xmlns.as_deref() != Some(TRANSITIONAL_SML_NS) {
            out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} root xmlns is {xmlns:?}, expected the Transitional SpreadsheetML namespace {TRANSITIONAL_SML_NS:?} (ISO/IEC 29500-4)")));
        }
        if xmlns_r.as_deref() != Some(TRANSITIONAL_R_NS) {
            out.push(hard(CODE_RELATIONSHIPS_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} root xmlns:r is {xmlns_r:?}, expected the Transitional officeDocument relationships namespace {TRANSITIONAL_R_NS:?}")));
        }
        if conformance.as_deref() == Some("strict") {
            out.push(hard(CODE_CONFORMANCE_ATTRIBUTE, format!("{WORKBOOK_PART} workbook@conformance is \"strict\" -- a document that declares Strict conformance cannot be honestly stamped Transitional")));
        }
        out.extend(worksheet_content_type_gaps(snapshot));
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/✳️transitional): delegates the real parse to the ✳️any
    /// subset's analyzer (same `XlsxSnapshot`), then folds real Transitional conformance diagnostics
    /// on top. `sniff` also delegates -- same rationale as ✳️strict's analyzer.
    pub struct XlsxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxTransitionalAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            XlsxAnyAnalyzer::sniff(source)
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = XlsxAnyAnalyzer::analyze(sources);
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
        use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlDocument};
        use crate::artifacts::zip::opc::OpcPackage;

        async fn attr(name: &str, value: &str) -> XmlAttr {
            XmlAttr { name: name.into(), value: value.into() }
        }

        async fn workbook_xml(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> Vec<u8> {
            let mut attrs = vec![attr("xmlns", xmlns), attr("xmlns:r", xmlns_r)];
            if let Some(c) = conformance {
                attrs.push(attr("conformance", c));
            }
            let doc = XmlDocument { root: Some(XmlNode::Element { name: "workbook".into(), attrs, children: vec![XmlNode::Element { name: "sheets".into(), attrs: vec![], children: vec![] }] }), doctype: None, declaration: None, prolog: Vec::new() };
            xml_document_to_text(&doc).into_bytes()
        }

        async fn snapshot_with_workbook(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> XlsxSnapshot {
            let mut opc = OpcPackage::empty();
            opc.set_part(WORKBOOK_PART, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", workbook_xml(xmlns, xmlns_r, conformance));
            XlsxSnapshot::from_parts(opc, Default::default())
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_transitional_workbook_has_no_hard_diagnostics() {
            let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn strict_namespace_is_hard() {
            let snapshot = snapshot_with_workbook("http://purl.oclc.org/ooxml/spreadsheetml/main", "http://purl.oclc.org/ooxml/officeDocument/relationships", None);
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_RELATIONSHIPS_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn explicit_strict_conformance_attribute_is_hard() {
            let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, Some("strict"));
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn explicit_transitional_conformance_attribute_is_fine() {
            let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, Some("transitional"));
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn worksheet_wrong_content_type_is_soft() {
            let mut snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
            snapshot.opc.set_part("xl/worksheets/sheet1.xml", "application/xml", b"<worksheet/>".to_vec());
            let diagnostics = check_transitional_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_WORKSHEET_CONTENT_TYPE && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec XlsxTransitionalBuilderFacets {
        construction: XlsxTransitionalBuilderConstruction,
        analysis: XlsxTransitionalAnalyzerAnalysis,
        composition: super::io::derived_composition::XlsxTransitionalComposerComposition,
    }
    builder: XlsxTransitionalBuilder,
    analyzer: XlsxTransitionalAnalyzer,
    composer: XlsxTransitionalComposer,
);
//#endregion 🧬️DerivedArtifactFacets
