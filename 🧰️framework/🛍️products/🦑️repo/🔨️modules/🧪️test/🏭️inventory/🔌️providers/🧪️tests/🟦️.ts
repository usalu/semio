import { strict as assert } from "node:assert";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import { escape, minimatch } from "minimatch";
import { admitMutationInventoryProviderV1, discoverMutationInventoryProvidersV1, selectMutationInventoryProviderV1 } from "../🟦️.ts";

/** 🧪️ Proves provider admission and explicit ownership against an independent schema oracle. */
export function runMutationInventoryProviderChecksV1(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const schema = read("../🧬️schema/🔣️.json"), vectors = read("../🧫️fixtures/🔣️.json");
  const oracle = new Ajv({ strict: true }).compile(schema);
  for (const vector of vectors.cases) {
    assert.equal(Boolean(oracle(vector.value)), vector.valid, vector.name);
    if (vector.valid) assert.deepEqual(admitMutationInventoryProviderV1(vector.value), vector.value, vector.name);
    else assert.throws(() => admitMutationInventoryProviderV1(vector.value), vector.name);
  }
  for (const vector of vectors.selectionCases) {
    const expected = vector.providers.flatMap((provider: { roots: string[] }, index: number) => provider.roots.some(root => root === vector.owner || minimatch(vector.owner, escape(root) + "/**", { dot: true })) ? [index] : []);
    assert.deepEqual(expected.length > 1 ? "ambiguous" : expected[0] ?? null, vector.expected, vector.name);
    const providers = vector.providers.map((provider: { script: string; roots: string[] }) => ({ script: resolve(...provider.script.split("/")), roots: provider.roots.map(root => resolve(...root.split("/"))) }));
    const owner = resolve(...vector.owner.split("/"));
    if (vector.expected === "ambiguous") assert.throws(() => selectMutationInventoryProviderV1(providers, owner), vector.name);
    else assert.equal(selectMutationInventoryProviderV1(providers, owner), vector.expected === null ? undefined : providers[vector.expected], vector.name);
  }
  const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(import.meta.dir, "🤖️generated"));
  mkdirSync(artifacts, { recursive: true });
  for (const vector of vectors.discoveryCases) {
    const root = mkdtempSync(join(artifacts, "provider-discovery-"));
    try {
      for (const [path, source] of Object.entries(vector.files) as [string, string][]) {
        const target = resolve(root, ...path.split("/"));
        mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, source);
        if (path.endsWith("Cargo.toml")) assert.deepEqual(Bun.TOML.parse(source), parseToml(source), vector.name);
      }
      if (vector.refused) assert.throws(() => discoverMutationInventoryProvidersV1(root), vector.name);
      else {
        const expected = vector.expected.map((provider: { script: string; roots: string[] }) => ({ script: resolve(root, ...provider.script.split("/")), roots: provider.roots.map(path => resolve(root, ...path.split("/"))) }));
        assert.deepEqual(discoverMutationInventoryProvidersV1(root), expected, vector.name);
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  }
  return vectors.cases.length + vectors.selectionCases.length + vectors.discoveryCases.length;
}
