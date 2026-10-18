import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji, sourceRelative } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync, readdirSync, readlinkSync, rmdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { dirname, join, posix, relative } from "node:path";
import { type TaxonomyApplyOptions, type TaxonomyFailureStage, type TaxonomyInventory, type TaxonomyPlan, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, fsyncDirectory } from "../💾️durable-journal/🟦️.ts";
import { assertJournalPlanMembership } from "../🩹️recovery-authorities/🟦️.ts";
import { generatorTreeInventory } from "../🏭️generator-inputs/🟦️.ts";
import { applyEditsToContent } from "../✂️reference-edits/🟦️.ts";
import { sourceTreeDigest } from "../👥️mutation-projection/🟦️.ts";
import { directoryHash, inventoryDigestOf } from "../📏️fixed-contracts/🟦️.ts";
import { hashPath, leafPreimage, retargetedMovePreimage } from "../🧐️preimage-assertions/🟦️.ts";
import { inheritReferenceInventoryContext } from "../🔗️reference-inventory/🟦️.ts";
import { noFollowTreeDigest, noFollowTreeDigestExcluding } from "../🌳️tree-digests/🟦️.ts";

export function injectFailure(options: TaxonomyApplyOptions, stage: TaxonomyFailureStage): void {
  if (options.injectFailureAt === stage) throw new Error(`Injected taxonomy failure at ${stage}`);
}

export function emptySourceParents(repoRoot: string, plan: TaxonomyPlan, ticketRoot: string): readonly string[] {
  const candidates = new Set<string>();
  const ticketRelative = sourceRelative(relative(repoRoot, ticketRoot));
  for (const move of plan.moves) {
    let parent = posix.dirname(move.sourcePath);
    while (parent && parent !== "." && parent !== ticketRelative && !ticketRelative.startsWith(`${parent}/`)) {
      candidates.add(parent);
      parent = posix.dirname(parent);
    }
  }
  const empty = new Set<string>();
  for (const path of [...candidates].sort((left, right) => right.split("/").length - left.split("/").length || generatorPathCompare(right, left))) {
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Move source parent", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isDirectory()) throw new Error(`Move source parent is no longer a directory: ${path}`);
    if (readdirSync(absolute).every((name) => empty.has(`${path}/${name}`))) empty.add(path);
  }
  return [...empty];
}

export function inventoryAfterSourceParentPruning(inventory: TaxonomyInventory, paths: readonly string[]): TaxonomyInventory {
  if (paths.length === 0) return inventory;
  const pruned = new Set(paths);
  if (inventory.entries.some((entry) => pruned.has(entry.sourcePath) && entry.nodeKind !== "directory")) throw new Error("Source-parent pruning may only project exact directory entries");
  const entries = inventory.entries.filter((entry) => !pruned.has(entry.sourcePath)).map((entry) => ({ ...entry, referencesIn: entry.referencesIn.filter((path) => !pruned.has(path)) }));
  const referenced = entries.find((entry) => entry.referencesOut.some((path) => pruned.has(path)));
  if (referenced) throw new Error(`Prunable source directory remains referenced by ${referenced.sourcePath}`);
  const childrenByParent = new Map<string, typeof entries>();
  for (const entry of entries) {
    const parent = posix.dirname(entry.sourcePath);
    const children = childrenByParent.get(parent) ?? [];
    children.push(entry);
    childrenByParent.set(parent, children);
  }
  for (const entry of entries.filter((entry) => entry.nodeKind === "directory").sort((left, right) => right.sourcePath.split("/").length - left.sourcePath.split("/").length)) entry.contentHash = directoryHash(entry.sourcePath, childrenByParent.get(entry.sourcePath) ?? []);
  const { inventoryDigest: _inventoryDigest, repoRoot, taxonomyPath, ...metadata } = inventory;
  const partial = { ...metadata, entries, violations: inventory.violations.filter((entry) => !pruned.has(entry.path)), sourceTreeDigest: sourceTreeDigest(entries) };
  return inheritReferenceInventoryContext(inventory, { ...partial, repoRoot, taxonomyPath, inventoryDigest: inventoryDigestOf(partial) });
}

export function pruneEmptySourceParents(repoRoot: string, paths: readonly string[]): void {
  for (const path of paths) {
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Committed move source parent", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isDirectory() || readdirSync(absolute).length !== 0) throw new Error(`Committed source-parent pruning preimage changed: ${path}`);
    rmdirSync(absolute);
    fsyncDirectory(dirname(absolute));
  }
}

export function committedSourceParentPrunePaths(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, ticketRoot: string): readonly string[] {
  assertJournalPlanMembership(plan, journal);
  const ticketRelative = sourceRelative(relative(repoRoot, ticketRoot));
  const paths = [...journal.sourceParentPrunePaths].sort((left, right) => right.split("/").length - left.split("/").length || generatorPathCompare(right, left));
  const approved = new Set(paths);
  for (const path of paths) {
    if (path === ticketRelative || ticketRelative.startsWith(`${path}/`)) throw new Error(`Committed source-parent pruning cannot remove the ticket root or its ancestors: ${path}`);
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Journal-bound source parent", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isDirectory() || readdirSync(absolute).some((name) => !approved.has(`${path}/${name}`))) throw new Error(`Committed source-parent pruning preimage changed: ${path}`);
  }
  return paths;
}

export function rollbackDestinationAncestors(repoRoot: string, plan: TaxonomyPlan): void {
  for (const ancestor of plan.destinationAncestorPreimages.filter((entry) => entry.state === "absent").sort((left, right) => right.path.split("/").length - left.path.split("/").length || generatorPathCompare(right.path, left.path))) {
    const path = absolutePath(repoRoot, ancestor.path);
    const stat = lstatOrNull(path);
    if (!stat) continue;
    if (!stat.isDirectory() || stat.isSymbolicLink() || readdirSync(path).length > 0) throw new Error(`Rollback-created destination ancestor is occupied: ${ancestor.path}`);
    rmdirSync(path);
    fsyncDirectory(dirname(path));
  }
}

export function reconcileRollbackTuples(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): boolean {
  let changed = false;
  const remove = (array: string[], id: string): void => { const next = array.filter((entry) => entry !== id); if (next.length !== array.length) { array.splice(0, array.length, ...next); changed = true; } };
  const add = (array: string[], id: string): void => { if (!array.includes(id)) { array.push(id); changed = true; } };
  const present = (path: string): boolean => Boolean(lstatOrNull(path));
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const pendingRestorePaths = new Set<string>();
  for (const path of Object.keys(journal.backups)) {
    const identity = sha256(path).slice(0, 24);
    if (readdirSync(backupRoot).some((name) => splitLeadingEmoji(name).rest.startsWith(`restore-${identity}-`))) pendingRestorePaths.add(path);
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind === "symlink" && (backup.targetHash !== sha256(backup.target) || backup.size !== Buffer.byteLength(backup.target))) throw new Error(`rollback-state-drift: symlink backup ${path}`);
    if (backup.kind !== "file") continue;
    const stored = join(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`rollback-state-drift: file backup ${path}`);
  }
  for (const move of plan.moves) {
    const source = absolutePath(repoRoot, move.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination = absolutePath(repoRoot, move.destinationPath);
    const states = [present(source), present(stage), present(destination)];
    const pendingRestoreExchange = states.every((state) => !state) && pendingRestorePaths.has(move.destinationPath);
    if (!pendingRestoreExchange && states.filter(Boolean).length !== 1) throw new Error(`rollback-state-drift: move ${move.operationId}`);
    if (pendingRestoreExchange) continue;
    const index = states.findIndex(Boolean);
    const current = requireTaxonomyValue([source, stage, destination][index],"[source, stage, destination][index]");
    const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
    let edited = false;
    let expectedPreimage = retargetedMovePreimage(move, installedLink);
    if (index === 2 && journal.appliedEditPaths.includes(move.destinationPath)) {
      const backup = journal.backups[move.destinationPath];
      if (!backup || backup.kind !== "file") throw new Error(`rollback-state-drift: move edit backup ${move.operationId}`);
      const result = applyEditsToContent(readFileSync(join(absolutePath(repoRoot, journal.backupRoot), backup.backupPath), "utf8"), plan.edits.filter((edit) => edit.path === move.destinationPath));
      const postimage = { nodeKind: "file", contentHash: sha256(result), mode: backup.mode, size: Buffer.byteLength(result) } as const;
      edited = canonicalJson(leafPreimage(current)) === canonicalJson(postimage);
      if (edited) expectedPreimage = postimage;
    }
    if (canonicalJson(leafPreimage(current)) !== canonicalJson(expectedPreimage)) throw new Error(`rollback-state-drift: move preimage ${move.operationId}`);
    if (states[0]) { remove(journal.installedMoveIds, move.operationId); remove(journal.stagedMoveIds, move.operationId); remove(journal.preparedMoveIds, move.operationId); }
    else if (states[1]) { remove(journal.installedMoveIds, move.operationId); add(journal.stagedMoveIds, move.operationId); add(journal.preparedMoveIds, move.operationId); }
    else if (!journal.installedMoveIds.includes(move.operationId)) throw new Error(`rollback-state-drift: unowned move destination ${move.operationId}`);
  }
  for (const entry of plan.embeddedTicketRootRelocations) {
    const source = absolutePath(repoRoot, entry.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), destination = absolutePath(repoRoot, entry.destinationPath);
    const states = [present(source), present(stage), present(destination)];
    if (states.filter(Boolean).length !== 1) throw new Error(`rollback-state-drift: relocation ${entry.operationId}`);
    const current = requireTaxonomyValue([source, stage, destination][states.findIndex(Boolean)],"[source, stage, destination][states.findIndex(Boolean)]");
    if (canonicalJson(leafPreimage(current)) !== canonicalJson(entry.preimage)) throw new Error(`rollback-state-drift: relocation preimage ${entry.operationId}`);
    if (states[0]) { remove(journal.installedEmbeddedRelocationIds, entry.operationId); remove(journal.stagedEmbeddedRelocationIds, entry.operationId); remove(journal.preparedEmbeddedRelocationIds, entry.operationId); }
    else if (states[1]) { remove(journal.installedEmbeddedRelocationIds, entry.operationId); add(journal.stagedEmbeddedRelocationIds, entry.operationId); add(journal.preparedEmbeddedRelocationIds, entry.operationId); }
    else if (!journal.installedEmbeddedRelocationIds.includes(entry.operationId)) throw new Error(`rollback-state-drift: unowned relocation destination ${entry.operationId}`);
  }
  for (const entry of plan.evidenceRemovals) {
    const source = absolutePath(repoRoot, entry.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`);
    const states = [present(source), present(stage)];
    if (states.filter(Boolean).length !== 1 || canonicalJson(leafPreimage(states[0] ? source : stage)) !== canonicalJson(entry.preimage)) throw new Error(`rollback-state-drift: removal ${entry.operationId}`);
    if (states[0]) { remove(journal.stagedEvidenceRemovalIds, entry.operationId); remove(journal.preparedEvidenceRemovalIds, entry.operationId); }
    else { add(journal.stagedEvidenceRemovalIds, entry.operationId); add(journal.preparedEvidenceRemovalIds, entry.operationId); }
  }
  for (const root of plan.embeddedTicketRoots) {
    const source = absolutePath(repoRoot, root.sourceMetadataRoot), stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    const states = [present(source), present(stage)];
    if (states.filter(Boolean).length !== 1) throw new Error(`rollback-state-drift: embedded root ${root.operationId}`);
    const current = states[0] ? root.sourceMetadataRoot : normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`);
    const children = [...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath), ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)];
    const tree = states[0] ? noFollowTreeDigestExcluding(repoRoot, current, children) : noFollowTreeDigest(repoRoot, current);
    if (canonicalJson(tree) !== canonicalJson(root.residualTreeDigest)) throw new Error(`rollback-state-drift: embedded root tree ${root.operationId}`);
    if (states[0]) { remove(journal.stagedEmbeddedRootIds, root.operationId); remove(journal.preparedEmbeddedRootIds, root.operationId); }
    else { add(journal.stagedEmbeddedRootIds, root.operationId); add(journal.preparedEmbeddedRootIds, root.operationId); }
  }
  for (const edit of plan.symlinkTargetEdits) {
    const link = absolutePath(repoRoot, edit.finalPath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    const linkStat = lstatOrNull(link), stageStat = lstatOrNull(stage);
    const oldAtLink = linkStat?.isSymbolicLink() && readlinkSync(link) === edit.oldTarget;
    const newAtLink = linkStat?.isSymbolicLink() && readlinkSync(link) === edit.newTarget;
    const oldAtStage = stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget;
    if (oldAtLink && !stageStat) { remove(journal.installedSymlinkTargetEditIds, edit.operationId); remove(journal.stagedSymlinkTargetEditIds, edit.operationId); remove(journal.preparedSymlinkTargetEditIds, edit.operationId); }
    else if ((!linkStat || newAtLink) && oldAtStage) { add(journal.stagedSymlinkTargetEditIds, edit.operationId); add(journal.preparedSymlinkTargetEditIds, edit.operationId); if (newAtLink) add(journal.installedSymlinkTargetEditIds, edit.operationId); else remove(journal.installedSymlinkTargetEditIds, edit.operationId); }
    else throw new Error(`rollback-state-drift: symlink edit ${edit.operationId}`);
  }
  for (const path of [...new Set(plan.edits.map((entry) => entry.path))].filter((entry) => journal.backups[entry])) {
    if (pendingRestorePaths.has(path)) continue;
    const backup = journal.backups[path];
    if (!backup || backup.kind !== "file") throw new Error(`rollback-state-drift: edit backup ${path}`);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    const pre = stat?.isFile() && !stat.isSymbolicLink() && hashPath(current) === backup.contentHash && (stat.mode & 0o7777) === backup.mode && stat.size === backup.size;
    const result = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), plan.edits.filter((edit) => edit.path === path));
    const post = stat?.isFile() && !stat.isSymbolicLink() && readFileSync(current, "utf8") === result && (stat.mode & 0o7777) === backup.mode && stat.size === Buffer.byteLength(result);
    if (pre) {
      if (!journal.appliedEditPaths.includes(path)) throw new Error(`rollback-state-drift: unowned restored edit ${path}`);
    }
    else if (post) add(journal.appliedEditPaths, path);
    else throw new Error(`rollback-state-drift: edit ${path}`);
  }
  for (const regeneration of plan.regenerations) {
    if (journal.startedRegenerationIds.includes(regeneration.id)) continue;
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`rollback-state-drift: unstarted regeneration ${regeneration.id}`);
  }
  return changed;
}
