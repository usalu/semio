/** 🧬️ Mutation leaf identity law: leaf, fixture-mirror and fixture-vector directories resolve from the leaf's own canonical descriptor, never from a hand-maintained name list. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import Ajv from "ajv";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { inventoryTaxonomy, type TaxonomyInventory } from "../../🧹️normalization/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
const repoRoot = process.env.SEMIO_FIXTURE_REPO_ROOT ?? resolve(import.meta.dir, "../../../../../../../");
const vector = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧫️mutation-leaf-identity/🔣️.json"), "utf8"));

const taxonomy = loadCatalogTaxonomy();
const ajv = new Ajv({ strict: true, allErrors: true });
const fold = (value: string): string => value.normalize("NFC").replaceAll("️", "");
const inventories = new Map<string, TaxonomyInventory>();
const inventoryOf = (scope: string): TaxonomyInventory => inventories.get(scope) ?? inventories.set(scope, inventoryTaxonomy({ repoRoot, scope, workers: 1 })).get(scope)!;
const descriptorProof = (ownerPath: string) => ajv.compile({
  type: "object",
  required: ["schemaVersion", vector.descriptor.ownerField, vector.descriptor.identityField],
  properties: {
    schemaVersion: { type: "integer", const: vector.descriptor.schemaVersion },
    [vector.descriptor.ownerField]: { type: "string", const: ownerPath },
    [vector.descriptor.identityField]: { type: "string", minLength: 1 },
  },
});

test("the mutation leaf identity vectors satisfy their schema (Ajv)", () => {
  
  expect(vector["contract"]).toEqual("mutation-leaf-identity-v1");expect(vector["descriptor"]["filename"]).toEqual("🔣️.json");expect(vector["descriptor"]["schemaVersion"]).toEqual(1);expect(vector["descriptor"]["ownerField"]).toEqual("owner");expect(vector["descriptor"]["identityField"]).toEqual("semanticKind");expect(vector["leafKindId"]).toEqual("members-of-schema");expect(vector["vectorKindId"]).toEqual("members-of-fixtures");
});

test("leaf identity is structural: no per-name leaf registry remains and the descriptor authority is the taxonomy's", () => {
  const authority = taxonomy.mutationPayloadSchemaAuthority, descriptorKind = taxonomy.fileKinds[authority.descriptorFileKindId]!;
  expect({ filename: `${descriptorKind.emoji}${descriptorKind.extensionChains[0]}`, schemaVersion: authority.descriptorSchemaVersion, ownerField: authority.descriptorOwnerField, identityField: authority.descriptorIdentityField }).toEqual(vector.descriptor);
  expect(taxonomy.semanticProjectedMemberKinds[taxonomy.mutationCatalogProjection.projectedMemberKindId]!.sourceMemberKindId).toBe(vector.leafKindId);
  for (const id of vector.retiredRegistries) expect(Object.hasOwn(taxonomy.semanticDirectoryMemberKinds, id), id).toBe(false);
  for (const row of vector.cases.filter((candidate: { expectedKindId: string | null }) => candidate.expectedKindId !== null)) {
    const names = taxonomy.semanticDirectoryMemberKinds[row.expectedKindId]!.memberNames.map(fold);
    expect(names.includes(fold(basename(row.path))), row.id).toBe(false);
  }
});

for (const row of vector.cases)
  test(`mutation leaf identity agrees with the Ajv descriptor oracle: ${row.id}`, () => {
    const descriptorPath = row.owner === null ? null : join(repoRoot, row.scope, row.owner, vector.descriptor.filename);
    const proven = descriptorPath !== null && existsSync(descriptorPath) && descriptorProof(`${row.scope}/${row.owner}`)(JSON.parse(readFileSync(descriptorPath, "utf8")));
    expect(proven, row.id).toBe(row.expectedKindId !== null);
    const path = `${row.scope}/${row.path}`, entry = inventoryOf(row.scope).entries.find((candidate) => candidate.sourcePath === path);
    expect(entry?.nodeKind, row.id).toBe("directory");
    expect(entry!.normalizedPath).toBe(path);
    expect(entry!.violations.some((violation) => violation.code === "directory-kind-unresolved"), row.id).toBe(row.expectedKindId === null);
  }, 180_000);
