import { TICKET_GENERATED_OUTPUT_DIRECTORY } from "../../🗂️workspaces/🧬️metadata/🟦️.ts";
import { taxonomyScopedGitPathspec, type TaxonomySourceIndexEntry, sourceAdmissionRecord, sourceAdmissionSafePath, sourceAdmissionRepositoryFences, sourceAdmissionByteCompare, sourceAdmissionContainingRepository } from "../🚪️source-admission/🟦️.ts";
import { TAXONOMY_RELATIVE_PATH } from "../🔣️taxonomy/🟦️.ts";
import { sha256, noFollowDirectoryChain } from "../📁️input/🟦️.ts";
import { sourceAdmissionAssertLexical, sourceAdmissionGitRows } from "../🚪️source-admission/📁️io/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import type { TaxonomyApplyOptions, TaxonomyPlan } from "../📜️plan-contracts/🟦️.ts";
import { pathsOverlap, removalAuthorityPaths } from "../🧐️preimage-assertions/🟦️.ts";

type TransactionRepositoryAuthorityFailureReason = "missing-authority" | "invalid-index" | "index-drift" | "repository-boundary" | "invalid-access";

export class TransactionRepositoryAuthorityError extends Error {
  constructor(readonly reason: TransactionRepositoryAuthorityFailureReason, cause: unknown) {
    super(cause instanceof Error ? cause.message : String(cause), { cause });
    this.name = "TransactionRepositoryAuthorityError";
  }
}

export function isTransactionRepositoryAuthorityError(error: unknown): error is TransactionRepositoryAuthorityError {
  return error instanceof TransactionRepositoryAuthorityError;
}

export function transactionRepositoryFinally(body: () => void, release: () => void): void {
  let preserve = false;
  try { body(); }
  catch (error) { preserve = isTransactionRepositoryAuthorityError(error); throw error; }
  finally { if (!preserve) release(); }
}

export class TransactionRepositoryAuthority {
  readonly #validated = true;
  readonly repoRoot: string;
  readonly indexRows: readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[];
  readonly repositoryFences: readonly string[];
  readonly indexWitness: string;

  constructor(repoRoot: string, rows: unknown) {
    try {
      sourceAdmissionAssertLexical(repoRoot, "Transaction repository root", true);
      if (!isAbsolute(repoRoot) || resolve(repoRoot) !== repoRoot) throw new Error("Transaction repository root is not absolute and canonical");
    } catch (cause) {
      if (isTransactionRepositoryAuthorityError(cause)) throw cause;
      throw new TransactionRepositoryAuthorityError("invalid-access", cause);
    }
    if (!Array.isArray(rows)) throw new TransactionRepositoryAuthorityError("invalid-index", new Error("Transaction index rows are invalid"));
    this.repoRoot = repoRoot;
    this.indexRows = Object.freeze(rows.map((row: unknown) => {
      if (!sourceAdmissionRecord(row, ["path", "entry"]) || typeof row.path !== "string" || !sourceAdmissionSafePath(row.path) || !sourceAdmissionRecord(row.entry, ["stage", "mode", "objectId"])) throw new TransactionRepositoryAuthorityError("invalid-index", new Error("Transaction index row is invalid"));
      const entry = row.entry;
      if (!Number.isInteger(entry.stage) || Number(entry.stage) < 0 || Number(entry.stage) > 3 || !["100644", "100755", "120000", "160000"].includes(entry.mode as string) || typeof entry.objectId !== "string" || !/^(?:[a-f0-9]{40}|[a-f0-9]{64})$/u.test(entry.objectId)) throw new TransactionRepositoryAuthorityError("invalid-index", new Error("Transaction index entry is invalid"));
      return Object.freeze({ path: row.path, entry: Object.freeze({ stage: entry.stage as number, mode: entry.mode as string, objectId: entry.objectId }) });
    }));
    this.repositoryFences = Object.freeze(sourceAdmissionRepositoryFences(this.indexRows));
    this.indexWitness = sha256(this.indexRows.map((row) => canonicalJson(row)).sort(sourceAdmissionByteCompare).join("\0"));
    Object.freeze(this);
  }

  static owns(value: unknown): value is TransactionRepositoryAuthority {
    return typeof value === "object" && value !== null && #validated in value;
  }
}

export function captureTransactionRepositoryAuthority(repoRoot: string): TransactionRepositoryAuthority {
  try {
    sourceAdmissionAssertLexical(repoRoot, "Transaction repository root", true);
    if (!isAbsolute(repoRoot) || resolve(repoRoot) !== repoRoot) throw new Error("Transaction repository root is not absolute and canonical");
    noFollowDirectoryChain(repoRoot);
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  try { return new TransactionRepositoryAuthority(repoRoot, sourceAdmissionGitRows(repoRoot, taxonomyScopedGitPathspec(undefined, ["compose"]))); }
  catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-index", cause);
  }
}

/** 🚧️ "subtree" rejects any overlap (a real recursive filesystem operation could relocate a nested boundary); "point"/"input" reject only paths at-or-below a fence — "input" additionally admits the fence path itself, since a terminal generator-input node is recorded, never descended. */
export function assertTransactionRepositoryPath(authority: TransactionRepositoryAuthority, path: string, role: "point" | "subtree" | "input", label: string): void {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  try {
    if (role !== "point" && role !== "subtree" && role !== "input") throw new Error("Transaction repository access role is invalid");
    if (path !== "") sourceAdmissionAssertLexical(path, label, false);
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  const boundary = role === "subtree"
    ? authority.repositoryFences.find((fence) => pathsOverlap(path, fence))
    : sourceAdmissionContainingRepository(path, authority.repositoryFences, role === "point");
  if (boundary !== null && boundary !== undefined) throw new TransactionRepositoryAuthorityError("repository-boundary", new Error(label + " crosses an index-owned repository boundary: " + path + " (" + boundary + ")"));
}

export function assertTransactionRepositoryWitness(authority: TransactionRepositoryAuthority, rows: unknown): void {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  const current = new TransactionRepositoryAuthority(authority.repoRoot, rows);
  if (current.indexWitness !== authority.indexWitness) {
    throw new TransactionRepositoryAuthorityError("index-drift", new Error("Transaction repository index changed since capture"));
  }
}

export function transactionRepositoryBootstrapPaths(plan: TaxonomyPlan, options: TaxonomyApplyOptions, repoRoot: string): Readonly<{
  taxonomyPath: string;
  ticketDir: string;
  planArtifactPath: string;
  resumeJournal?: string;
  accesses: readonly Readonly<{ path: string; role: "point" | "subtree" | "input"; label: string }>[];
}> {
  if (options.repoRoot !== ".") sourceAdmissionAssertLexical(options.repoRoot, "repoRoot", true);
  sourceAdmissionAssertLexical(repoRoot, "repoRoot", true);
  const accesses: { path: string; role: "point" | "subtree" | "input"; label: string }[] = [];
  const add = (path: string, role: "point" | "subtree" | "input", label: string): void => {
    if (path !== "") sourceAdmissionAssertLexical(path, label, false);
    accesses.push({ path, role, label });
  };
  const local = (value: string, label: string): string => {
    sourceAdmissionAssertLexical(value, label, true);
    const path = relative(repoRoot, isAbsolute(value) ? value : join(repoRoot, value)).split(sep).join("/");
    sourceAdmissionAssertLexical(path, label, false);
    add(path, "point", label);
    return path;
  };
  const taxonomyPath = local(options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath");
  const ticketDir = local(options.ticketDir, "ticketDir");
  const planArtifactPath = local(options.planArtifactPath ?? `${ticketDir}/${TICKET_GENERATED_OUTPUT_DIRECTORY}/📊️taxonomy-plan/🔣️.json`, "planArtifactPath");
  const resumeJournal = options.resumeJournal === undefined ? undefined : local(options.resumeJournal, "resumeJournal");
  if (options.cancelFile !== undefined) local(options.cancelFile, "cancelFile");
  if (options.explicitTicketDir !== undefined) add(local(options.explicitTicketDir, "explicitTicketDir"), "subtree", "Explicit source ticket");
  add(plan.scope ?? "", "subtree", "Plan scope");
  for (const move of plan.moves) {
    add(move.sourcePath, "subtree", "Move source"); add(move.destinationPath, "subtree", "Move destination");
    for (const input of move.sourceAuthority?.inputs ?? []) add(input.path, "point", "Move source authority");
    for (const edit of move.referenceEdits) add(edit.path, "point", "Move reference");
  }
  for (const root of plan.embeddedTicketRoots) for (const path of [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]) add(path, "subtree", "Embedded root");
  for (const relocation of plan.embeddedTicketRootRelocations) for (const path of [relocation.sourcePath, relocation.destinationPath]) add(path, "subtree", "Embedded relocation");
  for (const edit of plan.symlinkTargetEdits) {
    add(edit.sourcePath, "subtree", "Symlink source"); add(edit.finalPath, "subtree", "Symlink destination");
    add(edit.logicalTargetSourcePath, "point", "Symlink logical source"); add(edit.logicalTargetFinalPath, "point", "Symlink logical destination");
  }
  for (const removal of plan.evidenceRemovals) {
    add(removal.sourcePath, "subtree", "Evidence removal");
    for (const path of removalAuthorityPaths(removal.authority)) add(path, "point", "Removal authority");
  }
  for (const ancestor of plan.destinationAncestorPreimages) add(ancestor.path, "point", "Destination ancestor metadata");
  for (const edit of plan.edits) add(edit.path, "subtree", "Reference edit");
  for (const regeneration of plan.regenerations) {
    add(regeneration.cwd, "point", "Generator working directory");
    add(regeneration.cwd + "/📋️project.json", "point", "Generator project manifest");
    for (const root of regeneration.outputRoots) add(root, "subtree", "Generator output root");
    for (const input of regeneration.inputs) add(input.path, "input", "Generator input");
    for (const output of [...regeneration.preOutputs, ...regeneration.outputs]) add(output.path, "subtree", "Generator output");
    for (const path of regeneration.staleRemovals) add(path, "subtree", "Generator stale removal");
    for (const node of regeneration.preview.nodes) add(node.path, "subtree", "Generator preview");
    for (const path of regeneration.preview.staleRemovals) add(path, "subtree", "Generator preview stale removal");
  }
  return { taxonomyPath, ticketDir, planArtifactPath, resumeJournal, accesses };
}
