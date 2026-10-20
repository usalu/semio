/** 🏁️ The transaction run of the plan apply: stages, installs, edits, regenerates, verifies and commits one attempt and rolls it back on failure. */
import { report } from "../../🏃️operation/🟦️.ts";

import { lstatOrNull } from "../../📁️input/🟦️.ts";

import { readdirSync } from "node:fs";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { join } from "node:path";
import { type TaxonomyApplyOptions, type TaxonomyApplyResult, type TaxonomyPlan, requireTaxonomyValue } from "../../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../../🏷️file-kind-matching/🟦️.ts";

import { JOURNAL_OPERATION_ARRAYS, type MutableJournalRecord, journalSnapshot, persistJournal, readJournal } from "../../💾️durable-journal/🟦️.ts";
import { TaxonomyStartedRegenerationPartialError, assertJournalBackupAuthority, assertJournalPhaseMembership, assertJournalPlanMembership } from "../../🩹️recovery-authorities/🟦️.ts";


import { isTransactionRepositoryAuthorityError } from "../../🏛️repository-authority/🟦️.ts";


import { type TransactionLeaseHandle } from "../../🔐️transaction-lease/🟦️.ts";
import { assertRecoveryRootNames, reconcileTransactionOwnedTuples, recoverReferenceEditPreparations, recoverRestorePreparations, recoverTransactionBackups } from "../../🛟️recovery/🟦️.ts";

import { reconcileJournalWal } from "../../🔄️journal-reconciliation/🟦️.ts";
import { validateLeasePreparationEvidence } from "../../🗃️affected-state/🟦️.ts";

import { rollbackTransaction } from "../../↩️rollback-transaction/🟦️.ts";
import { type PreflightReferenceBasis } from "../../🛫️embedded-roots/🟦️.ts";


import type { LoadedTaxonomy } from "../../🔣️taxonomy/📚️loaded/🟦️.ts";
import { applyReferenceEditGroups, installRelocationsAndRetargetSymlinks, regenerateGeneratorOutputs, stageEmbeddedRootsAndInstallMoves, stageTransactionOperations, verifyAndCommitTransaction } from "./🔧️transaction-steps/🟦️.ts";

/** 🏆️ Runs the prepared attempt through staging, installing, retargeting, editing, regenerating, verification and commit, and rolls it back when a phase fails. */
export function runTransactionAttempt({ options, transactionProbe, attemptRelative, repoRoot, plan, journal, sourceSet, txn, journalPath, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, editPreparationName, editWritePreparationName, editWriteCandidateName, taxonomy, ticketRoot, planAuthority, transactionRootRelative, releaseLease, digest, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName, journalFilename, restorePreparationName, leasePreparationName, leaseJsonWritePreparationName }: {
  readonly options: TaxonomyApplyOptions;
  readonly transactionProbe: (phase: string, path?: string) => void;
  readonly attemptRelative: string;
  readonly repoRoot: string;
  readonly plan: TaxonomyPlan;
  readonly journal: MutableJournalRecord;
  readonly sourceSet: Set<string>;
  readonly txn: { attemptOrdinal: string; journalRelative: string; selectedAttempt: { readonly ordinal: string; readonly attemptRelative: string; readonly journal: MutableJournalRecord; readonly journalRelative: string; } | undefined; leaseHandle: TransactionLeaseHandle | undefined; preflightReferenceBasis: PreflightReferenceBasis | undefined; lockedResumeJournal: MutableJournalRecord | undefined; };
  readonly journalPath: string;
  readonly backupPreparationName: (identity: string, pid: number, token: string) => string;
  readonly backupWritePreparationName: (pid: number, token: string) => string;
  readonly backupWriteCandidateName: string;
  readonly editPreparationName: (identity: string, pid: number, token: string) => string;
  readonly editWritePreparationName: (pid: number, token: string) => string;
  readonly editWriteCandidateName: string;
  readonly taxonomy: LoadedTaxonomy;
  readonly ticketRoot: string;
  readonly planAuthority: { path: string; bytes: Buffer<ArrayBuffer>; } | undefined;
  readonly transactionRootRelative: string;
  readonly releaseLease: () => void;
  readonly digest: string;
  readonly journalWriteDirectory: string;
  readonly journalJsonWritePreparationName: (pid: number, token: string) => string;
  readonly jsonPreviousName: string;
  readonly journalFilename: string;
  readonly restorePreparationName: (identity: string, pid: number, token: string) => string;
  readonly leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string;
  readonly leaseJsonWritePreparationName: (pid: number, token: string) => string;
}): TaxonomyApplyResult {
  try {
    stageTransactionOperations({ options, transactionProbe, attemptRelative, repoRoot, plan, journal, sourceSet, txn, journalPath });
    stageEmbeddedRootsAndInstallMoves({ options, journal, repoRoot, journalPath, plan });
    installRelocationsAndRetargetSymlinks({ options, plan, repoRoot, journal, journalPath });
    applyReferenceEditGroups({ plan, repoRoot, options, journal, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, journalPath, editPreparationName, editWritePreparationName, editWriteCandidateName });
    regenerateGeneratorOutputs({ repoRoot, options, journal, journalPath, plan, taxonomy, backupPreparationName, backupWritePreparationName, backupWriteCandidateName });
    verifyAndCommitTransaction({ journal, repoRoot, journalPath, options, plan, ticketRoot, taxonomy, planAuthority, transactionRootRelative, transactionProbe, releaseLease });
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    const failureMessage = error instanceof Error ? error.message : String(error);
    const committedFailure = {};
    try {
      let durable = (() => {
        if (lstatOrNull(journalPath)) return readJournal(journalPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        const walRoot = join(absolutePath(repoRoot, journal.stagingRoot), journalWriteDirectory);
        const walStat = lstatOrNull(walRoot);
        if (!walStat?.isDirectory() || walStat.isSymbolicLink()) throw new Error("Taxonomy journal recovery has no canonical journal or direct WAL authority");
        const preparations = readdirSync(walRoot).filter((name) => name !== journalFilename);
        if (preparations.length !== 1) throw new Error("Taxonomy journal recovery has ambiguous previous-image authority");
        const previous = join(walRoot, requireTaxonomyValue(preparations[0],"preparations[0]"), jsonPreviousName);
        const previousStat = lstatOrNull(previous);
        if (!previousStat?.isFile() || previousStat.isSymbolicLink()) throw new Error("Taxonomy journal recovery previous-image authority is unavailable");
        const recovered = readJournal(previous, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        if (recovered.planDigest !== digest || recovered.attemptOrdinal !== txn.attemptOrdinal || recovered.stagingRoot !== journal.stagingRoot || recovered.backupRoot !== journal.backupRoot) throw new Error("Taxonomy journal recovery previous-image identity differs from the active attempt");
        assertJournalPlanMembership(plan, recovered);
        assertJournalPhaseMembership(plan, recovered);
        assertJournalBackupAuthority(plan, recovered);
        return recovered;
      })();
      durable.probe = transactionProbe;
      if (durable.state === "committed") throw committedFailure;
      assertRecoveryRootNames(repoRoot, plan, durable, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
      validateLeasePreparationEvidence(absolutePath(repoRoot, durable.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, txn.attemptOrdinal);
      const prospective = reconcileJournalWal(repoRoot, journalPath, durable, plan, taxonomy, true);
      recoverTransactionBackups(repoRoot, plan, prospective, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
      recoverRestorePreparations(repoRoot, plan, prospective, restorePreparationName, true);
      recoverReferenceEditPreparations(repoRoot, plan, prospective, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
      durable = reconcileJournalWal(repoRoot, journalPath, durable, plan, taxonomy);
      if (durable.state === "committed") throw committedFailure;
      const recovered: MutableJournalRecord = { ...durable, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...durable[key]]])), backups: { ...durable.backups } } as MutableJournalRecord;
      const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
      recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
      let recoveredTuples = false;
      try { recoveredTuples = reconcileTransactionOwnedTuples(repoRoot, plan, recovered, taxonomy); }
      catch (resumeError) { if (!(resumeError instanceof TaxonomyStartedRegenerationPartialError)) throw resumeError; }
      if (recoveredBackups || recoveredTuples || canonicalJson(journalSnapshot(durable)) !== canonicalJson(journalSnapshot(recovered))) {
        durable = recovered;
        persistJournal(repoRoot, journalPath, durable);
      }
      journal = durable;
      journal.error = failureMessage;
      rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
    } catch (rollbackError) {
      if (isTransactionRepositoryAuthorityError(rollbackError)) throw rollbackError;
      if (rollbackError === committedFailure) {
        releaseLease();
        throw error;
      }
      journal.error = `${failureMessage}; rollback failed: ${rollbackError instanceof Error ? rollbackError.message : String(rollbackError)}`;
      try { persistJournal(repoRoot, journalPath, journal); } catch (persistError) { if (isTransactionRepositoryAuthorityError(persistError)) throw persistError; }
      releaseLease();
      throw new Error(journal.error);
    }
    report(options.progress, "apply", "rolled-back", 0, plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length);
    releaseLease();
    return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
  }
}
