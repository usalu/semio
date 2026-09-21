import { INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE, INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, policyReadRustPolicySource, interactivityMountedPreparedRenderFailures } from "../../../../../../📜️script.ts";

/** 📚️ One policy source per audited file, arity-preserving so the failure checkers keep their positional parameters. */
type PreparedRenderPolicySources = [string, string, string, string, string, string];

/** 🧪️ Executes interactivity mounted prepared render policy assertions. */
export function interactivityMountedPreparedRenderSelfTests(repoRoot: string): void {
  const files = [
    INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE,
    INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE,
    INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE,
    INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE,
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🖼️frame/🦀️.rs",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs",
  ];
  const clean: Readonly<PreparedRenderPolicySources> = [
    policyReadRustPolicySource(repoRoot, files[0]),
    policyReadRustPolicySource(repoRoot, files[1]),
    policyReadRustPolicySource(repoRoot, files[2]),
    policyReadRustPolicySource(repoRoot, files[3]),
    policyReadRustPolicySource(repoRoot, files[4]),
    policyReadRustPolicySource(repoRoot, files[5]),
  ] as const;
  const mutations: readonly [string, number, string, string][] = [
    ["blocking-process-ledger", 0, "static PREPARED_RENDER_PROCESS_PERMITS: AtomicU64", "static PREPARED_RENDER_PROCESS_PERMITS: Mutex<u64>"],
    ["wrapping-process-generation", 0, "current_generation.checked_add(1)", "Some(current_generation.wrapping_add(1))"],
    ["missing-process-drop", 0, "impl Drop for PreparedRenderProcessPermit", "impl PreparedRenderProcessPermit"],
    ["dynamic-metadata", 0, "pages: [Option<Box<PreparedFixedPage<T>>>; PREPARED_RENDER_METADATA_PAGES]", "pages: Vec<Box<PreparedFixedPage<T>>>"],
    ["dynamic-command-pages", 0, "directories: [Option<Box<PreparedRenderCommandDirectory>>; PREPARED_RENDER_COMMAND_DIRECTORIES]", "directories: Vec<Box<PreparedRenderCommandDirectory>>"],
    ["unowned-command-refusal", 0, "Result<(), PreparedRenderCommand>", "Result<(), ()>"],
    ["missing-draw-cursor", 0, "draw_cursor: Option<DrawMeasureCursor>", "draw_index: usize"],
    ["missing-overlay-owner", 0, "packet_overlay: bool", "overlay: usize"],
    ["missing-input-abandonment", 0, "impl Drop for PreparedRenderInput", "impl PreparedRenderInput"],
    ["missing-job-abandonment", 0, "impl Drop for PreparedRenderJob", "impl PreparedRenderJob"],
    ["bulk-worker-fuel", 0, "cx.consume_fuel(1);", "cx.consume_fuel(64);"],
    ["worker-loop", 0, "let Some(usage) = self.measure_next() else", "while let Some(usage) = self.measure_next() { return StepOutcome::Yield; }\n        let Some(usage) = self.measure_next() else"],
    ["missing-pre-publish-generation", 0, "input.preview_generation != cx.generation().0", "false"],
    ["cursorless-tessellation", 0, "draw_cursor: Some(prepared_cursor)", "draw_cursor: None"],
    ["unretained-publication", 0, "self.receiver.publish(packet)", "drop(packet); Ok(())"],
    ["whole-command-capacity", 0, "PreparedRenderCommandDirectory::default()", "Vec::with_capacity(PREPARED_RENDER_COMMAND_PAGES)"],
    ["missing-input-drop-law", 0, "input_drop_hands_back_exact_process_permits_for_incremental_close", "input_drop_smoke"],
    ["missing-worker-panic-law", 0, "worker_panic_hands_back_the_exact_job_and_mailbox_owners", "worker_panic_smoke"],
    ["missing-command-max-law", 0, "fixed_command_pages_reject_max_plus_one_without_consuming_the_owner", "command_max_smoke"],
    ["missing-gpu-drop", 2, "impl Drop for PreparedGpuPresentCursor", "impl PreparedGpuPresentCursor"],
    ["bulk-gpu-command", 2, "cursor.command.checked_add(1)", "packet.command_pages().len()"],
    ["missing-glass-owner", 2, "prepared_glass_command(packet, cursor.command)?", "packet.draw.glass_regions.first().unwrap()"],
    ["bulk-gpu-blur", 2, "cursor.blur_mip.checked_add(1)", "SCENE_MIP_LEVELS"],
    ["missing-gpu-watchdog", 2, "admit_prepared_gpu_opportunity(cursor.overrun_run, elapsed)", "Ok(0)"],
    ["whole-gpu-render", 2, "self.encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay())?", "self.render_prepared(packet)?"],
    ["missing-current-backdrop", 2, "blit_prepared_composite(&self.device, &mut encoder, scene.mip_view(0), composite)", "blit_prepared_scene(&self.device, &mut encoder, composite.view(), scene)"],
    ["missing-ui-scalar", 1, "pub fn encode_prepared_ui_scalar", "fn encode_ui_batch"],
    ["dynamic-ui-scalar", 1, "std::slice::from_ref(instance)", "&vec![*instance]"],
    ["whole-vector-draw", 1, "pass.draw(0..3, 0..1)", "pass.draw(0..vertices.len() as u32, 0..1)"],
    ["whole-world-instance", 1, "pass.draw_indexed(0..mesh.index_count, 0, 0..1)", "pass.draw_indexed(0..mesh.index_count, 0, 0..instances.len() as u32)"],
    ["whole-world-line", 1, "pass.draw(0..2, 0..1)", "pass.draw(0..vertices.len() as u32, 0..1)"],
    ["unmounted-gpu-drain", 3, "PreparedGpuPresentCursor::close_abandoned_step()", "true"],
    ["unmounted-input-drain", 3, "PreparedRenderInput::close_abandoned_step()", "true"],
    ["unmounted-job-drain", 3, "PreparedRenderJob::close_abandoned_step()", "true"],
    ["unmounted-packet-drain", 3, "PreparedRenderPacket::close_abandoned_step()", "true"],
    ["caller-bulk-fuel", 3, "fuel_per_step: 1", "fuel_per_step: 64"],
    [
      "caller-wide-deadline",
      3,
      'site: "os_renderer.prepare.worker", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_us: 1000',
      'site: "os_renderer.prepare.worker", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_us: 16000',
    ],
    ["whole-frame-builder", 4, "#[cfg(test)]\n    pub fn build_frame", "    pub fn build_frame"],
    ["whole-scene-builder", 5, "#[cfg(any(test, feature = \"backend-testing\"))]\n    pub fn finish(", "    pub fn finish("],
    ["missing-gpu-interruption-law", 2, "interrupted_present_cursor_hands_back_generation_and_fixed_owners", "present_interruption_smoke"],
  ];
  for (const [name, index, needle, replacement] of mutations) {
    const mutated: PreparedRenderPolicySources = [...clean];
    mutated[index] = mutated[index]!.replaceAll(needle, replacement);
    if (mutated[index] === clean[index]) throw new Error(`[verify interactivity p5d] mutation ${name} did not bind live source`);
    if (interactivityMountedPreparedRenderFailures(...mutated).length === 0) throw new Error(`[verify interactivity p5d] mutation ${name} was falsely accepted`);
  }
  const failures = interactivityMountedPreparedRenderFailures(...clean);
  if (failures.length !== 0) throw new Error(`[verify interactivity p5d] live source rejected before mutations: ${failures.join("; ")}`);
}
