import { describe, expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🗂️mutation-leaf-case-table/🔣️.json";
import { foldMutationCaseName, mutationLeafCaseTableProblems, parseMutationLeafCaseTable } from "../../🔍️discovery/🗂️mutation-leaf-case-table/🟦️.ts";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { validateMutationCasePairs } from "../../🧹️normalization/👥️mutation-projection/🟦️.ts";

const taxonomy = loadCatalogTaxonomy(), coverage = taxonomy.mutationCatalogProjection.fixtureCoverage;
const kind = taxonomy.fileKinds[coverage.leafCaseTable.fileKindId]!, tableName = `${kind.emoji}${kind.extensionChains[0]}`;
const validate = new Ajv({ strict: true }).compile(fixture.schema);
const repoRoot = execFileSync("git", ["rev-parse", "--show-toplevel"], { cwd: import.meta.dir, encoding: "utf8" }).trim();

const jsonFiles = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? jsonFiles(join(directory, entry.name)) : entry.name.endsWith(".json") ? [join(directory, entry.name)] : []);

describe("mutation leaf case table (language-agnostic fixture, Ajv cross-check)", () => {
  test("the fixture policy is the taxonomy policy", () => {
    expect(coverage.leafCaseTable).toEqual({ fileKindId: fixture.policy.fileKindId, casesField: fixture.policy.casesField });
    expect(kind.extensionChains).toEqual([fixture.policy.extensionChain]);
  });

  for (const row of fixture.parse) {
    test(`${row.id}: ${row.note}`, () => {
      const table = parseMutationLeafCaseTable(row.source, fixture.policy);
      expect("cases" in table).toBe(row.valid);
      if ("cases" in table) expect(table.cases).toEqual((row as { cases: string[] }).cases);
      else expect(table.error).toContain((row as { reason: string }).reason);
    });
    test(`${row.id}: Ajv agrees with the schema verdict`, () => {
      let document: unknown;
      try {
        document = JSON.parse(row.source);
      } catch {
        return;
      }
      expect(validate(document)).toBe(row.schemaValid);
    });
  }

  for (const row of fixture.problems) {
    test(`${row.id}: ${row.note}`, () => {
      expect(mutationLeafCaseTableProblems(row.listed, row.bundles)).toEqual(row.expected);
    });
  }
});

describe("every mutation leaf case table beside committed fixture bundles", () => {
  const levels = taxonomy.schemaScopeOwnerLevels, facets = coverage.exhaustiveCase.bundleFacetDirectoryNames;
  const mirror = `/${taxonomy.testsDirName}/${taxonomy.testFixturesDirName}/${levels.mutationsFacetDirName}/`;
  const listing = execFileSync("git", ["ls-files", "-z", "--cached", "--", `:(glob)**${mirror}**`], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 });
  const leaves = new Map<string, { root: string; owner: string }>();
  for (const path of listing.split("\0")) {
    const at = path.indexOf(mirror);
    if (at < 0) continue;
    const segments = path.slice(at + mirror.length).split("/");
    for (let depth = 1; depth < segments.length; depth++) leaves.set(`${path.slice(0, at)}\0${segments.slice(0, depth).join("/")}`, { root: path.slice(0, at), owner: segments.slice(0, depth).join("/") });
  }
  const tables = [...leaves.values()].filter(({ root, owner }) => existsSync(join(repoRoot, root, levels.facetDirName, levels.mutationsFacetDirName, owner, taxonomy.testsDirName, tableName))).sort((left, right) => `${left.root}/${left.owner}`.localeCompare(`${right.root}/${right.owner}`));

  test("the repository holds case tables", () => {
    expect(tables.length).toBeGreaterThan(0);
  });

  for (const { root, owner } of tables) {
    test(`${owner} beside ${root.split("/").slice(-5).join("/")} is a complete table of its leaf`, () => {
      const path = join(repoRoot, root, levels.facetDirName, levels.mutationsFacetDirName, owner, taxonomy.testsDirName, tableName), fixtureLeaf = join(repoRoot, root, taxonomy.testsDirName, taxonomy.testFixturesDirName, levels.mutationsFacetDirName, owner);
      const source = readFileSync(path, "utf8"), table = parseMutationLeafCaseTable(source, coverage.leafCaseTable);
      expect("error" in table ? table.error : "").toBe("");
      expect(validate(JSON.parse(source))).toBe(true);
      const bundles = readdirSync(fixtureLeaf, { withFileTypes: true }).filter((entry) => entry.isDirectory() && facets.every((facet) => existsSync(join(fixtureLeaf, entry.name, facet)))).map((entry) => entry.name);
      expect(mutationLeafCaseTableProblems("cases" in table ? table.cases : [], bundles)).toEqual([]);
      expect(new Set(bundles.map(foldMutationCaseName)).size).toBe(bundles.length);
      for (const name of bundles) {
        for (const file of jsonFiles(join(fixtureLeaf, name))) expect(() => JSON.parse(readFileSync(file, "utf8")), file).not.toThrow();
      }
    });
  }
});

describe("validateMutationCasePairs reads the case table of a leaf (scratch tree)", () => {
  const levels = taxonomy.schemaScopeOwnerLevels, facets = coverage.exhaustiveCase.bundleFacetDirectoryNames, subset = "subset", leaf = "leaf";
  const fixtureLeaf = `${subset}/${taxonomy.testsDirName}/${taxonomy.testFixturesDirName}/${levels.mutationsFacetDirName}/${leaf}`;
  const tablePath = `${subset}/${levels.facetDirName}/${levels.mutationsFacetDirName}/${leaf}/${taxonomy.testsDirName}/${tableName}`;
  const findings = (tableSource: string | null, bundles: readonly string[]): string[] => {
    const root = mkdtempSync(join(tmpdir(), "case-table-"));
    try {
      const entries = new Map<string, { sourcePath: string; normalizedPath: string; nodeKind: "file" | "directory"; violations: { code: string; path: string; message: string }[] }>();
      const add = (path: string, nodeKind: "file" | "directory"): void => void entries.set(path, { sourcePath: path, normalizedPath: path, nodeKind, violations: [] });
      for (const directory of [fixtureLeaf, ...bundles.flatMap((name) => name.startsWith("!") ? [`${fixtureLeaf}/${name.slice(1)}`] : [`${fixtureLeaf}/${name}`, ...facets.map((facet) => `${fixtureLeaf}/${name}/${facet}`)])]) add(directory, "directory");
      if (tableSource !== null) {
        add(tablePath, "file");
        mkdirSync(join(root, dirname(tablePath)), { recursive: true });
        writeFileSync(join(root, tablePath), tableSource);
      }
      validateMutationCasePairs(root, undefined, entries as never, new Map(), { schema: taxonomy, discoverySchema: taxonomy } as never);
      return [...entries.values()].flatMap((entry) => entry.violations.map((row) => `${row.code} ${entry.sourcePath.slice(entry.sourcePath.lastIndexOf("/") + 1)}`)).sort();
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  };
  const table = (...cases: string[]): string => JSON.stringify({ cases });

  test("a bundle without any coverage source is unpaired", () => {
    expect(findings(null, ["a", "b"])).toEqual([`mutation-fixture-unpaired a`, `mutation-fixture-unpaired b`]);
  });
  test("a table that lists every bundle covers them all", () => {
    expect(findings(table("a", "b", "c"), ["a", "b", "c"])).toEqual([]);
  });
  test("the listing compares names without U+FE0F", () => {
    expect(findings(table("\u{1F9F9}x"), ["\u{1F9F9}\uFE0Fx"])).toEqual([]);
  });
  test("a bundle the table does not list stays unpaired and the table is invalid", () => {
    expect(findings(table("a", "b"), ["a", "b", "c"])).toEqual([`mutation-case-table-invalid ${tableName}`, `mutation-fixture-unpaired c`]);
  });
  test("a listed case without a bundle invalidates the table", () => {
    expect(findings(table("a", "ghost"), ["a"])).toEqual([`mutation-case-table-invalid ${tableName}`]);
  });
  test("an unreadable table covers nothing", () => {
    expect(findings("{\"cases\": [", ["a"])).toEqual([`mutation-case-table-invalid ${tableName}`, `mutation-fixture-unpaired a`]);
  });
  test("a directory without all four facets is not a bundle", () => {
    expect(findings(table("a"), ["a", "!witness"])).toEqual([]);
    expect(findings(table("a", "witness"), ["a", "!witness"])).toEqual([`mutation-case-table-invalid ${tableName}`]);
  });
});
