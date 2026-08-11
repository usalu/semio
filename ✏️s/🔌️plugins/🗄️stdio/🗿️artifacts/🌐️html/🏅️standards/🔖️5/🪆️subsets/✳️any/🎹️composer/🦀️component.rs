//! 🎹️ HtmlComposer (s.stdio.html/5/*) — 🚧 scaffolded by W1b:
//! analyzer-only compose (decodes the subset's own JSON-pack payload). W2/W4 add real
//! cross-format compose sources once semio↔format import/export leaves land.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use crate::artifacts::html::standards::v5::subsets::any::analyzer::HtmlAnalyzer;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };

//#region 🔖️Composer
pub struct HtmlComposer;

impl ArtifactComposer for HtmlComposer {
    type Snapshot = HtmlSnapshot;
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
            return Err(ComposeError { message: "HtmlComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = HtmlAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "HtmlComposer: analysis produced no snapshot".into(),
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
    ::schema::register_artifact_schema_descriptor(crate::artifacts::html::standards::v5::subsets::any::schema::html_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<HtmlSnapshot, crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation>(crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::STDIO_HTML_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Register
