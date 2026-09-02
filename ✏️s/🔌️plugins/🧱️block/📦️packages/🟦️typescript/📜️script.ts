#!/usr/bin/env bun
/** 🧱️ Block source, schema, and publication-authority laws. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
type Fixture = { schema: string; owner: "Block5dPlayApp"; source: string; routes: { id: string; lane: "Artifact" }[]; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

function oracle(fixture: Fixture, source: string): boolean {
  const ids = [...source.match(/BLOCK5D_RETAINED_TOOL_IDS: &\[&str\] = &\[([^\]]*)\]/s)?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((match) => match[1]!);
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[ArtifactToolPublicationLane::Artifact\] \}/g)].map((match) => match[1]!);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = fixture.routes.map(({ id }) => id);
  return fixture.schema === "semio.app.publication-authority.v1" && Object.values(fixture.laws).every(Boolean)
    && fixture.ui.locales.join(",") === "en,de" && fixture.ui.accessibleLabels && fixture.ui.customizableUi
    && exact(ids, expected) && exact(contracts, expected) && exact(classifications, expected)
    && ["ToolExecutionContract::bounded_first_step", "semio_framework_plugin::bounded_first_step_tool_proofs!", "build_artifact_store_one_item_preparation_factory", "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()", "authority.prepare_one_item", "fn cancel(&mut self)", "fn begin_close(&mut self)", "base.return_to_registry()", "fn terminal_is_empty(&self)", "LocalizedLabel::native", "SelectionMode::Multiple", ".default_layout(edit_mode::layout())"].every((anchor) => source.includes(anchor));
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    const plugin = resolve(this.root, "../..");
    const fixture = await Bun.file(resolve(plugin, "🔣️publication-authority.json")).json() as Fixture;
    const schema = await Bun.file(resolve(plugin, "🔣️publication-authority.schema.json")).json();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`Block fixture failed strict Ajv: ${JSON.stringify(validate.errors)}`);
    const source = await Bun.file(resolve(plugin, fixture.source)).text();
    if (!oracle(fixture, source)) throw new Error("Block publication-authority oracle rejected production");
    const hostileSource = [source.replace('ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] },', ""), source.replace("            || request.base_revision != request.authority.base_revision()\n", ""), source.replace('.action_interactive_job("removeGrip", InteractiveJobClassification::Migrated)', "")];
    if (hostileSource.some((candidate) => oracle(fixture, candidate))) throw new Error("Block oracle accepted a hostile source mutation");
    const hostileFixture = { ...fixture, routes: fixture.routes.slice(1) };
    if (validate(hostileFixture) || oracle(hostileFixture, source)) throw new Error("Block accepted a hostile fixture mutation");
    console.error(`validated Block publication authority; routes=${fixture.routes.length}; schema=Ajv; oracle=owned; hostile=3`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
