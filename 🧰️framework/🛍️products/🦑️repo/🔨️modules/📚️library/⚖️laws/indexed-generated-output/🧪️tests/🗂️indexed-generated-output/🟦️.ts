import { describe, expect, test } from "bun:test";
import { findRepoRoot } from "../../../../📦️packages/🟦️typescript/🟦️.ts";
import { policyIndexedGeneratedOutputViolations } from "../../🟦️.ts";

describe("indexed generated output law", () => {
  test("git does not index wasm, wasm-pack bindings, jco browser bundles, or generated trees", () => {
    const root = findRepoRoot(import.meta.dir);
    const violations = policyIndexedGeneratedOutputViolations(root);
    if (violations.length > 0) console.error("[indexed-generated-output]", violations.join("\n"));
    expect(violations).toEqual([]);
  });
});
