import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import { runInNewContext } from "node:vm";

/** 🎮️ Checks one shared discovery snapshot per call against a language-neutral fixture and lodash. */
export async function testPlaygroundInputView(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")), ts = require("typescript");
  const registry = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
  const source = ts.createSourceFile("playgrounds.ts", readFileSync(join(registry, "🎮️playground/🔎️discovery/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const definition = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "generatePlaygroundRegistry");
  const code = ts.transpileModule(definition.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const rows = fixture.variants.map((variant: string) => ({ variant, cratePath: "package", app: variant }));
  for (const row of fixture.cases) {
    let created = 0;
    const view = { snapshot: "fixture" }, packages = ["fixture package"], checkView = (actual: unknown) => assert.equal(actual, view, "Every reader must receive the same bounded input view");
    const run = runInNewContext(`${code}; generatePlaygroundRegistry;`, {
      join, TAXONOMY: {},
      registryCatalogInputView: () => { created++; return view; },
      generatePluginRegistry: (_root: string, options: any) => { checkView(options.view); assert.equal(options.packages, packages); return [{ cratePath: "package", pluginId: "fixture" }]; },
      parseAssetsForCrate: (_path: string, _root: string, input: unknown) => { checkView(input); return []; },
      readDescriptorJson: (_root: string, _crate: string, input: unknown) => { checkView(input); return {}; },
      parsePlaygroundsForCrate: (_path: string, _plugin: string, _crate: string, _root: string, input: unknown) => { checkView(input); return rows; },
      declaredExampleIdsForPlayground: () => new Set(),
      discoverExamplesForPlayground: (_root: string, _crate: string, _declared: unknown, input: unknown) => { checkView(input); return ["example"]; }
    });
    const result = run(workspace, { packages, ...(row.provided ? { view } : {}) });
    assert.equal(created, row.created);
    const expected = require("lodash/sortBy")(rows, "variant").map((entry: object) => ({ ...entry, examples: ["example"], assets: [] }));
    assert.deepEqual(JSON.parse(JSON.stringify(result)), expected);
  }
  console.log("[DEBUG] Playground discovery shares one input view across all readers and preserves explicit snapshots; catalog rows match lodash PASS");
  const pluginSource = ts.createSourceFile("plugins.ts", readFileSync(join(registry, "🔎️discovery/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const resolver = pluginSource.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveRegistryPluginIdsForFilter");
  const resolverCode = ts.transpileModule(resolver.getText(pluginSource).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const { runtimeComponentClosure } = await import("../../../🕸️dependencies/🧩️runtime/🟨️.mjs");
  const rejectLiveRead = () => { throw new Error("Projected filtering must not rediscover the live repository"); };
  const filter = runInNewContext(`${resolverCode}; resolveRegistryPluginIdsForFilter;`, { runtimeComponentClosure, resolveRegistryPluginIdForFilter: rejectLiveRead, generatePluginRegistry: rejectLiveRead });
  const closure = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/runtime-components/🔣️.json"), "utf8"));
  for (const row of closure.cases.filter((row: { roots: string[] }) => row.roots.length === 1)) {
    assert.deepEqual(Array.from(filter(row.roots[0], closure.components, [])), row.expected);
    const playgrounds = [{ variant: "fixture-variant", aliases: ["fixture-alias"], pluginId: row.roots[0] }];
    for (const name of ["fixture-variant", "fixture-alias"]) assert.deepEqual(Array.from(filter(name, closure.components, playgrounds)), row.expected);
  }
  assert.deepEqual(Array.from(filter("unknown-plugin", closure.components, [])), []);
  console.log("[DEBUG] Projected plugin IDs, variants, aliases and unknown filters use the supplied catalog without any live discovery PASS");
}
