/** 🧾️ Mutation wire witness law: one taxonomy kind resolves the payload-only `🧾️wire-witness` evidence case in both placements of a mutation leaf's fixture scope. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { loadCatalogTaxonomy, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";
import { inventoryTaxonomy } from "../../🧹️normalization/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
const repoRoot = process.env.SEMIO_FIXTURE_REPO_ROOT ?? resolve(import.meta.dir, "../../../../../../..");
const vector = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧫️mutation-wire-witness/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️mutation-wire-witness/🔣️.json"), "utf8"));
const taxonomy = loadCatalogTaxonomy();
const spec = taxonomy.semanticDirectoryKinds[vector.kindId]!;
const unresolved = /^(?:directory|file)-kind-(?:unresolved|ambiguous)$|^semantic-stem-(?:unresolved|ambiguous)$/u;
const oracle = new Ajv({ strict: true, allErrors: true }).compile({
  type: "object",
  required: ["emoji", "slug", "parentKindId"],
  properties: { emoji: { type: "string", const: spec.emoji }, slug: { type: "string", pattern: spec.slugPattern }, parentKindId: { type: "string", enum: [...(spec.parentKindIds ?? [])] } },
});
const split = (name: string): { emoji: string; slug: string } => {
  const match = /^(\p{Extended_Pictographic}️?)(.*)$/u.exec(name);
  return match ? { emoji: match[1]!, slug: match[2]! } : { emoji: "", slug: name };
};

test("the mutation wire witness vectors satisfy their schema (Ajv)", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
});

test("exactly one registered kind claims the wire witness name, bounded to mutation-leaf fixture scopes", () => {
  const claims = Object.entries(taxonomy.semanticDirectoryKinds).filter(([, kind]) => vector.directoryName.startsWith(kind.emoji) && new RegExp(kind.slugPattern, "u").test(vector.directoryName.slice(kind.emoji.length)));
  expect(claims.map(([id]) => id)).toEqual([vector.kindId]);
  expect([...(spec.parentKindIds ?? [])].sort()).toEqual([...vector.parentKindIds].sort());
  expect(spec.allowEmojiOnly).toBe(false);
});

for (const row of vector.resolutions)
  test(`wire witness kind resolution agrees with the Ajv oracle: ${row.id}`, () => {
    const actual = semanticDirectoryKindId(row.name, taxonomy, row.parentKindId === null ? {} : { parentKindId: row.parentKindId });
    expect(actual).toBe(row.expectedKindId);
    expect(oracle({ ...split(row.name), parentKindId: row.parentKindId })).toBe(row.expectedKindId === vector.kindId);
  });

for (const placement of vector.placements)
  test(`wire witness placements resolve without taxonomy findings: ${placement.id}`, () => {
    const at = (path: string): string => `${placement.scope}/${path}`;
    const witnessRoot = at(placement.witnessRoot), fixtureScope = dirname(witnessRoot);
    expect(semanticDirectoryKindId(vector.directoryName, taxonomy, { parentKindId: placement.parentKindId })).toBe(vector.kindId);
    const inventory = inventoryTaxonomy({ repoRoot, scope: placement.scope, workers: 1 });
    const witness = inventory.entries.filter((entry) => entry.sourcePath === witnessRoot || entry.sourcePath.startsWith(`${witnessRoot}/`));
    expect(witness).toHaveLength(vector.witnessNodes);
    expect(witness.every((entry) => entry.sourcePath === entry.normalizedPath)).toBe(true);
    expect(witness.filter((entry) => entry.nodeKind === "file").map((entry) => relative(resolve(repoRoot, witnessRoot), resolve(repoRoot, entry.sourcePath)).replaceAll("\\", "/")).sort()).toEqual([...vector.witnessLeaves].sort());
    const scoped = new Set([witnessRoot, fixtureScope, ...placement.leafRoots.map(at)]);
    expect(inventory.entries.filter((entry) => scoped.has(entry.sourcePath) || entry.sourcePath.startsWith(`${witnessRoot}/`)).flatMap((entry) => entry.violations.filter((violation) => unresolved.test(violation.code)).map((violation) => `${violation.code} ${entry.sourcePath}`))).toEqual([]);
    for (const leaf of vector.witnessLeaves) expect(typeof JSON.parse(readFileSync(join(repoRoot, witnessRoot, leaf), "utf8"))).toBe("object");
    expect(inventory.entries.some((entry) => entry.sourcePath === `${at(placement.leafRoots[0])}/🔣️.json` && entry.nodeKind === "file")).toBe(true);
  }, 180_000);
