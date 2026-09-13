//! 🧠️ Puzzle 5d play app — the `Puzzle5dPrecomputeSession` wrapper that delegates every brush/fill
//! computation to the 3d artifact's own engine (the 5d document is the unification of a 2d board and
//! a 3d world, so its collision/placement solver IS puzzle3d's).
//!
//! 🚚️ Relocated from the deleted artifact-side `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a per-editing-session mutable-state facade
//! over `puzzle3d`'s own precompute session — genuinely app/session-side behaviour (constructed and
//! held as a `RefCell` field on `Puzzle5dPlayApp`, and consumed by the brush option's `measure()` and
//! both `🪟️windows/{◻️2d,🧊️3d}`'s `definition()`/`window_measures()`/`render()`), never artifact schema.

use semio_s_artifact_puzzle_3d::Puzzle3dError;
use crate::Puzzle5dError;

//#region 🔖️BrushEngine
pub use semio_s_artifact_puzzle_3d::BrushPlacePayload;

/// ⏳️ What the wrapped 3d fill run has done so far, in puzzle-5d's own vocabulary. Declared here
/// rather than re-exported from the 3d artifact so 5d's public surface names only 5d types: the 3d
/// summary is an implementation detail of the engine this node delegates to, and a caller that had
/// to name it would be coupled to a crate it never asked for.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle5dFillProgress {
    /// 🎯️ Placements the planner has accepted.
    pub count: usize,
    /// 📦️ Placements already in the document.
    pub applied_count: usize,
    /// 🙋️ What the operator asked for — never a planner ceiling.
    pub requested_count: usize,
    pub done: bool,
    pub tested: u64,
    pub rejected: u64,
    pub collisions: u64,
    /// 🧭️ Machine token of the phase the run is in; `terminology::puzzle5d_fill_stage_label` localizes it.
    pub stage: String,
    /// 🛑️ Machine token of why the run stopped short of the requested count, when it did.
    pub stall_reason: Option<String>,
}

/// 🧠️ A puzzle-5d brush/fill precompute session over the 3d app's retained solver and preview page.
pub struct Puzzle5dPrecomputeSession {
    inner: semio_s_artifact_puzzle_3d::Puzzle3dPrecomputeSession,
}

impl Default for Puzzle5dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle5dPrecomputeSession {
    pub fn new() -> Self {
        Self { inner: semio_s_artifact_puzzle_3d::Puzzle3dPrecomputeSession::new() }
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
        dsl::json::to_json_string(&self.inner.brush_candidates(grip_full_id))
    }

    pub fn brush_preview_json(&self, grip_full_id: &str, candidate_index: usize) -> Option<String> {
        self.inner.brush_preview(grip_full_id, candidate_index).map(|preview| dsl::json::to_json_string(&preview))
    }

    pub fn fill_preview_object_kind(&self) -> Option<String> {
        self.inner.fill_preview_object_kind()
    }

    /// ⏳️ The wrapped 3d run's live progress, projected onto 5d's own vocabulary. 5d's fill is the
    /// 3d planner — so the option panel reads the very same counters the 3d tool does instead of
    /// publishing an empty slider beside a solver that knows exactly where it is.
    pub fn fill_progress(&self) -> Puzzle5dFillProgress {
        let summary = self.inner.fill_progress_summary();
        Puzzle5dFillProgress {
            count: summary.count,
            applied_count: summary.applied_count,
            requested_count: summary.max_count,
            done: summary.done,
            tested: summary.tested,
            rejected: summary.rejected,
            collisions: summary.collisions,
            stage: summary.stage,
            stall_reason: summary.stall_reason,
        }
    }

    /// 🎚️ The count the wrapped plan is currently held to.
    pub fn fill_requested_count(&self) -> u32 {
        self.inner.fill_requested_count()
    }

    /// 🎚️ Retargets the wrapped plan, idempotently. Raising continues the same deterministic sequence
    /// from what is already locked and revives a run that reported itself done; lowering arms the
    /// document-tail deletion. Without this the 5d session only ever *projected* a count onto a plan
    /// the planner was still holding to its own target.
    pub fn set_fill_requested_count(&mut self, count: u32) {
        self.inner.set_fill_requested_count(count);
    }

    /// 🪪️ `(job, operation, generation)` of the live fill run, when one exists — the identity a
    /// cancel has to carry so a late click cannot kill the run that superseded the one it was
    /// rendered for.
    pub fn fill_job_identity(&self) -> Option<(u64, u64, u64)> {
        self.inner.fill_job_identity()
    }

    /// 🛑️ Cancels the identified fill run; a mismatched identity is a no-op, not a kill.
    pub fn cancel_fill_job_for(&mut self, job: u64, operation: u64, generation: u64) -> bool {
        self.inner.cancel_fill_job_for(job, operation, generation)
    }

    pub fn fill_preview_json_page(&self, color: &str, status_label: &str) -> Option<String> {
        self.inner.fill_preview_json_page(color, status_label)
    }

    /// 🎯️ Extracts the `Fixture` a 3d-engine `dispatch` call produced, re-serialized to the JSON
    /// string this node's own callers (the native `_rust` methods below, and the wasm-bindgen methods
    /// further down) expect — every dispatched command this file issues returns a `Fixture` outcome,
    /// so the `Unit`/`BrushPreview` arms are unreachable in practice.
    fn fixture_outcome_json(outcome: semio_s_artifact_puzzle_3d::Puzzle3dEngineOutcome) -> Result<String, Puzzle3dError> {
        match outcome {
            semio_s_artifact_puzzle_3d::Puzzle3dEngineOutcome::Fixture(fixture) => Ok(dsl::json::to_json_string(&fixture)),
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
        let payload: BrushPlacePayload = dsl::json::from_json_str(payload_json).map_err(Puzzle3dError::from)?;
        Ok(Self::fixture_outcome_json(self.inner.dispatch(semio_s_artifact_puzzle_3d::Puzzle3dEngineCommand::ApplyBrushPlacement { payload })?)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(Self::fixture_outcome_json(self.inner.dispatch(semio_s_artifact_puzzle_3d::Puzzle3dEngineCommand::ApplyFillCount { count })?)?)
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
        let payload: BrushPlacePayload = dsl::json::from_json_str(payload_json).map_err(Puzzle3dError::from)?;
        Ok(Self::fixture_outcome_json(self.inner.dispatch(semio_s_artifact_puzzle_3d::Puzzle3dEngineCommand::ApplyBrushPlacement { payload })?)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle5dError> {
        Ok(Self::fixture_outcome_json(self.inner.dispatch(semio_s_artifact_puzzle_3d::Puzzle3dEngineCommand::ApplyFillCount { count })?)?)
    }
}
//#endregion 🔖️BrushEngine
