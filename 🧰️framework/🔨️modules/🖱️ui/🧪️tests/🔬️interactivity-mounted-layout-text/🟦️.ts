import { policyReadFileSafe, interactivityMountedLayoutTextFailures } from "../../../../../📜️script.ts";

/** 🧪️ Executes interactivity mounted layout text policy assertions. */
export function interactivityMountedLayoutTextSelfTests(repoRoot: string): void {
  const paths = [
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧵️mounted_layout.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/⚙️engine.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🌲️tree.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🖌️paint.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎯️events.rs",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎬️scene_slots.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs",
  ] as const;
  const clean = paths.map((path) => policyReadFileSafe(repoRoot, path));
  const mutations: [string, number, string, string][] = [
    ["node-credit", 0, "LAYOUT_NODE_CREDITS: usize = 4_096", "LAYOUT_NODE_CREDITS: usize = 4_095"],
    ["glyph-credit", 0, "LAYOUT_GLYPH_CREDITS: usize = 16_384", "LAYOUT_GLYPH_CREDITS: usize = 16_383"],
    ["dynamic-node-storage", 0, "nodes: Box<ui_contract::UiFixedList<LayoutInputNode", "nodes: Vec<LayoutInputNode"],
    ["dynamic-atlas-storage", 0, "pages: [Option<Box<[u8; LAYOUT_ATLAS_PAGE_BYTES]>>; LAYOUT_ATLAS_PAGE_CREDITS]", "pages: Vec<Box<[u8; LAYOUT_ATLAS_PAGE_BYTES]>>"],
    ["missing-run-cursor", 0, "run_cursor: usize", "run_progress: usize"],
    ["missing-line-cursor", 0, "line_cursor: usize", "line_progress: usize"],
    ["whole-shape-loop", 0, "fn shape_one(&mut self)", "fn shape_one(&mut self) /* loop { */"],
    ["ui-thread-text-measure", 0, "self.text_worker.shape_one(input)", "FontAtlas::measure(input)"],
    ["missing-post-cancel", 0, "if cx.is_cancelled() {\n            return semio_framework_job::StepOutcome::Cancelled;\n        }\n        if self.fault.is_some()", "if self.fault.is_some()"],
    ["partial-live-publish", 0, "tree.write_inactive_layout", "tree.node_mut"],
    ["missing-atomic-commit", 0, "tree.commit_inactive_layout(generation)", "let _ = generation"],
    ["missing-completeness", 0, "self.results.len() != self.nodes.len()", "false"],
    ["bulk-close", 0, "self.results.pop().is_some()", "self.results.clear(); false"],
    ["dynamic-surface-registry", 1, "slots: [Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]", "slots: HashMap<String, UiWindow>"],
    ["dynamic-lane-ring", 1, "slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS]", "slots: VecDeque<SurfaceLaneEntry>"],
    ["wrapping-generation", 1, "window.layout_generation.checked_add(1)", "Some(window.layout_generation.wrapping_add(1))"],
    ["caller-lane-step", 1, "session.pump_one(pool, worker_lane(lane))", "job.worker_one(cx)"],
    ["missing-cancel-propagation", 1, "cancel: cx.cancel_token()", "cancel: semio_framework_job::CancelToken::root_now()"],
    ["bulk-session-drop", 1, "session.close_step(1", "window.layout_session = None; session.close_step(1"],
    ["missing-progressive-preview", 1, "MountedLayoutJob::take_preview_one", "MountedLayoutJob::latest_glyph_preview"],
    ["missing-double-buffer", 2, "mounted_layout: [MountedLayoutRecord; 2]", "mounted_layout: [MountedLayoutRecord; 1]"],
    ["missing-snapshot-swap", 2, "self.mounted_layout_active ^= 1", "self.mounted_layout_active = 0"],
    ["paint-live-layout", 3, "let Some(layout) = tree.accepted_layout(id) else { return RetainedNodePaintStep::Fault }", "let layout = Default::default()"],
    ["event-live-layout", 4, "let layout = tree.accepted_layout(id)?", "let layout = Default::default()"],
    ["slot-live-layout", 5, "let Some(layout) = tree.accepted_layout(id) else { return }", "let layout = Default::default()"],
    ["zero-production-driver", 6, "engine.step_layouts(&pool", "engine.needs_frame(); //"],
    ["unbounded-renderer-budget", 6, "StepBudget::new(1, now.saturating_add(1))", "StepBudget::new(u64::MAX, u64::MAX)"],
    ["second-scheduler", 7, "process_worker_pool", "WorkerPool::new"],
  ];
  for (const [name, index, needle, replacement] of mutations) {
    const mutated = [...clean];
    mutated[index] = mutated[index].replace(needle, replacement);
    if (mutated[index] === clean[index]) throw new Error(`[verify interactivity] P5c mutation ${name} did not alter source.`);
    if (interactivityMountedLayoutTextFailures(...mutated).length === 0) throw new Error(`[verify interactivity] P5c mutation ${name} was falsely accepted.`);
  }
  const lawMutations: [number, string][] = [
    [0, "job.rejected_glyph().map"],
    [0, "tree.contains(rejected.id)"],
    [0, "job.glyph_cursor - before <= 1"],
    [0, "retained - after, 1"],
    [0, "session.pump_one(&pool, lane)"],
    [0, "after.cancel_after_shape"],
    [0, "retained - after_one, 1"],
    [0, "tree.accepted_layout_generation(), before"],
    [1, "rejected.id, owner"],
    [1, "assert_eq!(ui.windows.get(\"theme\").map(|window| window.theme_revision), before_theme_revision)"],
    [1, "swaps, 1"],
    [1, "theme_revision = u64::MAX"],
    [1, "assert_eq!(first, second)"],
    [1, "Duration::from_millis(8)"],
    [1, "slice < LANE_WHEEL.len()"],
  ];
  for (const [index, needle] of lawMutations) {
    const mutated = [...clean];
    mutated[index] = mutated[index].replace(needle, "P5C_MUTATED_LAW_EVIDENCE");
    if (mutated[index] === clean[index]) throw new Error(`[verify interactivity] P5c law mutation ${needle} did not alter source.`);
    if (interactivityMountedLayoutTextFailures(...mutated).length === 0) throw new Error(`[verify interactivity] P5c law mutation ${needle} was falsely accepted.`);
  }
  const failures = interactivityMountedLayoutTextFailures(...clean);
  if (failures.length !== 0) throw new Error(`[verify interactivity] P5c mounted layout/text baseline was falsely rejected: ${failures.join("; ")}`);
}
