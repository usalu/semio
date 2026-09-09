//! 🧬️ XlsxSnapshot schema (ecma-376/🌉️transitional) — reuses the 🧱️base subset's `XlsxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.xlsx` schema id). ISO/IEC 29500-4 Transitional
//! conformance is a validation-gated dialect STAMP on top of that existing schema, not a new one
//! (D4's Tier-1 "same snapshot type, subset moves" semantics — `ArtifactCommand::MigrateDialect`).
//! This leaf exists so `🪆️subsets/🌉️transitional/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::standards::v_ecma_376::subsets::base::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::transitional::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    #[cfg(test)]
    use crate::standards::v_ecma_376::subsets::base::schema::mutations::set_snapshot;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSnapshot, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::transitional::schema::{check_transitional_conformance, TRANSITIONAL_R_NS, TRANSITIONAL_SML_NS};
    use crate::{XlsxDiff, XlsxMutation};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;
    use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlNode};

    const WORKBOOK_PART: &str = "xl/workbook.xml";
    const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

    //#region 🔖️Stamp
    /// 🖋️ Real-rewrites `snapshot.opc`'s `xl/workbook.xml` root attrs to explicit Transitional shape.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn stamp_transitional_namespace(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: &str) {
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
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(workbook: XlsxWorkbook) -> Self {
            Self { snapshot: stamp_transitional_namespace(crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(workbook)) }
        }
    }

    impl ArtifactBuilder for XlsxTransitionalBuilderConstruction {
        type Snapshot = XlsxSnapshot;
        type Mutation = XlsxMutation;
        type Diff = XlsxDiff;

        fn empty() -> Self {
            Self::new(XlsxWorkbook::default())
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard Transitional
        /// violation (Strict-shaped namespace/conformance attribute, or an unparsable workbook.xml)
        /// fails `build()`; the soft diagnostic passes through as advisory.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
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
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use crate::standards::v_ecma_376::subsets::base::schema::XlsxAnalyzer as XlsxAnyAnalyzer;
    pub use crate::standards::v_ecma_376::subsets::base::schema::XlsxParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, XmlNode};

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn workbook_root_attrs(snapshot: &XlsxSnapshot) -> Option<(Option<String>, Option<String>, Option<String>)> {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🩺️ Real worksheet content-type scan -- same check as 🔒️strict's own copy, duplicated (small
    /// enough, CODE_* consts stay subset-namespaced) rather than a cross-subset dependency.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn worksheet_content_type_gaps(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
        snapshot
            .opc
            .parts
            .iter()
            .filter(|p| p.path.starts_with("xl/worksheets/") && p.path.ends_with(".xml") && p.content_type != WORKSHEET_CONTENT_TYPE)
            .map(|p| soft(CODE_WORKSHEET_CONTENT_TYPE, format!("worksheet part {} resolves content type {:?}, expected {WORKSHEET_CONTENT_TYPE:?} (ECMA-376 Part 1 §12.3.24)", p.path, p.content_type)))
            .collect()
    }

    /// 🛡️ Real ISO/IEC 29500-4 (Transitional) conformance checks against one already-decoded
    /// `XlsxSnapshot`. Same single-source-of-truth role as 🔒️strict's `check_strict_conformance`:
    /// `XlsxTransitionalComposer::compose` and `XlsxTransitionalBuilder::build` hard-gate on this, and
    /// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_transitional_conformance(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
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
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/🌉️transitional): delegates the real parse to the 🧱️base
    /// subset's analyzer (same `XlsxSnapshot`), then folds real Transitional conformance diagnostics
    /// on top. `sniff` also delegates -- same rationale as 🔒️strict's analyzer.
    pub struct XlsxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxTransitionalAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            XlsxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
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
