//! ⚙️ `trinity.graph` artifact — headless compute over the projection (constitutional: engine).
//!
//! 📌️ The jack query-language compute itself (`run_jack_query` and friends) lives in the plugin's
//! `🫀️core` cross-artifact kernel — used by both the `jack` app's UI and the `rewrite` app's
//! `apply_rule` — not here. This file holds the one document-level pure helper the old bundle crate's
//! `⚙️engine` module also held.

use crate::artifacts::jack::{empty_trinity_graph_fixture, JackSnapshot};

/// 📦️ An empty trinity graph fixture — the app's zero-state initial document.
pub fn empty_jack_document() -> JackSnapshot {
    empty_trinity_graph_fixture()
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_jack_document_has_no_nodes_or_edges() {
        let fixture = empty_jack_document();
        assert!(fixture.nodes.is_empty());
        assert!(fixture.edges.is_empty());
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.document",
        extension: Some("trinity"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::jack::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::jack::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::jack::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::jack::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::jack::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::jack::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("jack.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::jack::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "jack.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::jack::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("jack.spr"),
    });
}



//#region 🔖️Register
/// 📎 Registers the artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::jack::schema::jack_artifact_schema_descriptor());
}

/// 🗂️ Registers codecs and schema descriptor.
pub fn register() {
    crate::artifacts::jack::io_registry::register();

    register_pilot_languages();
    register_artifact_schema();
    crate::apps::jack::config::schema::register_app_schema();
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
pub struct TrinityGraphEngine {
    artifact: crate::artifacts::jack::schema::JackArtifact,
    snapshot: crate::artifacts::jack::JackSnapshot,
}

impl TrinityGraphEngine {
    pub fn new(snapshot: crate::artifacts::jack::JackSnapshot) -> Self {
        Self {
            artifact: crate::artifacts::jack::schema::JackArtifact::from_snapshot(snapshot.clone()),
            snapshot,
        }
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::jack::standards::v1::subsets::any::schema::JackComposer as JackAnyComposer;
    use crate::artifacts::jack::standards::v1::subsets::any::schema::JackBuilder as JackAnyBuilder;

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
    const JACK_DIALECT: Dialect = Dialect { artifact_kind: "s.jack", standard: StandardId("1"), subset: SubsetId("*") };
    const JACK_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::jack::JackSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == JACK_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => JackAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => JackAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "JackComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == JACK_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::jack::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "JackComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::jack::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    fn compose_export_csv(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::jack::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    fn compose_export_md(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::jack::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::jack::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::jack::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<JackAnyComposer>(),
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[JACK_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[JACK_DIALECT], compose: compose_export_csv },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[JACK_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[JACK_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[JACK_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
