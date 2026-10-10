#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🖍️ Draw example twins plus the publication-authority law: every dispatchable route is declared once, in every place the framework joins. */
import { join, resolve } from "node:path";
import { runOwnedCommand } from "../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type Lane = "artifact" | "config" | "draft" | "presence" | "transient" | "window-config" | "window-transient" | "child" | "interaction" | "host-only";
type Route = { id: string; lanes: Lane[]; frameworkInjected?: true };
type AppAuthority = { owner: string; toolIdsConstants: string[]; source: string; routes: Route[]; reservedToolIdsConstant:string; reservedRoutes:Route[]; reservedProducerSource:string; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
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
  "semio_framework_plugin::bounded_first_step_tool_proofs!",
  "factory_type:",
  "ToolExecutionContract::bounded_first_step",
  "ToolExecutionContract::resumable",
  "build_artifact_store_one_item_preparation_factory",
  "fn register_window_config_owners(",
  "fn register_window_transient_owners(",
  "register_tool_job_factories",
  "build_tool_job",
  "request.operation != request.authority.operation()",
  "request.generation != request.authority.generation()",
  "request.base_revision != request.authority.base_revision()",
  "authority.prepare_one_item",
  "fn cancel(&mut self)",
  "fn begin_close(&mut self)",
  "base.return_to_registry()",
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
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", (?:semio_framework_plugin::)?InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
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
    source.replaceAll("request.base_revision != request.authority.base_revision()", ""),
    source.replace(`.action_interactive_job("${second.id}", semio_framework_plugin::InteractiveJobClassification::Migrated)`, ""),
    source.replace("fn register_window_config_owners(", "fn register_window_config_owners_detached("),
  ];
}

/** 🔎️ Strict checking for the owned path raster pipeline and its framework consumers. */
function typecheckPathRaster(repoRoot:string,raster:string,ownedSources:string[]=[]):void {
  runCmd(process.execPath, [join(repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", join(raster, "🟦️.ts"),...ownedSources], {cwd:repoRoot});
}

class TestScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    const subset = join(this.repoRoot, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any");
    runCmd(process.execPath,["test",join(subset,"🚪️io/📝️text/🔺️diff/🧪️tests/🔬️unit/🟦️.ts")]);
    await (await import(join(subset,"🚪️io/🖼️image/🧪️tests/🟦️.ts"))).testDrawingImageAdmission();
    const raster=join(subset,"🧬️schema/🧮️geometry/📷️raster");
    const sceneRaster=join(subset,"🧬️schema/🎬️scene/📷️raster");
    const scenePrepare=join(subset,"🧬️schema/🎬️scene/📋️prepare");
    const sceneBooleans=join(subset,"🧬️schema/🎬️scene/🔀️booleans");
    const sceneTrace=join(subset,"🧬️schema/🎬️scene/🔍️trace");
    const sceneRetire=join(subset,"🧬️schema/🎬️scene/🧹️retire");
    const sceneIdentity=join(subset,"🧬️schema/🎬️scene/🪪️identity");
    const sceneView=join(subset,"🧬️schema/🎬️scene/👁️view");
    const scenePaint=join(subset,"🧬️schema/🎬️scene/🎨️paint");
    const scenePicking=join(scenePaint,"📋️prepare/🎯️query");
    const selectionStatus=join(subset,"✏️editor/🧮️status");
    if(segments.length) {
      if(segments.length===1&&segments[0]==="font-outline") {const font=join(this.repoRoot,"🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font");runCmd(process.execPath,["test",join(font,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,font);return;}
      if(segments.length===1&&segments[0]==="pdf-write") {const writer=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write");runCmd(process.execPath,["test","--timeout","120000",join(writer,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,writer);return;}
      if(segments.length===1&&segments[0]==="svg-write") {const writer=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write");runCmd(process.execPath,["test",join(writer,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,writer);return;}
      if(segments.length===1&&segments[0]==="png-export") {const exported=join(subset,"✏️editor/🎮️commands/📤️export-document");runCmd(process.execPath,["test",join(exported,"🧪️tests/🔬️unit/🟦️.ts")]);typecheckPathRaster(this.repoRoot,exported);return;}
      if(segments.length===1&&segments[0]==="image-import") {const imported=join(subset,"✏️editor/🎮️commands/📥️import-image");runCmd(process.execPath,["test",join(imported,"🧪️tests/🔬️unit/🟦️.ts")]);typecheckPathRaster(this.repoRoot,imported);return;}
      if(segments.length===1&&segments[0]==="shape-coordinates") {
        const coordinates=join(subset,"🧬️schema/🔷️shape/✏️coordinates");
        runCmd(process.execPath,["test",join(coordinates,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,coordinates);
        return;
      }
      if(segments.length===1&&segments[0]==="field-history") {
        const patch=join(subset,"✏️editor/🎮️commands/🩹️patch-layer");
        runCmd(process.execPath,["test",join(patch,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,patch);
        return;
      }
      if(segments.length===1&&segments[0]==="affine-rendering") {
        const affine=join(this.repoRoot,"🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/↗️affine");
        runCmd(process.execPath,["test",join(affine,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,affine);
        return;
      }
      if(segments.length===1&&segments[0]==="path-editing") {
        const editing=join(subset,"🧬️schema/🧮️geometry/✏️editing");
        runCmd(process.execPath,["test",join(editing,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,editing);
        return;
      }
      if(segments.length===1&&segments[0]==="clipboard"){
        const clipboard=join(subset,"✏️editor/📋️clipboard");
        runCmd(process.execPath,["test",join(clipboard,"🧪️tests/🔬️unit/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🔀️combine-boolean/🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,clipboard);
        return;
      }
      if(segments.length===1&&segments[0]==="scene-placement"){
        const placement=join(subset,"🧬️schema/🎬️scene/📍️placement");
        runCmd(process.execPath,["test","--timeout","120000",join(placement,"🧪️tests/🔬️unit/🟦️.ts")]);
        typecheckPathRaster(this.repoRoot,placement,[join(scenePrepare,"🟦️.ts")]);
        return;
      }      if(segments.length===1&&segments[0]==="image-host"){runCmd(process.execPath,["test","--timeout","120000",join(subset,"../../../../🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts")]);return;}
      if(segments.length!==1||!['path-raster','scene-raster','scene-paint','scene-picking','selection-status','image-output'].includes(segments[0]!)) throw Error("Unknown Draw test selection "+segments.join(" "));
      const selected=segments[0]==='image-output'?join(subset,'🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any'):segments[0]==='scene-raster'?sceneRaster:segments[0]==='scene-paint'?scenePaint:segments[0]==='scene-picking'?scenePicking:segments[0]==='selection-status'?selectionStatus:raster;
      runCmd(process.execPath,["test","--timeout","120000",join(selected,"🧪️tests/🔬️unit/🟦️.ts"),...(selected===scenePaint?[join(scenePaint,"📋️prepare/🧪️tests/🔬️unit/🟦️.ts")]:[]),...(selected===sceneRaster?[join(sceneRaster,"🧪️tests/🔬️unit/🖼️images/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🖼️assets/🟦️.ts"),join(scenePrepare,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneBooleans,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneTrace,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRetire,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneIdentity,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneView,"🧪️tests/🔬️unit/🟦️.ts")]:[])]);
      typecheckPathRaster(this.repoRoot,selected,selected===scenePaint?[join(scenePaint,"📋️prepare/🟦️.ts")]:selected===sceneRaster?[join(subset,"🚪️io/🖼️image/🟦️.ts"),join(scenePrepare,"🟦️.ts"),join(sceneBooleans,"🟦️.ts"),join(sceneTrace,"🟦️.ts"),join(sceneRetire,"🟦️.ts"),join(sceneIdentity,"🟦️.ts"),join(sceneIdentity,"🚦️admission/🟦️.ts"),join(sceneView,"🟦️.ts"),join(scenePaint,"🟦️.ts"),join(scenePaint,"📋️prepare/🟦️.ts"),join(scenePaint,"📋️prepare/🎯️query/🟦️.ts")]:[]);
      return;
    }
    runCmd(process.execPath,["test",join(subset,"✏️editor/📋️clipboard/🧪️tests/🔬️unit/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🔀️combine-boolean/🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,join(subset,"✏️editor/📋️clipboard"));
    runCmd(process.execPath,["test",join(selectionStatus,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,selectionStatus);
    runCmd(process.execPath, ["test","--timeout","120000", join(scenePrepare,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneBooleans,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneTrace,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRetire,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneIdentity,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneView,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🖼️images/🟦️.ts"),join(sceneRaster,"🧪️tests/🔬️unit/🖼️assets/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/📷️raster/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "../🎨️style/🧬️schema/🧬️mutations/🧩️set-group-isolation/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/📄️document/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/↗️transform/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🎨️fill/🌀️rule/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🛤️path/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🎯️picking/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🎛️handles/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/↗️affine/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🟦️.ts"), join(subset, "../🎨️style/🧬️schema/🧬️mutations/📝️update-text/🧪️tests/🔬️unit/🟦️.ts"), join(subset,"../🎨️style/🧬️schema/🧬️mutations/🖼️update-image/🧪️tests/🔬️unit/🟦️.ts"), join(subset,"../🔀️transform/🧬️schema/🧬️mutations/🔷️shape-coordinate/🧪️tests/🔬️unit/🟦️.ts"), join(subset,"🧬️schema/🔷️shape/✏️coordinates/🧪️tests/🔬️unit/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🩹️patch-layer/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "👁️viewer/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/📷️framing/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️window/🟦️.ts"), join(subset, "✏️editor/🎮️commands/➕️add-layer/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/↔️translation/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🎨️fill/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🖊️stroke/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🎯️selection/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🧪️tests/🔬️canvas-tool/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️drag-layers/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️rotate-layers/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️scale-layers/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️drag-path-points/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/✏️editing/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "../🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "✏️editor/🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🟦️.ts"), join(subset, "🧬️schema/🧮️geometry/🧪️tests/📐️bounds/🟦️.ts"), join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,raster,[join(subset,"🚪️io/🖼️image/🟦️.ts"),join(scenePrepare,"🟦️.ts"),join(sceneBooleans,"🟦️.ts"),join(sceneTrace,"🟦️.ts"),join(sceneRetire,"🟦️.ts"),join(sceneIdentity,"🟦️.ts"),join(sceneIdentity,"🚦️admission/🟦️.ts"),join(sceneView,"🟦️.ts"),join(scenePaint,"🟦️.ts"),join(scenePaint,"📋️prepare/🟦️.ts"),join(scenePaint,"📋️prepare/🎯️query/🟦️.ts"),join(sceneRaster,"🟦️.ts"),join(subset,"🧬️schema/🟦️.ts"),join(subset,"🧬️schema/🧮️geometry/🎯️picking/🎨️paint/🟦️.ts"),join(subset,"🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"),join(subset,"../🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🦠️mutation/🟦️.ts"),join(subset,"../🎨️style/🧬️schema/🧬️mutations/🖼️update-image/🦠️mutation/🟦️.ts"),join(subset,"../🔀️transform/🧬️schema/🧬️mutations/🔷️shape-coordinate/🦠️mutation/🟦️.ts"),join(subset,"✏️editor/🎮️commands/🩹️patch-layer/🟦️.ts")]);
    runCmd(process.execPath,["test",join(subset,"✏️editor/🎮️commands/📥️import-image/🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,join(subset,"✏️editor/🎮️commands/📥️import-image"));
    const placement=join(subset,"🧬️schema/🎬️scene/📍️placement");
    runCmd(process.execPath,["test","--timeout","120000",join(placement,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,placement);    const exported=join(subset,"✏️editor/🎮️commands/📤️export-document");
    runCmd(process.execPath,["test",join(exported,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,exported);
    const pdfWriter=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write");runCmd(process.execPath,["test","--timeout","120000",join(pdfWriter,"🧪️tests/🟦️.ts")]);typecheckPathRaster(this.repoRoot,pdfWriter);
    const svgWriter=join(subset,"🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write");
    runCmd(process.execPath,["test",join(svgWriter,"🧪️tests/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,svgWriter);
    const affine=join(this.repoRoot,"🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/↗️affine");
    runCmd(process.execPath,["test",join(affine,"🧪️tests/🔬️unit/🟦️.ts")]);
    typecheckPathRaster(this.repoRoot,affine);
    await proveDrawPublicationAuthority(this.root, subset);
  }
}
/** 🔏️ Proves the current Drawing source authority and canonical field-patch outcomes. */
async function proveDrawPublicationAuthority(packageRoot: string, subset: string): Promise<void> {
    const plugin = resolve(packageRoot, "../..");
    const authority = resolve(plugin, "🧫️fixtures/🧪️publication-authority");
    const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
    const ajv = new Ajv({ allErrors: true, strict: true });
    const patchRoot = resolve(subset, "🧬️schema/🧬️mutations/🧫️fixtures/🎛️field-patch");
    const validatePatch = ajv.compile(await Bun.file(resolve(subset, "🧬️schema/🧬️mutations/🎛️field-patch/🧬️schema/🔣️.json")).json());
    const patchCases = await Bun.file(resolve(patchRoot, "🔣️.json")).json() as { patch: unknown; accepted: boolean }[];
    for (const test of patchCases) if (validatePatch(test.patch) !== test.accepted) throw new Error(`Draw field-patch oracle disagrees: ${JSON.stringify(test)}`);
    console.error(`Draw independent Ajv field-patch oracle: ${patchCases.length} cases`);
    const admission = resolve(subset, "../../../../🔨️modules/🏠️host/🧰️owned/🧫️fixtures/🧮️mutation-admission");
    const admissionFixture = await Bun.file(resolve(admission, "🔣️.json")).json() as { cases: unknown[]; bootstrapYield: BootstrapYield };
    bootstrapYieldOracle(admissionFixture.bootstrapYield);
    const sources = new Map<string, string>();
    for (const app of fixture.apps) {sources.set(app.owner, await Bun.file(resolve(plugin, app.source)).text());const producer=await Bun.file(resolve(plugin,app.reservedProducerSource)).text();for(const anchor of ["source:request.snapshot_read","NativeEncodeControl::resume","NativeDecodeControl::resume","DrawingLayerCloneWorkAuthority","fn next_close_capacity_byte_demand","fn terminal_is_empty"]){if(!producer.includes(anchor))throw new Error(`Draw reserved producer lost ${anchor}`);}console.error(`[DEBUG] Draw reserved clipboard original lease, codecs and close facets present: ${app.reservedRoutes.map(route=>route.id).join(",")}`);}
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
class PublicationAuthorityAuditScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("publication-authority-audit accepts no arguments");
    await proveDrawPublicationAuthority(this.root, resolve(this.root, "../../🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any"));
  }
}
class UtilityActionPolicyScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw Error("utility-action-policy accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧪️tests/🟦️.ts")],this.repoRoot,"draw-utility-action-policy",45000,{env:process.env});}
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", PublicationAuthorityAuditScript).register("utility-action-policy",UtilityActionPolicyScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
