//! 🎹️ SemioComposer (s.stdio.semio/v1/*) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
};
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::analyzer::SemioAnalyzer;

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

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec. Called from
/// this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::any::schema::semio_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioSnapshot, crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation>(crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Register
