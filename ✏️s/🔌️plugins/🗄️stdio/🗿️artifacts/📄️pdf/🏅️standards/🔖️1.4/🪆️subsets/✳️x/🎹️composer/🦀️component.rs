//! 🎹️ PdfXComposer (1.4/✳️x) — reads the same sources the ✳️any subset does (native `stdio.pdf`
//! 1.4, plus its `binary`/`deflate` DAG deps), delegates the actual parse to the ✳️any composer,
//! then folds the honestly-scope-limited PDF/X diagnostics on top. PASS-THROUGH by design (see
//! module doc comment on `🧐️analyzer`): `PageDoc{width,height,text}` has no object graph, so
//! there is no field this composer could inject or strip to enforce a hard gate -- unlike 1.7's
//! `✳️a`, `compose` here can never fail on conformance grounds, only on the same errors the
//! `✳️any` delegate itself can already return.
//!
//! Still registers a `SubsetValidator` (D5's generic validate-on-build hook) -- required by the
//! W1 `policyStandardSubsetVocabularyBreaches` policy rule for every real (non-`✳️any`) stdio
//! subset, regardless of whether it can hard-gate.

use std::sync::OnceLock;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::pdf::standards::v1_4::subsets::x::analyzer::check_pdf_x_conformance;
use crate::artifacts::pdf::standards::v1_4::subsets::any::composer::PdfComposer as PdfAnyComposer;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

const DIALECT_X: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("x") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct PdfXComposer;

impl ArtifactComposer for PdfXComposer {
    type Snapshot = PdfSnapshot;
    const WRITES: Dialect = DIALECT_X;

    fn reads() -> &'static [Dialect] {
        &[DIALECT_ANY, DIALECT_X, DEP_BINARY, DEP_DEFLATE]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = PdfAnyComposer::compose(sources)?;
        let mut diagnostics = inner.diagnostics;
        diagnostics.extend(check_pdf_x_conformance(&inner.snapshot));
        Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
pub struct PdfXValidator;

impl SubsetValidator for PdfXValidator {
    const DIALECT: Dialect = DIALECT_X;

    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_pdf_x_conformance(&snapshot),
            None => vec![Diagnostic {
                code: FaultCode::new("stdio.pdf.x.validate-decode-failed"),
                severity: Severity::Warning,
                span: TextSpan::at(1, 1),
                message: "PDF/X (1.4) SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

fn validator_entry() -> &'static SubsetValidatorEntry {
    VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfXValidator>)
}

/// 📌️ Registers this subset's `SubsetValidator`. Called from 1.4's own `⚙️engine::register()`.
/// The `ComposerEntry` itself is registered separately via this standard's own
/// `composer::entries()` aggregation.
pub fn register() {
    register_subset_validator(validator_entry());
}
//#endregion 🔖️SubsetValidator

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;
    use crate::artifacts::pdf::standards::v1_4::subsets::x::analyzer::CODE_SCHEMA_GAP;

    #[test]
    fn compose_always_carries_the_schema_gap_diagnostic() {
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&PdfSnapshot::default());
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = PdfXComposer::compose(&sources).expect("pass-through compose never fails on conformance grounds");
        assert!(composed.diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP), "got {:?}", composed.diagnostics);
    }

    #[test]
    fn subset_validator_reports_the_schema_gap_diagnostic() {
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&PdfSnapshot::default());
        let diagnostics = PdfXValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP), "got {diagnostics:?}");
    }
}
