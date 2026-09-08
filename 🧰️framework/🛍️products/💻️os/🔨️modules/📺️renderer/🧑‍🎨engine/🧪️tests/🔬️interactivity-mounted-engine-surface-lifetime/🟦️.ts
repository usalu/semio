import { INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_WINIT_HOST_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, policyReadFileSafe, interactivityMountedEngineSurfaceLifetimeFailures } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity mounted engine surface lifetime policy assertions. */
export function interactivityMountedEngineSurfaceLifetimeSelfTests(repoRoot: string): void {
  const files = [
    INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE,
    INTERACTIVITY_AUDIT_RENDERER_HOST_FILE,
    INTERACTIVITY_AUDIT_WINIT_HOST_FILE,
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs",
    INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE,
    "🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs",
    "🧰️framework/🔨️modules/✍️editor/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs",
  ] as const;
  const clean = files.map((file) => policyReadFileSafe(repoRoot, file));
  const mutations: readonly [string, number, string, string][] = [
    ["wrapping-cpu-generation", 0, "slot.generation.checked_add(1)", "Some(slot.generation.wrapping_add(1))"],
    ["blocking-cpu-close-registry", 0, "self.state().try_lock().ok()", "self.state().lock().ok()"],
    ["dynamic-gpu-registry", 0, "slots: ManuallyDrop<Option<Box<[EngineGpuSlot; ENGINE_SURFACE_CAPACITY]>>>", "slots: HashMap<String, EngineGpuSurface>"],
    ["missing-flow-disposer", 0, "NodeGraphEngineRetirement::Flow", "NodeGraphEngineRetirement::Dag"],
    ["non-atomic-publication", 0, "return slot.publish_candidate(packet, expected, primary_metrics_generation);", "slot.candidate = None; return Ok(true);"],
    ["missing-generation-abandonment", 1, "generation: AtomicU64,\n    exhausted: AtomicBool", "generation: usize,\n    exhausted: AtomicBool"],
    ["ordinary-retirement-drop", 1, "publish_os_host_retirement_abandonment(token, state)", "drop(state); Ok(())"],
    ["unpaired-cpu-close", 1, "runtime.close_engine_surface_step(token, self.operation, &mut self.sequence)", "true"],
    ["native-drop", 2, "host.try_into_retirement()", "drop(host); return"],
    ["browser-unmounted-abandonment", 3, "OsHostRetirement::close_abandoned_step()", "true"],
    ["resize-skips-engine-invalidation", 4, "if !self.engine.invalidate_primary_metrics_step()", "if false"],
    ["graph-whole-drop", 5, "dag::DagHostRetirement::new(dag)", "drop(dag); return"],
    ["flow-whole-store-drop", 6, "store.close_owned_step(1, 4_096)", "drop(store); Ok(SnapshotRetirementStep::Complete)"],
    ["ordinary-replacement-retirement-unmounted", 0, "if let Some(retirement) = slot.retirement.as_mut()", "if let Some(retirement) = Option::<&mut EngineGpuRetirement>::None"],
    ["realize-fault-returned-before-close", 4, "self.retained_fault = Some(format!(\"engine canvas present: {error}\"));", "return Err(format!(\"engine canvas present: {error}\"));"],
    ["single-rejected-packet-slot", 0, "} else if self.rejected_len < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {", "} else if self.rejected_len == 0 {"],
    ["outer-surface-weak-terminal-witness", 0, "self.phase == EngineSurfaceClosePhase::Released\n            && self.node_graph_source.is_none()", "self.phase == EngineSurfaceClosePhase::Released\n            || self.node_graph_source.is_none()"],
    ["packet-self-freshness", 0, "engine_surface_live_freshness(packet.surface.token)", "Ok(Some(EngineSurfaceLiveFreshness { identity: packet.surface, metrics_generation: packet.metrics_generation, document_generation: packet.document_generation, scene_revision: packet.scene_revision }))"],
    ["map-weak-terminal-witness", 7, "self.released\n            && self.positions.is_empty()", "self.released\n            || self.positions.is_empty()"],
    ["editor-weak-terminal-witness", 8, "self.released\n            && self.text.is_empty()", "self.released\n            || self.text.is_empty()"],
    ["dag-weak-terminal-witness", 9, "self.released\n            && self.engine.terminal_is_empty()", "self.released\n            || self.engine.terminal_is_empty()"],
    ["missing-populated-law", 0, "populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time", "populated_surface_smoke"],
    ["missing-abandonment-law", 1, "interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one", "host_retirement_smoke"],
  ];
  for (const [name, index, needle, replacement] of mutations) {
    const mutated = [...clean];
    mutated[index] = mutated[index]!.replace(needle, replacement);
    if (mutated[index] === clean[index]) throw new Error(`[verify interactivity p3mn] mutation ${name} did not bind live source`);
    if (interactivityMountedEngineSurfaceLifetimeFailures(...mutated).length === 0) throw new Error(`[verify interactivity p3mn] mutation ${name} was falsely accepted`);
  }
  const failures = interactivityMountedEngineSurfaceLifetimeFailures(...clean);
  if (failures.length !== 0) throw new Error(`[verify interactivity p3mn] live source rejected before mutations: ${failures.join("; ")}`);
}
