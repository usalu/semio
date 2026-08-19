//! 🚪️ IO s.rewrite (1/✳️any) — registration now flows through this module's own `io_registry::entries()`,
//! wired into `.composers(…)` by the artifact root's `declaration()`, not per-leaf register().
pub async fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"] }
pub async fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"] }
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::rewrite::RewriteSnapshot;
    use crate::artifacts::rewrite::standards::v1::subsets::any::schema::RewriteAnalyzer;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.rewrite", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DOCX: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_MD: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


    pub struct RewriteComposerComposition;

    impl ArtifactComposition for RewriteComposerComposition {
        type Snapshot = RewriteSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DOCX, DEP_JSON, DEP_MD, DEP_PDF, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = RewriteAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DOCX {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::rewrite::io::import::deserializers::artifacts::docx::v_ecma_376::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::rewrite::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_MD {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::rewrite::io::import::deserializers::artifacts::md::v_commonmark::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PDF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::rewrite::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::rewrite::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }

            }
            Err(ComposeError { message: "RewriteComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::rewrite::standards::v1::subsets::any::schema::RewriteComposer as RewriteAnyComposer;
    use crate::artifacts::rewrite::standards::v1::subsets::any::schema::RewriteBuilder as RewriteAnyBuilder;

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
    const REWRITE_DIALECT: Dialect = Dialect { artifact_kind: "s.rewrite", standard: StandardId("1"), subset: SubsetId("*") };
    const REWRITE_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::rewrite::RewriteSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == REWRITE_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => RewriteAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => RewriteAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "RewriteComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == REWRITE_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::rewrite::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "RewriteComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    async fn compose_export_txt(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::rewrite::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    async fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::rewrite::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    const EXPORT_DOCX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    async fn compose_export_docx(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::rewrite::io::export::serializers::artifacts::docx::v_ecma_376::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DOCX_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    async fn compose_export_md(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::rewrite::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    async fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::rewrite::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    })
}
    //#endregion 🔖️ExportEntries


    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<RewriteAnyComposer>(),
            ComposerEntry { writes: EXPORT_TXT_DIALECT, reads: &[REWRITE_DIALECT], compose: compose_export_txt },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[REWRITE_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_DOCX_DIALECT, reads: &[REWRITE_DIALECT], compose: compose_export_docx },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[REWRITE_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[REWRITE_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
