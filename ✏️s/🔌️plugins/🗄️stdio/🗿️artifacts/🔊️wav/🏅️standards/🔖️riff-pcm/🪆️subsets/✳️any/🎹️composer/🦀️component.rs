//! 🎹️ WavComposer (s.stdio.wav/riff-pcm/*) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::analyzer::WavAnalyzer;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct WavComposer;

impl ArtifactComposer for WavComposer {
    type Snapshot = WavSnapshot;
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
            return Err(ComposeError { message: "WavComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = WavAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "WavComposer: analysis produced no snapshot".into(),
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
    ::schema::register_artifact_schema_descriptor(crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<WavSnapshot, crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation>(crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::STDIO_WAV_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Register
