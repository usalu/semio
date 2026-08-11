//! 🎹️ TsvComposer (s.stdio.tsv/iana/*) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use crate::artifacts::tsv::standards::iana::subsets::any::analyzer::TsvAnalyzer;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct TsvComposer;

impl ArtifactComposer for TsvComposer {
    type Snapshot = TsvSnapshot;
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
            return Err(ComposeError { message: "TsvComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = TsvAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "TsvComposer: analysis produced no snapshot".into(),
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
    ::schema::register_artifact_schema_descriptor(crate::artifacts::tsv::standards::iana::subsets::any::schema::tsv_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<TsvSnapshot, crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation>(crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::STDIO_TSV_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Register
