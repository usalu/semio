#!/usr/bin/env bun
/** 🧱️ Block source, schema, and publication-authority laws. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
type Lane = "HostOnly" | "Artifact" | "Config" | "Draft" | "Presence" | "Transient" | "Child";
type AppAuthority = { owner: string; toolIdsConstant: string; source: string; routes: { id: string; lane: Lane }[]; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };
type Fixture = { schema: string; apps: AppAuthority[] };

/** 🧱️ Every anchor an app's retained/bounded publication apparatus must carry verbatim. */
const ANCHORS = [
  "ToolExecutionContract::bounded_first_step",
  "semio_framework_plugin::bounded_first_step_tool_proofs!",
  "factory_type:",
  "build_artifact_store_one_item_preparation_factory",
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
  ".default_layout(edit_mode::layout())",
];

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

/** ⚖️ One app's routes must be the same set in the Rust tool-id constant, the publication contracts and the `Migrated` classifications. */
function appOracle(app: AppAuthority, source: string): boolean {
  const ids = [...source.match(new RegExp(`${app.toolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((match) => match[1]!);
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[ArtifactToolPublicationLane::(\w+)\] \}/g)].map((match) => `${match[1]}:${match[2]}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = app.routes.map(({ id }) => id);
  return Object.values(app.laws).every(Boolean)
    && app.ui.locales.join(",") === "en,de" && app.ui.accessibleLabels && app.ui.customizableUi
    && exact(ids, expected) && exact(contracts, app.routes.map(({ id, lane }) => `${id}:${lane}`)) && exact(classifications, expected)
    && ANCHORS.every((anchor) => source.includes(anchor));
}

function oracle(fixture: Fixture, sources: Map<string, string>): boolean {
  return fixture.schema === "semio.app.publication-authority.v1" && fixture.apps.length > 0 && fixture.apps.every((app) => appOracle(app, sources.get(app.owner) ?? ""));
}

/** 🗡️ Three source mutations per app, derived from that app's own routes, that MUST break its oracle. */
function hostileSources(app: AppAuthority, source: string): string[] {
  const last = app.routes[app.routes.length - 1]!;
  const second = app.routes[Math.min(1, app.routes.length - 1)]!;
  return [
    source.replace(`ArtifactToolPublicationContract { tool_id: "${last.id}", lanes: &[ArtifactToolPublicationLane::${last.lane}] },`, ""),
    source.replace("request.base_revision != request.authority.base_revision()", ""),
    source.replace(`.action_interactive_job("${second.id}", InteractiveJobClassification::Migrated)`, ""),
  ];
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    const plugin = resolve(this.root, "../..");
    const authority = resolve(plugin, "🧪️publication-authority");
    const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
    const schema = await Bun.file(resolve(authority, "🧬️.schema.json")).json();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`Block fixture failed strict Ajv: ${JSON.stringify(validate.errors)}`);
    const sources = new Map<string, string>();
    for (const app of fixture.apps) sources.set(app.owner, await Bun.file(resolve(plugin, app.source)).text());
    if (!oracle(fixture, sources)) throw new Error("Block publication-authority oracle rejected production");
    let hostile = 0;
    for (const app of fixture.apps) {
      for (const candidate of hostileSources(app, sources.get(app.owner)!)) {
        hostile += 1;
        if (candidate === sources.get(app.owner)) throw new Error(`Block hostile mutation for ${app.owner} did not change its source`);
        if (appOracle(app, candidate)) throw new Error(`Block oracle accepted a hostile source mutation for ${app.owner}`);
      }
      const hostileFixture: Fixture = { ...fixture, apps: fixture.apps.map((entry) => (entry.owner === app.owner ? { ...entry, routes: entry.routes.slice(1) } : entry)) };
      hostile += 1;
      if (oracle(hostileFixture, sources)) throw new Error(`Block accepted a hostile fixture mutation for ${app.owner}`);
    }
    console.error(`validated Block publication authority; apps=${fixture.apps.map((app) => `${app.owner}:${app.routes.length}`).join(",")}; schema=Ajv; oracle=owned; hostile=${hostile}`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
