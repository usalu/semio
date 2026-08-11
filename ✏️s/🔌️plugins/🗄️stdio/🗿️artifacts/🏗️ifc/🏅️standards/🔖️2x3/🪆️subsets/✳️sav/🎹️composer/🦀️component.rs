//! 🎹️ Ifc2x3SavComposer (2x3/✳️sav) — reads the same sources ✳️any does, delegates the parse to
//! the ✳️any composer, then HARD-GATES the `sav` dialect stamp on real Structural Analysis View
//! conformance (refuses composition when no `IFCSTRUCTURALANALYSISMODEL` exists, per the roster's
//! own composer duty: "refuse when no analysis model exists"). Also registers this dialect's
//! `SubsetValidator`.

use std::sync::OnceLock;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::ifc::standards::v2x3::subsets::sav::analyzer::check_sav_conformance;
use crate::artifacts::ifc::standards::v2x3::subsets::any::composer::Ifc2x3Composer as Ifc2x3AnyComposer;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

const DIALECT_SAV: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("sav") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct Ifc2x3SavComposer;

impl ArtifactComposer for Ifc2x3SavComposer {
    type Snapshot = Ifc2x3Snapshot;
    const WRITES: Dialect = DIALECT_SAV;

    fn reads() -> &'static [Dialect] {
        &[DIALECT_ANY, DIALECT_SAV, DEP_TXT]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = Ifc2x3AnyComposer::compose(sources)?;
        let checks = check_sav_conformance(&inner.snapshot);
        let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
        if !hard.is_empty() {
            let mut all = hard.clone();
            all.extend(soft);
            return Err(ComposeError {
                message: format!("Structural Analysis View conformance violated: {} hard issue(s) -- not stamping the sav dialect", hard.len()),
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
pub struct Ifc2x3SavValidator;

impl SubsetValidator for Ifc2x3SavValidator {
    const DIALECT: Dialect = DIALECT_SAV;

    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_sav_conformance(&snapshot),
            None => vec![Diagnostic {
                code: FaultCode::new("stdio.ifc.2x3.sav.validate-decode-failed"),
                severity: Severity::Warning,
                span: TextSpan::at(1, 1),
                message: "Ifc2x3Sav SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

fn validator_entry() -> &'static SubsetValidatorEntry {
    VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3SavValidator>)
}

pub fn register() {
    register_subset_validator(validator_entry());
}
//#endregion 🔖️SubsetValidator

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;
    use crate::artifacts::ifc::standards::v2x3::subsets::sav::analyzer::CODE_NO_ANALYSIS_MODEL;
    use crate::artifacts::ifc::standards::v2x3::subsets::sav::builder::Ifc2x3SavBuilder;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[test]
    fn conforming_builder_snapshot_composes_and_stamps_sav() {
        let snapshot = Ifc2x3SavBuilder::new().build().expect("clean SAV document must build");
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = Ifc2x3SavComposer::compose(&sources).expect("clean document must compose to sav");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[test]
    fn no_analysis_model_fails_compose_with_real_diagnostic() {
        let mut snapshot = Ifc2x3SavBuilder::new().build().expect("build");
        snapshot.document.instances.retain(|i| !i.is_type("IFCSTRUCTURALANALYSISMODEL"));
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = Ifc2x3SavComposer::compose(&sources).expect_err("a document with no analysis model must not stamp sav");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NO_ANALYSIS_MODEL && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
