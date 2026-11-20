import { join, resolve } from "node:path";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";

/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type Lane = "artifact" | "config" | "draft" | "presence" | "transient" | "window-config" | "window-transient" | "child" | "interaction" | "host-only";
type Route = { id: string; lanes: Lane[]; frameworkInjected?: true };
type AppAuthority = { owner: string; toolIdsConstants: string[]; source: string; manifestSource: string; routes: Route[]; reservedToolIdsConstant:string; reservedRoutes:Route[]; reservedProducerSource:string; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
type Fixture = { schema: string; apps: AppAuthority[] };

type BootstrapYield = { expectedStages: string[]; expectedAllocationDelta: number };
type ScheduledContinuation = () => void | ScheduledContinuation;

function bootstrapYieldOracle(law: BootstrapYield): void {
  const scheduler = createRequire(import.meta.url)("scheduler/unstable_mock") as {
    unstable_NormalPriority: number;
    unstable_scheduleCallback(priority: number, callback: ScheduledContinuation): unknown;
    unstable_flushNumberOfYields(count: number): void;
    unstable_flushAllWithoutAsserting(): boolean;
    unstable_clearLog(): string[];
    unstable_hasPendingWork(): boolean;
    log(value: string): void;
  };
  let allocations = 0;
  scheduler.unstable_scheduleCallback(scheduler.unstable_NormalPriority, () => {
    scheduler.log("yielded");
    return () => { allocations += 1; scheduler.log("resumed"); };
  });
  scheduler.unstable_flushNumberOfYields(1);
  const stages = scheduler.unstable_clearLog();
  if (allocations !== law.expectedAllocationDelta || !scheduler.unstable_hasPendingWork()) throw new Error("React Scheduler lost or advanced the yielded bootstrap continuation");
  scheduler.unstable_flushAllWithoutAsserting();
  stages.push(...scheduler.unstable_clearLog());
  if (allocations !== 1 || scheduler.unstable_hasPendingWork() || JSON.stringify(stages) !== JSON.stringify(law.expectedStages)) throw new Error("React Scheduler failed the bootstrap resume law");
  console.error(`Drawing bootstrap third-party React Scheduler oracle: ${stages.join("→")}; allocations before resume=${law.expectedAllocationDelta}`);
}

/** 🖍️ Every anchor draw's publication apparatus must carry verbatim: the proof catalogs, both owned
 * factories, the one-item artifact store preparation authority, the exact Canvas window config +
 * transient owner registrations (the view lanes since 2026-09-15 — there is no plugin config store),
 * their freshness guards and their incremental close, plus the accessible bilingual UI surface. */
const ANCHORS = [
  "semio_plugin::bounded_first_step_tool_proofs!",
  "factory_type:",
  "ToolExecutionContract::bounded_first_step",
  "ToolExecutionContract::resumable",
  "build_artifact_store_one_item_preparation_factory",
  "fn register_window_config_owners(",
  "fn register_window_transient_owners(",
  "register_tool_job_factories",
  "build_tool_job",
  "DrawingArtifactStorePreparationFactory::default()",
  "fn cancel(&mut self)",
  "fn begin_close(&mut self)",
  "fn terminal_is_empty(&self)",
  "LocalizedLabel::native",
  "SelectionMode::Multiple",
  ".default_layout(edit::layout())",
];

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

const contractPattern = (id: string): RegExp => new RegExp(`ArtifactToolPublicationContract \\{ tool_id: "${id}", lanes: &\\[[^\\]]*\\] \\},?`);

/** ⚖️ One app's routes must be the same set in its tool-id constants, its publication contracts and its
 * `Migrated` classifications — the exact three-way join `validate_tool_job_rows` demands. A route the
 * framework injects already classified is excluded from the classification side only: it still needs
 * its constant row and its publication contract (draw has none today — the shell's utility swap is a
 * window-transient edit, not a routed tool). */
function appOracle(app: AppAuthority, source: string): boolean {
  const ids = app.toolIdsConstants.flatMap((constant) => [...(source.match(new RegExp(`${constant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? [])].map((match) => match[1]!));
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((match) => `${match[1]}:${[...match[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((lane) => lane[1]).sort().join("+")}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", (?:semio_plugin::)?InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = app.routes.map(({ id }) => id);
  const authored = app.routes.filter((route) => route.frameworkInjected !== true).map(({ id }) => id);
  const reserved=[...(source.match(new RegExp(`${app.reservedToolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`,"s"))?.[1]?.matchAll(/"([^"]+)"/g)??[])].map(match=>match[1]!);
  return Object.values(app.laws).every(Boolean)
    && exact(reserved,app.reservedRoutes.map(route=>route.id))&&app.reservedRoutes.every(route=>!expected.includes(route.id))&&source.includes("fn build_reserved_tool_job(")&&source.includes(`${app.reservedToolIdsConstant}.contains(&request.tool_id.as_str())`)
    && app.ui.locales.join(",") === "en,de" && app.ui.accessibleLabels && app.ui.customizableUi
    && app.routes.every((route) => route.lanes.length > 0 && (!route.lanes.includes("host-only") || route.lanes.length === 1))
    && exact(ids, expected) && exact(contracts, app.routes.map(({ id, lanes }) => `${id}:${[...lanes].map(variant).sort().join("+")}`)) && exact(classifications, authored)
    && ANCHORS.every((anchor) => source.includes(anchor));
}

function oracle(fixture: Fixture, sources: Map<string, string>): boolean {
  return fixture.schema === "semio.app.publication-authority.v1" && fixture.apps.length > 0 && fixture.apps.every((app) => appOracle(app, sources.get(app.owner) ?? ""));
}

/** 🗡️ Four source mutations per app, derived from that app's own routes, that MUST break its oracle. */
function hostileSources(app: AppAuthority, source: string): string[] {
  const last = app.routes[app.routes.length - 1]!;
  const authored = app.routes.filter((route) => route.frameworkInjected !== true);
  const second = authored[Math.min(1, authored.length - 1)]!;
  return [
    source.replace(contractPattern(last.id), ""),
    source.replaceAll("DrawingArtifactStorePreparationFactory::default()", ""),
    source.replace(`.action_interactive_job("${second.id}", semio_plugin::InteractiveJobClassification::Migrated)`, ""),
    source.replace("fn register_window_config_owners(", "fn register_window_config_owners_detached("),
  ];
}


/** 🔏️ Proves the current Drawing source authority and canonical field-patch outcomes. */
export async function proveDrawPublicationAuthority(packageRoot: string, subset: string): Promise<void> {
    const plugin = resolve(packageRoot, "../..");
    const authority = resolve(plugin, "🧪️tests/🧫️fixtures/🧪️publication-authority");
    const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
    const ajv = new Ajv({ allErrors: true, strict: true });
    const patchRoot = resolve(subset, "🧪️tests/🧫️fixtures/🧬️mutations/🎛️field-patch");
    const validatePatch = ajv.compile(await Bun.file(resolve(subset, "🧬️schema/🧬️mutations/🎛️field-patch/🧬️schema/🔣️.json")).json());
    const patchCases = await Bun.file(resolve(patchRoot, "🔣️.json")).json() as { patch: unknown; accepted: boolean }[];
    for (const test of patchCases) if (validatePatch(test.patch) !== test.accepted) throw new Error(`Draw field-patch oracle disagrees: ${JSON.stringify(test)}`);
    console.error(`Draw independent Ajv field-patch oracle: ${patchCases.length} cases`);
    const admission = resolve(subset, "../../../../🔨️modules/🏠️host/🧰️owned/🧪️tests/🧫️fixtures/🧮️mutation-admission");
    const admissionFixture = await Bun.file(resolve(admission, "🔣️.json")).json() as { cases: unknown[]; bootstrapYield: BootstrapYield };
    bootstrapYieldOracle(admissionFixture.bootstrapYield);
    const sources = new Map<string, string>();
    const preparation = await Bun.file(join(subset, "✏️editor/💾️store-preparation/🦀️.rs")).text();
    const cold = await Bun.file(resolve(packageRoot, "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs")).text();
    const requireSource = (source: string, owner: string, anchors: string[]) => {
      for (const anchor of anchors) if (!source.includes(anchor)) throw new Error(owner + " lost " + anchor);
    };
    requireSource(preparation, "Draw store preparation", ["store::ColdMutationApplyFactory<DrawingSnapshot, DrawingMutation>", "DrawingCold::default().begin(request, grant)", "ArtifactReplayPreparationFactory::begin(&DrawingCold::default(), request, grant)"]);
    requireSource(cold, "Cold store preparation", ["request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()", "authority.prepare_one_item", "self.owners.begin_close()", "self.owners.close_step", "self.owners.terminal_is_empty()"]);
    for (const app of fixture.apps) {
      const appSources = [app.source, app.manifestSource].map(file => readModuleTreeSource(resolve(plugin, file)));
      sources.set(app.owner, appSources.join("\n"));
      const producer = await Bun.file(resolve(plugin, app.reservedProducerSource)).text();
      requireSource(producer, "Draw clipboard producer", ["request.snapshot_read.duplicate_original(&mut workspace.unsealed", "workspace.sealing.begin(read)", "workspace.sealing.step(&mut workspace.source,grant)", "NativeEncodeControl::resume", "NativeDecodeControl::resume", "DrawingLayerCloneWorkAuthority", "fn close_demands", "fn terminal_is_empty"]);
    }
    if (!oracle(fixture, sources)) throw new Error("Draw publication-authority oracle rejected production");
    let hostile = 0;
    for (const app of fixture.apps) {
      for (const candidate of hostileSources(app, sources.get(app.owner)!)) {
        hostile += 1;
        if (candidate === sources.get(app.owner)) throw new Error(`Draw hostile mutation for ${app.owner} did not change its source`);
        if (appOracle(app, candidate)) throw new Error(`Draw oracle accepted a hostile source mutation for ${app.owner}`);
      }
      const hostileFixture: Fixture = { ...fixture, apps: fixture.apps.map((entry) => (entry.owner === app.owner ? { ...entry, routes: entry.routes.slice(1) } : entry)) };
      hostile += 1;
      if (oracle(hostileFixture, sources)) throw new Error(`Draw accepted a hostile fixture mutation for ${app.owner}`);
    }
    console.error(`validated Draw publication authority; apps=${fixture.apps.map((app) => `${app.owner}:${app.routes.length}`).join(",")}; oracle=owned; hostile=${hostile}; mutationAdmission=${admissionFixture.cases.length}`);
}
