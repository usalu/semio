//! 🎚️ Present cursor and helpers: the atlas upload cursor, the present phase, the opportunity budget, the glass and clip command helpers and the prepared present cursor with its drop. See [`super`].

use super::*;

// #region gpu
#[derive(Clone, Copy)]
pub(super) struct PreparedAtlasUploadCursor {
    pub(super) generation: u64,
    pub(super) upload: usize,
    pub(super) page: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PreparedGpuPresentPhase {
    EnsureTarget,
    ClearScene,
    InitializeComposite,
    Commands,
    SnapshotBackdrop,
    BlurScene,
    CompositeGlass,
    Present,
    Complete,
    Closing,
}

/// ⏱️ One prepared-present opportunity's wall ceiling. A breach names the PHASE it measured and the
/// microseconds it took: a verdict that cannot say what it measured is undiagnosable from the fault
/// banner alone (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) const PREPARED_GPU_OPPORTUNITY_CEILING_US: u64 = 2_000;

/// 🫧 Resolves an authored glass command to one exact region.
pub(super) fn address_prepared_glass_region(region: usize, len: usize) -> Result<usize, usize> {
    if region < len {
        Ok(region)
    } else {
        Err(len)
    }
}

pub(super) fn prepared_glass_command(packet: &PreparedRenderPacket, command: usize) -> Result<&semio_framework_ui_draw_contract::GlassRegion, String> {
    let command = packet.command_pages().get(command).ok_or_else(|| "prepared glass command was missing".to_string())?;
    let Some(DrawMeasureCursor::Glass(region)) = command.draw_cursor() else { return Err("prepared command did not address glass".to_string()) };
    let draw = if command.packet_overlay() { packet.overlay().ok_or_else(|| "prepared glass overlay owner was missing".to_string())? } else { packet.draw() };
    let index = address_prepared_glass_region(region, draw.glass_regions.len()).map_err(|len| format!("prepared glass region cursor was stale: region {region} of {len}"))?;
    draw.glass_regions.get(index).ok_or_else(|| "prepared glass region was missing".to_string())
}

/// ⚖️ Admits ONE measured opportunity against [`PREPARED_GPU_OPPORTUNITY_CEILING_US`]. `Ok` carries
/// the cursor's new consecutive-overrun run (`0` once an opportunity fits); `Err` carries the run
/// that reached [`semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS`] and is therefore
/// terminal. Pure so the law is testable without a GPU adapter.
pub(super) fn admit_prepared_gpu_opportunity(run: u32, elapsed_us: u64) -> Result<u32, u32> {
    if elapsed_us <= PREPARED_GPU_OPPORTUNITY_CEILING_US {
        return Ok(0);
    }
    let run = run.saturating_add(1);
    if run >= semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS {
        Err(run)
    } else {
        Ok(run)
    }
}

/// ⌛️ Times every successful advancement, including metadata and phase-only transitions.
pub(super) fn measure_prepared_gpu_opportunity(
    cursor: &mut PreparedGpuPresentCursor,
    command: Option<(u32, Option<DrawMeasureCursor>)>,
    mut now: impl FnMut() -> Option<u64>,
    advance: impl FnOnce(&mut PreparedGpuPresentCursor) -> Result<bool, String>,
) -> Result<bool, String> {
    let started = now().ok_or_else(|| "GPU opportunity requires a real monotonic clock".to_string())?;
    let opportunity = cursor.phase;
    let before = cursor.progress();
    let complete = advance(cursor)?;
    let elapsed = now().and_then(|now| now.checked_sub(started)).ok_or_else(|| "prepared GPU opportunity lost its monotonic clock".to_string())?;
    cursor.overrun_run = admit_prepared_gpu_opportunity(cursor.overrun_run, elapsed).map_err(|run| {
        format!(
            "prepared GPU opportunity exceeded the two millisecond ceiling for {run} consecutive opportunities: {opportunity:?} took {elapsed} us; command_kind={:?} draw={:?} progress={before:?}->{:?}",
            command.map(|command| command.0),
            command.and_then(|command| command.1),
            cursor.progress()
        )
    })?;
    Ok(complete)
}

pub(super) const PREPARED_GPU_ABANDONMENT_SLOTS: usize = 64;

pub(super) static PREPARED_GPU_ABANDONMENT_STATE: [AtomicU8; PREPARED_GPU_ABANDONMENT_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_GPU_ABANDONMENT_SLOTS];

static PREPARED_GPU_ABANDONMENT_OWNER: [AtomicPtr<PreparedGpuPresentCursor>; PREPARED_GPU_ABANDONMENT_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_GPU_ABANDONMENT_SLOTS];

pub(super) fn prepared_draw_scalar_uses_world_encoded_attachment(cursor: DrawMeasureCursor) -> bool {
    matches!(cursor, DrawMeasureCursor::PassSceneClear { .. } | DrawMeasureCursor::PassInstance { .. } | DrawMeasureCursor::PassMaterialInstance { .. } | DrawMeasureCursor::PassTexturedInstance { .. } | DrawMeasureCursor::PassGrid { .. } | DrawMeasureCursor::PassLineVertex { .. } | DrawMeasureCursor::PassPostprocess { .. })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PreparedCommandClipPiece {
    Unclipped,
    Empty,
    Scissor(semio_framework_ui_draw_contract::ScissorRect),
    Complete,
}

fn prepared_command_color_layer(draw: &semio_framework_ui_draw_contract::DrawList, cursor: DrawMeasureCursor) -> Result<Option<(usize, Option<[f32; 4]>)>, String> {
    let layer = match cursor {
        DrawMeasureCursor::LayerUi { layer, .. } | DrawMeasureCursor::LayerRaster { layer, .. } => Some((layer, None)),
        DrawMeasureCursor::LayerVector { layer, item, .. } if item % 3 == 2 => Some((layer, None)),
        DrawMeasureCursor::PassSceneClear { pass } | DrawMeasureCursor::PassInstance { pass, .. } | DrawMeasureCursor::PassMaterialInstance { pass, .. } | DrawMeasureCursor::PassTexturedInstance { pass, .. } | DrawMeasureCursor::PassGrid { pass } | DrawMeasureCursor::PassPostprocess { pass } => {
            let pass = draw.scene_passes.get(pass).ok_or_else(|| "prepared scene pass clip owner was stale".to_string())?;
            Some((pass.layer_index, Some(pass.viewport)))
        }
        DrawMeasureCursor::PassLineVertex { pass, vertex, .. } if vertex % 2 == 1 => {
            let pass = draw.scene_passes.get(pass).ok_or_else(|| "prepared scene pass clip owner was stale".to_string())?;
            Some((pass.layer_index, Some(pass.viewport)))
        }
        DrawMeasureCursor::Glass(region) => {
            let region = draw.glass_regions.get(region).ok_or_else(|| "prepared glass clip owner was stale".to_string())?;
            Some((region.layer_index, None))
        }
        _ => None,
    };
    Ok(layer)
}

pub(super) fn prepared_command_clip_piece(draw: &semio_framework_ui_draw_contract::DrawList, cursor: DrawMeasureCursor, piece: usize, width: f32, height: f32) -> Result<PreparedCommandClipPiece, String> {
    let Some((layer, viewport)) = prepared_command_color_layer(draw, cursor)? else { return Ok(PreparedCommandClipPiece::Unclipped) };
    let layer = draw.layers.get(layer).ok_or_else(|| "prepared command clip layer was stale".to_string())?;
    let surface = semio_framework_ui_draw_contract::ScissorRect { x: 0, y: 0, w: width.max(0.0) as u32, h: height.max(0.0) as u32 };
    let mut resolved = match layer.clip.as_ref() {
        Some(clip) => match clip.scissors.get(piece).copied() {
            Some(piece) => piece.intersect(&surface),
            None => return Ok(PreparedCommandClipPiece::Complete),
        },
        None if piece == 0 => surface,
        None => return Ok(PreparedCommandClipPiece::Complete),
    };
    if let Some(scissor) = layer.scissor {
        resolved = resolved.intersect(&scissor);
    }
    if let Some(viewport) = viewport {
        resolved = resolved.intersect(&semio_framework_ui_draw_contract::ScissorRect { x: viewport[0].max(0.0) as u32, y: viewport[1].max(0.0) as u32, w: viewport[2].max(0.0) as u32, h: viewport[3].max(0.0) as u32 });
    }
    Ok(if resolved.w == 0 || resolved.h == 0 { PreparedCommandClipPiece::Empty } else { PreparedCommandClipPiece::Scissor(resolved) })
}

/// 🎟️ Generation-qualified retained surface and command submission cursor.
pub struct PreparedGpuPresentCursor {
    pub(super) scene_revision: u64,
    pub(super) preview_generation: u64,
    pub(super) command: usize,
    pub(super) clip_piece: usize,
    pub(super) blur_mip: u32,
    pub(super) frame: Option<gpu_device::SurfaceTexture>,
    pub(super) view: Option<gpu_device::TextureView>,
    pub(super) phase: PreparedGpuPresentPhase,
    abandonment_slot: u8,
    pub(super) overrun_run: u32,
}

impl PreparedGpuPresentCursor {
    pub fn begin(scene_revision: u64, preview_generation: u64) -> Option<Self> {
        if scene_revision == 0 || scene_revision == u64::MAX || preview_generation == 0 || preview_generation == u64::MAX {
            return None;
        }
        let slot = PREPARED_GPU_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok())?;
        Some(Self { scene_revision, preview_generation, command: 0, clip_piece: 0, blur_mip: 1, frame: None, view: None, phase: PreparedGpuPresentPhase::EnsureTarget, abandonment_slot: slot as u8, overrun_run: 0 })
    }

    pub(super) fn matches(&self, packet: &PreparedRenderPacket) -> bool {
        self.scene_revision == packet.scene_revision() && self.preview_generation == packet.preview_generation() && packet.is_within_credits()
    }

    pub fn begin_close(&mut self) {
        self.phase = PreparedGpuPresentPhase::Closing;
    }

    /// 🐕️ Exposes every phase, authored command, and blur mip to the presentation watchdog.
    pub fn progress(&self) -> (u8, usize, usize, u32) {
        (self.phase as u8, self.command, self.clip_piece, self.blur_mip)
    }

    pub fn close_step(&mut self) -> bool {
        if self.view.take().is_some() {
            return false;
        }
        if self.frame.take().is_some() {
            return false;
        }
        self.command = 0;
        self.clip_piece = 0;
        self.blur_mip = 0;
        self.scene_revision = 0;
        self.preview_generation = 0;
        self.phase = PreparedGpuPresentPhase::Complete;
        if self.abandonment_slot != u8::MAX {
            let slot = usize::from(self.abandonment_slot);
            let Some(state) = PREPARED_GPU_ABANDONMENT_STATE.get(slot) else { return false };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
                return false;
            }
            self.abandonment_slot = u8::MAX;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.frame.is_none()
            && self.view.is_none()
            && self.command == 0
            && self.clip_piece == 0
            && self.blur_mip == 0
            && self.scene_revision == 0
            && self.preview_generation == 0
            && self.phase == PreparedGpuPresentPhase::Complete
            && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact GPU cursor owner recovered from an interrupted presentation.
    pub fn close_abandoned_step() -> bool {
        let Some(slot) = PREPARED_GPU_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = PREPARED_GPU_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            PREPARED_GPU_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
            return false;
        }
        let mut cursor = unsafe { Box::from_raw(pointer) };
        if cursor.close_step() || cursor.abandonment_slot == u8::MAX {
            drop(cursor);
        } else {
            PREPARED_GPU_ABANDONMENT_OWNER[slot].store(Box::into_raw(cursor), Ordering::Release);
            PREPARED_GPU_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        }
        false
    }
}

impl Drop for PreparedGpuPresentCursor {
    fn drop(&mut self) {
        if self.terminal_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_GPU_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let cursor = Box::new(Self {
            scene_revision: self.scene_revision,
            preview_generation: self.preview_generation,
            command: self.command,
            clip_piece: self.clip_piece,
            blur_mip: self.blur_mip,
            frame: self.frame.take(),
            view: self.view.take(),
            phase: PreparedGpuPresentPhase::Closing,
            abandonment_slot: self.abandonment_slot,
            overrun_run: self.overrun_run,
        });
        self.scene_revision = 0;
        self.preview_generation = 0;
        self.command = 0;
        self.clip_piece = 0;
        self.blur_mip = 0;
        self.phase = PreparedGpuPresentPhase::Complete;
        self.abandonment_slot = u8::MAX;
        PREPARED_GPU_ABANDONMENT_OWNER[slot].store(Box::into_raw(cursor), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}
// #endregion gpu
