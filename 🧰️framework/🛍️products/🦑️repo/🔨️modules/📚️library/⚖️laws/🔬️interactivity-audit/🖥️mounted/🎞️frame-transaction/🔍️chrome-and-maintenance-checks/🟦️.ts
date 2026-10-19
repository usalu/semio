/** 🔍️ The mounted frame transaction checks of the transaction, maintenance, preference and chrome owners. */
/** 🧾 Checks the frame transaction cursors and the maintenance owners. */
export function checkTransactionAndMaintenanceOwners({ requireAll, transaction, glue, glueSource, failures, frameCursors, buildBoundary, finishBoundary, deferredBoundary, maintenanceOwnerBoundary, maintenanceNativeOwnerBoundary, maintenanceCloseBoundary, maintenanceExecutionBoundary, maintenanceRegistryBoundary, maintenanceSpawnBoundary, maintenanceTerminalBoundary, maintenanceWorkerTerminalBoundary, findBoundary, shellSource, maintenanceBoundary }: {
  readonly requireAll: (source: string | undefined, needles: readonly string[], label: string) => void;
  readonly transaction: string;
  readonly glue: string;
  readonly glueSource: string;
  readonly failures: string[];
  readonly frameCursors: string;
  readonly buildBoundary: string;
  readonly finishBoundary: string;
  readonly deferredBoundary: string;
  readonly maintenanceOwnerBoundary: string;
  readonly maintenanceNativeOwnerBoundary: string;
  readonly maintenanceCloseBoundary: string | undefined;
  readonly maintenanceExecutionBoundary: string;
  readonly maintenanceRegistryBoundary: string;
  readonly maintenanceSpawnBoundary: string;
  readonly maintenanceTerminalBoundary: string;
  readonly maintenanceWorkerTerminalBoundary: string;
  readonly findBoundary: string;
  readonly shellSource: string;
  readonly maintenanceBoundary: string;
}) {
  requireAll(transaction, [
    "DrainProjectionDeltas",
    "RouteIntents",
    "FlushEffects",
    "PresentSurface",
    "ReconcileTree",
    "BuildRenderPackets",
    "PublishSnapshot",
    "context.set_stage(self.stage_label())",
  ], "P5a exact seven-stage mounted transaction is incomplete");
  requireAll(glue, [
    "struct FrameActionOwners",
    "slots: [Option<ActionDescriptor>; WORLD3D_DEADLINE_CAPACITY]",
    "fn try_push(&mut self, action: ActionDescriptor) -> Result<(), ActionDescriptor>",
    "fn pop_front(&mut self) -> Option<ActionDescriptor>",
    "EFFECT_STORM_BUDGET: u32 = 64",
  ], "P5a action/effect admission is not fixed, fallible, and identity preserving");
  requireAll(glueSource, ["frame_action_max_plus_one_returns_the_exact_owner", "assert_eq!(rejected.controller_id.as_ptr(), identity)"], "P5a exact MAX + 1 action owner law is missing");
  requireAll(transaction, [
    "operation: semio_framework_trace::OperationId",
    "generation: semio_framework_trace::Generation",
    "base_witness: Option<RuntimePresentationWitness>",
    "context.operation() != self.operation",
    "context.generation() != self.generation",
    "context.is_cancelled()",
    "context.deadline_exceeded()",
    "base_witness != current_witness",
    "checked_add(1)",
  ], "P5a opportunity freshness, revision, generation, cancellation, deadline, or effect guards are incomplete");
  for (const forbidden of ["VecDeque", "std::vec::IntoIter", "wrapping_add", "saturating_add", ".clear()", ".expect(", ".unwrap(", "loop {", "while "]) if (transaction.includes(forbidden)) failures.push(`P5a mounted transaction contains forbidden ${forbidden}`);
  requireAll(frameCursors, [
    "struct FrameBuildCursor",
    "previous_draw: Option<DrawList>",
    "previous_overlay: Option<DrawList>",
    "struct FrameFinishCursor",
    "icon_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>",
    "glyph_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>",
  ], "P5a mounted build/finish cursors do not retain draw, overlay, and atlas page ownership");
  requireAll(glue, ["struct FrameEnginePackets", "engine_packets: FrameEnginePackets", "fn try_push(&mut self, packet: engine_canvas::EngineCanvasPacket) -> Result<(), engine_canvas::EngineCanvasPacket>"], "P5a mounted packet collector is not fixed and identity-preserving");
  if (frameCursors.includes("engine_packets: Vec<") || frameCursors.includes("Vec<engine_canvas::EngineCanvasPacket>")) failures.push("P5a mounted frame cursor retains a dynamic engine packet workset");
  requireAll(buildBoundary, [
    "FrameBuildPhase::PreparedGpuAbandonment",
    "PreparedGpuPresentCursor::close_abandoned_step()",
    "FrameBuildPhase::PreparedInputAbandonment",
    "PreparedRenderInput::next_abandoned_close_demands(4096)",
    "PreparedRenderInput::close_abandoned_step(grant)",
    "FrameBuildPhase::PreparedJobAbandonment",
    "PreparedRenderJob::next_abandoned_close_demands(4096)",
    "PreparedRenderJob::close_abandoned_step(grant)",
    "FrameBuildPhase::PreparedAbandonment",
    "PreparedRenderReceiver::next_abandoned_close_demands(4096)",
    "PreparedRenderReceiver::close_abandoned_step(grant)",
    "FrameBuildPhase::PacketAbandonment",
    "PreparedRenderPacket::next_abandoned_close_demands(4096)",
    "PreparedRenderPacket::close_abandoned_step(grant)",
    "FrameBuildPhase::RetireDraw",
    "previous.retire_step()",
    "FrameBuildPhase::RetireOverlay",
    "render_chrome_step",
    "resources.append_step(input)",
    "resources.take_packet_step()",
    "PreparedAtlasPages::try_new",
    "pages.push_page(&self.icons.pixels, pages.next_row())",
    "PreparedRenderUpload::IconAtlasPages",
  ], "P5a pre-input boundary does not advance retained draw, chrome, packet, and upload owners one step at a time");
  requireAll(finishBoundary, [
    "FrameFinishPhase::GlyphUpload",
    "pages.push_page(&self.atlas.pixels, pages.next_row())",
    "PreparedRenderUpload::GlyphAtlasPages",
    "FrameFinishPhase::Draw",
    "FrameFinishPhase::Overlay",
    "self.pending_frame_deferred = Some",
  ], "P5a post-input boundary does not page atlas work and retain draw/overlay/deferred transfers");
  for (const forbidden of ["frame_before_input(", "frame_after_input(", "drive_pending_frame_deferred", "take_packets(", "append_to(", "self.draw.clear()", "self.overlay.clear()", "self.icons.pixels.clone()", "self.atlas.pixels.clone()"])
    if (glue.includes(forbidden)) failures.push(`P5a mounted call graph retains opaque whole-work callee ${forbidden}`);
  requireAll(deferredBoundary, ["cursor_value.take_next()", "spawn_frame_deferred_reserved", "try_spawn_frame_maintenance_reserved", "pending_frame_maintenance_refusal", "FrameDeferredCursor::close_step", "runtime.return_interaction(interaction)"], "P5a deferred boundary does not take, refuse, close, and resume exactly one retained work owner");
  for (const forbidden of [".expect(", ".unwrap(", "loop {", "while "])
    if (deferredBoundary.includes(forbidden)) failures.push(`P5a deferred boundary retains panicking or run-to-completion ${forbidden}`);
  requireAll(maintenanceOwnerBoundary, [
    "generation: u64",
    "cancel: semio_framework_async::CancelToken",
    "closing: bool",
    "struct FrameMaintenanceOwner",
    "deadline_ms: Option<u64>",
    "struct FrameMaintenanceOwnerCell<T>",
    "state: std::sync::atomic::AtomicU8",
    "compare_exchange(0, 1",
    "fn try_restore(&self, owner: T) -> Result<(), T>",
    "struct FrameMaintenanceRefusal",
    "struct FrameMaintenanceAuthority",
    "fn is_live(&self, generation: u64) -> bool",
    "fn release(&self, generation: u64) -> bool",
    "self.generation.store(0, Ordering::Release)",
    "compare_exchange(1, 0",
  ], "P5a maintenance authority is not generation/cancel/deadline-qualified with nonblocking exact-owner refusal handback");
  requireAll(maintenanceNativeOwnerBoundary, ["#[cfg(any(not(target_arch = \"wasm32\"), test))]"], "P5a native maintenance close is not available in production");
  requireAll(maintenanceCloseBoundary, ["self.closing = true"], "P5a native maintenance close does not retain its closing state");
  requireAll(maintenanceExecutionBoundary, [
    "struct FrameMaintenanceExecutionRegistry",
    "const QUEUED: u8 = 2",
    "const RUNNING: u8 = 3",
    "const ABANDONED: u8 = 4",
    "fn try_publish(&self, generation: u64",
    "fn try_begin(&self, generation: u64)",
    "fn abandon(&self, generation: u64) -> bool",
    "fn try_take_abandoned(&self)",
    "struct FrameMaintenanceExecutionEnvelope",
    "impl Drop for FrameMaintenanceExecutionEnvelope",
    "if self.armed && self.registry.abandon(self.generation)",
    "RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }",
    "struct FrameMaintenanceExecutionGuard",
    "impl Drop for FrameMaintenanceExecutionGuard",
    "self.owner_cell.restore_taken(owner)",
    "frame_maintenance_executions: Arc<FrameMaintenanceExecutionRegistry>",
  ], "P5a accepted maintenance execution lacks generation-qualified external Drop handback");
  for (const forbidden of ["Mutex", ".lock()", "std::mem::take", "Vec<", "loop {", "while "])
    if (maintenanceRegistryBoundary.includes(forbidden)) failures.push(`P5a maintenance execution registry retains blocking, dynamic, or recursive recovery ${forbidden}`);
  requireAll(maintenanceSpawnBoundary, [
    "fn try_spawn_frame_maintenance_reserved",
    "frame_maintenance.try_reserve(generation)",
    "renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job)",
    "error.into_job()",
    "FrameMaintenanceRefusal { owner: reclaimed, reservation_live: true }",
    "cursor.cancel.is_cancelled_now()",
    "presentation_authority.witness_for(generation).is_none()",
    "worker_pool.now_ms() > deadline",
    "frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded)",
    "cursor.begin_close()",
    "frame_maintenance.release(generation)",
    "execution.finish()",
  ], "P5a maintenance submission does not preserve exact owners across refusal/cancel/stale/deadline/close");
  requireAll(maintenanceTerminalBoundary, ["if cancelled", "else if stale", "else if deadline_exceeded", "frame maintenance cancelled", "frame maintenance generation became stale", "frame maintenance deadline expired"], "P5a maintenance terminal classifier is incomplete for cancel/stale/deadline");
  requireAll(maintenanceWorkerTerminalBoundary, ["frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded)", "cursor.begin_close()", "interaction.frame_fault = Some(fault.to_string())"], "P5a maintenance terminal outcome does not enter retained close before handback");
  for (const forbidden of ["KernelPoolFuture::spawn", "let _ = renderer_worker_pool().try_submit", "renderer_worker_pool().submit(", "Lane::Interactive", ".lock()", "loop {", "while "])
    if (maintenanceSpawnBoundary.includes(forbidden)) failures.push(`P5a maintenance submission retains discarded, blocking, interactive-lane, or whole-work ${forbidden}`);
  if (maintenanceOwnerBoundary.includes("self.actions = FrameActionOwners::default()")) failures.push("P5a deferred close bulk-drops the populated action owner");
  requireAll(findBoundary, [
    "slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>",
    "payload_bytes: usize",
    "generation: u64",
    "fn try_push_at(&mut self, generation: u64, item: ShellFindItem) -> Result<(), ShellFindItem>",
    "fn pop_front(&mut self) -> Option<ShellFindItem>",
    "fn close_step(&mut self) -> bool",
    "std::cell::Cell<Option<ActiveShellFindItems>>",
    "std::ptr::NonNull<ShellFindItems>",
    "pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem>",
    "collector.try_push_at(active.generation, item)",
  ], "P5a find producer is not fixed, generation-qualified, nonblocking, and exact-owner fallible");
  for (const forbidden of ["Arc<Mutex<Vec<ShellFindItem>>>", "Vec<ShellFindItem>", ".lock()", "std::mem::take", "take_find_items", "ACTIVE_FIND_ITEM_SINKS"])
    if (findBoundary.includes(forbidden)) failures.push(`P5a find producer retains blocking, dynamic, or whole-owner ${forbidden}`);
  requireAll(shellSource, ["find_item_max_plus_one_returns_the_exact_owned_item", "stale_find_generation_returns_the_exact_owned_item", "assert_eq!(rejected.id.as_ptr(), identity)"], "P5a find MAX + 1, stale-generation, close, or exact-owner law is missing");
  requireAll(maintenanceBoundary, [
    "load_requested: bool",
    "load_phase: u8",
    "introduction_read: Option<String>",
    "introduction_write: Option<String>",
    "persist_phase: u8",
    "request_chrome_preferences_load",
    "request_introduction_read",
    "request_panel_layout_persist",
    "request_presence_preview",
    "request_chrome_preferences_persist",
    "pub(crate) fn advance_chrome_maintenance_step",
    "read_ui_preferences()",
    "stored_field_get(",
    "stored_field_set(",
    "persist_ui_preferences(",
  ], "P5a Shell persistence/presence work is not a retained one-field/page child");
}

/** ⚙ Checks the preference, fixed file page and chrome step boundaries. */
export function checkPreferencesAndChromeSteps({ requireAll, prefsBoundary, fixedFilePageBoundary, servicesSource, failures, nativePrefsBoundary, chromeBoundary, glue, shellChildren, chromeTextBoundary, chromeGroupBoundary }: {
  readonly requireAll: (source: string | undefined, needles: readonly string[], label: string) => void;
  readonly prefsBoundary: string;
  readonly fixedFilePageBoundary: string;
  readonly servicesSource: string;
  readonly failures: string[];
  readonly nativePrefsBoundary: string;
  readonly chromeBoundary: string | undefined;
  readonly glue: string;
  readonly shellChildren: string | undefined;
  readonly chromeTextBoundary: string | undefined;
  readonly chromeGroupBoundary: string | undefined;
}) {
  requireAll(prefsBoundary, [
    "fn native_pref_field_path",
    "key.len() > SHELL_CHROME_IO_FIELD_BYTES",
    "fn native_pref_read_page",
    "semio_framework_os_services::storage_worker_read_fixed_file_document(&path, OS_SHELL_CONFIG_MAX_BYTES)",
    "semio_framework_os_services::storage_worker_read_fixed_file_page(&path, SHELL_CHROME_IO_FIELD_BYTES)",
    "String::from_utf8(page)",
    "fn native_pref_write_page",
    "semio_framework_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), SHELL_CHROME_IO_FIELD_BYTES)",
    "fn prefs_get(",
    "fn prefs_set(",
  ], "P5a native preference maintenance does not delegate document and field pages to the host-storage service");
  requireAll(fixedFilePageBoundary, [
    "pub const STORAGE_FIXED_FILE_PAGE_BYTES: usize = 16 * 1024",
    "pub fn storage_worker_read_fixed_file_page",
    "maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES",
    "logical_bytes > maximum_bytes",
    "let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES]",
    "file.read_exact(&mut page[..logical_bytes])",
    "if file.read(&mut trailing)? != 0",
    "pub fn storage_worker_write_fixed_file_page",
    "bytes.len() > maximum_bytes",
    "std::fs::create_dir_all(parent)?",
    "file.write_all(bytes)",
  ], "P5a host-storage service does not own exact 16 KiB page admission/read/write and growth rejection");
  if (!servicesSource.includes("fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page")) failures.push("P5a host-storage fixed page exact/max-plus-one/oracle preservation law is missing");
  for (const forbidden of ["PREFS_STORE", "Mutex", ".lock()", "serde_json::from_", "serde_json::to_", "OS_SHELL_CONFIG_STORAGE_KEY", "Vec::with_capacity", "vec![", "loop {", "while ", "std::fs::", "std::io::"])
    if (nativePrefsBoundary.includes(forbidden)) failures.push(`P5a native preference field page retains blocking, dynamic, or whole-config ${forbidden}`);
  for (const forbidden of ["load_ui_prefs_once()", "read_stored_introduction_seen(", "persist_panel_layout_if_changed()", "write_stored_introduction_seen(", "persist_ui_prefs_if_changed()", "publish_presence_heartbeat()"])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a chrome frame still reaches synchronous maintenance ${forbidden}`);
  requireAll(glue, ["FrameDeferredWork::ShellMaintenance", "shell_maintenance: bool", "semio_framework_async::Lane::Io", "PreparedAtlasPages::next_abandoned_close_demands(4096)", "PreparedAtlasPages::close_abandoned_step(grant)"], "P5a Shell I/O or abandoned atlas close is not mounted on the shared retained boundary");
  requireAll(chromeBoundary, [
    "ShellChromeFramePhase::MainWindow",
    "ShellChromeFramePhase::PaneOverlayHits",
    "ShellChromeFramePhase::Panels",
    "ShellChromeFramePhase::AgentChatHeader",
    "ShellChromeFramePhase::Navbar",
    "ShellChromeFramePhase::TutorialBar",
    "ShellChromeFramePhase::Footer",
    "ShellChromeFramePhase::Overlay",
    "ShellChromeFramePhase::IntroductionWrite",
    "introduction_seen_writes.pop()",
  ], "P5a shell chrome boundary is not a retained one-child cursor");
  requireAll(chromeBoundary, [
    "render_main_window_step(&mut cursor.child",
    "render_mobile_panel_step(&mut cursor.child",
    "render_panel_step(&mut cursor.child, anchor",
    "render_agent_chat_header_step(&mut cursor.child",
    "render_navbar_step(&mut cursor.child",
    "render_tutorial_bar_step(&mut cursor.child",
    "render_footer_step(&mut cursor.child",
    "render_overlay_step(&mut cursor.child",
    "cursor.advance(ShellChromeFramePhase::PaneOverlayHits)",
  ], "P5a shell chrome phases do not remain parked on retained child cursors");
  for (const forbidden of ["self.render_main_window(", "self.render_left_panel(", "self.render_right_panel(", "self.render_navbar(", "self.render_tutorial_bar(", "self.render_footer(", "self.render_overlay(", "self.render_tree_drag_overlay(", "render_tutorial_gesture_overlay("])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a shell chrome cursor reaches whole child ${forbidden}`);
  for (const forbidden of ["render_chrome(", "render_chrome_build(", ".clear()", "std::mem::take(&mut self.chrome_build.introduction_seen_writes)", "for app_id in", "while "])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a shell chrome cursor retains whole-work ${forbidden}`);
  requireAll(shellChildren, [
    "fn render_main_window_step",
    "render_ui_document_step(",
    "fn render_panel_step",
    "fn render_navbar_step",
    "fn render_tutorial_bar_step",
    "fn render_footer_step",
    "fn render_overlay_step",
    "fn render_context_menu_step",
    "fn render_chrome_tooltip_step",
    "fn render_chrome_dialog_step",
    "fn render_chrome_tour_step",
  ], "P5a live Shell children are not retained node/widget cursors");
  for (const forbidden of ["render_main_window(", "render_left_panel(", "render_right_panel(", "render_navbar(", "render_tutorial_bar(", "render_footer(", "render_overlay(", "render_ui_document(", "render_context_menu(", "render_chrome_tour(", "measure_chrome_group_item(", "render_chrome_group("])
    if (shellChildren?.includes(forbidden)) failures.push(`P5a live Shell child body reaches whole subtree ${forbidden}`);
  requireAll(chromeTextBoundary, [
    "fn chrome_text_step",
    "paint_retained_glyph_step",
    "fn chrome_text_complete_step",
    "RetainedGlyphStep::Pending",
    "RetainedGlyphStep::Complete",
    "RetainedGlyphStep::Fault",
    "cursor.reset()",
  ], "P5a Shell text callee is not a retained per-scalar/glyph authority");
  requireAll(chromeGroupBoundary, [
    "enum RetainedChromeGroupStep",
    "fn retained_chrome_group_item_width",
    "label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES",
    "fn render_retained_chrome_group_item_step",
    "group_phase: &mut u8",
    "glyph: &mut RetainedGlyphCursor",
    "chrome_text_complete_step",
    "RetainedChromeGroupStep::Pending",
    "RetainedChromeGroupStep::Complete",
    "RetainedChromeGroupStep::Fault",
  ], "P5a Shell chrome-group callee is not a fixed retained output/glyph authority");
}

/** 🔤 Checks the chrome text steps and the engine frame boundaries. */
export function checkChromeTextAndEngineFrame({ requireAll, shellSource, shell, failures, chromeGroupStepBoundary, chromeDialogBoundary, chromeTourBoundary, chromeTutorialBoundary, chromeFooterBoundary, chromeOverlayBoundary, chromeBoundary, chromeTextBoundary, chromeNavbarBoundary, documentBoundary, uiFrameBoundary, paint, interactiveSyncBoundary }: {
  readonly requireAll: (source: string | undefined, needles: readonly string[], label: string) => void;
  readonly shellSource: string;
  readonly shell: string;
  readonly failures: string[];
  readonly chromeGroupStepBoundary: string | undefined;
  readonly chromeDialogBoundary: string | undefined;
  readonly chromeTourBoundary: string | undefined;
  readonly chromeTutorialBoundary: string | undefined;
  readonly chromeFooterBoundary: string | undefined;
  readonly chromeOverlayBoundary: string | undefined;
  readonly chromeBoundary: string | undefined;
  readonly chromeTextBoundary: string | undefined;
  readonly chromeNavbarBoundary: string | undefined;
  readonly documentBoundary: string;
  readonly uiFrameBoundary: string;
  readonly paint: string;
  readonly interactiveSyncBoundary: string | undefined;
}) {
  requireAll(shellSource, [
    "#[cfg(test)]\nfn measure_chrome_group_item",
    "#[cfg(test)]\nfn render_chrome_group",
  ], "P5a legacy whole-string chrome-group oracles are not explicitly test-only");
  for (const forbidden of ["fn measure_chrome_group_item", "fn render_chrome_group"])
    if (shell.includes(forbidden)) failures.push(`P5a production Shell retains legacy whole-string chrome-group reachability ${forbidden}`);
  for (const forbidden of ["draw_text(", "chrome_text(", "measure_text(", "for ch in", "for item in", "loop {", "while "])
    if (chromeGroupStepBoundary?.includes(forbidden)) failures.push(`P5a retained Shell chrome group reaches whole work ${forbidden}`);
  requireAll(chromeDialogBoundary, ["chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false", "cursor.scalar += 1"], "P5a live dialog does not park on the retained glyph cursor");
  requireAll(chromeTourBoundary, ["chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false", "cursor.scalar += 1"], "P5a live tour does not park on the retained glyph cursor");
  requireAll(chromeTutorialBoundary, ["fn render_tutorial_bar_step", "chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false"], "P5a mounted tutorial bar does not park dynamic text on the retained glyph cursor");
  requireAll(chromeTutorialBoundary, ["render_retained_chrome_group_item_step", "retained_chrome_group_item_width", "RetainedChromeGroupStep::Pending => return false"], "P5a mounted tutorial groups do not park each output and glyph opportunity");
  requireAll(chromeFooterBoundary, ["render_retained_panel_chrome_item_step", "render_footer_status_step", "Ok(false) => return false"], "P5a mounted footer does not park on retained dynamic utility and sync labels");
  requireAll(chromeOverlayBoundary, ["fn render_overlay_step", "fn render_context_menu_step", "fn render_chrome_tooltip_step", "fn render_chrome_dialog_step", "fn render_chrome_tour_step", "chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false"], "P5a mounted overlay children do not park dynamic text on the retained glyph cursor");
  requireAll(chromeBoundary, ["ShellChromeFramePhase::Error", "chrome_text_complete_step(draw, atlas, error", "&mut cursor.child.glyph", "Ok(false) => return false"], "P5a mounted Shell error path does not park on the retained glyph cursor");
  for (const [boundary, label] of [[chromeTextBoundary, "text callee"], [chromeDialogBoundary, "dialog"], [chromeTourBoundary, "tour"]] as const)
    for (const forbidden of ["draw_text(", "chrome_text(", "for ch in", "for character in", ".chars().for_each", ".chars().count()", "loop {", "while "])
      if (boundary?.includes(forbidden)) failures.push(`P5a Shell ${label} retains whole-string work ${forbidden}`);
  for (const [boundary, label] of [[chromeNavbarBoundary, "navbar"], [chromeTutorialBoundary, "tutorial"], [chromeFooterBoundary, "footer"], [chromeOverlayBoundary, "overlay"]] as const)
    for (const forbidden of ["draw_text(", "chrome_text(", ".chars().count()", "for ch in", "for character in", "loop {", "while "])
      if (boundary?.includes(forbidden)) failures.push(`P5a mounted Shell ${label} reaches whole-string work ${forbidden}`);
  requireAll(shellSource, ["dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant", "dialog_and_tour_text_max_plus_one_fails_without_output", "dynamic_chrome_group_retains_every_glyph_and_border_opportunity", "dynamic_chrome_group_max_plus_one_is_identity_preserving"], "P5a Shell multi-megabyte one-scalar, chrome-group, or MAX + 1 law is missing");
  requireAll(documentBoundary, [
    "pub struct UiDocumentFrameCursor",
    "UiDocumentFramePhase::Ingress",
    "UiDocumentFramePhase::Layout",
    "UiDocumentFramePhase::Paint",
    "engine.apply_document_page",
    "drive_mounted_layout_text_one",
    "engine.frame_into_step",
  ], "P5a document consumer does not retain ingress/layout/node progress");
  for (const forbidden of ["engine.frame(", "composite_retained_draw_list(", "dispatch_pointer_events("])
    if (documentBoundary.includes(forbidden)) failures.push(`P5a live document consumer reaches whole work ${forbidden}`);
  requireAll(uiFrameBoundary, [
    "RETAINED_PAINT_DEPTH_CREDITS: usize = 64",
    "visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS]",
    "enum RetainedPaintWalkStep",
    "pub fn frame_into_step",
    "sync_interactive_state_node_step",
    "sync_node: Option<",
    "node_sync: RetainedInteractiveSyncCursor",
    "frame.node_sync.close_step()",
    "RetainedInteractiveSyncStep::Pending",
    "RetainedInteractiveSyncStep::Complete",
    "RetainedInteractiveSyncStep::Fault",
    "paint_node_step",
    "paint_node: Option<(",
    "node_paint: RetainedNodePaintCursor",
    "scene_node: Option<(",
    "scene_paint: ScenePaintCursor",
    "host.paint_slot_step",
    "scene_slot_for_node",
    "RetainedPaintPhase::Publish",
  ], "P5a retained UI frame does not advance one fixed visit/node/scene/publication unit");
  for (const forbidden of ["paint_tree(", "sync_interactive_state_node(&mut", "collect_scene_slots(", "window.draw.clear()", "for slot in"])
    if (uiFrameBoundary.includes(forbidden)) failures.push(`P5a live UI frame reaches whole subtree ${forbidden}`);
  requireAll(paint, [
    "RETAINED_SYNC_COLLECTION_ITEMS: usize = 256",
    "RETAINED_SYNC_OUTPUTS: usize = RETAINED_SYNC_COLLECTION_ITEMS * 2",
    "RETAINED_SYNC_DEPTH: usize = 64",
    "RETAINED_SYNC_KEY_BYTES: usize = 256",
  ], "P5a mounted interactive synchronization fixed limits are missing");
  requireAll(interactiveSyncBoundary, [
    "struct RetainedSyncTreeFrame",
    "items_pointer: usize",
    "items_len: usize",
    "tree_frames: [Option<RetainedSyncTreeFrame>; RETAINED_SYNC_DEPTH]",
    "tree_records: [Option<RetainedSyncTreeRecord>; RETAINED_SYNC_OUTPUTS]",
    "pub(crate) fn sync_interactive_state_node_step",
    "RetainedInteractiveSyncPhase::SelectItem",
    "RetainedInteractiveSyncPhase::SelectScan",
    "RetainedInteractiveSyncPhase::SelectWrite",
    "RetainedInteractiveSyncPhase::TreeItem",
    "RetainedInteractiveSyncPhase::TreeApplyScan",
    "RetainedInteractiveSyncPhase::TreeApplyWrite",
    "RetainedInteractiveSyncPhase::TreeClose",
    "pub(crate) fn close_step(&mut self) -> bool",
  ], "P5a mounted interactive synchronization lacks fixed retained item/depth/output ownership");
  for (const forbidden of ["Vec<", ".clone()", ".collect::<Vec", "tree.children(id)", "sync_tree_row_layout", "sync_tree_item_layout", "for index in", "for child in", "while ", "loop {"])
    if (interactiveSyncBoundary?.includes(forbidden)) failures.push(`P5a mounted interactive synchronization retains whole collection/depth work ${forbidden}`);
}
