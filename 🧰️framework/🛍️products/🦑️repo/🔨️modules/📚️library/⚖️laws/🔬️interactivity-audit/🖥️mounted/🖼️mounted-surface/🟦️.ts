/** 🖼️ Interactivity audit: mounted engine surface lifetime, prepared render and surface lane sources.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { interactivityProductionSource } from "../../🗄️database/🟦️.ts";

//#region 🖥️P3mnMountedEngineSurfaceLifetime
export function interactivityMountedEngineSurfaceLifetimeFailures(
  engineSource: string,
  hostSource: string,
  winitSource: string,
  browserSource: string,
  glueSource: string,
  graphSource: string,
  flowSource: string,
  mapSource: string,
  editorSource: string,
  dagSource: string,
): string[] {
  const engine = interactivityProductionSource(engineSource);
  const host = interactivityProductionSource(hostSource);
  const winit = interactivityProductionSource(winitSource);
  const browser = interactivityProductionSource(browserSource);
  const glue = interactivityProductionSource(glueSource);
  const graph = interactivityProductionSource(graphSource);
  const flow = interactivityProductionSource(flowSource);
  const map = interactivityProductionSource(mapSource);
  const editor = interactivityProductionSource(editorSource);
  const dag = interactivityProductionSource(dagSource);
  const failures: string[] = [];
  const requireAll = (source: string, needles: readonly string[], failure: string): void => {
    if (needles.some((needle) => !source.includes(needle))) failures.push(failure);
  };
  const registry = engine.slice(engine.indexOf("pub(crate) const ENGINE_SURFACE_CAPACITY"), engine.indexOf("//#region 📦️PreparedEngineCanvas"));
  const packetAuthority = engine.slice(engine.indexOf("const ENGINE_CANVAS_FRAME_PACKET_CAPACITY"), engine.indexOf("enum EngineGpuBuildPhase"));
  const presenter = engine.slice(engine.indexOf("enum EngineGpuBuildPhase"), engine.indexOf("//#endregion 📦️PreparedEngineCanvas"));
  const realize = presenter.slice(presenter.indexOf("pub(crate) fn realize_step"), presenter.indexOf("pub(crate) fn close_active_candidate_step"));
  const cpuClose = engine.slice(engine.indexOf("struct WorkerCell"), engine.indexOf("fn create_target_texture"));
  const mapRetirement = map.slice(map.indexOf("pub struct MapHostRetirement"), map.indexOf("enum MapInteraction"));
  const editorRetirement = editor.slice(editor.indexOf("pub struct EditorHostRetirement"), editor.indexOf("impl Default for EditorHost"));
  const dagRetirement = dag.slice(dag.indexOf("pub struct DagHostRetirement"), dag.indexOf("enum DagNodeEvalStatusKind"));
  const paired = host.slice(host.indexOf("enum PairedEngineSurfaceClosePhase"), host.indexOf("impl OsHost {"));
  const abandonment = host.slice(host.indexOf("const OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY"), host.indexOf("enum PairedEngineSurfaceClosePhase"));
  requireAll(registry, [
    "ENGINE_SURFACE_CAPACITY: usize = 256",
    "ENGINE_SURFACE_ID_BYTE_CAPACITY: usize = 256",
    "bytes: [u8; ENGINE_SURFACE_ID_BYTE_CAPACITY]",
    "generation.checked_add(1)",
    "exhausted: bool",
    "NodeGraphEngineRetirement::Dag",
    "NodeGraphEngineRetirement::Flow",
    "MapHostRetirement::new(owner)",
    "EditorHostRetirement::new(owner)",
    "EngineSurfaceClosePhase::NodeGraphSync",
    "EngineSurfaceClosePhase::MapSync",
    "EngineSurfaceClosePhase::EditorPack",
    "EngineSurfaceClosePhase::TileRequests",
    "close_string",
    "close_bytes",
  ], "P3mn CPU registry lacks fixed checked identity or exhaustive retained child/cache retirement");
  if (registry.includes("wrapping_add") || registry.includes("HashMap<String, EngineSurface") || registry.includes("drop(pack)") || registry.includes("drop(controller)")) failures.push("P3mn CPU registry retains wrapping, dynamic, or whole-owner close work");
  requireAll(registry, [
    "let EngineSurface {",
    "board_pending_events.terminal_is_empty()",
    "node_graph_sync_terminal(&self.sync_cache)",
    "map_sync_terminal(&self.map_sync_cache)",
    "board_sync_terminal(&self.board_sync_cache)",
    "self.phase == EngineSurfaceClosePhase::Released\n            && self.node_graph_source.is_none()",
  ], "P3mn outer EngineSurface retirement lacks an exhaustive field-wise terminal witness");
  if (registry.includes("ManuallyDrop::drop(&mut self.surface)")) failures.push("P3mn outer EngineSurface retirement still permits an ordinary whole-owner drop");
  requireAll(packetAuthority, [
    "rejected: Box<[Option<EngineCanvasPacket>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>",
    "EngineCanvasPacketDestination::Rejected(index)",
    "fn try_reserve_fresh_packet",
    "} else if self.rejected_len < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {",
    "reservation_sequence == self.published_reservation_sequence",
    "return Err(surface)",
  ], "P3mn packet overflow lacks a fixed generation-qualified multi-slot rejection authority");
  if (packetAuthority.includes("rejected: Option<EngineCanvasPacket>")) failures.push("P3mn packet overflow regressed to a single rejected owner slot");
  requireAll(cpuClose, [
    "fn try_borrow_mut(&self) -> Option<MutexGuard<'_, T>>",
    "self.state().try_lock().ok()",
    "pub(crate) fn engine_surface_token_at(index: usize) -> Result<Option<EngineSurfaceToken>, ()>",
    "cell.try_borrow_mut().ok_or(())?",
    "pub(crate) fn begin_engine_surface_close_token(token: EngineSurfaceToken) -> Result<bool, ()>",
    "pub(crate) fn engine_surface_terminal_nonopaque_is_empty(token: EngineSurfaceToken) -> Result<bool, ()>",
  ], "P3mn mounted CPU close probe can block or erase registry contention");
  requireAll(presenter, [
    "slots: ManuallyDrop<Option<Box<[EngineGpuSlot; ENGINE_SURFACE_CAPACITY]>>>",
    "EngineGpuBuildPhase::Reserve",
    "EngineGpuBuildPhase::Texture",
    "EngineGpuBuildPhase::View",
    "EngineGpuBuildPhase::Renderer",
    "EngineGpuBuildPhase::Render",
    "EngineGpuBuildPhase::ReplacementTexture",
    "EngineGpuBuildPhase::ReplacementView",
    "EngineGpuBuildPhase::Stage",
    "EngineGpuBuildPhase::Publish",
    "fn publish_candidate",
    "return slot.publish_candidate(packet, expected, primary_metrics_generation);",
    "candidate.matches(packet, expected, primary_metrics_generation)",
    "self.live.replace(published)",
    "self.retirement = Some(EngineGpuRetirement::new(displaced))",
    "if let Some(retirement) = slot.retirement.as_mut()",
    "retirement.close_step() && retirement.terminal_is_empty()",
    "candidate.close_step(gpu)",
    "retirement.close_step()",
    "engine_gpu_freshness_matches(self.surface, self.document_generation, self.scene_revision, self.metrics_generation, live)",
  ], "P3mn GPU presenter lacks fixed staged realization, atomic publish, or retained candidate/live close");
  requireAll(realize, [
    "if let Some(retirement) = slot.retirement.as_mut()",
    "retirement.close_step() && retirement.terminal_is_empty()",
    "engine_surface_live_freshness(packet.surface.token)",
    "if !build.matches_live(live)",
  ], "P3mn ordinary replacement or live CPU-slot publication freshness is not mounted in the GPU grant");
  requireAll(cpuClose, [
    "fn engine_surface_live_freshness(token: EngineSurfaceToken)",
    "metrics_generation: value.metrics_generation",
    "document_generation: value.document_generation",
    "scene_revision: value.scene_revision",
  ], "P3mn GPU publication lacks a nonblocking live CPU-slot freshness authority");
  for (const forbidden of ["HashMap<String, EngineGpuSurface>", "fn realize_one", "std::mem::forget(slots)"])
    if (presenter.includes(forbidden)) failures.push(`P3mn GPU presenter retains forbidden whole/dynamic ownership ${forbidden}`);
  requireAll(paired, [
    "PairedEngineSurfaceClosePhase::Scan",
    "PairedEngineSurfaceClosePhase::BeginCpu",
    "PairedEngineSurfaceClosePhase::BeginGpu",
    "PairedEngineSurfaceClosePhase::Cpu",
    "PairedEngineSurfaceClosePhase::Gpu",
    "PairedEngineSurfaceClosePhase::Witness",
    "runtime.begin_engine_surface_close(token)",
    "let Ok(cpu) = crate::engine_canvas::engine_surface_token_at(self.scan) else",
    "Err(()) => return false",
    "presenter.begin_engine_surface_close(token)",
    "runtime.close_engine_surface_step(token, self.operation, &mut self.sequence)",
    "presenter.close_engine_surface_step(token)",
  ], "P3mn mounted close does not retain an exact paired CPU/GPU token cursor");
  requireAll(abandonment, [
    "OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY: usize = 64",
    "AtomicPtr<OsHostRetirementState>",
    "generation: AtomicU64,\n    exhausted: AtomicBool",
    "exhausted: AtomicBool",
    "current.checked_add(1)",
    "publish_os_host_retirement_abandonment",
  ], "P3mn interrupted host close lacks fixed nonblocking generation-qualified handback");
  requireAll(host, [
    "pub(crate) fn try_into_retirement(self) -> Result<OsHostRetirement, Self>",
    "pub(crate) fn close_abandoned_step() -> bool",
    "impl Drop for OsHostRetirement",
    "publish_os_host_retirement_abandonment(token, state)",
    "engine_surfaces.close_step(runtime, presenter)",
  ], "P3mn host refusal/Drop/mounted recovery loses the exact retirement owner");
  requireAll(winit, ["host.try_into_retirement()", "Err(mut host)", "OsHostRetirement::close_abandoned_step()"], "P3mn native close does not return refusal ownership or pump abandonment");
  requireAll(winit, ["if self.presenter.has_pending_presentation()", "self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);"], "P3mn retained presentation faults are not rescheduled for incremental close");
  requireAll(browser, ["host.try_into_retirement()", "Err(host)", "OsHostRetirement::close_abandoned_step()"], "P3mn browser close does not return refusal ownership or pump abandonment");
  if ((browser.match(/OsHostRetirement::close_abandoned_step\(\)/g)?.length ?? 0) < 2) failures.push("P3mn browser tick and close must both pump abandoned host ownership");
  requireAll(glue, [
    "self.engine.realize_step",
    "self.engine.observe_primary_metrics_generation(candidate.metrics_generation())",
    "self.engine.invalidate_primary_metrics_step()",
    "if !self.engine.invalidate_primary_metrics_step()",
    "pub(crate) fn close_engine_surface_step",
    "retained_fault: Option<String>",
    "cursor.phase = AppPresentPhase::Aborted",
    "self.retained_fault = Some(format!(\"engine canvas present: {error}\"))",
    "return Ok(AppPresentStep::Pending);",
  ], "P3mn AppPresenter does not mount staged realization, P5e invalidation, and paired close");
  requireAll(graph, ["pub struct GraphHostRetirement", "dag::DagHostRetirement::new(dag)", "pub fn close_step", "terminal_nonopaque_is_empty"], "P3mn GraphHost lacks its domain-owned retained disposer");
  requireAll(flow, ["pub struct FlowHostRetirement", "dag::DagHostRetirement::new(dag)", "NeuralCacheRetirement::new", "store.close_owned_step(1, 4_096)", "close_owned_terminal_is_empty"], "P3mn FlowHost lacks retained DAG/cache/history retirement");
  requireAll(mapRetirement, ["let MapHost {", "self.released\n            && self.overlays.is_empty()", "self.vector_tiles.is_empty()", "self.events.is_empty()"], "P3mn MapHost lacks an exhaustive field-wise retained owner witness");
  requireAll(editorRetirement, ["let EditorHost {", "self.released\n            && self.text.is_empty()", "self.semantic_tokens.is_empty()", "self.diagnostics.is_empty()"], "P3mn EditorHost lacks an exhaustive field-wise retained owner witness");
  requireAll(dagRetirement, ["let DagHost {", "self.released\n            && self.engine.terminal_is_empty()", "self.icon_paint_cache.terminal_is_empty()", "self.unresolved_input_ports.is_empty()"], "P3mn DagHost lacks an exhaustive field-wise retained owner witness");
  for (const [name, source] of [["MapHost", mapRetirement], ["EditorHost", editorRetirement], ["DagHost", dagRetirement]] as const)
    if (source.includes("ManuallyDrop::drop")) failures.push(`P3mn ${name} retirement still permits an ordinary whole-owner drop`);
  requireAll(engineSource, [
    "engine_surface_registry_accepts_exact_id_capacity_and_refuses_generation_exhaustion",
    "populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time",
    "populated_flow_surface_closes_history_and_cache_before_slot_reuse",
    "engine_packet_capacity_plus_one_returns_the_exact_snapshot_before_scene_transfer",
    "gpu_publish_freshness_uses_the_live_cpu_identity_metrics_document_and_scene",
    "normal_replacement_drains_displaced_renderer_view_texture_before_next_candidate",
    "child_and_outer_surface_retirements_require_explicit_field_witnesses",
  ], "P3mn MAX+1, one-fuel populated owner, and Flow history hostile laws are missing");
  requireAll(glueSource, ["realize_fault_remains_scheduled_until_the_aborted_cursor_is_terminal"], "P3mn retained realization-fault progress law is missing");
  requireAll(hostSource, ["interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one"], "P3mn interrupted Drop recovery and fixed-registry MAX+1 hostile law is missing");
  return failures;
}


//#endregion 🖥️P3mnMountedEngineSurfaceLifetime

//#region 🎨️P5dMountedPreparedRender
export function interactivityMountedPreparedRenderFailures(preparedSource: string, drawSource: string, gpuSource: string, glueSource: string, frameSource: string, sceneSource: string): string[] {
  const prepared = interactivityProductionSource(preparedSource);
  const draw = interactivityProductionSource(drawSource);
  const gpu = interactivityProductionSource(gpuSource);
  const glue = interactivityProductionSource(glueSource);
  const frame = interactivityProductionSource(frameSource);
  const scene = interactivityProductionSource(sceneSource);
  const failures: string[] = [];
  const requireAll = (source: string, needles: readonly string[], failure: string): void => {
    if (needles.some((needle) => !source.includes(needle))) failures.push(failure);
  };
  const preparedCredits = prepared.slice(prepared.indexOf("const PREPARED_RENDER_ITEM_SHIFT"), prepared.indexOf("//#endregion 📊️Credits"));
  const fixedMetadata = prepared.slice(prepared.indexOf("pub struct PreparedFixedList"), prepared.indexOf("const PREPARED_RENDER_ITEM_SHIFT"));
  const commandPages = prepared.slice(prepared.indexOf("pub struct PreparedRenderCommand {"), prepared.indexOf("/// 🧱️ Send-capable frame data"));
  const inputBoundary = prepared.slice(prepared.indexOf("pub struct PreparedRenderInput"), prepared.indexOf("//#endregion 📦️Packet"));
  const jobBoundary = prepared.slice(prepared.indexOf("pub struct PreparedRenderJob"), prepared.indexOf("//#endregion ⚙️PreparationJob"));
  const gpuBoundary = gpu.slice(gpu.indexOf("pub struct PreparedGpuPresentCursor"), gpu.indexOf("pub fn upload_font_atlas"));
  const gpuTimingBoundary = gpu.slice(gpu.indexOf("fn measure_prepared_gpu_opportunity("), gpu.indexOf("const PREPARED_GPU_ABANDONMENT_SLOTS"));
  const drawBoundary = draw.slice(draw.indexOf("pub fn clear_prepared_scene"), draw.indexOf("pub fn render_scene_content"));
  const preparationBoundary = glue.slice(glue.indexOf("impl AppFrameBuild"), glue.indexOf("pub(crate) struct AppPresenter"));
  const presenterBoundary = glue.slice(glue.indexOf("AppPresentPhase::Stage =>"), glue.indexOf("AppPresentPhase::Acknowledge =>"));
  const abandonmentBoundary = glue.slice(glue.indexOf("FrameBuildPhase::Deferred =>"), glue.indexOf("FrameBuildPhase::Text =>"));

  requireAll(preparedCredits, [
    "static PREPARED_RENDER_PROCESS_PERMITS: AtomicU64",
    "PREPARED_RENDER_SLOT_GENERATION",
    "compare_exchange(current, next",
    "fn try_grow(&mut self, pages: usize, backing_bytes: usize) -> bool",
    "fn release_step(&mut self) -> bool",
    "impl Drop for PreparedRenderProcessPermit",
  ], "P5d process item/page/backing credits are not fixed, cumulative, generation-qualified, or recoverable");
  if (preparedCredits.includes("Mutex<") || preparedCredits.includes("wrapping_add")) failures.push("P5d process credits retain a blocking or wrapping ledger");
  requireAll(fixedMetadata, [
    "pages: [Option<Box<PreparedFixedPage<T>>>; PREPARED_RENDER_METADATA_PAGES]",
    "pub fn try_push(&mut self, value: T) -> Result<(), T>",
    "fn release_backing_step(&mut self) -> bool",
  ], "P5d metadata is not a fixed paged exact-owner list");
  if (fixedMetadata.includes("Vec<") || fixedMetadata.includes("HashMap<")) failures.push("P5d metadata retains a dynamic working set");
  requireAll(commandPages, [
    "directories: [Option<Box<PreparedRenderCommandDirectory>>; PREPARED_RENDER_COMMAND_DIRECTORIES]",
    "pages: [Option<Box<PreparedRenderCommandPage>>; PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS]",
    "fn try_push(&mut self, command: PreparedRenderCommand) -> Result<(), PreparedRenderCommand>",
    "fn close_step(&mut self) -> bool",
    "draw_cursor: Option<DrawMeasureCursor>",
    "packet_overlay: bool",
  ], "P5d render commands are not fixed paged cursor-bearing owners");
  if (commandPages.includes("Vec<") || commandPages.includes("with_capacity")) failures.push("P5d command publication retains dynamic or whole-capacity staging");
  requireAll(inputBoundary, [
    "pub fn try_new(",
    "PreparedRenderProcessPermit::try_reserve",
    "permit.try_grow(pages, bytes)",
    "PREPARED_RENDER_INPUT_ABANDONMENT_STATE",
    "pub fn close_abandoned_step() -> bool",
    "impl Drop for PreparedRenderInput",
  ], "P5d input admission lacks pre-transfer credit, exact refusal, or interruption handback");
  requireAll(jobBoundary, [
    "impl InteractiveJob for PreparedRenderJob",
    "cx.consume_fuel(1)",
    "if cx.should_yield()",
    "input.preview_generation != cx.generation().0",
    "PREPARED_RENDER_JOB_ABANDONMENT_STATE",
    "impl Drop for PreparedRenderJob",
    "PreparedRenderCommandKind::Tessellate",
    "draw_cursor: Some(prepared_cursor)",
    "self.receiver.publish(packet)",
  ], "P5d worker job does not retain one fresh credited scalar through exact publication and Drop recovery");
  if ((jobBoundary.match(/cx\.consume_fuel\(1\)/g)?.length ?? 0) !== 3) failures.push("P5d worker job must consume exactly one fuel unit at each producer/backing/scalar opportunity");
  if ((jobBoundary.match(/input\.preview_generation != cx\.generation\(\)\.0/g)?.length ?? 0) !== 3) failures.push("P5d worker job must guard generation before and after every bounded producer/scalar call");
  for (const forbidden of ["\n        while ", "loop {", "thread::spawn", "WorkerPool::new(", "render_prepared(", "finish_prepared(", ".collect::<Vec"])
    if (jobBoundary.includes(forbidden)) failures.push(`P5d worker job reaches whole or nested work ${forbidden}`);
  requireAll(preparationBoundary, [
    "PreparedRenderJob::try_new(input)",
    "BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>",
    "fuel_per_step: 1",
    "step_budget_us: 1000",
    "WorkerJobSessionAdmissionRejected",
    "session.begin_close()",
    "session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)",
  ], "P5d mounted caller does not use the shared retained worker session with exact refusal and close");
  if (preparationBoundary.includes("step_budget_us: 16000")) failures.push("P5d mounted worker deadline exceeds the admitted one-millisecond slice");
  requireAll(gpuBoundary, [
    "PreparedGpuPresentPhase::ClearScene",
    "PreparedGpuPresentPhase::Commands",
    "PreparedGpuPresentPhase::BlurScene",
    "PreparedGpuPresentPhase::SnapshotBackdrop",
    "PreparedGpuPresentPhase::CompositeGlass",
    "command.draw_cursor()",
    "encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay(), None)",
    "encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay(), Some(scissor))",
    "measure_prepared_gpu_opportunity(cursor, command, semio_framework_job::default_now_us",
    "self.advance_prepared_present(packet, cursor)?",
    "prepared_glass_command(packet, cursor.command)?",
    "blit_prepared_composite(&self.device, &mut encoder, scene.mip_view(0), composite)",
    "cursor.command.checked_add(1)",
    "cursor.blur_mip.checked_add(1)",
    "impl Drop for PreparedGpuPresentCursor",
  ], "P5d GPU consumer does not retain one command/mip/glass/platform opportunity with watchdog and Drop handback");
  requireAll(gpuTimingBoundary, [
    "let complete = advance(cursor)?",
    "now().and_then(|now| now.checked_sub(started))",
    "admit_prepared_gpu_opportunity(cursor.overrun_run, elapsed)",
    "Ok(complete)",
  ], "P5d GPU timing must admit every successful bounded advancement before returning");
  for (const forbidden of ["render_prepared(", "finish_prepared(", "render_scene_content(", "composite_to_swapchain(", "run_blur_chain(", "while ", "loop {"])
    if (gpuBoundary.includes(forbidden)) failures.push(`P5d GPU consumer reaches whole work ${forbidden}`);
  requireAll(drawBoundary, [
    "encode_prepared_ui_scalar",
    "encode_prepared_vector_triangle",
    "encode_prepared_world_instance",
    "encode_prepared_world_line",
    "encode_prepared_blur_mip",
    "encode_prepared_glass_scalar",
    "std::slice::from_ref(instance)",
    "pass.draw(0..6, 0..1)",
    "pass.draw(0..3, 0..1)",
    "pass.draw_indexed(0..mesh.index_count, 0, 0..1)",
    "pass.draw(0..2, 0..1)",
  ], "P5d prepared GPU callees do not emit one bounded UI/vector/world/line/blur/glass semantic unit");
  for (const forbidden of [".collect(", "Vec::", "\n        for ", "\n        while ", "loop {"])
    if (drawBoundary.includes(forbidden)) failures.push(`P5d prepared GPU callee retains dynamic or iterative work ${forbidden}`);
  requireAll(presenterBoundary, [
    "self.gate.stage_presented(packet)",
    "self.gpu.begin_prepared_present(packet, raster_witness)",
    "self.gpu.prepared_present_step(packet, gpu_cursor)",
    "gpu_cursor.begin_close()",
    "AppPresentPhase::CloseGpu",
  ], "P5d presenter does not retain generation-qualified packet/GPU owners through submit and close");
  requireAll(abandonmentBoundary, [
    "PreparedGpuPresentCursor::close_abandoned_step()",
    "PreparedRenderInput::next_abandoned_close_demands(4096)",
    "PreparedRenderInput::close_abandoned_step(grant)",
    "PreparedRenderJob::next_abandoned_close_demands(4096)",
    "PreparedRenderJob::close_abandoned_step(grant)",
    "PreparedRenderReceiver::next_abandoned_close_demands(4096)",
    "PreparedRenderReceiver::close_abandoned_step(grant)",
    "PreparedRenderPacket::next_abandoned_close_demands(4096)",
    "PreparedRenderPacket::close_abandoned_step(grant)",
  ], "P5d mounted frame drain does not recover every interrupted prepared owner incrementally");
  if (prepared.includes("Arc<Mutex<Vec<PreparedRenderPacket>>") || prepared.includes("Option<Arc<PreparedRenderPacket>>")) failures.push("P5d packet ownership is cloneable, blocking, or dynamically queued");
  if (gpu.includes("fn render_prepared") || gpu.includes("fn finish_prepared")) failures.push("P5d whole prepared renderer remains production reachable");
  if (frame.includes("pub fn build_frame") || scene.includes("pub fn finish(")) failures.push("P5d dormant synchronous frame/scene builder remains production reachable");
  requireAll(preparedSource, [
    "input_drop_hands_back_exact_process_permits_for_incremental_close",
    "worker_panic_hands_back_the_exact_job_and_mailbox_owners",
    "packet_drop_retires_nested_backings_and_permit_scalars_separately",
    "fixed_command_pages_reject_max_plus_one_without_consuming_the_owner",
    "tessellation_commands_retain_exact_scalar_and_overlay_cursors",
    "preparation_rejects_a_stale_generation_before_publication",
    "preparation_observes_cancellation_without_replacing_a_packet",
  ], "P5d exact Drop/MAX+1/cursor/stale/cancel hostile laws are missing");
  requireAll(gpuSource, [
    "interrupted_present_cursor_hands_back_generation_and_fixed_owners",
    "present_cursor_generation_and_capacity_boundaries_refuse_before_ownership",
  ], "P5d GPU interruption/generation/MAX+1 hostile laws are missing");
  return failures;
}


//#endregion 🎨️P5dMountedPreparedRender

//#region 📐️P5eMountedSurfaceLane
export function interactivityMountedSurfaceLaneFailures(surfaceSource: string, engineSource: string, hostSource: string, winitSource: string, glueSource: string): string[] {
  const surface = interactivityProductionSource(surfaceSource);
  const engine = interactivityProductionSource(engineSource);
  const host = interactivityProductionSource(hostSource);
  const winit = interactivityProductionSource(winitSource);
  const glue = interactivityProductionSource(glueSource);
  const failures: string[] = [];
  const requireAll = (source: string, needles: readonly string[], failure: string): void => {
    if (needles.some((needle) => !source.includes(needle))) failures.push(failure);
  };
  const lane = surface.slice(surface.indexOf("pub(crate) const SURFACE_RESIZE_LANE_CAPACITY"), surface.indexOf("//#endregion 🔖️SurfaceLane"));
  const job = lane.slice(lane.indexOf("struct SurfaceResizeJob"), lane.indexOf("pub(crate) struct SurfaceResizeLaneAdmissionRejected"));
  const theme = engine.slice(engine.indexOf("struct SurfaceLaneRing"), engine.indexOf("pub fn set_window_kind_icons"));
  const metrics = winit.slice(winit.indexOf("fn handle_metrics(&mut self"), winit.indexOf("fn redraw(&mut self"));
  const redraw = winit.slice(winit.indexOf("fn redraw_core(&mut self"), winit.indexOf("/// 🖼️ `build_and_publish_snapshot`"));
  const nativeClose = winit.slice(winit.indexOf("WindowEvent::CloseRequested"), winit.indexOf("fn recompute_control_flow"));
  const presenter = glue.slice(glue.indexOf("enum AppSurfaceResizePhase"), glue.indexOf("pub(crate) fn has_pending_presentation"));

  requireAll(lane, [
    "SURFACE_RESIZE_LANE_CAPACITY: usize = 64",
    "SURFACE_RESIZE_STEP_FUEL: u64 = 1;",
    "SURFACE_RESIZE_STEP_BUDGET_MS: u64 = 1;",
    "static SURFACE_LANE_OCCUPIED: [AtomicBool; SURFACE_RESIZE_LANE_CAPACITY]",
    "static SURFACE_LANE_GENERATIONS: [AtomicU64; SURFACE_RESIZE_LANE_CAPACITY]",
    "checked_add(1)",
    "MountedWorkerJobSession<SurfaceResizeJob>",
    "Lane::Interactive",
    "fuel_per_step: SURFACE_RESIZE_STEP_FUEL",
    "step_budget_ms: SURFACE_RESIZE_STEP_BUDGET_MS",
    "!scale_factor.is_finite() || scale_factor <= 0.0",
    "candidate.metrics_generation == self.metrics_generation",
  ], "P5e resize lane is not fixed, generation-qualified, finite-normalized, shared-pool mounted, and latest-wins");
  for (const forbidden of ["Vec<", "VecDeque", "HashMap<", "WorkerPool::new(", "thread::spawn", "wrapping_add", "saturating_add"])
    if (lane.includes(forbidden)) failures.push(`P5e mounted resize lane retains dynamic/private/wrapping work ${forbidden}`);
  if ((lane.match(/candidate\.metrics_generation == self\.metrics_generation/g)?.length ?? 0) !== 2) failures.push("P5e take and restore must both reject a stale metrics candidate");
  requireAll(job, [
    "ResizePhase::LogicalWidth",
    "ResizePhase::LogicalHeight",
    "ResizePhase::Seal",
    "cx.consume_fuel(SURFACE_RESIZE_STEP_FUEL)",
    "cx.is_cancelled() || cx.deadline_exceeded()",
    "self.candidate = Some(PreparedSurfaceResize",
    "fn close_step(&mut self, maximum_items: usize",
  ], "P5e worker does not retain one scalar phase with cancel/deadline and exact close");
  requireAll(lane, [
    "static SURFACE_LANE_ABANDONMENT: [AtomicPtr<MountedSurfaceResizeLane>; SURFACE_RESIZE_LANE_CAPACITY]",
    "pub(crate) fn close_abandoned_step() -> bool",
    "impl Drop for MountedSurfaceResizeLane",
    "SURFACE_LANE_ABANDONMENT[slot].store(Box::into_raw(lane), Ordering::Release)",
    "SURFACE_LANE_OCCUPIED[slot].store(false, Ordering::Release)",
  ], "P5e resize lane lacks generation-addressable Drop recovery and exact permit release");
  requireAll(metrics, [
    "enqueue_host_metrics(",
    "self.surface_resize.enqueue(metrics.physical.width, metrics.physical.height, metrics.scale_factor)",
    "RuntimeApply::Resize { width, height, dpr }",
  ], "P5e mounted metrics callback does not publish fixed metrics to both retained consumers");
  for (const forbidden of ["presenter.resize(", "gpu.resize(", "surface.configure", "create_texture", "create_view"])
    if (metrics.includes(forbidden)) failures.push(`P5e metrics callback reaches immediate platform work ${forbidden}`);
  requireAll(redraw, [
    "MountedSurfaceResizeLane::close_abandoned_step()",
    "self.surface_resize.drive_one()",
    "self.presenter.surface_resize_available()",
    "self.surface_resize.take_ready()",
    "self.presenter.begin_surface_resize(candidate)",
    "self.presenter.surface_resize_step()",
    "InvalidationReason::VIEWPORT",
  ], "P5e redraw does not rediscover, pump, admit, and progress the exact retained resize owner");
  requireAll(presenter, [
    "AppSurfaceResizePhase::Apply",
    "AppSurfaceResizePhase::Retire",
    "candidate.suspended()",
    "self.gpu.resize(candidate.logical_width(), candidate.logical_height(), candidate.scale_factor())",
    "close_surface_resize_step",
  ], "P5e presenter does not retain zero-size-safe UI capability progress and close");
  requireAll(host, [
    "surface_resize: crate::surface_lane::SurfaceResizeAuthority",
    "surface_resize: Option<crate::surface_lane::SurfaceResizeAuthority>",
    "surface_resize.begin_close()",
    "surface_resize.close_step()",
    "surface_resize.terminal_is_empty()",
    "struct OsHostRetirementState",
  ], "P5e OsHost and retirement do not preserve the exact surface-lane authority");
  requireAll(nativeClose, [
    "host.try_into_retirement()",
    "Err(mut host)",
    "retirement.close_step() && retirement.terminal_is_empty()",
    "ControlFlow::wait_duration(std::time::Duration::from_millis(1))",
  ], "P5e native close does not incrementally retire the populated host before exit");
  requireAll(theme, [
    "struct SurfaceLaneEntry",
    "slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS]",
    "reason: SurfaceLayoutReason",
    "epoch: u64",
    "struct ThemePropagationCursor",
    "tokens: [Option<UiSurfaceToken>; UI_LAYOUT_SURFACE_SLOTS]",
    "ThemePropagationPhase::Validate",
    "ThemePropagationPhase::Apply",
    "ThemePropagationPhase::Publish",
    "fn drive_theme_propagation_one(&mut self) -> bool",
    "self.enqueue_layout_token(token, SurfaceLayoutReason::Theme)",
    "window.layout_generation != entry.epoch",
  ], "P5e UI scheduler lacks fixed reason/epoch entries or retained one-surface theme propagation");
  const setTheme = theme.slice(theme.indexOf("pub fn set_theme"), theme.indexOf("pub fn try_admit_surface"));
  requireAll(setTheme, ["self.theme_propagation = Some(ThemePropagationCursor::new(theme))", "self.pending_theme = Some(theme)"], "P5e theme callback does not retain current and superseding fixed theme owners");
  for (const forbidden of [".ids().cloned()", "for id in ids", ".collect::<Vec", "VecDeque"])
    if (setTheme.includes(forbidden)) failures.push(`P5e theme callback retains whole-window work ${forbidden}`);
  requireAll(surfaceSource, [
    "resize_job_consumes_one_scalar_per_grant",
    "million_resize_samples_retain_only_the_latest_exact_request",
    "zero_size_suspends_and_invalid_scale_returns_exact_producer",
    "interrupted_lane_drop_is_rediscovered_and_incrementally_closed",
  ], "P5e resize scalar/coalescing/zero-size/Drop hostile laws are missing");
  requireAll(engineSource, ["changed_theme_propagates_one_fixed_surface_slot_per_opportunity", "resize_storm_coalesces_to_one_latest_surface_job", "interactive_storm_does_not_starve_background_surface_lane"], "P5e multi-window bounded theme/resize/fairness laws are missing");
  return failures;
}


//#endregion 📐️P5eMountedSurfaceLane
