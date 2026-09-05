import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { parseGraphOutputCatalog, writeGraphArtifacts } from "../📦️packages/🦀️rust/📜️script.ts";
import fixture from "./🔣️outputs.json";
import schema from "../🛂️manifest/🧬️outputs.schema.json";
import current from "../🛂️manifest/📇️outputs.json";

test("explicit output identities preserve independent manifest IDs and reject ambiguous paths", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
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

test("the actual generated registry loads every declared manifest through its current paths", async () => {
  const registry = await import("../🤖️generated/🟦️.ts");
  expect([...registry.MANIFEST_IDS].sort()).toEqual(current.manifests.map((row) => row.id).sort());
  for (const row of current.manifests) {
    const manifest = registry.manifestById(row.id);
    expect(manifest?.id).toBe(row.id);
    expect(manifest?.schema).toBe("manifest");
    const source = readFileSync(new URL(`../🤖️generated/${row.typescript}`, import.meta.url), "utf8");
    expect(source).toContain('from "../🔠️types.js"');
  }
  expect(registry.manifestById("unknown-manifest")).toBeUndefined();
});
