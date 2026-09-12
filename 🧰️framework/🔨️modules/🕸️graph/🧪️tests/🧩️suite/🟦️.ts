import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { findManifestFiles, parseGraphOutputCatalog, writeGraphArtifacts } from "../../🛂️manifest/🏭️generator/🟦️.ts";
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
});

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
});

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
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
});

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
});
