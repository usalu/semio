/** 🔧️ The transaction steps of the plan apply: staging, installing, retargeting, editing, regenerating and the verified commit of one attempt. */
import { report } from "../../../🏃️operation/🟦️.ts";
import { sourceRelative } from "../../../🛣️path/🟦️.ts";
import { lstatOrNull } from "../../../📁️input/🟦️.ts";
import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../../../🧾️serialization/🔣️json/🟦️.ts";
import { dirname, join, relative } from "node:path";
import { type ReferenceEdit, type TaxonomyApplyOptions, type TaxonomyPlan, requireTaxonomyValue } from "../../../📜️plan-contracts/🟦️.ts";
import { absolutePath, checkCancellation } from "../../../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../../../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, durableRename, durableSymlink, durablySyncGeneratorRecords, persistJournal } from "../../../💾️durable-journal/🟦️.ts";
import { type TaxonomyBackupExpectedPreimage } from "../../../🩹️recovery-authorities/🟦️.ts";
import { generatorTreeInventory } from "../../../🏭️generator-inputs/🟦️.ts";
import { assertDirectoryOnlyTree, leafPreimage, retargetedMovePreimage } from "../../../🧐️preimage-assertions/🟦️.ts";
import { noFollowTreeDigest } from "../../../🌳️tree-digests/🟦️.ts";
import { emptySourceParents, injectFailure, inventoryAfterSourceParentPruning } from "../../../⏪️rollback-reconciliation/🟦️.ts";
import { type TransactionLeaseHandle } from "../../../🔐️transaction-lease/🟦️.ts";
import { applyReferenceEditAtomically, backupPath, validateResumeTuples } from "../../../🛟️recovery/🟦️.ts";
import { cleanupCommittedTransaction } from "../../../🚿️transaction-cleanup/🟦️.ts";
import { actualAffectedDigest } from "../../../🗃️affected-state/🟦️.ts";
import { artifactProjectionPostApplyViolations, projectionStaleViolations } from "../../../♻️projection-staleness/🟦️.ts";
import { type PreflightReferenceBasis, embeddedTargetPaths, lexicalTargetIncomingReferences, validatePreflightReferenceBasis } from "../../../🛫️embedded-roots/🟦️.ts";
import { inventoryTaxonomyWithSourceParentPruning, inventoryWithoutTransactionEvidence } from "../../../🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../../🧠️planning/🟦️.ts";
import type { LoadedTaxonomy } from "../../../🔣️taxonomy/📚️loaded/🟦️.ts";

/** 🪣️ Stages the moves, relocations and evidence removals of the attempt into the staging root. */
export function stageTransactionOperations({ options, transactionProbe, attemptRelative, repoRoot, plan, journal, sourceSet, txn, journalPath }: {
  readonly options: TaxonomyApplyOptions;
  readonly transactionProbe: (phase: string, path?: string) => void;
  readonly attemptRelative: string;
  readonly repoRoot: string;
  readonly plan: TaxonomyPlan;
  readonly journal: MutableJournalRecord;
  readonly sourceSet: Set<string>;
  readonly txn: { attemptOrdinal: string; journalRelative: string; selectedAttempt: { readonly ordinal: string; readonly attemptRelative: string; readonly journal: MutableJournalRecord; readonly journalRelative: string; } | undefined; leaseHandle: TransactionLeaseHandle | undefined; preflightReferenceBasis: PreflightReferenceBasis | undefined; lockedResumeJournal: MutableJournalRecord | undefined; };
  readonly journalPath: string;
}) {
  if (!options.resumeJournal) transactionProbe("transaction-attempt-canonical-published", attemptRelative);
  checkCancellation(repoRoot, options.cancelFile);
  for (const move of plan.moves) {
    if (journal.stagedMoveIds.includes(move.operationId)) {
      const candidates = journal.installedMoveIds.includes(move.operationId)
        ? [absolutePath(repoRoot, move.destinationPath), join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)]
        : [join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)];
      const resumedPath = candidates.find((path) => lstatOrNull(path));
      if (!resumedPath) throw new Error(`Resume move state is invalid: ${move.operationId}`);
      const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
      if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(resumedPath)) !== canonicalJson(retargetedMovePreimage(move, installedLink))) throw new Error(`Resume move preimage changed: ${move.operationId}`);
      continue;
    }
    const source = absolutePath(repoRoot, move.sourcePath);
    const destination = absolutePath(repoRoot, move.destinationPath);
    const sourceStat = lstatOrNull(source);
    if (!sourceStat) throw new Error(`Move source is missing: ${move.sourcePath}`);
    if (canonicalJson(leafPreimage(source)) !== canonicalJson(move.sourcePreimage)) throw new Error(`Move source preimage changed: ${move.sourcePath}`);
    if (lstatOrNull(destination) && !sourceSet.has(move.destinationPath)) throw new Error(`Move destination is occupied: ${move.destinationPath}`);
  }
  if (txn.preflightReferenceBasis) validatePreflightReferenceBasis(txn.preflightReferenceBasis, options.cancelFile, options.progress);
  journal.state = "staging";
  persistJournal(repoRoot, journalPath, journal);
  for (const [index, move] of plan.moves.entries()) {
    checkCancellation(repoRoot, options.cancelFile);

    const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    if (!journal.preparedMoveIds.includes(move.operationId)) {
      journal.preparedMoveIds.push(move.operationId);
      persistJournal(repoRoot, journalPath, journal);
    }
    if (!lstatOrNull(stage) && !journal.installedMoveIds.includes(move.operationId)) {
      mkdirSync(dirname(stage), { recursive: true });
      durableRename(absolutePath(repoRoot, move.sourcePath), stage);
    }
    if (!journal.stagedMoveIds.includes(move.operationId)) {
      journal.stagedMoveIds.push(move.operationId);
      persistJournal(repoRoot, journalPath, journal);
    }
    report(options.progress, "apply", "staging", index + 1, plan.moves.length, move.sourcePath);
  }
  for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
    checkCancellation(repoRoot, options.cancelFile);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    if (!journal.preparedEmbeddedRelocationIds.includes(relocation.operationId)) { journal.preparedEmbeddedRelocationIds.push(relocation.operationId); persistJournal(repoRoot, journalPath, journal); }
    if (!lstatOrNull(stage) && !journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, relocation.sourcePath), stage); }
    if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId)) { journal.stagedEmbeddedRelocationIds.push(relocation.operationId); persistJournal(repoRoot, journalPath, journal); }
    report(options.progress, "apply", "staging-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.sourcePath);
    checkCancellation(repoRoot, options.cancelFile);
  }
  for (const [index, removal] of plan.evidenceRemovals.entries()) {
    checkCancellation(repoRoot, options.cancelFile);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    if (!journal.preparedEvidenceRemovalIds.includes(removal.operationId)) { journal.preparedEvidenceRemovalIds.push(removal.operationId); persistJournal(repoRoot, journalPath, journal); }
    if (!lstatOrNull(stage) && !journal.stagedEvidenceRemovalIds.includes(removal.operationId)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, removal.sourcePath), stage); }
    if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId)) { journal.stagedEvidenceRemovalIds.push(removal.operationId); persistJournal(repoRoot, journalPath, journal); }
    report(options.progress, "apply", "staging-evidence-removals", index + 1, plan.evidenceRemovals.length, removal.sourcePath);
    checkCancellation(repoRoot, options.cancelFile);
  }
}

/** 🚚️ Stages the embedded ticket roots and installs the staged moves. */
export function stageEmbeddedRootsAndInstallMoves({ options, journal, repoRoot, journalPath, plan }: {
  readonly options: TaxonomyApplyOptions;
  readonly journal: MutableJournalRecord;
  readonly repoRoot: string;
  readonly journalPath: string;
  readonly plan: TaxonomyPlan;
}) {
  injectFailure(options, "after-staging");
  journal.state = "disposing";
  persistJournal(repoRoot, journalPath, journal);
  for (const [index, root] of plan.embeddedTicketRoots.entries()) {
    checkCancellation(repoRoot, options.cancelFile);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    if (!journal.preparedEmbeddedRootIds.includes(root.operationId)) { journal.preparedEmbeddedRootIds.push(root.operationId); persistJournal(repoRoot, journalPath, journal); }
    if (!lstatOrNull(stage) && !journal.stagedEmbeddedRootIds.includes(root.operationId)) { assertDirectoryOnlyTree(absolutePath(repoRoot, root.sourceMetadataRoot)); if (canonicalJson(noFollowTreeDigest(repoRoot, root.sourceMetadataRoot)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Embedded root residual tree differs from frozen structure: ${root.sourceMetadataRoot}`); mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, root.sourceMetadataRoot), stage); }
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId)) { journal.stagedEmbeddedRootIds.push(root.operationId); persistJournal(repoRoot, journalPath, journal); }
    report(options.progress, "apply", "disposing-embedded-roots", index + 1, plan.embeddedTicketRoots.length, root.sourceMetadataRoot);
    checkCancellation(repoRoot, options.cancelFile);
  }
  injectFailure(options, "after-embedded-root-staging");
  journal.state = "installing";
  persistJournal(repoRoot, journalPath, journal);
  for (const [index, move] of plan.moves.entries()) {
    checkCancellation(repoRoot, options.cancelFile);

    const destination = absolutePath(repoRoot, move.destinationPath);
    const installed = journal.installedMoveIds.includes(move.operationId);
    if (!lstatOrNull(destination)) {
      mkdirSync(dirname(destination), { recursive: true });
      durableRename(join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination);
    }
    if (!installed) {
      journal.installedMoveIds.push(move.operationId);
      persistJournal(repoRoot, journalPath, journal);
    }
    report(options.progress, "apply", "moves", index + 1, plan.moves.length, move.destinationPath);
  }
}

/** 🔗️ Installs the embedded ticket root relocations and retargets the symlink targets. */
export function installRelocationsAndRetargetSymlinks({ options, plan, repoRoot, journal, journalPath }: {
  readonly options: TaxonomyApplyOptions;
  readonly plan: TaxonomyPlan;
  readonly repoRoot: string;
  readonly journal: MutableJournalRecord;
  readonly journalPath: string;
}) {
  injectFailure(options, "after-moves");
  for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
    checkCancellation(repoRoot, options.cancelFile);
    const destination = absolutePath(repoRoot, relocation.destinationPath);
    if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) {
      if (!lstatOrNull(destination)) { mkdirSync(dirname(destination), { recursive: true }); durableRename(join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`), destination); }
      journal.installedEmbeddedRelocationIds.push(relocation.operationId);
      persistJournal(repoRoot, journalPath, journal);
    }
    report(options.progress, "apply", "installing-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.destinationPath);
  }
  injectFailure(options, "after-relocations");
  journal.state = "retargeting";
  persistJournal(repoRoot, journalPath, journal);
  for (const [index, edit] of plan.symlinkTargetEdits.entries()) {
    checkCancellation(repoRoot, options.cancelFile);
    const link = absolutePath(repoRoot, edit.finalPath);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (!journal.preparedSymlinkTargetEditIds.includes(edit.operationId)) { journal.preparedSymlinkTargetEditIds.push(edit.operationId); persistJournal(repoRoot, journalPath, journal); }
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId)) {
      if (!lstatOrNull(stage)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(link, stage); }
      journal.stagedSymlinkTargetEditIds.push(edit.operationId);
      persistJournal(repoRoot, journalPath, journal);
    }
    if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
      if (!lstatOrNull(link)) durableSymlink(edit.newTarget, link, process.platform === "win32" ? edit.windowsLinkType : undefined);
      if (readlinkSync(link) !== edit.newTarget) throw new Error(`Symlink retarget verification failed: ${edit.finalPath}`);
      journal.installedSymlinkTargetEditIds.push(edit.operationId);
      persistJournal(repoRoot, journalPath, journal);
    }
    report(options.progress, "apply", "retargeting-symlinks", index + 1, plan.symlinkTargetEdits.length, edit.finalPath);
    checkCancellation(repoRoot, options.cancelFile);
  }
  injectFailure(options, "after-symlink-retargeting");
  journal.state = "editing";
  persistJournal(repoRoot, journalPath, journal);
}

/** 🖊️ Groups the reference edits by path and applies every group atomically. */
export function applyReferenceEditGroups({ plan, repoRoot, options, journal, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, journalPath, editPreparationName, editWritePreparationName, editWriteCandidateName }: {
  readonly plan: TaxonomyPlan;
  readonly repoRoot: string;
  readonly options: TaxonomyApplyOptions;
  readonly journal: MutableJournalRecord;
  readonly backupPreparationName: (identity: string, pid: number, token: string) => string;
  readonly backupWritePreparationName: (pid: number, token: string) => string;
  readonly backupWriteCandidateName: string;
  readonly journalPath: string;
  readonly editPreparationName: (identity: string, pid: number, token: string) => string;
  readonly editWritePreparationName: (pid: number, token: string) => string;
  readonly editWriteCandidateName: string;
}) {
  const editGroups = new Map<string, ReferenceEdit[]>();
  for (const edit of plan.edits) editGroups.set(edit.path, [...(editGroups.get(edit.path) ?? []), edit]);
  const sortedEditGroups = [...editGroups.entries()].sort(([a], [b]) => a.localeCompare(b));
  for (const [index, [path, edits]] of sortedEditGroups.entries()) {
    checkCancellation(repoRoot, options.cancelFile);

    if (!journal.appliedEditPaths.includes(path)) {
      const preimages = new Map(edits.map((edit) => [canonicalJson(edit.preimage), edit.preimage]));
      const preimage = [...preimages.values()][0];
      if (preimages.size !== 1 || !preimage || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson(preimage)) throw new Error(`Reference edit preimage changed: ${path}`);
      backupPath(repoRoot, path, absolutePath(repoRoot, journal.backupRoot), journal, preimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      persistJournal(repoRoot, journalPath, journal);
      applyReferenceEditAtomically(repoRoot, plan, journal, path, editPreparationName, editWritePreparationName, editWriteCandidateName);
      journal.appliedEditPaths.push(path);
      persistJournal(repoRoot, journalPath, journal);
    }
    report(options.progress, "apply", "edits", index + 1, sortedEditGroups.length, path);
  }
}

/** 🔄️ Runs every regeneration command and verifies its outputs against the plan. */
export function regenerateGeneratorOutputs({ repoRoot, options, journal, journalPath, plan, taxonomy, backupPreparationName, backupWritePreparationName, backupWriteCandidateName }: {
  readonly repoRoot: string;
  readonly options: TaxonomyApplyOptions;
  readonly journal: MutableJournalRecord;
  readonly journalPath: string;
  readonly plan: TaxonomyPlan;
  readonly taxonomy: LoadedTaxonomy;
  readonly backupPreparationName: (identity: string, pid: number, token: string) => string;
  readonly backupWritePreparationName: (pid: number, token: string) => string;
  readonly backupWriteCandidateName: string;
}) {
  checkCancellation(repoRoot, options.cancelFile);
  injectFailure(options, "after-edits");
  journal.state = "regenerating";
  persistJournal(repoRoot, journalPath, journal);
  for (const [index, regeneration] of plan.regenerations.entries()) {

    checkCancellation(repoRoot, options.cancelFile);
    if (validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    if (journal.completedRegenerationIds.includes(regeneration.id)) {
      if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.outputs)) throw new Error(`Completed regeneration output changed: ${regeneration.id}`);
      if (regeneration.verifyCommand) execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
      report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
      continue;
    }
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Regeneration output preimage changed before execution: ${regeneration.id}`);
    if (!journal.startedRegenerationIds.includes(regeneration.id)) {
      for (const output of regeneration.preOutputs) if (output.nodeKind !== "directory") backupPath(repoRoot, output.path, absolutePath(repoRoot, journal.backupRoot), journal, output as TaxonomyBackupExpectedPreimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      journal.startedRegenerationIds.push(regeneration.id);
      persistJournal(repoRoot, journalPath, journal);
    }
    execFileSync(regeneration.command[0], [...regeneration.command.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
    checkCancellation(repoRoot, options.cancelFile);
    const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
    if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.outputs)) throw new Error(`Regeneration ${regeneration.id} produced missing, stale, unexpected, byte-different, or mode-different output`);
    durablySyncGeneratorRecords(repoRoot, actualOutputs);
    if (regeneration.verifyCommand) execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
    checkCancellation(repoRoot, options.cancelFile);
    journal.completedRegenerationIds.push(regeneration.id);
    persistJournal(repoRoot, journalPath, journal);
    report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
  }
  injectFailure(options, "after-regenerations");
  checkCancellation(repoRoot, options.cancelFile);
  if ((plan.regenerations.length > 0 || plan.moves.some((move) => move.sourceAuthority !== undefined)) && validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
}

/** 🧾️ Verifies the post-state, commits the journal, cleans up the transaction and releases the lease. */
export function verifyAndCommitTransaction({ journal, repoRoot, journalPath, options, plan, ticketRoot, taxonomy, planAuthority, transactionRootRelative, transactionProbe, releaseLease }: {
  readonly journal: MutableJournalRecord;
  readonly repoRoot: string;
  readonly journalPath: string;
  readonly options: TaxonomyApplyOptions;
  readonly plan: TaxonomyPlan;
  readonly ticketRoot: string;
  readonly taxonomy: LoadedTaxonomy;
  readonly planAuthority: { path: string; bytes: Buffer<ArrayBuffer>; } | undefined;
  readonly transactionRootRelative: string;
  readonly transactionProbe: (phase: string, path?: string) => void;
  readonly releaseLease: () => void;
}) {
  journal.state = "verifying";
  persistJournal(repoRoot, journalPath, journal);
  injectFailure(options, "before-verify");
  const prunableSourceParents = emptySourceParents(repoRoot, plan, ticketRoot);
  const projectionState = artifactProjectionPostApplyViolations(repoRoot, plan, taxonomy, new Set(prunableSourceParents));
  if (projectionState.length > 0) throw new Error(`Projection verification failed: ${requireTaxonomyValue(projectionState[0],"projectionState[0]").code} at ${requireTaxonomyValue(projectionState[0],"projectionState[0]").path}`);
  const staleProjectionTokens = projectionStaleViolations(repoRoot, plan, taxonomy, undefined, options.explicitTicketDir);
  if (staleProjectionTokens.length > 0) throw new Error(`Projection verification found ${staleProjectionTokens.length} stale old-hierarchy token(s): ${requireTaxonomyValue(staleProjectionTokens[0],"staleProjectionTokens[0]").path}`);
  if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Post-state digest does not match plan expectation");
  const oldTargets = new Set<string>([...plan.moves.map((entry) => entry.sourcePath), ...plan.evidenceRemovals.map((entry) => entry.sourcePath)]);
  for (const root of plan.embeddedTicketRoots) for (const path of embeddedTargetPaths(plan, root)) oldTargets.add(path);
  const staleTransactionReferences = lexicalTargetIncomingReferences(repoRoot, oldTargets, [], taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress);
  if (staleTransactionReferences.length > 0) throw new Error(`Post-state contains ${staleTransactionReferences.length} structured reference(s) to disposed source paths`);
  const postInventoryRaw = inventoryTaxonomyWithSourceParentPruning({ repoRoot, scope: plan.scope, ticketDir: options.explicitTicketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile }, new Set(prunableSourceParents));
  const exactPlanArtifact = (() => {
    if (!planAuthority) return false;
    const stat = lstatOrNull(absolutePath(repoRoot, planAuthority.path));
    return Boolean(stat?.isFile() && !stat.isSymbolicLink() && readFileSync(absolutePath(repoRoot, planAuthority.path)).equals(planAuthority.bytes));
  })();
  const postInventory = inventoryAfterSourceParentPruning(inventoryWithoutTransactionEvidence(postInventoryRaw, transactionRootRelative, exactPlanArtifact ? planAuthority!.path : undefined), prunableSourceParents);
  const postPlan = planTaxonomy(postInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile });
  const pendingPostOperations = postPlan.moves.length + postPlan.embeddedTicketRoots.length + postPlan.embeddedTicketRootRelocations.length + postPlan.symlinkTargetEdits.length + postPlan.evidenceRemovals.length + postPlan.edits.length + postPlan.regenerations.length;
  if (pendingPostOperations > 0 || postPlan.unresolved.some((entry) => entry.severity === "error")) throw new Error(`Post-state does not converge to an empty plan: ${pendingPostOperations} operation(s), ${postPlan.unresolved.length} finding(s)${postPlan.unresolved[0] ? `; first ${postPlan.unresolved[0].code} at ${postPlan.unresolved[0].path}` : ""}`);
  checkCancellation(repoRoot, options.cancelFile);
  if (canonicalJson(emptySourceParents(repoRoot, plan, ticketRoot)) !== canonicalJson(prunableSourceParents)) throw new Error("Exact source-parent pruning set changed before commit");
  journal.sourceParentPrunePaths = [...prunableSourceParents].sort(generatorPathCompare);
  journal.state = "committed";
  persistJournal(repoRoot, journalPath, journal);
  transactionProbe("transaction-source-parent-pruning-committed", sourceRelative(relative(repoRoot, journalPath)));
  cleanupCommittedTransaction(repoRoot, journal, plan, ticketRoot);
  const appliedOperations = plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length;
  report(options.progress, "apply", "complete", appliedOperations, appliedOperations);
  releaseLease();
}
