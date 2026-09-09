import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import picomatch from "picomatch";
import { loadCatalogTaxonomy, validateTaxonomy } from "../../🔍️discovery/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
const vector = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🖼️runtime-taxonomy-asset-paths/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🖼️runtime-taxonomy-asset-paths/🔣️.json"), "utf8"));

test("runtime taxonomy inputs are owned production assets", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const baseline = loadCatalogTaxonomy();
  expect(validateTaxonomy(baseline)).toEqual([]);
  const targets = [
    {
      label: "semanticPackageProjectionContracts.nested-cargo-packages-v1",
      set: (taxonomy: any, path: string) => { taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogPath = path; },
      invalidId: "fixture-input",
      problem: (value: string) => value.includes("semanticPackageProjectionContracts.nested-cargo-packages-v1"),
    },
    {
      label: "semanticOwnedFileProjectionContracts.readme-license-owner-leaves-v1",
      set: (taxonomy: any, path: string) => { taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"].authorityCatalogPath = path; },
      invalidId: "schema-input",
      problem: (value: string) => value.includes("semanticOwnedFileProjectionContracts") && value.includes("authorityCatalogPath"),
    },
    {
      label: "currentSourceRevisions.testing-readme-protocol-v2-reviewed",
      set: (taxonomy: any, path: string) => { taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"].currentSourceRevisions["testing-readme-protocol-v2-reviewed"].expectationsPath = path; },
      invalidId: "asset-lookalike",
      problem: (value: string) => value.includes("currentSourceRevisions.testing-readme-protocol-v2-reviewed.expectationsPath"),
    },
  ];
  const oracle = picomatch(`**/${baseline.exampleAssetsDirName}/**`);
  for (const row of vector.cases) expect(oracle(row.path), row.id).toBe(row.valid);
  for (const target of targets) {
    for (const row of [vector.cases.find((candidate: any) => candidate.id === "direct-asset"), vector.cases.find((candidate: any) => candidate.id === target.invalidId)]) {
      const taxonomy = structuredClone(baseline) as any;
      target.set(taxonomy, row.path);
      const relevant = validateTaxonomy(taxonomy).filter(target.problem);
      expect(relevant.length === 0, `${target.label}: ${row.id}: ${relevant.join(" | ")}`).toBe(row.valid);
    }
  }
}, 60_000);
