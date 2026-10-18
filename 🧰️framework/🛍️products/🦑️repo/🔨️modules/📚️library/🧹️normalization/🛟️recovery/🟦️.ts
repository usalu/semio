import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { randomUUID } from "node:crypto";
import { chmodSync, copyFileSync, linkSync, mkdirSync, readFileSync, readdirSync, readlinkSync, symlinkSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, dirname, join, relative } from "node:path";
import { type TaxonomyBackupRecord, type TaxonomyGeneratorNodeRecord, type TaxonomyLeafPreimage, type TaxonomyPlan, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, durableRemove, durableRename, fsyncDirectory, fsyncFile } from "../💾️durable-journal/🟦️.ts";
import { type TaxonomyBackupExpectedPreimage, TaxonomyGeneratorInputDriftError, TaxonomyMoveSourceInputDriftError, TaxonomyStartedRegenerationPartialError, type TransactionBinaryWritePreparation, expectedBackupRecord, resumeGeneratorInputAuthority, resumeGeneratorInputPhysicalPath, resumeGeneratorInputView, transactionBackupAuthorities, writeTransactionBinaryCandidate } from "../🩹️recovery-authorities/🟦️.ts";
import { generatorInputPaths, generatorNodeRecord, generatorTreeInventory } from "../🏭️generator-inputs/🟦️.ts";
import { applyEditsToContent } from "../✂️reference-edits/🟦️.ts";
import { assertTicketImportantExactRemovalAuthority, assertTicketImportantRemovalAuthority, hashPath, leafPathPreimage, leafPreimage, retargetedMovePreimage } from "../🧐️preimage-assertions/🟦️.ts";
import { isTransactionRepositoryAuthorityError } from "../🏛️repository-authority/🟦️.ts";
import { noFollowTreeDigest, noFollowTreeDigestExcluding } from "../🌳️tree-digests/🟦️.ts";
import { TRANSACTION_LEASE_TOKEN } from "../🔐️transaction-lease/🟦️.ts";

/** 🧾️ Reads an input preimage at its journal-proven source, stage, destination, or edit backup after tuple validation. */
function resumeGeneratorInputRecord(repoRoot: string, authority: ReturnType<typeof resumeGeneratorInputAuthority>, journal: MutableJournalRecord, input: TaxonomyGeneratorNodeRecord, taxonomy: LoadedTaxonomy): TaxonomyGeneratorNodeRecord {
  const current = resumeGeneratorInputPhysicalPath(authority, journal, input.path);
  assertLexicalInputOutsideOpaque(repoRoot, current, "Journal-bound generator input");
  return { ...generatorNodeRecord(repoRoot, current, taxonomy), path: input.path };
}

/** 🔐️ Reconciles only authenticated transaction writes and output tuples, independently of forward read inputs. */
export function reconcileTransactionOwnedTuples(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): boolean {
  let reconciled = false;
  const present = (...paths: string[]): boolean[] => paths.map((path) => Boolean(lstatOrNull(path)));
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const pendingEditPaths = new Set<string>();
  for (const path of new Set(plan.edits.map((entry) => entry.path))) {
    const identity = sha256(path).slice(0, 24);
    if (readdirSync(stageRoot).some((name) => splitLeadingEmoji(name).rest.startsWith(`edit-${identity}-`))) pendingEditPaths.add(path);
  }
  for (const path of [...new Set(plan.edits.map((entry) => entry.path))].sort(generatorPathCompare)) {
    if (journal.appliedEditPaths.includes(path)) continue;
    const backup = journal.backups[path];
    if (backup?.kind !== "file") continue;
    const current = absolutePath(repoRoot, path), stat = lstatOrNull(current);
    const expected = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), plan.edits.filter((entry) => entry.path === path));
    if (stat?.isFile() && !stat.isSymbolicLink() && readFileSync(current, "utf8") === expected && (stat.mode & 0o7777) === backup.mode && stat.size === Buffer.byteLength(expected)) {
      journal.appliedEditPaths.push(path);
      reconciled = true;
    }
  }
  for (const move of plan.moves) {
    const source = absolutePath(repoRoot, move.sourcePath), stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination = absolutePath(repoRoot, move.destinationPath);
    const states = present(source, stage, destination);
    if (!journal.installedMoveIds.includes(move.operationId) && journal.stagedMoveIds.includes(move.operationId) && !states[0] && !states[1] && states[2] && canonicalJson(leafPreimage(destination)) === canonicalJson(move.sourcePreimage)) { journal.installedMoveIds.push(move.operationId); reconciled = true; }
    if (!journal.stagedMoveIds.includes(move.operationId) && journal.preparedMoveIds.includes(move.operationId) && !states[0] && states[1] && !states[2]) { journal.stagedMoveIds.push(move.operationId); reconciled = true; }
    const expected = journal.installedMoveIds.includes(move.operationId) ? 2 : journal.stagedMoveIds.includes(move.operationId) ? 1 : journal.preparedMoveIds.includes(move.operationId) && states[1] ? 1 : 0;
    const pendingEditExchange = expected === 2 && pendingEditPaths.has(move.destinationPath) && states.every((state) => !state);
    if (!pendingEditExchange && (states.filter(Boolean).length !== 1 || !states[expected])) throw new Error(`resume-state-drift: move ${move.operationId}`);
    if (pendingEditExchange) continue;
    const current = requireTaxonomyValue([source, stage, destination][expected],"[source, stage, destination][expected]");
    const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
    if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(current)) !== canonicalJson(retargetedMovePreimage(move, installedLink))) throw new Error(`resume-state-drift: move preimage ${move.operationId}`);
  }
  for (const entry of plan.embeddedTicketRootRelocations) {
    const states = present(absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath));
    if (!journal.installedEmbeddedRelocationIds.includes(entry.operationId) && journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && !states[0] && !states[1] && states[2] && canonicalJson(leafPreimage(absolutePath(repoRoot, entry.destinationPath))) === canonicalJson(entry.preimage)) { journal.installedEmbeddedRelocationIds.push(entry.operationId); reconciled = true; }
    if (!journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && journal.preparedEmbeddedRelocationIds.includes(entry.operationId) && !states[0] && states[1] && !states[2]) { journal.stagedEmbeddedRelocationIds.push(entry.operationId); reconciled = true; }
    const expected = journal.installedEmbeddedRelocationIds.includes(entry.operationId) ? 2 : journal.stagedEmbeddedRelocationIds.includes(entry.operationId) ? 1 : journal.preparedEmbeddedRelocationIds.includes(entry.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected]) throw new Error(`resume-state-drift: embedded relocation ${entry.operationId}`);
    if (canonicalJson(leafPreimage(requireTaxonomyValue([absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath)][expected],"[absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath)][expected]"))) !== canonicalJson(entry.preimage)) throw new Error(`resume-state-drift: embedded relocation preimage ${entry.operationId}`);
  }
  for (const entry of plan.evidenceRemovals) {
    const states = present(absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`));
    if (!journal.stagedEvidenceRemovalIds.includes(entry.operationId) && journal.preparedEvidenceRemovalIds.includes(entry.operationId) && !states[0] && states[1]) { journal.stagedEvidenceRemovalIds.push(entry.operationId); reconciled = true; }
    const expected = journal.stagedEvidenceRemovalIds.includes(entry.operationId) ? 1 : journal.preparedEvidenceRemovalIds.includes(entry.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected]) throw new Error(`resume-state-drift: evidence removal ${entry.operationId}`);
    if (canonicalJson(leafPreimage(requireTaxonomyValue([absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`)][expected],"[absolutePath(repoRoot, entry.sourcePath), join(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`)][expected]"))) !== canonicalJson(entry.preimage)) throw new Error(`resume-state-drift: evidence removal preimage ${entry.operationId}`);
    if (entry.authority.kind === "byte-and-mode-identical") for (const member of entry.authority.members.filter((candidate) => candidate.disposition === "retain")) {
      const owningMove = plan.moves.find((move) => move.sourcePath === member.sourcePath && move.destinationPath === member.finalPath);
      const retained = owningMove
        ? journal.installedMoveIds.includes(owningMove.operationId) ? absolutePath(repoRoot, owningMove.destinationPath)
          : journal.stagedMoveIds.includes(owningMove.operationId) ? join(absolutePath(repoRoot, journal.stagingRoot), owningMove.operationId)
            : absolutePath(repoRoot, owningMove.sourcePath)
        : absolutePath(repoRoot, member.finalPath);
      if (!lstatOrNull(retained) || canonicalJson(leafPreimage(retained)) !== canonicalJson(member.preimage)) throw new Error(`resume-state-drift: retained evidence ${member.finalPath}`);
    } else if (entry.authority.kind === "exact-path-mutation") assertTicketImportantExactRemovalAuthority(repoRoot, entry);
    else assertTicketImportantRemovalAuthority(repoRoot, entry, taxonomy);
  }
  for (const root of plan.embeddedTicketRoots) {
    const states = present(absolutePath(repoRoot, root.sourceMetadataRoot), join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`));
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId) && journal.preparedEmbeddedRootIds.includes(root.operationId) && !states[0] && states[1]) { journal.stagedEmbeddedRootIds.push(root.operationId); reconciled = true; }
    const expected = journal.stagedEmbeddedRootIds.includes(root.operationId) ? 1 : journal.preparedEmbeddedRootIds.includes(root.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected]) throw new Error(`resume-state-drift: embedded root ${root.operationId}`);
    const current = states[1] ? normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`) : root.sourceMetadataRoot;
    const children = [...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath), ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)];
    const currentTree = states[1] ? noFollowTreeDigest(repoRoot, current) : noFollowTreeDigestExcluding(repoRoot, current, children);
    if (canonicalJson(currentTree) !== canonicalJson(root.residualTreeDigest)) throw new Error(`resume-state-drift: embedded root tree ${root.operationId}`);
  }
  for (const edit of plan.symlinkTargetEdits) {
    const link = absolutePath(repoRoot, edit.finalPath), stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    const linkStat = lstatOrNull(link), stageStat = lstatOrNull(stage);
    if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId) && journal.stagedSymlinkTargetEditIds.includes(edit.operationId) && linkStat?.isSymbolicLink() && readlinkSync(link) === edit.newTarget && stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget) { journal.installedSymlinkTargetEditIds.push(edit.operationId); reconciled = true; }
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId) && journal.preparedSymlinkTargetEditIds.includes(edit.operationId) && !linkStat && stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget) { journal.stagedSymlinkTargetEditIds.push(edit.operationId); reconciled = true; }
    if (journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
      if (!linkStat?.isSymbolicLink() || readlinkSync(link) !== edit.newTarget || !stageStat?.isSymbolicLink() || readlinkSync(stage) !== edit.oldTarget) throw new Error(`resume-state-drift: symlink edit ${edit.operationId}`);
    } else if (journal.stagedSymlinkTargetEditIds.includes(edit.operationId) || (journal.preparedSymlinkTargetEditIds.includes(edit.operationId) && stageStat)) {
      if (linkStat || !stageStat?.isSymbolicLink() || readlinkSync(stage) !== edit.oldTarget) throw new Error(`resume-state-drift: symlink stage ${edit.operationId}`);
    } else if (!linkStat?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget) throw new Error(`resume-state-drift: symlink source ${edit.operationId}`);
    const targetMove = plan.moves.find((move) => move.sourcePath === edit.logicalTargetSourcePath && move.destinationPath === edit.logicalTargetFinalPath);
    const targetPath = targetMove ? journal.installedMoveIds.includes(targetMove.operationId) ? absolutePath(repoRoot, targetMove.destinationPath) : journal.stagedMoveIds.includes(targetMove.operationId) ? join(absolutePath(repoRoot, journal.stagingRoot), targetMove.operationId) : absolutePath(repoRoot, targetMove.sourcePath) : absolutePath(repoRoot, edit.logicalTargetSourcePath);
    const targetStat = lstatOrNull(targetPath);
    if (edit.logicalTargetPreimage.state === "absent") {
      if (targetStat) throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    } else if (edit.logicalTargetPreimage.state === "directory") {
      if (!targetStat?.isDirectory() || targetStat.isSymbolicLink()) throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    } else if (!journal.appliedEditPaths.includes(edit.logicalTargetFinalPath) && !pendingEditPaths.has(edit.logicalTargetFinalPath)) {
      const targetLinkEdit = plan.symlinkTargetEdits.find((candidate) => candidate.sourcePath === edit.logicalTargetSourcePath && candidate.finalPath === edit.logicalTargetFinalPath && journal.installedSymlinkTargetEditIds.includes(candidate.operationId));
      if (targetLinkEdit) {
        if (!targetStat?.isSymbolicLink() || readlinkSync(targetPath) !== targetLinkEdit.newTarget) throw new Error(`resume-state-drift: nested symlink logical target ${edit.operationId}`);
      } else if (!targetStat || canonicalJson(leafPathPreimage(targetPath)) !== canonicalJson(edit.logicalTargetPreimage)) throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    }
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind !== "file") continue;
    const stored = join(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || hashPath(stored) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`resume-state-drift: backup ${path}`);
  }
  for (const path of journal.appliedEditPaths) {
    const backup = journal.backups[path];
    if (!backup || backup.kind !== "file") throw new Error(`resume-state-drift: applied edit backup ${path}`);
    const edits = plan.edits.filter((edit) => edit.path === path);
    const expected = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), edits);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    if (!stat?.isFile() || stat.isSymbolicLink() || readFileSync(current, "utf8") !== expected || (stat.mode & 0o7777) !== backup.mode || stat.size !== Buffer.byteLength(expected)) throw new Error(`resume-state-drift: applied edit ${path}`);
  }
  const editPaths = [...new Set(plan.edits.map((entry) => entry.path))].sort(generatorPathCompare);
  for (const path of editPaths.filter((entry) => !journal.appliedEditPaths.includes(entry))) {
    if (pendingEditPaths.has(path)) continue;
    const backup = journal.backups[path];
    if (!backup) continue;
    if (backup.kind !== "file") throw new Error(`resume-state-drift: edit backup kind ${path}`);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    const edits = plan.edits.filter((entry) => entry.path === path);
    const expected = applyEditsToContent(readFileSync(join(backupRoot, backup.backupPath), "utf8"), edits);
    if (stat?.isFile() && !stat.isSymbolicLink() && readFileSync(current, "utf8") === expected && (stat.mode & 0o7777) === backup.mode && stat.size === Buffer.byteLength(expected)) {
      journal.appliedEditPaths.push(path);
      reconciled = true;
    } else if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(current) !== backup.contentHash || (stat.mode & 0o7777) !== backup.mode || stat.size !== backup.size) throw new Error(`resume-state-drift: prepared edit ${path}`);
  }
  const startedOutputs = new Set(plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id)).flatMap((entry) => entry.preOutputs.map((output) => output.path)));
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (journal.appliedEditPaths.includes(path) || startedOutputs.has(path) || backup.kind === "file") continue;
    const current = lstatOrNull(absolutePath(repoRoot, path));
    if ((backup.kind === "absent" && current) || (backup.kind === "symlink" && (!current?.isSymbolicLink() || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson({ nodeKind: "symlink", contentHash: backup.targetHash, mode: backup.mode, size: backup.size, target: backup.target })))) throw new Error(`resume-state-drift: typed backup source ${path}`);
  }
  for (const regeneration of plan.regenerations) {
    const outputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
    const pre = canonicalJson(outputs) === canonicalJson(regeneration.preOutputs);
    const post = canonicalJson(outputs) === canonicalJson(regeneration.outputs);
    if (journal.completedRegenerationIds.includes(regeneration.id) && !post) throw new Error(`resume-state-drift: regeneration outputs ${regeneration.id}`);
    if (journal.startedRegenerationIds.includes(regeneration.id) && !journal.completedRegenerationIds.includes(regeneration.id) && !pre && !post) throw new TaxonomyStartedRegenerationPartialError(regeneration.id);
    if (!journal.startedRegenerationIds.includes(regeneration.id) && !pre) throw new Error(`resume-state-drift: regeneration outputs ${regeneration.id}`);
    if (journal.startedRegenerationIds.includes(regeneration.id) && !journal.completedRegenerationIds.includes(regeneration.id) && post) {
      journal.completedRegenerationIds.push(regeneration.id);
      reconciled = true;
    }
  }
  return reconciled;
}

/** 🪪️ Requires every exact current-source input at its journal-proven logical preimage location. */
function validateForwardMoveSourceInputs(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): void {
  const inputAuthority = resumeGeneratorInputAuthority(plan, journal);
  for (const move of plan.moves) {
    if (!move.sourceAuthority) continue;
    try {
      for (const input of move.sourceAuthority.inputs) {
        const frozen = { path: input.path, ...input.preimage };
        const current = resumeGeneratorInputRecord(repoRoot, inputAuthority, journal, frozen, taxonomy);
        if (canonicalJson(current) !== canonicalJson(frozen)) throw new Error(`resume-state-drift: move source authority input ${move.operationId} ${input.role} ${input.path}`);
      }
    } catch (cause) { if (isTransactionRepositoryAuthorityError(cause)) throw cause; throw new TaxonomyMoveSourceInputDriftError(move.operationId, cause); }
  }
}

/** 📥️ Requires the complete current logical generator read set before any forward continuation. */
function validateForwardGeneratorInputs(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): void {
  const inputAuthority = resumeGeneratorInputAuthority(plan, journal);
  for (const regeneration of plan.regenerations) {
    try {
      const inputs = regeneration.inputs.map((input) => resumeGeneratorInputRecord(repoRoot, inputAuthority, journal, input, taxonomy));
      if (canonicalJson(inputs) !== canonicalJson(regeneration.inputs)) throw new Error(`resume-state-drift: regeneration inputs ${regeneration.id}`);
      const contract = requireTaxonomyValue(taxonomy.schema.generatorContracts[regeneration.contractId],"taxonomy.schema.generatorContracts[regeneration.contractId]");
      const view = resumeGeneratorInputView(repoRoot, plan, regeneration, inputAuthority, journal, taxonomy);
      const paths = generatorInputPaths({ repoRoot }, contract, taxonomy, undefined, view);
      if (canonicalJson(paths) !== canonicalJson(regeneration.inputs.map(input => input.path))) {
        const expected = new Set(regeneration.inputs.map(input => input.path)), actual = new Set(paths);
        const missing = [...expected].filter(path => !actual.has(path)), added = paths.filter(path => !expected.has(path));
        throw new Error(`resume-state-drift: regeneration input membership ${regeneration.id}; missing(${missing.length})=${JSON.stringify(missing.slice(0, 8))}; added(${added.length})=${JSON.stringify(added.slice(0, 8))}`);
      }
    } catch (cause) { if (isTransactionRepositoryAuthorityError(cause)) throw cause; throw new TaxonomyGeneratorInputDriftError(regeneration.id, cause); }
  }
}

export function validateResumeTuples(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): boolean {
  const reconciled = reconcileTransactionOwnedTuples(repoRoot, plan, journal, taxonomy);
  validateForwardMoveSourceInputs(repoRoot, plan, journal, taxonomy);
  validateForwardGeneratorInputs(repoRoot, plan, journal, taxonomy);
  return reconciled;
}

function assertStoredFileBackup(path: string, record: Extract<TaxonomyBackupRecord, { kind: "file" }>): void {
  const stat = lstatOrNull(path);
  if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(path) !== record.contentHash || (stat.mode & 0o7777) !== record.mode || stat.size !== record.size) throw new Error(`Stored transaction backup does not match its frozen preimage: ${path}`);
}

function transactionBinaryWritePreparations(container: string, preparationName: (pid: number, token: string) => string, candidateName: string): readonly TransactionBinaryWritePreparation[] {
  const preparations: TransactionBinaryWritePreparation[] = [];
  for (const name of readdirSync(container).sort(generatorPathCompare)) {
    const match = /^write-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match) continue;
    const pid = Number.parseInt(requireTaxonomyValue(match[1],"match[1]"), 10), token = requireTaxonomyValue(match[2],"match[2]");
    if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== preparationName(pid, token)) throw new Error(`Transaction binary write preparation name is invalid: ${name}`);
    const root = join(container, name), stat = lstatOrNull(root);
    if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction binary write preparation must be a no-follow directory: ${name}`);
    const children = readdirSync(root).sort(generatorPathCompare);
    if (children.length > 1 || children.length === 1 && children[0] !== candidateName) throw new Error(`Transaction binary write preparation contains unexpected evidence: ${name}`);
    const leaf = children.length === 1 ? join(root, candidateName) : undefined;
    const leafStat = leaf ? lstatOrNull(leaf) : undefined;
    if (leaf && (!leafStat?.isFile() || leafStat.isSymbolicLink())) throw new Error(`Transaction binary write candidate must be a regular no-follow file: ${leaf}`);
    preparations.push({ root, leaf });
  }
  if (preparations.length > 1) throw new Error(`Transaction binary writer has duplicate preparations: ${container}`);
  return preparations;
}

export function backupPath(repoRoot: string, logicalPath: string, backupRoot: string, journal: MutableJournalRecord, expected: TaxonomyBackupExpectedPreimage, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string): void {
  if (journal.backups[logicalPath] !== undefined) return;
  const source = absolutePath(repoRoot, logicalPath);
  const stat = lstatOrNull(source);
  if (!stat) throw new Error(`Backup source is absent: ${logicalPath}`);
  if (stat.isSymbolicLink()) {
    const target = readlinkSync(source);
    const actual: TaxonomyBackupExpectedPreimage = { nodeKind: "symlink", contentHash: sha256(target), mode: stat.mode & 0o7777, size: Buffer.byteLength(target), target };
    if (expected.nodeKind !== "symlink" || canonicalJson(actual) !== canonicalJson(expected)) throw new Error(`Backup source changed from its frozen symlink preimage: ${logicalPath}`);
    journal.backups[logicalPath] = expectedBackupRecord(logicalPath, expected);
    return;
  }
  if (!stat.isFile() || expected.nodeKind !== "file") throw new Error(`Backup target kind changed: ${logicalPath}`);
  const bytes = readFileSync(source);
  const record = expectedBackupRecord(logicalPath, expected);
  if (record.kind !== "file" || sha256(bytes) !== record.contentHash || bytes.byteLength !== record.size || (stat.mode & 0o7777) !== record.mode) throw new Error(`Backup source changed from its frozen file preimage: ${logicalPath}`);
  const token = randomUUID();
  const candidate = join(backupRoot, preparationName(record.backupPath.slice(0, 24), process.pid, token));
  const candidateLeaf = join(candidate, record.backupPath);
  const destination = join(backupRoot, record.backupPath);
  mkdirSync(candidate);
  fsyncDirectory(backupRoot);
  const writer = writeTransactionBinaryCandidate(candidate, writePreparationName, writeCandidateName, bytes, record.mode, journal.probe, "transaction-backup", logicalPath);
  assertStoredFileBackup(writer.leaf!, record);
  journal.probe?.("transaction-backup-write-prepared", logicalPath);
  const sourceAfter = leafPreimage(source);
  if (sourceAfter.nodeKind !== "file" || sourceAfter.contentHash !== record.contentHash || sourceAfter.mode !== record.mode || sourceAfter.size !== record.size) throw new Error(`Backup source changed during frozen snapshot publication: ${logicalPath}`);
  durableRename(writer.leaf!, candidateLeaf);
  journal.probe?.("transaction-backup-inner-exchange", logicalPath);
  durableRemove(writer.root, true);
  journal.probe?.("transaction-backup-exchange", logicalPath);
  const published = lstatOrNull(destination);
  if (published) assertStoredFileBackup(destination, record);
  else {
    try { linkSync(candidateLeaf, destination); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      assertStoredFileBackup(destination, record);
    }
    fsyncDirectory(backupRoot);
  }
  journal.probe?.("transaction-backup-retained", logicalPath);
  durableRemove(candidate, true);
  journal.backups[logicalPath] = record;
}

export function recoverTransactionBackups(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string, validateOnly = false): boolean {
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const authorities = transactionBackupAuthorities(plan);
  let changed = false;
  const rootChildren = readdirSync(backupRoot).sort(generatorPathCompare);
  const recordForStored = (identity: string, path: string): Extract<TaxonomyBackupRecord, { kind: "file" }> => {
    const authority = authorities.get(identity);
    if (!authority || authority.expected.nodeKind !== "file") throw new Error(`Transaction backup evidence has no unique frozen file authority: ${basename(path)}`);
    const stat = lstatOrNull(path);
    const record: Extract<TaxonomyBackupRecord, { kind: "file" }> = { kind: "file", backupPath: `${identity}.backup`, contentHash: authority.expected.contentHash, mode: authority.expected.mode, size: stat?.size ?? -1 };
    if (!stat?.isFile() || stat.isSymbolicLink() || authority.expected.size !== undefined && stat.size !== authority.expected.size) throw new Error(`Transaction backup evidence is not a regular exact-size file: ${path}`);
    assertStoredFileBackup(path, record);
    return record;
  };
  const candidates: { root: string; leaf?: string; writer?: TransactionBinaryWritePreparation; identity: string; record: Extract<TaxonomyBackupRecord, { kind: "file" }>; discard: boolean }[] = [];
  const records = new Map<string, Extract<TaxonomyBackupRecord, { kind: "file" }>>();
  for (const name of rootChildren) {
    const rest = splitLeadingEmoji(name).rest;
    const final = /^([0-9a-f]{24})\.backup$/u.exec(name);
    if (final) {
      const record = recordForStored(requireTaxonomyValue(final[1],"final[1]"), join(backupRoot, name));
      records.set(requireTaxonomyValue(final[1],"final[1]"), record);
      continue;
    }
    const match = /^backup-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(match[3],"match[3]"))) {
      if (rest.startsWith("restore-") || rest.startsWith("lease-")) continue;
      throw new Error(`Transaction backup root contains unauthorized evidence: ${name}`);
    }
    const identity = requireTaxonomyValue(match[1],"match[1]"), pid = Number.parseInt(requireTaxonomyValue(match[2],"match[2]"), 10), token = requireTaxonomyValue(match[3],"preparation token");
    if (name !== preparationName(identity, pid, token)) throw new Error(`Transaction backup preparation is not canonical: ${name}`);
    if (candidates.some((entry) => entry.identity === identity)) throw new Error(`Transaction backup root contains duplicate candidates for ${identity}`);
    const candidate = join(backupRoot, name);
    const candidateStat = lstatOrNull(candidate);
    if (!candidateStat?.isDirectory() || candidateStat.isSymbolicLink()) throw new Error(`Transaction backup preparation must be a no-follow directory: ${name}`);
    const expectedLeaf = `${identity}.backup`;
    const writers = transactionBinaryWritePreparations(candidate, writePreparationName, writeCandidateName);
    const writerNames = new Set(writers.map((entry) => basename(entry.root)));
    const candidateChildren = readdirSync(candidate).sort(generatorPathCompare);
    if (candidateChildren.some((child) => child !== expectedLeaf && !writerNames.has(child))) throw new Error(`Transaction backup preparation has unexpected evidence: ${name}`);
    const candidateLeaf = join(candidate, expectedLeaf);
    const authority = authorities.get(identity);
    if (!authority || authority.expected.nodeKind !== "file") throw new Error(`Transaction backup preparation has no frozen file authority: ${name}`);
    const record = expectedBackupRecord(authority.path, authority.expected);
    if (record.kind !== "file") throw new Error(`Transaction backup preparation kind is invalid: ${name}`);
    const outerPresent = Boolean(lstatOrNull(candidateLeaf));
    if (outerPresent) assertStoredFileBackup(candidateLeaf, record);
    const writer = writers[0];
    if (outerPresent && writer?.leaf) throw new Error(`Transaction backup preparation has duplicate outer and nested candidates: ${name}`);
    const writerExact = Boolean(writer?.leaf && (() => { try { assertStoredFileBackup(writer.leaf!, record); return true; } catch (error) { if (isTransactionRepositoryAuthorityError(error)) throw error; return false; } })());
    if (!outerPresent && !writerExact) {
      const current = absolutePath(repoRoot, authority.path), currentStat = lstatOrNull(current);
      const pre = currentStat?.isFile() && !currentStat.isSymbolicLink() && hashPath(current) === record.contentHash && (currentStat.mode & 0o7777) === record.mode && currentStat.size === record.size;
      if (!pre) throw new Error(`Incomplete transaction backup writer has no exact source preimage: ${name}`);
      candidates.push({ root: candidate, writer, identity, record, discard: true });
      continue;
    }
    const destination = join(backupRoot, expectedLeaf);
    if (lstatOrNull(destination)) {
      if (!outerPresent) throw new Error(`Published transaction backup has an unreachable preparation without its outer candidate: ${name}`);
      assertStoredFileBackup(destination, record);
      const finalRecord = records.get(identity) ?? recordForStored(identity, destination);
      if (canonicalJson(finalRecord) !== canonicalJson(record)) throw new Error(`Transaction backup candidate differs from its published leaf: ${identity}`);
      records.set(identity, finalRecord);
    } else records.set(identity, record);
    candidates.push({ root: candidate, leaf: outerPresent ? candidateLeaf : writer!.leaf, writer, identity, record, discard: false });
  }
  for (const [identity, record] of records) {
    const authority = authorities.get(identity);
    if (!authority) throw new Error(`Transaction backup leaf has no unique plan authority: ${identity}.backup`);
    const prior = journal.backups[authority.path];
    if (prior) {
      if (canonicalJson(prior) !== canonicalJson(record)) throw new Error(`Transaction backup journal evidence differs from its stored leaf: ${authority.path}`);
      continue;
    }
    if (!(authority.edit && journal.state === "editing") && !(authority.regenerationIds.length > 0 && journal.state === "regenerating")) throw new Error(`Transaction backup orphan is unreachable in journal state ${journal.state}: ${authority.path}`);
    const current = absolutePath(repoRoot, authority.path);
    const currentStat = lstatOrNull(current);
    const pre = currentStat?.isFile() && !currentStat.isSymbolicLink() && hashPath(current) === record.contentHash && (currentStat.mode & 0o7777) === record.mode && currentStat.size === record.size;
    if (!pre) throw new Error(`Transaction backup orphan source is not its frozen preimage: ${authority.path}`);
  }
  if (!validateOnly) for (const candidate of candidates) {
      if (candidate.discard) {
        durableRemove(candidate.root, true);
        continue;
      }
      const outerLeaf = join(candidate.root, candidate.record.backupPath);
      if (!lstatOrNull(outerLeaf)) durableRename(candidate.leaf!, outerLeaf);
      if (candidate.writer && lstatOrNull(candidate.writer.root)) durableRemove(candidate.writer.root, true);
      const destination = join(backupRoot, candidate.record.backupPath);
      if (!lstatOrNull(destination)) {
        try { linkSync(outerLeaf, destination); }
        catch (error) {
          if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
          assertStoredFileBackup(destination, candidate.record);
        }
        fsyncDirectory(backupRoot);
      }
      durableRemove(candidate.root, true);
    }
  for (const [identity, record] of records) {
    const authority = authorities.get(identity)!;
    if (!journal.backups[authority.path] && !validateOnly) {
      journal.backups[authority.path] = record;
      changed = true;
    } else if (!journal.backups[authority.path]) changed = true;
  }
  for (const regeneration of plan.regenerations) {
    const outputs = regeneration.preOutputs.filter((entry) => entry.nodeKind !== "directory");
    const hasOrphan = outputs.some((entry) => journal.backups[entry.path] !== undefined);
    if (!hasOrphan || journal.startedRegenerationIds.includes(regeneration.id)) continue;
    if (journal.state !== "regenerating") throw new Error(`Transaction generator backup orphan is outside its regenerating phase: ${regeneration.id}`);
    if (!validateOnly) {
      for (const output of outputs) backupPath(repoRoot, output.path, backupRoot, journal, output, preparationName, writePreparationName, writeCandidateName);
      journal.startedRegenerationIds.push(regeneration.id);
    }
    changed = true;
  }
  return changed;
}

export function restoreBackup(repoRoot: string, plan: TaxonomyPlan, logicalPath: string, backupRoot: string, encoded: TaxonomyBackupRecord, preparationName: (identity: string, pid: number, token: string) => string, probe?: (phase: string, path?: string) => void): void {
  const destination = absolutePath(repoRoot, logicalPath);
  mkdirSync(dirname(destination), { recursive: true });
  const current = lstatOrNull(destination);
  if (encoded.kind === "absent") {
    if (current?.isDirectory()) throw new Error(`Cannot remove directory while restoring absent backup: ${logicalPath}`);
    if (current) durableRemove(destination);
    return;
  }
  if (current?.isDirectory()) throw new Error(`Cannot replace directory while restoring backup: ${logicalPath}`);
  const identity = sha256(logicalPath).slice(0, 24), token = randomUUID();
  const candidateRoot = join(backupRoot, preparationName(identity, process.pid, token));
  const candidateLeaf = join(candidateRoot, `${identity}.backup`);
  const postLeaf = join(candidateRoot, `${identity}.post`);
  mkdirSync(candidateRoot);
  fsyncDirectory(backupRoot);
  probe?.("transaction-restore-mkdir", logicalPath);
  if (encoded.kind === "symlink") {
    if (encoded.targetHash !== sha256(encoded.target)) throw new Error(`Symlink backup target hash changed: ${logicalPath}`);
    symlinkSync(encoded.target, candidateLeaf);
    fsyncDirectory(candidateRoot);
  } else {
    const source = join(backupRoot, encoded.backupPath);
    assertStoredFileBackup(source, encoded);
    copyFileSync(source, candidateLeaf);
    chmodSync(candidateLeaf, encoded.mode);
    fsyncFile(candidateLeaf);
    fsyncDirectory(candidateRoot);
  }
  probe?.("transaction-restore-prepared", logicalPath);
  const candidatePreimage = leafPreimage(candidateLeaf);
  const expected = encoded.kind === "symlink" ? { nodeKind: "symlink", contentHash: encoded.targetHash, mode: encoded.mode, size: encoded.size, target: encoded.target } as TaxonomyLeafPreimage : { nodeKind: "file", contentHash: encoded.contentHash, mode: encoded.mode, size: encoded.size } as TaxonomyLeafPreimage;
  if (canonicalJson(candidatePreimage) !== canonicalJson(expected)) throw new Error(`Restore candidate does not match its typed backup: ${logicalPath}`);
  if (current && canonicalJson(leafPreimage(destination)) === canonicalJson(expected)) { durableRemove(candidateRoot, true); return; }
  if (current) {
    const edits = plan.edits.filter((entry) => entry.path === logicalPath);
    const transactionOwned = edits.length > 0 && encoded.kind === "file" && (() => {
      const rendered = applyEditsToContent(readFileSync(join(backupRoot, encoded.backupPath), "utf8"), edits);
      return current.isFile() && !current.isSymbolicLink() && readFileSync(destination, "utf8") === rendered && (current.mode & 0o7777) === encoded.mode;
    })();
    if (!transactionOwned) throw new Error(`Restore destination is not an exact transaction-owned postimage: ${logicalPath}`);
    durableRename(destination, postLeaf);
    probe?.("transaction-restore-exchange", logicalPath);
  }
  durableRename(candidateLeaf, destination);
  probe?.("transaction-restore-canonical-exchange", logicalPath);
  if (lstatOrNull(postLeaf)) durableRemove(postLeaf);
  durableRemove(candidateRoot, true);
}

export function recoverRestorePreparations(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, preparationName: (identity: string, pid: number, token: string) => string, validateOnly = false): void {
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const byIdentity = new Map<string, { path: string; backup: TaxonomyBackupRecord }>();
  for (const [path, backup] of Object.entries(journal.backups)) {
    const identity = sha256(path).slice(0, 24);
    if (byIdentity.has(identity)) throw new Error(`Restore preparation identity is ambiguous: ${identity}`);
    byIdentity.set(identity, { path, backup });
  }
  const actions: { root: string; backupLeaf?: string; postLeaf?: string; destination: string; destinationPre: boolean; destinationPost: boolean }[] = [];
  for (const name of readdirSync(backupRoot).sort(generatorPathCompare)) {
    const rest = splitLeadingEmoji(name).rest;
    if (!rest.startsWith("restore-")) continue;
    const match = /^restore-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(match[3],"match[3]")) || name !== preparationName(requireTaxonomyValue(match[1],"match[1]"), Number.parseInt(requireTaxonomyValue(match[2],"match[2]"), 10), requireTaxonomyValue(match[3],"preparation token"))) throw new Error(`Restore preparation name is invalid: ${name}`);
    const authority = byIdentity.get(requireTaxonomyValue(match[1],"match[1]"));
    if (!authority || journal.state !== "rolling-back") throw new Error(`Restore preparation has no rolling-back journal authority: ${name}`);
    const root = join(backupRoot, name), leaf = join(root, `${match[1]}.backup`), postLeaf = join(root, `${match[1]}.post`), destination = absolutePath(repoRoot, authority.path);
    const stat = lstatOrNull(root);
    const children = stat?.isDirectory() && !stat.isSymbolicLink() ? readdirSync(root).sort(generatorPathCompare) : [];
    if (!stat?.isDirectory() || stat.isSymbolicLink() || children.some((child) => child !== basename(leaf) && child !== basename(postLeaf)) || new Set(children).size !== children.length) throw new Error(`Restore preparation has incomplete or unexpected evidence: ${name}`);
    const backupPresent = children.includes(basename(leaf)), postPresent = children.includes(basename(postLeaf));
    const candidate = backupPresent ? leafPreimage(leaf) : undefined;
    const expected = authority.backup.kind === "file" ? { nodeKind: "file", contentHash: authority.backup.contentHash, mode: authority.backup.mode, size: authority.backup.size } as TaxonomyLeafPreimage : authority.backup.kind === "symlink" ? { nodeKind: "symlink", contentHash: authority.backup.targetHash, mode: authority.backup.mode, size: authority.backup.size, target: authority.backup.target } as TaxonomyLeafPreimage : undefined;
    if (!expected || candidate && canonicalJson(candidate) !== canonicalJson(expected)) throw new Error(`Restore preparation bytes differ from journal backup authority: ${name}`);
    const currentStat = lstatOrNull(destination);
    const restored = Boolean(currentStat && !currentStat.isDirectory() && canonicalJson(leafPreimage(destination)) === canonicalJson(expected));
    let transactionOwned = false;
    const edits = plan.edits.filter((entry) => entry.path === authority.path);
    if (edits.length > 0 && currentStat?.isFile() && !currentStat.isSymbolicLink() && authority.backup.kind === "file") {
      const rendered = applyEditsToContent(readFileSync(join(backupRoot, authority.backup.backupPath), "utf8"), edits);
      transactionOwned = readFileSync(destination, "utf8") === rendered && (currentStat.mode & 0o7777) === authority.backup.mode;
    }
    if (postPresent) {
      if (edits.length === 0 || authority.backup.kind !== "file") throw new Error(`Restore postimage has no exact edit authority: ${name}`);
      const rendered = applyEditsToContent(readFileSync(join(backupRoot, authority.backup.backupPath), "utf8"), edits), post = lstatOrNull(postLeaf);
      if (!post?.isFile() || post.isSymbolicLink() || readFileSync(postLeaf, "utf8") !== rendered || (post.mode & 0o7777) !== authority.backup.mode) throw new Error(`Restore postimage differs from transaction output: ${name}`);
    }
    const startedGeneratorAbsent = !currentStat && plan.regenerations.some((entry) => journal.startedRegenerationIds.includes(entry.id) && entry.preOutputs.some((output) => output.path === authority.path));
    const stateValid = !backupPresent && !postPresent ? transactionOwned || restored || startedGeneratorAbsent
      : backupPresent && postPresent ? !currentStat
        : backupPresent ? transactionOwned || !currentStat || restored
          : restored;
    if (!stateValid) throw new Error(`Restore preparation has an impossible exchange tuple: ${name}`);
    actions.push({ root, backupLeaf: backupPresent ? leaf : undefined, postLeaf: postPresent ? postLeaf : undefined, destination, destinationPre: restored, destinationPost: transactionOwned });
  }
  if (!validateOnly) for (const action of actions) {
    if (action.backupLeaf && !action.destinationPre) {
      if (action.destinationPost && !action.postLeaf) {
        const post = join(action.root, basename(action.backupLeaf).replace(/\.backup$/u, ".post"));
        durableRename(action.destination, post);
        action.postLeaf = post;
      }
      mkdirSync(dirname(action.destination), { recursive: true });
      durableRename(action.backupLeaf, action.destination);
    }
    if (action.postLeaf && lstatOrNull(action.postLeaf)) durableRemove(action.postLeaf);
    durableRemove(action.root, true);
  }
}

export function referenceEditResult(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, path: string): Readonly<{ bytes: Buffer; preimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }> }> {
  const edits = plan.edits.filter((entry) => entry.path === path);
  const preimages = new Map(edits.map((entry) => [canonicalJson(entry.preimage), entry.preimage]));
  const preimage = [...preimages.values()][0];
  const backup = journal.backups[path];
  if (preimages.size !== 1 || !preimage || backup?.kind !== "file" || canonicalJson(backup) !== canonicalJson(expectedBackupRecord(path, preimage))) throw new Error(`Reference edit lacks one exact frozen preimage and backup: ${path}`);
  return { bytes: Buffer.from(applyEditsToContent(readFileSync(join(absolutePath(repoRoot, journal.backupRoot), backup.backupPath), "utf8"), edits)), preimage };
}

export function applyReferenceEditAtomically(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, path: string, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string): void {
  const target = absolutePath(repoRoot, path), identity = sha256(path).slice(0, 24), result = referenceEditResult(repoRoot, plan, journal, path);
  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit preimage changed: ${path}`);
  const root = join(absolutePath(repoRoot, journal.stagingRoot), preparationName(identity, process.pid, randomUUID()));
  const leaf = join(root, `${identity}.edit`);
  const preLeaf = join(root, `${identity}.pre`);
  mkdirSync(root);
  fsyncDirectory(dirname(root));
  const writer = writeTransactionBinaryCandidate(root, writePreparationName, writeCandidateName, result.bytes, result.preimage.mode, journal.probe, "transaction-edit", path);
  const candidate = leafPreimage(writer.leaf!);
  if (candidate.nodeKind !== "file" || candidate.contentHash !== sha256(result.bytes) || candidate.mode !== result.preimage.mode || candidate.size !== result.bytes.byteLength) throw new Error(`Reference edit candidate differs from rendered bytes: ${path}`);
  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit source changed during candidate publication: ${path}`);
  journal.probe?.("transaction-edit-write-prepared", path);
  durableRename(writer.leaf!, leaf);
  journal.probe?.("transaction-edit-inner-exchange", path);
  durableRemove(writer.root, true);
  durableRename(target, preLeaf);
  journal.probe?.("transaction-edit-exchange", path);
  durableRename(leaf, target);
  journal.probe?.("transaction-edit-canonical-exchange", path);
  durableRemove(preLeaf);
  durableRemove(root, true);
}

export function recoverReferenceEditPreparations(repoRoot: string, plan: TaxonomyPlan, journal: MutableJournalRecord, preparationName: (identity: string, pid: number, token: string) => string, writePreparationName: (pid: number, token: string) => string, writeCandidateName: string, validateOnly = false): void {
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const editPaths = new Map<string, string>();
  for (const path of new Set(plan.edits.map((entry) => entry.path))) {
    const identity = sha256(path).slice(0, 24);
    if (editPaths.has(identity)) throw new Error(`Reference edit preparation identity collision: ${identity}`);
    editPaths.set(identity, path);
  }
  const actions: { root: string; leaf?: string; preLeaf?: string; writer?: TransactionBinaryWritePreparation; publishWriter: boolean; target: string; targetPre: boolean; targetPost: boolean; discard: boolean }[] = [];
  for (const name of readdirSync(stageRoot).sort(generatorPathCompare)) {
    const rest = splitLeadingEmoji(name).rest;
    if (!rest.startsWith("edit-")) continue;
    const match = /^edit-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(match[3],"match[3]")) || name !== preparationName(requireTaxonomyValue(match[1],"match[1]"), Number.parseInt(requireTaxonomyValue(match[2],"match[2]"), 10), requireTaxonomyValue(match[3],"preparation token")) || journal.state !== "editing") throw new Error(`Reference edit preparation is invalid or unreachable: ${name}`);
    const path = editPaths.get(requireTaxonomyValue(match[1],"match[1]"));
    if (!path || !journal.backups[path]) throw new Error(`Reference edit preparation has no frozen plan backup: ${name}`);
    const root = join(stageRoot, name), target = absolutePath(repoRoot, path), rootStat = lstatOrNull(root), expectedLeaf = `${match[1]}.edit`, expectedPreLeaf = `${match[1]}.pre`;
    if (!rootStat?.isDirectory() || rootStat.isSymbolicLink()) throw new Error(`Reference edit preparation must be a no-follow directory: ${name}`);
    const writers = transactionBinaryWritePreparations(root, writePreparationName, writeCandidateName);
    const writerNames = new Set(writers.map((entry) => basename(entry.root)));
    const children = readdirSync(root).sort(generatorPathCompare);
    if (children.some((child) => child !== expectedLeaf && child !== expectedPreLeaf && !writerNames.has(child)) || children.length > 3) throw new Error(`Reference edit preparation contains unexpected evidence: ${name}`);
    const result = referenceEditResult(repoRoot, plan, journal, path), targetStat = lstatOrNull(target);
    const pre = targetStat?.isFile() && !targetStat.isSymbolicLink() && canonicalJson(leafPreimage(target)) === canonicalJson(result.preimage);
    const post = targetStat?.isFile() && !targetStat.isSymbolicLink() && sha256(readFileSync(target)) === sha256(result.bytes) && (targetStat.mode & 0o7777) === result.preimage.mode && targetStat.size === result.bytes.byteLength;
    const outerLeaf = children.includes(expectedLeaf) ? join(root, expectedLeaf) : undefined;
    const preLeaf = children.includes(expectedPreLeaf) ? join(root, expectedPreLeaf) : undefined;
    if (preLeaf && canonicalJson(leafPreimage(preLeaf)) !== canonicalJson(result.preimage)) throw new Error(`Reference edit preimage exchange bytes are forged: ${name}`);
    const exactCandidate = (candidatePath: string): boolean => {
      const candidate = leafPreimage(candidatePath);
      return candidate.nodeKind === "file" && candidate.contentHash === sha256(result.bytes) && candidate.mode === result.preimage.mode && candidate.size === result.bytes.byteLength;
    };
    if (outerLeaf && !exactCandidate(outerLeaf)) throw new Error(`Reference edit preparation bytes are forged: ${name}`);
    const writer = writers[0], writerExact = Boolean(writer?.leaf && exactCandidate(writer.leaf));
    if (outerLeaf && writer?.leaf) throw new Error(`Reference edit preparation has duplicate outer and nested candidates: ${name}`);
    if (writer && (preLeaf || post)) throw new Error(`Reference edit writer coexists with a target exchange tuple: ${name}`);
    const leaf = outerLeaf ?? (writerExact ? writer!.leaf : undefined);
    const discard = Boolean(writer && !outerLeaf && !writerExact);
    if (discard && (!pre || preLeaf)) throw new Error(`Incomplete reference edit writer has an unreachable target tuple: ${name}`);
    const stateValid = !leaf && !preLeaf ? pre || post
      : leaf && preLeaf ? !targetStat
        : leaf ? pre
          : post;
    if (!stateValid) throw new Error(`Reference edit preparation has an impossible target tuple: ${name}`);
    actions.push({ root, leaf, preLeaf, writer, publishWriter: Boolean(!outerLeaf && writerExact), target, targetPre: Boolean(pre), targetPost: Boolean(post), discard });
  }
  if (!validateOnly) for (const action of actions) {
    if (action.discard) {
      durableRemove(action.root, true);
      continue;
    }
    if (action.publishWriter) {
      const outerLeaf = join(action.root, `${sha256(normalizeRelative(relative(repoRoot, action.target).replaceAll("\\", "/"))).slice(0, 24)}.edit`);
      durableRename(action.leaf!, outerLeaf);
      action.leaf = outerLeaf;
    }
    if (action.writer && lstatOrNull(action.writer.root)) durableRemove(action.writer.root, true);
    if (!action.discard && action.leaf && !action.targetPost) {
      if (action.targetPre && !action.preLeaf) {
        const preLeaf = join(action.root, basename(action.leaf).replace(/\.edit$/u, ".pre"));
        durableRename(action.target, preLeaf);
        action.preLeaf = preLeaf;
      }
      durableRename(action.leaf, action.target);
    }
    if (action.preLeaf && lstatOrNull(action.preLeaf)) durableRemove(action.preLeaf);
    durableRemove(action.root, true);
  }
}

export function assertRecoveryRootNames(
  repoRoot: string,
  plan: TaxonomyPlan,
  journal: MutableJournalRecord,
  backupPreparationName: (identity: string, pid: number, token: string) => string,
  restorePreparationName: (identity: string, pid: number, token: string) => string,
  editPreparationName: (identity: string, pid: number, token: string) => string,
  leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string,
): void {
  const stageKnown = new Set([
    journal.journalWriteDirectory,
    ...plan.moves.map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
  ]);
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot), backupRoot = absolutePath(repoRoot, journal.backupRoot);
  for (const name of (lstatOrNull(stagingRoot) ? readdirSync(stagingRoot) : []).sort(generatorPathCompare)) {
    if (stageKnown.has(name)) continue;
    const match = /^edit-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(match[3],"match[3]")) || name !== editPreparationName(requireTaxonomyValue(match[1],"match[1]"), Number.parseInt(requireTaxonomyValue(match[2],"match[2]"), 10), requireTaxonomyValue(match[3],"preparation token"))) throw new Error(`Transaction recovery staging root contains unauthorized evidence: ${name}`);
  }
  for (const name of (lstatOrNull(backupRoot) ? readdirSync(backupRoot) : []).sort(generatorPathCompare)) {
    if (/^[0-9a-f]{24}\.backup$/u.test(name)) continue;
    const rest = splitLeadingEmoji(name).rest;
    const leaf = /^(backup|restore)-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (leaf && TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(leaf[4],"leaf[4]"))) {
      const expected = leaf[1] === "backup" ? backupPreparationName(requireTaxonomyValue(leaf[2],"leaf[2]"), Number.parseInt(requireTaxonomyValue(leaf[3],"leaf[3]"), 10), requireTaxonomyValue(leaf[4],"preparation token")) : restorePreparationName(requireTaxonomyValue(leaf[2],"leaf[2]"), Number.parseInt(requireTaxonomyValue(leaf[3],"leaf[3]"), 10), requireTaxonomyValue(leaf[4],"preparation token"));
      if (name === expected) continue;
    }
    const lease = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(rest);
    if (lease && TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(lease[2],"lease[2]")) && name === leasePreparationName(Number.parseInt(requireTaxonomyValue(lease[1],"lease[1]"), 10), requireTaxonomyValue(lease[2],"lease[2]"), lease[3] as "preparing" | "stale")) continue;
    throw new Error(`Transaction recovery backup root contains unauthorized evidence: ${name}`);
  }
}
