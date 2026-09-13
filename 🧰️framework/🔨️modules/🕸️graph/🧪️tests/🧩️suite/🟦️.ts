import { expect, test } from "bun:test";
import Ajv from "ajv";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { findManifestFiles, readGraphManifestDocuments } from "../../🛂️manifest/📥️admission/🟦️.ts";
import { parseGraphOutputCatalog } from "../../🛂️manifest/📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "../../🛂️manifest/📽️projection/🟦️.ts";
import { writeGraphArtifacts } from "../../🛂️manifest/📤️publication/🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️outputs.json";
import schema from "../../🛂️manifest/🧬️schema/🔣️.json";
import current from "../../🛂️manifest/📇️outputs.json";

test("explicit output identities preserve independent manifest IDs and reject ambiguous paths", () => {
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/Outputs`)!;
  expect(validate(fixture.catalog)).toBe(true);
  const parsed = parseGraphOutputCatalog(fixture.catalog, fixture.manifestIds);
  expect([...Object.values(parsed.shared), ...parsed.manifests.flatMap((row) => [row.rust, row.typescript])]).toEqual(fixture.expectedPaths);
  expect(validate(current)).toBe(true);
  expect(parseGraphOutputCatalog(current, current.manifests.map((row) => row.id))).toEqual(current);
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

test("the producer writes exactly declared nested paths and refuses symlink traversal", () => {
  const sandbox = mkdtempSync(join(tmpdir(), "graph-output-"));
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
  const sandbox = mkdtempSync(join(tmpdir(), "graph-input-"));
  try {
    const area = "plugins";
    mkdirSync(join(sandbox, area, "real"), { recursive: true });
    writeFileSync(join(sandbox, area, "real", "fixture.manifest.json"), JSON.stringify({ schema: "manifest", id: "fixture" }));
    symlinkSync(join(sandbox, area, "real"), join(sandbox, area, "linked-directory"));
    expect(() => findManifestFiles(sandbox, [area])).toThrow(/symbolic link/u);
    rmSync(join(sandbox, area, "linked-directory"));
    symlinkSync(join(sandbox, area, "missing.manifest.json"), join(sandbox, area, "linked.manifest.json"));
    expect(() => findManifestFiles(sandbox, [area])).toThrow(/symbolic link/u);
    rmSync(join(sandbox, area, "linked.manifest.json"));
    const rootAlias = `${sandbox}-alias`;
    symlinkSync(sandbox, rootAlias);
    expect(() => findManifestFiles(rootAlias, [area])).toThrow(/ancestor is a symbolic link/u);
    rmSync(rootAlias);
    const ancestorTarget = join(sandbox, "ancestor-target");
    mkdirSync(join(ancestorTarget, area), { recursive: true });
    symlinkSync(ancestorTarget, join(sandbox, "linked-ancestor"));
    expect(() => findManifestFiles(sandbox, [`linked-ancestor/${area}`])).toThrow(/ancestor is a symbolic link/u);
    expect(() => readGraphManifestDocuments(sandbox, false, [area], () => { throw new Error("denied"); })).toThrow(/cannot read admitted graph manifest plugins\/real\/fixture\.manifest\.json/u);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("an unreadable admitted directory reports its exact admission boundary", () => {
  const sandbox = mkdtempSync(join(tmpdir(), "graph-unreadable-"));
  const blocked = join(sandbox, "plugins", "blocked");
  try {
    mkdirSync(blocked, { recursive: true });
    chmodSync(blocked, 0);
    let hostEnforcesMode = false;
    try { readdirSync(blocked); } catch { hostEnforcesMode = true; }
    if (hostEnforcesMode) expect(() => findManifestFiles(sandbox, ["plugins"])).toThrow(/cannot read admitted graph manifest directory plugins\/blocked/u);
  } finally {
    chmodSync(blocked, 0o700);
    rmSync(sandbox, { recursive: true, force: true });
  }
}, 15_000);

test("manifest parse and catalog failures never produce a partial artifact plan", () => {
  const sandbox = mkdtempSync(join(tmpdir(), "graph-plan-"));
  try {
    const area = "plugins";
    const outDir = join(sandbox, "output");
    mkdirSync(join(sandbox, area), { recursive: true });
    const input = join(sandbox, area, "fixture.manifest.json");
    writeFileSync(input, "{");
    expect(() => renderGraphArtifacts(sandbox, outDir, false, [area])).toThrow();
    expect(existsSync(outDir)).toBe(false);
    writeFileSync(input, JSON.stringify({ schema: "layout.manifest/v1", id: "fixture" }));
    expect(() => renderGraphArtifacts(sandbox, outDir, false, [area])).toThrow(/no graph manifest documents/u);
    expect(existsSync(outDir)).toBe(false);
    writeFileSync(input, JSON.stringify({ schema: "manifest", id: "fixture" }));
    expect(() => renderGraphArtifacts(sandbox, outDir, false, [area])).toThrow(/catalog and admitted manifest identities differ/u);
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
  expect([...registry.MANIFEST_IDS].sort()).toEqual(current.manifests.map((row) => row.id).sort());
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
  const project = JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json", import.meta.url), "utf8")) as { namedInputs: { default: string[] } };
  const required = [
    "{workspaceRoot}/✏️s/🔌️plugins/**/*manifest.json",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🧫️fixtures/🔣️outputs.json",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📤️publication/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json",
    "{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json",
    "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts",
    "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  ];
  for (const input of required) expect(project.namedInputs.default).toContain(input);
  const router = readFileSync(new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), "utf8");
  expect(router).toContain("../../🛂️manifest/🏃️execution/🟦️.ts");
  expect(router).not.toContain("renderGraphArtifacts");
}, 15_000);
