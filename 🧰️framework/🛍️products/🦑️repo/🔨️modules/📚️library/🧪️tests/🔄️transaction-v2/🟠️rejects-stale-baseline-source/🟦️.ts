/** 🟠️ The rejects stale baseline source tests of the transaction plan journal v2 aggregate suite. */
import { expect, test } from "bun:test";
import { chmodSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { applyTaxonomyPlan } from "../../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { type Fixture, type Snapshot } from "../📜️boundary-contracts/🟦️.ts";
import { embeddedFixture, generatorFixture, plan, referenceFixture, retainFixture, snapshot, transactionRoot } from "../🏗️fixture-builders/🟦️.ts";
import { activeReferenceFixture, attemptJournal, killedAt } from "../💥️child-control/🟦️.ts";

/** 🟡 Registers the rejects stale baseline source tests of the transaction plan journal v2 aggregate suite. */
export function registerRejectsStaleBaselineSourceTests(): void {
  test.concurrent("rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation", () => {
    const baseline = referenceFixture("stale-baseline"), source = referenceFixture("stale-source"), counterfeit = referenceFixture("counterfeit-segment");
    try {
      const baselinePlan = plan(baseline), baselineTransaction = snapshot(transactionRoot(baseline)), baselineWorkspace = snapshot(baseline.workspace);
      expect(() => applyTaxonomyPlan(baselinePlan, { repoRoot: baseline.repoRoot, ticketDir: baseline.ticketDir, expectedBaselineCommit: "0".repeat(40) })).toThrow(/expectedBaselineCommit/u);
      expect(snapshot(transactionRoot(baseline))).toEqual(baselineTransaction);
      expect(snapshot(baseline.workspace)).toEqual(baselineWorkspace);
      const sourcePlan = plan(source);
      writeFileSync(join(source.workspace, "unrelated.txt"), "drift\n");
      const sourceTransaction = snapshot(transactionRoot(source)), sourceWorkspace = snapshot(source.workspace);
      expect(() => applyTaxonomyPlan(sourcePlan, { repoRoot: source.repoRoot, ticketDir: source.ticketDir, expectedBaselineCommit: source.baselineCommit })).toThrow(/source-tree digest/u);
      expect(snapshot(transactionRoot(source))).toEqual(sourceTransaction);
      expect(snapshot(source.workspace)).toEqual(sourceWorkspace);
      const counterfeitPlan = plan(counterfeit), counterfeitPath = join(counterfeit.workspace, "🧾️taxonomy-transaction", "plain.txt");
      mkdirSync(dirname(counterfeitPath), { recursive: true });
      writeFileSync(counterfeitPath, "counterfeit\n");
      const counterfeitTransaction = snapshot(transactionRoot(counterfeit)), counterfeitWorkspace = snapshot(counterfeit.workspace);
      expect(() => applyTaxonomyPlan(counterfeitPlan, { repoRoot: counterfeit.repoRoot, ticketDir: counterfeit.ticketDir, expectedBaselineCommit: counterfeit.baselineCommit })).toThrow();
      expect(snapshot(transactionRoot(counterfeit))).toEqual(counterfeitTransaction);
      expect(snapshot(counterfeit.workspace)).toEqual(counterfeitWorkspace);
      const inventory = inventoryTaxonomy(counterfeit.options);
      expect(inventory.entries.some((entry) => entry.sourcePath.endsWith("🧾️taxonomy-transaction/plain.txt"))).toBe(true);
    } finally { retainFixture(baseline.root); retainFixture(source.root); retainFixture(counterfeit.root); }
  }, 15_000);
  test.concurrent("rejects stale resume preimage and malformed resume evidence byte-for-byte", async () => {
    const stale = await activeReferenceFixture("stale-resume"), forged = await activeReferenceFixture("forged-resume"), missing = referenceFixture("missing-resume");
    try {
      const stalePlan = plan(stale);
      writeFileSync(join(stale.workspace, "🧪️subject", "./🟦️.ts"), "export const value = 2;\n");
      const staleTransaction = snapshot(transactionRoot(stale)), staleWorkspace = snapshot(stale.workspace);
      expect(() => applyTaxonomyPlan(stalePlan, { repoRoot: stale.repoRoot, ticketDir: stale.ticketDir, expectedBaselineCommit: stale.baselineCommit, resumeJournal: attemptJournal(stale, stalePlan) })).toThrow(/resume-state-drift/u);
      expect(snapshot(transactionRoot(stale))).toEqual(staleTransaction);
      expect(snapshot(stale.workspace)).toEqual(staleWorkspace);
      const forgedPlan = plan(forged);
      writeFileSync(attemptJournal(forged, forgedPlan), "{}\n");
      const forgedTransaction = snapshot(transactionRoot(forged)), forgedWorkspace = snapshot(forged.workspace);
      expect(() => applyTaxonomyPlan(forgedPlan, { repoRoot: forged.repoRoot, ticketDir: forged.ticketDir, expectedBaselineCommit: forged.baselineCommit, resumeJournal: attemptJournal(forged, forgedPlan) })).toThrow();
      expect(snapshot(transactionRoot(forged))).toEqual(forgedTransaction);
      expect(snapshot(forged.workspace)).toEqual(forgedWorkspace);
      const missingPlan = plan(missing), missingTransaction = snapshot(transactionRoot(missing)), missingWorkspace = snapshot(missing.workspace);
      expect(() => applyTaxonomyPlan(missingPlan, { repoRoot: missing.repoRoot, ticketDir: missing.ticketDir, expectedBaselineCommit: missing.baselineCommit, resumeJournal: attemptJournal(missing, missingPlan) })).toThrow(/exact existing canonical attempt/u);
      expect(snapshot(transactionRoot(missing))).toEqual(missingTransaction);
      expect(snapshot(missing.workspace)).toEqual(missingWorkspace);
    } finally { retainFixture(stale.root); retainFixture(forged.root); retainFixture(missing.root); }
  }, 15_000);
  test.concurrent("rejects stale generator and embedded-reference authority before mutation", async () => {
    const generator = generatorFixture("stale-generator"), embedded = embeddedFixture("stale-reference");
    try {
      const generatorPlan = plan(generator);
      writeFileSync(join(generator.workspace, "🧪️generator", "./🟦️.ts"), "export const input = false;\n");
      const generatorTransaction = snapshot(transactionRoot(generator)), generatorWorkspace = snapshot(generator.workspace);
      expect(() => applyTaxonomyPlan(generatorPlan, { repoRoot: generator.repoRoot, ticketDir: generator.ticketDir, expectedBaselineCommit: generator.baselineCommit })).toThrow(/Regeneration input preimage changed/u);
      expect(snapshot(transactionRoot(generator))).toEqual(generatorTransaction);
      expect(snapshot(generator.workspace)).toEqual(generatorWorkspace);
      const embeddedPlan = plan(embedded);
      await killedAt(embedded, embeddedPlan, "transaction-wal-prepared");
      const reference = join(embedded.workspace, "incoming.json");
      writeFileSync(reference, `${JSON.stringify({ path: embeddedPlan.embeddedTicketRoots[0].sourceMetadataRoot })}\n`);
      const embeddedTransaction = snapshot(transactionRoot(embedded)), embeddedWorkspace = snapshot(embedded.workspace);
      expect(() => applyTaxonomyPlan(embeddedPlan, { repoRoot: embedded.repoRoot, ticketDir: embedded.ticketDir, expectedBaselineCommit: embedded.baselineCommit, resumeJournal: attemptJournal(embedded, embeddedPlan) })).toThrow(/embedded incoming references/u);
      expect(snapshot(transactionRoot(embedded))).toEqual(embeddedTransaction);
      expect(snapshot(embedded.workspace)).toEqual(embeddedWorkspace);
    } finally { retainFixture(generator.root); retainFixture(embedded.root); }
  }, 15_000);
  test.concurrent("rejects ordinal collisions and malformed attempt siblings without mutation", async () => {
    const canonical = await activeReferenceFixture("canonical-collision"), duplicate = await activeReferenceFixture("duplicate-collision"), future = await activeReferenceFixture("future-collision"), malformed = await activeReferenceFixture("malformed-sibling");
    try {
      const canonicalPlan = plan(canonical), canonicalAttempts = dirname(dirname(attemptJournal(canonical, canonicalPlan)));
      mkdirSync(join(canonicalAttempts, "🚧️prepare-000001-999999-00000000-0000-4000-8000-000000000001"));
      const canonicalTransaction = snapshot(transactionRoot(canonical)), canonicalWorkspace = snapshot(canonical.workspace);
      expect(() => applyTaxonomyPlan(canonicalPlan, { repoRoot: canonical.repoRoot, ticketDir: canonical.ticketDir, expectedBaselineCommit: canonical.baselineCommit, resumeJournal: attemptJournal(canonical, canonicalPlan) })).toThrow(/collides with canonical ordinal/u);
      expect(snapshot(transactionRoot(canonical))).toEqual(canonicalTransaction);
      expect(snapshot(canonical.workspace)).toEqual(canonicalWorkspace);
      const duplicatePlan = plan(duplicate), duplicateAttempts = dirname(dirname(attemptJournal(duplicate, duplicatePlan)));
      mkdirSync(join(duplicateAttempts, "🚧️prepare-000002-999998-00000000-0000-4000-8000-000000000002"));
      mkdirSync(join(duplicateAttempts, "🚧️prepare-000002-999999-00000000-0000-4000-8000-000000000003"));
      const duplicateTransaction = snapshot(transactionRoot(duplicate)), duplicateWorkspace = snapshot(duplicate.workspace);
      expect(() => applyTaxonomyPlan(duplicatePlan, { repoRoot: duplicate.repoRoot, ticketDir: duplicate.ticketDir, expectedBaselineCommit: duplicate.baselineCommit, resumeJournal: attemptJournal(duplicate, duplicatePlan) })).toThrow(/duplicate ordinal/u);
      expect(snapshot(transactionRoot(duplicate))).toEqual(duplicateTransaction);
      expect(snapshot(duplicate.workspace)).toEqual(duplicateWorkspace);
      const futurePlan = plan(future), futureAttempts = dirname(dirname(attemptJournal(future, futurePlan)));
      mkdirSync(join(futureAttempts, "🚧️prepare-000003-999999-00000000-0000-4000-8000-000000000004"));
      const futureTransaction = snapshot(transactionRoot(future)), futureWorkspace = snapshot(future.workspace);
      expect(() => applyTaxonomyPlan(futurePlan, { repoRoot: future.repoRoot, ticketDir: future.ticketDir, expectedBaselineCommit: future.baselineCommit, resumeJournal: attemptJournal(future, futurePlan) })).toThrow(/not exact next ordinal/u);
      expect(snapshot(transactionRoot(future))).toEqual(futureTransaction);
      expect(snapshot(future.workspace)).toEqual(futureWorkspace);
      const malformedPlan = plan(malformed), malformedAttempts = dirname(dirname(attemptJournal(malformed, malformedPlan)));
      mkdirSync(join(malformedAttempts, "malformed"));
      const malformedTransaction = snapshot(transactionRoot(malformed)), malformedWorkspace = snapshot(malformed.workspace);
      expect(() => applyTaxonomyPlan(malformedPlan, { repoRoot: malformed.repoRoot, ticketDir: malformed.ticketDir, expectedBaselineCommit: malformed.baselineCommit, resumeJournal: attemptJournal(malformed, malformedPlan) })).toThrow(/Unexpected transaction attempt entry/u);
      expect(snapshot(transactionRoot(malformed))).toEqual(malformedTransaction);
      expect(snapshot(malformed.workspace)).toEqual(malformedWorkspace);
    } finally { retainFixture(canonical.root); retainFixture(duplicate.root); retainFixture(future.root); retainFixture(malformed.root); }
  }, 15_000);
  test.concurrent("rejects forged backup and restore preparations without mutation", async () => {
    const backup = referenceFixture("forged-backup"), restore = referenceFixture("forged-restore");
    try {
      const backupPlan = plan(backup);
      const backupKill = await killedAt(backup, backupPlan, "transaction-backup-retained"), backupLeaf = Object.keys(backupKill.transaction).find((path) => path.endsWith(".backup"))!;
      writeFileSync(join(transactionRoot(backup), backupLeaf), "forged\n");
      const backupTransaction = snapshot(transactionRoot(backup)), backupWorkspace = snapshot(backup.workspace);
      expect(() => applyTaxonomyPlan(backupPlan, { repoRoot: backup.repoRoot, ticketDir: backup.ticketDir, expectedBaselineCommit: backup.baselineCommit, resumeJournal: attemptJournal(backup, backupPlan) })).toThrow(/backup/u);
      expect(snapshot(transactionRoot(backup))).toEqual(backupTransaction);
      expect(snapshot(backup.workspace)).toEqual(backupWorkspace);
      const restorePlan = plan(restore);
      const restoreKill = await killedAt(restore, restorePlan, "transaction-restore-prepared", "", "after-edits"), restoreLeaf = Object.keys(restoreKill.transaction).find((path) => path.includes("restore-") && path.endsWith(".backup"))!;
      writeFileSync(join(transactionRoot(restore), restoreLeaf), "forged\n");
      const restoreTransaction = snapshot(transactionRoot(restore)), restoreWorkspace = snapshot(restore.workspace);
      expect(() => applyTaxonomyPlan(restorePlan, { repoRoot: restore.repoRoot, ticketDir: restore.ticketDir, expectedBaselineCommit: restore.baselineCommit, resumeJournal: attemptJournal(restore, restorePlan) })).toThrow(/Restore preparation bytes/u);
      expect(snapshot(transactionRoot(restore))).toEqual(restoreTransaction);
      expect(snapshot(restore.workspace)).toEqual(restoreWorkspace);
    } finally { retainFixture(backup.root); retainFixture(restore.root); }
  }, 15_000);
  test.concurrent("rejects unreachable duplicate backup and edit publication tuples exactly", async () => {
    const backup = referenceFixture("duplicate-backup-tuple"), edit = referenceFixture("duplicate-edit-tuple"), exchanged = referenceFixture("writer-edit-tuple");
    try {
      const backupPlan = plan(backup), editPlan = plan(edit), exchangedPlan = plan(exchanged);
      const [backupKilled, editKilled, exchangedKilled] = await Promise.all([
        killedAt(backup, backupPlan, "transaction-backup-inner-exchange"),
        killedAt(edit, editPlan, "transaction-edit-inner-exchange"),
        killedAt(exchanged, exchangedPlan, "transaction-edit-exchange"),
      ]);
      const duplicateNestedLeaf = (row: Fixture, killed: Readonly<{ transaction: Snapshot }>, outerSuffix: ".backup" | ".edit"): void => {
        const writeRoot = Object.keys(killed.transaction).find((path) => killed.transaction[path].startsWith("directory|") && /write-[1-9][0-9]*-[0-9a-f-]+$/u.test(path));
        const outer = Object.keys(killed.transaction).find((path) => path.endsWith(outerSuffix) && !path.startsWith(`${writeRoot}/`));
        if (!writeRoot || !outer) throw new Error(`Missing duplicate tuple authority for ${outerSuffix}`);
        const target = join(transactionRoot(row), writeRoot, outerSuffix === ".backup" ? "🚧️.backup" : "🚧️.edit");
        writeFileSync(target, readFileSync(join(transactionRoot(row), outer)));
        chmodSync(target, Number.parseInt(killed.transaction[outer].split("|")[1], 8));
      };
      duplicateNestedLeaf(backup, backupKilled, ".backup");
      duplicateNestedLeaf(edit, editKilled, ".edit");
      const editPreparation = Object.keys(exchangedKilled.transaction).find((path) => exchangedKilled.transaction[path].startsWith("directory|") && path.includes("edit-"));
      if (!editPreparation) throw new Error("Missing exchanged edit preparation");
      mkdirSync(join(transactionRoot(exchanged), editPreparation, `🚧️write-999999-${crypto.randomUUID()}`));
      for (const [row, value, pattern] of [[backup, backupPlan, /duplicate outer and nested/u], [edit, editPlan, /duplicate outer and nested/u], [exchanged, exchangedPlan, /writer coexists/u]] as const) {
        const beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
        expect(() => applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value) })).toThrow(pattern);
        expect(snapshot(transactionRoot(row))).toEqual(beforeTransaction);
        expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
      }
    } finally { retainFixture(backup.root); retainFixture(edit.root); retainFixture(exchanged.root); }
  }, 15_000);
}
