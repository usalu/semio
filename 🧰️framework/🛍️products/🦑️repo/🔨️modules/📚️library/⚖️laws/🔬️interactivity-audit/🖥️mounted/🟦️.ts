import { interactivityProductionSource } from "../🗄️database/🟦️.ts";
import { buildReconcileState } from "./🔶️reconcile/🟦️.ts";
import { buildPart2 } from "./🔸️part2/🟦️.ts";
import { buildReserveState } from "./🟠️reserve/🟦️.ts";
import { buildPart4 } from "./🟢️part4/🟦️.ts";

export function interactivityLiveReconcileFailures(reconcileSource: string, patchesSource: string, reactorSource: string, valueSource: string, schemaSource = ""): string[] {
  const { schema, reactor, failures, reconcile, turnPatchBridgeStart, turnPatchBridge, patches, driveStart, drive, trackerDefinitions, rendererRetainedStart, rendererRetained, shellDocumentStart, shellDocuments, shellRefresh, commandDocumentStart, commandExchangeStart, commandDocuments, commandExchange, rendererSequenceStart, rendererSequence, windowMeasureStart, windowMeasure } = buildReconcileState({ reconcileSource, patchesSource, reactorSource, valueSource, schemaSource });
  buildPart2({ schema, reactor, schemaSource, failures, reconcile, turnPatchBridgeStart, turnPatchBridge });
  buildReserveState({ schemaSource, failures, valueSource, reconcile, patches, reactor, driveStart, drive, trackerDefinitions });
  buildPart4({ rendererRetainedStart, rendererRetained, schema, reactor, schemaSource, failures, shellDocumentStart, shellDocuments, shellRefresh, commandDocumentStart, commandExchangeStart, commandDocuments, commandExchange, rendererSequenceStart, rendererSequence, windowMeasureStart, reactorSource, windowMeasure, patchesSource, reconcileSource });
  return failures;
}

export function interactivityMountedLayoutTextFailures(mountedSource: string, engineSource: string, treeSource: string, paintSource: string, eventsSource: string, slotsSource: string, interpreterSource: string, glueSource: string): string[] {
  const mounted = interactivityProductionSource(mountedSource);
  const engine = interactivityProductionSource(engineSource);
  const tree = interactivityProductionSource(treeSource);
  const paint = interactivityProductionSource(paintSource);
  const events = interactivityProductionSource(eventsSource);
  const slots = interactivityProductionSource(slotsSource);
  const interpreter = interactivityProductionSource(interpreterSource);
  const glue = interactivityProductionSource(glueSource);
  const failures: string[] = [];
  const requireAll = (source: string, needles: readonly string[], label: string) => {
    if (needles.some((needle) => !source.includes(needle))) failures.push(label);
  };
  const count = (source: string, needle: string) => source.split(needle).length - 1;
  const operation = (source: string, start: string, end: string) => {
    const from = source.indexOf(start);
    const until = source.indexOf(end, from + start.length);
    return from < 0 ? "" : source.slice(from, until < 0 ? source.length : until);
  };
  const law = (source: string, name: string, evidence: readonly string[]) => {
    const start = source.indexOf(`fn ${name}`);
    const end = source.indexOf("\n    #[test]", start + name.length);
    const body = start < 0 ? "" : source.slice(start, end < 0 ? source.length : end);
    if (evidence.some((needle) => !body.includes(needle))) failures.push(`P5c hostile law ${name} is missing production-path evidence`);
  };

  requireAll(mounted, [
    "LAYOUT_NODE_CREDITS: usize = 4_096",
    "LAYOUT_GLYPH_CREDITS: usize = 16_384",
    "LAYOUT_DEPTH_CREDITS: usize = 64",
    "LAYOUT_ATLAS_PAGE_CREDITS: usize = 4",
    "LAYOUT_ATLAS_PAGE_BYTES: usize = 16 * 1024",
  ], "P5c fixed node/glyph/depth/atlas credits are missing or changed");
  requireAll(mounted, [
    "walk: Box<ui_contract::UiFixedList<WalkFrame",
    "nodes: Box<ui_contract::UiFixedList<LayoutInputNode",
    "glyphs: Box<ui_contract::UiFixedList<RetainedGlyphInput",
    "runs: Box<ui_contract::UiFixedList<RetainedTextRun",
    "lines: Box<ui_contract::UiFixedList<RetainedLine",
    "glyph_previews: Box<ui_contract::UiFixedList<RetainedGlyphPreview",
    "results: Box<ui_contract::UiFixedList<MountedLayoutResult",
    "pages: [Option<Box<[u8; LAYOUT_ATLAS_PAGE_BYTES]>>; LAYOUT_ATLAS_PAGE_CREDITS]",
  ], "P5c production working sets are not the actual fixed admitted owners");
  requireAll(mounted, ["text_byte: usize", "text_glyph_start: usize", "run_cursor: usize", "glyph_cursor: usize", "line_cursor: usize", "measure_cursor: usize", "arrange_cursor: usize", "preview_cursor: usize", "publish_cursor: usize"], "P5c retained node/run/glyph/line/preview/publication cursors are incomplete");
  for (const forbidden of ["nodes: Vec<", "glyphs: Vec<", "runs: Vec<", "lines: Vec<", "glyph_previews: Vec<", "results: Vec<", "walk: Vec<", "pages: Vec<", "HashMap<", "VecDeque", "FontAtlas", ".measure(", "std::mem::forget", "wrapping_add", "loop {", "while !job"]) if (mounted.includes(forbidden)) failures.push(`P5c mounted production source contains forbidden ${forbidden}`);
  if (!mounted.includes("child_scratch: Vec<usize>")) failures.push("P5c mounted production source lacks bounded child-index scratch reuse");
  const admit = operation(mounted, "pub(crate) fn admit_one", "fn admit_node_one");
  const worker = operation(mounted, "fn worker_one", "fn shape_one");
  const shape = operation(mounted, "fn shape_one", "fn measure_one");
  const publish = operation(mounted, "pub(crate) fn publish_one", "fn stage_label");
  const close = operation(mounted, "pub(crate) fn close_one", "pub(crate) fn terminal_is_empty");
  requireAll(admit, ["cx.is_cancelled()", "cx.should_yield()", "cx.consume_fuel(1)", "match self.admission"], "P5c admission is not one cancel/deadline/fuel-bounded cursor opportunity");
  requireAll(worker, ["if self.close_requested || cx.is_cancelled()", "cx.set_stage(self.stage_label())", "match self.stage {", "cx.consume_fuel(1)", "cx.deadline_exceeded()"], "P5c worker turn does not prove pool-thread and bounded before/after checks");
  if (count(worker, "cx.is_cancelled()") < 2) failures.push("P5c opaque text worker call is not cancellation-checked before and after");
  requireAll(shape, ["self.runs.get(self.run_cursor)", "self.text_worker.shape_one(input)", "self.atlas_candidate.retain_one", "self.glyph_cursor += 1"], "P5c text shaping is not one retained run/glyph/atlas unit");
  requireAll(mounted, ["trait OwnedTextWorker: Send", "generation: u64", "revision: u64", "atlas_page: u8", "atlas_offset: u16", "atlas_length: u8"], "P5c external text boundary or generation-tagged atlas output is missing");
  requireAll(publish, ["self.identity() != identity", "self.results.len() != self.nodes.len()", "tree.write_inactive_layout", "tree.commit_inactive_layout(generation)", "self.publication_committed = true"], "P5c publication is not complete, fresh, inactive, and one-swap-only");
  requireAll(close, ["rejected_result.take()", "rejected_glyph.take()", "self.results.pop()", "self.atlas_candidate.close_one()"], "P5c close does not release exactly one retained/rejected owner or atlas page");

  requireAll(engine, [
    "slots: Box<[Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]>",
    "generations: [u64; UI_LAYOUT_SURFACE_SLOTS]",
    "slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS]",
    "window_id: SurfaceId",
    "layout_queues: [SurfaceLaneRing; 3]",
  ], "P5c multi-surface authority or lane queues are not fixed token storage");
  for (const forbidden of ["VecDeque", "HashMap<String, UiWindow>", "wrapping_add", "job.worker_one", "try_step_on_caller", "WorkerPool::new(", "window.layout_session = None; session.close_step"]) if (engine.includes(forbidden)) failures.push(`P5c engine production path contains forbidden ${forbidden}`);
  const scheduler = operation(engine, "pub fn step_layouts", "fn enqueue_layout");
  requireAll(scheduler, ["WorkerJobPoll::CheckedOut", "session.pump_one(pool, worker_lane(lane), cx.retained_grant())", "cx.cancel_token()", "close_layout_worker_session(session", "job.identity() != identity", "MountedWorkerJobSession::try_admit_owned(&mut window.layout_job, &mut window.layout_admission.params, cx)"], "P5c engine does not retain exact mounted take/resume/close authority on the shared pool");
  if (scheduler.includes("MountedWorkerJobSession::try_new")) failures.push("P5c engine admits its mounted layout session through the removed fallible constructor");
  requireAll(engine, ["theme_layout_identity(&self.theme) != theme_layout_identity(&theme)", "window.theme_revision == u64::MAX", "window.viewport_revision.checked_add(1)", "window.layout_generation.checked_add(1)", "window.revision.checked_add(1)"], "P5c equality or non-wrapping theme/tree/viewport generations are incomplete");
  requireAll(engine, ["layout_preview: Option<MountedLayoutResult>", "glyph_preview: Option<RetainedGlyphPreview>", "MountedLayoutJob::take_preview_one", "job.latest_glyph_preview()", "window.layout_preview = Some(preview)", "preview.generation == window.layout_generation", "preview.revision == window.revision", "window.glyph_preview = Some(preview)"], "P5c progressive geometry/text preview retention is missing or stale-unchecked");

  requireAll(tree, ["mounted_layout: [MountedLayoutRecord; 2]", "mounted_layout_active: usize", "mounted_layout_generation: u64", "write_inactive_layout", "self.mounted_layout_active ^= 1", "mounted.generation == self.mounted_layout_generation", "self.mounted_layout_generation = generation"], "P5c tree lacks inactive/active O(1) accepted snapshot authority");
  const retainedPaint = operation(paint, "pub(crate) fn paint_node_step(", "pub(crate) fn paint_tree");
  const sceneSlotNode = operation(slots, "pub(crate) fn scene_slot_for_node", "pub(crate) fn collect_scene_slots");
  if (!retainedPaint.includes("let Some(layout) = tree.accepted_layout(id) else { return RetainedNodePaintStep::Fault }") || !operation(events, "fn hit_test_node", "fn is_plain_stack_container").includes("let layout = tree.accepted_layout(id)?") || !sceneSlotNode.includes("let layout = tree.accepted_layout(id)?")) failures.push("P5c retained paint, hit testing, and scene slots do not consume the same accepted snapshot");

  const driver = operation(interpreter, "fn drive_mounted_layout_text_one", "//#region 📄️RetainedDocumentConsumer");
  requireAll(driver, ["StepBudget::from_duration(1, now, 1000)", "crate::renderer_worker_pool()", "engine.step_layouts(&pool", "pool.pump(now / 1_000)"], "P5c mounted renderer does not drive one shared-pool opportunity");
  const documentRoute = operation(interpreter, "pub(crate) fn render_ui_document_step", "fn render_ui_image_step");
  const viewportArm = documentRoute.indexOf("UiDocumentFramePhase::Viewport");
  const layoutArm = documentRoute.indexOf("UiDocumentFramePhase::Layout");
  const paintArm = documentRoute.indexOf("UiDocumentFramePhase::Paint");
  const driveAt = documentRoute.indexOf("drive_mounted_layout_text_one");
  const frameAt = documentRoute.indexOf("frame_into_step", paintArm);
  if (!(viewportArm >= 0 && layoutArm > viewportArm && paintArm > layoutArm && driveAt > layoutArm && frameAt > paintArm && frameAt > driveAt && documentRoute.includes("engine.set_viewport"))) failures.push("P5c renderer production route does not prioritize fresh input then layout drive before frame");
  const poolGlue = operation(glue, "//#region 🧵️RendererWorkerPool", "//#endregion 🧵️RendererWorkerPool");
  requireAll(poolGlue, ["pub(crate) fn renderer_worker_pool()", "process_worker_pool", "ProcessKind::InteractiveNative"], "P5c renderer is not using the one process WorkerPool on native and Wasm-shaped routes");
  if (poolGlue.includes("WorkerPool::new")) failures.push("P5c renderer introduced a second layout scheduler");

  law(mountedSource, "mounted_layout_actual_glyph_max_plus_one_retains_the_exact_rejected_scalar", ["job.rejected_glyph().map", "Some('x')", "while !job.close_one()", "job.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_actual_node_max_plus_one_retains_the_exact_rejected_tree_owner", ["tree.contains(rejected.id)", "!job.nodes.iter().any", "while !job.close_one()"]);
  law(mountedSource, "mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn", ["job.glyph_cursor - before <= 1", "LAYOUT_ATLAS_PAGE_CREDITS - 1", "preview.generation == 11"]);
  law(mountedSource, "mounted_layout_deep_tree_depth_refusal_retains_walk_authority_for_close", ["job.rejected_walk.is_some()", "retained - after, 1", "job.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_worker_runs_on_shared_user_visible_lane_and_pool_thread", ["session.pump_one(&pool, lane)", "Lane::UserVisible", "owner.worker_thread_observed()", "session.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained", ["after.cancel_after_shape", "StepOutcome::Cancelled", "after.glyph_cursor, 1", "after.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner", ["StepBudget::new(1, 0)", "retained - after_one, 1", "job.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once", ["LayoutJobStep::Fault(\"layout.stale\")", "tree.accepted_layout_generation(), before", "tree.accepted_layout_generation(), 11"]);
  law(engineSource, "mounted_layout_surface_max_plus_one_returns_exact_owner_without_mutation", ["rejected.id, owner", "ui.windows.ids().cloned().collect"]);
  law(engineSource, "mounted_layout_equal_theme_does_not_invalidate_or_requeue", [
    "assert_eq!(ui.windows.get(\"theme\").map(|window| window.layout_generation), before_generation)",
    "assert_eq!(ui.windows.get(\"theme\").map(|window| window.theme_revision), before_theme_revision)",
    "assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), before_queue)",
  ]);
  law(engineSource, "mounted_layout_atomic_snapshot_keeps_last_valid_geometry_until_fresh_swap", ["after, old_generation", "old_layout", "swaps, 1", "progressive_glyph_preview"]);
  law(engineSource, "mounted_layout_revision_max_refuses_theme_tree_and_viewport_without_alias", ["theme_revision = u64::MAX", "viewport_revision = u64::MAX", "revision = u64::MAX"]);
  law(engineSource, "mounted_layout_replay_and_resize_supersede_are_deterministic", ["assert_eq!(first, second)", "second.width, 640.0"]);
  law(engineSource, "large_layout_and_shaping_job_admits_one_work_unit_per_slice", ["slices > 10_000", "assert_eq!(widest_slice, (1, 1)", "accepted_layout"]);
  law(engineSource, "interactive_storm_does_not_starve_background_surface_lane", ["slice < LANE_WHEEL.len()", "window_id.as_ref() == \"background\""]);
  return failures;
}
