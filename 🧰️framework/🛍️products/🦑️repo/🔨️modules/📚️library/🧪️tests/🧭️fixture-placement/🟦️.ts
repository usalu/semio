import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import fg from "fast-glob";
import { minimatch } from "minimatch";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { runtimeFixturePathV1 } from "../../🔍️discovery/🕸️runtime/🟦️.ts";
import { inspectTestLayoutSources } from "@semio-tech/repo-test";
import { testTaxonomy } from "@semio-tech/repo-test";
const root = resolve(import.meta.dir, "../../../../../../..");
const taxonomy = testTaxonomy(root);
const examples = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧭️fixture-placement/🔣️.json", import.meta.url), "utf8")) as { cases: { path: string; outside: boolean }[]; physicalScopes: string[] };
for (const row of examples.cases) test("fixture collection ancestry: " + row.path, () => {
  const pieces = row.path.split("/");
  const collections = pieces.flatMap((name, index) => name === taxonomy.testFixturesDirName && pieces[index - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName ? [index] : []);
  const tests = pieces.flatMap((name, index) => name === taxonomy.testsDirName && pieces[index - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName ? [pieces.slice(0, index + 1).join("/")] : []);
  const oracle = collections.some(index => !tests.some(prefix => minimatch(pieces.slice(0, index + 1).join("/"), prefix + "/**", { dot: true })));
  const actual = inspectTestLayoutSources(taxonomy, [], [row.path]).some(row => row.code === "fixture-outside-tests");
  expect(oracle).toBe(row.outside);
  expect(actual).toBe(row.outside);
});
test("all semantic-owner examples have true test ancestry", () => {
  const scopes = examples.physicalScopes;
  const ignores = ["**/node_modules/**", "**/.git/**", "**/target/**", "**/dist/**", "**/build/**", "**/out/**", "**/🗑️*/**"];
  const paths: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(resolve(root, directory), { withFileTypes: true })) {
      const path = directory + "/" + entry.name;
      if (ignores.some(pattern => minimatch(path, pattern, { dot: true }))) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) paths.push(path);
    }
  };
  for (const scope of scopes) walk(scope);
  const thirdParty = fg.sync(scopes.map(scope => scope + "/**"), { cwd: root, dot: true, onlyFiles: true, followSymbolicLinks: false, ignore: ignores });
  const names = new Set(["🧫️fixtures", "🧪️fixtures", "🧫️examples", "🧪️examples"]);
  const example = (path: string): boolean => path.split("/").some((name, index, all) => names.has(name) && all[index - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName);
  const physicalExamples = paths.filter(example);
  expect(physicalExamples.sort()).toEqual(thirdParty.filter(example).sort());
  const misplaced = physicalExamples.filter(path => {
    const parts = path.split("/");
    const at = parts.findIndex((name, index) => names.has(name) && parts[index - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName);
    return !parts.slice(0, at).some((name, index) => name === taxonomy.testsDirName && parts[index - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName);
  });
  expect(misplaced).toEqual([]);
}, 60000);

test("production coordinate metadata never enrolls fixture payload authority", () => {
  const contracts = Object.values(loadCatalogTaxonomy().frozenCoordinateEvidenceContracts);
  const archived = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧭️fixture-placement/📜️enrollment.json", import.meta.url), "utf8")).records;
  for (const row of Object.values(archived) as { path: string }[]) expect(runtimeFixturePathV1(row.path)).toBe(true);
  const actual = contracts.filter(row => runtimeFixturePathV1(row.path)).map(row => row.path);
  const names = new Set(["🧫️fixtures", "🧪️fixtures", "🧫️examples", "🧪️examples"]);
  const oracle = contracts.filter(row => row.path.split("/").some((name, at, parts) => names.has(name) && parts[at - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName && minimatch(row.path, "**/" + name + "/**", { dot: true }))).map(row => row.path);
  expect(actual).toEqual(oracle);
  expect(actual).toEqual([]);
});
