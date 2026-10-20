/** 🪪️ The journal preparation of the taxonomy plan apply. */
import { normalizeRelative } from "../../🛣️path/🟦️.ts";
import { lstatOrNull, assertNoFollowAncestors } from "../../📁️input/🟦️.ts";
import { randomUUID } from "node:crypto";
import { mkdirSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { dirname, join } from "node:path";
import { type TaxonomyApplyOptions, type TaxonomyApplyResult } from "../../📜️plan-contracts/🟦️.ts";
import { absolutePath, cancellationRequested, checkCancellation } from "../../🏷️file-kind-matching/🟦️.ts";
import { type MutableJournalRecord, durableRemove, durableRename, fsyncDirectory, fsyncFile, journalSnapshot, persistJournal, publishCanonicalJsonCandidate } from "../../💾️durable-journal/🟦️.ts";
import { TaxonomyGeneratorInputDriftError, TaxonomyMoveSourceInputDriftError, TaxonomyStartedRegenerationPartialError, assertJournalBackupAuthority, assertJournalPhaseMembership, assertJournalPlanMembership } from "../../🩹️recovery-authorities/🟦️.ts";
import { isTransactionRepositoryAuthorityError, transactionRepositoryFinally } from "../../🏛️repository-authority/🟦️.ts";
import { type TransactionLeaseRecord } from "../../🔐️transaction-lease/🟦️.ts";
import { reconcileTransactionOwnedTuples, recoverReferenceEditPreparations, recoverRestorePreparations, recoverTransactionBackups, validateResumeTuples } from "../../🛟️recovery/🟦️.ts";
import { assertActiveTransactionEvidence, cleanupCommittedTransaction } from "../../🚿️transaction-cleanup/🟦️.ts";
import { reconcileJournalWal } from "../../🔄️journal-reconciliation/🟦️.ts";
import { actualAffectedDigest } from "../../🗃️affected-state/🟦️.ts";
import { rollbackTransaction } from "../../↩️rollback-transaction/🟦️.ts";
import { evidenceRemovalIncomingReferences, lexicalEmbeddedIncomingReferences } from "../../🛫️embedded-roots/🟦️.ts";
import type { bindTransactionNames, checkPlanAuthority } from "../📜️plan-checks/🟦️.ts";
import type { defineLeaseAndPreChecks, selectAttempt } from "../🎯️attempt-selection/🟦️.ts";

/** 📒 Loads the resumed journal or publishes the initial journal and lease of a new attempt. */
export function prepareAttemptJournal({ options, txn, repoRoot, journalPath, plan, taxonomy, transactionProbe, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, restorePreparationName, editPreparationName, editWritePreparationName, editWriteCandidateName, attemptRelative, stageDirectory, backupDirectory, digest, ticketRoot, releaseLease, planAuthority, transactionRootRelative, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName, attemptsRelative, attemptsAbsolute, transactionRelative, transactionRootAbsolute, attemptPreparationName, leaseDirectory, journalFilename, leaseJsonWritePreparationName }: { readonly options: TaxonomyApplyOptions } & Pick<ReturnType<typeof selectAttempt>, "txn" | "journalPath" | "attemptRelative"> & Pick<ReturnType<typeof bindTransactionNames>, "repoRoot" | "plan" | "taxonomy" | "transactionProbe" | "backupPreparationName" | "backupWritePreparationName" | "backupWriteCandidateName" | "restorePreparationName" | "editPreparationName" | "editWritePreparationName" | "editWriteCandidateName" | "stageDirectory" | "backupDirectory" | "digest" | "ticketRoot" | "transactionRootRelative" | "journalWriteDirectory" | "journalJsonWritePreparationName" | "jsonPreviousName" | "attemptsRelative" | "transactionRelative" | "attemptPreparationName" | "leaseDirectory" | "journalFilename" | "leaseJsonWritePreparationName"> & Pick<ReturnType<typeof defineLeaseAndPreChecks>, "releaseLease"> & Pick<ReturnType<typeof checkPlanAuthority>, "planAuthority" | "attemptsAbsolute" | "transactionRootAbsolute">): TaxonomyApplyResult | { readonly journal: MutableJournalRecord; readonly sourceSet: Set<string> } {
  let journal: MutableJournalRecord;
  if (options.resumeJournal) {
    try {
    if (!txn.selectedAttempt || !txn.lockedResumeJournal) throw new Error("Resume journal has no selected owned-lease snapshot");
    journal = txn.lockedResumeJournal;
    journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
    journal.probe = transactionProbe;
    if (txn.lockedResumeJournal && canonicalJson(journalSnapshot(journal)) !== canonicalJson(journalSnapshot(txn.lockedResumeJournal))) throw new Error("Resume journal changed after its owned-lease snapshot");
    if (journal.state !== "committed" && journal.state !== "rolled-back") {
      const recovered: MutableJournalRecord = { ...journal, startedRegenerationIds: [...journal.startedRegenerationIds], backups: { ...journal.backups } };
      const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
      recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
      journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
      if (recoveredBackups) {
        let changed = false;
        for (const [path, backup] of Object.entries(recovered.backups)) {
          if (journal.backups[path]) {
            if (canonicalJson(journal.backups[path]) !== canonicalJson(backup)) throw new Error(`Recovered backup differs from promoted journal authority: ${path}`);
          } else { journal.backups[path] = backup; changed = true; }
        }
        for (const id of recovered.startedRegenerationIds) if (!journal.startedRegenerationIds.includes(id)) { journal.startedRegenerationIds.push(id); changed = true; }
        if (changed) persistJournal(repoRoot, journalPath, journal);
      }
    }
    assertJournalBackupAuthority(plan, journal);
    const expectedStagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const expectedBackupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    if (journal.attemptOrdinal !== txn.attemptOrdinal || journal.stagingRoot !== expectedStagingRoot || journal.backupRoot !== expectedBackupRoot) throw new Error("Resume journal attempt identity and transaction roots do not match the canonical plan attempt");
    assertJournalPlanMembership(plan, journal);
    assertJournalPhaseMembership(plan, journal);
    if (journal.planDigest !== digest) throw new Error("Resume journal belongs to a different plan");
    if (journal.state !== "committed" && journal.state !== "rolled-back") assertActiveTransactionEvidence(repoRoot, plan, journal, false);
    if (journal.state === "committed") {
      if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Committed resume post-state digest changed");
      cleanupCommittedTransaction(repoRoot, journal, plan, ticketRoot);
      return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
    }
    if (journal.state === "rolled-back") throw new Error(`Cannot resume journal in state ${journal.state}`);
    if (journal.state === "rolling-back") {
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    let partialOutput: TaxonomyStartedRegenerationPartialError | undefined;
    try { if (reconcileTransactionOwnedTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal); }
    catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError)) throw error;
      partialOutput = error;
    }
    if (partialOutput || cancellationRequested(repoRoot, options.cancelFile)) {
      if (partialOutput) {
        journal.state = "rolling-back";
        journal.error = partialOutput.message;
        persistJournal(repoRoot, journalPath, journal);
      }
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    for (const root of plan.embeddedTicketRoots) {
      const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = evidenceRemovalIncomingReferences(repoRoot, removal, plan, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, journal, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
    }
    try {
      if (validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    } catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError) && !(error instanceof TaxonomyGeneratorInputDriftError) && !(error instanceof TaxonomyMoveSourceInputDriftError)) throw error;
      journal.state = "rolling-back";
      journal.error = error.message;
      persistJournal(repoRoot, journalPath, journal);
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    assertActiveTransactionEvidence(repoRoot, plan, journal, true);
    } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      releaseLease();
      throw error;
    }
  } else {
    const stagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const backupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    journal = { schemaVersion: 2, revision: 0, planDigest: digest, attemptOrdinal: txn.attemptOrdinal, state: "prepared", stagingRoot, backupRoot, journalWriteDirectory, jsonWritePreparationName: journalJsonWritePreparationName, jsonPreviousName, probe: transactionProbe, preparedMoveIds: [], stagedMoveIds: [], installedMoveIds: [], preparedEmbeddedRelocationIds: [], stagedEmbeddedRelocationIds: [], installedEmbeddedRelocationIds: [], preparedEvidenceRemovalIds: [], stagedEvidenceRemovalIds: [], preparedEmbeddedRootIds: [], stagedEmbeddedRootIds: [], preparedSymlinkTargetEditIds: [], stagedSymlinkTargetEditIds: [], installedSymlinkTargetEditIds: [], appliedEditPaths: [], startedRegenerationIds: [], completedRegenerationIds: [], sourceParentPrunePaths: [], backups: {} };
    checkCancellation(repoRoot, options.cancelFile);
    assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, attemptsRelative), "transaction attempts root", true);
    const allocationAncestors = [attemptsAbsolute, absolutePath(repoRoot, transactionRelative), transactionRootAbsolute].map((path) => ({ path, existed: Boolean(lstatOrNull(path)) }));
    const preparationToken = randomUUID();
    const preparationRelative = normalizeRelative(`${attemptsRelative}/${attemptPreparationName(txn.attemptOrdinal, process.pid, preparationToken)}`);
    const preparationRoot = absolutePath(repoRoot, preparationRelative);
    const preparationStage = join(preparationRoot, stageDirectory);
    const preparationBackup = join(preparationRoot, backupDirectory);
    const preparationLease = join(preparationRoot, leaseDirectory);
    const leaseRecord: TransactionLeaseRecord = { schemaVersion: 1, planDigest: digest, attemptOrdinal: txn.attemptOrdinal, token: randomUUID(), pid: process.pid };
    try {
      mkdirSync(attemptsAbsolute, { recursive: true });
      fsyncDirectory(dirname(attemptsAbsolute));
      try { mkdirSync(preparationRoot); }
      catch (error) {
        if ((error as NodeJS.ErrnoException).code === "EEXIST") throw new Error(`Transaction attempt preparation collision at ${preparationRelative}`);
        throw error;
      }
      fsyncDirectory(attemptsAbsolute);
      transactionProbe("transaction-attempt-preparation-mkdir", preparationRelative);
      mkdirSync(preparationStage);
      mkdirSync(preparationBackup);
      mkdirSync(preparationLease);
      transactionProbe("transaction-attempt-preparation-children", preparationRelative);
      publishCanonicalJsonCandidate(preparationLease, journalFilename, jsonPreviousName, leaseRecord, leaseJsonWritePreparationName, undefined, transactionProbe, "transaction-initial-lease-json");
      transactionProbe("transaction-initial-lease-prepared", preparationRelative);
      const initialWalRoot = join(preparationStage, journalWriteDirectory);
      const initialWal = join(initialWalRoot, journalFilename);
      mkdirSync(initialWalRoot);
      transactionProbe("transaction-initial-wal-mkdir", preparationRelative);
      publishCanonicalJsonCandidate(initialWalRoot, journalFilename, jsonPreviousName, journalSnapshot(journal), journalJsonWritePreparationName, undefined, transactionProbe, "transaction-initial-journal");
      durableRename(initialWal, join(preparationRoot, journalFilename));
      transactionProbe("transaction-initial-journal-canonical", preparationRelative);
      durableRemove(initialWalRoot, true);
      fsyncFile(join(preparationRoot, journalFilename));
      fsyncDirectory(preparationStage);
      fsyncDirectory(preparationBackup);
      fsyncDirectory(preparationRoot);
      durableRename(preparationRoot, absolutePath(repoRoot, attemptRelative));
    } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      if (lstatOrNull(preparationRoot)) durableRemove(preparationRoot, true);
      for (const ancestor of allocationAncestors) if (!ancestor.existed && lstatOrNull(ancestor.path)?.isDirectory() && readdirSync(ancestor.path).length === 0) durableRemove(ancestor.path, true);
      if ((error as NodeJS.ErrnoException).code === "EEXIST" || (error as NodeJS.ErrnoException).code === "ENOTEMPTY") throw new Error(`Transaction attempt allocation race at ${attemptRelative}`);
      throw error;
    }
    txn.leaseHandle = { root: absolutePath(repoRoot, `${attemptRelative}/${leaseDirectory}`), filename: journalFilename, record: leaseRecord };
  }
  const sourceSet = new Set(plan.moves.map((move) => move.sourcePath));
  return { journal, sourceSet };
}
