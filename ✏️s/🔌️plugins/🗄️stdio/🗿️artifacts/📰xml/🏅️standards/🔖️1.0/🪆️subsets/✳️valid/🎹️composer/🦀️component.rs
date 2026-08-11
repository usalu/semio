//! 🎹️ XmlValidComposer (1.0/✳️valid) — reads the same sources the ✳️any subset does (native
//! `stdio.xml` 1.0, plus its `txt` DAG dependency), delegates the actual parse to the ✳️any
//! composer, then HARD-GATES the `valid` dialect stamp on real XML 1.0 §5.1 validity conformance.
//! A hard violation (missing doctype / root-name mismatch) fails composition outright with
//! specific `Diagnostic`s naming what's wrong; a soft one (suspicious standalone+external-subset,
//! the always-on schema-gap advisory) passes through as an advisory diagnostic on the successful
//! `Composition`.
//!
//! Also registers this dialect's `SubsetValidator` (D5's generic validate-on-build hook) -- the
//! SAME `check_valid_conformance` function backs both: the hard gate here runs pre-serialization
//! against the typed `XmlSnapshot` (authoritative), while the registered validator re-runs it
//! post-hoc against the wire `IoPayload` for the generic `io_dispatch`/`wire_artifact_compose` hook.

use std::sync::OnceLock;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::xml::standards::v1_0::subsets::any::composer::XmlComposer as XmlAnyComposer;
use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::XmlSnapshot;
use crate::artifacts::xml::standards::v1_0::subsets::valid::analyzer::check_valid_conformance;

const DIALECT_VALID: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };
const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct XmlValidComposer;

impl ArtifactComposer for XmlValidComposer {
    type Snapshot = XmlSnapshot;
    const WRITES: Dialect = DIALECT_VALID;

    fn reads() -> &'static [Dialect] {
        &[DIALECT_ANY, DIALECT_VALID, DEP_TXT]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = XmlAnyComposer::compose(sources)?;
        let checks = check_valid_conformance(&inner.snapshot);
        let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
        if !hard.is_empty() {
            let mut all = hard.clone();
            all.extend(soft);
            return Err(ComposeError {
                message: format!("XML 1.0 validity violated: {} hard issue(s) -- not stamping the valid dialect", hard.len()),
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
/// 🛡️ The registered `SubsetValidator` for `1.0/valid`.
pub struct XmlValidValidator;

impl SubsetValidator for XmlValidValidator {
    const DIALECT: Dialect = DIALECT_VALID;

    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <XmlSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_valid_conformance(&snapshot),
            None => vec![Diagnostic {
                code: FaultCode::new("stdio.xml.valid.validate-decode-failed"),
                severity: Severity::Warning,
                span: TextSpan::at(1, 1),
                message: "XML valid SubsetValidator: payload did not decode as an XmlSnapshot -- skipped".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

fn validator_entry() -> &'static SubsetValidatorEntry {
    VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XmlValidValidator>)
}

/// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
/// validate-on-build hook). Called from the 1.0 standard's own `⚙️engine::register()`. The
/// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
/// (`crate::artifacts::xml::standards::v1_0::composer::entries()`).
pub fn register() {
    register_subset_validator(validator_entry());
}
//#endregion 🔖️SubsetValidator

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;

    fn conforming_xml_text() -> String {
        "<!DOCTYPE root>\n<root/>".to_string()
    }

    #[test]
    fn conforming_document_composes_and_stamps_valid() {
        let text = conforming_xml_text();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let composed = XmlValidComposer::compose(&sources).expect("clean document must compose to valid");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[test]
    fn missing_doctype_fails_compose_with_real_diagnostic() {
        let text = "<root/>".to_string();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = XmlValidComposer::compose(&sources).expect_err("a document without a doctype must not stamp valid");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[test]
    fn root_name_mismatch_fails_compose_with_real_diagnostic() {
        let text = "<!DOCTYPE book>\n<root/>".to_string();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = XmlValidComposer::compose(&sources).expect_err("a doctype/root name mismatch must not stamp valid");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.root-name-mismatch" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[test]
    fn subset_validator_recheck_flags_only_soft_diagnostics_for_a_clean_document() {
        let text = conforming_xml_text();
        let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parses");
        let bytes = <XmlSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = XmlValidValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a composer-clean document: {diagnostics:?}");
    }
}
