import { report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { lstatOrNull } from "../📁️input/🟦️.ts";
import { chmodSync, mkdirSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { dirname, join } from "node:path";
import type { TaxonomyApplyOptions, TaxonomyBackupRecord, TaxonomyPlan } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, durableRemove, durableRename, fsyncDirectory, persistJournal } from "../💾️durable-journal/🟦️.ts";
import { generatorTreeInventory } from "../🏭️generator-inputs/🟦️.ts";
import { canonicalDirectoryName, hashPath } from "../🧐️preimage-assertions/🟦️.ts";
import { reconcileRollbackTuples, rollbackDestinationAncestors } from "../⏪️rollback-reconciliation/🟦️.ts";
import { restoreBackup } from "../🛟️recovery/🟦️.ts";
import { actualAffectedPreDigest } from "../🗃️affected-state/🟦️.ts";

export function cleanupRolledBackTransaction(repoRoot: string, journal: MutableJournalRecord, plan: TaxonomyPlan, validateOnly = false): void {
  if (journal.state !== "rolled-back") throw new Error("Rolled-back cleanup requires a terminal rolled-back journal");
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stagingStat = lstatOrNull(stagingRoot);
  const backupStat = lstatOrNull(backupRoot);
  if (!stagingStat && !backupStat) return;
  if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Rolled-back cleanup pre-state digest changed");
  if (stagingStat) {
    if (!stagingStat.isDirectory() || stagingStat.isSymbolicLink()) throw new Error("Rolled-back staging root is not a no-follow directory");
    if (readdirSync(stagingRoot).length > 0) throw new Error("Rolled-back staging root contains unexpected evidence");
  }
  if (backupStat) {
    if (!backupStat.isDirectory() || backupStat.isSymbolicLink()) throw new Error("Rolled-back backup root is not a no-follow directory");
    const expected = Object.values(journal.backups).filter((entry): entry is Extract<TaxonomyBackupRecord, { kind: "file" }> => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
    const actual = readdirSync(backupRoot).sort(generatorPathCompare);
    if (canonicalJson(actual) !== canonicalJson(expected)) throw new Error("Rolled-back backup root contains unexpected or missing evidence");
    for (const [path, backup] of Object.entries(journal.backups)) {
      if (backup.kind !== "file") continue;
      const stored = join(backupRoot, backup.backupPath);
      const stat = lstatOrNull(stored);
      if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`Rolled-back backup changed: ${path}`);
    }
  }
  if (!validateOnly) {
    if (stagingStat) {
      durableRemove(stagingRoot, true);
      journal.probe?.("transaction-terminal-rolled-back-stage-removed", journal.attemptOrdinal);
    }
    if (backupStat) durableRemove(backupRoot, true);
  }
}

export function rollbackTransaction(repoRoot: string, plan: TaxonomyPlan, journalPath: string, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy, options: TaxonomyApplyOptions): void {
  if (journal.state === "rolling-back") {
    if (reconcileRollbackTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
  } else {
    if (reconcileRollbackTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    journal.state = "rolling-back";
    persistJournal(repoRoot, journalPath, journal);
  }
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const restorePreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const started = new Set(journal.startedRegenerationIds);
  for (const regeneration of [...plan.regenerations].reverse()) {
    if (!started.has(regeneration.id)) continue;
    for (const root of [...regeneration.outputRoots].sort((left, right) => right.length - left.length || generatorPathCompare(right, left))) {
      const path = absolutePath(repoRoot, root);
      if (lstatOrNull(dirname(path))) durableRemove(path, true);
    }
    for (const directory of regeneration.preOutputs.filter((entry) => entry.nodeKind === "directory").sort((left, right) => left.path.split("/").length - right.path.split("/").length || generatorPathCompare(left.path, right.path))) {
      const path = absolutePath(repoRoot, directory.path);
      mkdirSync(path, { recursive: true });
      chmodSync(path, directory.mode);
      fsyncDirectory(path);
      fsyncDirectory(dirname(path));
    }
  }
  for (const [path, backup] of Object.entries(journal.backups).sort(([a], [b]) => generatorPathCompare(b, a))) {
    restoreBackup(repoRoot, plan, path, backupRoot, backup, restorePreparationName, journal.probe);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const regeneration of plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id))) {
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Rollback regeneration pre-state is incomplete: ${regeneration.id}`);
  }
  for (const edit of [...plan.symlinkTargetEdits].reverse()) {
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId)) continue;
    const link = absolutePath(repoRoot, edit.finalPath);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (lstatOrNull(link)) durableRemove(link);
    if (lstatOrNull(stage)) { mkdirSync(dirname(link), { recursive: true }); durableRename(stage, link); }
    report(options.progress, "apply", "rolling-back-symlink-target-edits", 1, plan.symlinkTargetEdits.length, edit.finalPath);
    journal.installedSymlinkTargetEditIds = journal.installedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    journal.stagedSymlinkTargetEditIds = journal.stagedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    journal.preparedSymlinkTargetEditIds = journal.preparedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const relocation of [...plan.embeddedTicketRootRelocations].reverse()) {
    if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) continue;
    const destination = absolutePath(repoRoot, relocation.destinationPath);
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(destination, stage); }
    journal.installedEmbeddedRelocationIds = journal.installedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  const activeIds = new Set([...journal.stagedMoveIds, ...journal.installedMoveIds]);
  for (const move of [...plan.moves].reverse()) {
    if (!journal.installedMoveIds.includes(move.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const destination = absolutePath(repoRoot, move.destinationPath);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) {
      mkdirSync(dirname(stage), { recursive: true });
      durableRename(destination, stage);
    }
    journal.installedMoveIds = journal.installedMoveIds.filter((id) => id !== move.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const root of [...plan.embeddedTicketRoots].reverse()) {
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    const source = absolutePath(repoRoot, root.sourceMetadataRoot);
    if (lstatOrNull(stage)) { if (lstatOrNull(source)) throw new Error(`Rollback embedded root source is occupied: ${root.sourceMetadataRoot}`); mkdirSync(dirname(source), { recursive: true }); durableRename(stage, source); }
    journal.stagedEmbeddedRootIds = journal.stagedEmbeddedRootIds.filter((id) => id !== root.operationId);
    journal.preparedEmbeddedRootIds = journal.preparedEmbeddedRootIds.filter((id) => id !== root.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const relocation of [...plan.embeddedTicketRootRelocations].reverse()) {
    if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    const source = absolutePath(repoRoot, relocation.sourcePath);
    if (lstatOrNull(stage)) { if (lstatOrNull(source)) throw new Error(`Rollback relocation source is occupied: ${relocation.sourcePath}`); mkdirSync(dirname(source), { recursive: true }); durableRename(stage, source); }
    journal.stagedEmbeddedRelocationIds = journal.stagedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    journal.preparedEmbeddedRelocationIds = journal.preparedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const removal of [...plan.evidenceRemovals].reverse()) {
    if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    const source = absolutePath(repoRoot, removal.sourcePath);
    if (lstatOrNull(stage)) { if (lstatOrNull(source)) throw new Error(`Rollback removal source is occupied: ${removal.sourcePath}`); mkdirSync(dirname(source), { recursive: true }); durableRename(stage, source); }
    journal.stagedEvidenceRemovalIds = journal.stagedEvidenceRemovalIds.filter((id) => id !== removal.operationId);
    journal.preparedEvidenceRemovalIds = journal.preparedEvidenceRemovalIds.filter((id) => id !== removal.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const move of [...plan.moves].reverse()) {
    if (!activeIds.has(move.operationId)) continue;
    const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const source = absolutePath(repoRoot, move.sourcePath);
    if (lstatOrNull(stage)) {
      mkdirSync(dirname(source), { recursive: true });
      if (lstatOrNull(source)) throw new Error(`Rollback source is occupied: ${move.sourcePath}`);
      durableRename(stage, source);
    }
    journal.stagedMoveIds = journal.stagedMoveIds.filter((id) => id !== move.operationId);
    journal.preparedMoveIds = journal.preparedMoveIds.filter((id) => id !== move.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  journal.appliedEditPaths = [];
  journal.startedRegenerationIds = [];
  journal.completedRegenerationIds = [];
  journal.installedMoveIds = [];
  journal.stagedMoveIds = [];
  journal.preparedMoveIds = [];
  journal.preparedEmbeddedRelocationIds = [];
  journal.stagedEmbeddedRelocationIds = [];
  journal.installedEmbeddedRelocationIds = [];
  journal.preparedEvidenceRemovalIds = [];
  journal.stagedEvidenceRemovalIds = [];
  journal.preparedEmbeddedRootIds = [];
  journal.stagedEmbeddedRootIds = [];
  journal.preparedSymlinkTargetEditIds = [];
  journal.stagedSymlinkTargetEditIds = [];
  journal.installedSymlinkTargetEditIds = [];
  rollbackDestinationAncestors(repoRoot, plan);
  if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Rollback did not restore the affected pre-state digest");
  journal.state = "rolled-back";
  persistJournal(repoRoot, journalPath, journal);
  cleanupRolledBackTransaction(repoRoot, journal, plan);
}
