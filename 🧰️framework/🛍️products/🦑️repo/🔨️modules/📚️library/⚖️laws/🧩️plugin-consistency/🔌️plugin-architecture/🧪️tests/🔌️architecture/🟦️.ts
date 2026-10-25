/** 🔌️ The artifacts-only plugin architecture law reports exactly the breaches its language-agnostic corpus names (shape, purity, state lanes, registration, dependency allowlist, effect capabilities, ratchet), agrees with an independent Python model, and rejects a malformed corpus. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import { policyApaBreaches, policyStateLaneExhaustivenessBreaches } from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

type Breach = { id: string; kind: string; priority?: string; line?: number; summary?: string };
const rules: Record<string, (root: string, row: any) => readonly Breach[]> = {
  policyApaBreaches: (root) => policyApaBreaches(root),
  policyStateLaneExhaustivenessBreaches: (root) => policyStateLaneExhaustivenessBreaches(root),
};
const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
const order = (rows: readonly unknown[]) => [...rows].sort((a, b) => { const x = JSON.stringify(a), y = JSON.stringify(b); return x < y ? -1 : x > y ? 1 : 0; });
const triples = (breaches: readonly Breach[]) => order(breaches.map((b) => [b.id, b.kind, b.priority ?? "", b.line ?? 0]));
const counts = (breaches: readonly Breach[]) => { const out: Record<string, number> = {}; for (const b of breaches) out[b.priority ?? ""] = (out[b.priority ?? ""] ?? 0) + 1; return out; };
const observe = (row: any, breaches: readonly Breach[]) => (row.expectCounts ? counts(breaches) : triples(breaches));
const wanted = (row: any) => (row.expectCounts ? row.expectCounts : order(row.expect));
const files = (row: any) => { const out: Record<string, string> = {}; for (const [path, lines] of Object.entries(row.files as Record<string, string[]>)) out[path] = lines.length ? lines.join("\n") + "\n" : ""; for (const g of row.generate ?? []) for (let i = 0; i < g.count; i++) out[g.path.replaceAll("{i}", String(i))] = (g.lines ?? []).map((l: string) => l.replaceAll("{i}", String(i))).join("\n"); return out; };
const run = (row: any) => {
  const root = mkdtempSync(join(artifacts, "lawcov-"));
  try {
    for (const [path, text] of Object.entries(files(row))) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); }
    return observe(row, rules[row.rule]!(root, row));
  } finally { rmSync(root, { recursive: true, force: true }); }
};
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍️.py"), resolve(import.meta.dir, "./🔣️.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(corpus.cases.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  test(`${row.rule}: ${row.id}`, () => {
    const found = run(row);
    expect(found, "law versus corpus").toEqual(wanted(row) as never);
    expect(row.expectCounts ? model[row.id] : order(model[row.id] as unknown[]), "oracle versus corpus").toEqual(wanted(row) as never);
  });
}
