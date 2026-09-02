//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::TsvAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct TsvComposerComposition;

    impl ArtifactComposition for TsvComposerComposition {
        type Snapshot = TsvSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "TsvComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = TsvAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "TsvComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::tsv::standards::iana::subsets::any::schema::tsv_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<TsvSnapshot, crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation>(
            crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::STDIO_TSV_DOCUMENT_SCHEMA,
        ));
    }

    /// 💡️ Registers `s.stdio.tsv.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to the artifact schema descriptor above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::tsv::standards::iana::subsets::any::schema::inferences::tsv_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation, no
/// engine needed.
pub mod io_registry {
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::TsvComposer as TsvRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TsvRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
