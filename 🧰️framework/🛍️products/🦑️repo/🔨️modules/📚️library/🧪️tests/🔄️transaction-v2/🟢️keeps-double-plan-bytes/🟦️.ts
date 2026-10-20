/** 🟢️ The keeps double plan bytes tests of the transaction plan journal v2 aggregate suite. */
import { expect, test } from "bun:test";
import { rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { applyTaxonomyPlan } from "../../../🧹️normalization/🚀️apply/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { plan, referenceFixture, retainFixture, snapshot, transactionRoot } from "../🏗️fixture-builders/🟦️.ts";
import { attemptJournal, expectBoundaryGolden, expectTerminalAttempts } from "../💥️child-control/🟦️.ts";

/** 🔵 Registers the keeps double plan bytes tests of the transaction plan journal v2 aggregate suite. */
export function registerKeepsDoublePlanBytesTests(): void {
  test.concurrent("keeps double-plan bytes stable, cancellation exact, and committed second apply immutable", () => {
    const stable = referenceFixture("double-plan"), cancelled = referenceFixture("cancelled"), committed = referenceFixture("second-apply");
    try {
      const first = plan(stable), second = plan(stable);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(first.planDigest).toBe(second.planDigest);
      const cancelledPlan = plan(cancelled), cancelledBefore = snapshot(cancelled.workspace), cancelFile = join(cancelled.ticketDir, "cancel");
      let cancellationFired = false;
      expect(cancelledPlan.moves.length).toBeGreaterThan(0);
      expect(applyTaxonomyPlan(cancelledPlan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: cancelled.baselineCommit, cancelFile, progress: (event) => { if (event.phase === "staging") { cancellationFired = true; writeFileSync(cancelFile, "cancel\n"); } } }).state).toBe("rolled-back");
      expect(cancellationFired).toBe(true);
      rmSync(cancelFile);
      expect(snapshot(cancelled.workspace)).toEqual(cancelledBefore);
      expectTerminalAttempts(cancelled, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(cancelled)), snapshot(cancelled.workspace), "rolledback:cancellation", cancelled.root);
      const cancelledTransaction = snapshot(transactionRoot(cancelled));
      expect(() => applyTaxonomyPlan(cancelledPlan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: cancelled.baselineCommit, resumeJournal: attemptJournal(cancelled, cancelledPlan) })).toThrow(/rolled-back/u);
      expect(snapshot(transactionRoot(cancelled))).toEqual(cancelledTransaction);
      expect(snapshot(cancelled.workspace)).toEqual(cancelledBefore);
      const committedPlan = plan(committed);
      expect(applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit }).state).toBe("committed");
      const committedTransaction = snapshot(transactionRoot(committed)), committedWorkspace = snapshot(committed.workspace);
      expect(() => applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit })).toThrow(/already has a committed/u);
      expect(snapshot(transactionRoot(committed))).toEqual(committedTransaction);
      expect(snapshot(committed.workspace)).toEqual(committedWorkspace);
    } finally { retainFixture(stable.root); retainFixture(cancelled.root); retainFixture(committed.root); }
  }, 15_000);
  test.concurrent("recovers caught allocation and journal previous-image callback failures", () => {
    const preparation = referenceFixture("caught-preparation"), published = referenceFixture("caught-published"), previous = referenceFixture("caught-previous");
    try {
      const preparationPlan = plan(preparation), preparationWorkspace = snapshot(preparation.workspace), preparationTransaction = snapshot(transactionRoot(preparation));
      expect(() => applyTaxonomyPlan(preparationPlan, { repoRoot: preparation.repoRoot, ticketDir: preparation.ticketDir, expectedBaselineCommit: preparation.baselineCommit, progress: (event) => { if (event.phase === "transaction-attempt-preparation-mkdir") throw new Error("caught preparation"); } })).toThrow(/caught preparation/u);
      expect(snapshot(preparation.workspace)).toEqual(preparationWorkspace);
      expect(snapshot(transactionRoot(preparation))).toEqual(preparationTransaction);
      const publishedPlan = plan(published), publishedWorkspace = snapshot(published.workspace);
      expect(applyTaxonomyPlan(publishedPlan, { repoRoot: published.repoRoot, ticketDir: published.ticketDir, expectedBaselineCommit: published.baselineCommit, progress: (event) => { if (event.phase === "transaction-attempt-canonical-published") throw new Error("caught published"); } }).state).toBe("rolled-back");
      expect(snapshot(published.workspace)).toEqual(publishedWorkspace);
      expectTerminalAttempts(published, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(published)), snapshot(published.workspace), "rolledback:caught-attempt-canonical-published", published.root);
      const previousPlan = plan(previous), previousWorkspace = snapshot(previous.workspace);
      expect(applyTaxonomyPlan(previousPlan, { repoRoot: previous.repoRoot, ticketDir: previous.ticketDir, expectedBaselineCommit: previous.baselineCommit, progress: (event) => { if (event.phase === "transaction-journal-previous-exchanged") throw new Error("caught previous"); } }).state).toBe("rolled-back");
      expect(snapshot(previous.workspace)).toEqual(previousWorkspace);
      expectTerminalAttempts(previous, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(previous)), snapshot(previous.workspace), "rolledback:caught-journal-previous-exchanged", previous.root);
    } finally { retainFixture(preparation.root); retainFixture(published.root); retainFixture(previous.root); }
  }, 15_000);
}
