import { jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import { buildSync } from "esbuild";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { generatePluginRegistry, generatePluginRegistryReport, parseComponentSourceOwnerV1, parseCompiledComponentOwnerV1, parseRegistryChannelDiagnosticsV1 } from "../🟦️.ts";
import { REGISTRY_HOST_APP_CHANNEL_VERSION } from "../../🧬️schema/🟦️.ts";
import contract from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import identity from "../../../../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json";
import { parseComponentSourceRowV1, parseCompiledComponentRowV1, parseDeployedRegistryEntryV1 } from "../🧬️schema/🟦️.ts";

const parsers = { source: parseComponentSourceRowV1, compiled: parseCompiledComponentRowV1, deployed: parseDeployedRegistryEntryV1 };
const names = { source: "ComponentSourceOwnerV1", compiled: "CompiledComponentOwnerV1", deployed: "DeployedRegistryEntryV1" };
test("source, compiled and deployed contracts agree with independent AJV", () => {
  const ajv = new Ajv({ strict: false }).addSchema(identity).addSchema(contract);
  expect(ajv.compile({ $ref: `${contract.$id}#/$defs/CorpusV1` })(corpus)).toBe(true);
  for (const row of corpus.cases) {
    const stage = row.stage as keyof typeof parsers;
    const oracle = ajv.compile({ $ref: `${contract.$id}#/$defs/${names[stage]}` });
    expect(oracle(row.value), row.name).toBe(row.valid);
    if (row.valid) expect(parsers[stage](row.value), row.name).toEqual(row.value);
    else expect(() => parsers[stage](row.value), row.name).toThrow();
  }
});


test("independent Node executes every authored stage vector", () => {
  const program = `import * as api from ${JSON.stringify(resolve(import.meta.dirname, "../🧬️schema/🟦️.ts"))}; const rows=${JSON.stringify(corpus.cases)}; const parsers={source:api.parseComponentSourceRowV1,compiled:api.parseCompiledComponentRowV1,deployed:api.parseDeployedRegistryEntryV1}; console.log(JSON.stringify(rows.map(row=>{try{parsers[row.stage](row.value);return true}catch{return false}})));`;
  const result = buildSync({ stdin: { contents: program, resolveDir: import.meta.dirname }, bundle: true, platform: "node", format: "esm", write: false });
  const node = spawnSync("node", ["--input-type=module"], { input: result.outputFiles[0]!.text, encoding: "utf8" });
  expect(node.status, node.stderr).toBe(0);
  expect(JSON.parse(node.stdout)).toEqual(corpus.cases.map(row => row.valid));
});

test("real source and descriptor admission preserves compiled-only ownership", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Component owner law requires ticket artifacts");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "component-owner-"));
  try {
    const cargo = join(root, "owner/📦️packages/🦀️rust/Cargo.toml");
    mkdirSync(dirname(cargo), { recursive: true });
    writeFileSync(cargo, corpus.source.cargo);
    const toml = parseToml(corpus.source.cargo) as { package: { metadata: { component: { package: string }; semio: { "component-kind": string } } } };
    const source = parseComponentSourceOwnerV1(cargo, root);
    expect(source.packageId).toBe(toml.package.metadata.component.package);
    expect(source.role).toBe(toml.package.metadata.semio["component-kind"]);
    expect(source.directoryName).toBeUndefined();
    for (const field of ["capabilities", "contributes", "activationEvents", "extensionPoints", "hashes"]) expect(Object.hasOwn(source, field)).toBe(false);
    expect(() => parseCompiledComponentOwnerV1(cargo, root)).toThrow();
    const descriptor = join(root, "owner/🔣️.json");
    writeFileSync(descriptor, JSON.stringify(corpus.source.descriptor));
    const compiled = parseCompiledComponentOwnerV1(cargo, root);
    expect(compiled).toEqual({ ...source, ...corpus.source.compiled });
    expect(() => parseDeployedRegistryEntryV1(compiled)).toThrow();
    writeFileSync(cargo, corpus.source.cargo + `deployment-directory=${JSON.stringify(corpus.source.directoryName)}\n`);
    expect(parseDeployedRegistryEntryV1(parseCompiledComponentOwnerV1(cargo, root)).directoryName).toBe(corpus.source.directoryName);
    writeFileSync(descriptor, JSON.stringify({ ...corpus.source.descriptor, packageId: "semio:other" }));
    expect(() => parseCompiledComponentOwnerV1(cargo, root)).toThrow(/identity/);
    writeFileSync(descriptor, JSON.stringify({ ...corpus.source.descriptor, executionProtocol: { appChannelVersion: 19 } }));
    expect(() => parseCompiledComponentOwnerV1(cargo, root)).toThrow(/registry descriptor/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("dev generation withholds stale-channel plugins and their dependents while the refusing gate names them", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Stale-channel registry law requires ticket artifacts");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "stale-channel-"));
  try {
    const owner = (id: string, channel: number, dependsOn: readonly string[] = []) => {
      const manifestPath = `${id}/📦️packages/🦀️rust/Cargo.toml`;
      mkdirSync(join(root, dirname(manifestPath)), { recursive: true });
      const cargo = corpus.source.cargo.replace('name = "portable-component"', `name = "${id}-component"`).replace('package = "semio:portable"', `package = "semio:${id}"`);
      writeFileSync(join(root, manifestPath), `${cargo}${dependsOn.length ? `depends-on = ${JSON.stringify(dependsOn)}\n` : ""}deployment-directory=${JSON.stringify(`🧪️${id}`)}\n`);
      writeFileSync(join(root, id, "🔣️.json"), JSON.stringify({ ...corpus.source.descriptor, packageId: `semio:${id}`, manifest: { ...corpus.source.descriptor.manifest, pluginId: id }, executionProtocol: { appChannelVersion: channel } }));
      return { lang: "🦀️rust", manifestPath };
    };
    const packages = [owner("fresh", REGISTRY_HOST_APP_CHANNEL_VERSION), owner("stale", REGISTRY_HOST_APP_CHANNEL_VERSION - 1), owner("dependent", REGISTRY_HOST_APP_CHANNEL_VERSION, ["stale"])] as never;
    const report = generatePluginRegistryReport(root, { packages, staleChannel: "exclude" });
    expect(report.entries.map((entry) => entry.pluginId)).toEqual(["fresh"]);
    expect(report.diagnostics).toEqual([
      { code: "stale-channel-dependency", pluginId: "dependent", cratePath: "dependent/📦️packages/🦀️rust", dependency: "stale" },
      { code: "stale-channel", pluginId: "stale", cratePath: "stale/📦️packages/🦀️rust", descriptorChannel: REGISTRY_HOST_APP_CHANNEL_VERSION - 1, hostChannel: REGISTRY_HOST_APP_CHANNEL_VERSION },
    ]);
    expect(parseRegistryChannelDiagnosticsV1(`${JSON.stringify(report.diagnostics, null, 2)}\n`)).toEqual(report.diagnostics);
    expect(() => generatePluginRegistry(root, { packages })).toThrow(/stale-channel descriptors refused .*stale \(stale\/📦️packages\/🦀️rust\)/);
    expect(() => generatePluginRegistryReport(root, { packages, staleChannel: "refuse" })).toThrow(/stale-channel descriptors refused/);
    expect(() => parseRegistryChannelDiagnosticsV1(JSON.stringify([{ ...report.diagnostics[1], extra: true }]))).toThrow(/undeclared shape/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("JSON-schema equality preserves array order and ignores object member order", () => {
  const ajv = new Ajv({ strict: false });
  for (const row of corpus.comparisons) {
    expect(ajv.compile({ const: row.left })(row.right), row.name).toBe(row.equal);
    expect(jsonSchemaSubsetValueEquals(row.left, row.right), row.name).toBe(row.equal);
    const schema = { type: "array", uniqueItems: true };
    expect(validateJsonSchemaSubset(schema, [row.left, row.right]).length === 0).toBe(ajv.compile(schema)([row.left, row.right]));
  }
});
