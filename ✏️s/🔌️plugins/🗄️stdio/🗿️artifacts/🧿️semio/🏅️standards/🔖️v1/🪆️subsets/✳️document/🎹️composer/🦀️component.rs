//! 🎹️ SemioDocumentComposer (s.stdio.semio/v1/document) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::analyzer::SemioDocumentAnalyzer;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

//#region 🔖️Composer
pub struct SemioDocumentComposer;

impl ArtifactComposer for SemioDocumentComposer {
    type Snapshot = SemioDocumentSnapshot;
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
            return Err(ComposeError { message: "SemioDocumentComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioDocumentAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioDocumentComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ 🚧 scaffolded by W1b — decode-only validator (no referential-invariant diagnostics yet;
/// W2 adds real cross-reference checks).
pub struct SemioDocumentValidator;

impl SubsetValidator for SemioDocumentValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(_) => Vec::new(),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_document.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioDocumentValidator: payload did not decode as a SemioDocumentSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDocumentValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
/// this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioDocumentSnapshot, crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>(crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
}
//#endregion 🔖️Register
