import { createHash } from "node:crypto";
import { existsSync, lstatSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { loadTaxonomy } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { fileKindIdForSourcePath, semanticOwnedInputFileSnapshot } from "../../../🔍️discovery/🟦️.ts";
import { inventoryTaxonomySources, type TaxonomySourceInventory } from "../../🟦️.ts";
import { POLICY_MUTATIONS_FACET } from "../🪪️identity/🟦️.ts";
import type { MutationTaxonomySourceIndex } from "../📇️index/🟦️.ts";

export interface MutationTaxonomyAssignmentRow { readonly mutationRootPath: string; readonly targetMutationDirectoryName: string; readonly lunaAuditor: string; readonly terraExecutor: string; readonly rootIntegrator: string }

export interface MutationTaxonomySourceRecord { readonly path: string; readonly sha256: string; readonly role: "source" | "assignment-ledger" | "taxonomy-schema" | "mutation-descriptor-schema" }


export interface MutationTaxonomyInventoryOptions {
  readonly afterFactsCollected?: () => void;
  readonly assignmentLedger?: unknown;
  readonly assignmentLedgerPath?: string;
  readonly cancelFile?: string;
  readonly progress?: (event: { readonly operation: "inventory"; readonly phase: string; readonly current: number; readonly total: number; readonly path?: string }) => void;
  readonly scope?: string;
  readonly explicitTicketDir?: string;
}


export type MutationTaxonomyCapturedSchema = { readonly path: string; readonly bytes: Buffer; readonly sha256: string };

export type MutationTaxonomyStructuralDirectory = { readonly path: string; readonly supportingObservations: readonly string[] };


export type MutationTaxonomyStructuralSourceView = Pick<MutationTaxonomySourceIndex, "admission" | "roots" | "files" | "bytes" | "contents" | "directories" | "taxonomySchema" | "mutationDescriptorSchema">;


export function mutationTaxonomyCompare(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}


export function mutationTaxonomyCapturedSchema(repoRoot: string, path: string, label: string): MutationTaxonomyCapturedSchema {
  const snapshot = semanticOwnedInputFileSnapshot(repoRoot, path);
  if (!snapshot) throw new Error(`[clean taxonomy --kind mutation] captured ${label} is absent: ${path}.`);
  const bytes = Buffer.from(snapshot.bytes), sha256 = createHash("sha256").update(bytes).digest("hex");
  return { path, bytes, sha256 };
}


export function mutationTaxonomyStructuralDirectories(admission: TaxonomySourceInventory): ReadonlyMap<string, MutationTaxonomyStructuralDirectory> {
  const supporting = new Map<string, Set<string>>();
  const add = (path: string, observation: string): void => {
    if (!path) return;
    const observations = supporting.get(path);
    if (observations) observations.add(observation);
    else supporting.set(path, new Set([observation]));
  };
  for (const observation of admission.observations) {
    if (observation.repositoryBoundary === "gitlink") continue;
    if (observation.observedKind === "directory") add(observation.sourcePath, observation.sourcePath);
    if (observation.observedKind !== "file" || (observation.worktreeMode !== "100644" && observation.worktreeMode !== "100755")) continue;
    const segments = observation.sourcePath.split("/");
    for (let length = 1; length < segments.length; length += 1) add(segments.slice(0, length).join("/"), observation.sourcePath);
  }
  const directories: [string, MutationTaxonomyStructuralDirectory][] = [...supporting].map(([path, observations]): [string, MutationTaxonomyStructuralDirectory] => [path, { path, supportingObservations: [...observations].sort(mutationTaxonomyCompare) }]);
  directories.sort(([left], [right]) => mutationTaxonomyCompare(left, right));
  return new Map(directories);
}


export function mutationTaxonomyStructuralView(index: MutationTaxonomySourceIndex): MutationTaxonomyStructuralSourceView {
  return { admission: index.admission, roots: index.roots, files: index.files, bytes: index.bytes, contents: index.contents, directories: index.directories, taxonomySchema: index.taxonomySchema, mutationDescriptorSchema: index.mutationDescriptorSchema };
}


export function mutationTaxonomyCancelled(repoRoot: string, options: MutationTaxonomyInventoryOptions): void {
  if (!options.cancelFile) return;
  const path = mutationTaxonomyInputPath(repoRoot, options.cancelFile);
  if (existsSync(path)) throw new Error("[clean taxonomy --kind mutation] cancelled during inventory.");
}


export function mutationTaxonomyInputPath(repoRoot: string, input: string): string {
  const lexical = input.replaceAll("\\", "/");
  if (lexical.split("/").some((segment) => segment.toLocaleLowerCase("en-US") === "compose")) throw new Error(`[clean taxonomy --kind mutation] opaque or escaping input path: ${input}.`);
  const path = resolve(repoRoot, input);
  const rel = relative(repoRoot, path).replaceAll("\\", "/");
  if (rel === "" || rel === ".." || rel.startsWith("../") || rel === "compose" || rel.startsWith("compose/")) throw new Error(`[clean taxonomy --kind mutation] opaque or escaping input path: ${input}.`);
  let current = repoRoot;
  for (const segment of rel.split("/")) {
    current = join(current, segment);
    try { if (lstatSync(current).isSymbolicLink()) throw new Error(`[clean taxonomy --kind mutation] input path must not traverse symlink ${input}.`); } catch (error) { if (error instanceof Error && !/ENOENT/u.test(String((error as NodeJS.ErrnoException).code ?? error.message))) throw error; }
  }
  return path;
}


export function mutationTaxonomyScope(scope: string): string {
  const normalized = scope.replaceAll("\\", "/");
  if (!normalized || isAbsolute(scope) || /^[A-Za-z]:[\\/]/u.test(scope) || normalized === "." || normalized === ".." || normalized.startsWith("../") || normalized.includes("/../") || normalized.split("/").some((segment) => segment.toLocaleLowerCase("en-US") === "compose")) throw new Error(`[clean taxonomy --kind mutation] source scope is opaque or non-relative: ${scope}.`);
  return normalized;
}


/** 🧾️ Obtains one closed repository membership authority; callers may inject it for pure projection. */
export function mutationTaxonomySourceAdmission(repoRoot: string, options: MutationTaxonomyInventoryOptions): TaxonomySourceInventory {
  const admission = inventoryTaxonomySources({
    repoRoot,
    ...(options.explicitTicketDir ? { ticketDir: options.explicitTicketDir } : {}),
    ...(options.cancelFile ? { cancelFile: options.cancelFile } : {}),
    ...(options.progress ? { progress: (event) => options.progress!({ operation: "inventory", phase: event.phase, current: event.current, total: event.total, ...(event.path ? { path: event.path } : {}) }) } : {}),
  });
  if (admission.status !== "complete") throw new Error(`[clean taxonomy --kind mutation] source admission is rejected: ${admission.diagnostics.map((diagnostic) => `${diagnostic.code}:${diagnostic.path}`).join(", ") || "unknown admission failure"}.`);
  return admission;
}


/** 🧾️ Selects only admitted regular files; this projection never traverses or reads the filesystem. */
export function mutationTaxonomySourceFiles(admission: TaxonomySourceInventory): string[] {
  if (admission.status !== "complete") throw new Error("[clean taxonomy --kind mutation] source admission is rejected.");
  return admission.observations
    .filter((observation) => observation.repositoryBoundary !== "gitlink" && observation.observedKind === "file" && (observation.worktreeMode === "100644" || observation.worktreeMode === "100755"))
    .map((observation) => observation.sourcePath)
    .sort(mutationTaxonomyCompare);
}


//#region 🧬️SourceFileFacts
export interface MutationTaxonomySourceFileFact {
  readonly sourcePath: string;
  readonly fileKindId: string | null;
  readonly fileRole: ReturnType<typeof loadTaxonomy>["fileKinds"][string]["role"] | null;
}


/** 🔬️ Classifies each admitted regular file without losing unknown kinds or raw path spelling. */
export function mutationTaxonomySourceFileFacts(admission: TaxonomySourceInventory, taxonomy: ReturnType<typeof loadTaxonomy>): readonly MutationTaxonomySourceFileFact[] {
  return mutationTaxonomySourceFiles(admission).map((sourcePath) => {
    const fileKindId = fileKindIdForSourcePath(sourcePath, taxonomy);
    return { sourcePath, fileKindId, fileRole: fileKindId === null ? null : taxonomy.fileKinds[fileKindId]!.role };
  });
}


export function policyFindAllMutationsDirs(repoRoot: string, admission: TaxonomySourceInventory = mutationTaxonomySourceAdmission(repoRoot, {})): string[] {
  if (admission.status !== "complete") throw new Error("[clean taxonomy --kind mutation] source admission is rejected.");
  const found: string[] = [];
  for (const observation of admission.observations) {
    if (observation.repositoryBoundary === "gitlink") continue;
    if (observation.observedKind !== "file" && observation.observedKind !== "directory") continue;
    const segments = observation.sourcePath.split("/");
    for (let index = 0; index < segments.length; index += 1) {
      if (observation.observedKind === "file" && index === segments.length - 1) continue;
      if (segments[index] === POLICY_MUTATIONS_FACET) found.push(segments.slice(0, index + 1).join("/"));
    }
  }
  return [...new Set(found)].sort(mutationTaxonomyCompare);
}


export const MUTATION_DESCRIPTOR_SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json";
