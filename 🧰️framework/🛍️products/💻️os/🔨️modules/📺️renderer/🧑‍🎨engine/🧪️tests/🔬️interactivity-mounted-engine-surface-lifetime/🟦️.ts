import { INTERACTIVITY_AUDIT_FILE_MARKER, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, INTERACTIVITY_AUDIT_RENDERER_HOST_FILE, INTERACTIVITY_AUDIT_WINIT_HOST_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, interactivityMountedEngineSurfaceLifetimeFailures } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import { policyReadFileSafe } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { policyModuleTreeFiles } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { listSourceFamily } from "../../../../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";
import { relative, sep } from "node:path";
import { POLICY_RUST_TEST_EVIDENCE_BOUNDARY } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { readdirSync } from "node:fs";
import { dirname, join } from "node:path";

/** 📖️ The mounted surface owners as the one text each unsplit module held: production of the live crate trees first, then the engine test corpus. */
export function interactivityMountedEngineSurfaceSources(repoRoot: string): Parameters<typeof interactivityMountedEngineSurfaceLifetimeFailures> {
  const rel = (absolute: string): string => relative(repoRoot, absolute).split(sep).join("/");
  const textOf = (file: string): string => `${INTERACTIVITY_AUDIT_FILE_MARKER}${file}\n${policyReadFileSafe(repoRoot, file)}`;
  const production = (...files: string[]): string => [...new Set(files.flatMap((file) => policyModuleTreeFiles(repoRoot, file, { crates: true })))].map(textOf).join("\n");
  const rustIn = (directory: string, needle: string): string => {
    const folder = readdirSync(directory).find((name) => name.includes(needle));
    if (!folder) throw new Error(`[verify interactivity p3mn] missing ${needle}`);
    const rust = readdirSync(join(directory, folder)).find((name) => name.endsWith(".rs"));
    if (!rust) throw new Error(`[verify interactivity p3mn] missing rust root ${needle}`);
    return rel(join(directory, folder, rust));
  };
  const engineDir = dirname(dirname(dirname(dirname(join(repoRoot, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE)))));
  const contracts = rustIn(dirname(dirname(join(repoRoot, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE))), "element-contracts");
  const elements = dirname(dirname(dirname(dirname(join(repoRoot, INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE)))));
  const graph = rustIn(elements, "NodeGraph");
  const flowHost = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs";
  const hostClose = rustIn(dirname(join(repoRoot, flowHost)), "host-close");
  const tests = listSourceFamily(engineDir, { tests: true }).map(rel).filter((file) => file.split("/").some((segment) => segment.includes("tests") || segment.includes("testing"))).map((file) => policyReadFileSafe(repoRoot, file)).join("\n");
  const withTests = (...files: string[]): string => production(...files) + POLICY_RUST_TEST_EVIDENCE_BOUNDARY + tests;
  const dagRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag";
  return [
    withTests(INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE, contracts),
    withTests(INTERACTIVITY_AUDIT_RENDERER_HOST_FILE),
    production(INTERACTIVITY_AUDIT_WINIT_HOST_FILE),
    production("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"),
    withTests(INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE),
    production(graph),
    production(flowHost, hostClose),
    production("🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs"),
    production("🧰️framework/🔨️modules/✍️editor/🦀️.rs"),
    ["🗃️model", "🕸️graph", "🧭️host"].map((node) => production(`${dagRoot}/${node}/🦀️.rs`)).join("\n"),
  ];
}

/** 🧪 Executes interactivity mounted engine surface lifetime policy assertions. */
export function interactivityMountedEngineSurfaceLifetimeSelfTests(repoRoot: string): void {
  const clean = interactivityMountedEngineSurfaceSources(repoRoot);
  const mutations: readonly [string, number, string, string][] = [
    ["wrapping-cpu-generation", 0, "slot.generation.checked_add(1)", "Some(slot.generation.wrapping_add(1))"],
    ["blocking-cpu-close-registry", 0, "self.state().try_lock().ok()", "self.state().lock().ok()"],
    ["dynamic-gpu-registry", 0, "slots: ManuallyDrop<Option<Box<[EngineGpuSlot; ENGINE_SURFACE_CAPACITY]>>>", "slots: HashMap<String, EngineGpuSurface>"],
    ["missing-flow-disposer", 0, "NodeGraphEngineRetirement::Flow", "NodeGraphEngineRetirement::Dag"],
    ["non-atomic-publication", 0, "return slot.publish_candidate(packet, expected, primary_metrics_generation);", "slot.candidate = None; return Ok(true);"],
    ["missing-generation-abandonment", 1, "generation: AtomicU64,\n    exhausted: AtomicBool", "generation: usize,\n    exhausted: AtomicBool"],
    ["ordinary-retirement-drop", 1, "publish_os_host_retirement_abandonment(token, state)", "drop(state); Ok(())"],
    ["unpaired-cpu-close", 1, "runtime.close_engine_surface_step(token, cx)", "true"],
    ["native-drop", 2, "OsHost::try_into_retirement(&mut self.host,cx)", "drop(host); return"],
    ["browser-unmounted-abandonment", 3, "OsHost::try_into_retirement(self.host, cx)", "true"],
    ["resize-skips-engine-invalidation", 4, "if !self.engine.invalidate_primary_metrics_step()", "if false"],
    ["graph-whole-drop", 5, "dag::DagHostRetirement::new(dag)", "drop(dag); return"],
    ["flow-whole-store-drop", 6, "store.close_owned_store_step", "drop(store)"],
    ["ordinary-replacement-retirement-unmounted", 0, "if !build.matches_live(live)", "if false"],
    ["realize-fault-returned-before-close", 4, "self.retained_fault = Some(format!(\"engine canvas present: {error}\"));", "return Err(format!(\"engine canvas present: {error}\"));"],
    ["single-rejected-packet-slot", 0, "rejected: semio_framework_async_slots::OriginalFixedSlotPages<EngineCanvasPacket,16,16>", "rejected: Option<EngineCanvasPacket>"],
    ["outer-surface-weak-terminal-witness", 0, "self.phase == EngineSurfaceClosePhase::Released\n            && self.node_graph_source.is_none()", "self.phase == EngineSurfaceClosePhase::Released\n            || self.node_graph_source.is_none()"],
    ["packet-self-freshness", 0, "engine_surface_live_freshness(packet.surface.token)", "Ok(Some(EngineSurfaceLiveFreshness { identity: packet.surface, metrics_generation: packet.metrics_generation, document_generation: packet.document_generation, scene_revision: packet.scene_revision }))"],
    ["map-weak-terminal-witness", 7, "self.released\n            && self.overlays.is_empty()", "self.released\n            || self.overlays.is_empty()"],
    ["editor-weak-terminal-witness", 8, "self.released\n            && self.text.is_empty()", "self.released\n            || self.text.is_empty()"],
    ["dag-weak-terminal-witness", 9, "self.owner.terminal_is_empty()", "true"],
    ["missing-populated-law", 0, "populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time", "populated_surface_smoke"],
    ["missing-abandonment-law", 1, "interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one", "host_retirement_smoke"],
  ];
  const baseline = interactivityMountedEngineSurfaceLifetimeFailures(...clean);
  const detected = (mutated: typeof clean): string[] => interactivityMountedEngineSurfaceLifetimeFailures(...mutated).filter((failure) => !baseline.includes(failure));
  const mutationFailures: string[] = [];
  for (const [name, index, needle, replacement] of mutations) {
    const mutated: typeof clean = [...clean];
    mutated[index] = mutated[index]!.split(needle).join(replacement);
    if (mutated[index] === clean[index]) mutationFailures.push(`mutation ${name} did not bind live source`);
    else if (!name.endsWith("-law") && clean[index]!.includes(POLICY_RUST_TEST_EVIDENCE_BOUNDARY) && clean[index]!.indexOf(needle) > clean[index]!.indexOf(POLICY_RUST_TEST_EVIDENCE_BOUNDARY)) mutationFailures.push(`mutation ${name} altered test evidence instead of production source`);
    else if (detected(mutated).length === 0) mutationFailures.push(`mutation ${name} was falsely accepted`);
  }
  if (baseline.length !== 0) mutationFailures.push(`live source rejected before mutations: ${baseline.join("; ")}`);
  if (mutationFailures.length !== 0) throw new Error(`[verify interactivity p3mn] ${mutationFailures.join("; ")}`);
}

try {
  const { test } = await import("bun:test");
  test("P3mn interactivity mounted engine surface lifetime", () => {
    interactivityMountedEngineSurfaceLifetimeSelfTests(process.cwd());
  }, 180_000);
} catch (error) {
  if (!(error instanceof Error) || !error.message.includes("outside of the test runner")) throw error;
}
