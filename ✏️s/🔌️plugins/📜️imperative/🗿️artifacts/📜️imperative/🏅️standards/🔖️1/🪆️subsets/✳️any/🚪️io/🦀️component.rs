//! 🚪️ IO s.imperative (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().
pub async fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.txt"] }
pub async fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.txt"] }

//#region 🔖️Bootstrap
/// 🧩️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
/// Placed beside `io_registry` rather than in the artifact root or the app: it has THREE callers across
/// two layers — the artifact root's `declaration()` (must warm the native-module registry before
/// building composers/inferences), the app's `🎚️config` (`crate::artifacts::imperative::io::default_imperative_contributions_json`),
/// and the app engine's `ImperativeHost::from_snapshot`. An artifact must not depend on its app, so this
/// stays artifact-side where both the root and the app can reach it by qualified path.
pub async fn default_imperative_contributions_json() -> String {
    static ENTRIES: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            let entries = vec![
                crate::extensions::effect::imperative_module_contribution(),
                crate::extensions::math::imperative_module_contribution(),
                crate::extensions::text::imperative_module_contribution(),
                crate::extensions::logic::imperative_module_contribution(),
                crate::extensions::control::imperative_module_contribution(),
            ];
            imperative_engine::contributions_json_from_entries(&entries)
        })
        .clone()
}

/// 🧵️ `Once`-guarded native-module registration — see `default_imperative_contributions_json`'s own
/// doc for why this lives here rather than the artifact root or the app. Widened to `pub` (was
/// `pub(crate)` while confined to the artifact crate) because the app engine's `ImperativeHost` is now
/// its second caller.
pub async fn bootstrap_imperative_runtime() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        imperative_engine::register_native_imperative_module("imperative-extension-core", crate::extensions::effect::register);
        imperative_engine::register_native_imperative_module("imperative-extension-math", crate::extensions::math::register);
        imperative_engine::register_native_imperative_module("imperative-extension-text", crate::extensions::text::register);
        imperative_engine::register_native_imperative_module("imperative-extension-logic", crate::extensions::logic::register);
        imperative_engine::register_default_imperative_contributions(default_imperative_contributions_json);
        imperative_engine::sync_imperative_module_contributions(&default_imperative_contributions_json());
    });
}
//#endregion 🔖️Bootstrap

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::imperative::ImperativeSnapshot;
    use crate::artifacts::imperative::standards::v1::subsets::any::schema::ImperativeAnalyzer;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_CSV: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_MD: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


    pub struct ImperativeComposerComposition;

    impl ArtifactComposition for ImperativeComposerComposition {
        type Snapshot = ImperativeSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_CSV, DEP_JSON, DEP_MD, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = ImperativeAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_CSV {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::imperative::io::import::deserializers::artifacts::csv::v_rfc4180::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::imperative::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_MD {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::imperative::io::import::deserializers::artifacts::md::v_commonmark::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::imperative::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }

            }
            Err(ComposeError { message: "ImperativeComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
/// The artifact root's own `io_registry` (a `&'static [&'static ComposerEntry]` view) wraps THIS
/// module's `entries()` (`&'static [ComposerEntry]`, owning storage) — deliberately different return
/// types; do not conflate them when qualifying paths.
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::imperative::standards::v1::subsets::any::schema::ImperativeComposer as ImperativeAnyComposer;
    use crate::artifacts::imperative::standards::v1::subsets::any::schema::ImperativeBuilder as ImperativeAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const IMPERATIVE_DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };
    const IMPERATIVE_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::imperative::ImperativeSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == IMPERATIVE_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => ImperativeAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => ImperativeAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "ImperativeComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == IMPERATIVE_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::imperative::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "ImperativeComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    async fn compose_export_csv(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::imperative::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    async fn compose_export_md(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::imperative::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    async fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::imperative::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    //#endregion 🔖️ExportEntries

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<ImperativeAnyComposer>(),
            ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[IMPERATIVE_DIALECT], compose: compose_export_csv },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[IMPERATIVE_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[IMPERATIVE_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
