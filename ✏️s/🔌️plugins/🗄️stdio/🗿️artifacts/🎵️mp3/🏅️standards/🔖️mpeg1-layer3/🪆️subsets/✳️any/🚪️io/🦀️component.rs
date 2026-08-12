//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{
        ArtifactComposition, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    };
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::Mp3Analyzer;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Mp3ComposerComposition;

    impl ArtifactComposition for Mp3ComposerComposition {
        type Snapshot = Mp3Snapshot;
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
                return Err(ComposeError { message: "Mp3ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Mp3Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "Mp3ComposerComposition: analysis produced no snapshot".into(),
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
        ::schema::register_artifact_schema_descriptor(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mp3_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<Mp3Snapshot, crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation>(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::STDIO_MP3_DOCUMENT_SCHEMA));
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
