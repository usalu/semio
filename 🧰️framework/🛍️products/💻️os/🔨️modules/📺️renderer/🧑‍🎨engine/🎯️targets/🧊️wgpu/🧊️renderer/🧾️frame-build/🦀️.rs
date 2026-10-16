//! 🧾️ Frame build: app handles, runtime and interaction state structs, frame packets, build cursors and the frame transaction.
use super::*;

#[path = "🧮️frame-transaction/🦀️.rs"]
mod frame_transaction;
pub(crate) use frame_transaction::*;

#[path = "📋️frame-preparation/🦀️.rs"]
mod frame_preparation;
pub(crate) use frame_preparation::*;

//#region 📮️RuntimeMailbox
//#region 🎮️AppInteractionState
/// 🪪️ Submits owned work without holding the original runtime or a mutex guard across suspension.
pub(crate) type AppHandle = RuntimeHandle<AppRuntime>;

/// 📮️ The runtime mailbox of the application runtime.
pub(crate) type AppMailbox = RuntimeMailbox<AppRuntime>;

pub(crate) struct AppRuntime {
    pub(crate) atlas: FontAtlas,
    pub(crate) icons: IconAtlas,
    pub(crate) icon_rebuild: Option<crate::icon_atlas::IconAtlasRebuild>,
    pub(crate) icon_raster_scale: f32,
    pub(crate) interaction: Option<AppInteractionState>,
    pub(crate) checkout: runtime_mailbox_core::InteractionCheckoutLedger,
    pub(crate) draw: DrawList,
    pub(crate) overlay: DrawList,
    pub(crate) pending_frame_deferred: Option<FrameDeferredCursor>,
    pub(crate) frame_actions: FrameActionOwners,

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) pending_frame_maintenance_refusal: Option<FrameMaintenanceRefusal>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) plugin_modules_root: std::path::PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) native_plugin_mtimes: std::collections::HashMap<std::path::PathBuf, std::time::SystemTime>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) native_hot_swap_scan: Option<RendererIoHandle>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) native_hot_swap_modified: Option<semio_framework_os_services::NativeModifiedSet>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) native_hot_swap_cursor: usize,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) native_reload_pending: bool,
}

impl std::ops::Deref for AppRuntime {
    type Target = AppInteractionState;

    fn deref(&self) -> &Self::Target {
        self.interaction.as_ref().expect("runtime interaction state is worker-owned")
    }
}

impl std::ops::DerefMut for AppRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.interaction.as_mut().expect("runtime interaction state is worker-owned")
    }
}

impl AppRuntime {
    pub(crate) fn interaction_available(&self) -> bool {
        self.interaction.is_some()
    }

    /// 📐️ The one resize the runtime owns: the content root moves in LOGICAL pixels and the glyph
    /// atlas — the only layout-facing owner that legitimately cares about device density — is
    /// re-pointed at the new scale factor. `set_raster_scale` is a no-op when the scale is
    /// unchanged, so an ordinary window drag never drops a single cached glyph; a display change
    /// does, and the atlas re-rasterises crisp at the new density on the next frame.
    /// Shadows the `Deref` route into [`AppInteractionState::resize`] deliberately.
    pub(crate) fn resize(&mut self, logical_width: f32, logical_height: f32, scale_factor: f32) {
        self.atlas.set_raster_scale(scale_factor);
        self.begin_icon_raster(scale_factor);
        if let Some(interaction) = self.interaction.as_mut() {
            interaction.resize(logical_width, logical_height, scale_factor);
        }
    }

    /// 🔁️ Arms a bounded icon re-rasterisation when `scale_factor` lands on a different atlas CELL
    /// size than the icons were rasterised at. Cell size, not raw scale: the atlas quantises the
    /// factor (`icon_cell_size` rounds and clamps to 3×), so a fractional resize that quantises the
    /// same way must not throw away 250 good rasterisations. The glyph atlas already re-rasterises
    /// lazily on the same event; this closes the icon half (ticket 26/09/17 packet W2k, W1g gap 1).
    fn begin_icon_raster(&mut self, scale_factor: f32) {
        if !scale_factor.is_finite() || scale_factor <= 0.0 {
            return;
        }
        let pending = crate::icon_atlas::IconAtlasRebuild::new(scale_factor);
        if pending.cell_size() == crate::icon_atlas::IconAtlasRebuild::new(self.icon_raster_scale).cell_size() {
            return;
        }
        self.icon_rebuild = Some(pending);
    }

    /// 🔁️ Spends one bounded slice on the in-flight icon rebuild. Answers the finished atlas exactly
    /// once, on the step that completes it, so the caller installs and re-uploads it.
    pub(crate) fn drive_icon_raster_step(&mut self) -> Option<IconAtlas> {
        let rebuild = self.icon_rebuild.as_mut()?;
        match rebuild.step(crate::icon_atlas::ICON_RASTER_STEP_BUDGET) {
            crate::icon_atlas::IconAtlasRebuildStep::Rasterized(_) => None,
            crate::icon_atlas::IconAtlasRebuildStep::Complete(atlas) => {
                let scale = rebuild.scale_factor();
                self.icon_rebuild = None;
                self.icon_raster_scale = scale;
                Some(atlas)
            }
        }
    }

    /// 🎟️ The ONE way the interaction state leaves the runtime, so every checkout is named and aged.
    ///
    /// ⚖️ `site` is the bounded step the state is lent to; it must come home through
    /// [`Self::return_interaction`] on every path, faults and supersession included.
    pub(crate) fn check_out_interaction(&mut self, site: &'static str) -> Option<AppInteractionState> {
        let interaction = self.interaction.take()?;
        if !self.checkout.check_out(site) {
            self.interaction = Some(interaction);
            return None;
        }
        Some(interaction)
    }

    /// 🎟️ The ONE way the interaction state comes home.
    pub(crate) fn return_interaction(&mut self, interaction: AppInteractionState) {
        self.checkout.check_in();
        self.interaction = Some(interaction);
    }

}

//#endregion 🎮️AppInteractionState
//#endregion 📮️RuntimeMailbox

//#region 📮️RuntimeMailbox
pub(crate) const FRAME_ENGINE_PACKET_CAPACITY: usize = 256;
pub(crate) const FRAME_ENGINE_PACKET_PAGE_SLOTS: usize = 16;
pub(crate) const FRAME_ENGINE_PACKET_PAGES: usize = FRAME_ENGINE_PACKET_CAPACITY / FRAME_ENGINE_PACKET_PAGE_SLOTS;
type FrameEnginePacketPage = [Option<engine_canvas::EngineCanvasPacket>; FRAME_ENGINE_PACKET_PAGE_SLOTS];

pub(crate) struct FrameEnginePackets {
    pub(crate) slots: [Option<Box<FrameEnginePacketPage>>; FRAME_ENGINE_PACKET_PAGES],
    pub(crate) len: usize,
}

impl Default for FrameEnginePackets {
    fn default() -> Self { Self { slots: [const { None }; FRAME_ENGINE_PACKET_PAGES], len: 0 } }
}

impl FrameEnginePackets {
    /// 🧱️ Admits one original page under the same caller before any retained packet can enter it.
    pub(crate) fn prepare_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Ok(true) };
        let grant = cx.retained_grant();
        let bytes = std::mem::size_of::<FrameEnginePacketPage>();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_capacity_bytes < bytes { return Ok(false); }
        if grant.maximum_depth == 0 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original frame packet page birth requires depth").into()); }
        self.slots[index] = Some(semio_framework_async::boxed_fixed_slots(|| None));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(index + 1 == FRAME_ENGINE_PACKET_PAGES)
    }

    pub(crate) fn try_push(&mut self, packet: engine_canvas::EngineCanvasPacket) -> Result<(), engine_canvas::EngineCanvasPacket> {
        if self.len == FRAME_ENGINE_PACKET_CAPACITY { return Err(packet); }
        let Some(page) = self.slots[self.len / FRAME_ENGINE_PACKET_PAGE_SLOTS].as_mut() else { return Err(packet) };
        page[self.len % FRAME_ENGINE_PACKET_PAGE_SLOTS] = Some(packet);
        self.len += 1;
        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Option<engine_canvas::EngineCanvasPacket> {
        let index = self.len.checked_sub(1)?;
        let packet = self.slots[index / FRAME_ENGINE_PACKET_PAGE_SLOTS].as_mut()?[index % FRAME_ENGINE_PACKET_PAGE_SLOTS].take();
        if packet.is_some() { self.len = index; }
        packet
    }

    fn last_mut(&mut self) -> Option<&mut engine_canvas::EngineCanvasPacket> {
        let index = self.len.checked_sub(1)?;
        self.slots[index / FRAME_ENGINE_PACKET_PAGE_SLOTS].as_mut()?[index % FRAME_ENGINE_PACKET_PAGE_SLOTS].as_mut()
    }

    pub(crate) fn get(&self, index: usize) -> Option<&engine_canvas::EngineCanvasPacket> {
        if index >= self.len { return None; }
        self.slots[index / FRAME_ENGINE_PACKET_PAGE_SLOTS].as_ref()?[index % FRAME_ENGINE_PACKET_PAGE_SLOTS].as_ref()
    }

    pub(crate) fn is_empty(&self) -> bool { self.len == 0 && self.slots.iter().flatten().all(|page| page.iter().all(Option::is_none)) }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.len == 0 && self.slots.iter().all(Option::is_none) }

    /// 📏️ Quotes one original allocated page independently of its logical packet count.
    pub(crate) fn backing_retirement_demands(&self) -> semio_framework_value::RetirementDemand {
        if self.terminal_is_empty() || self.len != 0 { return Default::default(); }
        semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Box<FrameEnginePacketPage>>(), release_bytes: std::mem::size_of::<FrameEnginePacketPage>(), depth: 1, ..Default::default() }
    }

    /// 🎟️ Releases only one actual empty original page under the unchanged caller grant.
    pub(crate) fn close_backing_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_value::ValueError> {
        use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: empty }); }
        if !self.is_empty() { return Ok(InteractiveJobCloseStep::Blocked); }
        let demand = self.backing_retirement_demands();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(InteractiveJobCloseStep::Pending { progress: empty }); }
        if grant.maximum_depth == 0 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original frame packet backing close requires depth")); }
        let index = self.slots.iter().rposition(Option::is_some).expect("original allocated page exists");
        drop(self.slots[index].take());
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..empty };
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } })
    }
}

pub(crate) struct AppFrameBuild {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    pub(crate) input: ui_wgpu::wgpu::PreparedRenderInput,
    pub(crate) input_candidate: Option<shell::PresentedInputCandidateWitness>,
    pub(crate) engine_packets: FrameEnginePackets,
    pub(crate) generation: semio_framework_trace::Generation,
    pub(crate) cursor: SemioCursor,
    pub(crate) theme_dark: bool,
    pub(crate) fullscreen: Option<bool>,
    pub(crate) cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
}

pub(crate) struct AppFrameAfterChrome {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    pub(crate) resource_input: Option<ui_wgpu::wgpu::PreparedRenderInput>,
    pub(crate) input_candidate: Option<shell::PresentedInputCandidateWitness>,
    pub(crate) upload_rejected: Option<ui_wgpu::wgpu::PreparedRenderUpload>,
    pub(crate) draw_rejected: Option<ui_wgpu::wgpu::PreparedRenderInputRejected>,
    pub(crate) engine_packets: Option<FrameEnginePackets>,
    pub(crate) fullscreen: Option<bool>,
    pub(crate) cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
    pub(crate) retirement: Option<AppFramePreparation>,
}

pub(crate) struct FrameBuildCursor {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    pub(crate) phase: FrameBuildPhase,
    pub(crate) retained_clock_index: usize,
    pub(crate) retained_clock_deadline: Option<f64>,
    pub(crate) presentation_witness: RuntimePresentationWitness,
    pub(crate) input_candidate: Option<shell::PresentedInputCandidateWitness>,
    pub(crate) fullscreen: Option<bool>,
    pub(crate) previous_draw: Option<DrawList>,
    pub(crate) previous_overlay: Option<DrawList>,
    pub(crate) icon_checked: bool,
    pub(crate) icon_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>,
    pub(crate) icon_upload: Option<ui_wgpu::wgpu::PreparedRenderUpload>,
    pub(crate) engine_resources: Option<engine_canvas::EngineCanvasBuildContext>,
    pub(crate) world_resources: Option<infinite_world::world::World3dBuildContext>,
    pub(crate) resource_input: Option<ui_wgpu::wgpu::PreparedRenderInput>,
    pub(crate) input_rejected: Option<ui_wgpu::wgpu::PreparedRenderInputRejected>,
    pub(crate) engine_packets: FrameEnginePackets,
    pub(crate) engine_rejected: Option<engine_canvas::EngineCanvasPacket>,
    pub(crate) world_rejected: Option<infinite_world::world::World3dBuildRejected>,
    pub(crate) retirement: Option<AppFramePreparation>,
    pub(crate) chrome: shell::ShellChromeFrameCursor,
    pub(crate) cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
}

#[derive(Clone, Copy)]
pub(crate) enum FrameBuildPhase {
    Deferred,
    PreparedGpuAbandonment,
    PreparedInputAbandonment,
    PreparedJobAbandonment,
    PreparedAbandonment,
    PacketAbandonment,
    Text,
    Fullscreen,
    NativePoll,
    NativeReload,
    ThemeResolve,
    ThemeAppearance,
    Drag,
    Hover,
    InputFrame,
    WheelDeadline,
    RetainedClock,
    TakeDraw,
    RetireDraw,
    TakeOverlay,
    RetireOverlay,
    IconAtlas,
    Tutorial,
    EnginePacketBacking,
    EngineResources,
    WorldResources,
    Chrome,
    JobProgressTake,
    JobProgressOverlay,
    ResourceInput,
    WorldTransfer,
    IconTransfer,
    EngineTransfer,
    CursorWake,
    Complete,
}

impl FrameBuildCursor {
    pub(crate) fn new(presentation_witness: RuntimePresentationWitness, retained: semio_framework_job::RetainedCloneGrant) -> Self {
        Self {
            retained,
            phase: FrameBuildPhase::Deferred,
            retained_clock_index: 0,
            retained_clock_deadline: None,
            presentation_witness,
            input_candidate: None,
            fullscreen: None,
            previous_draw: None,
            previous_overlay: None,
            icon_checked: false,
            icon_pages: None,
            icon_upload: None,
            engine_resources: None,
            world_resources: None,
            resource_input: None,
            input_rejected: None,
            engine_packets: FrameEnginePackets::default(),
            engine_rejected: None,
            world_rejected: None,
            retirement: None,
            chrome: shell::ShellChromeFrameCursor::default(),
            cursor_wake: None,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress: None,
        }
    }

    pub(crate) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        let policy=self.retained;
        cx.with_retained_loan(policy,|cx|self.close_original_step(cx))
    }

    fn close_original_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let grant=cx.retained_grant();
        if let Some(pages)=self.icon_pages.as_mut(){
            if !pages.terminal_is_empty(){return frame_receive_close(frame_child_step(pages.close_original_step(grant),grant,pages.terminal_is_empty()),cx)}
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.icon_pages=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(owner)=self.engine_resources.as_mut(){
            if !owner.terminal_is_empty(){return frame_received_child(owner.close_step(cx),grant,owner.terminal_is_empty())}
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.engine_resources=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(owner)=self.world_rejected.as_mut(){
            if !owner.terminal_is_empty(){
                let demand=owner.next_close_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
                return frame_original_primitive_close(cx,demand,|grant|{let step=owner.close_step(grant)?;step.admit(grant,owner.terminal_is_empty())});
            }
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.world_rejected=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(owner)=self.world_resources.as_mut(){
            if !owner.terminal_is_empty(){return frame_received_child(owner.close_step(cx),grant,owner.terminal_is_empty())}
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.world_resources=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(owner)=self.icon_upload.as_mut(){
            if !owner.terminal_is_empty(){
                let demand=owner.next_close_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
                return frame_original_primitive_close(cx,demand,|grant|{let step=owner.close_step(grant)?;step.admit(grant,owner.terminal_is_empty())});
            }
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.icon_upload=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if !self.engine_packets.terminal_is_empty() {
            if !self.engine_packets.is_empty() { return frame_receive_close(frame_unpriced_child(),cx); }
            let demand = self.engine_packets.backing_retirement_demands();
            return frame_original_primitive_close(cx,demand,|grant|self.engine_packets.close_backing_step(grant));
        }
        frame_receive_close(frame_unpriced_child(),cx)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.previous_draw.is_none()
            && self.previous_overlay.is_none()
            && self.engine_rejected.is_none()
            && self.world_rejected.is_none()
            && self.engine_packets.terminal_is_empty()
            && self.engine_resources.is_none()
            && self.world_resources.is_none()
            && self.resource_input.is_none()
            && self.input_rejected.is_none()
            && self.icon_upload.is_none()
            && self.icon_pages.is_none()
            && self.retirement.is_none()
            && self.cursor_wake.is_none()
            && self.input_candidate.is_none()
    }
}

pub(crate) struct FrameFinishCursor {
    pub(crate) phase: FrameFinishPhase,
    pub(crate) cursor: SemioCursor,
    pub(crate) pump_sync: bool,
    pub(crate) flush_tutorial: bool,
    pub(crate) shell_maintenance: bool,
    pub(crate) settle: bool,
    pub(crate) glyph_started: bool,
    pub(crate) glyph_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>,
    pub(crate) icon_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>,
}

#[derive(Clone, Copy)]
pub(crate) enum FrameFinishPhase {
    Inputs,
    Deferred,
    /// 🔁️ Drives `AppRuntime::drive_icon_raster_step` after a density change — one bounded slice per
    /// frame, then one whole-atlas upload on the step that finishes it.
    IconRaster,
    GlyphUpload,
    Cursor,
    Draw,
    Overlay,
    Complete,
}

impl Default for FrameFinishCursor {
    fn default() -> Self {
        Self { phase: FrameFinishPhase::Inputs, cursor: SemioCursor::Default, pump_sync: false, flush_tutorial: false, shell_maintenance: false, settle: false, glyph_started: false, glyph_pages: None, icon_pages: None }
    }
}

impl FrameFinishCursor {
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
        for pages in [&mut self.icon_pages, &mut self.glyph_pages] {
            if let Some(owner) = pages.as_mut() {
                if !owner.terminal_is_empty() { return frame_child_step(owner.close_original_step(grant), grant, owner.terminal_is_empty()); }
                let bytes = std::mem::size_of::<Option<ui_wgpu::wgpu::PreparedAtlasPages>>();
                if let Some(step) = frame_inline_gate(grant, bytes)? { return Ok(step); }
                *pages = None;
                return frame_inline_progress(bytes);
            }
        }
        Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.glyph_pages.is_none() && self.icon_pages.is_none() }
}

pub(crate) enum FrameBuildBoundaryStep {
    Pending,
    Complete(AppFrameAfterChrome),
    Fault(&'static str),
    CloseFailure(semio_framework_job::OriginalTurnFailure),
}

/// 🧾️ Validates the original recovered frame owner's full close receipt before advancing.
pub(crate) fn frame_prepared_abandonment_complete(grant:RetainedCloneGrant,step:Result<InteractiveJobCloseStep,semio_framework_value::ValueError>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let step=step.and_then(|step|step.admit(grant,matches!(step,InteractiveJobCloseStep::Complete{..})));
    Ok(matches!(frame_receive_close(step,cx)?,InteractiveJobCloseStep::Complete{..}))
}

pub(crate) enum FrameFinishBoundaryStep {
    Pending,
    Complete(AppFrameBuild),
    Fault(&'static str),
}

impl AppFrameAfterChrome {
    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let policy=self.retained;
        cx.with_retained_loan(policy,|cx|self.close_original_step(cx))
    }

    fn close_original_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let grant=cx.retained_grant();
        if self.draw_rejected.is_some(){return frame_receive_close(frame_unpriced_child(),cx)}
        if let Some(owner)=self.upload_rejected.as_mut(){
            if !owner.terminal_is_empty(){
                let demand=owner.next_close_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
                return frame_original_primitive_close(cx,demand,|grant|{let step=owner.close_step(grant)?;step.admit(grant,owner.terminal_is_empty())});
            }
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.upload_rejected=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(owner)=self.resource_input.as_mut(){
            if !owner.terminal_is_empty(){
                let demand=owner.next_close_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
                return frame_original_primitive_close(cx,demand,|grant|{let step=owner.close_step(grant)?;step.admit(grant,owner.terminal_is_empty())});
            }
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.resource_input=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(owner)=self.retirement.as_mut(){
            if !owner.terminal_is_empty(){return frame_received_child(owner.close_step(cx,None),grant,owner.terminal_is_empty())}
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.retirement=None;return frame_receive_close(frame_inline_progress(0),cx);
        }
        frame_receive_close(frame_unpriced_child(),cx)
    }

    pub(crate) fn terminal_is_empty(&self)->bool{self.resource_input.is_none()&&self.input_candidate.is_none()&&self.upload_rejected.is_none()&&self.draw_rejected.is_none()&&self.engine_packets.is_none()&&self.cursor_wake.is_none()&&self.retirement.is_none()}

}

/// 🚧️ Keeps an original child that has no declared physical retirement authority.
pub(crate) fn frame_unpriced_child() -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original frame child retirement authority is undeclared"))
}

/// 🎟️ One quoted primitive debits the unchanged frame caller exactly once.
fn frame_original_primitive_close(cx:&mut semio_framework_job::StepContext<'_>,quote:RetainedCloneGrant,advance:impl FnOnce(RetainedCloneGrant)->Result<InteractiveJobCloseStep,semio_framework_value::ValueError>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    let demand=semio_framework_value::RetirementDemand{copy_bytes:quote.maximum_copy_bytes,capacity_bytes:quote.maximum_capacity_bytes,release_bytes:quote.maximum_release_bytes,depth:quote.maximum_depth};
    let mut blocked=false;
    let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|advance(grant).map(|step|match step{InteractiveJobCloseStep::Pending{progress}=>semio_framework_value::RetainedCloneStep::Progress(progress),InteractiveJobCloseStep::Complete{progress}=>semio_framework_value::RetainedCloneStep::Complete(progress),InteractiveJobCloseStep::Blocked=>{blocked=true;semio_framework_value::RetainedCloneStep::Progress(Default::default())}}))?;
    if blocked{Ok(InteractiveJobCloseStep::Blocked)}else{Ok(step)}
}

/// 🧾️ Carries the real child receipt without publishing the parent as empty.
pub(crate) fn frame_child_step(step: Result<InteractiveJobCloseStep,semio_framework_value::ValueError>, grant: RetainedCloneGrant, terminal: bool) -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
    Ok(match step?.admit(grant, terminal)? {
        InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
        step => step,
    })
}

/// 🪙️ Quotes the original inline owner header after its physical children are empty.
pub(crate) fn frame_inline_gate(grant: RetainedCloneGrant, _bytes: usize) -> Result<Option<InteractiveJobCloseStep>,semio_framework_value::ValueError> {
    if grant.maximum_items == 0 { return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() })); }
    if grant.maximum_depth == 0 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original frame inline retirement requires depth")); }
    Ok(None)
}

/// ♻️ Returns the actual paid inline header receipt independently of heap release.
pub(crate) fn frame_inline_progress(_bytes: usize) -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
    Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: 0, ..RetainedCloneProgress::default() } })
}

/// 🧵️ Preserves all currencies and refusal progress from the original worker owner.
pub(crate) fn frame_worker_child_step(step: semio_framework_job_worker_authority::WorkerJobCloseStep, grant: RetainedCloneGrant, terminal: bool) -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
    use semio_framework_job_worker_authority::{WorkerJobCloseStep};
    let step = match step {
        WorkerJobCloseStep::Pending { progress } => InteractiveJobCloseStep::Pending { progress },
        WorkerJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Complete { progress },
        WorkerJobCloseStep::Blocked => InteractiveJobCloseStep::Blocked,
    };
    frame_child_step(Ok(step), grant, terminal)
}

pub(crate) fn frame_receive_close(step:Result<InteractiveJobCloseStep,semio_framework_value::ValueError>,cx:&mut semio_framework_job::StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    let step=step.map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
    cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
    if step.progress().copied_items!=0{cx.consume_fuel(1)}
    Ok(step)
}

pub(crate) fn frame_received_child(step:Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>,grant:RetainedCloneGrant,terminal:bool)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
    let step=step?.admit(grant,terminal).map_err(semio_framework_job::OriginalTurnFailure::from)?;
    Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})
}

pub(crate) struct AppFramePresentation {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    pub(crate) packet: Option<ui_wgpu::wgpu::PreparedRenderPacket>,
    pub(crate) input_candidate: Option<shell::PresentedInputCandidateWitness>,
    pub(crate) engine_packets: FrameEnginePackets,
    pub(crate) generation: semio_framework_trace::Generation,
    pub(crate) cursor: SemioCursor,
    pub(crate) theme_dark: bool,
    pub(crate) fullscreen: Option<bool>,
    pub(crate) cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
}

pub(crate) fn discard_frame_input_candidate(runtime: &AppMailbox, candidate: &mut Option<shell::PresentedInputCandidateWitness>) -> bool {
    let Some(witness) = *candidate else { return true };
    let Ok(mut runtime) = runtime.try_lock() else { return false };
    let Some(interaction) = runtime.interaction.as_mut() else { return false };
    if !interaction.shell.input.discard_presented_input_candidate(witness) {
        return false;
    }
    *candidate = None;
    true
}

impl AppFrameBuild {
    pub(crate) fn into_preparation(self) -> AppFramePreparation {
        let AppFrameBuild {
            retained,
            input,
            input_candidate,
            engine_packets,
            generation,
            cursor,
            theme_dark,
            fullscreen,
            cursor_wake,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress,
        } = self;
        let (job, job_rejected) = match ui_wgpu::wgpu::PreparedRenderJob::try_new(input) {
            Ok(job) => (Some(job), None),
            Err(rejected) => (None, Some(rejected)),
        };
        AppFramePreparation {
            retained,
            params: None,
            params_cancel_retirement: None,
            worker_semantic_pending: false,
            outcome_pending: None,
            outcome_acknowledged: false,
            refused_receipt: None,
            job,
            job_rejected,
            input_candidate,
            session: None,
            engine_packets: Some(engine_packets),
            generation,
            cursor,
            theme_dark,
            fullscreen,
            cursor_wake,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress,
            terminal: false,
            fault: None,
        }
    }
}
//#endregion 📮️RuntimeMailbox
