/** 🗂️ The interactivity-audit pattern table, scope roots and allowlist equal what their language-agnostic corpus pins, every regex matches exactly the samples the corpus names (checked by an independent Python regex engine), and every audited file constant sits in a production root. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import { gitSpawnEnv } from "../../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import * as audit from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const rules: Record<string, (root: string, ...args: any[]) => unknown> = {
  pattern: (_root, row: any) => { const found = audit.INTERACTIVITY_AUDIT_PATTERNS.find((p) => p.label === row.label); if (!found) throw new Error("unknown pattern " + row.label); expect({ category: found.category, scope: found.scope, source: found.re.source }).toEqual({ category: row.category, scope: row.scope, source: row.source }); return row.samples.map((s: string) => { found.re.lastIndex = 0; return found.re.test(s); }); },
  constants: () => ({ INTERACTIVITY_AUDIT_UI_ROOTS: [...audit.INTERACTIVITY_AUDIT_UI_ROOTS], INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES: [...audit.INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES], INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS: [...audit.INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS], INTERACTIVITY_AUDIT_PRODUCTION_ROOTS: [...audit.INTERACTIVITY_AUDIT_PRODUCTION_ROOTS] }),
  allowlist: () => audit.INTERACTIVITY_AUDIT_ALLOWLIST.map((e) => ({ file: e.file, lineHint: e.lineHint, pattern: e.pattern, inScope: e.inScope, expectedNeverToMatch: e.expectedNeverToMatch === true })),
  fileConstants: () => Object.keys(audit).filter((name) => /^INTERACTIVITY_[A-Z0-9_]*FILE$/.test(name)).sort(),
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
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍️.py"), resolve(import.meta.dir, "./🔣️.json")], { encoding: "utf8" });
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
  }, row.git ? 120000 : 30000);
}

test("every audited file constant names a Rust leaf inside the production roots or a ticket contract", () => {
  for (const [name, value] of Object.entries(audit)) {
    if (!/^INTERACTIVITY_[A-Z0-9_]*FILE$/.test(name) || typeof value !== "string") continue;
    const production = audit.INTERACTIVITY_AUDIT_PRODUCTION_ROOTS.some((root: string) => (value as string).startsWith(root + "/"));
    expect(production || (value as string).startsWith(".\u{1F9EC}semio/"), `${name} sits in a production root or a ticket folder`).toBe(true);
  }
  expect(Object.keys(audit).filter((name) => /^INTERACTIVITY_[A-Z0-9_]*FILE$/.test(name)).sort()).toEqual((corpus.cases.find((row: any) => row.rule === "fileConstants") as any).expect);
});
