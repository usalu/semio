/** 🏛️ The plugin-consistency foundation helpers (path normalisation, region and module span parsing, brace extraction, script-owner discovery) return exactly what their language-agnostic corpus says, agree with an independent Python model, and one recorded dot-prefix defect is pinned as a failing row. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { gitSpawnEnv } from "../../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { policyAllRustFiles, policyAppIdFromCrateDir, policyExtractFnBody, policyIsRootScriptSource, policyLabelName, policyModAtLine, policyNewSurfacePriority, policyNormalizeRelPath, policyPackageScriptModulePaths, policyPairRegionSpans, policyParseModSpans, policyParseRegionEvents, policyPascalAppStructName, policyRootScriptModulePaths } from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const rules: Record<string, (root: string, ...args: any[]) => unknown> = {
  policyAppIdFromCrateDir: (root, ...args) => (policyAppIdFromCrateDir as any)(...args),
  policyNormalizeRelPath: (root, ...args) => (policyNormalizeRelPath as any)(...args),
  policyNewSurfacePriority: (root, ...args) => (policyNewSurfacePriority as any)(...args),
  policyParseRegionEvents: (root, ...args) => (policyParseRegionEvents as any)(...args),
  policyPairRegionSpans: (root, ...args) => (policyPairRegionSpans as any)(...args),
  policyParseModSpans: (root, ...args) => (policyParseModSpans as any)(...args),
  policyModAtLine: (root, ...args) => (policyModAtLine as any)(...args),
  policyLabelName: (root, ...args) => (policyLabelName as any)(...args),
  policyExtractFnBody: (root, ...args) => (policyExtractFnBody as any)(...args),
  policyPascalAppStructName: (root, ...args) => (policyPascalAppStructName as any)(...args),
  policyIsRootScriptSource: (root, ...args) => (policyIsRootScriptSource as any)(...args),
  policyRootScriptModulePaths: (root, ...args) => (policyRootScriptModulePaths as any)(root, ...args),
  policyPackageScriptModulePaths: (root, ...args) => (policyPackageScriptModulePaths as any)(root, ...args),
  policyAllRustFiles: (root, ...args) => (policyAllRustFiles as any)(root, ...args),
};
const artifacts = realpathSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir()));
const libraryRoot = resolve(import.meta.dir, "../../../../.."), libraryRel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const canon = (value: unknown): unknown => (value === undefined ? null : JSON.parse(JSON.stringify(value)));
const admit = (root: string) => {
  for (const rel of ["🔣️taxonomy.json", "🧬️schema/🔣️.json"]) { mkdirSync(dirname(join(root, libraryRel, rel)), { recursive: true }); copyFileSync(join(libraryRoot, rel), join(root, libraryRel, rel)); }
  for (const args of [["init", "--quiet"], ["add", "-A"]]) expect(spawnSync("git", args, { cwd: root, env: gitSpawnEnv(), encoding: "utf8" }).status).toBe(0);
};
const run = (row: any) => {
  const root = mkdtempSync(join(artifacts, "lawcov-"));
  try {
    for (const [path, lines] of Object.entries((row.files ?? {}) as Record<string, string[]>)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), lines.length ? lines.join("\n") + "\n" : ""); }
    if (row.git) admit(root);
    return canon(rules[row.rule]!(root, ...(row.args ?? [])));
  } finally { rmSync(root, { recursive: true, force: true }); }
};
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍️oracle.py"), resolve(import.meta.dir, "./🔣️.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(run(row), "law versus corpus").toEqual(row.expect);
    if (row.oracle !== false) expect(model[row.id], "oracle versus corpus").toEqual(row.expect);
  });
}
