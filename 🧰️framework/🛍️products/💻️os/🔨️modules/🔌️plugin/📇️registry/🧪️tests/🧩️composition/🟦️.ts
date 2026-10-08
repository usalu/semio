import { devServeCommandV1 } from "../../../../🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import graphlib from "graphlib";
import { resolveFrameworkOsCatalogConfigurationV1 } from "../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧩️catalog/🟦️.ts";
import { prepareWgpuPluginModules, assertWgpuPluginPlan } from "../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️plugin-modules/🟦️.ts";
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🧩️composition/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import { admitPluginCatalogV1, pluginCatalogV1, type PluginCatalogAdmissionV1 } from "../../🟦️.ts";
import { artifactKindActivationOwner, resolvePlaygroundBoot, resolvePluginHostConfig, extensionRegistryFromCatalog } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
const authority = (changes: Partial<PluginCatalogAdmissionV1> = {}): PluginCatalogAdmissionV1 => ({ maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: 30000, now: () => 0, cancelled: () => false, progress: () => {}, ...changes });
const validate = new Ajv({ strict: false }).compile(schema);
const synthetic = () => structuredClone(fixture.cases[1]!.input);
describe("injected catalog portable laws", () => {
  for (const row of fixture.cases) test(row.id, () => {
    const oracle = validate(row.input);
    let admitted = true;
    try { admitPluginCatalogV1(row.input, authority()); } catch { admitted = false; }
    expect(oracle).toBe(row.valid);
    expect(admitted).toBe(oracle);
  });
  test("empty inventory produces no fabricated identity or artifact owner", () => {
    const catalog = pluginCatalogV1(admitPluginCatalogV1(fixture.cases[0]!.input, authority()));
    expect(resolvePlaygroundBoot(catalog, "").plugins).toEqual([]);
    expect(artifactKindActivationOwner(catalog, "custom.core")).toBeUndefined();
    expect(() => catalog.moduleUrl("custom.core")).toThrow();
  });
  test("all receiving metadata and exact module address survive composition", () => {
    const catalog = pluginCatalogV1(admitPluginCatalogV1(synthetic(), authority()));
    expect(resolvePlaygroundBoot(catalog, "canvas").plugins.map(row => row.pluginId)).toEqual(fixture.cases[1]!.expectedPlugins);
    expect(resolvePlaygroundBoot(catalog, "canvas").defaultAppId).toBe("canvas");
    expect(resolvePluginHostConfig(catalog, "studio")?.hostAppId).toBe("studio");
    expect(resolvePluginHostConfig(catalog, "canvas")).toBeUndefined();
    expect(artifactKindActivationOwner(catalog, "custom.core")).toBe("custom.core");
    expect(extensionRegistryFromCatalog(catalog)[0]?.pluginId).toBe("custom.extension");
    expect(catalog.moduleUrl("custom.core")).toBe("https://example.test/core.js?version=1");
    expect(catalog.plugins[1]?.consumes).toEqual(["tools"]);
  });
  test("mounts retain independent admitted immutable inventories", () => {
    const input = synthetic();
    const one = pluginCatalogV1(admitPluginCatalogV1(input, authority()));
    input.targets[0]!.moduleUrl = "https://example.test/second.js";
    const two = pluginCatalogV1(admitPluginCatalogV1(input, authority()));
    expect(one.moduleUrl("custom.core")).not.toBe(two.moduleUrl("custom.core"));
    expect(Object.isFrozen(one.plugins[0]?.consumes)).toBe(true);
  });
  test("identity references and alias collisions are refused", () => {
    for (const mutation of [(v: any) => v.targets.push(v.targets[0]), (v: any) => v.targets[0].dependsOn.push("absent"), (v: any) => v.hosts[0].pluginId = "absent", (v: any) => v.playgrounds[1].aliases.push("studio"), (v: any) => v.targets[0].moduleUrl = "https://user:pass@example.test/core.js"]) {
      const input = synthetic(); mutation(input);
      expect(() => admitPluginCatalogV1(input, authority())).toThrow();
    }
  });
  test("finite authority cancellation deadline bytes rows edges work and progress", () => {
    for (const change of [{ cancelled: () => true }, { deadlineMs: 0 }, { maxBytes: 1 }, { maxRows: 1 }, { maxEdges: 1 }, { maxWork: 1 }, { maxWork: Infinity }]) expect(() => admitPluginCatalogV1(synthetic(), authority(change))).toThrow();
    const progress: number[] = [];
    admitPluginCatalogV1(synthetic(), authority({ progress: value => progress.push(value.completed) }));
    expect(progress.length).toBeGreaterThan(0);
    expect(progress.at(-1)).toBe(6);
    let cancelled = false;
    expect(() => admitPluginCatalogV1(synthetic(), authority({ cancelled: () => cancelled, progress: () => { cancelled = true; } }))).toThrow();
  });
});

describe("catalog worker receiving laws", () => {
  for (const row of fixture.emptyPlans) test(`empty plan ${row.selection}`, async () => {
    const catalog = pluginCatalogV1(admitPluginCatalogV1(fixture.cases[0]!.input, authority()));
    let reads = 0;
    const prepared = await prepareWgpuPluginModules(catalog, [], { readDescriptor: async () => { reads++; throw new Error("empty inventory read descriptor"); } });
    const plan = resolvePlaygroundBoot(catalog, row.variant);
    let accepted = true;
    try { assertWgpuPluginPlan(plan, prepared, row.selection as "all" | "variant"); } catch { accepted = false; }
    expect(accepted).toBe(row.valid);
    expect(reads).toBe(0);
  });
  test("descriptor admission deadline is checked before fetch", async () => {
    const catalog = pluginCatalogV1(admitPluginCatalogV1(fixture.cases[0]!.input, authority()));
    let reads = 0;
    await expect(prepareWgpuPluginModules(catalog, [{ pluginId: "custom.core", moduleUrl: "https://example.test/core.js" }], { deadlineMs: 0, now: () => 0, readDescriptor: async () => { reads++; throw new Error("unexpected fetch"); } })).rejects.toThrow("deadline");
    expect(reads).toBe(0);
  });
});

describe("real React catalog configuration laws", () => {
  test("empty and simultaneous synthetic mounts use their own admitted inventories", () => {
    const idle = resolveFrameworkOsCatalogConfigurationV1(fixture.cases[0]!.input, undefined, authority());
    expect(idle.idle).toBe(true);
    expect(idle.boot.plugins).toEqual([]);
    const one = resolveFrameworkOsCatalogConfigurationV1(synthetic(), "canvas", authority());
    const changed = synthetic(); changed.targets[0]!.moduleUrl = "https://example.test/second.js";
    const two = resolveFrameworkOsCatalogConfigurationV1(changed, "canvas", authority());
    expect(one.catalog.moduleUrl("custom.core")).not.toBe(two.catalog.moduleUrl("custom.core"));
    const graph = new graphlib.Graph();
    for (const row of synthetic().targets) graph.setNode(row.pluginId);
    for (const row of synthetic().targets) for (const dependency of row.dependsOn) graph.setEdge(dependency, row.pluginId);
    expect(one.boot.plugins.map(row => row.pluginId)).toEqual(graphlib.alg.topsort(graph));
    expect(() => resolveFrameworkOsCatalogConfigurationV1(fixture.cases[0]!.input, "absent", authority())).toThrow();
  });
});

test("the actual WGPU serve process receives caller-owned composition authority", () => {
  const requested = { port: 6550, variant: "custom", renderer: "wgpu" as const, profile: "dev" as const, hubUrl: null, logPath: "fixture.log", compositionConfigPath: "fixtures/independent-composition.ts" };
  expect(devServeCommandV1(requested).args).toEqual(["serve", "custom", "dev", "--port", "6550", "--composition", requested.compositionConfigPath]);
});


test("injected metadata preserves capability authority and exact extension parent", () => {
 const rows = admitPluginCatalogV1(synthetic(), authority()), catalog = pluginCatalogV1(rows);
 expect(catalog.plugins[0].capabilities).toEqual(["media.render"]);
 expect(catalog.extensions[0].extends).toBe("custom.host");
});


test("canonical injected catalog policy owns seven current exports",async()=>{
 const {existsSync}=await import("node:fs"),{dirname,join}=await import("node:path");let root=import.meta.dir;while(!existsSync(join(root,"🧰️framework")))root=dirname(root);
 expect(schema.$id).toBe("https://json.schemas.assets.semio-tech.com/os/plugin/registry/component.json");expect(schema.title).toBe("InjectedPluginCatalogV1");
 expect(Object.keys(schema.$defs).sort()).toEqual(["PluginCatalogIdV1","PluginCatalogTextV1","PluginCatalogTextsV1","PluginCatalogTargetV1","PluginCatalogHostV1","PluginCatalogPlaygroundV1"].sort());
 console.log("[DEBUG] actual injected canonical catalog root and six helper identities admitted");
});

test("actual injected catalog scope preserves all original export and diagnostic laws",async()=>{
 const {existsSync}=await import("node:fs"),{dirname,join}=await import("node:path");let root=import.meta.dir;while(!existsSync(join(root,"bun.lock")))root=dirname(root);
 expect(schema.$id).toBe("https://json.schemas.assets.semio-tech.com/os/plugin/registry/component.json");expect(schema.title).toBe("InjectedPluginCatalogV1");
 const {inventorySchemaScopes}=await import("../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"),inventory=inventorySchemaScopes(root),scope=inventory.catalog.scopes["os.plugin.registry"],exports=["InjectedPluginCatalogV1","PluginCatalogIdV1","PluginCatalogTextV1","PluginCatalogTextsV1","PluginCatalogTargetV1","PluginCatalogHostV1","PluginCatalogPlaygroundV1"];
 expect(Object.keys(scope.exports).sort()).toEqual(exports.sort());expect(inventory.diagnostics.filter((row:any)=>row.path===scope.path||row.path.startsWith(scope.path+"/")&&!row.path.slice(scope.path.length+1).includes("/🧬️schema/"))).toEqual([]);
 console.log("[DEBUG] actual injected catalog canonical scope os.plugin.registry exports7 owned diagnostics0");
}, 300000);

import publicationSchema from "../../../../🧑‍💻dev/🎮️playground-session/🧬️schema/🔣️.json";
import publicationCases from "../../../../🧑‍💻dev/🎮️playground-session/🧫️fixtures/🔣️.json";
import { parsePlaygroundSessionPublicationRequestV1 } from "../../../../🧑‍💻dev/🎮️playground-session/🧬️schema/🟦️.ts";
const publicationOracle = new Ajv({ strict: false }).compile(publicationSchema);
for (const row of publicationCases.cases) test(`explicit session publication ${row.id}`, () => {
 expect(publicationOracle(row.input)).toBe(row.shape);
 if (row.accepted) expect(parsePlaygroundSessionPublicationRequestV1(row.input)).toEqual(row.input);
 else expect(() => parsePlaygroundSessionPublicationRequestV1(row.input)).toThrow();
});

 test("owned preview literal argv retains exact bytes through actual spawn", async () => {
  const { generatorPreviewScriptArguments, generatorPreviewExecution } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
  const source = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/🔤️arguments/🧫️fixtures/🔣️.json"), policy = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/🔤️arguments/🧬️schema/🔣️.json");
  const oracle = new Ajv({ strict: false }).compile(policy.default), { default: parseArgv } = await import("yargs-parser"), { spawnSync } = await import("node:child_process");
  for (const row of source.default.cases) {
    expect(oracle(row.arguments)).toBe(row.valid);
    const contract = { ownership: "owned" as const, target: "@neutral/compiler:generate", previewTarget: "@neutral/compiler:preview-browser", previewArguments: row.arguments };
    if (!row.valid) { expect(() => generatorPreviewScriptArguments(contract)).toThrow(); continue; }
    const args = generatorPreviewScriptArguments(contract);
    expect(args).toEqual(row.arguments);
    const actual = spawnSync(process.execPath, ["--eval", "process.stdout.write(JSON.stringify(process.argv.slice(1)))", "--", ...args], { encoding: "utf8", shell: false, timeout: 5000, maxBuffer: 1048576 });
    expect(actual.status, actual.stderr).toBe(0);
    expect(JSON.parse(actual.stdout)).toEqual(row.arguments);
    expect(parseArgv([...args], { configuration: { "parse-numbers": false } })._).toEqual(JSON.parse(actual.stdout));
  }
  const row = source.default.receiving, route = generatorPreviewExecution({ ownership: "owned", ownerPath: "compiler", target: "@neutral/compiler:generate", previewTarget: "@neutral/compiler:preview-browser", previewArguments: row.arguments.slice(1) }, { executor: "nx:run-commands", options: { cwd: "compiler", command: row.command } });
  expect(route.args).toEqual(row.arguments);
  console.log("[DEBUG] literal preview argv schema/Ajv/yargs/actual Bun child byte identity and explicit empty receiver accepted");
 });

 test("preview shell metadata has a finite literal grammar with an independent lexer", async () => {
  const { parseGeneratorPreviewLiteralCommandV1 } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/🔤️arguments/🟦️.ts");
  const { default: source } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/🔤️arguments/🧫️fixtures/🔣️.json"), { default: lexicalPolicy } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/🔤️arguments/🧬️schema/🔣️literal-command.json"), { default: split } = await import("string-argv");
  const lexical = new Ajv({ strict: false }).compile(lexicalPolicy);
  for (const row of source.literalCommands) {
    if (!row.valid) { expect(() => parseGeneratorPreviewLiteralCommandV1(row.command)).toThrow(); continue; }
    expect(lexical(row.command)).toBe(true);
    expect(parseGeneratorPreviewLiteralCommandV1(row.command)).toEqual(row.arguments);
    expect(split(row.command)).toEqual(row.arguments);
  }
  console.log("[DEBUG] preview strict literal command fixture and independent string-argv token identity accepted; expansion syntax refused");
 });

 test("declared preview progress preserves strict artifact output and bounded first-party event custody", async () => {
  const progress = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🟦️.ts"), { default: source } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🧫️fixtures/🔣️.json"), { default: policy } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🧬️schema/🔣️.json"), { default: eventPolicy } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🧬️schema/🔣️event.json"), jsonc = await import("jsonc-parser");
  const oracle = new Ajv({ strict: false }).compile(policy), eventOracle = new Ajv({ strict: false }).compile(eventPolicy), controller = new AbortController(), started = performance.now(), control = { cancelled: () => controller.signal.aborted, remainingMs: () => 30000 - (performance.now() - started) };
  expect(oracle(source.policy)).toBe(true);
  const admitted = progress.parseGeneratorPreviewProgressPolicyV1(source.policy);
  for (const row of source.cases) {
    const stderr = row.events.map(event => `[DEBUG] ${JSON.stringify(event)}\n`).join("");
    if (!row.valid) { expect(() => progress.receiveGeneratorPreviewProgressV1(stderr, admitted, control)).toThrow(); continue; }
    expect(row.events.every(eventOracle)).toBe(true);
    const events = progress.receiveGeneratorPreviewProgressV1(stderr, admitted, control);
    expect(events).toEqual(row.events);
    expect(stderr.split("\n").filter(Boolean).map(line => jsonc.parse(line.slice(8)))).toEqual(events);
    expect(events.map(event => progress.serializeGeneratorPreviewProgressV1(event, admitted)).join("")).toBe(stderr);
  }
  for (const row of source.policyCases) {
    const candidate = { ...source.policy, ...row.changes };
    expect(oracle(candidate), row.id).toBe(row.valid);
    if (row.valid) expect(progress.parseGeneratorPreviewProgressPolicyV1(candidate)).toEqual(candidate);
    else expect(() => progress.parseGeneratorPreviewProgressPolicyV1(candidate)).toThrow();
  }
  for (const row of source.capacityCases) {
    const policy = progress.parseGeneratorPreviewProgressPolicyV1({ ...source.policy, ...row.changes }), events = source.cases.find(candidate => candidate.id === row.caseId)!.events, stderr = events.map(event => `[DEBUG] ${JSON.stringify(event)}\n`).join("");
    expect(events.every(eventOracle)).toBe(true);
    if (row.valid) expect(progress.receiveGeneratorPreviewProgressV1(stderr, policy, control)).toEqual(events);
    else expect(() => progress.receiveGeneratorPreviewProgressV1(stderr, policy, control)).toThrow();
  }
  for (const stderr of source.foreignStderr) expect(() => progress.receiveGeneratorPreviewProgressV1(stderr, admitted, control)).toThrow();
  expect(() => progress.receiveGeneratorPreviewProgressV1("[DEBUG] {}\n", undefined, control)).toThrow();
  expect(progress.receiveGeneratorPreviewProgressV1("", undefined, control)).toEqual([]);
  expect(() => progress.receiveGeneratorPreviewProgressV1("", admitted, { ...control, remainingMs: () => 0 })).toThrow();
  controller.abort();expect(() => progress.receiveGeneratorPreviewProgressV1("", admitted, control)).toThrow();
  console.log("[DEBUG] explicit bounded preview progress Ajv/jsonc identity and malformed/undeclared/cancel/deadline refusals accepted");
 });
