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

#[cfg(not(target_arch = "wasm32"))]
fn native_renderer_asset_path(url: &str) -> Result<std::path::PathBuf, String> {
    if url.starts_with("/mesh/") {
        let source = &mesh_assets::resolve_mesh_asset(url)?.source;
        return Ok(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../../../..").join(source));
    }
    if let Some(path) = url.strip_prefix("file://") {
        return Ok(std::path::PathBuf::from(path));
    }
    if url.starts_with('/') {
        return Ok(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").join(url.trim_start_matches('/')));
    }
    Ok(std::path::PathBuf::from(url))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn stream_native_renderer_asset(mailbox: &RuntimeMailbox, fetch: &mut RendererAssetFetchOwner) -> Result<(), NativeAssetStreamFailure> {
    if fetch.url().starts_with("http://") || fetch.url().starts_with("https://") {
        return stream_native_renderer_http_asset(mailbox, fetch).await;
    }
    let path = native_renderer_asset_path(fetch.url()).map_err(NativeAssetStreamFailure::Unavailable)?;
    let mut offset = 0u64;
    loop {
        if mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::Cancelled);
        }
        let value = run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage { path: path.clone(), offset, max_bytes: WORLD_ASSET_RESPONSE_PAGE_BYTES }).await.map_err(NativeAssetStreamFailure::Unavailable)?;
        #[cfg(test)]
        mailbox.wait_native_asset_page_test_barrier(fetch);
        let semio_framework_os_services::NativeIoValue::Page { mut bytes, eof } = value else { return Err("native renderer asset I/O returned the wrong value".into()) };
        if mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::CancelledPage { payload: bytes });
        }
        if bytes.is_empty() && !eof {
            return Err("native renderer asset produced an empty nonterminal page".into());
        }
        let count = bytes.len();
        if count != 0 {
            if let Err(detail) = push_renderer_asset_page(fetch, &mut bytes) {
                return Err(NativeAssetStreamFailure::Page { detail, payload: bytes });
            }
            offset = offset.checked_add(count as u64).ok_or_else(|| "native renderer asset offset overflow".to_string())?;
        }
        if eof {
            break;
        }
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn push_renderer_asset_page(fetch: &mut RendererAssetFetchOwner, bytes: &mut semio_framework_job::RetainedJobPayload) -> Result<(), &'static str> {
    let source = bytes.single_page().ok_or("native renderer asset I/O must return one retained page")?;
    if !source.is_empty() {
        let page = WorldAssetResponsePage::try_from_owned(source.to_vec()).map_err(|_| "native renderer asset page exceeded fixed credits")?;
        fetch.owner_mut().push_page(page).map_err(|_| "native renderer asset response exceeded its reserved credits")?;
    }
    let _ = bytes.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn stream_native_renderer_http_asset(mailbox: &RuntimeMailbox, fetch: &mut RendererAssetFetchOwner) -> Result<(), NativeAssetStreamFailure> {
    let cancel = mailbox.0.native_asset_http_cancel.child_now();
    let context = semio_framework_async::OperationContext {
        actor: 0,
        generation: 0,
        trace: semio_framework_async::TraceId(0),
        lane: semio_framework_async::Lane::Io as u8,
        deadline_ms: Some(semio_framework_job::default_now_ms().and_then(|now| now.checked_add(15_000)).ok_or_else(|| "native asset deadline requires a real clock and checked range".to_string())?),
        cancel: cancel.clone(),
        capability: None,
    };
    let request = semio_framework_os_services::HttpRequest { method: "GET".into(), url: fetch.url().to_owned(), headers: Vec::new(), body: Vec::new() };
    let pool = if fetch.url().starts_with("https://") { &mailbox.0.native_asset_https } else { &mailbox.0.native_asset_http };
    let mut transport = None;
    let response = pool
        .fetch_started(mailbox.0.native_asset_http_runtime.as_ref(), &mailbox.0.native_asset_http_scope, context, semio_framework_actor::PackageId("os.renderer.asset".into()), semio_framework_actor::ActorId(0), request, |cancellation| {
            transport = Some(mailbox.retain_native_asset_transport(fetch, cancel.clone(), cancellation))
        })
        .await
        .map_err(|error| if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) { NativeAssetStreamFailure::Cancelled } else { NativeAssetStreamFailure::Unavailable(error.to_string()) });
    let _transport = transport;
    let (head, mut body) = response?;
    if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) {
        return Err(NativeAssetStreamFailure::Cancelled);
    }
    if !(200..300).contains(&head.status) {
        return Err(NativeAssetStreamFailure::Unavailable(format!("native renderer asset HTTP status {}", head.status)));
    }
    if let Some(length) = head.headers.iter().find(|(name, _)| name.eq_ignore_ascii_case("content-length")).and_then(|(_, value)| value.parse::<usize>().ok()) {
        if length > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err("native renderer asset Content-Length exceeded fixed credits".into());
        }
    }
    loop {
        if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::Cancelled);
        }
        let page = body.next_chunk().await;
        if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::Cancelled);
        }
        let Some(bytes) = page.map_err(|error| NativeAssetStreamFailure::Unavailable(error.to_string()))? else { break };
        if bytes.is_empty() || bytes.len() > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err("native renderer HTTP asset returned an invalid response page".into());
        }
        let page = WorldAssetResponsePage::try_from_owned(bytes).map_err(|_| "native renderer HTTP asset page exceeded fixed credits")?;
        fetch.owner_mut().push_page(page).map_err(|_| "native renderer HTTP asset response exceeded its reserved credits")?;
        let mut yielded = false;
        std::future::poll_fn(|context| {
            if yielded {
                std::task::Poll::Ready(())
            } else {
                yielded = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        })
        .await;
    }
    Ok(())
}

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
        let cursor = self.native_hot_swap_cursor % self.shell.plugins.len().max(1);
        self.native_hot_swap_cursor = self.native_hot_swap_cursor.wrapping_add(1);
        let Some(program) = self.shell.plugins.get(cursor) else { return };
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
        let plugin_filter = self.shell.plugin_filter.clone();
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
    pub(crate) fn frame_before_input_step(&mut self, handle: &AppHandle, build_directives: &frame_job::FrameDirectives, cursor: &mut FrameBuildCursor, context: &mut semio_framework_job::StepContext<'_>) -> FrameBuildBoundaryStep {
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
                if !ui_wgpu::wgpu::PreparedGpuPresentCursor::close_abandoned_step() {
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
                cursor.fullscreen = std::mem::take(&mut self.shell.fullscreen_toggle_requested).then(|| {
                    self.shell.fullscreen_active = !self.shell.fullscreen_active;
                    self.shell.fullscreen_active
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
                self.theme = shell::resolve_theme_for_ids(self.shell.active_theme_id_for_frame(), &self.shell.appearance_id);
                cursor.phase = FrameBuildPhase::ThemeAppearance;
            }
            FrameBuildPhase::ThemeAppearance => {
                self.theme_dark = appearance_is_dark(&self.shell.appearance_id);
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
                    engine_canvas::node_graph_clear_wheel_zoom_active();
                }
                cursor.phase = FrameBuildPhase::RetainedClock;
            }
            FrameBuildPhase::RetainedClock => {
                if let Some((window_id, surface)) = self.shell.retained_clock_surface(cursor.retained_clock_index).map(|(id, surface)| (id.to_owned(), surface)) {
                    cursor.retained_clock_index += 1;
                    if let Some(step) = crate::interpreter::advance_retained_window_clock(&window_id, surface, &mut self.input) {
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
                self.shell.tutorial_tick(wall_now_ms);
                if let Some(runtime) = handle.upgrade().map(RuntimeMailbox) {
                    let deadline = semio_framework_job::default_now_us().and_then(|now| self.shell.next_chrome_deadline(now as f64 / 1_000_000.0, wall_now_ms));
                    runtime.publish_shell_clock_deadline(deadline);
                }
                cursor.phase = FrameBuildPhase::EnginePacketBacking;
            }
            FrameBuildPhase::EnginePacketBacking => {
                match cursor.engine_packets.prepare_step(context) { Ok(true) => cursor.phase = FrameBuildPhase::EngineResources, Ok(false) => return FrameBuildBoundaryStep::Pending, Err(failure) => return FrameBuildBoundaryStep::CloseFailure(failure) }
            }
            FrameBuildPhase::EngineResources => {
                let resources=cursor.engine_resources.get_or_insert_with(engine_canvas::EngineCanvasBuildContext::default);
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
                let complete=match interaction.shell.render_chrome_step(&mut cursor.chrome,draw,overlay,atlas,icons,&mut interaction.input,&interaction.theme,world_resources,context){Ok(complete)=>complete,Err(failure)=>return FrameBuildBoundaryStep::CloseFailure(failure)};
                if complete {
                    let input_candidate = match interaction.shell.seal_presented_input_candidate(&interaction.theme) {
                        Ok(witness) => witness,
                        Err(fault) => return FrameBuildBoundaryStep::Fault(fault),
                    };
                    cursor.input_candidate = Some(input_candidate);
                    interaction.shell.sync_engine_surface_states();
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
                if !engine_canvas::stage_engine_packet_step(resources) {
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
    pub(crate) fn frame_after_input_step(&mut self, runtime: &RuntimeMailbox, partial: &mut AppFrameAfterChrome, cursor: &mut FrameFinishCursor, context: &mut semio_framework_job::StepContext<'_>) -> FrameFinishBoundaryStep {
        let cancel = context.cancel_token();
        match cursor.phase {
            FrameFinishPhase::Inputs => {
                runtime.update_frame_inputs(self);
                cursor.phase = FrameFinishPhase::Deferred;
            }
            FrameFinishPhase::Deferred => {
                cursor.flush_tutorial = !self.shell.tutorial_pending_document_ops.is_empty();
                cursor.shell_maintenance = self.shell.chrome_maintenance_pending();
                cursor.settle = self.shell.settle_pump_pending();
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
                        tree_drag: self.shell.tree_drag.is_some(),
                        dock_drag: self.shell.dock_drag.is_some(),
                        pointer_drag_active: self.input.drag.active,
                        pointer_drag_axis: self.input.drag.axis,
                        pointer_drag_kind: self.input.drag.kind,
                    },
                );
                let connection_cursor = matches!(base_cursor, SemioCursor::Default)
                    && self
                        .shell
                        .scene_pointer_target_at(self.last_pointer_x, self.last_pointer_y, &self.input, &self.theme)
                        .filter(|target| target.kind == ui_wgpu::wgpu::SurfaceKind::NodeGraph)
                        .and_then(|target| {
                            self.shell.node_graph_states.get(&target.host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                                engine_canvas::node_graph_connection_cursor_active(
                                    &target.host_id,
                                    f64::from(self.last_pointer_x - surface.bounds.x),
                                    f64::from(self.last_pointer_y - surface.bounds.y),
                                )
                            })
                        })
                        .unwrap_or(false);
                let base_cursor = if connection_cursor { SemioCursor::CrosshairCentered } else { base_cursor };
                cursor.cursor = match self.shell.utility_cursor_override(self.last_pointer_x, self.last_pointer_y) {
                    Some(utility_cursor) if matches!(base_cursor, SemioCursor::Default | SemioCursor::Grab | SemioCursor::Selectable | SemioCursor::Pointer) => utility_cursor,
                    _ => base_cursor,
                };
                cursor.phase = FrameFinishPhase::Draw;
            }
            FrameFinishPhase::Draw => {
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame draw transfer lost resource input") };
                input.time_seconds = crate::deadlines::ui_animation_seconds(semio_framework_job::default_now_us());
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

impl AppInteractionState {
    pub(crate) fn start_text_operation(&mut self, stream: u64, declared_bytes: usize) -> Result<(), String> {
        if self.text_streams.iter().any(|entry| entry.is_some_and(|entry| entry.id == stream)) {
            return Err("text stream was already started".to_string());
        }
        let slot = self.text_streams.iter().position(Option::is_none).ok_or_else(|| "text stream slots exhausted".to_string())?;
        let generation = self.input.text_buffer.generation();
        let token = self.input.text_buffer.begin(generation, declared_bytes, self.input.cursor_pos, self.input.cursor_pos).map_err(|fault| format!("text admission failed: {fault:?}"))?;
        self.text_streams[slot] = Some(AppTextStream { id: stream, token });
        Ok(())
    }

    pub(crate) fn push_text_operation(&mut self, stream: u64, text: String) -> Result<(), String> {
        let token = self.text_streams.iter().flatten().find(|entry| entry.id == stream).map(|entry| entry.token).ok_or_else(|| "text stream chunk arrived before start".to_string())?;
        self.input.text_buffer.push(token, text).map_err(|fault| format!("text chunk admission failed: {fault:?}"))
    }

    pub(crate) fn commit_text_operation(&mut self, stream: u64) -> Result<(), String> {
        let slot = self.text_streams.iter().position(|entry| entry.is_some_and(|entry| entry.id == stream)).ok_or_else(|| "text stream commit arrived before start".to_string())?;
        let token = self.text_streams[slot].expect("text stream slot").token;
        self.input.text_buffer.commit(token).map_err(|fault| format!("text stream commit failed: {fault:?}"))?;
        self.text_streams[slot] = None;
        Ok(())
    }

    pub(crate) fn abort_text_operation(&mut self, stream: u64) -> Result<(), String> {
        let slot = self.text_streams.iter().position(|entry| entry.is_some_and(|entry| entry.id == stream)).ok_or_else(|| "text stream abort arrived before start".to_string())?;
        let token = self.text_streams[slot].take().expect("text stream slot").token;
        self.input.text_buffer.abort(token).map_err(|fault| format!("text stream abort failed: {fault:?}"))
    }

    pub(crate) fn enqueue_text_operation(&mut self, text: String) -> Result<(), String> {
        let generation = self.input.text_buffer.generation();
        self.input.text_buffer.enqueue_owned(generation, text, self.input.cursor_pos, self.input.cursor_pos).map_err(|fault| format!("text admission failed: {fault:?}"))
    }

    pub(crate) fn cancel_text_operations(&mut self) {
        self.text_cancel_pending = true;
    }

    /// ⌨️ `mod+z` belongs to a FOCUSED text control, and to nothing else.
    ///
    /// 🩸️ It used to belong to the text buffer whenever that buffer held any undo state at all — and
    /// the buffer keeps its state after a control loses focus, so once the journey had opened the pane's
    /// Search box, every later `mod+z` was swallowed here and the shell's own undo chord (which
    /// dispatches `undo` to the app, exactly as React does) never ran (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` steps 27–28).
    pub(crate) fn undo_text_operation(&mut self) -> bool {
        if self.input.focused_id.is_none() {
            return false;
        }
        let Some(cursor) = self.input.text_buffer.undo() else { return false };
        self.input.cursor_pos = cursor.min(self.input.text_buffer.len());
        true
    }

    pub(crate) fn has_pending_text_work(&self) -> bool {
        self.input.text_buffer.runnable_work_pending() || self.input.retained_action_pending() || self.text_cancel_pending || engine_canvas::has_pending_text_editor_outbox()
    }

    pub(crate) fn drive_text_operation(&mut self) {
        if self.text_cancel_pending {
            let generation = self.input.text_buffer.generation();
            let _ = self.input.text_buffer.step(generation, 1, true);
            self.text_cancel_pending = self.input.text_buffer.reserved_bytes() != 0;
            return;
        }
        if let Err(fault) = self.input.drive_text_step() {
            self.text_fault = Some(format!("text edit step failed: {fault:?}"));
        }
        interpreter::sync_focused_table_editable_text_draft(&self.input);
    }

    /// 📐️ The content root is LOGICAL (CSS) pixels, exactly like every chrome constant
    /// (`chrome_px`, navbar/tab/footer heights, resize-handle hit widths) and like the React host's
    /// DOM. `dpr` never reaches layout — it only sizes the GPU surface, the projection divisor and
    /// the glyph atlas raster. See ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
    pub(crate) fn resize(&mut self, css_width: f32, css_height: f32, _dpr: f32) {
        self.shell.screen_w = css_width.max(1.0);
        self.shell.screen_h = css_height.max(1.0);
    }

    /// ⌨️ Offers one key to every board pane the pointer is inside, newest-first, and answers `true`
    /// as soon as one consumes it. Scoped to the pointer like React's `hoverActiveRef` gate, so a
    /// board in another pane never eats a chord meant for the focused window.
    fn board2d_key(&mut self, action: &KeyAction, modifiers: &PointerModifiers) -> bool {
        if !matches!(action, KeyAction::Escape | KeyAction::Tab) {
            return false;
        }
        let surfaces: Vec<(String, String)> = self.shell.board2d_states.keys().filter_map(|surface_id| self.shell.board2d_states.get(surface_id).map(|surface| (surface_id.to_string(), surface.controller_id.clone()))).collect();
        for (surface_id, controller_id) in surfaces {
            match engine_canvas::puzzle_board_key_into(&surface_id, &controller_id, action, modifiers, &mut self.input) {
                Ok(true) => return true,
                Ok(false) => {}
                Err(fault) => {
                    self.input.record_action_fault(fault);
                    return true;
                }
            }
        }
        false
    }

    /// ✍️ A focused text editor owns the keyboard before the shell's chord table does — the same
    /// precedence React gets from DOM focus on its `<textarea>`. Without this the editor's
    /// `text_editor_apply_key_into` had no production caller at all and typing into a TextEditor
    /// surface only ever fired global accelerators.
    ///
    /// ⎋️🚚️ A live World3d relocate drag owns `Escape` before ANY chord table does — React binds it
    /// as a CAPTURE-phase `keydown` while `relocateMode` is on and calls `event.stopPropagation()`
    /// only when a drag was actually cancelled (`🌐️World3dHost/🟦️.tsx:6873-6881`). The drag is
    /// dropped, its ghost cleared, and nothing is dispatched.
    ///
    /// ⌨️ A board pane under the pointer owns `Escape` (cancel area-select) and `Tab`/`shift+Tab`
    /// (walk the brush slot's candidates) before the shell's chord table does — React gets the same
    /// precedence from a CAPTURE-phase listener that calls `preventDefault` (`🖥️Board2dHost/🟦️.tsx`
    /// :1292-1313, :1338-1339). Both chords had NO wgpu path at all before this.
    ///
    /// 🔌️ w2-input-wiring: spawns the ASYNC `handle_keyboard_async` (mirrors this fn's own
    /// `on_button`/`on_move` sibling callbacks above, and the `spawn_app_task` pattern this fn
    /// used to hand-roll just for search/find-Enter-activation) instead of calling the sync
    /// `handle_keyboard` directly. Before this fix `handle_keyboard_async` was entirely dead code
    /// (see `report-w3-shell-input-cutover.md`'s "MAJOR FINDING"): the P4 app-keybinding dispatch,
    /// P5 idle-Escape-deactivates-utility, and — worst — committing a focused `Input`'s typed text
    /// via Enter/Escape never fired. `handle_keyboard_async`'s own top already reimplements the
    /// exact search/find-Enter-activation this fn used to hand-duplicate around the sync call, so
    /// that duplication is gone, not just moved.
    pub(crate) async fn handle_key(&mut self, action: KeyAction, modifiers: PointerModifiers) {
        if interpreter::apply_focused_vfs_control_key(&action, &mut self.input) {
            return;
        }
        if interpreter::apply_focused_table_editable_text_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if interpreter::apply_focused_table_stepper_key(&action, &mut self.input) {
            return;
        }
        if interpreter::apply_focused_ink_editor_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if interpreter::apply_focused_ink_surface_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if let KeyAction::Space(pressed) = &action {
            if *pressed && (self.shell.context_menu.is_some() || self.shell.retained_select_owns_keyboard()) {
                if let Err(err) = self.shell.handle_keyboard_async(KeyAction::Space(true), &modifiers, &mut self.input).await {
                    log_debug(&format!("keyboard failed: {err}"));
                }
                return;
            }
            if interpreter::apply_focused_text_editor_key(&action, &modifiers, &mut self.input) {
                return;
            }
            self.space_pressed = *pressed;
            return;
        }
        if interpreter::apply_focused_node_graph_note_key(&action, &modifiers) {
            return;
        }
        if interpreter::apply_focused_text_editor_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if action == KeyAction::Escape && self.shell.world3d_states.values_mut().fold(false, |cancelled, state| infinite_world::world::world3d_cancel_relocate_drag(state) || cancelled) {
            return;
        }
        if self.board2d_key(&action, &modifiers) {
            return;
        }
        if let Err(err) = self.shell.handle_keyboard_async(action, &modifiers, &mut self.input).await {
            log_debug(&format!("keyboard failed: {err}"));
        }
    }

    /// 🛑️ Clears pointer ownership without manufacturing a successful release.
    pub(crate) fn handle_pointer_cancel(&mut self, pointer: ui_render::PointerInfo) {
        if let Some([x, y]) = self.pointer_capture.position(pointer.id) {
            self.input.pointer_x = x;
            self.input.pointer_y = y;
        }
        if pointer.kind == ui_render::PointerKind::Touch {
            if let Some(target) = interpreter::captured_scene_pointer(pointer.id) {
                let outcome = match target.kind {
                    ui_wgpu::wgpu::SurfaceKind::TiledMap => self
                        .shell
                        .tiled_map_states
                        .get(&target.host_id)
                        .filter(|surface| surface.window_id == target.window_id)
                        .map(|surface| scenes::tiled_map_touch_pointer_up_into(&target.host_id, &surface.controller_id, pointer.id, &mut self.input)),
                    ui_wgpu::wgpu::SurfaceKind::Board2d => self
                        .shell
                        .board2d_states
                        .get(&target.host_id)
                        .filter(|surface| surface.window_id == target.window_id)
                        .map(|surface| scenes::puzzle_board_touch_pointer_up_into(&target.host_id, &surface.controller_id, pointer.id, &mut self.input)),
                    _ => None,
                };
                if let Some(Err(fault)) = outcome {
                    self.input.record_action_fault(fault);
                }
            }
        }
        self.pointer_capture.release(pointer.id);
        self.pointer_down = self.pointer_capture.any_active();
        self.shell.handle_pointer_cancel_for(pointer.id, &mut self.input);
    }

    /// 🎡️ Applies one owned normalized wheel event before the next dispatch can change its target.
    pub(crate) fn handle_pointer_wheel(&mut self, x: f32, y: f32, delta_x: f32, delta_y: f32, modifiers: PointerModifiers) {
        self.modifiers = modifiers.clone();
        self.input.modifiers = modifiers.clone();
        let target = self.shell.scene_pointer_target_at(x, y, &self.input, &self.theme);
        let fault = self.shell.world3d_states.take_fault().or_else(|| self.shell.node_graph_states.take_fault()).or_else(|| self.shell.tiled_map_states.take_fault()).or_else(|| self.shell.board2d_states.take_fault());
        if let Some(fault) = fault {
            self.frame_fault = Some(fault.to_owned());
            return;
        }
        if let Some(target) = target {
            if let Err(fault) = self.apply_scene_wheel(&target, x, y, delta_y, &modifiers) {
                self.frame_fault = Some(fault.to_owned());
            }
        } else {
            self.shell.handle_pointer_wheel(x, y, delta_x, delta_y, &mut self.input);
        }
    }

    /// 🎡️ Applies one notch to the captured published scene, never to overlapping peers.
    pub(crate) fn apply_scene_wheel(&mut self, target: &crate::element_contracts::ScenePointerTarget, x: f32, y: f32, delta: f32, modifiers: &PointerModifiers) -> Result<(), &'static str> {
        use ui_wgpu::wgpu::SurfaceKind;
        if !self.shell.scene_pointer_target_is_published(target) {
            return Ok(());
        }
        let surface_id = &target.surface_id;
        let host_id = &target.host_id;
        if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
            return Err("scene wheel identifier exceeded fixed credits");
        }
        let outcome = match target.kind {
            SurfaceKind::World3d => self.shell.world3d_states.get_mut(host_id).map(|state| {
                if state.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                enqueue_world3d_event(state, WorldInteractionIntent::wheel(x, y, delta, modifiers)).map(|_| ()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits)
            }),
            SurfaceKind::NodeGraph => self.shell.node_graph_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                let outcome = engine_canvas::node_graph_wheel_into(host_id, &surface.controller_id, surface.bounds, x, y, delta, modifiers.ctrl, &mut self.input).map(|_| ());
                if outcome.is_ok() {
                    self.wheel_zoom_deadline_ms = app_now_ms() + 120.0;
                }
                outcome
            }),
            SurfaceKind::TiledMap => self.shell.tiled_map_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                engine_canvas::tiled_map_wheel_into(host_id, &surface.controller_id, surface.bounds, x, y, delta, modifiers.ctrl, &mut self.input).map(|_| ())
            }),
            SurfaceKind::Board2d => self.shell.board2d_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                scenes::puzzle_board_wheel_into(host_id, &surface.controller_id, surface.bounds, x, y, delta, &mut self.input).map(|_| ())
            }),
            _ => None,
        };
        if let Some(Err(fault)) = outcome {
            self.input.record_action_fault(fault);
            return Err("scene wheel action admission failed");
        }
        Ok(())
    }

    pub(crate) async fn handle_pointer_button(&mut self, pointer: ui_render::PointerInfo, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        use ui_wgpu::wgpu::SurfaceKind;
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.input.modifiers = modifiers.clone();
        self.input.pointer_x = x;
        self.input.pointer_y = y;
        self.input.pointer_down = down;
        if down && button == 0 {
            interpreter::blur_focused_ink_editor_at(x, y, &mut self.input);
        }
        let target = if down { self.shell.scene_pointer_target_at(x, y, &self.input, &self.theme) } else { None };
        interpreter::blur_focused_table_editable_text_for_pointer(target.as_ref(), down, button, &mut self.input);
        interpreter::blur_focused_text_editor_for_pointer(target.as_ref(), down, button);
        interpreter::blur_focused_node_graph_caret_for_pointer(target.as_ref(), down, button);
        interpreter::blur_focused_table_stepper_for_pointer(down, button);
        interpreter::blur_focused_vfs_control_for_pointer(down, button);
        let owner = if down {
            let Some(owner) = self.pointer_capture.press(pointer.id, if target.is_some() { PointerHitOwner::Surface } else { PointerHitOwner::Chrome }, x, y) else {
                self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                return;
            };
            owner
        } else {
            self.pointer_capture.release(pointer.id)
        };
        self.pointer_down = self.pointer_capture.any_active();
        if down {
            self.shell.activate_window_under_pointer(x, y, &self.theme);
        }
        if owner == PointerHitOwner::Chrome {
            if let Err(err) = self.shell.handle_pointer_button_for(pointer.id, x, y, down, button, &mut self.input, &self.theme).await {
                log_debug(&format!("pointer failed: {err}"));
            }
            return;
        }
        let Some(target) = (if down { target } else { interpreter::release_scene_pointer(pointer.id) }).filter(|target| self.shell.scene_pointer_target_is_published(target)) else { return };
        if down && !interpreter::claim_scene_pointer_owner(target.clone(), pointer.id) {
            self.pointer_capture.release(pointer.id);
            self.pointer_down = self.pointer_capture.any_active();
            self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
            return;
        }
        if down && target.kind == SurfaceKind::World3d && button == 2 && !modifiers.shift && !modifiers.alt && !modifiers.meta {
            if let Err(err) = self.shell.handle_pointer_button_for(pointer.id, x, y, down, button, &mut self.input, &self.theme).await {
                log_debug(&format!("pointer failed: {err}"));
            }
        }
        let host_id = &target.host_id;
        let outcome = match target.kind {
            SurfaceKind::World3d => {
                self.shell.world3d_states.get_mut(host_id).map(|state| enqueue_world3d_event(state, WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers)).map(|_| ()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits))
            }
            SurfaceKind::NodeGraph => {
                let outcome = self.shell.node_graph_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                    if down {
                        engine_canvas::node_graph_pointer_down_into(host_id, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, self.space_pressed, &mut self.input).map(|_| ())
                    } else {
                        engine_canvas::node_graph_pointer_up_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())
                    }
                });
                interpreter::refresh_node_graph_caret_after_pointer(&target);
                outcome
            }
            SurfaceKind::TiledMap => self.shell.tiled_map_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                let touch_handled = pointer.kind == ui_render::PointerKind::Touch
                    && if down {
                        scenes::tiled_map_touch_pointer_down(host_id, surface.bounds, pointer.id, x, y)
                    } else {
                        match scenes::tiled_map_touch_pointer_up_into(host_id, &surface.controller_id, pointer.id, &mut self.input) {
                            Ok(handled) => handled,
                            Err(fault) => return Err(fault),
                        }
                    };
                if touch_handled {
                    Ok(())
                } else if down {
                    scenes::tiled_map_pointer_down_into(&target, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), &surface.selection_method, &mut self.input).map(|_| ())
                } else {
                    scenes::tiled_map_pointer_up_into(host_id, &surface.surface_id, &surface.controller_id, surface.bounds, x, y, &mut self.input).map(|_| ())
                }
            }),
            SurfaceKind::Board2d => self.shell.board2d_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                let touch_handled = pointer.kind == ui_render::PointerKind::Touch
                    && if down {
                        match scenes::puzzle_board_touch_pointer_down_into(host_id, surface.bounds, pointer.id, x, y, &mut self.input) {
                            Ok(handled) => handled,
                            Err(fault) => return Err(fault),
                        }
                    } else {
                        match scenes::puzzle_board_touch_pointer_up_into(host_id, &surface.controller_id, pointer.id, &mut self.input) {
                            Ok(handled) => handled,
                            Err(fault) => return Err(fault),
                        }
                    };
                if touch_handled {
                    Ok(())
                } else if down {
                    scenes::puzzle_board_pointer_down_into(host_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), &mut self.input)
                } else {
                    scenes::puzzle_board_pointer_up_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())
                }
            }),
            _ => None,
        };
        if let Some(Err(fault)) = outcome {
            self.input.record_action_fault(fault);
        }
    }

    pub(crate) async fn handle_pointer_move(&mut self, pointer: ui_render::PointerInfo, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        use ui_wgpu::wgpu::SurfaceKind;
        let (drag_dx, drag_dy) = self.pointer_capture.advance(pointer.id, x, y).unwrap_or((0.0, 0.0));
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = self.pointer_capture.any_active();
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.input.modifiers = modifiers.clone();
        self.input.pointer_x = x;
        self.input.pointer_y = y;
        self.input.pointer_down = down;
        self.shell.presence_pointer = Some((x, y));
        let target = match self.pointer_capture.holder(pointer.id) {
            Some(PointerHitOwner::Surface) => interpreter::captured_scene_pointer(pointer.id),
            Some(PointerHitOwner::Chrome) => None,
            None => self.shell.scene_pointer_target_at(x, y, &self.input, &self.theme),
        }
        .filter(|target| self.shell.scene_pointer_target_is_published(target));
        if target.is_none() && self.pointer_capture.holder(pointer.id) != Some(PointerHitOwner::Surface) {
            self.shell.handle_pointer_move_for(pointer.id, x, y, down, &mut self.input, &self.theme);
        }
        let world_id = target.as_ref().filter(|target| target.kind == SurfaceKind::World3d).map(|target| target.host_id.as_str());
        for (host_id, state) in self.shell.world3d_states.iter_mut() {
            let intent = if Some(host_id.as_str()) == world_id {
                WorldInteractionIntent::pointer_move(x, y, drag_dx, drag_dy, down, button, &modifiers)
            } else if world3d_hover_is_published(state) {
                WorldInteractionIntent::pointer_leave(x, y)
            } else {
                continue;
            };
            if enqueue_world3d_event(state, intent).is_err() {
                self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                return;
            }
        }
        for (host_id, surface) in &self.shell.board2d_states {
            if target.as_ref().is_some_and(|target| target.kind == SurfaceKind::Board2d && target.host_id == *host_id) {
                continue;
            }
            if let Err(fault) = scenes::puzzle_board_pointer_leave_into(host_id, &surface.controller_id, modifiers.alt, &mut self.input) {
                self.input.record_action_fault(fault);
                return;
            }
        }
        for (host_id, surface) in &self.shell.tiled_map_states {
            if target.as_ref().is_some_and(|target| target.kind == SurfaceKind::TiledMap && target.host_id == *host_id && target.window_id == surface.window_id) {
                continue;
            }
            if let Err(fault) = scenes::tiled_map_pointer_leave_into(host_id, &surface.surface_id, &surface.controller_id, &mut self.input) {
                self.input.record_action_fault(fault);
                return;
            }
        }
        let Some(target) = target else { return };
        let host_id = &target.host_id;
        let outcome = match target.kind {
            SurfaceKind::NodeGraph => self
                .shell
                .node_graph_states
                .get(host_id)
                .filter(|surface| surface.window_id == target.window_id)
                .map(|surface| engine_canvas::node_graph_pointer_move_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())),
            SurfaceKind::TiledMap => self
                .shell
                .tiled_map_states
                .get(host_id)
                .filter(|surface| surface.window_id == target.window_id)
                .map(|surface| {
                    if pointer.kind == ui_render::PointerKind::Touch && scenes::tiled_map_touch_pointer_move(host_id, surface.bounds, pointer.id, x, y) {
                        Ok(())
                    } else {
                        scenes::tiled_map_pointer_move_into(&target, &surface.controller_id, surface.bounds, x, y, down, &mut self.input).map(|_| ())
                    }
                }),
            SurfaceKind::Board2d => self
                .shell
                .board2d_states
                .get(host_id)
                .filter(|surface| surface.window_id == target.window_id)
                .map(|surface| {
                    if pointer.kind == ui_render::PointerKind::Touch && scenes::puzzle_board_touch_pointer_move(host_id, surface.bounds, pointer.id, x, y) {
                        Ok(())
                    } else {
                        scenes::puzzle_board_pointer_move_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())
                    }
                }),
            _ => None,
        };
        if let Some(Err(fault)) = outcome {
            self.input.record_action_fault(fault);
        }
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
pub(crate) fn step_world3d_camera_fit_after_snapshot(state: &mut infinite_world::world::World3dState) {
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
pub(crate) fn world3d_ingest_trace(surface_id: &str, state: &infinite_world::world::World3dState, stage: &str) {
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
pub(crate) fn world3d_interaction_trace(surface_id: &str, state: &infinite_world::world::World3dState, generation: u64, outcome: Option<WorldInteractionAuthorityStep>) {
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

/// 🧹️ P3c: this used to build a `PointerCallbacks` here (5 `Rc<RefCell<AppRuntime>>` clones, one
/// per input kind) and hand it back alongside `runtime`. `winit_app.rs`'s own `HostUserEvent` doc
/// comment records that its one caller stopped using it at the P3a enqueue-only
/// `WindowDelegate`/`dispatch_normalized_event` cutover -- `boot_runtime` was left constructing it
/// anyway because touching this signature wasn't that packet's job. It is
/// this packet's job (removing `self_weak`, see this crate's own `AppHandle` doc comment), and per
/// AGENTS.md's no-legacy-code rule, dead construction is deleted outright. Right-click remains a
/// lossless `DispatchEvent::PointerDown { button: Secondary }` in the enqueue-only contract;
/// `winit_app::dispatch_normalized_event` maps it to button `2` and calls the canonical
/// `handle_pointer_button`, whose Shell path opens the context menu. The redundant callbacks-only
/// `handle_context_menu` wrapper is deleted with its sole caller. See `📓️p3c-explicit-app-handle.md`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn boot_runtime(
    window: Arc<Window>,
    gpu: impl std::future::Future<Output = Result<GpuContext, String>> + Send + 'static,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")] plugins: Option<JsValue>,
    #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
    services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>,
) -> Result<(RuntimeMailbox, AppPresenter), String> {
    let dpr = window.scale_factor() as f32;
    let size = window.inner_size();
    #[cfg(target_arch = "wasm32")]
    let (css_width, css_height, dpr) = {
        use winit::platform::web::WindowExtWebSys;
        let dpr = web_sys::window().map(|host| host.device_pixel_ratio() as f32).unwrap_or(dpr);
        if let Some(canvas) = window.canvas() {
            let css_width = canvas.client_width().max(1) as f32;
            let css_height = canvas.client_height().max(1) as f32;
            canvas.set_width((css_width * dpr) as u32);
            canvas.set_height((css_height * dpr) as u32);
            (css_width, css_height, dpr)
        } else {
            (size.width as f32 / dpr, size.height as f32 / dpr, dpr)
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let css_width = size.width as f32 / dpr;
    #[cfg(not(target_arch = "wasm32"))]
    let css_height = size.height as f32 / dpr;

    const ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf");
    let font_bytes = match fetch_font_bytes("/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf").await {
        Ok(bytes) if bytes.len() > 256 => bytes,
        _ => ANTA_LATIN.to_vec(),
    };
    let mut atlas = FontAtlas::from_bytes(&font_bytes).map_err(|err| format!("atlas failed: {err}"))?;
    atlas.set_raster_scale(dpr);
    let icons = icon_atlas::build_icon_atlas_scaled(dpr);
    let mut gpu = gpu.await.map_err(|err| format!("gpu init failed: {err}"))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);
    gpu.upload_icon_atlas(&icons);

    #[cfg(target_arch = "wasm32")]
    let entries = {
        let plugins = plugins.ok_or("missing wasm programs")?;
        filter_plugins(parse_plugin_entries(plugins).map_err(|err| format!("program parse failed: {err}"))?, &plugin_filter)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let entries = filter_plugins(load_wasm_plugins(&plugin_filter, &plugin_modules_root).await?, &plugin_filter);

    let (locale, terminology) = shell::shell_language_axes()?;
    let mut shell = ShellState::new(entries, plugin_filter.clone(), locale, terminology);
    shell.install_document_services(services);
    shell.screen_w = css_width;
    shell.screen_h = css_height;
    shell.boot().await.map_err(|err| format!("shell boot failed: {err}"))?;

    let runtime = RuntimeMailbox::new(AppRuntime {
        atlas,
        icons,
        icon_rebuild: None,
        icon_raster_scale: dpr,
        interaction: Some(AppInteractionState {
            shell,
            input: InputState::default(),
            theme: Theme::default(),
            theme_dark: appearance_is_dark("system"),
            last_pointer_x: 0.0,
            last_pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            pointer_capture: PointerCapture::default(),
            modifiers: PointerModifiers::default(),
            space_pressed: false,
            wheel_zoom_deadline_ms: 0.0,
            text_streams: std::array::from_fn(|_| None),
            text_fault: None,
            frame_fault: None,
            text_cancel_pending: false,
            last_sync_pump_ms: 0.0,
        }),
        checkout: runtime_mailbox_core::InteractionCheckoutLedger::default(),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: FrameActionOwners::default(),
        #[cfg(not(target_arch = "wasm32"))]
        pending_frame_maintenance_refusal: None,
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: plugin_modules_root.clone(),
        #[cfg(not(target_arch = "wasm32"))]
        native_plugin_mtimes: std::collections::HashMap::new(),
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_scan: None,
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_modified: None,
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_cursor: 0,
        #[cfg(not(target_arch = "wasm32"))]
        native_reload_pending: false,
    });
    let presenter = AppPresenter {
        gpu,
        engine: engine_canvas::EngineCanvasPresenter::default(),
        gate: ui_wgpu::wgpu::PreparedRenderGate::default(),
        runtime: runtime.clone(),
        presentation_authority: runtime.presentation_authority(),
        raster_operation_authority: runtime.raster_operation_authority(),
        window: Some(window.clone()),
        #[cfg(target_arch = "wasm32")]
        offscreen_token: None,
        last_cursor: None,
        pending: None,
        retirement: None,
        retained_fault: None,
        surface_resize: None,
        stall: AppPresentStallWatch::default(),
    };

    log_debug("wgpu renderer booted");
    Ok((runtime, presenter))
}
