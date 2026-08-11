//! 🎹️ EpwComposer (s.stdio.epw/energyplus/*) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use crate::artifacts::epw::standards::energyplus::subsets::any::analyzer::EpwAnalyzer;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct EpwComposer;

impl ArtifactComposer for EpwComposer {
    type Snapshot = EpwSnapshot;
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
            return Err(ComposeError { message: "EpwComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = EpwAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "EpwComposer: analysis produced no snapshot".into(),
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
    ::schema::register_artifact_schema_descriptor(crate::artifacts::epw::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<EpwSnapshot, crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation>(crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::STDIO_EPW_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Register
