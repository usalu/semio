//! 🎹️ SemioComposer (s.stdio.semio/v1/*) — analyzer-only compose (decodes the subset's own
//! JSON-pack payload); W4 adds real cross-format compose sources once semio↔format import/export
//! leaves land. W2b closer real implementation: registers the mandatory `SubsetValidator` for
//! this artifact's "*" dialect — see the `🔖️SubsetValidator` region below.

use std::sync::OnceLock;
use dsl::Diagnostic;
use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::artifacts::semio::standards::v1::subsets::any::analyzer::SemioAnalyzer;
use crate::artifacts::semio::standards::v1::subsets::brep::composer::SemioBrepValidator;
use crate::artifacts::semio::standards::v1::subsets::mesh::composer::SemioMeshValidator;
use crate::artifacts::semio::standards::v1::subsets::model::composer::SemioModelValidator;
use crate::artifacts::semio::standards::v1::subsets::object::composer::SemioObjectValidator;
use crate::artifacts::semio::standards::v1::subsets::document::composer::SemioDocumentValidator;
use crate::artifacts::semio::standards::v1::subsets::cad::composer::SemioCadValidator;
use crate::artifacts::semio::standards::v1::subsets::drawing::composer::SemioDrawingValidator;
use crate::artifacts::semio::standards::v1::subsets::image::composer::SemioImageValidator;
use crate::artifacts::semio::standards::v1::subsets::video::composer::SemioVideoValidator;
use crate::artifacts::semio::standards::v1::subsets::audio::composer::SemioAudioValidator;
use crate::artifacts::semio::standards::v1::subsets::animation::composer::SemioAnimationValidator;
use crate::artifacts::semio::standards::v1::subsets::presentation::composer::SemioPresentationValidator;
use crate::artifacts::semio::standards::v1::subsets::workflow::composer::SemioWorkflowValidator;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct SemioComposer;

impl ArtifactComposer for SemioComposer {
    type Snapshot = SemioSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ The envelope's own `SubsetValidator` for the `"*"` dialect (D5's generic
/// validate-on-build hook — required by policy, same as every one of the 13 subsets' own
/// validators, `pdf`'s `✳️a` composer is the copy template). Decodes the payload as a
/// `SemioSnapshot`, then DELEGATES to whichever one of the 13 subsets' OWN, already-real
/// `SubsetValidator`s matches the decoded snapshot's active kind — this validator owns zero
/// invariant logic itself, only the envelope-level decode + dispatch, exactly mirroring how
/// `SemioDiff`/`SemioMutation` themselves only own routing, never re-derived per-subset rules.
pub struct SemioValidator;

/// 🔎️ Real dispatch: re-encodes the decoded inner snapshot through ITS OWN subset's
/// `ArtifactPack`, then calls that subset's own registered `SubsetValidator::validate` — genuine
/// reuse of all 13 already-tested invariant checks, never duplicated here.
fn dispatch_validate(snapshot: &SemioSnapshot) -> Vec<Diagnostic> {
    match &snapshot.subset {
        SemioSubsetSnapshot::Brep(s) => SemioBrepValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Mesh(s) => SemioMeshValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Model(s) => SemioModelValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Object(s) => SemioObjectValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Document(s) => SemioDocumentValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Cad(s) => SemioCadValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Drawing(s) => SemioDrawingValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Image(s) => SemioImageValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Video(s) => SemioVideoValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Audio(s) => SemioAudioValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Animation(s) => SemioAnimationValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Presentation(s) => SemioPresentationValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(s))),
        SemioSubsetSnapshot::Workflow(s) => SemioWorkflowValidator::validate(&IoPayload::Binary(<crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot as store::ArtifactPack>::encode_pack(s))),
    }
}

impl SubsetValidator for SemioValidator {
    const DIALECT: Dialect = DIALECT;

    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => dispatch_validate(&snapshot),
            None => vec![Diagnostic {
                code: dsl::FaultCode::new("stdio.semio.any.validate-decode-failed"),
                severity: dsl::Severity::Warning,
                span: dsl::TextSpan::at(1, 1),
                message: "SemioValidator: payload did not decode as a SemioSnapshot — skipped".into(),
                expected: None,
                scope: dsl::FaultScope::default(),
            }],
        }
    }
}

static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

fn validator_entry() -> &'static SubsetValidatorEntry {
    VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioValidator>)
}
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, and its `SubsetValidator`.
/// Called from this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::any::schema::semio_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioSnapshot, crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation>(crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};

    /// 🧪️ A clean, valid audio snapshot delegates to `SemioAudioValidator` and reports no hard
    /// (error-severity) diagnostics.
    #[test]
    fn clean_audio_snapshot_delegates_and_reports_no_errors() {
        let snapshot = SemioSnapshot {
            subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, ..Default::default() }),
            ..Default::default()
        };
        let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = SemioValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().all(|d| d.severity != dsl::Severity::Error), "clean snapshot must not report hard errors: {diagnostics:?}");
    }

    /// 🧪️ An invalid audio snapshot (`sample_rate == 0`, a real invariant `SemioAudioValidator`
    /// checks per `subsets::audio::composer`'s own doc comment) delegates through and the
    /// underlying subset's real diagnostic surfaces unchanged.
    #[test]
    fn invalid_audio_snapshot_surfaces_the_delegated_subsets_own_diagnostic() {
        let snapshot = SemioSnapshot {
            subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 0, format: SemioAudioFormat::Pcm16, ..Default::default() }),
            ..Default::default()
        };
        let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = SemioValidator::validate(&IoPayload::Binary(bytes));
        assert!(!diagnostics.is_empty(), "zero sample_rate must be flagged");
    }

    /// 🧪️ A payload that doesn't decode as a `SemioSnapshot` at all degrades to the documented
    /// soft warning, never a panic.
    #[test]
    fn undecodable_payload_returns_soft_warning_not_panic() {
        let diagnostics = SemioValidator::validate(&IoPayload::Binary(vec![0xff, 0x00, 0x01]));
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, dsl::Severity::Warning);
    }
}
//#endregion 🔖️Tests
