import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { loadCatalogTaxonomy, semanticProjectionCatalogProblems } from "../../🔍️discovery/🟦️.ts";
import { mutationCatalogProblems } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
const vector = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧬️mutation-scenario-identities/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🧬️mutation-scenario-identities/🔣️.json"), "utf8"));

test("logical mutation scenario ids and physical case names are independent exact identities", () => {
  const ajv = new Ajv({ strict: false });
  const validateVector = ajv.compile(schema), validateScenario = ajv.compile(vector.scenarioSchema);
  expect(validateVector(vector), JSON.stringify(validateVector.errors)).toBe(true);
  const ownerPath = "🗿️sample/🏅️standards/🔖️1/🪆️subsets/✳️any";
  const sourceRoot = `${ownerPath}/🧬️schema/🧬️mutations`;
  const baseline = loadCatalogTaxonomy();
  const taxonomy = { ...baseline, mutationDomainOwners: { ...baseline.mutationDomainOwners, [sourceRoot]: { "🎥️camera": { "🌱️create": "create-camera" } } } };
  for (const row of vector.cases) {
    const ids = row.scenarios.map((scenario: any) => scenario.id), directories = row.scenarios.map((scenario: any) => scenario.directoryName);
    const oracle = row.scenarios.every((scenario: any) => validateScenario(scenario)) && new Set(ids).size === ids.length && new Set(directories).size === directories.length;
    expect(oracle, `${row.id}: ${JSON.stringify(validateScenario.errors)}`).toBe(row.valid);
    const mutation = { mutationId: "create-camera", sourceMutationDirectoryName: "🌱️create", mutationDirectoryName: "🌱️create", scenarios: row.scenarios };
    const problems = semanticProjectionCatalogProblems([{ ownerPath, catalogId: row.id, vectors: [mutation] }], taxonomy);
    expect(problems.length === 0, `${row.id}: ${problems.join(" | ")}`).toBe(row.valid);
    const platformProblems = mutationCatalogProblems({ id: row.id, capability: "sample-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: ["create-camera"], vectors: [mutation] }, ownerPath, taxonomy);
    expect(platformProblems.length === 0, `${row.id}: ${platformProblems.join(" | ")}`).toBe(row.valid);
  }
});
