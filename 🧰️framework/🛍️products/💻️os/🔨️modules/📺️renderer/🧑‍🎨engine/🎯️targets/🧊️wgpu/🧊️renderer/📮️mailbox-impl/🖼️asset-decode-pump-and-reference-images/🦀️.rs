//! 🖼️ Asset decode pump and reference images: the renderer asset decode pump, the owner return and seal, the close steps, the currency checks, the rejection and the staged reference images. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl<R: MailboxRuntime> RuntimeMailbox<R> {
    pub(super) fn pump_renderer_asset_decode_step(&self) -> RendererAssetDecodeTurnKind {
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return RendererAssetDecodeTurnKind::Pending };
        if probe_slot.is_none() {
            let Some(owner) = self.take_completed_renderer_asset_step() else { return RendererAssetDecodeTurnKind::Idle };
            *probe_slot = Some(RendererAssetProbe::new(owner));
            return RendererAssetDecodeTurnKind::Pending;
        }
        let probe = probe_slot.as_mut().expect("asset probe initialized above");
        if matches!(probe.phase, RendererAssetProbePhase::Ready) {
            match self.renderer_asset_current(probe.owner()) {
                None => return RendererAssetDecodeTurnKind::Pending,
                Some(false) => {
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Cancelled;
                }
                Some(true) => {}
            }
            log_debug_diagnostic(&format!("[TRACE] asset ready kind={:?} url={} bytes={}", probe.owner().kind(), probe.owner().url(), probe.owner().received_bytes()));
            if matches!(probe.owner(), RendererAssetFetchOwner::Shared(_)) && matches!(probe.owner().kind(), WorldAssetRequestKind::Glb) {
                let Some(asset) = probe.take_ready_mesh_asset() else { return RendererAssetDecodeTurnKind::Pending };
                match semio_framework_os_renderer_wgpu_icon_host::icon_export::asset::publish(probe.owner().owner().token(), asset) {
                    Ok(()) => probe.finish_ready_mesh(),
                    Err(asset) => {
                        probe.restore_ready_mesh_asset(asset);
                        probe.begin_close();
                    }
                }
                return RendererAssetDecodeTurnKind::Published;
            }
            if let Some(bytes) = Self::take_shared_asset_bytes(probe) {
                match probe.owner().kind() {
                    WorldAssetRequestKind::UiImage { id } => semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::apply_ui_image_bytes(id.as_str(), probe.owner().url(), &bytes),
                    kind @ WorldAssetRequestKind::MapTile { .. } => semio_framework_os_renderer_wgpu_engine_board::tiled_map::apply_map_tile_bytes(kind, &bytes),
                    _ => {}
                }
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Published;
            }
            let surface_token = match probe.owner() {
                RendererAssetFetchOwner::World { surface_token, .. } => *surface_token,
                RendererAssetFetchOwner::Shared(_) => return RendererAssetDecodeTurnKind::Pending,
            };
            let generation_is_live = {
                let Ok(runtime) = self.try_lock() else { return RendererAssetDecodeTurnKind::Pending };
                let Some(interaction) = runtime.interaction() else { return RendererAssetDecodeTurnKind::Pending };
                interaction.shell.surfaces.world3d_states.get_token(surface_token).is_some()
            };
            if !generation_is_live {
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Pending;
            }
            if let WorldAssetRequestKind::Terrain { z, x, y } = probe.owner().kind() {
                let bytes = match collect_world3d_asset_bytes(match probe.owner_mut() {
                    RendererAssetFetchOwner::World { owner, .. } | RendererAssetFetchOwner::Shared(owner) => owner,
                }) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    }
                };
                let Ok(mut runtime) = self.try_lock() else {
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(interaction) = runtime.interaction_mut() else {
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else {
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Pending;
                };
                if world3d_asset_cancellation_requested(&state.assets, probe.owner().owner().token()) {
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Cancelled;
                }
                apply_world3d_terrain_tile_bytes(&mut state.terrain, z, x, y, &bytes);
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Published;
            }
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            if matches!(probe.owner().kind(), WorldAssetRequestKind::ReferenceImage) {
                let url = probe.owner().url().to_string();
                let token = probe.owner().owner().token();
                let staged = self.take_staged_reference_image(token, &url);
                if let Some(lease) = staged {
                    let Ok(mut runtime) = self.try_lock() else {
                        self.restore_staged_reference_image(token, url, lease);
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    let Some(interaction) = runtime.interaction_mut() else {
                        self.restore_staged_reference_image(token, url, lease);
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    if world3d_asset_cancellation_requested(&state.assets, probe.owner().owner().token()) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    let _ = apply_decoded_reference_raster(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, &url, lease);
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Published;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let Ok(mut runtime) = self.try_lock() else { return RendererAssetDecodeTurnKind::Pending };
                    let Some(interaction) = runtime.interaction_mut() else { return RendererAssetDecodeTurnKind::Pending };
                    let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    if world3d_asset_cancellation_requested(&state.assets, probe.owner().owner().token()) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    mark_world3d_asset_miss(&mut state.assets, &url);
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Published;
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    return self.pump_native_reference_decode(probe, surface_token, token, url);
                }
            }
            let Some(asset) = probe.take_ready_mesh_asset() else { return RendererAssetDecodeTurnKind::Pending };
            let Ok(mut runtime) = self.try_lock() else {
                probe.restore_ready_mesh_asset(asset);
                return RendererAssetDecodeTurnKind::Pending;
            };
            let Some(interaction) = runtime.interaction_mut() else {
                probe.restore_ready_mesh_asset(asset);
                return RendererAssetDecodeTurnKind::Pending;
            };
            let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else {
                probe.restore_ready_mesh_asset(asset);
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Pending;
            };
            if world3d_asset_cancellation_requested(&state.assets, probe.owner().owner().token()) {
                probe.restore_ready_mesh_asset(asset);
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Cancelled;
            }
            match publish_world3d_asset_mesh(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, probe.owner().url(), asset) {
                Ok(()) => {
                    log_debug_diagnostic(&format!("[TRACE] asset mesh published url={}", probe.owner().url()));
                    probe.finish_ready_mesh();
                    return RendererAssetDecodeTurnKind::Published;
                }
                Err(rejected) => {
                    let stale = rejected.fault == WorldDynamicFault::StaleToken;
                    probe.restore_ready_mesh_asset(rejected.value);
                    if stale {
                        probe.begin_close();
                        drop(runtime);
                        drop(probe_slot);
                        self.record_frame_fault("asset mesh publication generation/revision witness was stale");
                        return RendererAssetDecodeTurnKind::Fault;
                    }
                    return RendererAssetDecodeTurnKind::Pending;
                }
            }
        }
        if matches!(probe.phase, RendererAssetProbePhase::Closing) {
            if !probe.close_step() {
                return RendererAssetDecodeTurnKind::Pending;
            }
            let Some(owner) = probe.take_terminal_owner() else { return RendererAssetDecodeTurnKind::Pending };
            match self.finish_renderer_asset_owner(owner, probe.rejection.clone()) {
                Ok(()) => {
                    *probe_slot = None;
                    return RendererAssetDecodeTurnKind::Cancelled;
                }
                Err(owner) => {
                    probe.owner = Some(owner);
                    return RendererAssetDecodeTurnKind::Pending;
                }
            }
        }
        RendererAssetDecodeTurnKind::Pending
    }

    fn close_renderer_asset_probe_step(&self) -> bool {
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return false };
        let Some(probe) = probe_slot.as_mut() else { return true };
        if !probe.close_step() {
            return false;
        }
        let Some(owner) = probe.take_terminal_owner() else { return false };
        match self.finish_renderer_asset_owner(owner, probe.rejection.clone()) {
            Ok(()) => *probe_slot = None,
            Err(owner) => probe.owner = Some(owner),
        }
        false
    }

    pub fn return_renderer_asset_owner(&self, fetch: RendererAssetFetchOwner) -> Result<(), RendererAssetFetchOwner> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => return_renderer_asset(owner).map_err(RendererAssetFetchOwner::Shared),
            RendererAssetFetchOwner::World { surface, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(interaction) = runtime.interaction_mut() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                return_world3d_asset(&mut state.assets, owner).map_err(|owner| RendererAssetFetchOwner::World { surface, surface_token, owner })
            }
        }
    }

    /// 📡️ Seals one fetched response against the authority that admitted it.
    ///
    /// ⚖️ Three of the four ways this can decline are BACK-PRESSURE, not refusals: the runtime lock is
    /// taken, the interaction state is checked out by a live apply, or the surface has not been
    /// re-attached yet. They all used to collapse into one `false`, which the worker raised as
    /// `"asset response could not release its unused byte credits"` — a hard `asset-stream-fault`
    /// that quarantined the whole shell for a lock that was free a millisecond later. Measured on
    /// the puzzle3d playground: the second World3d surface's copy of the concrete-forest GLB sealed
    /// 40 ms into its fetch, straight into an apply that owned `AppInteractionState`, and the black
    /// canvas that followed was read as a mesh-lane bug (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w3d-world3d-glb-url-lane.md`). Only [`RendererAssetSealStep::Refused`] is a verdict about
    /// the RESPONSE; `Busy` asks the caller to come back, which is what the terrain, mesh and
    /// reference-image reservation loops already do one layer up.
    pub fn seal_renderer_asset_response(&self, fetch: &mut RendererAssetFetchOwner) -> RendererAssetSealStep {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => {
                if seal_renderer_asset_response(owner) {
                    RendererAssetSealStep::Granted
                } else {
                    RendererAssetSealStep::Refused("shared asset authority refused the seal witness")
                }
            }
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(mut runtime) = self.0.runtime.try_lock() else { return RendererAssetSealStep::Busy };
                let Some(interaction) = runtime.interaction_mut() else { return RendererAssetSealStep::Busy };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(*surface_token) else { return RendererAssetSealStep::Refused("the world surface generation retired") };
                match seal_world3d_asset_response(&mut state.assets, owner) {
                    Ok(()) => RendererAssetSealStep::Granted,
                    Err(_) => RendererAssetSealStep::Refused("the world asset authority refused the seal witness"),
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn close_native_reference_decode_step(&self, token: Option<WorldAssetRequestToken>) -> bool {
        let Ok(mut slot) = self.0.native_reference_decode.try_lock() else { return false };
        let Some(job) = slot.as_ref() else { return true };
        if token.is_some_and(|token| token != job.token) {
            return true;
        }
        if !matches!(job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), NativeReferenceDecodeCloseStep::Complete) {
            return false;
        }
        *slot = None;
        true
    }

    /// 🧹️ Keeps one closing component's asset authority reachable until its exact borrowers return.
    pub fn close_component_asset_step(&self, target: &semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget) -> bool {
        if !matches!(target.kind, semio_framework_ui_declaration::ui::SurfaceKind::World3d | semio_framework_ui_declaration::ui::SurfaceKind::IconRender) {
            return true;
        }
        let surface_token = {
            let Ok(mut runtime) = self.try_lock() else { return false };
            let Some(interaction) = runtime.interaction_mut() else { return false };
            let Some(token) = interaction.shell.surfaces.world3d_states.token(&target.host_id) else { return true };
            let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(token) else { return false };
            infinite_world::world::begin_world3d_asset_close(&mut state.assets);
            token
        };
        #[cfg(not(target_arch = "wasm32"))]
        if !matches!(self.cancel_native_asset_transport_step(Some(surface_token), None), NativeAssetTransportCloseStep::Terminal) {
            return false;
        }
        {
            let Ok(session) = self.0.asset_decode_session.try_lock() else { return false };
            if let Some(session) = session.as_ref().filter(|session| session.surface_token == Some(surface_token)) {
                session.cancel();
                return false;
            }
        }
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return false };
        if let Some(probe) = probe_slot.as_mut().filter(|probe| matches!(probe.owner(), RendererAssetFetchOwner::World { surface_token: token, .. } if *token == surface_token)) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if !self.close_native_reference_decode_step(Some(probe.owner().owner().token())) {
                    return false;
                }
            }
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            self.discard_staged_reference_image(probe.owner().owner().token());
            if !probe.close_step() {
                return false;
            }
            let Some(owner) = probe.take_terminal_owner() else { return false };
            match self.finish_renderer_asset_owner(owner, probe.rejection.clone()) {
                Ok(()) => *probe_slot = None,
                Err(owner) => {
                    probe.owner = Some(owner);
                    return false;
                }
            }
            return false;
        }
        drop(probe_slot);
        let Ok(mut runtime) = self.try_lock() else { return false };
        let Some(interaction) = runtime.interaction_mut() else { return false };
        let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else { return false };
        infinite_world::world::close_world3d_asset_step(&mut state.assets)
    }

    pub fn close_renderer_asset_step(&self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.pump_asset_decode_session_step(true, cx)?.is_some() {
            return Ok(false);
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.0.native_asset_http_cancel.cancel_now();
        #[cfg(not(target_arch = "wasm32"))]
        if !matches!(self.cancel_native_asset_transport_step(None, None), NativeAssetTransportCloseStep::Terminal) {
            return Ok(false);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !self.close_native_reference_decode_step(None) {
            return Ok(false);
        }
        if !self.close_renderer_asset_probe_step() {
            return Ok(false);
        }
        if self.0.native_asset_fetching.load(Ordering::Acquire) {
            let _ = close_renderer_asset_step();
            return Ok(false);
        }
        if self.pump_native_asset_handoff_step(true).is_some() {
            return Ok(false);
        }
        Ok(close_renderer_asset_step())
    }

    pub fn renderer_asset_current(&self, fetch: &RendererAssetFetchOwner) -> Option<bool> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => renderer_asset_io().try_lock().ok().map(|authority| !authority.cancellation_requested(owner.token())),
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(runtime) = self.try_lock() else { return None };
                let interaction = runtime.interaction()?;
                Some(interaction.shell.surfaces.world3d_states.get_token(*surface_token).is_some_and(|state| !world3d_asset_cancellation_requested(&state.assets, owner.token())))
            }
        }
    }

    pub fn renderer_asset_cancelled(&self, fetch: &RendererAssetFetchOwner) -> bool {
        self.renderer_asset_current(fetch) == Some(false)
    }

    pub fn browser_renderer_asset_current(&self, fetch: &RendererAssetFetchOwner) -> bool {
        self.renderer_asset_current(fetch) != Some(false)
    }

    pub fn reject_renderer_asset(&self, fetch: &RendererAssetFetchOwner) -> bool {
        match fetch {
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return false };
                let Some(interaction) = runtime.interaction_mut() else { return false };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(*surface_token) else { return true };
                if !world3d_asset_cancellation_requested(&state.assets, owner.token()) {
                    mark_world3d_asset_miss(&mut state.assets, owner.url());
                }
            }
            RendererAssetFetchOwner::Shared(owner) => {
                let Ok(authority) = renderer_asset_io().try_lock() else { return false };
                let cancelled = authority.cancellation_requested(owner.token());
                drop(authority);
                if !cancelled {
                    if matches!(owner.kind(), WorldAssetRequestKind::Glb) {
                        semio_framework_os_renderer_wgpu_icon_host::icon_export::asset::reject(owner.token(), "asset transport was unavailable");
                    }
                    Self::record_rejected_asset(&("asset transport was unavailable", owner.kind(), owner.url().to_string()));
                }
            }
        }
        true
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn restore_staged_reference_image(&self, token: WorldAssetRequestToken, url: String, lease: SceneRasterLease) {
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.restore(token, url, lease);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn begin_renderer_reference_image(&self, fetch: &RendererAssetFetchOwner, width: u32, height: u32, digest: &str) -> Result<u8, &'static str> {
        if !matches!(fetch.kind(), WorldAssetRequestKind::ReferenceImage) || !self.browser_renderer_asset_current(fetch) {
            return Err("reference image request retired");
        }
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("reference image source digest is invalid");
        }
        let source_digest = [u64::from_str_radix(&digest[..16], 16).map_err(|_| "reference image source digest is invalid")?, u64::from_str_radix(&digest[16..32], 16).map_err(|_| "reference image source digest is invalid")?];
        let descriptor = SceneRasterDescriptor { width, height, source_digest, source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        if descriptor.byte_len().is_none_or(|bytes| bytes > semio_framework_ui_raster_owner::SCENE_RASTER_ITEM_BYTES) {
            return Err("reference image exceeded fixed RGBA credits");
        }
        let owner = fetch.generation().wrapping_add(fetch.revision().rotate_left(17)).max(1);
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.begin(&self.0.scene_raster_pool, fetch.owner().token(), fetch.url().to_string(), descriptor, owner)
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn push_renderer_reference_image_rows(&self, fetch: &RendererAssetFetchOwner, offset: usize, pixels: &[u8]) -> Result<bool, &'static str> {
        if !matches!(fetch.kind(), WorldAssetRequestKind::ReferenceImage) || !self.browser_renderer_asset_current(fetch) {
            return Err("reference image request retired");
        }
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.push(&self.0.scene_raster_pool, fetch.owner().token(), offset, pixels)
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn seal_renderer_reference_image(&self, fetch: &RendererAssetFetchOwner) -> Result<bool, &'static str> {
        if !matches!(fetch.kind(), WorldAssetRequestKind::ReferenceImage) || !self.browser_renderer_asset_current(fetch) {
            return Err("reference image request retired");
        }
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.seal(&self.0.scene_raster_pool, fetch.owner().token())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn discard_staged_reference_image(&self, token: WorldAssetRequestToken) {
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.discard(&self.0.scene_raster_pool, token);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn take_staged_reference_image(&self, token: WorldAssetRequestToken, url: &str) -> Option<SceneRasterLease> {
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.take(token, url)
    }

}
//#endregion 📮️RuntimeMailbox
