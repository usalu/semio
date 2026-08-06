//! ⚙️ Puzzle 5d artifact — headless compute over a puzzle-5d scene: the `Puzzle5dPrecomputeSession`
//! wrapper that delegates every brush/fill computation to the 3d artifact's own engine (the 5d
//! document is the unification of a 2d board and a 3d world, so its collision/placement solver IS
//! puzzle3d's), the shared grip-kind compatibility lookup both projections honor, and the empty/id
//! document helpers.
//!
//! 📚️ Sibling topic files: `🦀️transfer.rs` (the copy/paste closure rules and the translate/replace-kind
//! helpers), `🦀️compose.rs` (the semio-compose Design → `Puzzle5dProjection` importer).
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here. Helpers taking an app-only view-state
//! type (`Puzzle5dConfig`, `Puzzle5dScene`) never come here — artifacts must not depend on apps.

use crate::artifacts::puzzle3d::Puzzle3dError;
use crate::artifacts::puzzle5d::{Puzzle5dError, Puzzle5dProjection};
use std::collections::HashSet;

//#region 🔖️Reexports
// 🧩️ The sibling topic modules are declared (with their `#[path]`s) in the plugin-root `📦️glue.rs`,
// beside every other taxonomy component; these re-exports keep the whole engine surface reachable
// under one `crate::artifacts::puzzle5d::engine::…` name regardless of which topic file owns it.
pub use crate::artifacts::puzzle5d::engine::compose::import_compose_design_json;
pub use crate::artifacts::puzzle5d::engine::transfer::{centroid_2d, copy_selection, find_replaceable_kinds, paste_selection, translate_parts};
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
pub fn empty_puzzle5d_projection() -> Puzzle5dProjection {
    Puzzle5dProjection::default()
}

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
/// 🔤️ Parses `.puzzle5d` DSL text (`Puzzle5dProjection`'s `dsl::DslDocument` grammar) into the same
/// camelCase JSON shape callers previously got from a hand-authored `*.5d.json` fixture — lets
/// non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the
/// DSL grammar.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = puzzle5dParseDslJson)]
pub fn puzzle5d_parse_dsl_json(dsl_text: &str) -> Result<String, wasm_bindgen::JsValue> {
    use store::DocumentDsl;
    let projection = Puzzle5dProjection::parse_dsl(dsl_text).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
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
