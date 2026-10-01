import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parse } from "@iarna/toml";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { parseRegistrySourceRoots, registryArtifactSourceRoots, type RegistryCatalogInputView } from "../../🔍️discovery/🟦️.ts";
import schema from "../../🧬️schema/📂️registry-source-roots/🔣️.json";
import fixture from "../../🧫️fixtures/📂️registry-source-roots/🔣️.json";

test("portable explicit source roots agree with independent TOML and JSON schema oracles", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const row of fixture.accepted) {
    const decoded = (parse(row.manifest) as any).package.metadata.semio.sources;
    expect(validate(decoded), row.id).toBe(true);
    expect(decoded, row.id).toEqual(row.expected);
    expect({ artifacts: parseRegistrySourceRoots(row.manifest) }, row.id).toEqual(decoded);
  }
  for (const row of fixture.rejected) {
    expect(() => parseRegistrySourceRoots(row.manifest), row.id).toThrow();
    let decoded: unknown;
    try { decoded = (parse(row.manifest) as any).package.metadata.semio.sources; } catch { continue; }
    expect(validate(decoded), row.id).toBe(false);
  }
  expect(parseRegistrySourceRoots("[package.metadata.semio]\nrole = \"hub\"\n")).toEqual([]);
});

test("actual source directories resolve across physical owners and reject escapes, missing and linked ancestors", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "registry-sources-"));
  try {
    for (const row of fixture.resolution) {
      mkdirSync(join(root, row.crate), { recursive: true });
      mkdirSync(join(root, row.expected), { recursive: true });
      writeFileSync(join(root, row.crate, "Cargo.toml"), "[package.metadata.semio.sources]\nartifacts = " + JSON.stringify([row.declared]) + "\n");
      expect(registryArtifactSourceRoots(root, row.crate), row.id).toEqual([row.expected]);
      const malformed = (value: string): void => writeFileSync(join(root, row.crate, "Cargo.toml"), "[package.metadata.semio.sources]\nartifacts = " + JSON.stringify([value]) + "\n");
      malformed("../../../../../../outside");
      expect(() => registryArtifactSourceRoots(root, row.crate)).toThrow("escapes");
      malformed("missing");
      expect(() => registryArtifactSourceRoots(root, row.crate)).toThrow("nofollow");
      const source = "[package.metadata.semio.sources]\nartifacts = " + JSON.stringify([row.declared]) + "\n";
      const view: RegistryCatalogInputView = { readText: () => source, readBytes: () => new TextEncoder().encode(source), entries: () => [], kind: (path) => path.endsWith("Cargo.toml") ? "file" : path === row.expected.split("/")[0] ? "symlink" : "directory" };
      expect(() => registryArtifactSourceRoots(root, row.crate, view)).toThrow("nofollow");
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});
