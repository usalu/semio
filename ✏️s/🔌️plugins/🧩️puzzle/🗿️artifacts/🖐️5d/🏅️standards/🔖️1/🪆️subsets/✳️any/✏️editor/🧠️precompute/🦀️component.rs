//! 🧠️ Puzzle 5d play app — the `Puzzle5dPrecomputeSession` wrapper that delegates every brush/fill
//! computation to the 3d artifact's own engine (the 5d document is the unification of a 2d board and
//! a 3d world, so its collision/placement solver IS puzzle3d's).
//!
//! 🚚️ Relocated from the deleted artifact-side `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a per-editing-session mutable-state facade
//! over `puzzle3d`'s own precompute session — genuinely app/session-side behaviour (constructed and
//! held as a `RefCell` field on `Puzzle5dPlayApp`, and consumed by the brush option's `measure()` and
//! both `🪟️windows/{◻2d,🧊️3d}`'s `definition()`/`window_measures()`/`render()`), never artifact schema.

use crate::artifacts::puzzle3d::Puzzle3dError;
use crate::artifacts::puzzle5d::Puzzle5dError;

//#region 🔖️BrushEngine
pub use crate::artifacts::puzzle3d::schema::BrushPlacePayload;

/// 🧠️ A puzzle-5d brush/fill precompute session — a thin JSON-string facade over the 3d artifact's
/// `Puzzle3dPrecomputeSession`, which owns the actual collision/placement solver.
pub struct Puzzle5dPrecomputeSession {
    inner: crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession,
}

impl Default for Puzzle5dPrecomputeSession {
    async fn default() -> Self {
        Self::new()
    }
}

impl Puzzle5dPrecomputeSession {
    pub async fn new() -> Self {
        Self { inner: crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession::new() }
    }

    pub async fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.inner.register_mesh(url, positions, indices);
    }

    pub async fn has_mesh(&self, url: &str) -> bool {
        self.inner.has_mesh(url)
    }

    pub async fn precompute_step(&mut self, budget: u32) -> bool {
        self.inner.precompute_step(budget)
    }

    /// 🎯️ The 3d engine's headless-engine-law fix (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`) made
    /// `brush_candidates` typed (`BrushCollisionFreeResult`, not a JSON string) — re-serialized here so
    /// this node's own JSON-string surface for its callers stays unchanged.
    pub async fn brush_candidates(&self, grip_full_id: &str) -> String {
        serde_json::to_string(&self.inner.brush_candidates(grip_full_id)).unwrap_or_else(|_| "{}".to_string())
    }

    pub async fn brush_preview_json(&self, grip_full_id: &str, candidate_index: usize) -> Option<String> {
        self.inner.brush_preview(grip_full_id, candidate_index).and_then(|preview| serde_json::to_string(&preview).ok())
    }

    pub async fn fill_progress(&self) -> String {
        serde_json::to_string(&self.inner.fill_progress()).unwrap_or_else(|_| "{}".to_string())
    }

    /// 🎯️ Extracts the `Fixture` a 3d-engine `dispatch` call produced, re-serialized to the JSON
    /// string this node's own callers (the native `_rust` methods below, and the wasm-bindgen methods
    /// further down) expect — every dispatched command this file issues returns a `Fixture` outcome,
    /// so the `Unit`/`BrushPreview` arms are unreachable in practice.
    async fn fixture_outcome_json(outcome: crate::artifacts::puzzle3d::schema::Puzzle3dEngineOutcome) -> Result<String, Puzzle3dError> {
        match outcome {
            crate::artifacts::puzzle3d::schema::Puzzle3dEngineOutcome::Fixture(fixture) => Ok(serde_json::to_string(&fixture)?),
            _ => Err(Puzzle3dError::BrushPlacementRejected),
        }
    }
}

/// 🧵️ Native/WASI-p2 build: the 3d precompute session's `Puzzle3dError`-typed `_rust`-suffixed API
/// surface is available under this cfg — mirrors the 3d session's own matching split.
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
impl Puzzle5dPrecomputeSession {
    pub async fn set_scene(&mut self, json: &str) -> Result<(), Puzzle5dError> {
        Ok(self.inner.set_scene(json)?)
    }

    pub async fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        let payload: BrushPlacePayload = serde_json::from_str(payload_json).map_err(Puzzle3dError::from)?;
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::schema::Puzzle3dEngineCommand::ApplyBrushPlacement { payload })?)?)
    }

    pub async fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::schema::Puzzle3dEngineCommand::ApplyFillCount { count })?)?)
    }
}

/// 🌐️ Browser wasm-bindgen build (wasm32, non-p2): the 3d precompute session's `JsValue`-typed API
/// surface is available instead — mirrors those method names/signatures 1:1 so callers on this
/// target get the same capability.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl Puzzle5dPrecomputeSession {
    pub async fn set_scene(&mut self, json: &str) -> Result<(), wasm_bindgen::JsValue> {
        self.inner.set_scene(json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
    }

    pub async fn apply_brush_placement_json(&mut self, payload_json: &str) -> Result<String, wasm_bindgen::JsValue> {
        self.apply_brush_placement_rust(payload_json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
    }

    pub async fn apply_fill_count(&mut self, count: u32) -> Result<String, wasm_bindgen::JsValue> {
        self.apply_fill_count_rust(count).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
    }

    pub async fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle5dError> {
        let payload: BrushPlacePayload = serde_json::from_str(payload_json).map_err(Puzzle3dError::from)?;
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::schema::Puzzle3dEngineCommand::ApplyBrushPlacement { payload })?)?)
    }

    pub async fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(Self::fixture_outcome_json(self.inner.dispatch(crate::artifacts::puzzle3d::schema::Puzzle3dEngineCommand::ApplyFillCount { count })?)?)
    }
}
//#endregion 🔖️BrushEngine
