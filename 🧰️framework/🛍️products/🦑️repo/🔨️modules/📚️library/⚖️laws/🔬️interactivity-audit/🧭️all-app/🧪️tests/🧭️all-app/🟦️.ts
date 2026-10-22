/** 🧭️ The all-app interactivity gate reads app descriptors, extension bundles, playground catalogs, the renderer axis and the verify gate manifest exactly as their language-agnostic corpus says, and an independent Python port reproduces every row; the JSON oracle agrees with the built-in JSON parser. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { gitSpawnEnv } from "../../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import * as audit from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const rules: Record<string, (root: string, ...args: any[]) => unknown> = {
  descriptor: (_root, file: string, source: string, ext = "") => audit.interactivityAllAppDescriptorFromSource(file, source, ext),
  descriptorCounts: (_root, file: string, source: string) => { const r = audit.interactivityAllAppDescriptorFromSource(file, source); return { failures: r.failures, appIds: r.row?.appIds.length ?? null, actions: r.row?.actions.length ?? null }; },
  disposition: (_root, descriptors: any[]) => audit.interactivityAllAppActionDispositionFailures(descriptors),
  production: (_root, descriptors: any[], commands: any[], routes: string[]) => audit.interactivityAllAppActionProductionFailures(descriptors, commands, routes),
  playgrounds: (_root, source: string) => audit.interactivityAllAppPlaygroundsFromSource(source),
  renderers: (_root, source: string) => audit.interactivityAllAppRenderersFromSource(source),
  surfaces: (_root, playground: any, renderers: string[]) => audit.interactivityAllAppPlaygroundSurfaces(playground, renderers),
  coverage: (_root, descriptors: any[], playgrounds: any[], renderers: string[]) => audit.interactivityAllAppSurfaceCoverageFailures(descriptors, playgrounds, renderers),
  gates: (_root, manifest: string, schema: string, gates?: any[]) => (gates === undefined ? audit.interactivityAllAppGateFailures(manifest, schema) : audit.interactivityAllAppGateFailures(manifest, schema, gates)),
  discovery: (root) => { const r = audit.interactivityAllAppDiscovery(root); return { ...r, descriptors: [...r.descriptors].sort((a, b) => (a.file < b.file ? -1 : a.file > b.file ? 1 : 0)) }; },
  discoverySummary: (root) => { const r = audit.interactivityAllAppDiscovery(root); return { ...r, descriptors: r.descriptors.length }; },
  constants: () => ({ apps: audit.INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY, actions: audit.INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY, gates: audit.INTERACTIVITY_ALL_APP_REQUIRED_GATES.map((gate) => ({ id: gate.id, check: [...gate.check] })) }),
  oracleJson: () => { throw new Error("oracleJson rows run in the dedicated test"); },
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

for (const row of (corpus.cases as any[]).filter((candidate) => candidate.rule !== "oracleJson")) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(run(row), "law versus corpus").toEqual(row.expect);
    if (row.oracle !== false) expect(model[row.id], "oracle versus corpus").toEqual(row.expect);
  }, row.git ? 120000 : 30000);
}

for (const row of (corpus.cases as any[]).filter((candidate) => candidate.rule === "oracleJson")) {
  test(`oracleJson: ${row.id}`, async () => {
    if (row.expect === null) {
      await expect(audit.interactivityAllAppOracleJson(row.args[0])).rejects.toThrow();
      expect(() => JSON.parse(row.args[0])).toThrow();
      return;
    }
    expect(await audit.interactivityAllAppOracleJson(row.args[0])).toBe(row.expect);
    expect(JSON.stringify(JSON.parse(row.args[0]))).toBe(row.expect);
  });
}
