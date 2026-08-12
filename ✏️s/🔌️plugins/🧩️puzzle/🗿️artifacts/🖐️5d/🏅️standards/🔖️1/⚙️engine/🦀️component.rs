//! ⚙️ Puzzle 5d artifact — headless compute over a puzzle-5d scene: the `Puzzle5dPrecomputeSession`
//! wrapper that delegates every brush/fill computation to the 3d artifact's own engine (the 5d
//! document is the unification of a 2d board and a 3d world, so its collision/placement solver IS
//! puzzle3d's), the shared grip-kind compatibility lookup both projections honor, and the empty/id
//! document helpers.
//!
//! 📚️ Sibling topic files: `🦀️transfer.rs` (the copy/paste closure rules and the translate/replace-kind
//! helpers). Compose design import was removed in PUZZLE-DESIGN-PARITY Wave 1 (parity harness is Wave 5).
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here. Helpers taking an app-only view-state
//! type (`Puzzle5dConfig`, `Puzzle5dScene`) never come here — artifacts must not depend on apps.

use crate::artifacts::puzzle3d::Puzzle3dError;
use crate::artifacts::puzzle5d::{Puzzle5dError, Puzzle5dSnapshot};
use std::collections::HashSet;

//#region 🔖️Reexports
// 🧩️ The sibling topic modules are declared (with their `#[path]`s) in the plugin-root `📦️glue.rs`,
// beside every other taxonomy component; these re-exports keep the whole engine surface reachable
// under one `crate::artifacts::puzzle5d::engine::…` name regardless of which topic file owns it.
pub use crate::artifacts::puzzle5d::engine::transfer::{centroid_2d, copy_selection, find_replaceable_kinds, paste_selection, translate_parts};
pub use crate::artifacts::puzzle5d::engine::flatten::{flatten_snapshot, flatten_snapshot_inplace};
//#endregion 🔖️Reexports

//#region 🔖️BrushEngine
pub use crate::artifacts::puzzle3d::engine::BrushPlacePayload;

/// 🧠️ A puzzle-5d brush/fill precompute session — a thin JSON-string facade over the 3d artifact's
/// `Puzzle3dPrecomputeSession`, which owns the actual collision/placement solver.
pub struct Puzzle5dPrecomputeSession {
    inner: crate::artifacts::puzzle3d::engine::Puzzle3dPrecomputeSession,
}

impl Default for Puzzle5dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle5dPrecomputeSession {
    pub fn new() -> Self {
        Self { inner: crate::artifacts::puzzle3d::engine::Puzzle3dPrecomputeSession::new() }
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.inner.register_mesh(url, positions, indices);
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.inner.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.inner.precompute_step(budget)
    }

    /// 🎯️ The 3d engine's headless-engine-law fix (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`) made
    /// `brush_candidates` typed (`BrushCollisionFreeResult`, not a JSON string) — re-serialized here so
    /// this node's own JSON-string surface for its callers stays unchanged.
    pub fn brush_candidates(&self, grip_full_id: &str) -> String {
        serde_json::to_string(&self.inner.brush_candidates(grip_full_id)).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn brush_preview_json(&self, grip_full_id: &str, candidate_index: usize) -> Option<String> {
        self.inner.brush_preview(grip_full_id, candidate_index).and_then(|preview| serde_json::to_string(&preview).ok())
    }

    pub fn fill_progress(&self) -> String {
        serde_json::to_string(&self.inner.fill_progress()).unwrap_or_else(|_| "{}".to_string())
    }

    /// 🎯️ Extracts the `Fixture` a 3d-engine `dispatch` call produced, re-serialized to the JSON
    /// string this node's own callers (the native `_rust` methods below, and the wasm-bindgen methods
    /// further down) expect — every dispatched command this file issues returns a `Fixture` outcome,
    /// so the `Unit`/`BrushPreview` arms are unreachable in practice.
    fn fixture_outcome_json(outcome: crate::artifacts::puzzle3d::engine::Puzzle3dEngineOutcome) -> Result<String, Puzzle3dError> {
        match outcome {
            crate::artifacts::puzzle3d::engine::Puzzle3dEngineOutcome::Fixture(fixture) => Ok(serde_json::to_string(&fixture)?),
            _ => Err(Puzzle3dError::BrushPlacementRejected),
        }
    }
}

/// 🧵️ Native/WASI-p2 build: the 3d precompute session's `Puzzle3dError`-typed `_rust`-suffixed API
/// surface is available under this cfg — mirrors the 3d session's own matching split.
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
impl Puzzle5dPrecomputeSession {
    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle5dError> {
        Ok(self.inner.set_scene(json)?)
    }

    pub fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        let payload: BrushPlacePayload = serde_json::from_str(payload_json).map_err(Puzzle3dError::from)?;
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::engine::Puzzle3dEngineCommand::ApplyBrushPlacement { payload })?)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::engine::Puzzle3dEngineCommand::ApplyFillCount { count })?)?)
    }
}

/// 🌐️ Browser wasm-bindgen build (wasm32, non-p2): the 3d precompute session's `JsValue`-typed API
/// surface is available instead — mirrors those method names/signatures 1:1 so callers on this
/// target get the same capability.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl Puzzle5dPrecomputeSession {
    pub fn set_scene(&mut self, json: &str) -> Result<(), wasm_bindgen::JsValue> {
        self.inner.set_scene(json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
    }

    pub fn apply_brush_placement_json(&mut self, payload_json: &str) -> Result<String, wasm_bindgen::JsValue> {
        self.apply_brush_placement_rust(payload_json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
    }

    pub fn apply_fill_count(&mut self, count: u32) -> Result<String, wasm_bindgen::JsValue> {
        self.apply_fill_count_rust(count).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
    }

    pub fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        let payload: BrushPlacePayload = serde_json::from_str(payload_json).map_err(Puzzle3dError::from)?;
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::engine::Puzzle3dEngineCommand::ApplyBrushPlacement { payload })?)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::engine::Puzzle3dEngineCommand::ApplyFillCount { count })?)?)
    }
}
//#endregion 🔖️BrushEngine

//#region 🔖️KindCompatibility
pub const PUZZLE5D_DEFAULT_MANIFEST_ID: &str = "puzzle5d-default";

/// 🧲️ Looks up whether two grip kinds are compatible per the `puzzle5d-default` manifest's
/// `kindCompatibility` rows — the single shared table both the 2D board and 3D world honor so
/// brush/fill suggestions agree across projections.
pub fn puzzle5d_grip_kinds_compatible(source_kind: &str, target_kind: &str) -> bool {
    let Some(manifest) = math::graph::manifest::manifest_by_id(PUZZLE5D_DEFAULT_MANIFEST_ID) else {
        return false;
    };
    manifest.kind_compatibility.iter().any(|row| {
        let source = row.get("source").and_then(|value| value.as_str());
        let target = row.get("target").and_then(|value| value.as_str());
        let bidirectional = row.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == Some(source_kind) && target == Some(target_kind)) || (bidirectional && source == Some(target_kind) && target == Some(source_kind))
    })
}
//#endregion 🔖️KindCompatibility

//#region 🔖️DocumentHelpers
pub fn empty_puzzle5d_snapshot() -> Puzzle5dSnapshot {
    Puzzle5dSnapshot::default()
}


//#region ⚠️ComposeImportShim
/// ⚠️ Temporary empty shim after deleting `⚙️engine/🌉️compose/` (PUZZLE-DESIGN-PARITY Wave 1).
//#endregion ⚠️ComposeImportShim


/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️WasmBridge
/// 🔤️ Parses `.puzzle5d` DSL text (`Puzzle5dSnapshot`'s `dsl::DslArtifact` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets
/// non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the
/// DSL grammar.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, wasm_bindgen::JsValue> {
    use store::ArtifactDsl;
    let projection = Puzzle5dSnapshot::parse_dsl(dsl_text).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_grip_kinds_compatible_reads_manifest_rows() {
        assert!(puzzle5d_grip_kinds_compatible("port", "port"));
        assert!(puzzle5d_grip_kinds_compatible("vortex", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("port", "vortex"));
        assert!(!puzzle5d_grip_kinds_compatible("unknown-kind", "port"));
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle5d",
        extension: Some("puzzle5d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::puzzle5d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle5d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("puzzle.puzzle5d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle5d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::puzzle5d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle5d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("puzzle.puzzle5d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle5d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::puzzle5d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle5d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("puzzle.puzzle5d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "5d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("5d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "5d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("5d.spr"),
    });
}


//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent puzzle5d artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct Puzzle5dEngine {
    artifact: crate::artifacts::puzzle5d::schema::Puzzle5dArtifact,
    snapshot: crate::artifacts::puzzle5d::Puzzle5dSnapshot,
}

impl Puzzle5dEngine {
    pub fn new(snapshot: crate::artifacts::puzzle5d::Puzzle5dSnapshot) -> Self {
        let artifact = crate::artifacts::puzzle5d::schema::Puzzle5dArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::puzzle5d::Puzzle5dSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️IoFacet
pub fn register_io() {
    crate::artifacts::puzzle5d::io_registry::register();
}
//#endregion 🔖️IoFacet
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::Puzzle5dComposer as Puzzle5dAnyComposer;
    use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::Puzzle5dBuilder as Puzzle5dAnyBuilder;

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
    const PUZZLE5D_DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle5d", standard: StandardId("1"), subset: SubsetId("*") };
    const PUZZLE5D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::puzzle5d::Puzzle5dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE5D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Puzzle5dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Puzzle5dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Puzzle5dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE5D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::puzzle5d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Puzzle5dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_zip(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle5d::io::export::serializers::artifacts::zip::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_ZIP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle5d::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle5d::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle5d::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle5d::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<Puzzle5dAnyComposer>(),
            ComposerEntry { writes: EXPORT_ZIP_DIALECT, reads: &[PUZZLE5D_DIALECT], compose: compose_export_zip },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PUZZLE5D_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PUZZLE5D_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[PUZZLE5D_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[PUZZLE5D_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
