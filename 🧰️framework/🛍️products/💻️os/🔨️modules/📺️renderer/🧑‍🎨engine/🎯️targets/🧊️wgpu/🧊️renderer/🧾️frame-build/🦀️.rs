//! 🧾️ Frame build: app handles, runtime and interaction state structs, frame packets, build cursors and the frame transaction.
use super::*;

//#region 📮️RuntimeMailbox
//#region 🎮️AppInteractionState
/// 🪪️ Submits owned work without holding the original runtime or a mutex guard across suspension.
pub(crate) type AppHandle = std::sync::Weak<RuntimeMailboxInner>;

const TEXT_STREAM_CAPACITY: usize = 64;

#[derive(Clone, Copy)]
pub(crate) struct AppTextStream {
    pub(crate) id: u64,
    pub(crate) token: ui_contract::TextIngressToken,
}

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

pub(crate) struct AppInteractionState {
    pub(crate) shell: ShellState,
    pub(crate) input: InputState<ActionDescriptor>,
    pub(crate) theme: Theme,
    pub(crate) theme_dark: bool,
    pub(crate) last_pointer_x: f32,
    pub(crate) last_pointer_y: f32,
    pub(crate) pointer_down: bool,
    pub(crate) pointer_button: i16,
    /// 🎯️ The layer that owns the pointer SEQUENCE in flight — set on the press from
    /// [`ShellState::pointer_owner_at`] and cleared on the release. React gets this from the DOM: the
    /// element a `pointerdown` resolved to receives the whole gesture, so a press that landed on
    /// chrome keeps every move and the release away from the canvas underneath it, whatever the
    /// pointer travels over in between.
    pub(crate) pointer_capture: PointerCapture,
    pub(crate) modifiers: PointerModifiers,
    pub(crate) space_pressed: bool,
    pub(crate) wheel_zoom_deadline_ms: f64,
    pub(crate) text_streams: [Option<AppTextStream>; TEXT_STREAM_CAPACITY],
    pub(crate) text_fault: Option<String>,
    pub(crate) frame_fault: Option<String>,
    pub(crate) text_cancel_pending: bool,
    pub(crate) last_sync_pump_ms: f64,
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

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn submit_interaction<F, Fut>(&mut self, handle: &AppHandle, key: Option<&'static str>, work: F) -> bool
    where
        F: FnOnce(AppInteractionState) -> Fut,
        Fut: Future<Output = AppInteractionState> + Send + 'static,
    {
        let Some(interaction) = self.check_out_interaction("submit-interaction") else { return false };
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            self.return_interaction(interaction);
            return false;
        };
        if !mailbox.reserve_interaction_future() {
            self.return_interaction(interaction);
            return false;
        }
        mailbox.spawn_interaction_reserved(key, work(interaction));
        true
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

pub(crate) struct FrameTransaction {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    directives: Option<frame_job::FrameDirectives>,
    pub(crate) operation: semio_framework_trace::OperationId,
    pub(crate) generation: semio_framework_trace::Generation,
    base_witness: Option<RuntimePresentationWitness>,
    effect_opportunities: u32,
    pub(crate) stage: FrameTransactionStage,
    pub(crate) phase: AppFrameTransactionPhase,
    board_authority_cursor: usize,
    world3d_authority_cursor: usize,
    scene_camera_cursor: Option<scenes::SceneCameraDispatchCursor>,
    /// 🥽️ How many world surfaces this frame has already offered a `registerBrushMesh` step — a
    /// plain index into `world3d_states`, so the phase owns no closable authority.
    brush_mesh_cursor: usize,
    pub(crate) build_cursor: Option<FrameBuildCursor>,
    finish_cursor: Option<FrameFinishCursor>,
    pub(crate) after_chrome: Option<AppFrameAfterChrome>,
    raster_uploads: Option<scenes::PendingRasterUploadCursor>,
    raster_rejected: Option<ui_wgpu::wgpu::PreparedRasterProducer>,
}

pub(crate) enum AppFrameTransactionStep {
    Pending,
    Complete(AppFrameBuild),
    /// 🌀️ This opportunity's inputs moved under it — a newer input generation, a newer scene
    /// revision, or a cancelled operation. NOT a defect: the candidate is discarded and the next
    /// opportunity builds a fresh one from the current witness.
    ///
    /// 🩸️ Every one of those conditions used to answer [`Self::Fault`] AND record a runtime frame
    /// fault, which the browser worker turns into a surface quarantine. The scene revision is bumped
    /// by every runtime completion (`enqueue_runtime_completion`, `finish`), so a frame build long
    /// enough to ingest a retained document is superseded as a matter of course — and the whole shell
    /// died with `frame opportunity base revision was superseded` instead of simply rebuilding
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
    Superseded,
    Fault,
    CloseFailure(semio_framework_job::OriginalTurnFailure),
}

/// 🧭️ The mounted frame's seven monotonic publication stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FrameTransactionStage {
    DrainProjectionDeltas,
    RouteIntents,
    FlushEffects,
    PresentSurface,
    ReconcileTree,
    BuildRenderPackets,
    PublishSnapshot,
}

impl FrameTransactionStage {
    pub(crate) fn latency_stage(self) -> frame_latency::FrameLatencyStage {
        match self {
            Self::DrainProjectionDeltas => frame_latency::FrameLatencyStage::TransactionDrainProjectionDeltas,
            Self::RouteIntents => frame_latency::FrameLatencyStage::TransactionRouteIntents,
            Self::FlushEffects => frame_latency::FrameLatencyStage::TransactionFlushEffects,
            Self::PresentSurface => frame_latency::FrameLatencyStage::TransactionPresentSurface,
            Self::ReconcileTree => frame_latency::FrameLatencyStage::TransactionReconcileTree,
            Self::BuildRenderPackets => frame_latency::FrameLatencyStage::TransactionBuildRenderPackets,
            Self::PublishSnapshot => frame_latency::FrameLatencyStage::TransactionPublishSnapshot,
        }
    }
}

enum AppFrameTransactionPhase {
    SceneCamera,
    BrushMesh,
    Build,
    InputEvents,
    FrameDeferred,
    BoardAuthority,
    World3dSnapshot,
    World3dAuthority,
    RasterUploads,
    Finish,
    Terminal,
}

impl FrameTransaction {
    pub(crate) fn new(directives: frame_job::FrameDirectives, operation: semio_framework_trace::OperationId, generation: semio_framework_trace::Generation, retained: semio_framework_job::RetainedCloneGrant) -> Self {
        Self {
            retained,
            directives: Some(directives),
            operation,
            generation,
            base_witness: None,
            effect_opportunities: 0,
            stage: FrameTransactionStage::DrainProjectionDeltas,
            phase: AppFrameTransactionPhase::SceneCamera,
            board_authority_cursor: 0,
            world3d_authority_cursor: 0,
            scene_camera_cursor: None,
            brush_mesh_cursor: 0,
            build_cursor: None,
            finish_cursor: None,
            after_chrome: None,
            raster_uploads: None,
            raster_rejected: None,
        }
    }

    /// 🎟️ No interaction state, no frame — and NO PARKING.
    ///
    /// 🩸️ Parking here (`Pending`) kept the build alive across the whole checkout, and a live build
    /// both freezes the frame generation and keeps asking for frames, so the worker isolate spun its
    /// whole interactive share on a transaction that could not move — starving the very suspended
    /// turn whose completion returns the state. Measured on 6118 as a third dispatched pointer move
    /// that never came home, after which the isolate answered nothing for thirty seconds
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
    /// Ending the build instead retires the session in this same turn: the mailbox is pumped from
    /// every advance, the returning completion is applied at the next opportunity, and the build that
    /// needs the state is admitted fresh against it.
    ///
    /// 🥽️ ONE `registerBrushMesh` per world surface per frame — React's `BrushMeshRegistrar`
    /// announces a loaded GLB's collision geometry to the puzzle guest per window instance,
    /// and `drainPuzzle3dBrushMeshQueue` keeps exactly one page outstanding while a first
    /// upload runs. The wgpu host dispatched nothing at all, so the guest's brush and
    /// volume-brush utilities had no collision body on this renderer (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w11a-…-missing.md` §6 family B). The row is
    /// `gesture` provenance like every other surface-derived action on this lane.
    ///
    /// 🌩️ ONE effect-flush ROUND, counted where the transaction actually enters the stage.
    ///
    /// 🩸️ The counter used to be charged on every `step` taken WHILE the stage was
    /// `FlushEffects`, which is not a storm measure at all: the stage covers the whole
    /// post-intent tail (deferred frame work, board authority, world snapshots, the prepared
    /// packet), and that tail legitimately needs far more than {@link EFFECT_STORM_BUDGET}
    /// one-cursor steps. It only stayed invisible while the wgpu browser frame never reached
    /// this stage at all; the moment it did, every frame faulted
    /// `frame effect storm budget exhausted` and quarantined the surface
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
    ///
    /// 🔁️ A `Pending` draw rebuild does NOT end this phase's turn.
    ///
    /// 🩸️ `step_world3d_draw_rebuild` owns only the PUBLISH half of a rebuild and answers
    /// `Pending` for as long as its cursor is unsealed — and the only thing that can seal
    /// it is `step_world3d_snapshot`, below. Returning here therefore made the two steps
    /// wait on each other forever: measured on 6118 as `procedural-preview` frozen at
    /// `apply-page=0/3 apply-item=1 rebuild=true state-meshes=2 draws=0` across 3.8 M
    /// frame-transaction turns, with the solid `extrude@solid` sitting unconsumed in its
    /// mesh lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Both shared drivers —
    /// `♾️infinite/🌍️world`'s own `drive_scene_bridge` and
    /// `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces` — already run the two steps in
    /// exactly this order, unconditionally; this host was the one place that diverged.
    ///
    /// 🧹️ `Stale` is NOT a fault: it means the rebuild cursor was begun for a revision
    /// or generation the state has since moved past, and the owning module's own rule
    /// (`♾️infinite/🌍️world` §`retained_draw_rebuild_stale_and_interrupted_close_never_publish`)
    /// is to CLOSE that cursor down its stepped ladder and let the next rebuild begin.
    /// The reference driver `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:191`
    /// faults on `Fault` alone; this host was the one place that also faulted on `Stale`,
    /// and the fault code is fatal — `record_frame_fault` quarantines the surface for
    /// good. Measured on 6118: a hover on the ELEVENTH gesture of a session published
    /// `seq=3751 frame-credits: world3d retained draw rebuild faulted` and the frame
    /// wire froze at 3 751 batches for the remaining nine gestures, with the page
    /// showing "Surface: quarantined · input accepted: no"
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🚧️🚪️ A modal layer OPENING clears the hover the pane published, with no pointer
    /// move of its own. React's overlays take pointer events the instant they mount, so
    /// r3f raises `onPointerOut` there and then — its `context-menu` step journals the
    /// clear inside the step that opened the menu. This renderer only handed a surface its
    /// leave on the next MOVE (W12a), so the clear rode along to the next step that moved
    /// the pointer at all: the stray `interactionHover targets:[]` in
    /// `example-picker-open`, ten steps and six keyboard chords later
    /// (`🗑️generated/w12c-parity-run-19/steps.json` step 33, ticket 26/09/17 packet
    /// W13c §1). Idempotent by construction — the clear retires the published hover, so
    /// the predicate answers `false` on every later frame the layer stays open.
    ///
    /// 🖼️ One bitmap per step off `ui_wgpu`'s own image ledger — the drain
    /// `take_ui_image_upload`'s docstring names ("the host's per-frame drain") and never had.
    /// It answers `None` whenever a `SceneHost` is registered (the renderer's normal case:
    /// `render_ui_image_step` owns `UiNode::Image` then), so this is free here and is what
    /// makes the no-scene-host paint path in `ui_wgpu` actually show a decoded `data:` image
    /// instead of the `alt` placeholder.
    pub(crate) fn step(&mut self, runtime: &RuntimeMailbox, handle: &AppHandle, context: &mut semio_framework_job::StepContext<'_>) -> AppFrameTransactionStep {
        let _latency = frame_latency::FrameLatencyTimer::start(frame_latency::FrameLatencyAuthority::renderer_frame(self.generation.0), self.stage.latency_stage(), 1);
        context.set_stage(self.stage_label());
        if context.operation() != self.operation || context.generation() != self.generation || context.is_cancelled() || context.deadline_exceeded() {
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Superseded;
        }
        let Some(current_witness) = runtime.presentation_witness_for(self.generation.0) else {
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Superseded;
        };
        if let Some(base_witness) = self.base_witness {
            if base_witness != current_witness {
                self.phase = AppFrameTransactionPhase::Terminal;
                return AppFrameTransactionStep::Superseded;
            }
        } else {
            self.base_witness = Some(current_witness);
        }
        let Some(directives) = self.directives.as_ref() else { return AppFrameTransactionStep::Pending };
        if context.should_yield() {
            return AppFrameTransactionStep::Pending;
        }
        let Ok(mut app) = runtime.try_lock() else { return AppFrameTransactionStep::Pending };
        if !app.interaction_available() {
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Superseded;
        }
        if !matches!(self.phase, AppFrameTransactionPhase::InputEvents) && app.frame_actions.staged_batch_remaining().is_some() {
            let transfer = {
                let app = &mut *app;
                let interaction = app.interaction.as_mut().expect("runtime interaction state is worker-owned");
                transfer_frame_input_action(&mut interaction.input, &mut app.frame_actions)
            };
            return match transfer {
                Ok(FrameInputActionStep::Pending | FrameInputActionStep::Transferred) => AppFrameTransactionStep::Pending,
                Ok(FrameInputActionStep::Empty | FrameInputActionStep::Deferred) => {
                    runtime.record_frame_fault("staged frame input action batch lost its runtime owner");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
                Err(fault) => {
                    runtime.record_frame_fault(fault);
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
            };
        }
        match self.phase {
            AppFrameTransactionPhase::SceneCamera => {
                let grant = context.retained_grant();
                if self.scene_camera_cursor.is_none() {
                    match scenes::SceneCameraDispatchCursor::begin(app_now_ms(), grant) {
                        Ok((cursor, progress)) => {
                            self.scene_camera_cursor = Some(cursor);
                            if context.consume_retained(progress).is_err() { runtime.record_frame_fault("camera original admission receipt exceeded frame authority"); return AppFrameTransactionStep::Fault; }
                            return AppFrameTransactionStep::Pending;
                        }
                        Err(error) => {
                            let _ = context.consume_retained(error.retained_progress());
                            runtime.record_frame_fault("camera original storage admission refused");
                            return AppFrameTransactionStep::Fault;
                        }
                    }
                }
                let (step, progress) = match self.scene_camera_cursor.as_mut().unwrap().step(grant) {
                    Ok(receipt) => receipt,
                    Err(error) => {
                        let _ = context.consume_retained(error.retained_progress());
                        runtime.record_frame_fault("camera original storage transition refused");
                        return AppFrameTransactionStep::Fault;
                    }
                };
                if context.consume_retained(progress).is_err() { runtime.record_frame_fault("camera original transition receipt exceeded frame authority"); return AppFrameTransactionStep::Fault; }
                match step {
                scenes::SceneCameraDispatchStep::Pending => AppFrameTransactionStep::Pending,
                scenes::SceneCameraDispatchStep::Action(action) => {
                    if let Err(_action) = app.frame_actions.try_push(action) {
                        runtime.record_frame_fault("frame deferred action credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    AppFrameTransactionStep::Pending
                }
                scenes::SceneCameraDispatchStep::Complete => {
                    self.stage = FrameTransactionStage::RouteIntents;
                    self.phase = AppFrameTransactionPhase::BrushMesh;
                    AppFrameTransactionStep::Pending
                }
                scenes::SceneCameraDispatchStep::Fault(fault) => {
                    runtime.record_frame_fault(fault);
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
                }
            },
            AppFrameTransactionPhase::BrushMesh => {
                let Some(surface) = app.shell.world3d_states.keys().nth(self.brush_mesh_cursor).cloned() else {
                    self.phase = AppFrameTransactionPhase::Build;
                    return AppFrameTransactionStep::Pending;
                };
                self.brush_mesh_cursor = self.brush_mesh_cursor.saturating_add(1);
                let action = app.shell.world3d_states.get_mut(surface.as_str()).and_then(infinite_world::world::step_world3d_brush_mesh_announce);
                if let Some(action) = action {
                    if let Err(_action) = app.frame_actions.try_push(action) {
                        runtime.record_frame_fault("frame brush mesh action credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::Build => {
                if self.build_cursor.is_none() {
                    let Some(presentation_witness) = runtime.presentation_witness_for(self.generation.0) else {
                        runtime.record_frame_fault("frame presentation input generation was stale before candidate construction");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    };
                    runtime.admit_build_presentation_witness(presentation_witness);
                    self.build_cursor = Some(FrameBuildCursor::new(presentation_witness, self.retained));
                    return AppFrameTransactionStep::Pending;
                }
                let Some(cursor) = self.build_cursor.as_mut() else { return AppFrameTransactionStep::Pending };
                match app.frame_before_input_step(handle, directives, cursor, context) {
                    FrameBuildBoundaryStep::Pending => AppFrameTransactionStep::Pending,
                    FrameBuildBoundaryStep::Complete(partial) => {
                        self.build_cursor = None;
                        self.after_chrome = Some(partial);
                        self.stage = FrameTransactionStage::RouteIntents;
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    FrameBuildBoundaryStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                    FrameBuildBoundaryStep::CloseFailure(failure)=>AppFrameTransactionStep::CloseFailure(failure),
                }
            }
            AppFrameTransactionPhase::InputEvents => {
                let transfer = {
                    let app = &mut *app;
                    let interaction = app.interaction.as_mut().expect("runtime interaction state is worker-owned");
                    transfer_frame_input_action(&mut interaction.input, &mut app.frame_actions)
                };
                match transfer {
                    Ok(FrameInputActionStep::Pending | FrameInputActionStep::Transferred) => {
                        if self.after_chrome.is_none() {
                            runtime.record_frame_fault("frame input phase lost the retained chrome owner");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        return AppFrameTransactionStep::Pending;
                    }
                    Ok(FrameInputActionStep::Empty) => {
                        match app.input.drive_retained_action_step() {
                            Ok(Some(action)) => {
                                if app.frame_actions.try_push_queued(action).is_err() {
                                    runtime.record_frame_fault("retained input action credits exceeded");
                                    self.phase = AppFrameTransactionPhase::Terminal;
                                    return AppFrameTransactionStep::Fault;
                                }
                                return AppFrameTransactionStep::Pending;
                            }
                            Ok(None) if app.input.retained_action_pending() => return AppFrameTransactionStep::Pending,
                            Ok(None) => {}
                            Err(_) => {
                                runtime.record_frame_fault("retained input action transport failed");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                return AppFrameTransactionStep::Fault;
                            }
                        }
                        match engine_canvas::drive_text_editor_retained_action_step() {
                            Ok(engine_canvas::TextEditorRetainedActionStep::Pending) => return AppFrameTransactionStep::Pending,
                            Ok(engine_canvas::TextEditorRetainedActionStep::Ready(actions)) => {
                                let admitted = match actions.second {
                                    Some(second) => app.frame_actions.try_push_queued_pair(actions.first, second).is_ok(),
                                    None => app.frame_actions.try_push_queued(actions.first).is_ok(),
                                };
                                if !admitted {
                                    runtime.record_frame_fault("retained text editor action credits exceeded");
                                    self.phase = AppFrameTransactionPhase::Terminal;
                                    return AppFrameTransactionStep::Fault;
                                }
                                return AppFrameTransactionStep::Pending;
                            }
                            Ok(engine_canvas::TextEditorRetainedActionStep::Idle) => {}
                            Err(_) => {
                                runtime.record_frame_fault("retained text editor action transport failed");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                return AppFrameTransactionStep::Fault;
                            }
                        }
                        match engine_canvas::drive_text_editor_outbox_step(&mut app.input) {
                            Ok(true) => return AppFrameTransactionStep::Pending,
                            Ok(false) => {}
                            Err(fault) => {
                                log_debug(&format!("[TRACE] text editor outbox admission failed: {fault:?}"));
                                runtime.record_frame_fault("text editor outbox admission failed");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                return AppFrameTransactionStep::Fault;
                            }
                        }
                    }
                    Ok(FrameInputActionStep::Deferred) => {}
                    Err(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                if interpreter::drive_scene_interaction_step(&mut app.input) {
                    return AppFrameTransactionStep::Pending;
                }
                match interpreter::drive_clipboard_io_step(&mut app.input,context) { Ok(true)=>return AppFrameTransactionStep::Pending,Ok(false)=>{},Err(failure)=>return AppFrameTransactionStep::CloseFailure(failure) }
                let Some(next) = self.effect_opportunities.checked_add(1) else {
                    runtime.record_frame_fault("frame effect opportunity counter exhausted");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if next > EFFECT_STORM_BUDGET {
                    runtime.record_frame_fault("frame effect storm budget exhausted");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                self.effect_opportunities = next;
                self.stage = FrameTransactionStage::FlushEffects;
                self.phase = AppFrameTransactionPhase::FrameDeferred;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::FrameDeferred => {
                let mut deferred = app.pending_frame_deferred.take();
                if deferred.is_some() && !RuntimeApply::start_frame_deferred(&mut deferred, &mut app, handle) {
                    app.pending_frame_deferred = deferred;
                    return AppFrameTransactionStep::Pending;
                }
                self.phase = AppFrameTransactionPhase::BoardAuthority;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::BoardAuthority => {
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.board2d_states.id_at(self.board_authority_cursor) else {
                    self.board_authority_cursor = 0;
                    self.phase = AppFrameTransactionPhase::World3dSnapshot;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("board authority surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let Some(surface) = interaction.shell.board2d_states.get(&surface_id) else {
                    runtime.record_frame_fault("board authority surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("board authority controller identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let controller_id = surface.controller_id.clone();
                match engine_canvas::drive_board_authority_step(&surface_id, context) {
                    infinite_canvas::BoardAuthorityStep::Pending => AppFrameTransactionStep::Pending,
                    infinite_canvas::BoardAuthorityStep::Cancelled => {
                        if let Err(fault) = engine_canvas::release_board_pointer_claim(&surface_id, &mut app.input) {
                            app.input.record_action_fault(fault);
                            runtime.record_frame_fault("board cancelled claim release faulted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        self.board_authority_cursor += 1;
                        AppFrameTransactionStep::Pending
                    }
                    infinite_canvas::BoardAuthorityStep::Fault => {
                        let _ = engine_canvas::release_board_pointer_claim(&surface_id, &mut app.input);
                        runtime.record_frame_fault("board retained authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                    infinite_canvas::BoardAuthorityStep::Complete => match engine_canvas::publish_board_pointer_step(&surface_id, &mut app.input) {
                        Ok(true) => {
                            self.phase = AppFrameTransactionPhase::InputEvents;
                            AppFrameTransactionStep::Pending
                        }
                        Ok(false) => match engine_canvas::publish_board_event_step(&surface_id, &controller_id, &mut app.input) {
                            Ok(true) => {
                                self.phase = AppFrameTransactionPhase::InputEvents;
                                AppFrameTransactionStep::Pending
                            }
                            Ok(false) => {
                                self.board_authority_cursor += 1;
                                AppFrameTransactionStep::Pending
                            }
                            Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                                self.phase = AppFrameTransactionPhase::InputEvents;
                                AppFrameTransactionStep::Pending
                            }
                            Err(fault) => {
                                app.input.record_action_fault(fault);
                                runtime.record_frame_fault("board event publication faulted");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                AppFrameTransactionStep::Fault
                            }
                        },
                        Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                            self.phase = AppFrameTransactionPhase::InputEvents;
                            AppFrameTransactionStep::Pending
                        }
                        Err(fault) => {
                            app.input.record_action_fault(fault);
                            runtime.record_frame_fault("board retained publication faulted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            AppFrameTransactionStep::Fault
                        }
                    },
                }
            }
            AppFrameTransactionPhase::World3dSnapshot => {
                let Some(interaction) = app.interaction.as_mut() else {
                    return AppFrameTransactionStep::Pending;
                };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(self.world3d_authority_cursor) else {
                    self.world3d_authority_cursor = 0;
                    self.phase = AppFrameTransactionPhase::World3dAuthority;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("world3d snapshot surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let Some(state) = interaction.shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d snapshot surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                world3d_ingest_trace(&surface_id, state, "phase-entry");
                let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
                let terrain=infinite_world::world::step_world3d_terrain(state,terrain_grant,context);
                if !terrain.ownership.fits(terrain_grant) {
                    runtime.record_frame_fault("world3d terrain ownership receipt exceeded caller grants");
                    self.phase=AppFrameTransactionPhase::Terminal;return AppFrameTransactionStep::Fault;
                }
                if let infinite_world::world::WorldTerrainMeshPublicationStep::Fault(fault)=terrain.step {
                    runtime.record_frame_fault(&format!("world3d terrain ownership faulted: {fault:?}"));
                    self.phase=AppFrameTransactionPhase::Terminal;return AppFrameTransactionStep::Fault;
                }

                if retire_cancelled_world3d_asset_step(state) {
                    context.consume_fuel(1);
                    return AppFrameTransactionStep::Pending;
                }
                match step_world3d_scene_bridge(state, context) {
                    World3dSceneBridgeStep::Pending => return AppFrameTransactionStep::Pending,
                    World3dSceneBridgeStep::Fault => {
                        log_debug(&format!("[TRACE] world3d bridge fault surface={surface_id} {}", state.ingest_census()));
                        runtime.record_frame_fault("world3d scene mesh-wire bridge faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    World3dSceneBridgeStep::Idle | World3dSceneBridgeStep::Complete => {}
                }
                match step_world3d_draw_rebuild(state, context) {
                    WorldDrawRebuildStep::Pending | WorldDrawRebuildStep::Complete => {}
                    WorldDrawRebuildStep::Stale => {
                        log_debug(&format!("[TRACE] world3d draw rebuild surface={surface_id} step=Stale {}", state.ingest_census()));
                        close_world3d_draw_rebuild_step(state, context);
                        return AppFrameTransactionStep::Pending;
                    }
                    WorldDrawRebuildStep::Fault => {
                        log_debug(&format!("[TRACE] world3d draw rebuild surface={surface_id} step=Fault {}", state.ingest_census()));
                        runtime.record_frame_fault("world3d retained draw rebuild faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                let snapshot_step = step_world3d_snapshot(state, context);
                world3d_ingest_trace(&surface_id, state, &format!("snapshot-{snapshot_step:?}"));
                if matches!(snapshot_step, World3dSnapshotApplyStep::Complete) {
                    log_debug(&format!("[TRACE] world3d delivery applied surface={surface_id} {}", state.ingest_census()));
                }
                match snapshot_step {
                    World3dSnapshotApplyStep::Idle | World3dSnapshotApplyStep::Complete => {
                        step_world3d_camera_fit_after_snapshot(state);
                        self.world3d_authority_cursor += 1;
                        AppFrameTransactionStep::Pending
                    }
                    World3dSnapshotApplyStep::Pending => AppFrameTransactionStep::Pending,
                    World3dSnapshotApplyStep::Stale | World3dSnapshotApplyStep::Fault => {
                        runtime.record_frame_fault("world3d typed snapshot apply faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::World3dAuthority => {
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(self.world3d_authority_cursor) else {
                    self.world3d_authority_cursor = 0;
                    self.stage = FrameTransactionStage::ReconcileTree;
                    self.phase = AppFrameTransactionPhase::RasterUploads;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("world3d authority surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let AppInteractionState { shell, input, .. } = interaction;
                let modal = shell.pointer_input_is_modal();
                let Some(state) = shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d authority surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if modal && world3d_hover_clear_is_owed(state) && enqueue_world3d_event(state, WorldInteractionIntent::pointer_leave(state.bounds.x - 1.0, state.bounds.y - 1.0)).is_err() {
                    input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                let Some(generation) = world3d_interaction_front_generation(state) else {
                    self.world3d_authority_cursor += 1;
                    return AppFrameTransactionStep::Pending;
                };
                world3d_interaction_trace(&surface_id, state, generation, None);
                let interaction_step = step_world3d_interaction(state, generation, input, context);
                world3d_interaction_trace(&surface_id, state, generation, Some(interaction_step));
                match interaction_step {
                    WorldInteractionAuthorityStep::Idle | WorldInteractionAuthorityStep::Complete | WorldInteractionAuthorityStep::Stale => {
                        self.world3d_authority_cursor += 1;
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    WorldInteractionAuthorityStep::Pending => AppFrameTransactionStep::Pending,
                    WorldInteractionAuthorityStep::OutputBlocked => {
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    WorldInteractionAuthorityStep::Fault => {
                        runtime.record_frame_fault("world3d retained interaction authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::RasterUploads => {
                if let Some(upload) = ui_wgpu::wgpu::take_ui_image_upload() {
                    scenes::queue_decoded_raster_upload("ui-image", upload.key, upload.width, upload.height, upload.pixels);
                    return AppFrameTransactionStep::Pending;
                }
                let cursor = self.raster_uploads.get_or_insert_with(Default::default);
                match cursor.step() {
                    scenes::PendingRasterUploadStep::Pending => AppFrameTransactionStep::Pending,
                    scenes::PendingRasterUploadStep::Upload(checked_out) => {
                        let Ok(mut producer) = checked_out.take() else {
                            runtime.record_frame_fault("frame raster producer checkout was stale");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(partial) = self.after_chrome.as_mut() else {
                            runtime.record_frame_fault("raster upload phase lost the retained chrome owner");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(input) = partial.resource_input.as_mut() else {
                            runtime.record_frame_fault("raster upload phase lost the retained resource input");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(upload_items) = input.raster_producers.len().checked_add(input.uploads.len()) else {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster upload item credits exhausted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        if upload_items >= input.limits.max_upload_items {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster upload item credits exceeded");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        if !producer.bind_frame_generation(input.preview_generation) {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster producer generation was stale");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        if let Err(producer) = input.try_push_raster_producer(producer) {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster producer fixed admission was refused");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        AppFrameTransactionStep::Pending
                    }
                    scenes::PendingRasterUploadStep::Complete => {
                        self.raster_uploads = None;
                        self.stage = FrameTransactionStage::BuildRenderPackets;
                        self.phase = AppFrameTransactionPhase::Finish;
                        AppFrameTransactionStep::Pending
                    }
                    scenes::PendingRasterUploadStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::Finish => {
                let Some(partial) = self.after_chrome.as_mut() else {
                    runtime.record_frame_fault("frame finish lost the retained chrome owner");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let cursor = self.finish_cursor.get_or_insert_with(Default::default);
                match app.frame_after_input_step(runtime, partial, cursor, context) {
                    FrameFinishBoundaryStep::Pending => AppFrameTransactionStep::Pending,
                    FrameFinishBoundaryStep::Complete(frame) => {
                        self.after_chrome = None;
                        self.finish_cursor = None;
                        self.stage = FrameTransactionStage::PublishSnapshot;
                        self.phase = AppFrameTransactionPhase::Terminal;
                        self.directives = None;
                        AppFrameTransactionStep::Complete(frame)
                    }
                    FrameFinishBoundaryStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::Terminal => AppFrameTransactionStep::Pending,
        }
    }

    /// ♻️ Preserves the camera child's physical receipts before any other frame child is considered.
    pub(crate) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let grant=cx.retained_grant();
        if let Some(camera) = self.scene_camera_cursor.as_mut() {
            if !camera.terminal_is_empty() { return frame_receive_close(frame_child_step(camera.close_step(grant),grant,camera.terminal_is_empty()),cx); }
            let bytes = std::mem::size_of::<Option<scenes::SceneCameraDispatchCursor>>();
            if let Some(step) = frame_inline_gate(grant,bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.scene_camera_cursor = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if let Some(cursor) = self.finish_cursor.as_mut() {
            if !cursor.terminal_is_empty() { return frame_receive_close(frame_child_step(cursor.close_step(grant),grant,cursor.terminal_is_empty()),cx); }
            let bytes = std::mem::size_of::<Option<FrameFinishCursor>>();
            if let Some(step) = frame_inline_gate(grant,bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.finish_cursor = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if let Some(cursor) = self.build_cursor.as_mut() {
            if !cursor.terminal_is_empty(){return frame_received_child(cursor.close_step(cx),grant,cursor.terminal_is_empty())}
        }
        if grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if let Some(owner)=self.after_chrome.as_mut(){if !owner.terminal_is_empty(){return frame_received_child(owner.close_step(cx),grant,owner.terminal_is_empty())}}
        frame_receive_close(frame_unpriced_child(),cx)
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FrameTransactionStage::DrainProjectionDeltas => "DrainProjectionDeltas",
            FrameTransactionStage::RouteIntents => "RouteIntents",
            FrameTransactionStage::FlushEffects => "FlushEffects",
            FrameTransactionStage::PresentSurface => "PresentSurface",
            FrameTransactionStage::ReconcileTree => "ReconcileTree",
            FrameTransactionStage::BuildRenderPackets => "BuildRenderPackets",
            FrameTransactionStage::PublishSnapshot => "PublishSnapshot",
        }
    }

    /// 🧭️ Other nested frame children still require their own physical retirement witnesses.
    pub(crate) fn terminal_is_empty(&self) -> bool {
        false
    }

    pub(crate) fn discard_presented_input_candidate(&mut self, runtime: &RuntimeMailbox) -> bool {
        if let Some(cursor) = self.build_cursor.as_mut() {
            if !discard_frame_input_candidate(runtime, &mut cursor.input_candidate) {
                return false;
            }
        }
        if let Some(partial) = self.after_chrome.as_mut() {
            if !discard_frame_input_candidate(runtime, &mut partial.input_candidate) {
                return false;
            }
        }
        true
    }
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

pub(crate) fn discard_frame_input_candidate(runtime: &RuntimeMailbox, candidate: &mut Option<shell::PresentedInputCandidateWitness>) -> bool {
    let Some(witness) = *candidate else { return true };
    let Ok(mut runtime) = runtime.try_lock() else { return false };
    let Some(interaction) = runtime.interaction.as_mut() else { return false };
    if !interaction.shell.discard_presented_input_candidate(witness) {
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
