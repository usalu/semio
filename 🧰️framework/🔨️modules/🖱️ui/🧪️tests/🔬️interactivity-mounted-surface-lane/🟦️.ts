import { INTERACTIVITY_AUDIT_SURFACE_LANE_FILE, INTERACTIVITY_AUDIT_UI_ENGINE_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_WINIT_HOST_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, policyReadRustPolicySource, interactivityMountedSurfaceLaneFailures } from "../../../../../📜️script.ts";

/** 🧪️ Executes interactivity mounted surface lane policy assertions. */
export function interactivityMountedSurfaceLaneSelfTests(repoRoot: string): void {
  const files = [INTERACTIVITY_AUDIT_SURFACE_LANE_FILE, INTERACTIVITY_AUDIT_UI_ENGINE_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_WINIT_HOST_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE] as const;
  const clean = files.map((file) => policyReadRustPolicySource(repoRoot, file));
  const mutations: readonly [string, number, string, string][] = [
    ["dynamic-resize-registry", 0, "static SURFACE_LANE_OCCUPIED: [AtomicBool; SURFACE_RESIZE_LANE_CAPACITY]", "static SURFACE_LANE_OCCUPIED: Vec<AtomicBool>"],
    ["wrapping-resize-generation", 0, "checked_add(1)", "wrapping_add(1)"],
    ["bulk-worker-fuel", 0, "const SURFACE_RESIZE_STEP_FUEL: u64 = 1", "const SURFACE_RESIZE_STEP_FUEL: u64 = 64"],
    ["wide-worker-deadline", 0, "const SURFACE_RESIZE_STEP_BUDGET_MS: u64 = 1", "const SURFACE_RESIZE_STEP_BUDGET_MS: u64 = 16"],
    ["missing-finite-gate", 0, "!scale_factor.is_finite() || scale_factor <= 0.0", "false"],
    ["stale-candidate-publication", 0, "candidate.metrics_generation == self.metrics_generation", "true"],
    ["missing-lane-drop", 0, "impl Drop for MountedSurfaceResizeLane", "impl MountedSurfaceResizeLane"],
    ["missing-abandonment-drain", 3, "MountedSurfaceResizeLane::close_abandoned_step()", "true"],
    ["immediate-callback-resize", 3, "self.surface_resize.enqueue(metrics.physical.width, metrics.physical.height, metrics.scale_factor)", "self.presenter.resize(width, height, metrics.scale_factor)"],
    ["native-ordinary-host-drop", 3, "host.try_into_retirement()", "drop(host); return"],
    ["cursorless-presenter", 3, "self.presenter.begin_surface_resize(candidate)", "drop(candidate)"],
    ["dynamic-lane-entry", 1, "slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS]", "slots: Vec<SurfaceLaneEntry>"],
    ["unqualified-lane-entry", 1, "epoch: u64", "queued: bool"],
    ["dynamic-theme-tokens", 1, "tokens: [Option<UiSurfaceToken>; UI_LAYOUT_SURFACE_SLOTS]", "tokens: Vec<UiSurfaceToken>"],
    ["whole-theme-propagation", 1, "self.theme_propagation = Some(ThemePropagationCursor::new(theme))", "self.theme = theme"],
    ["missing-drop-law", 0, "interrupted_lane_drop_is_rediscovered_and_incrementally_closed", "interrupted_lane_drop_smoke"],
  ];
  for (const [name, index, needle, replacement] of mutations) {
    const mutated = [...clean];
    mutated[index] = mutated[index]!.replace(needle, replacement);
    if (mutated[index] === clean[index]) throw new Error(`[verify interactivity p5e] mutation ${name} did not bind live source`);
    if (interactivityMountedSurfaceLaneFailures(...mutated).length === 0) throw new Error(`[verify interactivity p5e] mutation ${name} was falsely accepted`);
  }
  const failures = interactivityMountedSurfaceLaneFailures(...clean);
  if (failures.length !== 0) throw new Error(`[verify interactivity p5e] live source rejected before mutations: ${failures.join("; ")}`);
}
