import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { relative, sep } from "node:path";
import { canonicalJson, type TaxonomySourceInventory } from "../../🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validation/🟦️.ts";
import { MUTATION_DESCRIPTOR_SCHEMA_REL, mutationTaxonomyCancelled, mutationTaxonomyCapturedSchema, mutationTaxonomyCompare, mutationTaxonomyInputPath, mutationTaxonomyScope, mutationTaxonomySourceAdmission, mutationTaxonomySourceFileFacts, mutationTaxonomyStructuralDirectories, policyFindAllMutationsDirs, type MutationTaxonomyAssignmentRow, type MutationTaxonomyCapturedSchema, type MutationTaxonomyInventoryOptions, type MutationTaxonomySourceRecord, type MutationTaxonomyStructuralDirectory } from "../📸️captured-source/🟦️.ts";
import { getRepoMetaDir, loadTaxonomy } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { semanticOwnedInputFileSnapshot } from "../../../🔍️discovery/🟦️.ts";

export const MUTATION_TAXONOMY_ASSIGNMENT_LEDGER_SCHEMA = {
  type: "object",
  required: ["schemaVersion", "assignments"],
  properties: {
    schemaVersion: { const: 1 },
    assignments: {
      type: "array",
      items: {
        type: "object",
        required: ["mutationRootPath", "targetMutationDirectoryName", "lunaAuditor", "terraExecutor", "rootIntegrator"],
        properties: {
          mutationRootPath: { type: "string", minLength: 1 },
          targetMutationDirectoryName: { type: "string", minLength: 1 },
          lunaAuditor: { type: "string", minLength: 1 },
          terraExecutor: { type: "string", minLength: 1 },
          rootIntegrator: { type: "string", minLength: 1 },
        },
        additionalProperties: false,
      },
    },
  },
  additionalProperties: false,
} as const;

export type MutationTaxonomySourceIndex = { readonly admission: TaxonomySourceInventory; readonly roots: readonly string[]; readonly files: readonly string[]; readonly bytes: ReadonlyMap<string, Buffer>; readonly contents: ReadonlyMap<string, string>; readonly directories: ReadonlyMap<string, MutationTaxonomyStructuralDirectory>; readonly taxonomySchema: MutationTaxonomyCapturedSchema; readonly mutationDescriptorSchema: MutationTaxonomyCapturedSchema; readonly sourceRoster: readonly MutationTaxonomySourceRecord[]; readonly sourceTreeDigest: string; readonly ledger: { readonly path: string | null; readonly rows: readonly MutationTaxonomyAssignmentRow[]; readonly invalidReason: string | null } };

//#endregion 🧬️SourceFileFacts

export function mutationTaxonomyAssignmentLedger(repoRoot: string, options: MutationTaxonomyInventoryOptions): { readonly path: string | null; readonly bytes: Buffer | null; readonly rows: readonly MutationTaxonomyAssignmentRow[]; readonly invalidReason: string | null } {
  const path = options.assignmentLedger === undefined ? options.assignmentLedgerPath ?? null : null;
  let raw = options.assignmentLedger;
  let bytes: Buffer | null = raw === undefined ? null : Buffer.from(canonicalJson(raw));
  if (raw === undefined && path) {
    let resolved: string;
    try { resolved = mutationTaxonomyInputPath(repoRoot, path); } catch (error) { return { path, bytes, rows: [], invalidReason: error instanceof Error ? error.message : "Assignment ledger path is invalid." }; }
    if (existsSync(resolved)) {
      const content = readFileSync(resolved);
      bytes = content;
      try { raw = JSON.parse(content.toString("utf8")); } catch { return { path, bytes, rows: [], invalidReason: "Assignment ledger is not valid JSON." }; }
    }
  }
  if (raw === undefined) return { path, bytes: null, rows: [], invalidReason: null };
  const errors = validateJsonSchemaSubset(MUTATION_TAXONOMY_ASSIGNMENT_LEDGER_SCHEMA, raw);
  if (errors.length > 0) return { path, bytes, rows: [], invalidReason: `Assignment ledger violates its schema: ${errors.join("; ")}` };
  return { path, bytes, rows: (raw as { assignments: MutationTaxonomyAssignmentRow[] }).assignments, invalidReason: null };
}


export function mutationTaxonomySourceIndex(repoRoot: string, options: MutationTaxonomyInventoryOptions, injectedAdmission?: TaxonomySourceInventory): MutationTaxonomySourceIndex {
  const admission = injectedAdmission ?? mutationTaxonomySourceAdmission(repoRoot, options);
  if (admission.status !== "complete") throw new Error("[clean taxonomy --kind mutation] source admission is rejected.");
  const taxonomySchema = mutationTaxonomyCapturedSchema(repoRoot, admission.taxonomyPath, "taxonomy schema");
  if (taxonomySchema.sha256 !== admission.taxonomyContentHash) throw new Error(`[clean taxonomy --kind mutation] captured taxonomy schema hash disagrees with admission: ${admission.taxonomyPath}.`);
  const taxonomy = JSON.parse(taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>;
  const mutationDescriptorSchema = mutationTaxonomyCapturedSchema(repoRoot, MUTATION_DESCRIPTOR_SCHEMA_REL, "mutation descriptor schema");
  const allRoots = policyFindAllMutationsDirs(repoRoot, admission).sort(mutationTaxonomyCompare);
  const scopes = (options.scope ?? "").split(",").map((scope) => scope.trim()).filter(Boolean).map(mutationTaxonomyScope);
  const roots = scopes.length === 0 ? allRoots : allRoots.filter((root) => scopes.some((scope) => root === scope || root.startsWith(`${scope}/`) || scope.startsWith(`${root}/`)));
  const excludedSourcePaths = new Set((options.excludedSourcePaths ?? []).map(mutationTaxonomyScope));
  const ticketPrefix = `${relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/")}/🎫️tickets/`;
  const retainedSource = (path: string): boolean => !excludedSourcePaths.has(path) && !path.startsWith(ticketPrefix);
  const retainedObservations = admission.observations.filter((observation) => retainedSource(observation.sourcePath));
  const membershipDigest = retainedObservations.length === admission.observations.length ? admission.membershipDigest : createHash("sha256").update(canonicalJson({ schemaVersion: admission.schemaVersion, scope: admission.scope, status: admission.status, observations: retainedObservations, diagnostics: admission.diagnostics })).digest("hex");
  const evidenceFile = (path: string): boolean => roots.some((root) => path.startsWith(`${root}/`)) || /(?:\.(?:rs|ts|tsx|json|graphql|proto|semio|toml|yaml|yml|md)$|(?:^|\/)(?:package\.json|Cargo\.toml|go\.mod)$)/u.test(path);
  const files = mutationTaxonomySourceFileFacts(admission, taxonomy).filter((fact) => retainedSource(fact.sourcePath) && (evidenceFile(fact.sourcePath) || fact.fileRole === "source" || fact.fileRole === "schema" || fact.fileRole === "specification")).map((fact) => fact.sourcePath);
  const bytes = new Map<string, Buffer>();
  const contents = new Map<string, string>();
  const initialBytes = new Map<string, Buffer>([[taxonomySchema.path, taxonomySchema.bytes], [mutationDescriptorSchema.path, mutationDescriptorSchema.bytes]]);
  for (const [index, file] of files.entries()) {
    mutationTaxonomyCancelled(repoRoot, options);
    const snapshot = initialBytes.get(file) ?? semanticOwnedInputFileSnapshot(repoRoot, file)?.bytes;
    if (!snapshot) throw new Error(`[clean taxonomy --kind mutation] admitted source disappeared before content capture: ${file}.`);
    const value = Buffer.from(snapshot);
    bytes.set(file, value);
    contents.set(file, value.toString("utf8"));
    options.progress?.({ operation: "inventory", phase: "source-index", current: index + 1, total: files.length, path: file });
  }
  const ledger = mutationTaxonomyAssignmentLedger(repoRoot, options);
  const sourceRoster: MutationTaxonomySourceRecord[] = files.map((path) => ({ path, sha256: createHash("sha256").update(bytes.get(path)!).digest("hex"), role: "source" }));
  sourceRoster.push({ path: taxonomySchema.path, sha256: taxonomySchema.sha256, role: "taxonomy-schema" as const }, { path: mutationDescriptorSchema.path, sha256: mutationDescriptorSchema.sha256, role: "mutation-descriptor-schema" as const });
  if (ledger.bytes) sourceRoster.push({ path: ledger.path ?? "<supplied-assignment-ledger>", sha256: createHash("sha256").update(ledger.bytes).digest("hex"), role: "assignment-ledger" });
  sourceRoster.sort((left, right) => mutationTaxonomyCompare(`${left.role}\0${left.path}`, `${right.role}\0${right.path}`));
  return { admission, roots, files, bytes, contents, directories: mutationTaxonomyStructuralDirectories(admission), taxonomySchema, mutationDescriptorSchema, sourceRoster, sourceTreeDigest: createHash("sha256").update(canonicalJson({ roots, sourceRoster, membershipDigest, taxonomyContentHash: admission.taxonomyContentHash, mutationDescriptorSchemaHash: mutationDescriptorSchema.sha256 })).digest("hex"), ledger: { path: ledger.path, rows: ledger.rows, invalidReason: ledger.invalidReason } };
}


export function mutationTaxonomySourceSnapshot(repoRoot: string, options: MutationTaxonomyInventoryOptions = {}): MutationTaxonomySourceIndex {
  return mutationTaxonomySourceIndex(repoRoot, options);
}
