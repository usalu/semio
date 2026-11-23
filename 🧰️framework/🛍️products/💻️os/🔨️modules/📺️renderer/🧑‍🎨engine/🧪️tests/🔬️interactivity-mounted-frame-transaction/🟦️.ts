import { INTERACTIVITY_AUDIT_FILE_MARKER, INTERACTIVITY_AUDIT_SHELL_FILE, INTERACTIVITY_AUDIT_FIND_ITEMS_FILE, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_WORLD3D_FILES, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_OS_SERVICES_FILE, interactivityMountedFrameTransactionFailures } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import { policyReadFileSafe } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { policyModuleTreeFiles } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { listSourceFamily } from "../../../../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";
import { relative, sep } from "node:path";
import { POLICY_RUST_TEST_EVIDENCE_BOUNDARY } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/** 📖️ The seventeen checked owner sources as the one text each unsplit module held: production first, then the declared test crates. */
export function interactivityMountedFrameSources(repoRoot: string): Parameters<typeof interactivityMountedFrameTransactionFailures> {
  const text = (file: string): string => `${INTERACTIVITY_AUDIT_FILE_MARKER}${file}\n${policyReadFileSafe(repoRoot, file)}`;
  const production = (...files: string[]): string => [...new Set(files.flatMap((file) => policyModuleTreeFiles(repoRoot, file, { crates: true })))].map(text).join("\n");
  const testCorpus = (scope: string): string => listSourceFamily(`${repoRoot}/${scope}`, { tests: true })
    .map((absolute) => relative(repoRoot, absolute).split(sep).join("/"))
    .filter((file) => file.split("/").some((segment) => segment.includes("tests") || segment.includes("testing")))
    .map((file) => policyReadFileSafe(repoRoot, file))
    .join("\n");
  const engineTests = testCorpus("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
  const uiTests = testCorpus("🧰️framework/🔨️modules/🖱️ui");
  const worldTests = testCorpus("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world");
  const servicesTests = testCorpus("🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services");
  const readWith = (tests: string, ...files: string[]): string => production(...files) + POLICY_RUST_TEST_EVIDENCE_BOUNDARY + tests;
  const family = (directory: string): string[] => listSourceFamily(`${repoRoot}/${directory}`)
    .map((absolute) => relative(repoRoot, absolute).split(sep).join("/"))
    .filter((file) => !file.split("/").some((segment) => segment.includes("fixture")))
    .map(text);
  const readFamily = (tests: string, ...directories: string[]): string => directories.flatMap(family).join("\n") + POLICY_RUST_TEST_EVIDENCE_BOUNDARY + tests;
  const uiWgpu = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu";
  return [
    readWith(engineTests, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs", '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑\u200d🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs', '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑\u200d🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs', '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑\u200d🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs', "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"),
    readWith(engineTests, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs"),
    readWith(engineTests, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs"),
    readWith(engineTests, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs"),
    readWith(uiTests, "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️.rs"),
    readWith(engineTests, INTERACTIVITY_AUDIT_SHELL_FILE, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪧️shell-view-chrome-overlays/🦀️.rs", INTERACTIVITY_AUDIT_FIND_ITEMS_FILE),
    readWith(engineTests, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE),
    readWith(worldTests, ...INTERACTIVITY_AUDIT_WORLD3D_FILES),
    readFamily(uiTests, `${uiWgpu}/🎟️prepared`),
    readFamily(uiTests, `${uiWgpu}/🖍️draw`),
    readFamily(uiTests, `${uiWgpu}/🖍️draw`),
    readWith(engineTests, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"),
    readFamily(uiTests, `${uiWgpu}/⚙️engine`),
    readFamily(uiTests, `${uiWgpu}/🖌️paint`),
    readFamily(uiTests, `${uiWgpu}/📨️scene_slots`),
    readWith(engineTests, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"),
    readWith(servicesTests, INTERACTIVITY_AUDIT_OS_SERVICES_FILE),
  ];
}

/** 🧪️ Executes interactivity mounted frame transaction policy assertions. */
/** 🧪️ Executes interactivity mounted frame transaction policy assertions: stale-boundary preflight, the mutation battery measured against the baseline, and the clean baseline. */
export function interactivityMountedFrameTransactionSelfTests(repoRoot: string): void {
  const clean = interactivityMountedFrameSources(repoRoot);
  const navbarScopeMarker = "pub fn render_navbar_step";
  const navbarScopeFailure = "P5a policy boundary is stale: Shell navbar";
  for (const [name, shell] of [
    ["missing", clean[5]!.replace(navbarScopeMarker, "fn stale_render_navbar_step")],
    ["duplicate", `${navbarScopeMarker}() {}\n${clean[5]}`],
  ] as const) {
    const scoped: typeof clean = [...clean];
    scoped[5] = shell;
    const failures = interactivityMountedFrameTransactionFailures(...scoped);
    if (failures.filter((failure) => failure === navbarScopeFailure).length !== 1) throw new Error(`[verify interactivity] P5a ${name} Rust block scope did not emit one named stale-boundary failure`);
  }
  const mutations: [string, number, string, string, string?][] = [
    ["zero-mounted-constructor", 1, "semio_renderer_wgpu_frame_transaction::FrameTransaction::new(", "semio_renderer_wgpu_frame_transaction::UnmountedFrameTransaction::new("],
    ["second-runtime", 1, "if cx.should_yield(){return Ok(None)}", "if cx.should_yield(){return Ok(None)}let _second=semio_framework_async::WorkerPool::new(4);"],
    ["caller-drive", 1, "session.try_step_on_original_caller(cx)", "session.try_step_on_caller(cx)"],
    ["dynamic-actions", 0, "slots: [Option<FrameActionEnvelope>; WORLD3D_DEADLINE_CAPACITY]", "slots: Vec<FrameActionEnvelope>"],
    ["postallocation-credit", 7, "checked_add(input.uploads.len())", "saturating_add(input.uploads.len())"],
    ["wrapping-generation", 2, "generation.checked_add(1)", "Some(generation.wrapping_add(1))"],
    ["stage-fallthrough", 0, "context.set_stage(self.stage_label())", "loop { context.set_stage(self.stage_label())"],
    ["missing-input-freshness", 0, "base_witness != current_witness", "false"],
    ["missing-terminal-witness", 0, "pub fn terminal_is_empty(&self) -> bool {\n        false", "fn removed_terminal_is_empty(&self) -> bool {\n        false"],
    ["missing-worker-close", 1, "transaction.close_step(cx)", "transaction.drop_step(cx)"],
    ["reintroduced-take-rejected", 1, "session.take_outcome(ticket)", "session.take_rejected()"],
    ["missing-resume", 1, "match owner.resume(){Ok(())=>self.ticket=None,Err(owner)=>self.original_outcome=Some(owner)}", "{owner.begin_close();self.ticket=None}"],
    ["missing-terminal-take", 1, "session.take_terminal()", "session.drop_terminal()"],
    ["partial-stale-publish", 2, "if generation.0!=self.frame_generation", "if false"],
    ["snapshot-wrap", 3, "current.checked_add(1)?", "current.wrapping_add(1)"],
    ["missing-effect-budget", 0, "EFFECT_STORM_BUDGET: u32 = 64", "EFFECT_STORM_BUDGET: u32 = u32::MAX"],
    ["missing-max-identity-law", 0, "assert_eq!(rejected.controller_id.as_ptr(), identity)", "assert_eq!(identity, identity)"],
    ["missing-input-storm-law", 2, "mounted_pointer_storm_callback_p99_stays_below_two_milliseconds", "pointer_storm_smoke"],
    ["missing-last-valid-law", 3, "revision_exhaustion_is_permanent_and_preserves_last_valid_snapshot", "revision_exhaustion_smoke"],
    ["dormant-production-authority", 4, "#[cfg(test)]\n#[path = \"../../🔄️transaction/🦀️.rs\"]", "#[path = \"../../🔄️transaction/🦀️.rs\"]"],
    ["opaque-before-callee", 0, "app.frame_before_input_step(handle, directives, cursor, context)", "app.frame_before_input(handle, directives, cursor, context)"],
    ["bulk-draw-clear", 0, "let step = match previous.retire_granted(grant) {", "let step = match { self.draw.clear(); previous.retire_granted(grant) } {"],
    ["select-whole-materialization", 13, "UiNode::Select(select) if node.state.open => {\n                    if select.items.len()", "UiNode::Select(select) if node.state.open => {\n                    let _whole_select = select.items.iter().collect::<Vec<_>>();\n                    if select.items.len()", "P5a mounted interactive synchronization retains whole collection/depth work .collect::<Vec"],
    ["immediate-deferred-drive", 0, "self.pending_frame_deferred = Some", "self.drive_pending_frame_deferred(handle); self.pending_frame_deferred = Some"],
    ["whole-chrome", 0, ".render_chrome_step(&mut cursor.chrome", ".render_chrome(&mut cursor.chrome"],
    ["mounted-navbar-whole-label", 5, "RetainedChromeGroupStep::Fault => self.chrome.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string())", "RetainedChromeGroupStep::Fault => { chrome_text(draw, atlas, input, theme, item.label.unwrap_or_default(), rect.x, rect.y, theme.font_size_small, theme.text); self.chrome.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string()) }"],
    ["whole-main-child", 5, "render_main_window_step(&mut cursor.child", "render_main_window(draw, &mut overlay_slot, atlas, icons, input, theme, body, engine_resources, world_resources); render_main_window_step(&mut cursor.child"],
    ["sync-whole-child-materialization", 13, "pub fn sync_interactive_state_node_step(tree: &mut UiTree, id: NodeId, theme: &Theme, locale: Option<Locale>, cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {", "pub fn sync_interactive_state_node_step(tree: &mut UiTree, id: NodeId, theme: &Theme, locale: Option<Locale>, cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {\n    let _whole_children = tree.children(id).collect::<Vec<_>>();"],
    ["bulk-engine-take", 6, "take_packet_step", "take_packets"],
    ["dynamic-world-uploads", 7, "pub uploads: WorldSlots<PreparedRenderUpload>,", "pub uploads: Vec<PreparedRenderUpload>,"],
    ["bulk-world-append", 7, "pub fn append_step", "pub fn append_to"],
    ["maintenance-authority-release-erasure", 0, "fn release(&self, generation: u64) -> bool {\n        if !self.is_live(generation) {\n            return false;\n        }\n        self.generation.store(0, Ordering::Release);\n        self.state.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_ok()\n    }", "fn release(&self, _generation: u64) -> bool {\n        true\n    }", "P5a maintenance authority is not generation/cancel/deadline-qualified with nonblocking exact-owner refusal handback"],
    ["deferred-run-to-completion", 0, "let Some(work) = cursor_value.take_next() else {", "while cursor_value.take_next().is_some() {}\n        let Some(work) = cursor_value.take_next() else {"],
    ["bulk-deferred-close", 0, "if !self.actions.close_step(cx)? { return Ok(false); }", "if !self.actions.is_empty() { self.actions = FrameActionOwners::default(); return Ok(false); }", "P5a deferred close bulk-drops the populated action owner"],
    ["atlas-credit-after-allocation", 8, "let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes) else {", "let _premature = Box::<[u8; PREPARED_ATLAS_PAGE_BYTES]>::new_uninit(); let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes) else {"],
    ["bulk-atlas-close", 8, "drop(slots[index].take())", "slots.fill(None)"],
    ["two-atlas-pages-per-upload", 9, "cursor.page.checked_add(1)", "cursor.page.checked_add(2)"],
    ["whole-document-frame", 11, "engine.frame_into_step", "engine.frame"],
    ["dynamic-paint-stack", 12, "visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS]", "visits: Vec<RetainedPaintVisit>"],
    ["complete-text-wrap", 13, "let Some(ch) = value[cursor.byte..].chars().next() else", "for ch in value[cursor.byte..].chars() { let _ = atlas.ensure_glyph(ch, size); }\n    let Some(ch) = value[cursor.byte..].chars().next() else"],
    ["whole-nontext-paint", 13, "UiNode::Tree(tree_node) => retained_tree_node_step(tree, id, tree_node, bounds, theme, presentation.driver.drag, block_reversed, inline, presentation.locale, atlas, icons, draw, cursor),", "UiNode::Tree(_) => { paint_node_self(tree, id, origin_x, origin_y, theme, atlas, icons, has_scene_host, draw); RetainedNodePaintStep::Complete },", "P5a paint node recursively reaches child work paint_node_self("],
    ["whole-scene-collection", 14, "UiNode::ComponentScene", "collect_scene_slots(tree, id); UiNode::ComponentScene"],
    ["whole-context-menu", 5, "render_context_menu_levels_step", "render_context_menu"],
    ["whole-tour", 5, "render_chrome_tour_step", "render_chrome_tour"],
    ["whole-shell-glyph-callee", 5, "paint_retained_glyph_step_flowed(text, Rect::new", "for ch in text.chars() { let _ = atlas.ensure_glyph(ch, size); }\n    paint_retained_glyph_step_flowed(text, Rect::new", "P5a Shell text callee retains whole-string work for ch in"],
    ["cloned-cleanup-key", 5, "extract_if(|_, _| true).next()", "keys().next().cloned()"],
    ["dynamic-find-owner", 5, "slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>", "slots: Vec<ShellFindItem>"],
    ["blocking-find-binding", 5, "std::cell::Cell<Option<ActiveShellFindItems>>", "std::sync::Mutex<Option<ActiveShellFindItems>>"],
    ["unowned-find-push", 5, "pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem>", "pub fn try_push_find_item(item: ShellFindItem)"],
    ["whole-find-take", 5, "fn pop_front(&mut self) -> Option<ShellFindItem>", "fn take_find_items(&mut self) -> Option<ShellFindItem>"],
    ["missing-find-max-law", 5, "find_item_max_plus_one_returns_the_exact_owned_item", "find_item_max_plus_one_smoke"],
    ["synchronous-preferences-load", 5, "self.chrome.request_chrome_preferences_load();", "self.chrome.load_ui_prefs_once();"],
    ["synchronous-introduction-read", 5, "self.docs.request_introduction_read(&mut self.chrome);", "self.docs.read_stored_introduction_seen(&mut self.chrome);"],
    ["synchronous-layout-persist", 5, "self.dock.request_panel_layout_persist(&mut self.chrome);", "self.dock.persist_panel_layout_if_changed();"],
    ["synchronous-presence-preview", 5, "self.sync.request_presence_preview(&mut self.chrome);", "self.sync.publish_presence_heartbeat();"],
    ["synchronous-preferences-persist", 5, "self.prefs.request_chrome_preferences_persist(&mut self.chrome);", "self.prefs.persist_ui_prefs_if_changed();"],
    ["maintenance-on-interactive-lane", 0, "renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job)", "renderer_worker_pool().try_submit(semio_framework_async::Lane::Interactive, job)"],
    ["missing-maintenance-cancel", 0, "let cancelled = cursor.cancel.as_ref().is_none_or(|cancel| cancel.is_cancelled_now());", "let cancelled = false;"],
    ["missing-maintenance-stale-witness", 0, "mailbox.0.presentation_authority.witness_for(generation).is_none()", "false"],
    ["maintenance-terminal-keeps-deferred-work", 0, "if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale) {\n                    cursor.begin_close();", "if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale) {\n                    cursor.shell_maintenance = false;"],
    ["discarded-maintenance-submission", 0, "match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job) {", "renderer_worker_pool().submit(semio_framework_async::Lane::Io, job);\n        return Ok(());\n        match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job) {"],
    ["dynamic-preference-page", 16, "let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES];", "let mut page = vec![0u8; STORAGE_FIXED_FILE_PAGE_BYTES];"],
    ["whole-preference-json", 5, "String::from_utf8(page).ok()", "serde_json::from_slice::<String>(&page).ok()"],
    ["ui-bypasses-host-storage-read", 5, "semio_os_services::storage_worker_read_fixed_file_page(&path, SHELL_CHROME_IO_FIELD_BYTES)", "std::fs::read(&path)", "P5a native preference maintenance does not delegate document and field pages to the host-storage service"],
    ["ui-bypasses-host-storage-write", 5, "semio_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), SHELL_CHROME_IO_FIELD_BYTES)", "std::fs::write(&path, value.as_bytes())", "P5a native preference maintenance does not delegate document and field pages to the host-storage service"],
    ["missing-fixed-page-law", 16, "fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page", "fixed_file_page_smoke"],
    ["missing-paint-output-credit", 13, "draw.begin_retained_output(strikes, strikes * size_of::<semio_ui_draw_contract::UiInstance>())", "Ok::<(), ()>(())", "P5a paint child authority is not retained by byte, glyph, line, output credit, and close state"],
    ["missing-paint-byte-cursor", 13, "pub(super) byte: usize,\n    pub(super) line: usize,\n    pen_x: f32,", "pub(super) bytes: Vec<u8>,\n    pub(super) line: usize,\n    pen_x: f32,"],
    ["missing-multimegabyte-text-law", 13, "retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity", "retained_text_large_smoke"],
    ["missing-nontext-large-law", 13, "retained_multi_megabyte_input_advances_one_scalar_per_grant", "retained_large_input_smoke"],
    ["missing-shell-large-law", 5, "dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant", "dialog_and_tour_text_smoke"],
    ["missing-maintenance-drop-handback", 0, "if self.armed && self.registry.abandon(self.generation)", "if false && self.registry.abandon(self.generation)"],
    ["missing-maintenance-terminal-law", 0, "frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant", "frame_maintenance_terminal_smoke"],
    ["whole-component-scene-renderer", 11, "render_component_scene_step(scene, slot.rect, &mut ctx, cursor, &mut hosts, self.driver_drag, self.window_generation)", "render_component_scene(scene, slot.rect, &mut ctx)"],
    ["whole-image-renderer", 11, "render_ui_image_step(image, slot.rect, &mut ctx, cursor)", "render_ui_image(image, slot.rect, &mut ctx)"],
    ["missing-scene-node-owner", 14, "node: Option<NodeId>", "node: NodeId"],
    ["bulk-scene-byte-run", 15, "cursor.advance_byte()", "cursor.advance_byte_run()"],
    ["bulk-image-byte-run", 11, "cursor.advance_byte()", "cursor.advance_byte_run()"],
    ["missing-scene-stale-law", 14, "scene_paint_cursor_rejects_stale_node_without_consuming_owner", "scene_paint_cursor_stale_smoke"],
    ["blocking-atlas-ledger", 8, "static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64", "static PREPARED_ATLAS_PROCESS_PERMITS: Mutex<usize>"],
    ["missing-atlas-backing-dimension", 8, "prepared_atlas_field(current, PREPARED_ATLAS_BACKING_SHIFT", "prepared_atlas_field(current, PREPARED_ATLAS_PAYLOAD_SHIFT"],
    ["missing-atlas-drop-recovery", 8, "impl Drop for PreparedAtlasPages", "impl PreparedAtlasPages"],
    ["unmounted-atlas-abandonment-drain", 0, "PreparedAtlasPages::close_abandoned_step(grant)", "true"],
    ["missing-atlas-interrupted-close-law", 8, "interrupted_atlas_close_rejoins_the_same_abandonment_authority", "interrupted_atlas_close_smoke"],
  ];
  const baseline = interactivityMountedFrameTransactionFailures(...clean);
  const detected = (mutated: typeof clean): string[] => interactivityMountedFrameTransactionFailures(...mutated).filter((failure) => !baseline.includes(failure));
  const mutationFailures: string[] = [];
  for (const [name, index, needle, replacement, expected] of mutations) {
    const mutated: typeof clean = [...clean];
    mutated[index] = mutated[index]!.split(needle).join(replacement);
    if (mutated[index] === clean[index]) mutationFailures.push(`mutation ${name} did not alter source`);
    else if (!name.endsWith("-law") && clean[index]!.indexOf(needle) > clean[index]!.indexOf(POLICY_RUST_TEST_EVIDENCE_BOUNDARY)) mutationFailures.push(`mutation ${name} altered test evidence instead of production source`);
    else {
      const failures = detected(mutated);
      if (expected && !failures.includes(expected)) mutationFailures.push(`mutation ${name} did not emit assigned failure ${expected}`);
      else if (failures.length === 0) mutationFailures.push(`mutation ${name} was falsely accepted`);
    }
  }
  const legacyChromeMutations: [string, string, string][] = [
    ["production-chrome-measure-oracle", "pub fn measure_chrome_group_item", "fn measure_chrome_group_item"],
    ["production-chrome-render-oracle", "pub fn render_chrome_group", "fn render_chrome_group"],
    ["mounted-legacy-chrome-group-restoration", "RetainedChromeGroupStep::Fault => self.chrome.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string())", "RetainedChromeGroupStep::Fault => { render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true); self.chrome.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string()) }"],
  ];
  for (const [name, needle, replacement] of legacyChromeMutations) {
    const mutated: typeof clean = [...clean];
    mutated[5] = mutated[5]!.replace(needle, replacement);
    if (mutated[5] === clean[5]) mutationFailures.push(`mutation ${name} did not alter Shell source`);
    else if (detected(mutated).length === 0) mutationFailures.push(`mutation ${name} was falsely accepted`);
  }
  if (baseline.length !== 0) mutationFailures.push(`mounted frame baseline was falsely rejected: ${baseline.join("; ")}`);
  if (mutationFailures.length !== 0) throw new Error(`[verify interactivity] P5a ${mutationFailures.join("; ")}`);
}

try {
  const { test } = await import("bun:test");
  test("P5a interactivity mounted frame transaction", () => {
    interactivityMountedFrameTransactionSelfTests(process.cwd());
  }, 180_000);
} catch (error) {
  if (!(error instanceof Error) || !error.message.includes("outside of the test runner")) throw error;
}
