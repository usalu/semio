//! 📥️ Renderer asset steps: the renderer asset take, reserve, finish and rejection steps, the shared asset bytes, the decode boundary and the asset decode session steps. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl<R: MailboxRuntime> RuntimeMailbox<R> {
    pub fn take_renderer_asset_step(&self) -> Option<RendererAssetFetchOwner> {
        if let Some(owner) = take_next_renderer_asset() {
            return Some(RendererAssetFetchOwner::Shared(owner));
        }
        let Ok(mut runtime) = self.try_lock() else { return None };
        let interaction = runtime.interaction_mut()?;
        let Ok(mut cursor) = self.0.world3d_asset_cursor.try_lock() else { return None };
        let Some(surface_id) = interaction.shell.surfaces.world3d_states.id_at(*cursor).map(str::to_owned) else {
            *cursor = 0;
            return None;
        };
        *cursor += 1;
        let surface_token = interaction.shell.surfaces.world3d_states.token(&surface_id)?;
        let world = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token)?;
        let owner = take_next_world3d_asset(&mut world.assets)?;
        let surface = WorldAssetMetadataId::try_from_str(&surface_id).ok()?;
        Some(RendererAssetFetchOwner::World { surface, surface_token, owner })
    }

    /// 📡️ Admits the response byte credits one fetch declared. Same three-way answer as
    /// [`Self::seal_renderer_asset_response`], and for the same reason: this runs the instant a
    /// `fetch()` resolves, which is just as likely to land inside a live apply as the seal is.
    pub fn reserve_renderer_asset_response(&self, fetch: &mut RendererAssetFetchOwner, byte_credits: usize) -> RendererAssetSealStep {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => {
                if reserve_renderer_asset_response(owner, byte_credits) {
                    RendererAssetSealStep::Granted
                } else {
                    RendererAssetSealStep::Refused("shared asset authority refused the response credits")
                }
            }
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return RendererAssetSealStep::Busy };
                let Some(interaction) = runtime.interaction_mut() else { return RendererAssetSealStep::Busy };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(*surface_token) else { return RendererAssetSealStep::Refused("the world surface generation retired") };
                match reserve_world3d_asset_response(&mut state.assets, owner, byte_credits) {
                    Ok(()) => RendererAssetSealStep::Granted,
                    Err(_) => RendererAssetSealStep::Refused("the world asset authority refused the response credits"),
                }
            }
        }
    }

    pub fn take_completed_renderer_asset_step(&self) -> Option<RendererAssetFetchOwner> {
        if let Some(owner) = take_next_completed_renderer_asset_step() {
            return Some(RendererAssetFetchOwner::Shared(owner));
        }
        let Ok(mut runtime) = self.try_lock() else { return None };
        let interaction = runtime.interaction_mut()?;
        let Ok(mut cursor) = self.0.world3d_asset_decode_cursor.try_lock() else { return None };
        let Some(surface_id) = interaction.shell.surfaces.world3d_states.id_at(*cursor).map(str::to_owned) else {
            *cursor = 0;
            return None;
        };
        *cursor += 1;
        let surface_token = interaction.shell.surfaces.world3d_states.token(&surface_id)?;
        let world = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token)?;
        let owner = take_next_completed_world3d_asset_step(&mut world.assets)?;
        let surface = WorldAssetMetadataId::try_from_str(&surface_id).ok()?;
        Some(RendererAssetFetchOwner::World { surface, surface_token, owner })
    }

    pub fn finish_renderer_asset_owner(&self, fetch: RendererAssetFetchOwner, rejection: Option<(&'static str, WorldAssetRequestKind, String)>) -> Result<(), RendererAssetFetchOwner> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => {
                if matches!(owner.kind(), WorldAssetRequestKind::Glb) {
                    if let Some((detail, _, _)) = rejection {
                        semio_framework_os_renderer_wgpu_icon_host::icon_export::asset::reject(owner.token(), detail);
                    }
                }
                finish_renderer_asset(owner).map_err(RendererAssetFetchOwner::Shared)
            }
            RendererAssetFetchOwner::World { surface, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(interaction) = runtime.interaction_mut() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                if let Some((_, kind, url)) = rejection.filter(|_| !world3d_asset_cancellation_requested(&state.assets, owner.token())) {
                    match kind {
                        WorldAssetRequestKind::Terrain { z, x, y } => {
                            apply_world3d_terrain_tile_bytes(&mut state.terrain, z, x, y, &[]);
                        }
                        _ => mark_world3d_asset_miss(&mut state.assets, &url),
                    }
                }
                finish_world3d_asset(&mut state.assets, owner).map_err(|owner| RendererAssetFetchOwner::World { surface, surface_token, owner })
            }
        }
    }

    /// 🗑️ Answers a decoder's refusal of ONE response. The shared kinds record their own miss right
    /// here — `apply_*` with empty bytes is exactly the miss those lanes already speak — while a
    /// World-owned kind carries its rejection to [`Self::finish_renderer_asset_owner`], which is the
    /// one place holding that surface's state. Printed unconditionally (not behind
    /// `runtime_diagnostics_enabled`): an asset that never paints is a defect report, not a trace.
    pub(super) fn record_rejected_asset(rejection: &(&'static str, WorldAssetRequestKind, String)) {
        let (detail, kind, url) = rejection;
        log_debug(&format!("asset refused: {detail} — url={url}"));
        match *kind {
            WorldAssetRequestKind::UiImage { id } => semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::apply_ui_image_bytes(id.as_str(), url, &[]),
            kind @ WorldAssetRequestKind::MapTile { .. } => semio_framework_os_renderer_wgpu_engine_board::tiled_map::apply_map_tile_bytes(kind, &[]),
            WorldAssetRequestKind::Glb | WorldAssetRequestKind::ReferenceImage | WorldAssetRequestKind::Terrain { .. } => {}
        }
    }

    /// 📥️ Drains the sealed response pages of a `Ready` probe whose kind applies WITHOUT a
    /// `World3dState` — `UiImage` and `MapTile`, the two kinds `RendererAssetFetchOwner::Shared`
    /// carries. `None` means "not one of those kinds"; a collect failure answers an empty payload so
    /// the caller still closes the probe (`apply_*` treats empty bytes as the miss they are) rather
    /// than leaving the lane parked on an undrainable response.
    pub(super) fn take_shared_asset_bytes(probe: &mut RendererAssetProbe) -> Option<Vec<u8>> {
        if !matches!(probe.owner().kind(), WorldAssetRequestKind::UiImage { .. } | WorldAssetRequestKind::MapTile { .. }) {
            return None;
        }
        let owner = match probe.owner_mut() {
            RendererAssetFetchOwner::World { owner, .. } | RendererAssetFetchOwner::Shared(owner) => owner,
        };
        Some(collect_world3d_asset_bytes(owner).unwrap_or_default())
    }

    fn restore_asset_decode_boundary(&self, boundary: (RendererAssetProbe, Option<RendererAssetDecodeResult>)) -> Result<RendererAssetDecodeTurnKind, (RendererAssetProbe, Option<RendererAssetDecodeResult>)> {
        let Ok(mut slot) = self.0.asset_probe.try_lock() else { return Err(boundary) };
        if slot.is_some() {
            return Err(boundary);
        }
        let (mut probe, mut result) = boundary;
        if !matches!(result, Some(RendererAssetDecodeResult::Cancelled | RendererAssetDecodeResult::Fault(_))) {
            match self.renderer_asset_current(probe.owner()) {
                None => return Err((probe, result)),
                Some(false) => result = Some(RendererAssetDecodeResult::Cancelled),
                Some(true) => {}
            }
        }
        let kind = match result {
            Some(RendererAssetDecodeResult::Cancelled) => {
                probe.begin_close();
                RendererAssetDecodeTurnKind::Cancelled
            }
            Some(RendererAssetDecodeResult::Fault(_)) => {
                probe.begin_close();
                RendererAssetDecodeTurnKind::Fault
            }
            _ => RendererAssetDecodeTurnKind::Pending,
        };
        let rejection = matches!(result, Some(RendererAssetDecodeResult::Rejected(_))).then(|| probe.rejection.clone()).flatten();
        *slot = Some(probe);
        drop(slot);
        if let Some(rejection) = rejection {
            Self::record_rejected_asset(&rejection);
        }
        if let Some(RendererAssetDecodeResult::Fault(detail)) = result {
            self.record_frame_fault(detail);
        }
        Ok(kind)
    }

    pub(super) fn pump_asset_decode_session_step(&self, closing: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<RendererAssetDecodeTurnKind>, semio_framework_job::OriginalTurnFailure> {
        let Ok(mut slot) = self.0.asset_decode_session.try_lock() else { return Ok(Some(RendererAssetDecodeTurnKind::Pending)) };
        if let Some(active) = slot.as_mut() {
            let cancelled = if let Some(surface_token) = active.surface_token {
                self.try_lock()
                    .ok()
                    .and_then(|runtime| runtime.interaction().map(|interaction| interaction.shell.surfaces.world3d_states.get_token(surface_token).is_none_or(|state| world3d_asset_cancellation_requested(&state.assets, active.request_token))))
                    .unwrap_or(false)
            } else {
                renderer_asset_io().try_lock().is_ok_and(|authority| authority.cancellation_requested(active.request_token))
            };
            if closing || cancelled {
                active.cancel();
            }
            let wake = active.pump_one(cx)?;
            let mut kind = RendererAssetDecodeTurnKind::Pending;
            if let Some(boundary) = active.take_boundary() {
                match self.restore_asset_decode_boundary(boundary) {
                    Ok(restored) => kind = restored,
                    Err(boundary) => active.boundary = Some(boundary),
                }
            }
            if wake {
                #[cfg(not(target_arch = "wasm32"))]
        if let Ok(waker) = self.0.waker.try_lock() { if let Some(waker) = waker.as_ref() { waker(); } }
            }
            if active.terminal_is_empty() {
                *slot = None;
            }
            return Ok(Some(kind));
        }
        if closing {
            return Ok(None);
        }
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return Ok(Some(RendererAssetDecodeTurnKind::Pending)) };
        let Some(probe) = probe_slot.as_ref() else { return Ok(None) };
        if matches!(probe.phase, RendererAssetProbePhase::Ready | RendererAssetProbePhase::Closing) {
            return Ok(None);
        }
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(Some(RendererAssetDecodeTurnKind::Pending)); }
        cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        cx.consume_fuel(1);
        let Some(probe) = probe_slot.take() else { return Ok(Some(RendererAssetDecodeTurnKind::Pending)) };
        let active = RendererAssetDecodeSession::new(probe, cx);
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(waker) = self.0.waker.try_lock() { if let Some(waker) = waker.as_ref() { waker(); } }
        *slot = Some(active);
        Ok(Some(RendererAssetDecodeTurnKind::Pending))
    }

    /// 🕐️ The answer for a turn whose owner could not yet fund its original authority: pending, with nothing consumed.
    pub fn asset_decode_deferred_turn(&self) -> RendererAssetDecodeTurn {
        RendererAssetDecodeTurn { kind: RendererAssetDecodeTurnKind::Pending, pending: true, detail: None }
    }

    /// ♻️ Closes the live decoder session under the original authority it was admitted with; true once none remains.
    pub fn close_asset_decode_session_step(&self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        Ok(self.pump_asset_decode_session_step(true, cx)?.is_none())
    }

    /// 🧵️ Advances the private asset owner independently of frame construction and presentation.
    pub fn asset_decode_step(&self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<RendererAssetDecodeTurn, semio_framework_job::OriginalTurnFailure> {
        if semio_framework_os_renderer_wgpu_icon_host::icon_export::asset::retire_cancelled_step() {
            return Ok(self.asset_decode_deferred_turn());
        }
        let session = self.pump_asset_decode_session_step(false, cx)?;
        let kind = session.unwrap_or_else(|| if retire_cancelled_renderer_asset_step() { RendererAssetDecodeTurnKind::Pending } else { self.pump_renderer_asset_decode_step() });
        #[cfg(not(target_arch = "wasm32"))]
        if session.is_none() && !matches!(kind, RendererAssetDecodeTurnKind::Idle | RendererAssetDecodeTurnKind::Fault) {
            if let Ok(wake) = self.0.waker.try_lock() {
                if let Some(wake) = wake.as_ref() {
                    wake();
                }
            }
        }
        Ok(RendererAssetDecodeTurn {
            kind,
            pending: kind != RendererAssetDecodeTurnKind::Fault && (kind == RendererAssetDecodeTurnKind::Pending || self.has_pending_asset_decode()),
            detail: (kind == RendererAssetDecodeTurnKind::Fault).then_some("asset decoder failed; see the retained runtime fault"),
        })
    }

}
//#endregion 📮️RuntimeMailbox
