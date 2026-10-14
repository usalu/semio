import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import glob from "fast-glob";
import { minimatch } from "minimatch";
import { discoverTestCases, repoRootFromHere, testTaxonomy, validateCaseContract, type OracleRegistry } from "../../🟦️.ts";
import { discoverCaseDirs } from "../../🟨️.mjs";
import examples from "../🧫️fixtures/🧫️case-fixtures/🔣️.json";
const real = repoRootFromHere(), vocabulary = testTaxonomy(real);
const taxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const original = readFileSync(join(real, taxonomyPath), "utf8");
const kind = vocabulary.fileKinds[vocabulary.testFeatureFileKindId]!;
const featureName = kind.emoji + kind.extensionChains[0];
const registry: OracleRegistry = { schemaVersion: 1, oracles: [], probes: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], testEvidence: [], contributions: [] };
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("Caller-owned ticket output required for case fixture verification");
mkdirSync(output, { recursive: true });
for (const row of examples.cases) test(row.id, () => {
  const root = mkdtempSync(join(output, "cf-"));
  const put = (path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
  try {
    put(taxonomyPath, original);
    for (const path of row.features) put(path + "/" + featureName, "Feature: Illustrative specimen\n");
    for (const child of row.children) mkdirSync(join(root, child.case, child.name), { recursive: true });
    const physical = glob.sync("**/" + featureName, { cwd: root, onlyFiles: true, followSymbolicLinks: false });
    const oracle = physical.filter(path => {
      const parts = path.split("/"), at = parts.length - 3;
      return parts[at] === vocabulary.testsDirName && parts[at - 1] !== vocabulary.schemaScopeOwnerLevels.moduleMemberDirName && parts[at + 1] !== vocabulary.testFixturesDirName && !parts.slice(0, at + 1).some((name, index) => name === vocabulary.testFixturesDirName && parts[index - 1] !== vocabulary.schemaScopeOwnerLevels.moduleMemberDirName) && minimatch(path, "**/" + vocabulary.testsDirName + "/*/" + featureName, { dot: true });
    }).map(path => dirname(path)).sort();
    const actual = discoverTestCases(root);
    expect(oracle).toEqual([...row.expected].sort());
    expect(actual.map(row => row.caseDir).sort()).toEqual(oracle);
    expect(discoverCaseDirs(root)).toEqual(oracle);
    const unknown = actual.flatMap(entry => validateCaseContract(root, entry, registry).filter(finding => finding.id === "unknown-case-child").map(finding => finding.scope)).sort();
    expect(unknown).toEqual([...row.unknown].sort());
    console.log("[DEBUG] opaque fixture discovery " + JSON.stringify({ id: row.id, actual: actual.map(row => row.caseDir), oracle, unknown }));
  } finally { rmSync(root, { recursive: true, force: true }); }
});
