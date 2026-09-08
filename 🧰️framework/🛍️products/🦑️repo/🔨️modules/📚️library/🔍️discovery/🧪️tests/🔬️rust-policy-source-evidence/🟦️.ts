import { afterAll, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { interactivityProductionSource, policyMutateRustSourceEvidence, policyReadRustPolicySource, policyReadRustSourceEvidence } from "../../../../../../../../📜️script.ts";
import vector from "./🔣️.json";

const root = mkdtempSync(join(tmpdir(), "semio-rust-policy-evidence-"));
for (const [path, source] of [[vector.productionPath, vector.production], [vector.testPath, vector.test]] as const) {
  mkdirSync(dirname(join(root, path)), { recursive: true });
  writeFileSync(join(root, path), source);
}
afterAll(() => rmSync(root, { recursive: true, force: true }));

test("declared canonical Rust test evidence retains source identity", () => {
  const evidence = policyReadRustSourceEvidence(root, vector.productionPath);
  expect(evidence.production.path).toBe(vector.productionPath);
  expect(evidence.tests.map(({ path }) => path)).toEqual([vector.testPath]);
  expect(evidence.tests[0]?.source).toContain(vector.law);
  const mutated = policyMutateRustSourceEvidence(evidence, vector.law, "fn removed_law()");
  expect(mutated.production.source).toBe(evidence.production.source);
  expect(mutated.tests[0]?.source).not.toContain(vector.law);
  const policySource = policyReadRustPolicySource(root, vector.productionPath);
  expect(policySource).toContain(vector.law);
  expect(interactivityProductionSource(policySource)).not.toContain(vector.law);
});

test("rustc independently accepts and executes the declared canonical module", () => {
  const executable = join(root, process.platform === "win32" ? "oracle.exe" : "oracle");
  const compiled = Bun.spawnSync(["rustc", "--crate-name", "policy_evidence_oracle", "--edition", "2021", "--test", join(root, vector.productionPath), "-o", executable]);
  expect(compiled.exitCode, compiled.stderr.toString()).toBe(0);
  const executed = Bun.spawnSync([executable]);
  expect(executed.exitCode, executed.stderr.toString()).toBe(0);
}, 30_000);
