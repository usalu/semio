import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { sha256 } from "../📁️input/🟦️.ts";
import { randomUUID } from "node:crypto";
import { chmodSync, closeSync, fsyncSync, mkdirSync, openSync, writeSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { registryCatalogInputView } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type RegistryCatalogInputView } from "../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
import { join, posix } from "node:path";
import type { TaxonomyBackupRecord, TaxonomyJournalState, TaxonomyPlan, TaxonomyRegeneration } from "../📜️plan-contracts/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, fsyncDirectory, fsyncFile } from "../💾️durable-journal/🟦️.ts";

export class TaxonomyStartedRegenerationPartialError extends Error {
  readonly regenerationId: string;
  constructor(regenerationId: string) {
    super(`resume-state-drift: started regeneration has a transaction-owned partial output tree: ${regenerationId}`);
    this.regenerationId = regenerationId;
  }
}

export class TaxonomyGeneratorInputDriftError extends Error {
  readonly regenerationId: string;
  constructor(regenerationId: string, cause: unknown) {
    super(cause instanceof Error ? cause.message : String(cause), { cause });
    this.regenerationId = regenerationId;
  }
}

export class TaxonomyMoveSourceInputDriftError extends Error {
  readonly operationId: string;
  constructor(operationId: string, cause: unknown) {
    super(cause instanceof Error ? cause.message : String(cause), { cause });
    this.operationId = operationId;
  }
}

export function resumeGeneratorInputAuthority(plan: TaxonomyPlan, journal: MutableJournalRecord) {
  return {
    moves: new Map(plan.moves.map((entry) => [entry.sourcePath, entry])),
    relocations: new Map(plan.embeddedTicketRootRelocations.map((entry) => [entry.sourcePath, entry])),
    removals: new Map(plan.evidenceRemovals.map((entry) => [entry.sourcePath, entry])),
    roots: plan.embeddedTicketRoots,
    links: new Map(plan.symlinkTargetEdits.filter((entry) => journal.stagedSymlinkTargetEditIds.includes(entry.operationId)).map((entry) => [entry.sourcePath, entry])),
    editPaths: new Set(plan.edits.map((entry) => entry.path)),
  };
}

/** 🧭️ Resolves one logical input to its journal-proven preimage location. */
export function resumeGeneratorInputPhysicalPath(authority: ReturnType<typeof resumeGeneratorInputAuthority>, journal: MutableJournalRecord, path: string): string {
  const move = authority.moves.get(path);
  const relocation = authority.relocations.get(path);
  const removal = authority.removals.get(path);
  const root = authority.roots.find((entry) => path === entry.sourceMetadataRoot || path.startsWith(`${entry.sourceMetadataRoot}/`));
  const finalPath = move?.destinationPath ?? relocation?.destinationPath ?? path;
  let current = path;
  if (move) current = journal.installedMoveIds.includes(move.operationId) ? move.destinationPath : journal.stagedMoveIds.includes(move.operationId) ? `${journal.stagingRoot}/${move.operationId}` : move.sourcePath;
  else if (relocation) current = journal.installedEmbeddedRelocationIds.includes(relocation.operationId) ? relocation.destinationPath : journal.stagedEmbeddedRelocationIds.includes(relocation.operationId) ? `${journal.stagingRoot}/relocation-${relocation.operationId}` : relocation.sourcePath;
  else if (removal && journal.stagedEvidenceRemovalIds.includes(removal.operationId)) current = `${journal.stagingRoot}/removal-${removal.operationId}`;
  else if (root && journal.stagedEmbeddedRootIds.includes(root.operationId)) current = `${journal.stagingRoot}/root-${root.operationId}${path.slice(root.sourceMetadataRoot.length)}`;
  const link = authority.links.get(path);
  if (link) current = `${journal.stagingRoot}/symlink-${link.operationId}`;
  const backup = journal.backups[finalPath];
  if (current === finalPath && backup?.kind === "file" && authority.editPaths.has(finalPath)) current = `${journal.backupRoot}/${backup.backupPath}`;
  return current;
}

/** 🌳️ Reconstructs logical input membership without hiding any unplanned physical child. */
export function resumeGeneratorInputView(repoRoot: string, plan: TaxonomyPlan, regeneration: TaxonomyRegeneration, authority: ReturnType<typeof resumeGeneratorInputAuthority>, journal: MutableJournalRecord, taxonomy: LoadedTaxonomy): RegistryCatalogInputView {
  const native = registryCatalogInputView(repoRoot, taxonomy.discoverySchema);
  const sources = new Set([...authority.moves.keys(), ...authority.relocations.keys(), ...authority.removals.keys(), ...authority.links.keys(), ...authority.roots.map(entry => entry.sourceMetadataRoot)]);
  const children = new Map<string, Set<string>>();
  for (const source of sources) {
    const parent = posix.dirname(source) === "." ? "" : posix.dirname(source);
    const names = children.get(parent) ?? new Set<string>();
    names.add(posix.basename(source));
    children.set(parent, names);
  }
  const installed = new Set([
    ...plan.moves.filter(entry => journal.installedMoveIds.includes(entry.operationId)).map(entry => entry.destinationPath),
    ...plan.embeddedTicketRootRelocations.filter(entry => journal.installedEmbeddedRelocationIds.includes(entry.operationId)).map(entry => entry.destinationPath),
  ]);
  const created = new Set(plan.destinationAncestorPreimages.filter(entry => entry.state === "absent").map(entry => entry.path));
  const hiddenCache = new Map<string, boolean>();
  const pathFor = (path: string, name: string): string => path ? `${path}/${name}` : name;
  const hidden = (path: string): boolean => {
    if (installed.has(path) || regeneration.outputRoots.some(root => path === root || path.startsWith(`${root}/`))) return true;
    if (!created.has(path)) return false;
    if (hiddenCache.has(path)) return hiddenCache.get(path)!;
    const nodeKind = native.kind(path);
    const invisible = nodeKind === null || nodeKind === "directory" && native.entries(path).every(entry => hidden(pathFor(path, entry.name)));
    hiddenCache.set(path, invisible);
    return invisible;
  };
  const kind = (path: string): ReturnType<RegistryCatalogInputView["kind"]> => {
    const physical = resumeGeneratorInputPhysicalPath(authority, journal, path);
    if (!sources.has(path) && hidden(path)) return null;
    return native.kind(physical);
  };
  return {
    kind,
    entries(path) {
      const parentKind = kind(path);
      if (parentKind === null) return [];
      if (parentKind !== "directory") throw new Error(`Generator input directory changed kind: ${path}`);
      const entries = new Map(native.entries(resumeGeneratorInputPhysicalPath(authority, journal, path)).map(entry => [entry.name, entry.nodeKind]));
      const names = new Set([...entries.keys(), ...(children.get(path) ?? [])]);
      return [...names].sort(generatorPathCompare).flatMap(name => {
        const child = pathFor(path, name);
        if (!sources.has(child) && hidden(child)) return [];
        const nodeKind = sources.has(child) ? kind(child) : entries.get(name) ?? kind(child);
        return nodeKind === null ? [] : [{ name, nodeKind }];
      });
    },
    readText(path) { return native.readText(resumeGeneratorInputPhysicalPath(authority, journal, path)); },
    readBytes(path) { return native.readBytes(resumeGeneratorInputPhysicalPath(authority, journal, path)); },
  };
}

export function assertJournalPhaseMembership(plan: TaxonomyPlan, journal: MutableJournalRecord): void {
  if (journal.state === "rolling-back" || journal.state === "rolled-back") return;
  const rank: Record<TaxonomyJournalState, number> = { prepared: 0, staging: 1, disposing: 2, installing: 3, retargeting: 4, editing: 5, regenerating: 6, verifying: 7, committed: 8, "rolling-back": -1, "rolled-back": -1 };
  const exact = (actual: readonly string[], expected: readonly string[], label: string): void => {
    const sorted = [...expected].sort(generatorPathCompare);
    if (canonicalJson(actual) !== canonicalJson(sorted)) throw new Error(`Resume journal ${label} is incomplete for state ${journal.state}`);
  };
  const phase = rank[journal.state];
  if (phase >= 2) {
    exact(journal.stagedMoveIds, plan.moves.map((entry) => entry.operationId), "staged moves");
    exact(journal.stagedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations.map((entry) => entry.operationId), "staged embedded relocations");
    exact(journal.stagedEvidenceRemovalIds, plan.evidenceRemovals.map((entry) => entry.operationId), "staged evidence removals");
  }
  if (phase >= 3) exact(journal.stagedEmbeddedRootIds, plan.embeddedTicketRoots.map((entry) => entry.operationId), "staged embedded roots");
  if (phase >= 4) {
    exact(journal.installedMoveIds, plan.moves.map((entry) => entry.operationId), "installed moves");
    exact(journal.installedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations.map((entry) => entry.operationId), "installed embedded relocations");
  }
  if (phase >= 5) exact(journal.installedSymlinkTargetEditIds, plan.symlinkTargetEdits.map((entry) => entry.operationId), "installed symlink target edits");
  if (phase >= 6) exact(journal.appliedEditPaths, [...new Set(plan.edits.map((entry) => entry.path))], "applied edit paths");
  if (phase >= 7) exact(journal.completedRegenerationIds, plan.regenerations.map((entry) => entry.id), "completed regenerations");
}

export function assertJournalPlanMembership(plan: TaxonomyPlan, journal: MutableJournalRecord): void {
  if (journal.sourceParentPrunePaths.some((path) => !plan.moves.some((move) => move.sourcePath.startsWith(`${path}/`)) || plan.moves.some((move) => move.destinationPath.startsWith(`${path}/`)))) throw new Error("Resume journal source-parent pruning does not match the plan");
  const subset = (child: readonly string[], parent: readonly string[]): boolean => child.every((id) => parent.includes(id));
  const exactPlanIds = (ids: readonly string[], records: readonly { readonly operationId: string }[]): boolean => ids.every((id) => records.some((record) => record.operationId === id));
  if (!subset(journal.stagedMoveIds, journal.preparedMoveIds) || !subset(journal.installedMoveIds, journal.stagedMoveIds) || !exactPlanIds(journal.preparedMoveIds, plan.moves) || !subset(journal.stagedEmbeddedRelocationIds, journal.preparedEmbeddedRelocationIds) || !subset(journal.installedEmbeddedRelocationIds, journal.stagedEmbeddedRelocationIds) || !exactPlanIds(journal.preparedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations) || !subset(journal.stagedEvidenceRemovalIds, journal.preparedEvidenceRemovalIds) || !exactPlanIds(journal.preparedEvidenceRemovalIds, plan.evidenceRemovals) || !subset(journal.stagedEmbeddedRootIds, journal.preparedEmbeddedRootIds) || !exactPlanIds(journal.preparedEmbeddedRootIds, plan.embeddedTicketRoots) || !subset(journal.stagedSymlinkTargetEditIds, journal.preparedSymlinkTargetEditIds) || !subset(journal.installedSymlinkTargetEditIds, journal.stagedSymlinkTargetEditIds) || !exactPlanIds(journal.preparedSymlinkTargetEditIds, plan.symlinkTargetEdits) || !subset(journal.completedRegenerationIds, journal.startedRegenerationIds) || journal.startedRegenerationIds.some((id) => !plan.regenerations.some((record) => record.id === id)) || journal.appliedEditPaths.some((path) => !plan.edits.some((edit) => edit.path === path))) throw new Error("Resume journal operation state does not match the plan");
}

export function assertJournalBackupAuthority(plan: TaxonomyPlan, journal: MutableJournalRecord): void {
  const editPaths = new Set(plan.edits.map((entry) => entry.path));
  const generatorPreimages = new Map(plan.regenerations.filter((entry) => journal.state === "rolled-back" || journal.startedRegenerationIds.includes(entry.id)).flatMap((entry) => entry.preOutputs.filter((output) => output.nodeKind !== "directory").map((output) => [output.path, output] as const)));
  const editBackupsAllowed = ["editing", "regenerating", "verifying", "committed", "rolling-back", "rolled-back"].includes(journal.state);
  const seenStored = new Set<string>();
  for (const [path, backup] of Object.entries(journal.backups)) {
    const generatorPreimage = generatorPreimages.get(path);
    if (!editPaths.has(path) && !generatorPreimage) throw new Error(`Resume journal has an unauthorized backup path: ${path}`);
    if (editPaths.has(path) && !editBackupsAllowed) throw new Error(`Resume journal contains a reference-edit backup before the editing phase: ${path}`);
    if (editPaths.has(path) && backup.kind !== "file") throw new Error(`Reference-edit backup must be a regular file: ${path}`);
    if (editPaths.has(path)) {
      const preimages = new Map(plan.edits.filter((entry) => entry.path === path).map((entry) => [canonicalJson(entry.preimage), entry.preimage]));
      const preimage = [...preimages.values()][0];
      if (preimages.size !== 1 || !preimage || backup.kind !== "file" || backup.contentHash !== preimage.contentHash || backup.mode !== preimage.mode || backup.size !== preimage.size) throw new Error(`Reference-edit backup does not match its frozen preimage: ${path}`);
    }
    if (generatorPreimage) {
      const matches = generatorPreimage.nodeKind === "file"
        ? backup.kind === "file" && backup.contentHash === generatorPreimage.contentHash && backup.mode === generatorPreimage.mode && backup.size === generatorPreimage.size
        : generatorPreimage.nodeKind === "symlink" ? backup.kind === "symlink" && backup.targetHash === generatorPreimage.contentHash && backup.mode === generatorPreimage.mode && backup.size === generatorPreimage.size && backup.target === generatorPreimage.target : false;
      if (!matches) throw new Error(`Generator backup does not match its frozen preOutput: ${path}`);
    }
    if (backup.kind !== "file") continue;
    const expected = `${sha256(path).slice(0, 24)}.backup`;
    if (backup.backupPath !== expected || seenStored.has(backup.backupPath)) throw new Error(`Resume journal backup storage identity is invalid: ${path}`);
    seenStored.add(backup.backupPath);
  }
  for (const path of journal.appliedEditPaths) if (!journal.backups[path]) throw new Error(`Applied reference edit has no frozen typed backup: ${path}`);
  for (const regeneration of plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id))) {
    for (const output of regeneration.preOutputs.filter((entry) => entry.nodeKind !== "directory")) if (!journal.backups[output.path]) throw new Error(`Started regeneration has no frozen typed backup: ${regeneration.id}:${output.path}`);
  }
}

export type TaxonomyBackupExpectedPreimage =
  | Readonly<{ nodeKind: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ nodeKind: "symlink"; contentHash: string; mode: number; size: number; target: string }>;

export function transactionBackupAuthorities(plan: TaxonomyPlan): ReadonlyMap<string, Readonly<{ path: string; expected: TaxonomyBackupExpectedPreimage; edit: boolean; regenerationIds: readonly string[] }>> {
  const byPath = new Map<string, { path: string; expected: TaxonomyBackupExpectedPreimage; edit: boolean; regenerationIds: string[] }>();
  const add = (path: string, expected: TaxonomyBackupExpectedPreimage, edit: boolean, regenerationId?: string): void => {
    const prior = byPath.get(path);
    if (prior) {
      if (canonicalJson(prior.expected) !== canonicalJson(expected)) throw new Error(`Transaction backup path has incompatible frozen authorities: ${path}`);
      prior.edit ||= edit;
      if (regenerationId && !prior.regenerationIds.includes(regenerationId)) prior.regenerationIds.push(regenerationId);
      return;
    }
    byPath.set(path, { path, expected, edit, regenerationIds: regenerationId ? [regenerationId] : [] });
  };
  for (const edit of plan.edits) add(edit.path, edit.preimage, true);
  for (const regeneration of plan.regenerations) for (const output of regeneration.preOutputs) if (output.nodeKind !== "directory") add(output.path, output, false, regeneration.id);
  const byIdentity = new Map<string, Readonly<{ path: string; expected: TaxonomyBackupExpectedPreimage; edit: boolean; regenerationIds: readonly string[] }>>();
  for (const authority of byPath.values()) {
    const identity = sha256(authority.path).slice(0, 24);
    const prior = byIdentity.get(identity);
    if (prior && prior.path !== authority.path) throw new Error(`Transaction backup storage identity collision: ${prior.path} <> ${authority.path}`);
    byIdentity.set(identity, authority);
  }
  return byIdentity;
}

export function expectedBackupRecord(path: string, expected: TaxonomyBackupExpectedPreimage): TaxonomyBackupRecord {
  if (expected.nodeKind === "symlink") {
    if (sha256(expected.target) !== expected.contentHash || Buffer.byteLength(expected.target) !== expected.size) throw new Error(`Frozen symlink backup preimage is incomplete: ${path}`);
    return { kind: "symlink", target: expected.target, targetHash: expected.contentHash, mode: expected.mode, size: expected.size };
  }
  return { kind: "file", backupPath: `${sha256(path).slice(0, 24)}.backup`, contentHash: expected.contentHash, mode: expected.mode, size: expected.size };
}

export interface TransactionBinaryWritePreparation {
  readonly root: string;
  readonly leaf?: string;
}

export function writeTransactionBinaryCandidate(container: string, preparationName: (pid: number, token: string) => string, candidateName: string, bytes: Buffer, mode: number, probe?: (phase: string, path?: string) => void, phasePrefix = "transaction-binary", logicalPath?: string): TransactionBinaryWritePreparation {
  const root = join(container, preparationName(process.pid, randomUUID()));
  mkdirSync(root);
  fsyncDirectory(container);
  probe?.(`${phasePrefix}-write-mkdir`, logicalPath ?? root);
  const leaf = join(root, candidateName), descriptor = openSync(leaf, "wx", mode);
  try {
    const midpoint = Math.ceil(bytes.byteLength / 2);
    if (midpoint > 0) writeSync(descriptor, bytes.subarray(0, midpoint));
    fsyncSync(descriptor);
    probe?.(`${phasePrefix}-write-mid`, logicalPath ?? leaf);
    if (midpoint < bytes.byteLength) writeSync(descriptor, bytes.subarray(midpoint));
    fsyncSync(descriptor);
  } finally { closeSync(descriptor); }
  chmodSync(leaf, mode);
  fsyncFile(leaf);
  fsyncDirectory(root);
  return { root, leaf };
}
