//! 🎹️ AviComposer (s.stdio.avi/1.0/*) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::artifacts::avi::standards::v1_0::subsets::any::analyzer::AviAnalyzer;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct AviComposer;

impl ArtifactComposer for AviComposer {
    type Snapshot = AviSnapshot;
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
            return Err(ComposeError { message: "AviComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = AviAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "AviComposer: analysis produced no snapshot".into(),
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
    ::schema::register_artifact_schema_descriptor(crate::artifacts::avi::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<AviSnapshot, crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation>(crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Register
