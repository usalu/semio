#!/usr/bin/env bun
/** 🖍️ Draw example twins plus the publication-authority law: every dispatchable route is declared once, in every place the framework joins. */
import { join, resolve } from "node:path";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type Lane = "artifact" | "config" | "draft" | "presence" | "transient" | "window-config" | "window-transient" | "child" | "interaction" | "host-only";
type Route = { id: string; lanes: Lane[]; frameworkInjected?: true };
type AppAuthority = { owner: string; toolIdsConstants: string[]; source: string; routes: Route[]; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
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
  console.error(`[DEBUG] Drawing bootstrap third-party React Scheduler oracle: ${stages.join("→")}; allocations before resume=${law.expectedAllocationDelta}`);
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
  return Object.values(app.laws).every(Boolean)
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

class TestScript extends BundleScript {
  async run(): Promise<void> {
    const subset = join(this.repoRoot, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any");
    runCmd(process.execPath, ["test", join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")]);
    const plugin = resolve(this.root, "../..");
    const authority = resolve(plugin, "🧫️fixtures/🧪️publication-authority");
    const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
    const module = await Bun.file(resolve(plugin, "🧬️schema", "🔣️.json")).json() as { $id: string };
    const ajv = new Ajv({ allErrors: true, strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    ajv.addSchema(module);
    const validate = ajv.compile({ $ref: `${module.$id}#/$defs/DrawPublicationAuthority` });
    if (!validate(fixture)) throw new Error(`Draw fixture failed strict Ajv: ${JSON.stringify(validate.errors)}`);
    const admission = resolve(subset, "🧬️schema/🧰️owned/🧫️fixtures/🧮️mutation-admission");
    const admissionSchema = await Bun.file(resolve(admission, "🧬️schema/🔣️.json")).json() as { $id: string };
    const admissionFixture = await Bun.file(resolve(admission, "🔣️.json")).json() as { cases: unknown[]; bootstrapYield: BootstrapYield };
    ajv.addSchema(admissionSchema);
    const validateAdmission = ajv.getSchema(admissionSchema.$id);
    if (!validateAdmission?.(admissionFixture)) throw new Error(`Drawing mutation-admission fixture failed strict Ajv: ${JSON.stringify(validateAdmission?.errors)}`);
    bootstrapYieldOracle(admissionFixture.bootstrapYield);
    const sources = new Map<string, string>();
    for (const app of fixture.apps) sources.set(app.owner, await Bun.file(resolve(plugin, app.source)).text());
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
    console.error(`validated Draw publication authority; apps=${fixture.apps.map((app) => `${app.owner}:${app.routes.length}`).join(",")}; schema=Ajv; oracle=owned; hostile=${hostile}; mutationAdmission=${admissionFixture.cases.length}`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
