import { INTERACTIVITY_AUDIT_SHELL_FILE, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_WORLD3D_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_OS_SERVICES_FILE, policyReadRustPolicySource, interactivityMountedFrameTransactionFailures } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity mounted frame transaction policy assertions. */
export function interactivityMountedFrameTransactionSelfTests(repoRoot: string): void {
  const files = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs",
    "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️.rs",
    INTERACTIVITY_AUDIT_SHELL_FILE,
    INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE,
    INTERACTIVITY_AUDIT_WORLD3D_FILE,
    INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE,
    INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE,
    INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE,
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/⚙️engine.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🖌️paint.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎬️scene_slots.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    INTERACTIVITY_AUDIT_OS_SERVICES_FILE,
  ];
  const clean = files.map((file) => policyReadRustPolicySource(repoRoot, file));
  const mutations: [string, number, string, string][] = [
    ["zero-mounted-constructor", 1, "crate::FrameTransaction::new", "crate::UnmountedFrameTransaction::new"],
    ["second-runtime", 1, "renderer_worker_pool()", "semio_framework_async::WorkerPool::new()"],
    ["caller-drive", 1, "try_submit_step(&crate::renderer_worker_pool(), Lane::Interactive)", "try_step_on_caller()"],
    ["dynamic-actions", 0, "slots: [Option<ActionDescriptor>; WORLD3D_DEADLINE_CAPACITY]", "slots: Vec<ActionDescriptor>"],
    ["postallocation-credit", 0, "checked_add(input.uploads.len())", "saturating_add(input.uploads.len())"],
    ["wrapping-generation", 2, "generation.checked_add(1)", "Some(generation.wrapping_add(1))"],
    ["stage-fallthrough", 0, "context.set_stage(self.stage_label())", "loop { context.set_stage(self.stage_label())"],
    ["missing-input-freshness", 0, "base_witness != current_witness", "false"],
    ["missing-terminal-witness", 0, "pub(crate) fn terminal_is_empty", "fn removed_terminal_is_empty"],
    ["missing-worker-close", 1, "transaction.close_step() && transaction.terminal_is_empty()", "transaction.close_step()"],
    ["missing-take-rejected", 1, "session.take_rejected()", "session.drop_rejected()"],
    ["missing-resume", 1, "rejected.resume()", "rejected.begin_close()"],
    ["missing-terminal-take", 1, "session.take_terminal()", "session.drop_terminal()"],
    ["partial-stale-publish", 2, "if generation.0 != self.frame_generation", "if false"],
    ["snapshot-wrap", 3, "current.checked_add(1)?", "current.wrapping_add(1)"],
    ["missing-effect-budget", 0, "EFFECT_STORM_BUDGET: u32 = 64", "EFFECT_STORM_BUDGET: u32 = u32::MAX"],
    ["missing-max-identity", 0, "assert_eq!(rejected.controller_id.as_ptr(), identity)", "assert_eq!(identity, identity)"],
    ["missing-input-storm-law", 2, "mounted_pointer_storm_callback_p99_stays_below_two_milliseconds", "pointer_storm_smoke"],
    ["missing-last-valid-law", 3, "revision_exhaustion_is_permanent_and_preserves_last_valid_snapshot", "revision_exhaustion_smoke"],
    ["dormant-production-authority", 4, "#[cfg(test)]\n#[path = \"🦀️transaction.rs\"]", "#[path = \"🦀️transaction.rs\"]"],
    ["opaque-before-callee", 0, "app.frame_before_input_step(handle, directives, self.dpr, cursor)", "app.frame_before_input(handle, directives, self.dpr, cursor)"],
    ["bulk-draw-clear", 0, "if previous.retire_step() {\n                    cursor.previous_draw = None;", "if { self.draw.clear(); true } {\n                    cursor.previous_draw = None;"],
    ["select-whole-materialization", 13, "UiNode::Select(select) => {\n            if select.items.len()", "UiNode::Select(select) => {\n            let _whole_select = select.items.iter().collect::<Vec<_>>();\n            if select.items.len()"],
    ["immediate-deferred-drive", 0, "self.pending_frame_deferred = Some", "self.drive_pending_frame_deferred(handle); self.pending_frame_deferred = Some"],
    ["whole-chrome", 5, "render_chrome_step", "render_chrome"],
    ["mounted-navbar-whole-label", 5, "RetainedChromeGroupStep::Fault => self.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string())", "RetainedChromeGroupStep::Fault => { chrome_text(draw, atlas, input, theme, item.label.unwrap_or_default(), rect.x, rect.y, theme.font_size_small, theme.text); self.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string()) }"],
    ["whole-main-child", 5, "render_main_window_step(&mut cursor.child", "render_main_window(draw, &mut overlay_slot, atlas, icons, input, theme, body, engine_resources, world_resources); render_main_window_step(&mut cursor.child"],
    ["sync-whole-child-materialization", 13, "pub(crate) fn sync_interactive_state_node_step(tree: &mut UiTree, id: NodeId, theme: &Theme, cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {", "pub(crate) fn sync_interactive_state_node_step(tree: &mut UiTree, id: NodeId, theme: &Theme, cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {\n    let _whole_children = tree.children(id).collect::<Vec<_>>();"],
    ["bulk-engine-take", 6, "take_packet_step", "take_packets"],
    ["dynamic-world-uploads", 7, "uploads: Box<[Option<PreparedRenderUpload>; WORLD3D_FRAME_RESOURCE_CAPACITY]>", "uploads: Vec<PreparedRenderUpload>"],
    ["bulk-world-append", 7, "pub fn append_step", "pub fn append_to"],
    ["maintenance-authority-release-erasure", 0, "fn release(&self, generation: u64) -> bool {\n        if !self.is_live(generation) {\n            return false;\n        }\n        self.generation.store(0, std::sync::atomic::Ordering::Release);\n        self.state.compare_exchange(1, 0, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()\n    }", "fn release(&self, _generation: u64) -> bool {\n        true\n    }"],
    ["deferred-run-to-completion", 0, "let Some(work) = cursor_value.take_next() else {", "while cursor_value.take_next().is_some() {}\n        let Some(work) = cursor_value.take_next() else {"],
    ["bulk-deferred-close", 0, "if self.actions.pop_front().is_some() {\n            return false;\n        }", "if !self.actions.is_empty() {\n            self.actions = FrameActionOwners::default();\n            return false;\n        }"],
    ["atlas-credit-after-allocation", 8, "let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)", "let _premature = Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY]); let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)"],
    ["bulk-atlas-close", 8, "slots[index] = None", "slots.fill(None)"],
    ["two-atlas-pages-per-upload", 9, "cursor.page.checked_add(1)", "cursor.page.checked_add(2)"],
    ["whole-document-frame", 11, "engine.frame_into_step", "engine.frame"],
    ["dynamic-paint-stack", 12, "visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS]", "visits: Vec<RetainedPaintVisit>"],
    ["complete-text-wrap", 13, "let Some(ch) = value[cursor.byte..].chars().next() else", "for ch in value[cursor.byte..].chars() { let _ = atlas.ensure_glyph(ch, size); }\n    let Some(ch) = value[cursor.byte..].chars().next() else"],
    ["whole-nontext-paint", 13, "UiNode::Tree(tree_node) => retained_tree_node_step(tree_node, bounds, theme, atlas, icons, draw, cursor),", "UiNode::Tree(_) => { paint_node_self(tree, id, origin_x, origin_y, theme, atlas, icons, has_scene_host, draw); RetainedNodePaintStep::Complete },"],
    ["whole-scene-collection", 14, "UiNode::ComponentScene", "collect_scene_slots(tree, id); UiNode::ComponentScene"],
    ["whole-context-menu", 5, "render_context_menu_step", "render_context_menu"],
    ["whole-tour", 5, "render_chrome_tour_step", "render_chrome_tour"],
    ["whole-shell-glyph-callee", 5, "paint_retained_glyph_step(text, Rect::new", "for ch in text.chars() { let _ = atlas.ensure_glyph(ch, size); }\n    paint_retained_glyph_step(text, Rect::new"],
    ["cloned-cleanup-key", 5, "extract_if(|_, _| true).next()", "keys().next().cloned()"],
    ["dynamic-find-owner", 5, "slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>", "slots: Vec<ShellFindItem>"],
    ["blocking-find-binding", 5, "std::cell::Cell<Option<ActiveShellFindItems>>", "std::sync::Mutex<Option<ActiveShellFindItems>>"],
    ["unowned-find-push", 5, "pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem>", "pub fn try_push_find_item(item: ShellFindItem)"],
    ["whole-find-take", 5, "fn pop_front(&mut self) -> Option<ShellFindItem>", "fn take_find_items(&mut self) -> Option<ShellFindItem>"],
    ["missing-find-max-law", 5, "find_item_max_plus_one_returns_the_exact_owned_item", "find_item_max_plus_one_smoke"],
    ["synchronous-preferences-load", 5, "self.request_chrome_preferences_load();", "self.load_ui_prefs_once();"],
    ["synchronous-introduction-read", 5, "self.request_introduction_read();", "self.read_stored_introduction_seen();"],
    ["synchronous-layout-persist", 5, "self.request_panel_layout_persist();", "self.persist_panel_layout_if_changed();"],
    ["synchronous-presence-preview", 5, "self.request_presence_preview();", "self.publish_presence_heartbeat();"],
    ["synchronous-preferences-persist", 5, "self.request_chrome_preferences_persist();", "self.persist_ui_prefs_if_changed();"],
    ["maintenance-on-interactive-lane", 0, "renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job)", "renderer_worker_pool().try_submit(semio_framework_async::Lane::Interactive, job)"],
    ["missing-maintenance-cancel", 0, "let cancelled = cursor.cancel.is_cancelled_now();", "let cancelled = false;"],
    ["missing-maintenance-stale-witness", 0, "mailbox.0.presentation_authority.witness_for(generation).is_none()", "false"],
    ["maintenance-terminal-keeps-deferred-work", 0, "if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded) {\n                    cursor.begin_close();", "if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded) {\n                    cursor.shell_maintenance = false;"],
    ["discarded-maintenance-submission", 0, "match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job) {", "renderer_worker_pool().submit(semio_framework_async::Lane::Io, job);\n        return Ok(());\n        match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job) {"],
    ["dynamic-preference-page", 16, "let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES];", "let mut page = vec![0u8; STORAGE_FIXED_FILE_PAGE_BYTES];"],
    ["whole-preference-json", 5, "String::from_utf8(page).ok()", "serde_json::from_slice::<String>(&page).ok()"],
    ["ui-bypasses-host-storage-read", 5, "semio_framework_os_services::storage_worker_read_fixed_file_page(&path, SHELL_CHROME_IO_FIELD_BYTES)", "std::fs::read(&path)"],
    ["ui-bypasses-host-storage-write", 5, "semio_framework_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), SHELL_CHROME_IO_FIELD_BYTES)", "std::fs::write(&path, value.as_bytes())"],
    ["missing-fixed-page-law", 16, "fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page", "fixed_file_page_smoke"],
    ["missing-paint-output-credit", 13, "draw.begin_retained_output(1, std::mem::size_of::<crate::wgpu::draw::UiInstance>())", "Ok::<(), ()>(())"],
    ["missing-paint-byte-cursor", 13, "byte: usize", "bytes: Vec<u8>"],
    ["missing-multimegabyte-text-law", 13, "retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity", "retained_text_large_smoke"],
    ["missing-nontext-large-law", 13, "retained_multi_megabyte_input_advances_one_scalar_per_grant", "retained_large_input_smoke"],
    ["missing-shell-large-law", 5, "dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant", "dialog_and_tour_text_smoke"],
    ["missing-maintenance-drop-handback", 0, "if self.armed && self.registry.abandon(self.generation)", "if false && self.registry.abandon(self.generation)"],
    ["missing-maintenance-terminal-law", 0, "frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant", "frame_maintenance_terminal_smoke"],
    ["whole-component-scene-renderer", 11, "render_component_scene_step(scene, slot.rect, &mut ctx, cursor)", "render_component_scene(scene, slot.rect, &mut ctx)"],
    ["whole-image-renderer", 11, "render_ui_image_step(image, slot.rect, &mut ctx, cursor)", "render_ui_image(image, slot.rect, &mut ctx)"],
    ["missing-scene-node-owner", 14, "node: Option<NodeId>", "node: NodeId"],
    ["bulk-scene-byte-run", 15, "cursor.advance_byte()", "cursor.advance_byte_run()"],
    ["bulk-image-byte-run", 11, "cursor.advance_byte()", "cursor.advance_byte_run()"],
    ["missing-scene-stale-law", 14, "scene_paint_cursor_rejects_stale_node_without_consuming_owner", "scene_paint_cursor_stale_smoke"],
    ["blocking-atlas-ledger", 8, "static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64", "static PREPARED_ATLAS_PROCESS_PERMITS: Mutex<usize>"],
    ["missing-atlas-backing-dimension", 8, "prepared_atlas_field(current, PREPARED_ATLAS_BACKING_SHIFT", "prepared_atlas_field(current, PREPARED_ATLAS_PAYLOAD_SHIFT"],
    ["missing-atlas-drop-recovery", 8, "impl Drop for PreparedAtlasPages", "impl PreparedAtlasPages"],
    ["unmounted-atlas-abandonment-drain", 0, "PreparedAtlasPages::close_abandoned_step()", "true"],
    ["missing-atlas-interrupted-close-law", 8, "interrupted_atlas_close_rejoins_the_same_abandonment_authority", "interrupted_atlas_close_smoke"],
  ];
  for (const [name, index, needle, replacement] of mutations) {
    const mutated = [...clean];
    mutated[index] = mutated[index]!.replace(needle, replacement);
    if (mutated[index] === clean[index]) throw new Error(`[verify interactivity] P5a mutation ${name} did not alter source.`);
    if (interactivityMountedFrameTransactionFailures(...mutated).length === 0) throw new Error(`[verify interactivity] P5a mutation ${name} was falsely accepted.`);
  }
  const legacyChromeMutations: [string, string, string][] = [
    ["production-chrome-measure-oracle", "#[cfg(test)]\nfn measure_chrome_group_item", "fn measure_chrome_group_item"],
    ["production-chrome-render-oracle", "#[cfg(test)]\nfn render_chrome_group", "fn render_chrome_group"],
    ["mounted-legacy-chrome-group-restoration", "RetainedChromeGroupStep::Fault => self.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string())", "RetainedChromeGroupStep::Fault => { render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true); self.error = Some(\"Shell fullscreen item exceeded the retained glyph boundary\".to_string()) }"],
  ];
  for (const [name, needle, replacement] of legacyChromeMutations) {
    const mutated = [...clean];
    mutated[5] = mutated[5]!.replace(needle, replacement);
    if (mutated[5] === clean[5]) throw new Error(`[verify interactivity] P5a mutation ${name} did not alter Shell source.`);
    if (interactivityMountedFrameTransactionFailures(...mutated).length === 0) throw new Error(`[verify interactivity] P5a mutation ${name} was falsely accepted.`);
  }
  const failures = interactivityMountedFrameTransactionFailures(...clean);
  if (failures.length !== 0) throw new Error(`[verify interactivity] P5a mounted frame baseline was falsely rejected: ${failures.join("; ")}`);
}
