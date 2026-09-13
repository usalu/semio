import { expect, test } from "bun:test";
import Ajv from "ajv";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, posix } from "node:path";
import { tmpdir } from "node:os";
import { mutationVectorRegistryBreaches, repoRootFromHere, testTaxonomy, type OracleRegistry } from "../../📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🧬️mutation-fixtures/🔣️.json";
import htmlPairs from "../../🧫️fixtures/🌐️html-source-pair/🔣️.json";
import htmlPairSchema from "../../🧫️fixtures/🌐️html-source-pair/🧬️schema/🔣️.json";
import { parse, serialize } from "parse5";
import { loadCatalogTaxonomy, semanticDescendantNodeRelativePath, semanticDirectoryKindId } from "../../../📚️library/🔍️discovery/🟦️.ts";
import { verifyFixture } from "../../🟦️.ts";
import { inventoryTaxonomy } from "../../../📚️library/🧹️normalization/🟦️.ts";

const taxonomy = testTaxonomy(repoRootFromHere());
const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: ["change-value"], vectors: [{ mutationId: "change-value", sourceMutationDirectoryName: vectors.mutation, mutationDirectoryName: vectors.mutation, scenarios: [{ id: "changes-the-value", directoryName: vectors.scenario }] }] };
const contribution = { owner: vectors.owner, manifestPath: `${vectors.owner}/🔣️oracle.json`, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], migrationStatus: {} };
const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [contribution] } as unknown as OracleRegistry;

test("mutation fixture examples satisfy the owning schema", () => {
  const valid = new Ajv({ strict: true }).compile(schema.$defs.MutationFixtureCases);
  expect(valid(vectors)).toBe(true);
});

test("HTML source pair controls preserve the six-node semantic boundary", () => {
  const validate = new Ajv({ strict: true }).compile(htmlPairSchema);
  expect(validate(htmlPairs), JSON.stringify(validate.errors)).toBe(true);
  const catalog = loadCatalogTaxonomy();
  expect(semanticDescendantNodeRelativePath({ pathSegments: [], nodeType: "file", kindId: "html" }, catalog)).toBe("🌐️.html");
  expect(semanticDescendantNodeRelativePath({ pathSegments: [], nodeType: "file", kindId: "json" }, catalog)).toBe("🔣️.json");
  expect(() => semanticDescendantNodeRelativePath({ pathSegments: [], nodeType: "file", kindId: "missing-file-kind" }, catalog)).toThrow("primary extension chain");
  const contract = catalog.semanticDescendantContracts[htmlPairs.descendantContract];
  expect(contract).toBeDefined();
  if (!contract || "contractKind" in contract) throw new Error("HTML source pair requires explicit descendants");
  expect(contract.rootDirectoryKindId).toBe(htmlPairs.fixtureKind);
  expect(contract.realizedNodeCount).toBe(6);
  expect(contract.exclusiveAlternatives).toEqual([]);
  expect(contract.requiredNodes.map(node => ({ path: semanticDescendantNodeRelativePath(node, catalog), type: node.nodeType, kind: "kindId" in node ? node.kindId : null }))).toEqual(htmlPairs.nodes);
  expect([...catalog.semanticDirectoryMemberKinds[htmlPairs.fixtureKind]!.memberNames].sort()).toEqual(htmlPairs.pairs.map(pair => pair.directoryName).sort());
  for (const pair of htmlPairs.pairs) {
    expect(semanticDirectoryKindId(pair.directoryName, catalog, { parentKindId: "fixtures" }), pair.id).toBe(htmlPairs.fixtureKind);
    expect(semanticDirectoryKindId(pair.directoryName, catalog, { parentKindId: "packages" }), pair.id).not.toBe(htmlPairs.fixtureKind);
  }
  expect(semanticDirectoryKindId("🧪️unregistered-html-pair", catalog, { parentKindId: "fixtures" })).not.toBe(htmlPairs.fixtureKind);
}, 15000);

test("HTML source pair normalization accepts the primary leaf of a multi-extension kind", () => {
  const root = repoRootFromHere();
  const scope = posix.join(posix.dirname(htmlPairs.manifestPath), "../🧫️fixtures");
  const owners = htmlPairs.pairs.map(pair => `${scope}/${pair.directoryName}/`);
  const expected = htmlPairs.pairs.flatMap(pair => htmlPairs.nodes.filter(node => node.type === "file").map(node => posix.join(scope, pair.directoryName, node.path))).sort();
  const inventory = inventoryTaxonomy({ repoRoot: root, scope, workers: 1 });
  const leaves = inventory.entries.filter(entry => entry.nodeKind === "file" && owners.some(owner => entry.sourcePath.startsWith(owner)));
  expect(leaves.map(entry => entry.sourcePath).sort()).toEqual(expected);
  for (const leaf of leaves) {
    expect(leaf.fileKind, leaf.sourcePath).toBe("html");
    expect(leaf.normalizedPath, leaf.sourcePath).toBe(leaf.sourcePath);
    expect(leaf.violations, leaf.sourcePath).toEqual([]);
  }
}, 30_000);

test("HTML source pair manifests retain exact native-reader and parser inputs", async () => {
  const root = repoRootFromHere();
  const manifest = JSON.parse(readFileSync(join(root, htmlPairs.manifestPath), "utf8"));
  expect(manifest.fixtureManifests.map((fixture: { id: string }) => fixture.id).sort()).toEqual(htmlPairs.pairs.map(pair => pair.id).sort());
  for (const pair of htmlPairs.pairs) {
    const fixture = manifest.fixtureManifests.find((candidate: { id: string }) => candidate.id === pair.id);
    const owner = join(root, dirname(htmlPairs.manifestPath), "../🧫️fixtures", pair.directoryName);
    const realized: { path: string; type: string }[] = [];
    const visit = (relative: string): void => {
      const path = join(owner, relative), stat = lstatSync(path);
      expect(stat.isSymbolicLink(), path).toBe(false);
      realized.push({ path: relative, type: stat.isDirectory() ? "directory" : "file" });
      if (stat.isDirectory()) for (const entry of readdirSync(path)) visit(relative ? `${relative}/${entry}` : entry);
      else expect(stat.isFile(), path).toBe(true);
    };
    visit("");
    const expected = htmlPairs.nodes.map(({ path, type }) => ({ path, type }));
    expect(realized.sort((a, b) => a.path.localeCompare(b.path))).toEqual(expected.sort((a, b) => a.path.localeCompare(b.path)));
    const exactShape = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: expected.map(node => node.path), properties: Object.fromEntries(expected.map(node => [node.path, { const: node.type }])) });
    expect(exactShape(Object.fromEntries(realized.map(node => [node.path, node.type])))).toBe(true);
    expect(fixture.files).toHaveLength(2);
    expect(verifyFixture(root, { ...fixture, manifestDir: dirname(htmlPairs.manifestPath) }).every(result => result.ok)).toBe(true);
    for (const [index, phase] of ["⬅️before", "➡️after"].entries()) {
      const role = index === 0 ? "expected-before-html" : "expected-after-html";
      const file = fixture.files.find((candidate: { role: string }) => candidate.role === role);
      expect(file.path).toBe(`../🧫️fixtures/${pair.directoryName}/📸️snapshot/${phase}/🌐️.html`);
      const bytes = readFileSync(join(root, dirname(htmlPairs.manifestPath), file.path));
      expect(bytes.length).toBe(file.bytes);
      expect(`sha256:${Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")}`).toBe(file.sha256);
      const canonical = serialize(parse(bytes.toString("utf8")));
      expect(serialize(parse(canonical))).toBe(canonical);
    }
  }
});

test("HTML source pair readers declare every external Nx cache input", () => {
  const root = repoRootFromHere();
  const files = htmlPairs.pairs.flatMap(pair => ["⬅️before", "➡️after"].map(phase => posix.join(posix.dirname(htmlPairs.manifestPath), "../🧫️fixtures", pair.directoryName, "📸️snapshot", phase, "🌐️.html")));
  const expected = [htmlPairs.manifestPath, ...files].map(path => `{workspaceRoot}/${path}`).sort();
  for (const consumer of htmlPairs.nxConsumers) {
    const project = JSON.parse(readFileSync(join(root, consumer.projectPath), "utf8"));
    expect(project.namedInputs?.[consumer.inputSet]?.toSorted(), consumer.projectPath).toEqual(expected);
    for (const target of consumer.targets) {
      expect(project.targets[target].inputs, `${consumer.projectPath}:${target}`).toContain(consumer.inputSet);
      for (const input of consumer.sourceInputs) expect(project.targets[target].inputs, `${consumer.projectPath}:${target}`).toContain(`{workspaceRoot}/${input}`);
    }
  }
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
