/** 🛫️ The preflight reference basis and the resume snapshot lock of the taxonomy plan apply. */
import { sha256, lstatOrNull } from "../../📁️input/🟦️.ts";
import { readFileSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { join } from "node:path";
import { type TaxonomyApplyOptions } from "../../📜️plan-contracts/🟦️.ts";
import { absolutePath, checkCancellation } from "../../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../../🧾️plan-parsing/🟦️.ts";
import { JOURNAL_OPERATION_ARRAYS, type MutableJournalRecord, durableRemove, readJournal } from "../../💾️durable-journal/🟦️.ts";
import { TaxonomyStartedRegenerationPartialError, assertJournalBackupAuthority, assertJournalPhaseMembership, assertJournalPlanMembership } from "../../🩹️recovery-authorities/🟦️.ts";
import { generatorNodeRecord, generatorTreeInventory } from "../../🏭️generator-inputs/🟦️.ts";
import { incomingEmbeddedReferences } from "../../➰️symlink-targets/🟦️.ts";
import { assertLeafPreimage, assertTicketImportantExactRemovalAuthority, assertTicketImportantRemovalAuthority, assertWritableAncestors, leafPathPreimage, removalIncomingIgnoredSourceRoots } from "../../🧐️preimage-assertions/🟦️.ts";
import { assertTransactionRepositoryPath, assertTransactionRepositoryWitness, captureTransactionRepositoryAuthority, isTransactionRepositoryAuthorityError } from "../../🏛️repository-authority/🟦️.ts";
import { transactionTreeDigest } from "../../🌳️tree-digests/🟦️.ts";
import { reconcileRollbackTuples } from "../../⏪️rollback-reconciliation/🟦️.ts";
import { type TransactionLeaseRecord, readTransactionLease, transactionLeaseProcessIsAlive } from "../../🔐️transaction-lease/🟦️.ts";
import { assertRecoveryRootNames, reconcileTransactionOwnedTuples, recoverReferenceEditPreparations, recoverRestorePreparations, recoverTransactionBackups } from "../../🛟️recovery/🟦️.ts";
import { reconcileJournalWal, recoverCanonicalJsonCandidates } from "../../🔄️journal-reconciliation/🟦️.ts";
import { actualAffectedPreDigest, validateLeasePreparationEvidence } from "../../🗃️affected-state/🟦️.ts";
import { removalReferenceProjection } from "../../🗑️removal-references/🟦️.ts";
import { capturePreflightReferenceBasis, embeddedTargetPaths, evidenceRemovalIncomingReferences, lexicalEmbeddedIncomingReferences, lexicalTargetIncomingReferences } from "../../🛫️embedded-roots/🟦️.ts";
import { inventoryTaxonomy, inventoryWithoutTransactionEvidence } from "../../🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../🧠️planning/🟦️.ts";
import type { bindTransactionNames, checkPlanAuthority } from "../📜️plan-checks/🟦️.ts";
import type { defineAttemptValidators } from "../🔎️attempt-discovery/🟦️.ts";
import type { defineLeaseAndPreChecks, selectAttempt } from "../🎯️attempt-selection/🟦️.ts";

/** 🧪 Captures the preflight reference basis and verifies the pre-state of every planned operation. */
export function capturePreflight({ options, repositoryAuthority, repoRoot, bootstrap, transactionRootRelative, plan, taxonomy, planAuthority, txn }: { readonly options: TaxonomyApplyOptions } & Pick<ReturnType<typeof bindTransactionNames>, "repositoryAuthority" | "repoRoot" | "bootstrap" | "transactionRootRelative" | "plan" | "taxonomy"> & Pick<ReturnType<typeof checkPlanAuthority>, "planAuthority"> & Pick<ReturnType<typeof selectAttempt>, "txn">) {
  if (!options.resumeJournal) {
    assertTransactionRepositoryWitness(repositoryAuthority, captureTransactionRepositoryAuthority(repoRoot).indexRows);
    for (const access of bootstrap.accesses) assertTransactionRepositoryPath(repositoryAuthority, access.path, access.role, access.label);
    assertTransactionRepositoryPath(repositoryAuthority, transactionRootRelative, "subtree", "Taxonomy transaction executor");
    if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Affected pre-state digest does not match plan expectation");
    for (const path of [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...plan.symlinkTargetEdits.map((entry) => entry.finalPath), ...plan.edits.map((entry) => entry.path), ...plan.regenerations.flatMap((entry) => entry.outputRoots)]) assertWritableAncestors(repoRoot, path);
    const referenceInventory = inventoryTaxonomy({ repoRoot, scope: plan.scope, ticketDir: options.explicitTicketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile });
    const moveSources = new Set(plan.moves.map((move) => move.sourcePath));
    for (const move of plan.moves) {
      const source = absolutePath(repoRoot, move.sourcePath);
      assertLeafPreimage(repoRoot, move.sourcePath, move.sourcePreimage);
      if (lstatOrNull(absolutePath(repoRoot, move.destinationPath)) && !moveSources.has(move.destinationPath)) throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      assertLeafPreimage(repoRoot, relocation.sourcePath, relocation.preimage);
      if (lstatOrNull(absolutePath(repoRoot, relocation.destinationPath))) throw new Error(`Embedded relocation destination is occupied: ${relocation.destinationPath}`);
    }
    for (const removal of plan.evidenceRemovals) assertLeafPreimage(repoRoot, removal.sourcePath, removal.preimage);
    for (const removal of plan.evidenceRemovals) {
      if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members) assertLeafPreimage(repoRoot, member.sourcePath, member.preimage);
      else if (removal.authority.kind === "exact-path-mutation") assertTicketImportantExactRemovalAuthority(repoRoot, removal);
      else assertTicketImportantRemovalAuthority(repoRoot, removal, taxonomy);
    }
    for (const edit of plan.symlinkTargetEdits) {
      const link = absolutePath(repoRoot, edit.sourcePath);
      if (!lstatOrNull(link)?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget) throw new Error(`Symlink target preimage changed: ${edit.sourcePath}`);
      const logical = lstatOrNull(absolutePath(repoRoot, edit.logicalTargetSourcePath));
      if ((edit.logicalTargetPreimage.state === "absent" && logical) || (edit.logicalTargetPreimage.state === "directory" && !logical?.isDirectory()) || ((edit.logicalTargetPreimage.state === "file" || edit.logicalTargetPreimage.state === "symlink") && (!logical || canonicalJson(leafPathPreimage(absolutePath(repoRoot, edit.logicalTargetSourcePath))) !== canonicalJson(edit.logicalTargetPreimage)))) throw new Error(`Logical symlink target preimage changed: ${edit.logicalTargetSourcePath}`);
    }
    for (const root of plan.embeddedTicketRoots) {
      if (canonicalJson(transactionTreeDigest(repositoryAuthority, root.sourceMetadataRoot, root.sourceMetadataRoot, [])) !== canonicalJson(root.sourceTreeDigest)) throw new Error(`Embedded root tree preimage changed: ${root.sourceMetadataRoot}`);
      const children = [
        ...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
        ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
      ].sort(generatorPathCompare);
      if (children.some((path) => !path.startsWith(`${root.sourceTicketRoot}/`) || !path.startsWith(`${root.sourceMetadataRoot}/`)) || canonicalJson(transactionTreeDigest(repositoryAuthority, root.sourceMetadataRoot, root.sourceMetadataRoot, children)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Embedded root residual authority changed: ${root.sourceMetadataRoot}`);
      const incoming = incomingEmbeddedReferences(referenceInventory, root.sourceMetadataRoot).filter((row) => {
        const source = row.split("\u0000")[1];
        if (!planAuthority || source !== planAuthority.path) return true;
        const stat = lstatOrNull(absolutePath(repoRoot, source));
        return !stat?.isFile() || stat.isSymbolicLink() || !readFileSync(absolutePath(repoRoot, source)).equals(planAuthority.bytes);
      });
      if (sha256(`sha256-taxonomy-reference-set-v1\u0000${canonicalJson(incoming)}`) !== root.incomingReferenceDigest || incoming.length > 0) throw new Error(`Embedded root incoming reference set changed: ${root.sourceMetadataRoot}`);
      txn.preflightReferenceBasis ??= capturePreflightReferenceBasis(repoRoot, taxonomy, options.explicitTicketDir, transactionRootRelative, plan, options.cancelFile, options.progress);
      const lexicalIncoming = lexicalTargetIncomingReferences(repoRoot, embeddedTargetPaths(plan, root), [root.sourceMetadataRoot], taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress, undefined, txn.preflightReferenceBasis);
      if (lexicalIncoming.length > 0) throw new Error(`Embedded root structured incoming reference set changed: ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      txn.preflightReferenceBasis ??= capturePreflightReferenceBasis(repoRoot, taxonomy, options.explicitTicketDir, transactionRootRelative, plan, options.cancelFile, options.progress);
      const project = removal.authority.kind === "nested-cargo-generated-source" || removal.authority.kind === "exact-owner-generated-source" ? removalReferenceProjection(repoRoot, plan) : undefined;
      const incoming = lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), removalIncomingIgnoredSourceRoots(removal), taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress, project, txn.preflightReferenceBasis);
      if (incoming.length > 0) throw new Error(`Evidence-removal structured incoming reference set changed: ${removal.sourcePath}`);
    }
    for (const regeneration of plan.regenerations) {
      const actualInputs = regeneration.inputs.map((input) => generatorNodeRecord(repoRoot, input.path, taxonomy));
      if (canonicalJson(actualInputs) !== canonicalJson(regeneration.inputs)) throw new Error(`Regeneration input preimage changed: ${regeneration.id}`);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Regeneration output preimage changed: ${regeneration.id}`);
    }
    checkCancellation(repoRoot, options.cancelFile);
    {
      const authorityInventory = inventoryWithoutTransactionEvidence(referenceInventory, transactionRootRelative, planAuthority?.path);
      const authorityPlan = planTaxonomy(authorityInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile, progress: options.progress });
      const operationSets = [
        ["source-tree digest", plan.sourceTreeDigest, authorityPlan.sourceTreeDigest],
        ["affected pre-state digest", plan.expectedAffectedPreStateDigest, authorityPlan.expectedAffectedPreStateDigest],
        ["affected post-state digest", plan.expectedPostStateDigest, authorityPlan.expectedPostStateDigest],
        ["moves", plan.moves, authorityPlan.moves],
        ["embedded roots", plan.embeddedTicketRoots, authorityPlan.embeddedTicketRoots],
        ["embedded relocations", plan.embeddedTicketRootRelocations, authorityPlan.embeddedTicketRootRelocations],
        ["symlink target edits", plan.symlinkTargetEdits, authorityPlan.symlinkTargetEdits],
        ["evidence removals", plan.evidenceRemovals, authorityPlan.evidenceRemovals],
        ["destination ancestor preimages", plan.destinationAncestorPreimages, authorityPlan.destinationAncestorPreimages],
        ["reference edits", plan.edits, authorityPlan.edits],
        ["regenerations", plan.regenerations, authorityPlan.regenerations],
        ["unresolved findings", plan.unresolved, authorityPlan.unresolved],
      ] as const;
      const mismatch = operationSets.find(([, submitted, derived]) => canonicalJson(submitted) !== canonicalJson(derived));
      if (mismatch) throw new Error(`Plan ${mismatch[0]} cannot be rederived exactly from current schema-owned authority`);
    }
  }
}

/** 🔒 Validates the selected resume snapshot and locks the owned lease for it. */
export function lockResumeSnapshot({ txn, repoRoot, leaseDirectory, journalFilename, digest, journalWriteDirectory, jsonPreviousName, journalJsonWritePreparationName, plan, taxonomy, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName, leaseJsonWritePreparationName, backupWritePreparationName, backupWriteCandidateName, editWritePreparationName, editWriteCandidateName, options, planAuthority, transactionRootRelative, acquireLease, unpublishedAttempts, existingAttempts, closeTerminalAttempt, releaseLease }: Pick<ReturnType<typeof selectAttempt>, "txn"> & Pick<ReturnType<typeof bindTransactionNames>, "repoRoot" | "leaseDirectory" | "journalFilename" | "digest" | "journalWriteDirectory" | "jsonPreviousName" | "journalJsonWritePreparationName" | "plan" | "taxonomy" | "backupPreparationName" | "restorePreparationName" | "editPreparationName" | "leasePreparationName" | "leaseJsonWritePreparationName" | "backupWritePreparationName" | "backupWriteCandidateName" | "editWritePreparationName" | "editWriteCandidateName" | "transactionRootRelative"> & { readonly options: TaxonomyApplyOptions } & Pick<ReturnType<typeof checkPlanAuthority>, "planAuthority" | "unpublishedAttempts" | "existingAttempts"> & Pick<ReturnType<typeof defineLeaseAndPreChecks>, "acquireLease" | "releaseLease"> & Pick<ReturnType<typeof defineAttemptValidators>, "closeTerminalAttempt">) {
  const validateSelectedResumeSnapshot = (ownedLease?: TransactionLeaseRecord): MutableJournalRecord | undefined => {
    if (!txn.selectedAttempt || txn.selectedAttempt.journal.state === "committed" || txn.selectedAttempt.journal.state === "rolled-back") return txn.selectedAttempt?.journal;
    const selectedLeaseRoot = join(absolutePath(repoRoot, txn.selectedAttempt.attemptRelative), leaseDirectory);
    if (lstatOrNull(selectedLeaseRoot)) {
      const selectedLease = readTransactionLease(selectedLeaseRoot, journalFilename, digest, txn.selectedAttempt.ordinal);
      if (transactionLeaseProcessIsAlive(selectedLease.pid) && (!ownedLease || canonicalJson(selectedLease) !== canonicalJson(ownedLease))) throw new Error(`Transaction attempt is leased by active pid ${selectedLease.pid}`);
    }
    const canonicalJournal = absolutePath(repoRoot, txn.selectedAttempt.journalRelative);
    const selectedWal = join(absolutePath(repoRoot, txn.selectedAttempt.journal.stagingRoot), journalWriteDirectory);
    const selectedProspective = lstatOrNull(selectedWal) ? recoverCanonicalJsonCandidates(selectedWal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidatePath) => {
      const candidate = readJournal(candidatePath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (candidate.planDigest !== digest || candidate.attemptOrdinal !== txn.selectedAttempt!.ordinal || candidate.stagingRoot !== txn.selectedAttempt!.journal.stagingRoot || candidate.backupRoot !== txn.selectedAttempt!.journal.backupRoot) throw new Error("Resume WAL identity differs from the selected canonical attempt");
      assertJournalPlanMembership(plan, candidate);
      assertJournalPhaseMembership(plan, candidate);
      assertJournalBackupAuthority(plan, candidate);
    }, true, true, canonicalJournal) : lstatOrNull(canonicalJournal) ? canonicalJournal : undefined;
    if (!selectedProspective) throw new Error("Selected resume attempt has no recoverable durable journal");
    const current = readJournal(lstatOrNull(canonicalJournal) ? canonicalJournal : selectedProspective, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
    const durable = reconcileJournalWal(repoRoot, canonicalJournal, current, plan, taxonomy, true);
    assertRecoveryRootNames(repoRoot, plan, durable, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
    validateLeasePreparationEvidence(absolutePath(repoRoot, durable.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, txn.selectedAttempt.ordinal);
    recoverTransactionBackups(repoRoot, plan, durable, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
    recoverRestorePreparations(repoRoot, plan, durable, restorePreparationName, true);
    recoverReferenceEditPreparations(repoRoot, plan, durable, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
    const tupleProbe: MutableJournalRecord = { ...durable, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...durable[key]]])), backups: { ...durable.backups } } as MutableJournalRecord;
    try {
      if (durable.state === "rolling-back") reconcileRollbackTuples(repoRoot, plan, tupleProbe, taxonomy);
      else reconcileTransactionOwnedTuples(repoRoot, plan, tupleProbe, taxonomy);
    } catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError)) throw error;
    }
    for (const root of plan.embeddedTicketRoots) {
      const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = evidenceRemovalIncomingReferences(repoRoot, removal, plan, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, tupleProbe, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
    }
    return durable;
  };
  validateSelectedResumeSnapshot();
  try {
    if (txn.selectedAttempt && txn.selectedAttempt.journal.state !== "committed") {
      acquireLease(txn.selectedAttempt.journal.backupRoot, validateSelectedResumeSnapshot);
      txn.lockedResumeJournal = validateSelectedResumeSnapshot(txn.leaseHandle?.record);
    }
    for (const preparation of unpublishedAttempts) durableRemove(preparation.path, true);
    for (const attempt of existingAttempts) closeTerminalAttempt(attempt);
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    releaseLease();
    throw error;
  }
}
