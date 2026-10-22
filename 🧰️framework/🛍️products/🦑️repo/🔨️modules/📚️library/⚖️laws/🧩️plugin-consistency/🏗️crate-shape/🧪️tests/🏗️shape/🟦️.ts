/** 🏗️ The crate-shape laws report exactly the breaches their language-agnostic Rust-source corpus names (regions, manifest, naming, layout, SDK delegation), agree with an independent Python model and reject a malformed corpus. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { policyArtifactAppLawDelegateBreaches, policyLabelsStructBreaches, policyManifestRegionBreaches, policyModLayoutBreaches, policyRegionFormatBreaches, policySelectionIdsBreaches, policyStructNamingBreaches, policyTreeItemBreaches } from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

type Breach = { id: string; kind: string; priority?: string; line?: number };
const text = (lines: readonly string[]) => lines.join("\n") + "\n";
const rules: Record<string, (row: any) => readonly Breach[]> = {
  policyRegionFormatBreaches: (row) => policyRegionFormatBreaches(row.scope, row.source),
  policyManifestRegionBreaches: (row) => policyManifestRegionBreaches(row.scope, row.source),
  policyStructNamingBreaches: (row) => policyStructNamingBreaches(row.scope, text(row.source)),
  policyModLayoutBreaches: (row) => policyModLayoutBreaches(row.scope, row.source),
  policySelectionIdsBreaches: (row) => policySelectionIdsBreaches(row.scope, text(row.source)),
  policyArtifactAppLawDelegateBreaches: (row) => policyArtifactAppLawDelegateBreaches(row.scope, text(row.source)),
  policyTreeItemBreaches: (row) => policyTreeItemBreaches({ dir: row.crate.dir, libRelPath: "", role: "", ownerRel: "", pluginId: row.crate.pluginId }, text(row.source), row.source),
  policyLabelsStructBreaches: (row) => policyLabelsStructBreaches({ dir: row.crate.dir, libRelPath: "", role: "", ownerRel: "", pluginId: row.crate.pluginId }, text(row.source)),
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
  });
}
