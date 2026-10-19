//! 🧵️ Precompute session: the scene the host syncs in and `dispatch`, which drives `Puzzle3dEngineCommand` and `Puzzle3dEngineOutcome` through the collision engine.

use super::*;

//#region 🔖️Session
pub struct Puzzle3dPrecomputeSession {
    pub(super) engine: Puzzle3dCollision,
}

impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Puzzle3dPrecomputeSession {
    #[cfg(any(test, feature = "testkit"))]
    /// 📊️ See [`Puzzle3dCollision::brush_candidate_cache_len`].
    pub fn brush_candidate_cache_len(&self) -> usize {
        self.engine.brush_candidate_cache_len()
    }

    pub fn new() -> Self {
        Self { engine: Puzzle3dCollision::new() }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.set_scene_config(semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?);
        Ok(())
    }

    /// 🧊️ Typed sync — see [`Puzzle3dCollision::set_scene_config`].
    pub fn set_scene_config(&mut self, scene: SceneConfig) {
        self.engine.set_scene_config(scene);
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions, indices);
    }

    pub fn register_mesh_fallback(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh_fallback(url.to_string(), positions, indices);
    }

    /// 🧩️ Admits one page of a client's mesh upload run and installs the geometry the moment the run
    /// closes — the paged half of the `registerBrushMesh` wire contract, which is how a document-scale
    /// GLB reaches this session at all: one whole mesh never fitted in the shared retained command's
    /// 8 192 raw bytes. `Ok(None)` is a closed, digest-verified, installed mesh; `Ok(Some(next))` is an
    /// open run waiting for page `next`. See [`stage_brush_mesh_page`].
    pub fn stage_mesh_page(&mut self, url: &str, digest: &str, page: u32, page_count: u32, positions: &[f32], indices: &[u32]) -> Result<Option<u32>, Puzzle3dMeshUploadFault> {
        if page == 0 && !digest.is_empty() && self.adopt_shared_mesh(url, Some(digest)) {
            return Ok(None);
        }
        match stage_brush_mesh_page(url, digest, page, page_count, positions, indices)? {
            Puzzle3dMeshUploadStep::Staged { next_page, .. } => Ok(Some(next_page)),
            Puzzle3dMeshUploadStep::Complete(positions, indices) => {
                self.register_mesh(url, &positions, &indices);
                Ok(None)
            }
        }
    }

    /// 🧵️ Hands the brush lane out to the app's session slot, leaving this engine empty — the check-in
    /// half of the per-instance session.
    pub fn take_collision_session(&mut self) -> Puzzle3dCollisionSession {
        self.engine.take_session()
    }

    /// 🧵️ Adopts a brush lane a previous call (possibly on another worker) left in the session slot.
    pub fn install_collision_session(&mut self, session: Puzzle3dCollisionSession) {
        self.engine.install_session(session);
    }

    /// 🥽️ Installs real geometry for one mesh identity out of the process-wide derived-mesh store —
    /// the id-only wire path, so a mesh uploaded once serves every document instance.
    pub fn adopt_shared_mesh(&mut self, url: &str, digest: Option<&str>) -> bool {
        self.engine.adopt_shared_mesh(url, digest)
    }

    /// 🚚️ Turns a refused id-only announcement into a standing request for the bytes — see
    /// [`Puzzle3dCollision::request_mesh_reupload`]. Answers whether the standing set actually GREW, which
    /// is the only case whose world body has anything new to say.
    pub fn request_mesh_reupload(&mut self, url: &str) -> bool {
        self.engine.request_mesh_reupload(url)
    }

    /// 🚚️ The mesh ids this session is waiting on bytes for, stable-ordered — see
    /// [`Puzzle3dCollision::mesh_reupload_requests`].
    pub fn mesh_reupload_requests(&self) -> &[String] {
        self.engine.mesh_reupload_requests()
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.engine.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn enqueue_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.enqueue_brush_target(vortex_full_id);
    }

    pub fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.invalidate_brush_target(vortex_full_id);
    }

    pub fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        self.engine.refresh_brush_candidates(vortex_full_id);
    }

    /// 🎯️ Typed readout — was a JSON string before the headless-engine-law fix; the app now reads
    /// `.free`/`.unknown_pending` directly.
    pub fn brush_candidates(&self, vortex_full_id: &str) -> BrushCollisionFreeResult {
        self.engine.brush_cache.get(vortex_full_id).cloned().unwrap_or(BrushCollisionFreeResult { free: vec![], unknown_pending: true, resume_candidate_index: 0 })
    }

    pub fn brush_preview(&self, vortex_full_id: &str, candidate_index: usize) -> Option<BrushPreviewState> {
        self.engine.brush_preview(vortex_full_id, candidate_index)
    }

    /// 🎯️ Single typed entry point for every mutating engine action — the headless replacement for the
    /// old per-action `update_kind_weights`/`brush_preview_json` wasm-bindgen
    /// methods. Each arm calls the SAME underlying typed `Puzzle3dCollision` method those JSON wrappers
    /// always delegated to — no reimplementation.
    pub fn dispatch(&mut self, command: Puzzle3dEngineCommand) -> Result<Puzzle3dEngineOutcome, Puzzle3dError> {
        match command {
            Puzzle3dEngineCommand::SetScene { scene } => {
                self.set_scene_config(scene);
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights } => {
                self.engine.update_kind_weights(object_weights, vortex_weights);
                Ok(Puzzle3dEngineOutcome::Unit)
            }
            Puzzle3dEngineCommand::BrushPreview { vortex_full_id, candidate_index } => Ok(Puzzle3dEngineOutcome::BrushPreview(self.engine.brush_preview(&vortex_full_id, candidate_index as usize))),
        }
    }
}
//#endregion 🔖️Session
