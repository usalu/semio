//! 🎹️ The derived composer composition and the derived io registry entries.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::RasterAnalyzer;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.raster.raster", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_BMP: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GIF: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
    const DEP_JPG: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_TIFF: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

    pub struct RasterComposerComposition;

    impl ArtifactComposition for RasterComposerComposition {
        type Snapshot = RasterSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BMP, DEP_DWG, DEP_GIF, DEP_JPG, DEP_JSON, DEP_PNG, DEP_SVG, DEP_TIFF]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = RasterAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_BMP {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GIF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::gif::v87a::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JPG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::jpg::v_jfif_1_01::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TIFF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::import::deserializers::artifacts::tiff::v6_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "RasterComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated verbatim from `⚙️engine` (rule 5). The artifact root's `declaration()` calls
/// `…::io::io_registry::entries()` (qualified — see that file's own `🔖️Register` region); the root's
/// own shadowing `io_registry` (returns `&'static [&'static ComposerEntry]`, a different type) must
/// never be confused with this module's `entries() -> &'static [ComposerEntry]`.
pub mod io_registry {
    use crate::RasterBuilder as RasterAnyBuilder;
    use crate::RasterComposer as RasterAnyComposer;
    use {semio_framework_plugin::composer_entry_of,semio_framework_plugin::ArtifactBuilder,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use std::sync::OnceLock;

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
    const RASTER_DIALECT: Dialect = Dialect { artifact_kind: "s.raster.raster", standard: StandardId("1"), subset: SubsetId("*") };
    const RASTER_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == RASTER_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => RasterAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => RasterAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "RasterComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == RASTER_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "RasterComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_GIF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
    fn compose_export_gif(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::gif::v87a::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GIF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_JPG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    fn compose_export_jpg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::jpg::v_jfif_1_01::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JPG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_BMP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    fn compose_export_bmp(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_BMP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_TIFF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    fn compose_export_tiff(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::export::serializers::artifacts::tiff::v6_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TIFF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<RasterAnyComposer>(),
                    ComposerEntry { writes: EXPORT_GIF_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_gif },
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_svg },
                    ComposerEntry { writes: EXPORT_JPG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_jpg },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_BMP_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_bmp },
                    ComposerEntry { writes: EXPORT_TIFF_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_tiff },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
