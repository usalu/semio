/** 🎯️ The mutation-outcome code law reports exactly the vocabulary breaches its language-agnostic corpus names across Rust, TypeScript, Python, feature and JSON sources, agrees with an independent Python model and rejects a malformed corpus. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { policyOutcomeCodeFileBreaches } from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

type Breach = { id: string; kind: string; priority?: string; line?: number };
const rules: Record<string, (row: any) => readonly Breach[]> = {
  policyOutcomeCodeFileBreaches: (row) => policyOutcomeCodeFileBreaches({ codes: new Map(row.vocabulary.codes.map((c: any) => [c.code, c.level])), applyPattern: new RegExp(row.vocabulary.applyPattern), applyLevel: row.vocabulary.applyLevel }, row.path, row.source.length ? row.source.join("\n") + "\n" : ""),
};
const order = (rows: readonly unknown[]) => [...rows].sort((a, b) => { const x = JSON.stringify(a), y = JSON.stringify(b); return x < y ? -1 : x > y ? 1 : 0; });
const triples = (breaches: readonly Breach[]) => order(breaches.map((b) => [b.id, b.kind, b.priority ?? "", b.line ?? 0]));
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍️oracle.py"), resolve(import.meta.dir, "./🔣️.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown[]>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(triples(rules[row.rule]!(row)), "law versus corpus").toEqual(order(row.expect) as never);
    if (row.oracle !== false) expect(order(model[row.id]!), "oracle versus corpus").toEqual(order(row.expect) as never);
  }, 30000);
}
