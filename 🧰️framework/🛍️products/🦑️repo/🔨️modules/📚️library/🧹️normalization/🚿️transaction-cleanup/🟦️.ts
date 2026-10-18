import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { lstatOrNull } from "../📁️input/🟦️.ts";
import { lstatSync, readdirSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { join } from "node:path";
import type { TaxonomyBackupRecord, TaxonomyPlan } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, durableRemove } from "../💾️durable-journal/🟦️.ts";
import { hashPath, leafPreimage } from "../🧐️preimage-assertions/🟦️.ts";
import { noFollowTreeDigest } from "../🌳️tree-digests/🟦️.ts";
import { committedSourceParentPrunePaths, pruneEmptySourceParents } from "../⏪️rollback-reconciliation/🟦️.ts";

export function cleanupCommittedTransaction(repoRoot: string, journal: MutableJournalRecord, plan: TaxonomyPlan, ticketRoot: string, validateOnly = false): void {
  if (journal.state !== "committed") throw new Error("Committed cleanup requires a terminal committed journal");
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  if (!lstatOrNull(stagingRoot) && !lstatOrNull(backupRoot)) return;
  const pruning = committedSourceParentPrunePaths(repoRoot, plan, journal, ticketRoot);
  const stagingStat = lstatOrNull(stagingRoot);
  if (stagingStat) {
    if (!stagingStat.isDirectory() || stagingStat.isSymbolicLink()) throw new Error("Committed staging root must be a no-follow directory");
    const expectedTop = [
      ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
      ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
      ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
    ].sort(generatorPathCompare);
    if (canonicalJson(readdirSync(stagingRoot).sort(generatorPathCompare)) !== canonicalJson(expectedTop)) throw new Error("Committed staging root has unexpected or missing operation evidence");
  }
  for (const root of plan.embeddedTicketRoots) {
    const staged = normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`);
    if (lstatOrNull(absolutePath(repoRoot, staged)) && canonicalJson(noFollowTreeDigest(repoRoot, staged)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Committed embedded root residual tree changed: ${root.operationId}`);
  }
  for (const removal of plan.evidenceRemovals) {
    const staged = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    if (lstatOrNull(staged) && canonicalJson(leafPreimage(staged)) !== canonicalJson(removal.preimage)) throw new Error(`Committed removal stage preimage changed: ${removal.operationId}`);
  }
  for (const edit of plan.symlinkTargetEdits) {
    const staged = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (lstatOrNull(staged) && (!lstatSync(staged).isSymbolicLink() || readlinkSync(staged) !== edit.oldTarget)) throw new Error(`Committed symlink stage preimage changed: ${edit.operationId}`);
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind !== "file") continue;
    const stored = join(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`Committed typed backup changed: ${path}`);
  }
  const backupStat = lstatOrNull(backupRoot);
  if (backupStat) {
    if (!backupStat.isDirectory() || backupStat.isSymbolicLink()) throw new Error("Committed backup root must be a no-follow directory");
    const expected = Object.values(journal.backups).filter((entry): entry is Extract<TaxonomyBackupRecord, { kind: "file" }> => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
    if (canonicalJson(readdirSync(backupRoot).sort(generatorPathCompare)) !== canonicalJson(expected)) throw new Error("Committed backup root has unexpected or missing evidence");
  } else if (Object.values(journal.backups).some((entry) => entry.kind === "file")) throw new Error("Committed backup root is missing frozen file evidence");
  if (!validateOnly) {
    pruneEmptySourceParents(repoRoot, pruning);
    if (stagingStat) {
      durableRemove(stagingRoot, true);
      journal.probe?.("transaction-terminal-committed-stage-removed", journal.attemptOrdinal);
    }
    if (backupStat) durableRemove(backupRoot, true);
  }
}

export function assertActiveTransactionEvidence(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, exact: boolean, backupPreparationsAlreadyClassified = false): void {
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stage = lstatOrNull(stagingRoot), backup = lstatOrNull(backupRoot);
  if (!stage?.isDirectory() || stage.isSymbolicLink() || !backup?.isDirectory() || backup.isSymbolicLink()) throw new Error("Active transaction stage/backup roots must be direct no-follow directories");
  const allowedStage = new Set([
    ...plan.moves.map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
    journal.journalWriteDirectory,
  ]);
  const actualStage = readdirSync(stagingRoot).sort(generatorPathCompare);
  const unexpectedStage = actualStage.find((name) => !allowedStage.has(name));
  if (unexpectedStage) throw new Error(`Active transaction staging root contains unexpected evidence: ${unexpectedStage}`);
  const expectedBackups = Object.values(journal.backups).filter((entry): entry is Extract<TaxonomyBackupRecord, { kind: "file" }> => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
  const actualBackups = readdirSync(backupRoot).sort(generatorPathCompare);
  if (!backupPreparationsAlreadyClassified && canonicalJson(actualBackups) !== canonicalJson(expectedBackups)) throw new Error("Active transaction backup root contains unexpected or missing evidence");
  if (!exact) return;
  const expectedStage = [
    ...plan.moves.filter((entry) => journal.stagedMoveIds.includes(entry.operationId) && !journal.installedMoveIds.includes(entry.operationId)).map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.filter((entry) => journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && !journal.installedEmbeddedRelocationIds.includes(entry.operationId)).map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.filter((entry) => journal.stagedEvidenceRemovalIds.includes(entry.operationId)).map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.filter((entry) => journal.stagedEmbeddedRootIds.includes(entry.operationId)).map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.filter((entry) => journal.stagedSymlinkTargetEditIds.includes(entry.operationId)).map((entry) => `symlink-${entry.operationId}`),
  ].sort(generatorPathCompare);
  if (canonicalJson(actualStage) !== canonicalJson(expectedStage)) throw new Error("Active transaction staging root does not match its exact journal tuple set");
}
