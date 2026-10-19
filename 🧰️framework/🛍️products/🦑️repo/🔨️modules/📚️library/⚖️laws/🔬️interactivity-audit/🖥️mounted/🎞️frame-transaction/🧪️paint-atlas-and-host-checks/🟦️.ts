/** 🧪️ The mounted frame transaction checks of the paint, scene, atlas, frame job and host owners. */
/** 🖌 Checks the paint node, interactive sync and scene slot boundaries. */
export function checkPaintSyncAndScene({ requireAll, paint, paintNodeBoundary, failures, paintSource, glueSource, sceneNodeBoundary, sceneSlotsSource, sceneHostBoundary, retainedSceneBoundary, retainedImageBoundary }: {
  readonly requireAll: (source: string | undefined, needles: readonly string[], label: string) => void;
  readonly paint: string;
  readonly paintNodeBoundary: string | undefined;
  readonly failures: string[];
  readonly paintSource: string;
  readonly glueSource: string;
  readonly sceneNodeBoundary: string | undefined;
  readonly sceneSlotsSource: string;
  readonly sceneHostBoundary: string;
  readonly retainedSceneBoundary: string;
  readonly retainedImageBoundary: string;
}) {
  requireAll(paint, ["RETAINED_NODE_TEXT_MAX_BYTES: usize = 4 * 1024 * 1024"], "P5a retained node text bound is missing");
  requireAll(paintNodeBoundary, [
    "pub(crate) struct RetainedNodePaintCursor",
    "glyph: RetainedGlyphCursor",
    "phase: u16",
    "item: usize",
    "path: [usize; RETAINED_TREE_DEPTH]",
    "byte: usize",
    "line: usize",
    "pen_x: f32",
    "pub(crate) fn paint_node_step",
    "draw.begin_retained_output(strikes, strikes * size_of::<crate::wgpu::draw::UiInstance>())",
    "draw.finish_retained_output()",
    "value[cursor.byte..].chars().next()",
    "cursor.byte = next_byte",
    "atlas.ensure_glyph(ch, size)",
    "cursor.line.checked_add(1)",
    "match &node.spec.0",
    "UiNode::Select(select)",
    "UiNode::KeyValue(key_value)",
    "UiNode::Tree(tree_node) => retained_tree_node_step",
    "fn retained_tree_node_step",
    "retained_control_is_bounded",
  ], "P5a paint child authority is not retained by byte, glyph, line, output credit, and close state");
  for (const forbidden of ["wrap_text(", ".collect::<Vec", "for (index, line)", "for ch in", "for character in", "paint_stack(tree", "paint_node(tree", "paint_node_self(", "paint_control(", "for child in", "loop {", "while "])
    if (paintNodeBoundary?.includes(forbidden)) failures.push(`P5a paint node recursively reaches child work ${forbidden}`);
  requireAll(paintSource, [
    "retained_text_paint_emits_at_most_one_glyph_per_grant",
    "retained_multi_megabyte_input_advances_one_scalar_per_grant",
    "retained_select_max_plus_one_refuses_before_output_without_moving_tree_owner",
    "retained_select_sync_writes_at_most_one_row_per_grant",
    "retained_select_sync_max_plus_one_fault_closes_exact_cursor_owner",
    "retained_tree_sync_abandonment_releases_one_record_or_depth_owner_per_grant",
    "retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity",
    "retained_text_cancel_close_preserves_exact_terminal_witness",
  ], "P5a retained node text/non-text one-scalar, collection MAX + 1, identity, or close law is missing");
  requireAll(glueSource, [
    "frame_deferred_cancel_token_closes_one_exact_owner_per_grant",
    "frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation",
    "frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking",
    "frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant",
    "accepted_frame_maintenance_queue_drop_publishes_exact_generation_owner",
    "interrupted_frame_maintenance_execution_restores_before_incremental_recovery",
  ], "P5a maintenance cancel/ABA/refusal exact-owner laws are missing");
  requireAll(sceneNodeBoundary, [
    "pub struct ScenePaintCursor",
    "node: Option<NodeId>",
    "phase: u16",
    "item: usize",
    "page: usize",
    "byte: usize",
    "pub fn advance_byte(&mut self) -> Result<(), ScenePaintCursorError>",
    "fn paint_slot_step",
    "pub(crate) fn scene_slot_for_node",
    "UiNode::ComponentScene",
    "UiNode::Image",
  ], "P5a scene child authority is not one-node and producer-cursor addressable");
  if (sceneNodeBoundary?.includes("collect_scene_slots(")) failures.push("P5a one-node scene authority reaches whole scene collection");
  requireAll(sceneSlotsSource, ["scene_paint_cursor_rejects_stale_node_without_consuming_owner", "scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner"], "P5a scene stale-owner, scalar, or close law is missing");
  requireAll(sceneHostBoundary, ["fn paint_slot_step", "cursor.bind(slot.node)", "render_component_scene_step", "render_ui_image_step"], "P5a Framework scene host does not resume exact retained scene/image consumers");
  for (const forbidden of ["render_component_scene(", "render_ui_image(", "fn paint_slot("])
    if (sceneHostBoundary.includes(forbidden)) failures.push(`P5a Framework scene host reaches complete leaf renderer ${forbidden}`);
  requireAll(retainedSceneBoundary, ["cursor.phase()", "cursor.byte()", "cursor.advance_byte()", "try_reserve_retained_items(1)", "cursor.advance_item()", "cursor.finish()"], "P5a component scene consumer is not retained by scalar/item phase");
  requireAll(retainedImageBoundary, ["cursor.phase()", "cursor.byte()", "cursor.advance_byte()", "queue_ui_image_url_fetch", "try_reserve_retained_items(1)", "cursor.advance_item()", "cursor.finish()"], "P5a image consumer is not retained by scalar, asset request, and output item phase");
  if ((retainedSceneBoundary.match(/cursor\.advance_byte\(\)/g)?.length ?? 0) !== 2 || (retainedImageBoundary.match(/cursor\.advance_byte\(\)/g)?.length ?? 0) !== 2)
    failures.push("P5a component scene and image consumers must each retain both scalar byte phases");
  for (const forbidden of ["resolve_ui_image(", "resolve_ui_image_svg(", "queue_canvas_image_upload_sized("])
    if (retainedImageBoundary.includes(forbidden)) failures.push(`P5a retained image consumer reaches complete decode ${forbidden}`);
}

/** 🗺 Checks the atlas page credit and the prepared page boundaries. */
export function checkAtlasAndPreparedPages({ requireAll, atlasBoundary, failures, preparedSource, atlasGpuBoundary, atlasDrawBoundary, glue, shell, engineBoundary, worldBoundary, transaction }: {
  readonly requireAll: (source: string | undefined, needles: readonly string[], label: string) => void;
  readonly atlasBoundary: string;
  readonly failures: string[];
  readonly preparedSource: string;
  readonly atlasGpuBoundary: string;
  readonly atlasDrawBoundary: string;
  readonly glue: string;
  readonly shell: string;
  readonly engineBoundary: string;
  readonly worldBoundary: string;
  readonly transaction: string;
}) {
  requireAll(atlasBoundary, [
    "PREPARED_ATLAS_PAGE_BYTES: usize = 16 * 1024",
    "PREPARED_ATLAS_PAGE_CAPACITY: usize = 2_048",
    "PREPARED_ATLAS_PROCESS_ITEMS: usize = 64",
    "PREPARED_ATLAS_PROCESS_PAGES: usize = 4_096",
    "PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES: usize = 64 * 1024 * 1024",
    "PREPARED_ATLAS_PROCESS_BACKING_BYTES: usize = 96 * 1024 * 1024",
    "static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64",
    "prepared_atlas_field(current, PREPARED_ATLAS_ITEM_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_PAGE_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_PAYLOAD_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_BACKING_SHIFT",
    "struct PreparedAtlasPermit",
    "items: usize",
    "pages: usize",
    "payload_bytes: usize",
    "backing_bytes: usize",
    "PREPARED_ATLAS_PROCESS_PERMITS.compare_exchange(current, next",
    "slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>",
    "let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)",
    "Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])",
    "Box::new([0; PREPARED_ATLAS_PAGE_BYTES])",
    "pub fn close_step(&mut self) -> bool",
    "slots[index] = None",
    "permit.release_step()",
    "pub fn close_abandoned_step() -> bool",
    "impl Drop for PreparedAtlasPages",
    "PreparedAtlasAbandonment",
  ], "P5a atlas authority lacks nonblocking exact item/page/payload/backing permits or retained abandonment close");
  const atlasCredit = atlasBoundary.indexOf("let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)");
  const atlasSlots = atlasBoundary.indexOf("Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])");
  const atlasPage = atlasBoundary.indexOf("Box::new([0; PREPARED_ATLAS_PAGE_BYTES])");
  if (atlasCredit < 0 || atlasSlots < atlasCredit || atlasPage < atlasCredit) failures.push("P5a atlas backing allocates before process credit transfer");
  for (const forbidden of ["PREPARED_ATLAS_PROCESS_LEDGER", "Mutex<usize>", ".lock()", "Vec<PreparedAtlasPage>", "Vec::with_capacity(byte_len)", "Vec::with_capacity(source.len())", ".truncate(", ".fill(None)", "while ", "loop {"])
    if (atlasBoundary.includes(forbidden)) failures.push(`P5a atlas authority retains whole-backing work ${forbidden}`);
  requireAll(preparedSource, [
    "atlas_process_item_max_plus_one_is_nonblocking_and_recovers_every_permit",
    "abandoned_atlas_schedules_the_same_incremental_close_authority",
    "interrupted_atlas_close_rejoins_the_same_abandonment_authority",
    "atlas_allocation_refusal_preserves_the_packed_permit_ledger",
    "atlas_contended_permit_attempts_are_nonblocking_and_poison_free",
  ], "P5a atlas MAX + 1, allocation refusal, contention, abandonment, or interrupted-close law is missing");
  requireAll(atlasGpuBoundary, ["struct PreparedAtlasUploadCursor", "GlyphAtlasPages", "IconAtlasPages", "upload_glyph_atlas_page", "upload_icon_atlas_page", "cursor.page.checked_add(1)"], "P5a GPU atlas upload is not one retained page per grant");
  if ((atlasGpuBoundary.match(/cursor\.page\.checked_add\(1\)/g)?.length ?? 0) !== 2) failures.push("P5a glyph and icon atlas consumers must each advance exactly one page per grant");
  requireAll(atlasDrawBoundary, ["upload_glyph_atlas_page", "upload_icon_atlas_page", "origin: wgpu::Origin3d", "rows_per_image: Some(rows)"], "P5a atlas page upload boundary is incomplete");
  for (const forbidden of ["Vec::with_capacity(self.icons.pixels.len())", "Vec::with_capacity(self.atlas.pixels.len())", "keys().next().cloned()"])
    if (glue.includes(forbidden) || shell.includes(forbidden)) failures.push(`P5a mounted child authority retains uncredited whole owner ${forbidden}`);
  requireAll(engineBoundary, [
    "ENGINE_CANVAS_FRAME_PACKET_CAPACITY: usize = 256",
    "packets: Box<[Option<EngineCanvasPacket>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>",
    "take_packet_step",
    "Result<Option<EngineCanvasPacket>, EngineCanvasPacket>",
    "terminal_is_empty",
  ], "P5a EngineCanvas packet producer is not fixed, fallible, and one-owner resumable");
  for (const forbidden of ["packets: Vec<EngineCanvasPacket>", "take_packets(", "std::mem::take(&mut self.packets)"])
    if (engineBoundary.includes(forbidden)) failures.push(`P5a EngineCanvas boundary retains dynamic or bulk packet staging ${forbidden}`);
  requireAll(worldBoundary, [
    "WORLD3D_FRAME_RESOURCE_CAPACITY: usize = 256",
    "uploads: Box<[Option<PreparedRenderUpload>; WORLD3D_FRAME_RESOURCE_CAPACITY]>",
    "evictions: Box<[Option<PreparedRenderEviction>; WORLD3D_FRAME_RESOURCE_CAPACITY]>",
    "pub fn append_step",
    "Result<bool, World3dBuildRejected>",
    "checked_add(input.uploads.len())",
    "pub fn terminal_is_empty",
  ], "P5a World3D resource producer is not fixed, fallible, and one-owner resumable");
  for (const forbidden of ["uploads: Vec<", "evictions: Vec<", "mesh_requests: HashSet", "raster_requests: HashSet", "append_to(", ".append(", ".clear()"])
    if (worldBoundary.includes(forbidden)) failures.push(`P5a World3D boundary retains dynamic or bulk resource staging ${forbidden}`);
  requireAll(transaction, [
    "pub(crate) fn close_step(&mut self) -> bool",
    "pub(crate) fn terminal_is_empty(&self) -> bool",
  ], "P5a retained close and terminal witness are incomplete");
}

/** ⏱ Checks the frame job, host storm and snapshot revision boundaries. */
export function checkFrameJobAndHostStorm({ requireAll, frameJob, failures, host, hostSource, snapshot, snapshotSource, uiRuntimeGlue }: {
  readonly requireAll: (source: string | undefined, needles: readonly string[], label: string) => void;
  readonly frameJob: string;
  readonly failures: string[];
  readonly host: string;
  readonly hostSource: string;
  readonly snapshot: string;
  readonly snapshotSource: string;
  readonly uiRuntimeGlue: string;
}) {
  requireAll(frameJob, [
    "WorkerJobSession::try_admit_owned(&mut self.original_job,&mut self.original_params,cx)",
    "FrameTransaction::new",
    "session.try_step_on_original_caller(cx)",
    "session.take_terminal()",
    "session.take_outcome(ticket)",
    "owner.take_retained_step_receipt()",
    "owner.acknowledge_outcome(",
    "session.has_original_cancel_alias_witness(cx.original_cancel_token())",
    "session.return_original_cancel_alias_step(cx.original_cancel_token(),grant)",
    "session.close_step(cx)",
    "transaction.close_step(cx)",
    "generation_is_fresh(presentation_generation,job.presentation_generation)",
  ], "P5a mounted original-caller admission/take/acknowledge/close/terminal laws are incomplete");
  if ((frameJob.match(/session\.take_rejected\(\)/g)?.length ?? 0) !== 0 || (frameJob.match(/session\.take_terminal\(\)/g)?.length ?? 0) !== 1 || (frameJob.match(/session\.take_outcome\(ticket\)/g)?.length ?? 0) !== 1)
    failures.push("P5a mounted frame owner must admit through try_admit_owned and keep exactly one take-terminal and one take-outcome receipt protocol");
  for (const forbidden of ["try_step_on_caller", "WorkerPool::new(", "thread::spawn", "loop {", "WorkerJobSession::try_new", "WorkerJobSessionAdmissionRejected", "close_step(self.retained)", "poll_runtime_and_resubmit(&mut self"]) if (frameJob.includes(forbidden)) failures.push(`P5a mounted worker path contains forbidden ${forbidden}`);
  requireAll(host, [
    "fn advance_frame_generation",
    "generation.checked_add(1)",
    "observe_presentation_input_generation(build_generation.0)",
    "AppPresentStep::Complete { generation, cursor, fullscreen, cursor_wake }",
    "if generation.0 != self.frame_generation",
    "self.snapshot_sink.publish",
  ], "P5a host does not fail closed on exhaustion/stale publication or atomically publish the accepted frame");
  requireAll(hostSource, ["mounted_pointer_storm_callback_p99_stays_below_two_milliseconds", "mounted_resize_storm_callback_p99_stays_below_two_milliseconds", "mounted_frame_generation_exhaustion_is_permanent_and_non_wrapping"], "P5a mounted input/resize/latency/exhaustion laws are missing");
  if (host.includes("frame_generation.wrapping_add") || host.includes("snapshot_sink.publish(crate::render_snapshot::RenderSnapshot::new(revision, generation, timestamp_us, cursor")) failures.push("P5a host retains wrapping generation or callback-local snapshot publication");
  requireAll(snapshot, ["pub fn next_revision(&self) -> Option<u64>", "current.checked_add(1)?", "compare_exchange(current, next"], "P5a snapshot revision does not permanently fail closed at exhaustion");
  if (!snapshotSource.includes("revision_exhaustion_is_permanent_and_preserves_last_valid_snapshot")) failures.push("P5a last-valid snapshot exhaustion law is missing");
  if (uiRuntimeGlue.includes("mod transaction") || uiRuntimeGlue.includes("pub use transaction::*")) failures.push("P5a dormant headless transaction is production reachable beside the mounted authority");
}
