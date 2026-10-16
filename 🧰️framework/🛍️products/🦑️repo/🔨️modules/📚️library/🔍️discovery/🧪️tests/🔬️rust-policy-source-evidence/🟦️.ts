import { afterAll, expect, test } from "bun:test";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { interactivityProductionSource } from "../../../⚖️laws/🔬️interactivity-audit/🗄️database/🟦️.ts";
import { policyMutateRustSourceEvidence, policyReadRustPolicySource, policyReadRustSourceEvidence } from "../../📖️source-access/🦀️rust-evidence/🟦️.ts";
import { POLICY_SPLIT_SOURCE_ROOTS, policyModuleTreeFiles, policyReadModuleTreeSource } from "../../📖️source-access/🟦️.ts";
import vector from "../🧫️fixtures/🔬️rust-policy-source-evidence/🔣️.json";

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

test("a split source reads as one text and exposes the canonical tests its children declare", () => {
  const split = vector.splitTree;
  for (const [path, source] of Object.entries(split.files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), source);
  }
  const evidence = policyReadRustSourceEvidence(root, split.productionPath);
  expect(evidence.production.path).toBe(split.productionPath);
  expect(evidence.production.source).toContain("pub fn head()");
  expect(evidence.production.source).toContain(split.childOnly);
  expect(evidence.production.source.indexOf("pub fn head()")).toBeLessThan(evidence.production.source.indexOf(split.childOnly));
  expect(evidence.tests.map(({ path }) => path)).toEqual([split.testPath]);
  expect(evidence.tests[0]?.source).toContain(split.law);
  expect(evidence.production.source).not.toContain(split.law);
  expect(policyReadRustPolicySource(root, split.productionPath)).toContain(split.law);
  for (const [path, source] of Object.entries(split.unregistered.files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), source);
  }
  const unregistered = policyReadRustSourceEvidence(root, split.unregistered.productionPath);
  expect(unregistered.production.source).toContain("pub fn head()");
  expect(unregistered.production.source).not.toContain(split.unregistered.mountedOnly);
  const mutated = policyMutateRustSourceEvidence(evidence, split.childOnly, "pub fn removed() {}");
  expect(mutated.production.source).not.toContain(split.childOnly);
  expect(mutated.tests[0]?.source).toContain(split.law);
});

test("every declared split source root names a real file that reads as one text", () => {
  const repoRoot = join(import.meta.dir, "../../../../../../../..");
  for (const path of POLICY_SPLIT_SOURCE_ROOTS) {
    expect(existsSync(join(repoRoot, path)), path).toBe(true);
    const files = policyModuleTreeFiles(repoRoot, path);
    expect(files[0], path).toBe(path);
    expect(policyReadModuleTreeSource(repoRoot, path).length, path).toBeGreaterThanOrEqual(readFileSync(join(repoRoot, path), "utf8").length);
  }
}, 60_000);

test("cfg(test) attributes hide exactly their test-only items, same-line or negated", () => {
  const production = interactivityProductionSource(vector.cfgTestItems.source);
  for (const kept of vector.cfgTestItems.production) expect(production).toContain(kept);
  for (const hidden of vector.cfgTestItems.test) expect(production).not.toContain(hidden);
});

test("rustc independently compiles the cfg(test) item vector with and without the test harness", () => {
  const path = join(root, vector.cfgTestItems.path);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, vector.cfgTestItems.source);
  for (const harness of [["--crate-type", "lib"], ["--test"]]) {
    const compiled = Bun.spawnSync(["rustc", "--crate-name", "policy_cfg_test_oracle", "--edition", "2021", "-A", "warnings", ...harness, path, "-o", join(root, `cfg-test-oracle-${harness.at(-1)}`)]);
    expect(compiled.exitCode, compiled.stderr.toString()).toBe(0);
  }
}, 30_000);

test("rustc independently accepts and executes the declared canonical module", () => {
  const executable = join(root, process.platform === "win32" ? "oracle.exe" : "oracle");
  const compiled = Bun.spawnSync(["rustc", "--crate-name", "policy_evidence_oracle", "--edition", "2021", "--test", join(root, vector.productionPath), "-o", executable]);
  expect(compiled.exitCode, compiled.stderr.toString()).toBe(0);
  const executed = Bun.spawnSync([executable]);
  expect(executed.exitCode, executed.stderr.toString()).toBe(0);
}, 30_000);
