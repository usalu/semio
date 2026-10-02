/** 🏗️ Canonical 🚪️source-admission source service. */
import { inScope, normalizeRelative } from "../🛣️path/🟦️.ts";

export type TaxonomyNodeKind = "directory" | "file" | "symlink";

export type TaxonomySourceOrigin = "tracked" | "nonignored-untracked" | "ignored-generator" | "explicit-ticket";

export type TaxonomySourceObservedKind = TaxonomyNodeKind | "absent" | "unobserved" | "other";

export interface TaxonomySourceIndexEntry {
  readonly stage: number;
  readonly mode: string;
  readonly objectId: string;
}

export interface TaxonomySourceGeneratorOutput {
  readonly contractId: string;
  readonly rootPath: string;
  readonly inclusion: "tracked" | "ignored";
}

export interface TaxonomySourceCandidateObservation {
  readonly sourcePath: string;
  readonly observedKind: TaxonomySourceObservedKind;
  readonly worktreeMode: string | null;
  readonly explicitDirectory: boolean;
  readonly origins: readonly TaxonomySourceOrigin[];
  readonly indexEntries: readonly TaxonomySourceIndexEntry[];
  readonly unsafeAncestor: boolean;
}

export interface TaxonomySourceAdmissionInput {
  readonly scope: string | null;
  readonly cancelledDuring?: string | null;
  readonly opaquePrefixes: readonly string[];
  readonly generatorOutputRoots: readonly TaxonomySourceGeneratorOutput[];
  readonly candidates: readonly TaxonomySourceCandidateObservation[];
}

export interface TaxonomySourceObservation extends Omit<TaxonomySourceCandidateObservation, "unsafeAncestor"> {
  readonly generatorOutputs: readonly TaxonomySourceGeneratorOutput[];
  readonly repositoryBoundary: "gitlink" | null;
}

export interface TaxonomySourceAdmissionDiagnostic {
  readonly code: string;
  readonly path: string;
  readonly message: string;
}

export interface TaxonomySourceAdmission {
  readonly schemaVersion: 1;
  readonly scope: string | null;
  readonly status: "complete" | "rejected";
  readonly observations: readonly TaxonomySourceObservation[];
  readonly diagnostics: readonly TaxonomySourceAdmissionDiagnostic[];
}

export const SOURCE_ADMISSION_ORIGINS: readonly TaxonomySourceOrigin[] = ["tracked", "nonignored-untracked", "ignored-generator", "explicit-ticket"];

export const sourceAdmissionByteCompare = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));

export function sourceAdmissionSafePath(path: string): boolean {
  return path.length > 0 && !path.startsWith("/") && !/^[A-Za-z]:/u.test(path) && !path.includes("\\") && !/[\u0000-\u001f\u007f]/u.test(path) && Buffer.from(path).toString("utf8") === path && path.split("/").every((part) => part.length > 0 && part !== "." && part !== "..");
}

export function sourceAdmissionOpaque(path: string, prefixes: readonly string[]): boolean {
  const normalized = path.normalize("NFC");
  return prefixes.some((prefix) => {
    const expected = prefix.normalize("NFC");
    return expected === "compose" ? normalized.split("/").some((part) => part.toLowerCase() === "compose") : normalized === expected || normalized.startsWith(expected + "/");
  });
}

export function sourceAdmissionRecord(value: unknown, required: readonly string[], optional: readonly string[] = []): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value) && required.every((key) => Object.hasOwn(value, key)) && Object.keys(value).every((key) => required.includes(key) || optional.includes(key));
}

function sourceAdmissionInputShape(value: unknown): value is TaxonomySourceAdmissionInput {
  if (!sourceAdmissionRecord(value, ["scope", "opaquePrefixes", "generatorOutputRoots", "candidates"], ["cancelledDuring"])) return false;
  if (value.scope !== null && typeof value.scope !== "string") return false;
  if (value.cancelledDuring !== undefined && value.cancelledDuring !== null && typeof value.cancelledDuring !== "string") return false;
  if (!Array.isArray(value.opaquePrefixes) || value.opaquePrefixes.some((prefix) => typeof prefix !== "string" || prefix.length === 0) || new Set(value.opaquePrefixes).size !== value.opaquePrefixes.length) return false;
  if (!Array.isArray(value.generatorOutputRoots) || value.generatorOutputRoots.some((root) => !sourceAdmissionRecord(root, ["contractId", "rootPath", "inclusion"]) || typeof root.contractId !== "string" || !root.contractId || typeof root.rootPath !== "string" || !root.rootPath || (root.inclusion !== "tracked" && root.inclusion !== "ignored"))) return false;
  if (!Array.isArray(value.candidates)) return false;
  return value.candidates.every((row) => {
    if (!sourceAdmissionRecord(row, ["sourcePath", "observedKind", "worktreeMode", "explicitDirectory", "origins", "indexEntries", "unsafeAncestor"])) return false;
    if (typeof row.sourcePath !== "string" || !row.sourcePath || !["file", "directory", "symlink", "absent", "unobserved", "other"].includes(row.observedKind as string) || ![null, "100644", "100755", "120000", "160000", "040000"].includes(row.worktreeMode as string | null) || typeof row.explicitDirectory !== "boolean" || typeof row.unsafeAncestor !== "boolean") return false;
    if (!Array.isArray(row.origins) || row.origins.some((origin) => !SOURCE_ADMISSION_ORIGINS.includes(origin)) || new Set(row.origins).size !== row.origins.length || !Array.isArray(row.indexEntries)) return false;
    return row.indexEntries.every((entry) => sourceAdmissionRecord(entry, ["stage", "mode", "objectId"]) && Number.isInteger(entry.stage) && Number(entry.stage) >= 0 && Number(entry.stage) <= 3 && ["100644", "100755", "120000", "160000"].includes(entry.mode as string) && typeof entry.objectId === "string" && /^(?:[a-f0-9]{40}|[a-f0-9]{64})$/u.test(entry.objectId));
  });
}

function sourceAdmissionPhysicalConsistent(row: TaxonomySourceCandidateObservation): boolean {
  return (row.observedKind === "file" && (row.worktreeMode === "100644" || row.worktreeMode === "100755") && !row.explicitDirectory)
    || (row.observedKind === "directory" && row.worktreeMode === "040000" && row.explicitDirectory)
    || (row.observedKind === "symlink" && row.worktreeMode === "120000" && !row.explicitDirectory)
    || (["absent", "unobserved", "other"].includes(row.observedKind) && row.worktreeMode === null && !row.explicitDirectory);
}

export function sourceAdmissionRepositoryFences(rows: readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[]): readonly string[] {
  return [...new Set(rows.filter((row) => row.entry.mode === "160000" && sourceAdmissionSafePath(row.path)).map((row) => row.path))].sort(sourceAdmissionByteCompare);
}

export function sourceAdmissionContainingRepository(path: string, fences: readonly string[], includeRoot: boolean): string | null {
  const normalized = path.normalize("NFC");
  return fences.find((fence) => {
    const root = fence.normalize("NFC");
    return (includeRoot && normalized === root) || normalized.startsWith(root + "/");
  }) ?? null;
}

export function sourceAdmissionAssertRepositoryPath(path: string, fences: readonly string[], label: string, allowRoot: boolean): void {
  const boundary = sourceAdmissionContainingRepository(path, fences, !allowRoot);
  if (boundary !== null) throw new Error(`${label} crosses an index-owned repository boundary: ${path} (${boundary})`);
}

/** 🧾️ Projects supplied source observations; only inventoryTaxonomySources performs filesystem admission. */
export function projectTaxonomySourceAdmission(value: unknown): TaxonomySourceAdmission {
  if (!sourceAdmissionInputShape(value)) return { schemaVersion: 1, scope: null, status: "rejected", observations: [], diagnostics: [{ code: "invalid-admission-input", path: "$", message: "Source admission input does not satisfy its closed schema" }] };
  const input = value;
  if (input.cancelledDuring !== undefined && input.cancelledDuring !== null) return { schemaVersion: 1, scope: input.scope, status: "rejected", observations: [], diagnostics: [{ code: "cancelled", path: input.cancelledDuring || "$", message: "Cancellation prevents a partial-success admission result" }] };
  if (input.scope !== null && !sourceAdmissionSafePath(input.scope)) return { schemaVersion: 1, scope: input.scope, status: "rejected", observations: [], diagnostics: [{ code: "invalid-scope", path: input.scope || "$", message: "Scope is not a safe repository-relative slash path" }] };
  const repositoryFences = sourceAdmissionRepositoryFences(input.candidates.flatMap((row) => row.indexEntries.filter((entry) => entry.mode === "160000").map((entry) => ({ path: row.sourcePath, entry }))));
  if (input.scope !== null && sourceAdmissionContainingRepository(input.scope, repositoryFences, false) !== null) return { schemaVersion: 1, scope: input.scope, status: "rejected", observations: [], diagnostics: [{ code: "scope-inside-repository-boundary", path: input.scope, message: "Scope is below an index-owned repository boundary" }] };
  const diagnostics: TaxonomySourceAdmissionDiagnostic[] = [];
  const diagnose = (code: string, path: string, message: string): void => { diagnostics.push({ code, path, message }); };
  for (const prefix of input.opaquePrefixes) if (!sourceAdmissionSafePath(prefix)) diagnose("invalid-opaque-prefix", prefix, "Opaque prefix is not a safe repository-relative slash path");
  for (const output of input.generatorOutputRoots) if (!sourceAdmissionSafePath(output.rootPath)) diagnose("invalid-generator-root", output.rootPath, "Generator output root is not a safe repository-relative slash path");
  for (const output of input.generatorOutputRoots) if (sourceAdmissionSafePath(output.rootPath) && sourceAdmissionContainingRepository(output.rootPath, repositoryFences, false) !== null) diagnose("generator-root-inside-repository-boundary", output.rootPath, "Generator output root is below an index-owned repository boundary");
  const generatorPolicies = new Map<string, TaxonomySourceGeneratorOutput[]>();
  for (const output of input.generatorOutputRoots) {
    const key = JSON.stringify([output.contractId, output.rootPath]);
    const group = generatorPolicies.get(key) ?? [];
    group.push(output);
    generatorPolicies.set(key, group);
  }
  for (const group of generatorPolicies.values()) if (new Set(group.map((output) => output.inclusion)).size > 1) diagnose("contradictory-generator-output", group[0].rootPath, "One generator contract/root identity declares conflicting inclusion policies");
  const groups = new Map<string, TaxonomySourceCandidateObservation[]>();
  for (const row of input.candidates) {
    if (!sourceAdmissionSafePath(row.sourcePath)) { diagnose("invalid-source-path", row.sourcePath, "Candidate sourcePath is not a safe repository-relative slash path"); continue; }
    if (sourceAdmissionContainingRepository(row.sourcePath, repositoryFences, false) !== null) diagnose("repository-boundary-descendant", row.sourcePath, "Candidate is below an index-owned repository boundary");
    if (!inScope(row.sourcePath, input.scope ?? undefined)) continue;
    const group = groups.get(row.sourcePath) ?? [];
    group.push(row);
    groups.set(row.sourcePath, group);
  }
  const observations = [...groups].map(([sourcePath, rows]): TaxonomySourceObservation => {
    const first = rows[0], normalized = sourcePath.normalize("NFC");
    const physical = new Set(rows.map((row) => JSON.stringify([row.observedKind, row.worktreeMode, row.explicitDirectory])));
    const indexEntries = [...new Map(rows.flatMap((row) => row.indexEntries).map((entry) => [JSON.stringify([entry.stage, entry.mode, entry.objectId]), entry])).values()].sort((left, right) => left.stage - right.stage || sourceAdmissionByteCompare(left.mode, right.mode) || sourceAdmissionByteCompare(left.objectId, right.objectId));
    const generatorOutputs = [...new Map(input.generatorOutputRoots.filter((root) => normalized === root.rootPath.normalize("NFC") || normalized.startsWith(root.rootPath.normalize("NFC") + "/")).map((root) => [JSON.stringify([root.contractId, root.rootPath, root.inclusion]), root])).values()].sort((left, right) => sourceAdmissionByteCompare(left.contractId, right.contractId) || sourceAdmissionByteCompare(left.rootPath, right.rootPath) || sourceAdmissionByteCompare(left.inclusion, right.inclusion));
    const supplied = new Set(rows.flatMap((row) => row.origins)), hasIgnored = generatorOutputs.some((root) => root.inclusion === "ignored");
    const opaque = sourceAdmissionOpaque(sourcePath, input.opaquePrefixes), unsafe = rows.some((row) => row.unsafeAncestor);
    const stageZero = indexEntries.some((entry) => entry.stage === 0), conflicted = indexEntries.some((entry) => entry.stage !== 0);
    const repositoryBoundary = !opaque && !unsafe && physical.size === 1 && supplied.has("tracked") && indexEntries.length === 1 && indexEntries[0].stage === 0 && indexEntries[0].mode === "160000" && sourceAdmissionPhysicalConsistent(first) && (first.observedKind === "directory" || first.observedKind === "absent") ? "gitlink" as const : null;
    if (physical.size > 1) diagnose("contradictory-physical-observation", sourcePath, "Duplicate rows disagree on observed physical kind, mode, or directory status");
    if (new Set(indexEntries.map((entry) => entry.stage)).size !== indexEntries.length) diagnose("contradictory-index-entry", sourcePath, "Duplicate rows disagree on an exact Git index stage identity");
    if (supplied.has("ignored-generator") && !hasIgnored) diagnose("untrusted-generator-origin", sourcePath, "Ignored-generator authority is derived only from declared ignored output roots");
    if (opaque) diagnose("opaque-path", sourcePath, "Configured opaque prefix rejected before candidate projection");
    if (unsafe) diagnose("unsafe-ancestor", sourcePath, "A symlink or non-directory ancestor prevented observation");
    if (rows.some((row) => !sourceAdmissionPhysicalConsistent(row) && row.worktreeMode !== "160000")) diagnose("inconsistent-physical-observation", sourcePath, "Observed kind, worktree mode, and explicit-directory status are inconsistent");
    if (rows.some((row) => row.observedKind === "other" || row.worktreeMode === "160000") || (repositoryBoundary === null && indexEntries.some((entry) => entry.mode === "160000"))) diagnose("nonregular-node", sourcePath, "Gitlink and other nonregular nodes cannot be admitted as authored source");
    if (stageZero && !supplied.has("tracked")) diagnose("index-without-tracked-origin", sourcePath, "Stage-zero index identity requires tracked admission provenance");
    if (supplied.has("tracked") && !stageZero && !conflicted) diagnose("tracked-origin-without-stage-zero", sourcePath, "Tracked admission requires an exact stage-zero index identity");
    if (conflicted) diagnose("conflicted-index", sourcePath, "Nonzero Git index stages prevent unambiguous source admission");
    if (rows.some((row) => row.observedKind === "unobserved") && !opaque && !unsafe && !conflicted) diagnose("unobserved-without-error", sourcePath, "Unobserved candidates require an explicit unsafe, opaque, conflict, or cancellation cause");
    if (rows.some((row) => row.observedKind === "absent") && stageZero) diagnose("tracked-path-absent", sourcePath, "Stage-zero index identity is retained although the worktree path is absent");
    if (hasIgnored) supplied.add("ignored-generator");
    const origins = opaque ? [] : SOURCE_ADMISSION_ORIGINS.filter((origin) => supplied.has(origin) && (origin !== "ignored-generator" || hasIgnored));
    if (!opaque && origins.length === 0 && !conflicted && !stageZero && !supplied.has("ignored-generator")) diagnose("no-admission-origin", sourcePath, "Candidate has no admitted source authority");
    return {
      sourcePath,
      observedKind: physical.size === 1 && !opaque && !unsafe ? first.observedKind : "unobserved",
      worktreeMode: physical.size === 1 && !opaque && !unsafe ? first.worktreeMode : null,
      explicitDirectory: physical.size === 1 && !opaque && !unsafe && first.explicitDirectory,
      origins,
      indexEntries: opaque ? [] : indexEntries,
      generatorOutputs: opaque ? [] : generatorOutputs,
      repositoryBoundary,
    };
  }).sort((left, right) => sourceAdmissionByteCompare(left.sourcePath, right.sourcePath));
  diagnostics.sort((left, right) => sourceAdmissionByteCompare(left.path, right.path) || sourceAdmissionByteCompare(left.code, right.code) || sourceAdmissionByteCompare(left.message, right.message));
  return { schemaVersion: 1, scope: input.scope, status: diagnostics.some((row) => row.code !== "tracked-path-absent") ? "rejected" : "complete", observations, diagnostics };
}

export interface TaxonomyScopedGitPathspec {
  readonly normalizedScope: string | null;
  readonly conservativePrefix: string;
  readonly positivePathspec: string;
  readonly exclusionPathspecs: readonly string[];
}

/** 🧲️ Renders a byte-literal Git candidate prefix while retaining NFC scope authority in memory. */
export function taxonomyScopedGitPathspec(inputScope: string | null | undefined, opaqueExclusions: readonly string[]): TaxonomyScopedGitPathspec {
  const normalizedScope = inputScope === null || inputScope === undefined ? null : normalizeRelative(inputScope) || null;
  const stable: string[] = [];
  for (const segment of normalizedScope?.split("/") ?? []) {
    if (segment.normalize("NFD") !== segment) break;
    stable.push(segment);
  }
  const conservativePrefix = normalizedScope && stable.length > 0 ? stable.join("/") : ".";
  const intersects = (exclusion: string): boolean => conservativePrefix === "." || exclusion === conservativePrefix || exclusion.startsWith(`${conservativePrefix}/`) || conservativePrefix.startsWith(`${exclusion}/`);
  const exclusionPathspecs = [...new Set(opaqueExclusions.map(normalizeRelative))]
    .filter(intersects)
    .sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))
    .map((path) => `:(exclude,top,literal)${path}`);
  return {
    normalizedScope,
    conservativePrefix,
    positivePathspec: conservativePrefix === "." ? "." : `:(top,literal)${conservativePrefix}`,
    exclusionPathspecs,
  };
}
