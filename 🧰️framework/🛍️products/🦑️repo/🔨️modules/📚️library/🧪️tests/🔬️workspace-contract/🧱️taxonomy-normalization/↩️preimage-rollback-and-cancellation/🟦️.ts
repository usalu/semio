/** ↩️ The stale preimage, failure stage rollback and cancellation retry tests. */
import { expect, test } from "bun:test";
import { rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { applyTaxonomyPlan } from "../../../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { opaqueTreeDigest } from "../../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { planTaxonomy } from "../../../../🧹️normalization/🧠️planning/🟦️.ts";
import { verifyTaxonomy } from "../../../../🧹️normalization/✅️verification/🟦️.ts";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { expectNormalizationApplyFailure, normalizationFixture, normalizationPlan, normalizationWorkspaceSnapshot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🛑 Registers the stale preimage block, the failure stage rollbacks and the cancellation rollback with a converging retry. */
export function registerPreimageRollbackAndCancellationTests(): void {
  test("stale preimages block apply without changing source bytes", () => {
    const fixture = normalizationFixture("stale", { "🦀️component.rs": "pub const VALUE: i32 = 1;\n" });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      expect(plan.moves.length).toBeGreaterThan(0);
      writeFileSync(join(fixture.workspace, "🦀️component.rs"), "pub const VALUE: i32 = 2;\n");
      const before = normalizationWorkspaceSnapshot(fixture.workspace);
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest }));
      expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
      expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });
  for (const failureStage of ["after-staging", "after-moves", "after-edits", "before-verify"] as const) {
    test(`rolls back byte-for-byte when failure is injected at ${failureStage}`, () => {
      const fixture = normalizationFixture(`failure-${failureStage}`, {
        "🟦️subject.ts": "export const value = 1;\n",
        "🧪️consumer/🟦️.ts": "export { value } from \"🧪️tests/🧪️fixture/🟦️subject.ts\";\n",
      });
      try {
        const { plan } = normalizationPlan(fixture);
        expect(plan.moves.length).toBeGreaterThan(0);
        const before = normalizationWorkspaceSnapshot(fixture.workspace);
        expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, injectFailureAt: failureStage }));
        expect(normalizationWorkspaceSnapshot(fixture.workspace)).toEqual(before);
        expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
      } finally {
        rmSync(fixture.root, { recursive: true, force: true });
      }
    });
  }
  test("cancellation rolls back and a successful retry converges to an empty second plan", () => {
    const cancelled = normalizationFixture("cancel", { "🟦️.ts": "export const value = 1;\n" });
    try {
      const { plan } = normalizationPlan(cancelled);
      const before = normalizationWorkspaceSnapshot(cancelled.workspace);
      const cancelFile = join(cancelled.ticketDir, "cancel");
      writeFileSync(cancelFile, "cancel\n");
      expectNormalizationApplyFailure(() => applyTaxonomyPlan(plan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, cancelFile }));
      expect(normalizationWorkspaceSnapshot(cancelled.workspace)).toEqual(before);
      expect(opaqueTreeDigest(cancelled.repoRoot, "compose")).toEqual(cancelled.opaqueDigest);
    } finally {
      rmSync(cancelled.root, { recursive: true, force: true });
    }

    const fixture = normalizationFixture("convergence", { "🦀️.rs": "pub const VALUE: i32 = 1;\n" });
    try {
      const { plan } = normalizationPlan(fixture);
      expect(plan.unresolved).toEqual([]);
      const result = applyTaxonomyPlan(plan, { repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest });
      expect(result.planDigest).toBe(plan.planDigest);
      expect(result.appliedMoves).toBe(plan.moves.length);
      expect(result.appliedEdits).toBe(plan.edits.length);
      expect(String(result.state)).toMatch(/^(?:applied|committed|complete)$/u);
      const verification = verifyTaxonomy({ ...fixture.options, baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(canonicalJson(verification)).not.toMatch(/"severity":"error"/u);
      const secondInventory = inventoryTaxonomy(fixture.options);
      const secondPlan = planTaxonomy(secondInventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [fixture.opaqueDigest] });
      expect(secondPlan.moves).toEqual([]);
      expect(secondPlan.edits).toEqual([]);
      expect(secondPlan.regenerations).toEqual([]);
      expect(secondPlan.unresolved).toEqual([]);
      expect(opaqueTreeDigest(fixture.repoRoot, "compose")).toEqual(fixture.opaqueDigest);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  }, 120_000);
}
