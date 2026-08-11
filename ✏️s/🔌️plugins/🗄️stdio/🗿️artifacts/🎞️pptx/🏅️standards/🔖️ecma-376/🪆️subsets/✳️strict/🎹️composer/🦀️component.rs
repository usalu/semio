//! 🎹️ PptxStrictComposer (ecma-376/✳️strict) — reads the same sources the ✳️any subset does
//! (native `stdio.pptx` ecma-376, plus its `zip`/`xml` DAG deps), delegates the actual parse to
//! the ✳️any composer, then HARD-GATES the `strict` dialect stamp on real ISO/IEC 29500-1:2016
//! Strict conformance (mirrors `📄️pdf` 1.7 `✳️a`'s `PdfAComposer`, D5 requirement #2: "Dialect
//! stamped only when clean"). A hard violation (wrong main namespace, Transitional namespace or
//! VML present, non-Strict relationship base) fails composition outright with specific
//! `Diagnostic`s naming what's wrong; a soft one (missing `conformance="strict"`,
//! `mc:AlternateContent` markup) passes through as an advisory diagnostic on the successful
//! `Composition`.
//!
//! Also registers this dialect's `SubsetValidator` (D5's generic validate-on-build hook, see
//! `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`) — the SAME `check_strict_conformance` function
//! backs both: the hard gate here runs pre-serialization against the typed `PptxSnapshot`
//! (authoritative), while the registered validator re-runs it post-hoc against the wire
//! `IoPayload` for the generic `io_dispatch`/`wire_artifact_compose` hook.

use std::sync::OnceLock;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::analyzer::check_strict_conformance;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::composer::PptxComposer as PptxAnyComposer;
use crate::artifacts::pptx::PptxSnapshot;

const DIALECT_STRICT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct PptxStrictComposer;

impl ArtifactComposer for PptxStrictComposer {
    type Snapshot = PptxSnapshot;
    const WRITES: Dialect = DIALECT_STRICT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT_ANY, DIALECT_STRICT, DEP_ZIP, DEP_XML]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = PptxAnyComposer::compose(sources)?;
        let checks = check_strict_conformance(&inner.snapshot);
        let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
        if !hard.is_empty() {
            let mut all = hard.clone();
            all.extend(soft);
            return Err(ComposeError {
                message: format!("ISO/IEC 29500-1 Strict conformance violated: {} hard issue(s) -- not stamping the strict dialect", hard.len()),
                diagnostics: all,
            });
        }
        let mut diagnostics = inner.diagnostics;
        diagnostics.extend(soft);
        Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
pub struct PptxStrictValidator;

impl SubsetValidator for PptxStrictValidator {
    const DIALECT: Dialect = DIALECT_STRICT;

    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <PptxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_strict_conformance(&snapshot),
            None => vec![Diagnostic {
                code: FaultCode::new("stdio.pptx.strict.validate-decode-failed"),
                severity: Severity::Warning,
                span: TextSpan::at(1, 1),
                message: "Strict SubsetValidator: payload did not decode as a PptxSnapshot -- skipped".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

fn validator_entry() -> &'static SubsetValidatorEntry {
    VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PptxStrictValidator>)
}

/// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
/// validate-on-build hook). Called from ecma-376's own `⚙️engine::register()`. The `ComposerEntry`
/// itself is registered separately by the standard-level composer aggregator
/// (`crate::artifacts::pptx::standards::v_ecma_376::composer::entries()`), matching how `✳️any`'s
/// own entry is registered.
pub fn register() {
    register_subset_validator(validator_entry());
}
//#endregion 🔖️SubsetValidator

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;
    use crate::artifacts::zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE};

    const STRICT_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    fn hex_encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// 🩹 Builds real OPC zip bytes directly via `opc::encode_opc` (never `PptxSnapshot`'s
    /// `ArtifactPack::encode_pack`, which round-trips through `⚙️engine::encode_pptx` --
    /// `regenerate_presentation_parts` unconditionally rewrites `ppt/presentation.xml` with
    /// hardcoded Transitional markup, which would silently clobber this hand-crafted Strict
    /// content). Routing through `AnalyzeSource::Text(hex)` (`ArtifactDsl::parse_dsl`) is how the
    /// `📄️pdf` 1.7 `✳️a` pilot's own composer tests exercise the real decode path -- same
    /// technique here.
    fn strict_package_hex() -> String {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", STRICT_PRESENTATION_XML.as_bytes().to_vec());
        opc.set_part("ppt/slideMasters/slideMaster1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", b"<p:sldMaster/>".to_vec());
        opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
        opc.add_relationship("ppt/presentation.xml", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster", "slideMasters/slideMaster1.xml");
        let bytes = opc::encode_opc(&opc).expect("encode hand-built Strict OPC package");
        hex_encode(&bytes)
    }

    #[test]
    fn conforming_strict_package_composes_and_stamps_strict() {
        let hex = strict_package_hex();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let composed = PptxStrictComposer::compose(&sources).expect("clean Strict document must compose to strict");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[test]
    fn transitional_document_fails_compose_with_real_diagnostic() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let transitional_xml = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", transitional_xml.as_bytes().to_vec());
        opc.set_part("ppt/slideMasters/slideMaster1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml", b"<p:sldMaster/>".to_vec());
        opc.add_relationship("", "rId1", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument", "ppt/presentation.xml");
        opc.add_relationship("ppt/presentation.xml", "rId1", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster", "slideMasters/slideMaster1.xml");
        let bytes = opc::encode_opc(&opc).expect("encode hand-built Transitional OPC package");
        let hex = hex_encode(&bytes);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let err = PptxStrictComposer::compose(&sources).expect_err("a Transitional document must not stamp strict");
        assert!(err.diagnostics.iter().any(|d| d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[test]
    fn subset_validator_recheck_flags_clean_document_as_clean() {
        let hex = strict_package_hex();
        let diagnostics = PptxStrictValidator::validate(&IoPayload::Text(hex));
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a clean Strict document: {diagnostics:?}");
    }
}
