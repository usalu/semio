import { expect, test } from "bun:test";
import { lstatSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { inventoryTaxonomy } from "../../🧹️normalization/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
const repoRoot = process.env.SEMIO_FIXTURE_REPO_ROOT ?? resolve(import.meta.dir, "../../../../../../../");
const vector = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧬️mutation-case-pair/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🧬️mutation-case-pair/🔣️.json"), "utf8"));

test("canonical mutation pairs remain normalized at their schema and fixture owners", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const implementationRoot = `${vector.subset}/${vector.implementationRoot}`, fixtureRoot = `${vector.subset}/${vector.fixtureRoot}`;
  const inventory = inventoryTaxonomy({ repoRoot, scope: vector.subset, workers: 1 });
  const nodes = (root: string) => inventory.entries.filter((entry) => entry.sourcePath === root || entry.sourcePath.startsWith(`${root}/`));
  const implementation = nodes(implementationRoot), fixtures = nodes(fixtureRoot);
  expect(implementation).toHaveLength(vector.implementationNodes);
  expect(fixtures).toHaveLength(vector.fixtureNodes);
  expect([...implementation, ...fixtures].every((entry) => entry.sourcePath === entry.normalizedPath)).toBe(true);
  expect([...implementation, ...fixtures].flatMap((entry) => entry.violations).filter((violation) => /^(?:mutation|projection)-/u.test(violation.code))).toEqual([]);
  expect(fixtures.filter((entry) => entry.nodeKind === "file").map((entry) => relative(resolve(repoRoot, fixtureRoot), resolve(repoRoot, entry.sourcePath)).replaceAll("\\", "/")).sort()).toEqual([...vector.fixtureLeaves].sort());
  for (const path of [implementationRoot, fixtureRoot, ...vector.fixtureLeaves.map((leaf: string) => `${fixtureRoot}/${leaf}`)]) expect(lstatSync(join(repoRoot, path)).isSymbolicLink(), path).toBe(false);
  expect(inventory.entries.some((entry) => /\/🧪️tests\/🪆️[^/]+\//u.test(entry.sourcePath))).toBe(false);
}, 180_000);
