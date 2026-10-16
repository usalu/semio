//! 🧠️ App runtime: native asset streams, runtime and interaction state implementations, world3d traces and the boot runtime.
use super::*;

//#region 📮️RuntimeMailbox
/// 🧪️ P3c: `self_weak` was the only field that made `AppRuntime` definitionally `Rc<RefCell<_>>`-owned
/// (see `AppHandle`'s own doc comment above). With it gone, this assertion lets the compiler — not a
/// person re-deriving the per-field audit by hand every time a field is added — settle whether the
/// struct is `Send` today. The mounted native compiler gate exercising this assertion is recorded in
/// `📓️p3c-explicit-app-handle.md`; wasm32 deliberately excludes it because the renderer's
/// browser-side handles are single-threaded platform values.
#[cfg(not(target_arch = "wasm32"))]
const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<AppRuntime>();
    assert_send::<AppFrameBuild>();
    assert_send::<AppFramePresentation>();
};

/// 🩺️ Names the LANE a refused world resource came from, instead of reporting one message for all
/// four. A frame fault kills the page, so the message is the whole post-mortem a probe gets: the
/// generic wording cost a full rebuild-and-rerun cycle to tell a raster ledger exhaustion (what it
/// actually was) from a mesh, eviction or upload overflow (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md`). The raster admission
/// carries its OWN refusal, so that one is named exactly.
fn world_rejection_fault(rejected: &infinite_world::world::World3dBuildRejected) -> &'static str {
    match rejected {
        infinite_world::world::World3dBuildRejected::Upload(_) => "frame world upload admission exceeded fixed credits",
        infinite_world::world::World3dBuildRejected::RasterProducer(_) => "frame world raster producer admission exceeded fixed credits",
        infinite_world::world::World3dBuildRejected::RasterAdmission(rejected) => rejected.fault(),
        infinite_world::world::World3dBuildRejected::Eviction(_) => "frame world eviction admission exceeded fixed credits",
    }
}

impl MailboxRuntime for AppRuntime {
    fn interaction(&self) -> Option<&AppInteractionState> {
        self.interaction.as_ref()
    }

    fn interaction_mut(&mut self) -> Option<&mut AppInteractionState> {
        self.interaction.as_mut()
    }

    fn interaction_available(&self) -> bool {
        AppRuntime::interaction_available(self)
    }

    fn check_out_interaction(&mut self, site: &'static str) -> Option<AppInteractionState> {
        AppRuntime::check_out_interaction(self, site)
    }

    fn return_interaction(&mut self, interaction: AppInteractionState) {
        AppRuntime::return_interaction(self, interaction)
    }

    fn resize(&mut self, logical_width: f32, logical_height: f32, scale_factor: f32) {
        AppRuntime::resize(self, logical_width, logical_height, scale_factor)
    }

    fn pending_frame_deferred(&self) -> &Option<FrameDeferredCursor> {
        &self.pending_frame_deferred
    }

    fn pending_frame_deferred_mut(&mut self) -> &mut Option<FrameDeferredCursor> {
        &mut self.pending_frame_deferred
    }

    fn frame_actions_mut(&mut self) -> &mut FrameActionOwners {
        &mut self.frame_actions
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal(&self) -> &Option<FrameMaintenanceRefusal> {
        &self.pending_frame_maintenance_refusal
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pending_frame_maintenance_refusal_mut(&mut self) -> &mut Option<FrameMaintenanceRefusal> {
        &mut self.pending_frame_maintenance_refusal
    }
}

impl AppRuntime {
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_native_plugin_hot_swap(&mut self) {
        if let Some(entries) = self.native_hot_swap_modified.as_mut() {
            if let Some((path, mtime)) = entries.pop() {
                let previous = self.native_plugin_mtimes.get(&path);
                if previous.is_some_and(|previous| *previous != mtime) {
                    self.native_reload_pending = true;
                }
                self.native_plugin_mtimes.insert(path, mtime);
                return;
            }
            self.native_hot_swap_modified = None;
            return;
        }
        if let Some(scan) = self.native_hot_swap_scan.as_mut() {
            let Some(result) = scan.try_take() else { return };
            self.native_hot_swap_scan = None;
            match result {
                Ok(semio_framework_os_services::NativeIoValue::Modified(entries)) => self.native_hot_swap_modified = Some(entries),
                Ok(_) => log_debug("plugin hot-swap scan returned the wrong native I/O value"),
                Err(error) => log_debug(&format!("plugin hot-swap scan failed: {error}")),
            }
            return;
        }
        let cursor = self.native_hot_swap_cursor % self.shell.plugins.plugins.len().max(1);
        self.native_hot_swap_cursor = self.native_hot_swap_cursor.wrapping_add(1);
        let Some(program) = self.shell.plugins.plugins.get(cursor) else { return };
        let Some(path) = program.wasm_artifact_path().map(std::path::Path::to_path_buf) else { return };
        let mut paths = semio_framework_os_services::NativePathSet::new();
        if paths.try_push(path).is_err() {
            return;
        }
        self.native_hot_swap_scan = submit_renderer_io(semio_framework_os_services::NativeIoRequest::Modified(paths)).ok();
    }

    /// 🎠️ Hot-reload preparation snapshots only the filter and module root. Loading runs on the
    /// process pool; its completion re-enters through the runtime mailbox.
    #[cfg(not(target_arch = "wasm32"))]
    fn maybe_reload_native_plugins(&mut self, handle: &AppHandle) {
        if !self.native_reload_pending {
            return;
        }
        self.native_reload_pending = false;
        let plugin_filter = self.shell.plugins.plugin_filter.clone();
        let modules_root = self.plugin_modules_root.clone();
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else { return };
        let accepted = mailbox.reserve_future(Some("plugin-reload"));
        if accepted {
            let revision = mailbox.0.next_revision.fetch_add(1, Ordering::Relaxed);
            spawn_app_task(async move {
                let result = load_wasm_plugins(&plugin_filter, &modules_root).await.map(|entries| filter_plugins(entries, &plugin_filter));
                mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: true, restores_interaction: false, apply: RuntimeApply::PluginReload(Some(result)) });
            });
        }
        if !accepted {
            self.native_reload_pending = true;
        }
    }

    /// 🧵️ Advances exactly one retained pre-input frame owner or chrome child.
    ///
    /// ♻️ Retires the registry the PREVIOUS build left staged, one entry per boundary
    /// step. The pointer's own authority is the last COMPLETE frame's buffer, which this
    /// never touches — see `ui_wgpu::wgpu::HitRegistry`.
    pub fn frame_before_input_step(&mut self, handle: &AppHandle, build_directives: &FrameDirectives, cursor: &mut FrameBuildCursor, context: &mut semio_framework_job::StepContext<'_>) -> FrameBuildBoundaryStep {
        match cursor.phase {
            FrameBuildPhase::Deferred => {
                let grant = context.retained_grant();
                let step = ui_wgpu::wgpu::PreparedAtlasPages::close_abandoned_step(grant);
                match frame_prepared_abandonment_complete(grant, step,context) {
                    Ok(true) => cursor.phase = FrameBuildPhase::PreparedGpuAbandonment,
                    Ok(false) => return FrameBuildBoundaryStep::Pending,
                    Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure),
                }
            }
            FrameBuildPhase::PreparedGpuAbandonment => {
                if !ui_gpu::PreparedGpuPresentCursor::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::PreparedInputAbandonment;
            }
            FrameBuildPhase::PreparedInputAbandonment => {
                let grant=context.retained_grant();
                match frame_prepared_abandonment_complete(grant, ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step(grant),context) {
                    Ok(true) => cursor.phase = FrameBuildPhase::PreparedJobAbandonment,
                    Ok(false) => return FrameBuildBoundaryStep::Pending,
                    Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure),
                }
            }
            FrameBuildPhase::PreparedJobAbandonment => {
                let grant=context.retained_grant();
                match frame_prepared_abandonment_complete(grant, ui_wgpu::wgpu::PreparedRenderJob::close_abandoned_step(grant),context) {
                    Ok(true) => cursor.phase = FrameBuildPhase::PreparedAbandonment,
                    Ok(false) => return FrameBuildBoundaryStep::Pending,
                    Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure),
                }
            }
            FrameBuildPhase::PreparedAbandonment => {
                let grant=context.retained_grant();
                match frame_prepared_abandonment_complete(grant, ui_wgpu::wgpu::PreparedRenderReceiver::close_abandoned_step(grant),context) {
                    Ok(true) => cursor.phase = FrameBuildPhase::PacketAbandonment,
                    Ok(false) => return FrameBuildBoundaryStep::Pending,
                    Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure),
                }
            }
            FrameBuildPhase::PacketAbandonment => {
                let grant=context.retained_grant();
                match frame_prepared_abandonment_complete(grant, ui_wgpu::wgpu::PreparedRenderPacket::close_abandoned_step(grant),context) {
                    Ok(true) => cursor.phase = FrameBuildPhase::Text,
                    Ok(false) => return FrameBuildBoundaryStep::Pending,
                    Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure),
                }
            }
            FrameBuildPhase::Text => {
                self.drive_text_operation();
                cursor.phase = FrameBuildPhase::Fullscreen;
            }
            FrameBuildPhase::Fullscreen => {
                cursor.fullscreen = std::mem::take(&mut self.shell.chrome.fullscreen_toggle_requested).then(|| {
                    self.shell.chrome.fullscreen_active = !self.shell.chrome.fullscreen_active;
                    self.shell.chrome.fullscreen_active
                });
                cursor.phase = FrameBuildPhase::NativePoll;
            }
            FrameBuildPhase::NativePoll => {
                #[cfg(not(target_arch = "wasm32"))]
                self.poll_native_plugin_hot_swap();
                cursor.phase = FrameBuildPhase::NativeReload;
            }
            FrameBuildPhase::NativeReload => {
                #[cfg(not(target_arch = "wasm32"))]
                self.maybe_reload_native_plugins(handle);
                cursor.phase = FrameBuildPhase::ThemeResolve;
            }
            FrameBuildPhase::ThemeResolve => {
                self.theme = semio_framework_os_renderer_wgpu_shell_docs::resolve_theme_for_ids(self.shell.chrome.active_theme_id_for_frame(), &self.shell.prefs.appearance_id);
                cursor.phase = FrameBuildPhase::ThemeAppearance;
            }
            FrameBuildPhase::ThemeAppearance => {
                self.theme_dark = appearance_is_dark(&self.shell.prefs.appearance_id);
                cursor.phase = FrameBuildPhase::Drag;
            }
            FrameBuildPhase::Drag => {
                if !self.pointer_down && self.input.drag.active {
                    self.input.end_drag();
                }
                cursor.phase = FrameBuildPhase::Hover;
            }
            FrameBuildPhase::Hover => {
                let (hover_x, hover_y) = (self.last_pointer_x, self.last_pointer_y);
                self.input.update_hover(hover_x, hover_y);
                cursor.phase = FrameBuildPhase::InputFrame;
            }
            FrameBuildPhase::InputFrame => {
                if self.input.retire_hit_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                self.input.wheel_delta = 0.0;
                self.input.right_click_pos = None;
                cursor.phase = FrameBuildPhase::WheelDeadline;
            }
            FrameBuildPhase::WheelDeadline => {
                if build_directives.wheel_zoom_deadline_cleared && self.wheel_zoom_deadline_ms > 0.0 && app_now_ms() >= self.wheel_zoom_deadline_ms {
                    self.wheel_zoom_deadline_ms = 0.0;
                    semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_clear_wheel_zoom_active();
                }
                cursor.phase = FrameBuildPhase::RetainedClock;
            }
            FrameBuildPhase::RetainedClock => {
                if let Some((window_id, surface)) = self.shell.input.retained_clock_surface(cursor.retained_clock_index).map(|(id, surface)| (id.to_owned(), surface)) {
                    cursor.retained_clock_index += 1;
                    if let Some(step) = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::advance_retained_window_clock(&window_id, surface, &mut self.input) {
                        if let Some(deadline) = step.next_deadline {
                            cursor.retained_clock_deadline = Some(cursor.retained_clock_deadline.map_or(deadline, |current| current.min(deadline)));
                        }
                    }
                } else {
                    let Some(runtime) = handle.upgrade().map(RuntimeMailbox) else { return FrameBuildBoundaryStep::Fault("control clock lost its runtime owner") };
                    runtime.publish_retained_control_deadline(cursor.retained_clock_deadline);
                    cursor.phase = FrameBuildPhase::TakeDraw;
                }
            }
            FrameBuildPhase::TakeDraw => {
                cursor.previous_draw = Some(std::mem::take(&mut self.draw));
                cursor.phase = FrameBuildPhase::RetireDraw;
            }
            FrameBuildPhase::RetireDraw => {
                let Some(previous) = cursor.previous_draw.as_mut() else {
                    cursor.phase = FrameBuildPhase::TakeOverlay;
                    return FrameBuildBoundaryStep::Pending;
                };
                if previous.retire_step() {
                    cursor.previous_draw = None;
                    cursor.phase = FrameBuildPhase::TakeOverlay;
                }
            }
            FrameBuildPhase::TakeOverlay => {
                cursor.previous_overlay = Some(std::mem::take(&mut self.overlay));
                cursor.phase = FrameBuildPhase::RetireOverlay;
            }
            FrameBuildPhase::RetireOverlay => {
                let Some(previous) = cursor.previous_overlay.as_mut() else {
                    cursor.phase = FrameBuildPhase::IconAtlas;
                    return FrameBuildBoundaryStep::Pending;
                };
                if previous.retire_step() {
                    cursor.previous_overlay = None;
                    cursor.phase = FrameBuildPhase::IconAtlas;
                }
            }
            FrameBuildPhase::IconAtlas => {
                if !cursor.icon_checked {
                    let mut pending = false;
                    let mut observed = false;
                    ICON_ATLAS_RUNTIME.with(|cell| {
                        if let Some(atlas) = cell.borrow_mut().take() {
                            observed = true;
                            self.icons = atlas;
                            match ui_wgpu::wgpu::PreparedAtlasPages::try_new(self.icons.width, self.icons.height, 4, self.icons.pixels.len(), ui_wgpu::wgpu::PreparedAtlasAuthority { normal: context.retained_grant(), retirement: cursor.retained }) {
                                Ok((pages, progress)) => { cursor.icon_pages = Some(pages); pending = context.consume_retained(progress).is_ok(); }
                                Err(error) => { let _ = context.consume_retained(error.retained_progress()); }
                            }
                        }
                    });
                    cursor.icon_checked = true;
                    if observed && !pending {
                        return FrameBuildBoundaryStep::Fault("frame icon atlas page admission failed");
                    }
                    return FrameBuildBoundaryStep::Pending;
                }
                if let Some(pages) = cursor.icon_pages.as_mut() {
                    if !pages.slots_are_admitted() {
                        let progress = match pages.admit_original_slots(context.retained_grant()) { Ok(progress) => progress, Err(_) => return FrameBuildBoundaryStep::Fault("frame icon slot authority was refused") };
                        if context.consume_retained(progress).is_err() { return FrameBuildBoundaryStep::Fault("frame icon slot receipt exceeded original authority"); }
                        return FrameBuildBoundaryStep::Pending;
                    }
                    let complete = match pages.push_page(&self.icons.pixels, pages.next_row(), context.retained_grant()) {
                        Ok((complete, progress)) => { if context.consume_retained(progress).is_err() { return FrameBuildBoundaryStep::Fault("frame icon page receipt exceeded original authority"); } complete },
                        Err(_) => return FrameBuildBoundaryStep::Fault("frame icon atlas page construction faulted"),
                    };
                    if !complete {
                        return FrameBuildBoundaryStep::Pending;
                    }
                    let Some(pages) = cursor.icon_pages.take() else { return FrameBuildBoundaryStep::Fault("frame icon atlas pages lost ownership") };
                    cursor.icon_upload = Some(ui_wgpu::wgpu::PreparedRenderUpload::IconAtlasPages { pixels: pages });
                }
                cursor.phase = FrameBuildPhase::Tutorial;
            }
            FrameBuildPhase::Tutorial => {
                let wall_now_ms = app_now_ms();
                semio_framework_os_renderer_wgpu_shell_window_actions::WindowActionsCtx::of(&mut self.shell).tutorial_tick(wall_now_ms);
                if let Some(runtime) = handle.upgrade().map(RuntimeMailbox) {
                    let deadline = semio_framework_job::default_now_us().and_then(|now| self.shell.surfaces.next_chrome_deadline(now as f64 / 1_000_000.0, wall_now_ms, &self.shell.chrome, &self.shell.tutorial));
                    runtime.publish_shell_clock_deadline(deadline);
                }
                cursor.phase = FrameBuildPhase::EnginePacketBacking;
            }
            FrameBuildPhase::EnginePacketBacking => {
                match cursor.engine_packets.prepare_step(context) { Ok(true) => cursor.phase = FrameBuildPhase::EngineResources, Ok(false) => return FrameBuildBoundaryStep::Pending, Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure) }
            }
            FrameBuildPhase::EngineResources => {
                let resources=cursor.engine_resources.get_or_insert_with(semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::EngineCanvasBuildContext::default);
                match resources.prepare_step(context){Ok(true)=>cursor.phase=FrameBuildPhase::WorldResources,Ok(false)=>return FrameBuildBoundaryStep::Pending,Err(failure)=>return FrameBuildBoundaryStep::CloseFailure(failure)}
            }
            FrameBuildPhase::WorldResources => {
                if cursor.world_resources.is_none(){
                    let Some(runtime)=handle.upgrade().map(RuntimeMailbox)else{return FrameBuildBoundaryStep::Fault("frame runtime lost its wake authority")};
                    cursor.world_resources=Some(infinite_world::world::World3dBuildContext::new(runtime.world_cursor_wake_authority()));
                }
                let Some(resources)=cursor.world_resources.as_mut()else{return FrameBuildBoundaryStep::Fault("frame world preparation lost original resources")};
                match resources.prepare_step(context){Ok(true)=>cursor.phase=FrameBuildPhase::Chrome,Ok(false)=>return FrameBuildBoundaryStep::Pending,Err(failure)=>return FrameBuildBoundaryStep::CloseFailure(failure)}
            }
            FrameBuildPhase::Chrome => {
                if cursor.engine_resources.is_none() {
                    return FrameBuildBoundaryStep::Fault("frame chrome lost engine resources");
                }
                if cursor.world_resources.is_none() {
                    return FrameBuildBoundaryStep::Fault("frame chrome lost world resources");
                }
                let AppRuntime { atlas, icons, interaction, draw, overlay, .. } = self;
                let Some(interaction) = interaction.as_mut() else { return FrameBuildBoundaryStep::Fault("frame chrome lost interaction state") };
                let Some(world_resources) = cursor.world_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame chrome lost world resources") };
                let complete=match semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx::of(&mut interaction.shell).render_chrome_step(&mut cursor.chrome,draw,overlay,atlas,icons,&mut interaction.input,&interaction.theme,world_resources,context){Ok(complete)=>complete,Err(failure)=>return FrameBuildBoundaryStep::CloseFailure(failure)};
                if complete {
                    let input_candidate = match semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx::of(&mut interaction.shell).seal_presented_input_candidate(&interaction.theme) {
                        Ok(witness) => witness,
                        Err(fault) => return FrameBuildBoundaryStep::Fault(fault),
                    };
                    cursor.input_candidate = Some(input_candidate);
                    semio_framework_os_renderer_wgpu_shell_session::SessionCtx::of(&mut interaction.shell).sync_engine_surface_states();
                    cursor.phase = FrameBuildPhase::JobProgressTake;
                }
            }
            FrameBuildPhase::JobProgressTake => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    cursor.job_progress = kernel_runtime::take_job_progress_presentation();
                }
                cursor.phase = FrameBuildPhase::JobProgressOverlay;
            }
            FrameBuildPhase::JobProgressOverlay => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(progress) = cursor.job_progress.as_ref() {
                    let (kind, applied) = progress.visual();
                    let theme = self.interaction.as_ref().map_or_else(Theme::default, |state| state.theme);
                    let color = match kind {
                        semio_framework_actor::JobProgressKind::CommitValidated => theme.success,
                        semio_framework_actor::JobProgressKind::Cancelled | semio_framework_actor::JobProgressKind::Fault => theme.error,
                        _ => theme.progress,
                    };
                    self.overlay.push_solid_overlay([12.0, 12.0, 24.0 + applied.min(100) as f32 * 2.0, 4.0], color);
                }
                cursor.phase = FrameBuildPhase::ResourceInput;
            }
            FrameBuildPhase::ResourceInput => {
                match ui_wgpu::wgpu::PreparedRenderInput::try_new(cursor.presentation_witness.scene_revision, cursor.presentation_witness.input_generation, DrawList::empty(), None, 0.0) {
                    Ok(input) => cursor.resource_input = Some(input),
                    Err(rejected) => {
                        cursor.input_rejected = Some(rejected);
                        return FrameBuildBoundaryStep::Fault("frame prepared input process admission was refused");
                    }
                }
                cursor.phase = FrameBuildPhase::WorldTransfer;
            }
            FrameBuildPhase::WorldTransfer => {
                let Some(resources) = cursor.world_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame transfer lost world resources") };
                let Some(input) = cursor.resource_input.as_mut() else { return FrameBuildBoundaryStep::Fault("frame transfer lost resource input") };
                match resources.append_step(input) {
                    Ok(true) => cursor.phase = FrameBuildPhase::IconTransfer,
                    Ok(false) => {}
                    Err(rejected) => {
                        let fault = world_rejection_fault(&rejected);
                        cursor.world_rejected = Some(rejected);
                        return FrameBuildBoundaryStep::Fault(fault);
                    }
                }
            }
            FrameBuildPhase::IconTransfer => {
                if let Some(upload) = cursor.icon_upload.take() {
                    let Some(input) = cursor.resource_input.as_mut() else { return FrameBuildBoundaryStep::Fault("frame icon transfer lost resource input") };
                    if let Err(upload) = input.try_push_upload(upload) {
                        cursor.world_rejected = Some(infinite_world::world::World3dBuildRejected::Upload(upload));
                        return FrameBuildBoundaryStep::Fault("frame icon upload fixed admission was refused");
                    }
                }
                cursor.phase = FrameBuildPhase::EngineTransfer;
            }
            FrameBuildPhase::EngineTransfer => {
                let Some(resources) = cursor.engine_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame transfer lost engine resources") };
                if !semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::stage_engine_packet_step(resources) {
                    return FrameBuildBoundaryStep::Pending;
                }
                match resources.take_packet_step() {
                    Ok(Some(packet)) => {
                        if let Err(rejected) = cursor.engine_packets.try_push(packet) {
                            cursor.engine_rejected = Some(rejected);
                            return FrameBuildBoundaryStep::Fault("frame engine packet credits exceeded");
                        }
                    }
                    Ok(None) => cursor.phase = FrameBuildPhase::CursorWake,
                    Err(packet) => {
                        cursor.engine_rejected = Some(packet);
                        return FrameBuildBoundaryStep::Fault("frame engine packet producer exceeded fixed credits");
                    }
                }
            }
            FrameBuildPhase::CursorWake => {
                let Some(resources) = cursor.world_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame cursor wake lost world resources") };
                match resources.take_cursor_wake() {
                    Ok(token) => cursor.cursor_wake = token,
                    Err(_) => return FrameBuildBoundaryStep::Fault("frame cursor wake authority faulted"),
                }
                cursor.phase = FrameBuildPhase::Complete;
            }
            FrameBuildPhase::Complete => {
                if let Some(resources)=cursor.engine_resources.as_mut(){
                    if !resources.terminal_is_empty(){match resources.close_step(context){Ok(_)=>return FrameBuildBoundaryStep::Pending,Err(failure)=>return FrameBuildBoundaryStep::CloseFailure(failure)}}
                }
                if let Some(resources)=cursor.world_resources.as_mut(){
                    if !resources.terminal_is_empty(){
                        match resources.close_step(context){Ok(_)=>return FrameBuildBoundaryStep::Pending,Err(failure)=>return FrameBuildBoundaryStep::CloseFailure(failure)}
                    }
                }
                if cursor.engine_resources.as_ref().is_some_and(|resources| !resources.terminal_is_empty()) || cursor.world_resources.as_ref().is_some_and(|resources| !resources.terminal_is_empty()) {
                    return FrameBuildBoundaryStep::Fault("frame build resource cursor was not terminal-empty");
                }
                let Some(resource_input) = cursor.resource_input.take() else { return FrameBuildBoundaryStep::Fault("frame build lost resource input") };
                let engine_packets = std::mem::take(&mut cursor.engine_packets);
                return FrameBuildBoundaryStep::Complete(AppFrameAfterChrome {
                    retained: cursor.retained,
                    resource_input: Some(resource_input),
                    input_candidate: cursor.input_candidate.take(),
                    upload_rejected: None,
                    draw_rejected: None,
                    engine_packets: Some(engine_packets),
                    fullscreen: cursor.fullscreen.take(),
                    cursor_wake: cursor.cursor_wake.take(),
                    #[cfg(not(target_arch = "wasm32"))]
                    job_progress: cursor.job_progress.take(),
                    retirement: None,
                });
            }
        }
        FrameBuildBoundaryStep::Pending
    }

    /// 🎞️ Advances one retained post-input owner transfer without driving deferred work inline.
    ///
    /// 🫀️ The settle lane is re-read every frame, never latched across one: the shell's own
    /// predicate answers over armed work, an owed refresh scope and whichever producers
    /// report `computing`, so a frame owes a settle step exactly while there is a chain to
    /// converge (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `ShellSettlePump`).
    ///
    /// 🩺️ Only a frame that actually carries a dispatchable action says so — the
    /// tutorial/maintenance installs run several times a second and are not news.
    /// Its ABSENCE after a `frame input action` line was the defect
    /// `📓️wgpu-deferred-action-commit-2026-09-13.md` closes; a `frame input action`
    /// now always has one of these behind it, however many builds were superseded in
    /// between.
    pub fn frame_after_input_step(&mut self, runtime: &AppMailbox, partial: &mut AppFrameAfterChrome, cursor: &mut FrameFinishCursor, context: &mut semio_framework_job::StepContext<'_>) -> FrameFinishBoundaryStep {
        let cancel = context.cancel_token();
        match cursor.phase {
            FrameFinishPhase::Inputs => {
                runtime.update_frame_inputs(self);
                cursor.phase = FrameFinishPhase::Deferred;
            }
            FrameFinishPhase::Deferred => {
                cursor.flush_tutorial = !self.shell.tutorial.tutorial_pending_document_ops.is_empty();
                cursor.shell_maintenance = self.shell.surfaces.chrome_maintenance_pending(&self.shell.chrome);
                cursor.settle = semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut self.shell).settle_pump_pending();
                cursor.pump_sync = app_now_ms() - self.last_sync_pump_ms >= SHELL_SYNC_PUMP_INTERVAL_MS;
                if cursor.pump_sync {
                    self.last_sync_pump_ms = app_now_ms();
                }
                cursor.phase = FrameFinishPhase::IconRaster;
            }
            FrameFinishPhase::IconRaster => {
                if cursor.icon_pages.is_none() {
                    if self.icon_rebuild.is_none() { cursor.phase = FrameFinishPhase::GlyphUpload; return FrameFinishBoundaryStep::Pending; }
                    let Some(atlas) = self.drive_icon_raster_step() else { return FrameFinishBoundaryStep::Pending };
                    let result = ui_wgpu::wgpu::PreparedAtlasPages::try_new(atlas.width, atlas.height, 1, atlas.pixels.len(), ui_wgpu::wgpu::PreparedAtlasAuthority { normal: context.retained_grant(), retirement: partial.retained });
                    self.icons = atlas;
                    let (pages, progress) = match result { Ok(result) => result, Err(error) => { let _ = context.consume_retained(error.retained_progress()); return FrameFinishBoundaryStep::Fault("frame icon atlas page admission failed"); } };
                    cursor.icon_pages = Some(pages);
                    if context.consume_retained(progress).is_err() { return FrameFinishBoundaryStep::Fault("frame icon header receipt exceeded original authority"); }
                    return FrameFinishBoundaryStep::Pending;
                }
                let Some(pages) = cursor.icon_pages.as_mut() else { return FrameFinishBoundaryStep::Fault("frame icon atlas pages lost ownership") };
                if !pages.slots_are_admitted() {
                    let progress = match pages.admit_original_slots(context.retained_grant()) { Ok(progress) => progress, Err(_) => return FrameFinishBoundaryStep::Fault("frame icon slot authority was refused") };
                    if context.consume_retained(progress).is_err() { return FrameFinishBoundaryStep::Fault("frame icon slot receipt exceeded original authority"); }
                    return FrameFinishBoundaryStep::Pending;
                }
                let (complete, progress) = match pages.push_page(&self.icons.pixels, pages.next_row(), context.retained_grant()) { Ok(result) => result, Err(error) => { let _ = context.consume_retained(error.retained_progress()); return FrameFinishBoundaryStep::Fault("frame icon atlas page construction faulted"); } };
                if context.consume_retained(progress).is_err() { return FrameFinishBoundaryStep::Fault("frame icon page receipt exceeded original authority"); }
                if !complete { return FrameFinishBoundaryStep::Pending; }
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame icon transfer lost resource input") };
                let Some(pages) = cursor.icon_pages.take() else { return FrameFinishBoundaryStep::Fault("frame icon atlas pages lost ownership") };
                if let Err(rejected) = input.try_push_upload(ui_wgpu::wgpu::PreparedRenderUpload::IconAtlasPages { pixels: pages }) {
                    partial.upload_rejected = Some(rejected);
                    return FrameFinishBoundaryStep::Fault("frame icon upload fixed admission was refused");
                }
                cursor.phase = FrameFinishPhase::GlyphUpload;
            }
            FrameFinishPhase::GlyphUpload => {
                if !cursor.glyph_started {
                    if !self.atlas.take_dirty() {
                        cursor.phase = FrameFinishPhase::Cursor;
                        return FrameFinishBoundaryStep::Pending;
                    }
                    let Some(input) = partial.resource_input.as_ref() else { return FrameFinishBoundaryStep::Fault("frame glyph transfer lost resource input") };
                    let Some(next) = input.raster_producers.len().checked_add(input.uploads.len()).and_then(|count| count.checked_add(1)) else { return FrameFinishBoundaryStep::Fault("frame glyph upload credits exhausted") };
                    if next > input.limits.max_upload_items {
                        return FrameFinishBoundaryStep::Fault("frame glyph upload credits exceeded");
                    }
                    let Ok((pages, progress)) = ui_wgpu::wgpu::PreparedAtlasPages::try_new(self.atlas.width, self.atlas.height, 1, self.atlas.pixels.len(), ui_wgpu::wgpu::PreparedAtlasAuthority { normal: context.retained_grant(), retirement: partial.retained }) else {
                        return FrameFinishBoundaryStep::Fault("frame glyph atlas page admission failed");
                    };
                    cursor.glyph_pages = Some(pages);
                    if context.consume_retained(progress).is_err() { return FrameFinishBoundaryStep::Fault("frame glyph header receipt exceeded original authority"); }
                    cursor.glyph_started = true;
                    return FrameFinishBoundaryStep::Pending;
                }
                let Some(pages) = cursor.glyph_pages.as_mut() else { return FrameFinishBoundaryStep::Fault("frame glyph atlas pages lost ownership") };
                if !pages.slots_are_admitted() {
                    let progress = match pages.admit_original_slots(context.retained_grant()) { Ok(progress) => progress, Err(_) => return FrameFinishBoundaryStep::Fault("frame glyph slot authority was refused") };
                    if context.consume_retained(progress).is_err() { return FrameFinishBoundaryStep::Fault("frame glyph slot receipt exceeded original authority"); }
                    return FrameFinishBoundaryStep::Pending;
                }
                let complete = match pages.push_page(&self.atlas.pixels, pages.next_row(), context.retained_grant()) {
                    Ok((complete, progress)) => { if context.consume_retained(progress).is_err() { return FrameFinishBoundaryStep::Fault("frame glyph page receipt exceeded original authority"); } complete },
                    Err(_) => return FrameFinishBoundaryStep::Fault("frame glyph atlas page construction faulted"),
                };
                if !complete {
                    return FrameFinishBoundaryStep::Pending;
                }
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame glyph transfer lost resource input") };
                let Some(pages) = cursor.glyph_pages.take() else { return FrameFinishBoundaryStep::Fault("frame glyph atlas pages lost ownership") };
                if let Err(upload) = input.try_push_upload(ui_wgpu::wgpu::PreparedRenderUpload::GlyphAtlasPages { pixels: pages }) {
                    partial.upload_rejected = Some(upload);
                    return FrameFinishBoundaryStep::Fault("frame glyph upload fixed admission was refused");
                }
                cursor.phase = FrameFinishPhase::Cursor;
            }
            FrameFinishPhase::Cursor => {
                let hit = self.input.hit_at(self.last_pointer_x, self.last_pointer_y);
                let base_cursor = resolve_semio_cursor(
                    hit,
                    CursorDragState {
                        tree_drag: self.shell.input.tree_drag.is_some(),
                        dock_drag: self.shell.dock.dock_drag.is_some(),
                        pointer_drag_active: self.input.drag.active,
                        pointer_drag_axis: self.input.drag.axis,
                        pointer_drag_kind: self.input.drag.kind,
                    },
                );
                let connection_cursor = matches!(base_cursor, SemioCursor::Default)
                    && self
                        .shell.input
                        .scene_pointer_target_at(self.last_pointer_x, self.last_pointer_y, &self.input, &self.theme)
                        .filter(|target| target.kind == ui_wgpu::wgpu::SurfaceKind::NodeGraph)
                        .and_then(|target| {
                            self.shell.surfaces.node_graph_states.get(&target.host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                                semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_connection_cursor_active(
                                    &target.host_id,
                                    f64::from(self.last_pointer_x - surface.bounds.x),
                                    f64::from(self.last_pointer_y - surface.bounds.y),
                                )
                            })
                        })
                        .unwrap_or(false);
                let base_cursor = if connection_cursor { SemioCursor::CrosshairCentered } else { base_cursor };
                cursor.cursor = match self.shell.input.utility_cursor_override(self.last_pointer_x, self.last_pointer_y, &self.shell.docs, &self.shell.windows) {
                    Some(utility_cursor) if matches!(base_cursor, SemioCursor::Default | SemioCursor::Grab | SemioCursor::Selectable | SemioCursor::Pointer) => utility_cursor,
                    _ => base_cursor,
                };
                cursor.phase = FrameFinishPhase::Draw;
            }
            FrameFinishPhase::Draw => {
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame draw transfer lost resource input") };
                input.time_seconds = semio_framework_os_renderer_wgpu_host_plumbing::deadlines::ui_animation_seconds(semio_framework_job::default_now_us());
                cursor.phase = FrameFinishPhase::Overlay;
            }
            FrameFinishPhase::Overlay => {
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame overlay transfer lost resource input") };
                let draw = std::mem::replace(&mut self.draw, DrawList::empty());
                let overlay = Some(std::mem::replace(&mut self.overlay, DrawList::empty()));
                if let Err(rejected) = input.try_bind_draw(draw, overlay) {
                    partial.draw_rejected = Some(rejected);
                    return FrameFinishBoundaryStep::Fault("frame draw process admission was refused");
                }
                cursor.phase = FrameFinishPhase::Complete;
            }
            FrameFinishPhase::Complete => {
                let has_deferred = cursor.pump_sync || !self.frame_actions.is_empty() || cursor.flush_tutorial || cursor.shell_maintenance || cursor.settle;
                if has_deferred && self.pending_frame_deferred.is_some() {
                    let mut deferred = self.pending_frame_deferred.take();
                    let handle = runtime.downgrade();
                    if !RuntimeApply::start_frame_deferred(&mut deferred, self, &handle) {
                        self.pending_frame_deferred = deferred;
                        return FrameFinishBoundaryStep::Pending;
                    }
                }
                let Some(input) = partial.resource_input.take() else {
                    return FrameFinishBoundaryStep::Fault("frame completion lost resource input");
                };
                let Some(engine_packets) = partial.engine_packets.take() else {
                    partial.resource_input = Some(input);
                    return FrameFinishBoundaryStep::Fault("frame completion lost engine packets");
                };
                if has_deferred {
                    let deferred_actions = std::mem::take(&mut self.frame_actions);
                    if !deferred_actions.is_empty() {
                        log_debug("[TRACE] frame deferred install carries an action");
                    }
                    self.pending_frame_deferred = Some(FrameDeferredCursor::new(deferred_actions, cursor.pump_sync, cursor.flush_tutorial, cursor.shell_maintenance, cursor.settle, input.preview_generation, cancel));
                }
                return FrameFinishBoundaryStep::Complete(AppFrameBuild {
                    retained: partial.retained,
                    generation: semio_framework_trace::Generation(input.preview_generation),
                    input,
                    input_candidate: partial.input_candidate.take(),
                    engine_packets,
                    cursor: cursor.cursor,
                    theme_dark: self.theme_dark,
                    fullscreen: partial.fullscreen.take(),
                    cursor_wake: partial.cursor_wake.take(),
                    #[cfg(not(target_arch = "wasm32"))]
                    job_progress: partial.job_progress.take(),
                });
            }
        }
        FrameFinishBoundaryStep::Pending
    }
}

//#region 🔖️OsHostDecomposition — SemioApp deletion
// 🏚️ DELETED by ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host):
// `start_frame_loop` (used to live here, ~line 2291 pre-edit — the recursive `schedule_frame`
// rAF/timer chain that called `app.frame()` and immediately rescheduled itself, unconditionally,
// forever), `enum HostUserEvent` and `struct SemioApp` + its `ApplicationHandler` impl (`resumed`
// set `ControlFlow::Poll` at boot — ~line 2383 pre-edit; `window_event`'s `RedrawRequested` arm
// called `window.request_redraw()` unconditionally right after building a frame — ~line 2406
// pre-edit; `about_to_wait` polled a thread-local task pool then ALSO unconditionally
// `window.request_redraw()` every single iteration — ~line 2416-2424 pre-edit). Replaced by
// `winit_app::{HostUserEvent, WinitApp}` — same two-phase boot handshake, but steady-state control
// flow is `WaitUntil(next deadline)`/`Wait`, redraw only fires `if let Some(reason) =
// scheduler.should_render(now)`, while native continuations run on the process pool. See
// `📓️terra-os-host-report.md`'s redraw audit for the full
// before/after per site.
//#endregion 🔖️OsHostDecomposition — SemioApp deletion
//#endregion 📮️RuntimeMailbox

/// 🎥️ Advances camera residency after snapshot sealing. Frame construction precedes this phase, so
/// the pending witness deliberately schedules one more build when this step applies a new orbit.
pub fn step_world3d_camera_fit_after_snapshot(state: &mut infinite_world::world::World3dState) {
    let _ = step_world3d_camera_fit(state);
}

/// 🩺️ Rate-limited `[TRACE] ` trace of one World3d surface's MESH INGEST, taken where the frame
/// transaction actually drives it rather than where the chrome paints it. Behind the runtime
/// diagnostics switch like every other per-frame census (`🎞️Scenes`' own `world3d surface=…`), so a
/// boot with diagnostics off carries no world chatter at all.
///
/// ⚖️ A retained window republishes its cached paint while its revision is unchanged, so
/// `render_world_3d` — and with it the only existing world3d console line — runs once per DOCUMENT,
/// not once per frame. An ingest that stalls between two documents is therefore invisible: on 6118
/// the preview sat at `apply=true state-meshes=2 draws=0` with no further evidence for 95 s
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Every terminal step is reported, and a `Pending` one
/// every `WORLD3D_INGEST_TRACE_STRIDE` frames.
pub fn world3d_ingest_trace(surface_id: &str, state: &infinite_world::world::World3dState, stage: &str) {
    const WORLD3D_INGEST_TRACE_STRIDE: u32 = 512;
    static STEPS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if !infinite_world::world::world3d_cursor_work_pending(state) {
        return;
    }
    let seen = STEPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if seen % WORLD3D_INGEST_TRACE_STRIDE != 0 {
        return;
    }
    log_debug_diagnostic(&format!("[TRACE] world3d ingest surface={surface_id} stage={stage} steps={seen} {}", state.ingest_census()));
}

/// 🕹️ One `[TRACE] ` line per INTENT, not per authority step, and only while the runtime diagnostics
/// switch is armed — an orbit drag raises one intent per pointer move, so an ungated pair per intent
/// is per-frame chatter on exactly the gesture whose latency this ticket measures.
///
/// ⚖️ The authority answers `Pending` for as many turns as a ray-cast needs triangles, so a
/// per-step trace drowns the console while a per-frame one misses the single turn that decides the
/// outcome. The ENTER line is emitted once per queue generation and the LEAVE line only on a
/// terminal step, which is exactly one pair per pointer, wheel or button intent
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🐌️ An intent that never terminates is the one failure this trace exists for: the authority
/// answers `Pending` and the frame transaction re-enters the same phase forever, so without a
/// stride line the console shows an `enter` with no `leave` and nothing about WHERE it stopped.
/// 🔀️ A Pending step is logged only when the authority's own census CHANGED — that is exactly
/// its transition ladder (`Pick[Hover …]` → `Plan[0/1 Camera]` → …), which is what a step that
/// faults or never terminates has to be read off. A stride line on top catches a spin whose
/// census is constant.
pub fn world3d_interaction_trace(surface_id: &str, state: &infinite_world::world::World3dState, generation: u64, outcome: Option<WorldInteractionAuthorityStep>) {
    const WORLD3D_INTERACTION_STUCK_STRIDE: u64 = 4096;
    static ENTERED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
    static PENDING: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static CENSUS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    match outcome {
        None => {
            if ENTERED.swap(generation, std::sync::atomic::Ordering::Relaxed) == generation {
                return;
            }
            PENDING.store(0, std::sync::atomic::Ordering::Relaxed);
            log_debug_diagnostic(&format!("[TRACE] world3d interaction surface={surface_id} enter g={generation} {}", state.interaction_census()));
        }
        Some(WorldInteractionAuthorityStep::Pending) => {
            let pending = PENDING.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            let census = state.interaction_census();
            let digest = {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                census.hash(&mut hasher);
                hasher.finish()
            };
            if CENSUS.swap(digest, std::sync::atomic::Ordering::Relaxed) != digest || pending % WORLD3D_INTERACTION_STUCK_STRIDE == 0 {
                log_debug_diagnostic(&format!("[TRACE] world3d interaction surface={surface_id} step g={generation} pending={pending} {census}"));
            }
        }
        Some(step) => log_debug_diagnostic(&format!("[TRACE] world3d interaction surface={surface_id} leave g={generation} step={step:?} {}", state.interaction_census())),
    }
}

//#region 🖼️IconAtlasRuntime
thread_local! {
    pub static ICON_ATLAS_RUNTIME: RefCell<Option<IconAtlas>> = RefCell::new(None);
}
//#endregion 🖼️IconAtlasRuntime
