import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { mutationVectorRegistryBreaches, repoRootFromHere, testTaxonomy, type OracleRegistry } from "../../📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🧬️mutation-fixtures/🔣️.json";

const taxonomy = testTaxonomy(repoRootFromHere());
const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: ["change-value"], vectors: [{ mutationId: "change-value", sourceMutationDirectoryName: vectors.mutation, mutationDirectoryName: vectors.mutation, scenarios: [{ id: "changes-the-value", directoryName: vectors.scenario }] }] };
const contribution = { owner: vectors.owner, manifestPath: `${vectors.owner}/🔣️oracle.json`, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], migrationStatus: {} };
const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [contribution] } as unknown as OracleRegistry;

test("mutation fixture examples satisfy the owning schema", () => {
  const valid = new Ajv({ strict: true }).compile(schema.$defs.MutationFixtureCases);
  expect(valid(vectors)).toBe(true);
});
for (const row of vectors.cases) test(row.id, () => {
  const root = mkdtempSync(join(tmpdir(), "semio-mutation-fixture-"));
  const files: Record<string, string> = { ...vectors.files, ...row.add };
  for (const path of row.remove) delete files[path];
  try {
    for (const [path, source] of Object.entries(files)) {
      const destination = join(root, vectors.owner, path);
      mkdirSync(dirname(destination), { recursive: true });
      writeFileSync(destination, source);
    }
    const oracle = new Ajv({ strict: true, allowMatchingProperties: true }).compile({ type: "object", additionalProperties: false, required: Object.keys(vectors.files), properties: Object.fromEntries(Object.keys(vectors.files).map(path => [path, { type: "string" }])), patternProperties: { "^🧬️schema/🧬️mutations/[^/]+/🧪️tests/[^/]+/🦀️\\.rs$": { type: "string" } } });
    expect(oracle(files)).toBe(row.valid);
    const findings = mutationVectorRegistryBreaches(root, registry, taxonomy);
    expect(findings.length === 0, JSON.stringify(findings)).toBe(row.valid);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
