import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { sha256, lstatOrNull } from "../📁️input/🟦️.ts";
import { randomUUID } from "node:crypto";
import { closeSync, fsyncSync, mkdirSync, openSync, readFileSync, readdirSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, dirname, join, relative } from "node:path";
import { PLAN_HASH, PLAN_OPERATION_ID, type TaxonomyBackupRecord, type TaxonomyGeneratorNodeRecord, type TaxonomyJournalRecord, type TaxonomyJournalState, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare, planInteger, planPath, planRecord, planString } from "../🧾️plan-parsing/🟦️.ts";

export interface MutableJournalRecord {
  schemaVersion: 2;
  revision: number;
  planDigest: string;
  attemptOrdinal: string;
  state: TaxonomyJournalState;
  stagingRoot: string;
  backupRoot: string;
  journalWriteDirectory: string;
  jsonWritePreparationName: (pid: number, token: string) => string;
  jsonPreviousName: string;
  probe?: (phase: string, path?: string) => void;
  preparedMoveIds: string[];
  stagedMoveIds: string[];
  installedMoveIds: string[];
  preparedEmbeddedRelocationIds: string[];
  stagedEmbeddedRelocationIds: string[];
  installedEmbeddedRelocationIds: string[];
  preparedEvidenceRemovalIds: string[];
  stagedEvidenceRemovalIds: string[];
  preparedEmbeddedRootIds: string[];
  stagedEmbeddedRootIds: string[];
  preparedSymlinkTargetEditIds: string[];
  stagedSymlinkTargetEditIds: string[];
  installedSymlinkTargetEditIds: string[];
  appliedEditPaths: string[];
  startedRegenerationIds: string[];
  completedRegenerationIds: string[];
  sourceParentPrunePaths: string[];
  backups: Record<string, TaxonomyBackupRecord>;
  error?: string;
}

export function fsyncDirectory(path: string): void {
  try {
    const directory = openSync(path, "r");
    try { fsyncSync(directory); } finally { closeSync(directory); }
  } catch (error) {
    if (!["EINVAL", "ENOTSUP", "EISDIR"].includes(String((error as NodeJS.ErrnoException).code))) throw error;
  }
}

export function fsyncFile(path: string): void {
  const file = openSync(path, "r");
  try { fsyncSync(file); } finally { closeSync(file); }
}

export function durableRename(source: string, destination: string): void {
  renameSync(source, destination);
  fsyncDirectory(dirname(source));
  if (dirname(destination) !== dirname(source)) fsyncDirectory(dirname(destination));
}

export function durableSymlink(target: string, path: string, type?: "file" | "dir"): void {
  symlinkSync(target, path, type);
  fsyncDirectory(dirname(path));
}

export function durableRemove(path: string, recursive = false): void {
  rmSync(path, { recursive, force: true });
  fsyncDirectory(dirname(path));
}

export function durablySyncGeneratorRecords(repoRoot: string, records: readonly TaxonomyGeneratorNodeRecord[]): void {
  for (const record of [...records].sort((left, right) => right.path.split("/").length - left.path.split("/").length || generatorPathCompare(left.path, right.path))) {
    const path = absolutePath(repoRoot, record.path);
    if (record.nodeKind === "file") fsyncFile(path);
    else if (record.nodeKind === "directory") fsyncDirectory(path);
    else fsyncDirectory(dirname(path));
  }
}

function writeCanonicalFile(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  const temporary = `${path}.writing`;
  writeFileSync(temporary, `${canonicalJson(value)}\n`, "utf8");
  const file = openSync(temporary, "r");
  try { fsyncSync(file); } finally { closeSync(file); }
  durableRename(temporary, path);
}

export function publishCanonicalJsonCandidate(container: string, finalName: string, previousName: string, value: unknown, preparationName: (pid: number, token: string) => string, finalPath?: string, probe?: (phase: string, path?: string) => void, phasePrefix = "transaction-json"): void {
  mkdirSync(container, { recursive: true });
  const root = join(container, preparationName(process.pid, randomUUID()));
  const leaf = join(root, finalName);
  mkdirSync(root);
  fsyncDirectory(container);
  probe?.(`${phasePrefix}-write-mkdir`, root);
  const descriptor = openSync(leaf, "wx", 0o600);
  try { writeFileSync(descriptor, `${canonicalJson(value)}\n`, "utf8"); fsyncSync(descriptor); } finally { closeSync(descriptor); }
  fsyncDirectory(root);
  probe?.(`${phasePrefix}-candidate-written`, leaf);
  const final = finalPath ?? join(container, finalName);
  const previous = join(root, previousName), finalStat = lstatOrNull(final);
  if (finalStat && (!finalStat.isFile() || finalStat.isSymbolicLink())) throw new Error(`Canonical JSON destination is occupied: ${final}`);
  if (finalStat && readFileSync(final).equals(readFileSync(leaf))) durableRemove(leaf);
  else {
    if (finalStat) { durableRename(final, previous); probe?.(`${phasePrefix}-previous-exchanged`, previous); }
    durableRename(leaf, final);
    probe?.(`${phasePrefix}-canonical-exchanged`, final);
    if (lstatOrNull(previous)) durableRemove(previous);
  }
  durableRemove(root, true);
}

export function journalSnapshot(journal: MutableJournalRecord): TaxonomyJournalRecord {
  return {
    schemaVersion: 2,
    revision: journal.revision,
    planDigest: journal.planDigest,
    attemptOrdinal: journal.attemptOrdinal,
    state: journal.state,
    stagingRoot: journal.stagingRoot,
    backupRoot: journal.backupRoot,
    preparedMoveIds: [...journal.preparedMoveIds].sort(generatorPathCompare),
    stagedMoveIds: [...journal.stagedMoveIds].sort(generatorPathCompare),
    installedMoveIds: [...journal.installedMoveIds].sort(generatorPathCompare),
    preparedEmbeddedRelocationIds: [...journal.preparedEmbeddedRelocationIds].sort(generatorPathCompare),
    stagedEmbeddedRelocationIds: [...journal.stagedEmbeddedRelocationIds].sort(generatorPathCompare),
    installedEmbeddedRelocationIds: [...journal.installedEmbeddedRelocationIds].sort(generatorPathCompare),
    preparedEvidenceRemovalIds: [...journal.preparedEvidenceRemovalIds].sort(generatorPathCompare),
    stagedEvidenceRemovalIds: [...journal.stagedEvidenceRemovalIds].sort(generatorPathCompare),
    preparedEmbeddedRootIds: [...journal.preparedEmbeddedRootIds].sort(generatorPathCompare),
    stagedEmbeddedRootIds: [...journal.stagedEmbeddedRootIds].sort(generatorPathCompare),
    preparedSymlinkTargetEditIds: [...journal.preparedSymlinkTargetEditIds].sort(generatorPathCompare),
    stagedSymlinkTargetEditIds: [...journal.stagedSymlinkTargetEditIds].sort(generatorPathCompare),
    installedSymlinkTargetEditIds: [...journal.installedSymlinkTargetEditIds].sort(generatorPathCompare),
    appliedEditPaths: [...journal.appliedEditPaths].sort(generatorPathCompare),
    startedRegenerationIds: [...journal.startedRegenerationIds].sort(generatorPathCompare),
    completedRegenerationIds: [...journal.completedRegenerationIds].sort(generatorPathCompare),
    sourceParentPrunePaths: [...journal.sourceParentPrunePaths].sort(generatorPathCompare),
    backups: Object.fromEntries(Object.entries(journal.backups).sort(([a], [b]) => generatorPathCompare(a, b))),
    error: journal.error,
  };
}

export function persistJournal(repoRoot: string, path: string, journal: MutableJournalRecord): void {
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const walRoot = join(stageRoot, journal.journalWriteDirectory);
  const walStat = lstatOrNull(walRoot);
  if (walStat && (!walStat.isDirectory() || walStat.isSymbolicLink() || readdirSync(walRoot).length > 0)) throw new Error(`Taxonomy journal WAL is occupied: ${walRoot}`);
  mkdirSync(walRoot, { recursive: true });
  const next = { ...journal, revision: journal.revision + 1 };
  publishCanonicalJsonCandidate(walRoot, basename(path), journal.jsonPreviousName, journalSnapshot(next), journal.jsonWritePreparationName, path, journal.probe, "transaction-journal");
  journal.probe?.("transaction-wal-prepared", normalizeRelative(relative(repoRoot, walRoot).replaceAll("\\", "/")));
  journal.revision = next.revision;
  durableRemove(walRoot, true);
}

export function readJournal(path: string, journalWriteDirectory: string, jsonWritePreparationName: (pid: number, token: string) => string, jsonPreviousName: string): MutableJournalRecord {
  const bytes = readFileSync(path, "utf8");
  const parsed = JSON.parse(bytes);
  const value = planRecord(parsed, "taxonomy journal", ["schemaVersion", "revision", "planDigest", "attemptOrdinal", "state", "stagingRoot", "backupRoot", "preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds", "sourceParentPrunePaths", "backups"], ["error"]) as Partial<TaxonomyJournalRecord>;
  const arrays = ["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds", "sourceParentPrunePaths"] as const;
  const states: readonly TaxonomyJournalState[] = ["prepared", "staging", "disposing", "installing", "retargeting", "editing", "regenerating", "verifying", "committed", "rolling-back", "rolled-back"];
  if (value.schemaVersion !== 2 || typeof value.revision !== "number" || !Number.isSafeInteger(value.revision) || value.revision < 0 || typeof value.planDigest !== "string" || !PLAN_HASH.test(value.planDigest) || typeof value.attemptOrdinal !== "string" || !/^[0-9]{6}$/u.test(value.attemptOrdinal) || !states.includes(value.state as TaxonomyJournalState) || typeof value.stagingRoot !== "string" || typeof value.backupRoot !== "string" || !arrays.every((key) => Array.isArray(value[key])) || !value.backups || typeof value.backups !== "object" || (value.error !== undefined && typeof value.error !== "string")) throw new Error(`Invalid taxonomy journal at ${path}`);
  planPath(value.stagingRoot, "taxonomy journal stagingRoot");
  planPath(value.backupRoot, "taxonomy journal backupRoot");
  for (const key of arrays) {
    const ids = value[key] as readonly unknown[];
    const pattern = key === "appliedEditPaths" || key === "sourceParentPrunePaths" ? undefined : PLAN_OPERATION_ID;
    const parsedIds = ids.map((entry, index) => pattern ? planString(entry, `taxonomy journal ${key}[${index}]`, pattern) : planPath(entry, `taxonomy journal ${key}[${index}]`));
    if (new Set(parsedIds).size !== parsedIds.length || parsedIds.some((entry, index) => index > 0 && Buffer.from(requireTaxonomyValue(parsedIds[index - 1],"parsedIds[index - 1]")).compare(Buffer.from(entry)) >= 0)) throw new Error(`Taxonomy journal ${key} must be unique and bytewise sorted`);
  }
  for (const [logicalPath, backup] of Object.entries(value.backups)) {
    planPath(logicalPath, "taxonomy journal backup path");
    const candidate = planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind"], ["backupPath", "contentHash", "mode", "size", "target", "targetHash"]);
    if (candidate.kind === "absent") planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind"]);
    else if (candidate.kind === "file") {
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind", "backupPath", "contentHash", "mode", "size"]);
      planPath(candidate.backupPath, `taxonomy journal backup ${logicalPath}.backupPath`);
      planString(candidate.contentHash, `taxonomy journal backup ${logicalPath}.contentHash`, PLAN_HASH);
      planInteger(candidate.mode, `taxonomy journal backup ${logicalPath}.mode`, 0o7777);
      planInteger(candidate.size, `taxonomy journal backup ${logicalPath}.size`);
    } else if (candidate.kind === "symlink") {
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind", "target", "targetHash", "mode", "size"]);
      const target = planString(candidate.target, `taxonomy journal backup ${logicalPath}.target`);
      if (planString(candidate.targetHash, `taxonomy journal backup ${logicalPath}.targetHash`, PLAN_HASH) !== sha256(target) || planInteger(candidate.mode, `taxonomy journal backup ${logicalPath}.mode`, 0o7777) < 0 || planInteger(candidate.size, `taxonomy journal backup ${logicalPath}.size`) !== Buffer.byteLength(target)) throw new Error(`Taxonomy journal backup ${logicalPath} symlink preimage changed`);
    } else throw new Error(`Taxonomy journal backup ${logicalPath}.kind is invalid`);
  }
  const journal: MutableJournalRecord = {
    schemaVersion: 2,
    revision: value.revision,
    planDigest: value.planDigest,
    attemptOrdinal: value.attemptOrdinal,
    state: value.state as TaxonomyJournalState,
    stagingRoot: value.stagingRoot,
    backupRoot: value.backupRoot,
    journalWriteDirectory,
    jsonWritePreparationName,
    jsonPreviousName,
    preparedMoveIds: [...value.preparedMoveIds!],
    stagedMoveIds: [...value.stagedMoveIds!],
    installedMoveIds: [...value.installedMoveIds!],
    preparedEmbeddedRelocationIds: [...value.preparedEmbeddedRelocationIds!],
    stagedEmbeddedRelocationIds: [...value.stagedEmbeddedRelocationIds!],
    installedEmbeddedRelocationIds: [...value.installedEmbeddedRelocationIds!],
    preparedEvidenceRemovalIds: [...value.preparedEvidenceRemovalIds!],
    stagedEvidenceRemovalIds: [...value.stagedEvidenceRemovalIds!],
    preparedEmbeddedRootIds: [...value.preparedEmbeddedRootIds!],
    stagedEmbeddedRootIds: [...value.stagedEmbeddedRootIds!],
    preparedSymlinkTargetEditIds: [...value.preparedSymlinkTargetEditIds!],
    stagedSymlinkTargetEditIds: [...value.stagedSymlinkTargetEditIds!],
    installedSymlinkTargetEditIds: [...value.installedSymlinkTargetEditIds!],
    appliedEditPaths: [...value.appliedEditPaths!],
    startedRegenerationIds: [...value.startedRegenerationIds!],
    completedRegenerationIds: [...value.completedRegenerationIds!],
    sourceParentPrunePaths: [...value.sourceParentPrunePaths!],
    backups: { ...value.backups! },
    error: value.error,
  };
  if (bytes !== `${canonicalJson(journalSnapshot(journal))}\n`) throw new Error(`Taxonomy journal at ${path} is not canonical JSON`);
  if (journal.state !== "committed" && journal.sourceParentPrunePaths.length > 0) throw new Error("Source-parent pruning authority requires a committed journal");
  const empty = (keys: readonly (keyof MutableJournalRecord)[]): boolean => keys.every((key) => Array.isArray(journal[key]) && (journal[key] as unknown[]).length === 0);
  const moveFuture = ["installedMoveIds"] as const;
  const disposalFuture = ["preparedEmbeddedRootIds", "stagedEmbeddedRootIds"] as const;
  const relocationFuture = ["installedEmbeddedRelocationIds"] as const;
  const linkFuture = ["preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds"] as const;
  const editFuture = ["appliedEditPaths"] as const;
  const regenerationFuture = ["startedRegenerationIds", "completedRegenerationIds"] as const;
  if ((journal.state === "prepared" && (!empty(["preparedMoveIds", "stagedMoveIds", ...moveFuture, "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", ...relocationFuture, "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", ...disposalFuture, ...linkFuture, ...editFuture, ...regenerationFuture]) || Object.keys(journal.backups).length > 0))
    || (journal.state === "staging" && !empty([...moveFuture, ...disposalFuture, ...relocationFuture, ...linkFuture, ...editFuture, ...regenerationFuture]))
    || (journal.state === "disposing" && !empty([...moveFuture, ...relocationFuture, ...linkFuture, ...editFuture, ...regenerationFuture]))
    || (journal.state === "installing" && !empty([...linkFuture, ...editFuture, ...regenerationFuture]))
    || (journal.state === "retargeting" && !empty([...editFuture, ...regenerationFuture]))
    || (journal.state === "editing" && !empty(regenerationFuture))
    || (journal.state === "rolled-back" && !empty(["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"]))) throw new Error(`Taxonomy journal state ${journal.state} contains operations from an impossible phase`);
  return journal;
}

export const JOURNAL_OPERATION_ARRAYS = ["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"] as const;

export function assertJournalTransition(current: MutableJournalRecord, next: MutableJournalRecord): void {
  const order: readonly TaxonomyJournalState[] = ["prepared", "staging", "disposing", "installing", "retargeting", "editing", "regenerating", "verifying", "committed"];
  const currentRank = order.indexOf(current.state), nextRank = order.indexOf(next.state);
  const transition = current.state === "committed" || current.state === "rolled-back"
    ? next.state === current.state
    : current.state === "rolling-back" ? next.state === "rolling-back" || next.state === "rolled-back"
      : next.state === current.state || nextRank === currentRank + 1 || next.state === "rolling-back";
  if (!transition) throw new Error(`Taxonomy journal WAL has an invalid ${current.state} -> ${next.state} transition`);
  if (!(current.state === "verifying" && next.state === "committed") && canonicalJson(current.sourceParentPrunePaths) !== canonicalJson(next.sourceParentPrunePaths)) throw new Error("Taxonomy journal WAL cannot change source-parent pruning authority outside commit");
  const contains = (parent: readonly string[], child: readonly string[]): boolean => child.every((entry) => parent.includes(entry));
  const currentTerminal = current.state === "committed" || current.state === "rolled-back";
  const rollingBack = current.state === "rolling-back";
  for (const key of JOURNAL_OPERATION_ARRAYS) {
    const valid = currentTerminal
      ? canonicalJson(current[key]) === canonicalJson(next[key])
      : rollingBack ? contains(current[key], next[key]) : contains(next[key], current[key]);
    if (!valid) throw new Error(`Taxonomy journal WAL ${key} is not a legal monotonic transition`);
  }
  const currentBackupKeys = Object.keys(current.backups);
  const backupsValid = currentTerminal
    ? canonicalJson(current.backups) === canonicalJson(next.backups)
    : currentBackupKeys.every((key) => canonicalJson(current.backups[key]) === canonicalJson(next.backups[key]));
  if (!backupsValid) throw new Error("Taxonomy journal WAL backups are not a legal monotonic transition");
  if (currentTerminal && next.error !== current.error) throw new Error("Taxonomy journal WAL cannot alter terminal error evidence");
}
