import { expect, test } from "bun:test";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { inventoryTaxonomy } from "../../🧹️normalization/🟦️.ts";

const owner = resolve(import.meta.dir, "../..");
const repoRoot = process.env.SEMIO_FIXTURE_REPO_ROOT ?? resolve(import.meta.dir, "../../../../../../..");
const vector = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧬️mutation-case-pair/🔣️.json"), "utf8"));

const domainOwners = (JSON.parse(readFileSync(join(owner, "🔣️taxonomy.json"), "utf8")) as { mutationDomainOwners: Record<string, unknown> }).mutationDomainOwners;

test("the canonical mutation case pair vectors satisfy their schema (Ajv)", () => {
  
  expect(vector["contract"]).toEqual("canonical-mutation-case-pair-v1");
});

for (const pair of vector.cases)
  test(`canonical mutation pairs remain normalized at their schema and fixture owners: ${pair.id}`, () => {
    const implementationRoot = `${pair.subset}/${pair.implementationRoot}`, fixtureRoot = `${pair.subset}/${pair.fixtureRoot}`, fixtureMirror = `${pair.subset}/🧫️fixtures/🧬️mutations`;
    const inventory = inventoryTaxonomy({ repoRoot, scope: pair.subset, workers: 1 });
    const nodes = (root: string) => inventory.entries.filter((entry) => entry.sourcePath === root || entry.sourcePath.startsWith(`${root}/`));
    const implementation = nodes(implementationRoot), fixtures = nodes(fixtureRoot);
    expect(implementation).toHaveLength(pair.implementationNodes);
    expect(fixtures).toHaveLength(pair.fixtureNodes);
    expect([...implementation, ...fixtures].every((entry) => entry.sourcePath === entry.normalizedPath)).toBe(true);
    expect([...implementation, ...fixtures].flatMap((entry) => entry.violations).filter((violation) => /^(?:mutation|projection)-/u.test(violation.code))).toEqual([]);
    expect(fixtures.filter((entry) => entry.nodeKind === "file").map((entry) => relative(resolve(repoRoot, fixtureRoot), resolve(repoRoot, entry.sourcePath)).replaceAll("\\", "/")).sort()).toEqual([...pair.fixtureLeaves].sort());
    for (const path of [implementationRoot, fixtureRoot, ...pair.fixtureLeaves.map((leaf: string) => `${fixtureRoot}/${leaf}`)]) expect(lstatSync(join(repoRoot, path)).isSymbolicLink(), path).toBe(false);
    expect(inventory.entries.some((entry) => /\/🧪️tests\/🪆️[^/]+\//u.test(entry.sourcePath))).toBe(false);
    const ancestors: string[] = [];
    for (let path = dirname(fixtureRoot); path.startsWith(`${fixtureMirror}/`); path = dirname(path)) ancestors.push(path);
    const mirrored = inventory.entries.filter((entry) => entry.sourcePath === pair.subset || ancestors.includes(entry.sourcePath));
    expect(mirrored).toHaveLength(ancestors.length + 1);
    const refused = Object.hasOwn(domainOwners, `${pair.subset}/🧬️schema/🧬️mutations`) ? /^(?:directory-kind-unresolved|mutation-payload-schema-authority-invalid) /u : /^mutation-payload-schema-authority-invalid /u;
    expect(mirrored.flatMap((entry) => entry.violations.map((violation) => `${violation.code} ${entry.sourcePath}`)).filter((row) => refused.test(row))).toEqual([]);
  }, 180_000);
