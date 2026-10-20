/** 💾️ Every inferred target says why a task cache may or may not replay it. The corpus in `./🔣️.json` is language-agnostic data: the repository contract and the independent Ajv implementation must agree on every decision, every synthesized playground target must carry the decision its fixture row names, and the native input plan must keep authored inputs while replacing only the broad project groups. */
import { expect, test } from "bun:test";
import { createRequire } from "node:module";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { parseCatalogBuildContributionV1 } from "@semio-tech/plugin-registry-publication/build";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "@semio-tech/process/routing";
import { frameworkOsEngineSourceOwnersV1 } from "../../../../../../../../💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import cacheSchema from "../../../../🏃️execution/🧬️schema/💾️cache.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const require = createRequire(import.meta.url);
const plugin = await import("../../🟨️.mjs");
await plugin.libraryBootstrap;
const api = plugin.cacheInternals;
const ajv = new Ajv({ strict: true, allErrors: true });
const validateDecision = ajv.compile<ReturnType<typeof api.cacheDecisionV1>>(cacheSchema);
const decisionOf = (target: any): string[] | null => { const value = target?.metadata?.semio?.cache; return value ? [value.class, value.reason] : null; };
const expand = (template: string, variant: string, profile = "") => template.replaceAll("{variant}", variant).replaceAll("{profile}", profile);
const artifacts = () => process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();

test("the corpus and the cache decision schema satisfy the strict independent validator", () => {
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.decisions.map((row) => row.id)).size).toBe(corpus.decisions.length);
  expect(new Set(corpus.nativeInputs.map((row) => row.id)).size).toBe(corpus.nativeInputs.length);
});

test("the repository contract admits exactly the decisions the independent schema admits", () => {
  for (const row of corpus.decisions) {
    const decision: unknown = row.value, admitted = validateDecision(decision);
    expect(admitted, row.id).toBe(row.valid);
    const value = row.value as Record<string, unknown>;
    if (value.version !== 1 || Object.keys(value).sort().join() !== "class,reason,version") continue;
    if (admitted) expect(api.cacheDecisionV1(value.class as string, value.reason as string), row.id).toEqual(decision);
    else expect(() => api.cacheDecisionV1(value.class as string, value.reason as string), row.id).toThrow("Repository target cache decision refused");
  }
  expect(api.cacheDecisionV1("continuous", "serves-until-stopped")).toBe(api.cacheDecisionV1("continuous", "serves-until-stopped"));
});

test("workspace policy refusals carry the physical reason and replayable names carry none", () => {
  for (const row of corpus.policy) {
    const target: any = api.targetPolicy(row.name, { cache: row.authored === false ? false : undefined, ...structuredClone(row.target ?? {}), options: { command: "bun ./📜️script.ts " + row.name, ...(row.target as any)?.options } });
    expect(target.cache, row.name).toBe(row.cache);
    expect(target.continuous === true, row.name).toBe(row.continuous === true);
    expect(decisionOf(target), row.name).toEqual(row.decision);
    if (row.decision) expect(validateDecision(target.metadata.semio.cache), row.name).toBe(true);
  }
});

test("a replayed producer declares its outputs and a target nothing classifies never replays", () => {
  for (const row of corpus.outputsGate) {
    const target: any = api.cachedOutputsAdmittedV1(api.targetPolicy(row.name, structuredClone(row.target)));
    expect(target.cache, row.id).toBe(row.expect.cache);
    expect(decisionOf(target), row.id).toEqual(row.expect.decision);
    if (row.expect.decision) expect(validateDecision(target.metadata.semio.cache), row.id).toBe(true);
  }
  expect(() => api.targetPolicy("unheard-of-kind", { metadata: { semio: { cache: { version: 1, class: "cacheable", reason: "pure-function-of-inputs" } } } }), "an open declared class").toThrow("Repository target cache decision refused");
  expect(() => api.targetPolicy("unheard-of-kind", { metadata: { semio: { cache: { version: 1, class: "deterministic", reason: "pure-function-of-inputs", note: "x" } } } }), "a declared decision with an extra key").toThrow("Repository target cache decision refused");
});

test("the emoji prefix of a target name never hides it from its class", () => {
  for (const [name, bare] of [["🏭️generator-build", "generator-build"], ["⚙️generator", "generator"], ["🧪️test", "test"], ["plain", "plain"]]) expect(api.normalizedTargetName(name), name).toBe(bare);
  for (const name of ["generator-build", "🏭️generator-build", "⚙️generator-manifests"]) expect(decisionOf({ metadata: { semio: { cache: api.targetClassV1(name) } } }), name).toEqual(["side-effecting", "mutates-workspace"]);
  expect(api.targetClassV1("totally-unheard-of")).toBeUndefined();
  expect(api.targetClassV1("a-contract-b")?.reason).toBe("check-without-outputs");
});

test("native targets run tests by contract, by name or through the Cargo test driver", () => {
  for (const row of corpus.nativeRunsTests) expect(api.nativeTargetRunsTestsV1(row.name, row.execution ?? undefined, row.commandSources), row.id).toBe(row.expect);
});

test("generated shared command inputs never count as authored native inputs", () => {
  for (const row of corpus.nativeAuthored) expect(api.nativeAuthoredInputsV1(row.policyInputs, row.commandInputs ?? undefined), row.id).toEqual(row.expect);
});

test("native inputs replace only the broad project groups and keep every authored input", () => {
  for (const row of corpus.nativeInputs) expect(api.nativeTargetInputsV1(row.runsTests, row.generated, row.declared), row.id).toEqual(row.expect);
});

test("every synthesized playground target carries the decision its row names and that decision agrees with its cache flags", async () => {
  const row = corpus.playground, root = await mkdtemp(join(resolve(artifacts()), "pc-")), toml = require("@iarna/toml");
  const contributions = [parseCatalogBuildContributionV1({ version: 1, publication: row.publication, projectRoot: "catalog", project: "catalog", generateTarget: "generate", inputs: [] })];
  const put = async (path: string, text: string) => { await mkdir(dirname(join(root, path)), { recursive: true }); await writeFile(join(root, path), text); };
  const wgpu = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript", flow = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust";
  try {
    await put("Cargo.toml", "[workspace]\nmembers=['components/a','components/b']\nresolver='2'\n");
    await put(wgpu + "/📋️project.json", JSON.stringify({ name: "renderer", targets: { wasm: { configurations: { prototype: { env: { SEMIO_BUILD_MODE: "prototype" } }, release: { env: { SEMIO_BUILD_MODE: "release" } } } } } }));
    await put(flow + "/📋️project.json", JSON.stringify({ name: "flow", targets: { wasm: {} } }));
    for (const [index, owner] of frameworkOsEngineSourceOwnersV1("distribution").entries()) await put(owner + "/📋️project.json", JSON.stringify({ name: "engine" + index, targets: { wasm: {} } }));
    for (const component of row.components) {
      const manifest = { package: { name: component.id, version: "0.0.0", metadata: { component: { package: "semio:" + component.id }, semio: { "component-kind": "plugin", "depends-on": component.dependencies, playground: component.id === "a" ? row.variants.map((variant) => ({ variant: variant.name, ports: { wgpu: 12345 }, ...("distDir" in variant ? { distDir: variant.distDir } : {}) })) : [] } } }, dependencies: Object.fromEntries(component.dependencies.map((id) => [id, { path: "../" + id }])) };
      await put("components/" + component.id + "/Cargo.toml", toml.stringify(manifest));
    }
    const files = row.components.map((component) => "components/" + component.id + "/Cargo.toml");
    const targets = await api.playgroundPreparationTargets(files, root, row.owner, { generator: row.generator }, contributions, { workspaceRoot: root, preparationControl: { advance: async () => {} } });
    const expected = new Map<string, any>();
    for (const variant of row.variants) {
      for (const template of row.perProfile) for (const profile of row.profiles) expected.set(expand(template.name, variant.name, profile), template);
      for (const template of row.perVariant) expected.set(expand(template.name, variant.name), template);
    }
    if (row.variants.some((variant) => "distDir" in variant)) for (const template of row.once) expected.set(template.name, template);
    expect(Object.keys(targets).sort()).toEqual([...expected.keys()].sort());
    for (const [name, template] of expected) {
      const target: any = targets[name];
      expect(decisionOf(target), name).toEqual([template.class, template.reason]);
      expect(validateDecision(target.metadata.semio.cache), name).toBe(true);
      expect(target.cache, name).toBe(template.cache);
      expect(target.continuous === true, name).toBe(template.continuous);
      expect(target.cache === true, name).toBe(target.metadata.semio.cache.class === "deterministic");
      expect(target.continuous === true, name).toBe(target.metadata.semio.cache.class === "continuous");
      expect(target.metadata.semio.execution, name).toBeDefined();
    }
    const sessions = await api.playgroundSessionTargets(files, root, contributions);
    expect(Object.keys(sessions).sort()).toEqual(row.variants.map((variant) => "session-" + variant.name).sort());
    for (const [name, target] of Object.entries<any>(sessions)) {
      expect(decisionOf(target), name).toEqual([row.sessions.class, row.sessions.reason]);
      expect(target.cache, name).toBe(row.sessions.cache);
    }
    const catalog = api.collectPlaygroundCatalog(files, root, contributions), sites = api.pluginSiteTargetsForCrate("components/a", catalog, "staging", root);
    expect(Object.keys(sites).sort()).toEqual(["build", ...row.variants.map((variant) => "build-" + variant.name + "-site")].sort());
    for (const [name, target] of Object.entries<any>(sites)) {
      expect(decisionOf(target), name).toEqual([row.sites.class, row.sites.reason]);
      expect(target.cache, name).toBe(row.sites.cache);
    }
  } finally { await rm(root, { recursive: true, force: true }); }
}, 30000);

test("artifact packages replay their tests, checks and receipted store-linked build", () => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "inferred-cache-artifact", maximumElapsedMilliseconds: 30000 }, {}, Date.now()), async (invocation) => {
  const row = corpus.artifact, root = await mkdtemp(join(resolve(artifacts()), "pa-"));
  try {
    for (const [path, text] of Object.entries(row.files)) { await mkdir(dirname(join(root, path)), { recursive: true }); await writeFile(join(root, path), text); }
    const json = { name: "art", tags: row.tags, targets: row.targets };
    await writeFile(join(root, row.root, "📋️project.json"), JSON.stringify(json));
    const project = await plugin.inferReadOnlyBuildProjectSourcesV1(json, row.root, root, invocation, createIssuedResourceControlV1(invocation, "graph:project-source", 0));
    for (const [name, expected] of Object.entries(row.expect)) {
      const target = project.targets[name] as any;
      expect(target.cache, name).toBe(expected.cache);
      expect(decisionOf(target), name).toEqual(expected.decision);
      const outputs = (expected as { outputs?: readonly string[] }).outputs; if (outputs) expect(target.outputs, name).toEqual(outputs);
      if (expected.decision) expect(validateDecision(target.metadata.semio.cache), name).toBe(true);
    }
    const test = project.targets.test as any;
    for (const group of ["artifactSources", "artifactCommandSources"]) expect(test.inputs, group).toContain(group);
    expect(test.inputs).toContainEqual({ env: "SEMIO_BUILD_MODE" });
    expect((project.targets.build as any).inputs).toContainEqual({ env: "SEMIO_BUILD_MODE" });
  } finally { await rm(root, { recursive: true, force: true }); }
}), 30000);
