import "../🪆️record-owner/🟦️.ts";
import "../../🛂️manifest/🧪️tests/🏷️type/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { findManifestFiles, readGraphManifestDocuments } from "../../🛂️manifest/📥️admission/🟦️.ts";
import { parseGraphOutputCatalog } from "../../🛂️manifest/📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "../../🛂️manifest/📽️projection/🟦️.ts";
import { writeGraphArtifacts } from "../../🛂️manifest/📤️publication/🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️outputs.json";
import schema from "../../🛂️manifest/🧬️schema/🔣️.json";
function testArtifactRoot(): string {
  const root = process.env.SEMIO_TEST_ARTIFACTS_DIR;
  if (!root) throw new Error("Graph contract requires caller-owned SEMIO_TEST_ARTIFACTS_DIR");
  mkdirSync(root, { recursive: true });
  return root;
}

import currentInput from "../../🛂️manifest/📇️outputs.json";
const current = parseGraphOutputCatalog(currentInput, currentInput.manifests.map((row: { id: string }) => row.id));
const emptyCatalog = parseGraphOutputCatalog(fixture.empty, []);
const fixtureCatalog = parseGraphOutputCatalog(fixture.catalog, fixture.manifestIds);
import consumption from "../../🛂️manifest/🧫️fixtures/🧩️consumption/🔣️.json";
import consumptionSchema from "../../🛂️manifest/🧬️schema/🧩️consumption/🔣️.json";

test("graph admission and projection run with every product physically unavailable to an independent Node loader", async () => {
  const { build } = await import("esbuild");
  const framework = resolve(import.meta.dir, "../../../..");
  const refused = resolve(framework, "🛍️products").replaceAll("\\", "/") + "/";
  const program = "import {parseGraphOutputCatalog} from " + JSON.stringify(resolve(import.meta.dir, "../../🛂️manifest/📇️catalog/🟦️.ts")) + ";import {renderGraphArtifacts} from " + JSON.stringify(resolve(import.meta.dir, "../../🛂️manifest/📽️projection/🟦️.ts")) + ";const fixture=" + JSON.stringify(fixture) + ";const parsed=parseGraphOutputCatalog(fixture.catalog,fixture.manifestIds);const rendered=renderGraphArtifacts(" + JSON.stringify(import.meta.dir) + "," + JSON.stringify(resolve(import.meta.dir, "unwritten-output")) + ",fixture.empty,false);console.log(JSON.stringify({paths:[...Object.values(parsed.shared),...parsed.manifests.flatMap(row=>[row.rust,row.typescript])],manifestCount:rendered.manifestCount,outputs:rendered.artifacts.length}));";
  const result = await build({ stdin: { contents: program, resolveDir: import.meta.dir }, bundle: true, platform: "node", format: "esm", write: false, plugins: [{ name: "removed-graph-products", setup(builder) { builder.onLoad({ filter: /.*/ }, input => input.path.replaceAll("\\", "/").startsWith(refused) ? { errors: [{ text: "General graph loads a removed product: " + input.path }] } : undefined); } }] });
  const native = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(result.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
  expect(native.exitCode, Buffer.from(native.stderr).toString()).toBe(0);
  const actual = JSON.parse(Buffer.from(native.stdout).toString());
  expect(actual).toEqual({ paths: fixture.expectedPaths, manifestCount: 0, outputs: 3 });
}, 15_000);

test("explicit graph manifest consumption follows independent schema ownership", () => {
  const ajv = new Ajv({ strict: true }).addSchema(schema);
  expect(ajv.compile(consumptionSchema)(consumption)).toBe(true);
  for (const row of consumption.cases) {
    const validate = ajv.compile({ type: "object", properties: row.manifest === null ? {} : { manifestId: { const: row.manifest.id } } });
    expect(validate(row.snapshot)).toBe(row.expected.accepted);
    if (row.expected.accepted) {
      expect(row.manifest?.id ?? null).toBe(row.expected.manifestId);
      const observed = (row.snapshot.nodes ?? []).map((node) => node.kind);
      expect([...new Set([...(row.manifest?.nodeKinds ?? []).map((kind) => kind.id), ...observed])].sort()).toEqual(row.expected.nodeKinds);
    }
  }
});

test("explicit output identities preserve independent manifest IDs and reject ambiguous paths", () => {
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/Outputs`)!;
  expect(validate(fixture.catalog)).toBe(true);
  expect(validate(fixture.empty)).toBe(true);
  expect(parseGraphOutputCatalog(fixture.empty,[]).manifests).toEqual([]);
  const parsed = parseGraphOutputCatalog(fixture.catalog, fixture.manifestIds);
  expect([...Object.values(parsed.shared), ...parsed.manifests.flatMap((row) => [row.rust, row.typescript])]).toEqual(fixture.expectedPaths);
  expect(validate(currentInput)).toBe(true);
  expect<unknown>(parseGraphOutputCatalog(currentInput, current.manifests.map((row) => row.id))).toEqual(currentInput);
  for (const row of fixture.invalid) {
    const invalid = structuredClone(fixture.catalog) as unknown as Record<string | number, unknown>;
    let owner = invalid;
    for (const key of row.path.slice(0, -1)) owner = owner[key] as Record<string | number, unknown>;
    owner[row.path.at(-1)!] = row.value;
    expect(validate(invalid)).toBe(!row.schemaInvalid);
    expect(() => parseGraphOutputCatalog(invalid, fixture.manifestIds)).toThrow();
  }
  expect(() => parseGraphOutputCatalog(fixture.catalog, ["chronology"])).toThrow();
  expect(() => parseGraphOutputCatalog(fixture.catalog, ["chronology", "absent"])).toThrow();
  expect(() => parseGraphOutputCatalog(fixture.catalog, ["chronology", "chronology"])).toThrow();
}, 15_000);

test("catalog identity is required and agrees with the independent schema", () => {
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/Outputs`)!;
  expect(validate(fixture.catalog)).toBe(true);
  expect(parseGraphOutputCatalog(fixture.catalog, fixture.manifestIds).contractId).toBe(fixture.catalog.contractId);
  const missing = structuredClone(fixture.catalog) as Record<string, unknown>;
  delete missing.contractId;
  expect(validate(missing)).toBe(false);
  expect(() => parseGraphOutputCatalog(missing, fixture.manifestIds)).toThrow();
});

test("independent owner previews execute with every specific owner unavailable", async () => {
  const { build } = await import("esbuild");
  const workspace = resolve(import.meta.dir, "../../../../..");
  const refused = ["🧰️framework/🛍️products", "✏️s", "🌎️hub"].map((area) => resolve(workspace, area).replaceAll("\\", "/") + "/");
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/Outputs`)!;
  for (const row of fixture.previewCases) {
    const catalog = { ...fixture.empty, contractId: row.contractId };
    expect(validate(catalog)).toBe(true);
    const sandbox = mkdtempSync(join(testArtifactRoot(), "graph-preview-owner-"));
    try {
      const packageRoot = join(sandbox, "owner", "📦️packages", "🦀️rust");
      const catalogPath = join(sandbox, "owner", "🛂️manifest", "📇️outputs.json");
      mkdirSync(packageRoot, { recursive: true });
      mkdirSync(join(sandbox, "owner", "🛂️manifest"), { recursive: true });
      writeFileSync(catalogPath, JSON.stringify(catalog));
      const execution = resolve(import.meta.dir, "../../🛂️manifest/🏃️execution/🟦️.ts");
      const program = `import {PreviewGeneratedScript} from ${JSON.stringify(execution)};await new PreviewGeneratedScript(${JSON.stringify(packageRoot)},${JSON.stringify(sandbox)}).run();`;
      const result = await build({ stdin: { contents: program, resolveDir: import.meta.dir }, bundle: true, platform: "node", format: "esm", write: false, plugins: [{ name: "removed-specific-owners", setup(builder) { builder.onLoad({ filter: /.*/ }, (input) => refused.some((prefix) => input.path.replaceAll("\\", "/").startsWith(prefix)) ? { errors: [{ text: "General preview loads a removed owner: " + input.path }] } : undefined); } }] });
      const child = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(result.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
      expect(child.exitCode, Buffer.from(child.stderr).toString()).toBe(0);
      const preview = JSON.parse(Buffer.from(child.stdout).toString());
      expect(new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["contractId", "nodes", "schemaVersion", "staleRemovals"], properties: { contractId: { const: row.expectedContractId }, nodes: { type: "array", minItems: 1 }, schemaVersion: { const: 1 }, staleRemovals: { type: "array", maxItems: 0 } } })(preview)).toBe(true);
      expect(preview.contractId).toBe(row.expectedContractId);
      expect(preview.nodes.filter((node: { nodeKind: string }) => node.nodeKind === "file")).toHaveLength(3);
      expect(existsSync(join(sandbox, "owner", "🤖️generated"))).toBe(false);
      console.log(`[DEBUG] graph-preview-owner ${JSON.stringify({ expected: row.expectedContractId, actual: preview.contractId, nodes: preview.nodes.length, published: false })}`);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  }
}, 15_000);

test("actual graph contract sources pass strict compiler admission", async () => {
  const { default: ts } = await import("typescript");
  const sources = ["../../🛂️manifest/📇️catalog/🟦️.ts", "../../🛂️manifest/🏃️execution/🟦️.ts", "./🟦️.ts"].map((path) => resolve(import.meta.dir, path));
  const program = ts.createProgram(sources, { noEmit: true, strict: true, noUncheckedIndexedAccess: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, skipLibCheck: true, types: ["bun", "node"] });
  const diagnostics = ts.getPreEmitDiagnostics(program);
  expect(diagnostics.map((row) => ts.flattenDiagnosticMessageText(row.messageText, "\n"))).toEqual([]);
  console.log(`[DEBUG] graph-contract strictSources=${sources.length} diagnostics=${diagnostics.length}`);
}, 30_000);

test("the producer writes exactly declared nested paths and refuses symlink traversal", () => {
  const sandbox = mkdtempSync(join(testArtifactRoot(), "graph-output-"));
  try {
    const outDir = join(sandbox, "output");
    const first = join(outDir, "🕰️clock/🦀️.rs");
    const second = join(outDir, "🌡️sensor/🟦️.ts");
    writeGraphArtifacts(outDir, [{ path: first, content: "clock" }, { path: second, content: "sensor" }]);
    expect(readFileSync(first, "utf8")).toBe("clock");
    expect(readFileSync(second, "utf8")).toBe("sensor");
    for (const escape of ["Z:\\outside\\🦀️.rs", "\\\\outside\\share\\🦀️.rs", join(outDir, "Z:\\outside.rs")]) {
      expect(() => writeGraphArtifacts(outDir, [{ path: escape, content: "must-not-write" }])).toThrow();
    }
    expect(readFileSync(first, "utf8")).toBe("clock");
    writeGraphArtifacts(outDir, [{ path: first, content: "clock-current" }]);
    expect(() => readFileSync(second)).toThrow();
    const outside = join(sandbox, "outside");
    writeFileSync(outside, "protected");
    symlinkSync(outside, join(outDir, "🪤️escape.rs"));
    expect(() => writeGraphArtifacts(outDir, [{ path: first, content: "must-not-write" }])).toThrow();
    expect(readFileSync(first, "utf8")).toBe("clock-current");
    expect(readFileSync(outside, "utf8")).toBe("protected");
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("manifest admission refuses linked inputs instead of following or silently omitting them", () => {
  const sandbox = mkdtempSync(join(testArtifactRoot(), "graph-input-"));
  try {
    const area = "plugins";
    mkdirSync(join(sandbox, area, "real"), { recursive: true });
    writeFileSync(join(sandbox, area, "real", "fixture.manifest.json"), JSON.stringify({ schema: "manifest", id: "fixture" }));
    symlinkSync(join(sandbox, area, "real"), join(sandbox, area, "linked-directory"));
    expect(() => findManifestFiles(sandbox, [area], [])).toThrow(/symbolic link/u);
    rmSync(join(sandbox, area, "linked-directory"));
    symlinkSync(join(sandbox, area, "missing.manifest.json"), join(sandbox, area, "linked.manifest.json"));
    expect(() => findManifestFiles(sandbox, [area], [])).toThrow(/symbolic link/u);
    rmSync(join(sandbox, area, "linked.manifest.json"));
    const rootAlias = `${sandbox}-alias`;
    symlinkSync(sandbox, rootAlias);
    expect(() => findManifestFiles(rootAlias, [area], [])).toThrow(/ancestor is a symbolic link/u);
    rmSync(rootAlias);
    const ancestorTarget = join(sandbox, "ancestor-target");
    mkdirSync(join(ancestorTarget, area), { recursive: true });
    symlinkSync(ancestorTarget, join(sandbox, "linked-ancestor"));
    expect(() => findManifestFiles(sandbox, [`linked-ancestor/${area}`], [])).toThrow(/ancestor is a symbolic link/u);
    expect(() => readGraphManifestDocuments(sandbox, false, [area], [], () => { throw new Error("denied"); })).toThrow(/cannot read admitted graph manifest plugins\/real\/fixture\.manifest\.json/u);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("an unreadable admitted directory reports its exact admission boundary", () => {
  const sandbox = mkdtempSync(join(testArtifactRoot(), "graph-unreadable-"));
  const blocked = join(sandbox, "plugins", "blocked");
  try {
    mkdirSync(blocked, { recursive: true });
    chmodSync(blocked, 0);
    let hostEnforcesMode = false;
    try { readdirSync(blocked); } catch { hostEnforcesMode = true; }
    if (hostEnforcesMode) expect(() => findManifestFiles(sandbox, ["plugins"], [])).toThrow(/cannot read admitted graph manifest directory plugins\/blocked/u);
  } finally {
    chmodSync(blocked, 0o700);
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("manifest exclusions belong to the declared owner and apply before filesystem admission", () => {
  const sandbox = mkdtempSync(join(testArtifactRoot(), "graph-owner-policy-"));
  try {
    for (const row of fixture.admission.files) {
      const path = join(sandbox, row.path);
      mkdirSync(join(path, ".."), { recursive: true });
      if (row.linked) symlinkSync(join(sandbox, "missing"), path);
      else writeFileSync(path, JSON.stringify({ schema: "manifest", id: row.id }));
    }
    const docs = readGraphManifestDocuments(sandbox, false, fixture.admission.inputAreas, fixture.admission.excludedInputPaths);
    expect(docs.map(doc => doc.id)).toEqual(fixture.admission.expectedIds);
    expect(() => findManifestFiles(sandbox, fixture.admission.inputAreas, [])).toThrow(/symbolic link/u);
    expect(findManifestFiles(sandbox, ["owners/hidden"], fixture.admission.excludedInputPaths)).toEqual([]);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("manifest parse and catalog failures never produce a partial artifact plan", () => {
  const sandbox = mkdtempSync(join(testArtifactRoot(), "graph-plan-"));
  try {
    const area = "plugins";
    const outDir = join(sandbox, "output");
    mkdirSync(join(sandbox, area), { recursive: true });
    const input = join(sandbox, area, "fixture.manifest.json");
    writeFileSync(input, "{");
    expect(() => renderGraphArtifacts(sandbox, outDir, {...emptyCatalog,inputAreas:[area]}, false)).toThrow();
    expect(existsSync(outDir)).toBe(false);
    writeFileSync(input, JSON.stringify({ schema: "layout.manifest/v1", id: "fixture" }));
    expect(renderGraphArtifacts(sandbox, outDir, {...emptyCatalog,inputAreas:[area]}, false).manifestCount).toBe(0);
    expect(existsSync(outDir)).toBe(false);
    writeFileSync(input, JSON.stringify({ schema: "manifest", id: "fixture" }));
    expect(() => renderGraphArtifacts(sandbox, outDir, {...emptyCatalog,inputAreas:[area]}, false)).toThrow(/catalog and admitted manifest identities differ/u);
    expect(existsSync(outDir)).toBe(false);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("the actual generated registry loads every declared manifest through its current paths", async () => {
  const rustRegistry = new URL(`../../🤖️generated/${current.shared.rustRegistry}`, import.meta.url);
  const rustReferences = [...readFileSync(rustRegistry, "utf8").matchAll(/#\[path = "([^"]+)"\]/gu)].map((match) => new URL(match[1]!, rustRegistry));
  expect(rustReferences.map((url) => url.href).sort()).toEqual(current.manifests.map((row) => new URL(`../../🤖️generated/${row.rust}`, import.meta.url).href).sort());
  for (const url of rustReferences) expect(readFileSync(url, "utf8").length).toBeGreaterThan(0);
  const registry = await import("../../🤖️generated/🟦️.ts");
  expect([...registry.MANIFEST_IDS].sort()).toEqual<readonly string[]>(current.manifests.map((row) => row.id).sort());
  for (const row of current.manifests) {
    const manifest = registry.manifestById(row.id);
    expect(manifest?.id).toBe(row.id);
    expect(manifest?.schema).toBe("manifest");
    const source = readFileSync(new URL(`../../🤖️generated/${row.typescript}`, import.meta.url), "utf8");
    expect(source).toContain(`from "../${current.shared.typescriptTypes.replace(/\.ts$/u, ".js")}"`);
  }
  expect(registry.manifestById("unknown-manifest")).toBeUndefined();
}, 15_000);

test("cached graph routes hash every direct owner, oracle and source-data input", () => {
  const project = JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json", import.meta.url), "utf8")) as { namedInputs: { default: string[];generatorSources:string[] } };
  const required = [
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🧫️fixtures/🔣️outputs.json",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📤️publication/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json",
    "{workspaceRoot}/🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🪪️identity/🧩️grapheme/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts",
  ];
  for (const input of required) expect(project.namedInputs.generatorSources).toContain(input);
  const router = readFileSync(new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), "utf8");
  expect(router).toContain("../../🛂️manifest/🏃️execution/🟦️.ts");
  expect(router).not.toContain("renderGraphArtifacts");
}, 15_000);


test("owner profiles emit exact independent manifest values and first-party enum codecs", () => {
  const sandbox=mkdtempSync(join(testArtifactRoot(),"graph-owned-profile-"));
  try {
    mkdirSync(join(sandbox,"owners"),{recursive:true});
    mkdirSync(join(sandbox,"foreign"),{recursive:true});
    const validate=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/GraphManifestDocument`)!;
    for(const doc of fixture.emission) {
      expect(validate(doc)).toBe(true);
      writeFileSync(join(sandbox,"owners",`${doc.id}.manifest.json`),JSON.stringify(doc));
    }
    writeFileSync(join(sandbox,"foreign","foreign.manifest.json"),JSON.stringify({schema:"manifest",id:"foreign"}));
    const rendered=renderGraphArtifacts(sandbox,join(sandbox,"output"),fixtureCatalog,false);
    expect(rendered.manifestCount).toBe(2);
    expect(renderGraphArtifacts(sandbox,join(sandbox,"empty"),emptyCatalog,false).manifestCount).toBe(0);
    for(const row of fixture.catalog.manifests) {
      const rust=rendered.artifacts.find((artifact)=>artifact.path.endsWith(row.rust))!.content;
      const jsonLiteral=rust.match(/_MANIFEST_JSON: &str = (".*");/u)![1]!;
      expect(JSON.parse(JSON.parse(jsonLiteral))).toEqual(fixture.emission.find((doc)=>doc.id===row.id));
      expect(rust).toContain("impl semio_framework_value::ToValue");
      expect(rust).toContain("impl semio_framework_value::FromValue");
      expect(rust).not.toContain("foreign");
    }
  } finally {rmSync(sandbox,{recursive:true,force:true});}
},15_000);
