import { requireRecord, type UnknownRecord } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type SemanticPathProjectionContract } from "../🔣️taxonomy/🧭️path-projection/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { parseCurrentSourceApprovals, semanticExactOwnedFileCatalog } from "../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { semanticPackageIgnoredGeneratedOutputPaths, semanticPackageProjectionAuthority, semanticPackageProjectionCatalog } from "../../🔍️discovery/🎁️package-projection/🟦️.ts";
import { type SemanticPackageProjectionCatalog, type OwnerProjectionEntry, type OwnerProjectionPolicy } from "../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";
import { type SemanticOwnedInputFileSnapshot } from "../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";
import { mutationPayloadSchemaProblems } from "../../🔍️discovery/📄️implementation-findings/🟦️.ts";
import { jsonDocumentDuplicateKeys } from "../../🧬️schema/🔣️json-document/🟦️.ts";
import { mutationOwnerIdentity } from "../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { basename, dirname, posix } from "node:path";
import { type SemanticProjectionAuthorityNode } from "../../🔍️discovery/🧱️path-projection-contracts/🟦️.ts";
import { TICKET_IMPORTANT_EXACT_GOVERNED_SOURCES, TICKET_IMPORTANT_EXACT_MUTATIONS_CATALOG_PATH, type TaxonomyInventoryEntry, type TaxonomyLeafPreimage, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, isExcluded, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare, parseLeafPreimage, planPath, planRecord, planString } from "../🧾️plan-parsing/🟦️.ts";
import type { MutableInventoryEntry } from "../📸️repository-capture/🟦️.ts";
import { entrySubtree, pathPolicyViolations } from "../👥️mutation-projection/🟦️.ts";

export type ArtifactProjectionRationale = "artifact-example-model-catalog-projection-v1" | "artifact-editor-command-projection-v1";

export interface ArtifactProjectionLocation {
  readonly artifactRoot: string;
  readonly sourceRoot: string;
}

export function artifactProjectionContracts(taxonomy: LoadedTaxonomy): readonly Readonly<{ id: string; contract: SemanticPathProjectionContract & { readonly sourceArtifactMemberName: string; readonly rationaleRule: ArtifactProjectionRationale } }>[] {
  return Object.entries(taxonomy.schema.semanticPathProjectionContracts)
    .filter((entry): entry is [string, SemanticPathProjectionContract & { readonly sourceArtifactMemberName: string; readonly rationaleRule: ArtifactProjectionRationale }] => entry[1].sourceArtifactMemberName !== undefined && (entry[1].rationaleRule === "artifact-example-model-catalog-projection-v1" || entry[1].rationaleRule === "artifact-editor-command-projection-v1"))
    .map(([id, contract]) => ({ id, contract }))
    .sort((left, right) => left.id.localeCompare(right.id));
}

export function artifactProjectionAuthorityNodes(repoRoot: string, sourceRoot: string, entries: ReadonlyMap<string, Pick<TaxonomyInventoryEntry, "sourcePath" | "nodeKind">>, taxonomy: LoadedTaxonomy): readonly SemanticProjectionAuthorityNode[] {
  const nodes: SemanticProjectionAuthorityNode[] = [];
  for (const entry of entrySubtree(entries, sourceRoot).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath))) {
    if (isExcluded(entry.sourcePath, taxonomy)) throw new Error(`Artifact projection crosses opaque path ${entry.sourcePath}`);
    if (entry.nodeKind !== "file") {
      nodes.push({ path: entry.sourcePath, nodeKind: entry.nodeKind });
      continue;
    }
    let content: string;
    try {
      content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolutePath(repoRoot, entry.sourcePath)));
    } catch {
      throw new Error(`Artifact projection source is not readable UTF-8: ${entry.sourcePath}`);
    }
    nodes.push({ path: entry.sourcePath, nodeKind: "file", content });
  }
  return nodes;
}

function commonProjectionDirectory(paths: readonly string[], floor: string): string {
  if (paths.length === 0) return floor;
  const parts = paths.map((path) => path.split("/"));
  const common: string[] = [];
  for (let index = 0; index < Math.min(...parts.map((row) => row.length)); index++) {
    const segment = requireTaxonomyValue(requireTaxonomyValue(parts[0],"parts[0]")[index],"parts[0][index]");
    if (!parts.every((row) => row[index] === segment)) break;
    common.push(segment);
  }
  const result = common.join("/");
  return result === floor || result.startsWith(`${floor}/`) ? result : floor;
}

export function artifactProjectionDirectoryMappings(sourceRoot: string, destinationRoot: string, mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[], entries: ReadonlyMap<string, MutableInventoryEntry>): ReadonlyMap<string, string> {
  const result = new Map<string, string>([[sourceRoot, destinationRoot]]);
  const directories = entrySubtree(entries, sourceRoot).filter((entry) => entry.nodeKind === "directory" && entry.sourcePath !== sourceRoot).sort((left, right) => left.sourcePath.split("/").length - right.sourcePath.split("/").length || generatorPathCompare(left.sourcePath, right.sourcePath));
  for (const entry of directories) {
    const descendants = mappings.filter((mapping) => mapping.sourcePath.startsWith(`${entry.sourcePath}/`)).map((mapping) => dirname(mapping.destinationPath));
    if (descendants.length === 0) continue;
    const parentDestination = result.get(dirname(entry.sourcePath)) ?? destinationRoot;
    const common = commonProjectionDirectory(descendants, parentDestination);
    const commonSegments = common.split("/");
    const parentSegments = parentDestination.split("/");
    let destination = common;
    for (let index = commonSegments.length - 1; index >= parentSegments.length; index--) if (commonSegments[index] === basename(entry.sourcePath)) {
      destination = commonSegments.slice(0, index + 1).join("/");
      break;
    }
    result.set(entry.sourcePath, destination);
  }
  return result;
}

export function applyArtifactProjectionPath(entry: MutableInventoryEntry, destinationPath: string, taxonomy: LoadedTaxonomy): void {
  entry.normalizedPath = destinationPath.normalize("NFC");
  const superseded = new Set(["directory-kind-ambiguous", "directory-kind-unresolved", "file-kind-ambiguous", "file-kind-unresolved", "semantic-stem-ambiguous", "semantic-stem-unresolved", "path-too-long", "path-component-too-long", "trailing-dot-or-space", "windows-reserved-name"]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}

/** 📚️ The nested Cargo package catalog with its admission problem instead of a throw: `semanticPackageProjectionCatalog` refuses a catalog
 * whose bytes or census drifted from the registration; inventory reports that as a violation and planning proceeds without the catalog. */
function nestedCargoCatalogRecord(repoRoot: string, taxonomy: LoadedTaxonomy["discoverySchema"]): Readonly<{ catalog: SemanticPackageProjectionCatalog | null; problem: string | null }> {
  try { return { catalog: semanticPackageProjectionCatalog(repoRoot, taxonomy), problem: null }; }
  catch (error) {
    if (!(error instanceof Error) || !error.message.startsWith("Nested Cargo ")) throw error;
    return { catalog: null, problem: error.message };
  }
}

/** 📦️ The admitted nested Cargo package catalog, or `null` when it is absent or refused (the refusal is an inventory violation). */
export function nestedCargoCatalog(repoRoot: string, taxonomy: LoadedTaxonomy["discoverySchema"]): SemanticPackageProjectionCatalog | null {
  return nestedCargoCatalogRecord(repoRoot, taxonomy).catalog;
}

export function projectNestedCargoPackages(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy, prunableSourceParents: ReadonlySet<string>): void {
  const { catalog, problem } = nestedCargoCatalogRecord(repoRoot, taxonomy.discoverySchema), catalogPath = requireTaxonomyValue(taxonomy.discoverySchema.semanticPackageProjectionContracts["nested-cargo-packages-v1"],"taxonomy.discoverySchema.semanticPackageProjectionContracts[\"nested-cargo-packages-v1\"]").authorityCatalogPath;
  if (problem) entries.get(catalogPath)?.violations.push(violation("nested-cargo-catalog-invalid", catalogPath, problem));
  if (!catalog) return;
  for (const row of catalog.packages) {
    const sourceManifest = `${row.sourceRoot}/Cargo.toml`, destinationManifest = `${row.destinationRoot}/Cargo.toml`;
    if (!entries.has(sourceManifest) && !entries.has(destinationManifest)) continue;
    const destination = entries.has(destinationManifest), root = destination ? row.semanticOwnerRoot : row.sourceRoot;
    const candidates = entrySubtree(entries, root).filter((entry) => !(entry.nodeKind === "directory" && prunableSourceParents.has(entry.sourcePath)));
    const anchor = entries.get(destination ? destinationManifest : sourceManifest)!;
    try {
      const nodes = candidates.map((entry): SemanticProjectionAuthorityNode => ({ path: entry.sourcePath, nodeKind: entry.nodeKind, ...(entry.nodeKind === "file" ? { content: readFileSync(assertLexicalInputOutsideOpaque(repoRoot, entry.sourcePath, "Nested Cargo source", true), "utf8") } : {}) }));
      const admitted = new Set(candidates.map((entry) => entry.sourcePath));
      const ignoredOutputParents = new Set(destination ? semanticPackageIgnoredGeneratedOutputPaths(row, taxonomy.discoverySchema).map((path) => posix.dirname(path)) : []);
      for (const directory of candidates.filter((entry) => entry.nodeKind === "directory")) {
        for (const name of readdirSync(assertLexicalInputOutsideOpaque(repoRoot, directory.sourcePath, "Nested Cargo directory", true))) {
          const path = directory.sourcePath + "/" + name;
          if (admitted.has(path)) continue;
          if (ignoredOutputParents.has(path)) {
            const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Nested Cargo ignored output parent", true);
            const stat = lstatOrNull(absolute);
            if (stat?.isDirectory() && !stat.isSymbolicLink() && readdirSync(absolute).length === 0) { nodes.push({ path, nodeKind: "directory" }); continue; }
          }
          if (prunableSourceParents.has(path)) {
            if (!lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, path, "Nested Cargo prunable source parent", true))?.isDirectory()) throw new Error("Nested Cargo prunable source parent changed kind: " + path);
            continue;
          }
          const relativePath = path.slice(row.sourceRoot.length + 1);
          if (!destination && row.ignoredSourcePatterns.some((pattern) => taxonomy.pathMatcher.matches(relativePath, pattern) || pattern.endsWith("/**") && relativePath === pattern.slice(0, -3))) continue;
          throw new Error("Nested Cargo has an unadmitted physical child: " + path);
        }
      }
      const occupiedPaths = [...entries.keys()].filter((path) => !admitted.has(path));
      if (!destination) for (const path of new Set([...row.mappings.map((mapping) => mapping.destinationPath), ...row.adapters.map((adapter) => adapter.path), ...row.derivedLeaves.map((leaf) => leaf.path)])) {
        if (lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, path, "Nested Cargo destination", true))) occupiedPaths.push(path);
      }
      const evidence = row.workspaceKind === "repository" ? { cargoWorkspaceContent: readFileSync(assertLexicalInputOutsideOpaque(repoRoot, "Cargo.toml", "Nested Cargo workspace", true), "utf8"), nodeWorkspaceContent: readFileSync(assertLexicalInputOutsideOpaque(repoRoot, "package.json", "Nested Node workspace", true), "utf8") } : {};
      const authority = semanticPackageProjectionAuthority({ packageId: row.id, nodes, layout: destination ? "destination" : "source", occupiedPaths, ...evidence }, catalog, taxonomy.discoverySchema);
      if (authority.problems.length > 0) { anchor.violations.push(violation("nested-cargo-authority-invalid", anchor.sourcePath, authority.problems.join(" | "))); continue; }
      const mappingByPath = new Map(authority.mappings.map((mapping) => [destination ? mapping.destinationPath : mapping.sourcePath, mapping]));
      for (const entry of candidates) {
        const mapping = mappingByPath.get(entry.sourcePath);
        applyArtifactProjectionPath(entry, mapping?.destinationPath ?? (entry.sourcePath === row.sourceRoot && !destination && row.sourceRoot !== row.semanticOwnerRoot ? row.destinationRoot : entry.sourcePath), taxonomy);
        entry.violations = entry.violations.filter((problem) => !["package-role-unresolved", "package-implementation-file", "package-implementation-destination-unresolved"].includes(problem.code));
        if (entry.nodeKind === "file") {
          entry.semanticStem = null;
          entry.packageRole = mapping?.disposition === "implementation" || mapping?.disposition === "generated" || row.derivedLeaves.some((leaf) => leaf.path === entry.sourcePath) ? "implementation" : "configuration";
          if (basename(entry.sourcePath) === "Cargo.toml") entry.fixedContractId = "cargo-manifest";
        }
      }
    } catch (error) { anchor.violations.push(violation("nested-cargo-authority-invalid", anchor.sourcePath, error instanceof Error ? error.message : String(error))); }
  }
}

export function exactOwnedFileCatalog(repoRoot: string, taxonomy: LoadedTaxonomy): OwnerProjectionPolicy | null {
  return semanticExactOwnedFileCatalog(repoRoot, taxonomy.discoverySchema);
}

export interface ExactOwnedCatalogSnapshot {
  readonly catalog: OwnerProjectionPolicy | null;
  readonly input: SemanticOwnedInputFileSnapshot | null;
}

export function exactOwnedCatalogSnapshot(repoRoot: string, taxonomy: LoadedTaxonomy): ExactOwnedCatalogSnapshot {
  let input: SemanticOwnedInputFileSnapshot | null = null;
  const catalog = semanticExactOwnedFileCatalog(repoRoot, taxonomy.discoverySchema, (snapshot) => { input = snapshot; });
  return { catalog, input };
}

export function exactOwnedCurrentRevisions(taxonomy: LoadedTaxonomy): ReturnType<typeof parseCurrentSourceApprovals> | undefined {
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  return contract?.contractKind === "exact-owner-path-catalog" ? contract.currentSourceRevisions : undefined;
}

export function exactOwnedCurrentRawPath(path: string, snapshot: ExactOwnedCatalogSnapshot, taxonomy: LoadedTaxonomy): boolean {
  return Object.values(exactOwnedCurrentRevisions(taxonomy) ?? {}).some((row) => row.sourcePath === path && snapshot.catalog?.entries.some(entry => entry.sourcePath === row.sourcePath) === true);
}

export function exactOwnedEvidenceProblems(repoRoot: string, entry: OwnerProjectionEntry, catalog: OwnerProjectionPolicy): readonly string[] {
  const evidence = requireTaxonomyValue(catalog.ownerEvidence[entry.ownerEvidenceId],"catalog.ownerEvidence[entry.ownerEvidenceId]");
  const problems: string[] = [];
  for (const declared of evidence.evidencePaths) {
    const projected = catalog.entries.find((candidate) => candidate.sourcePath === declared);
    const path = projected && !lstatOrNull(absolutePath(repoRoot, declared)) ? projected.destinationPath : declared;
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Exact owner evidence", true);
    const stat = lstatOrNull(absolute);
    if (!stat?.isFile()) {
      problems.push("Exact owner evidence is absent or not a regular file: " + path);
      continue;
    }
    if (evidence.expectedPackageName && basename(path) === "package.json") {
      let manifest: UnknownRecord;
      try { manifest = requireRecord(JSON.parse(readFileSync(absolute, "utf8")), "Exact publisher manifest"); } catch { problems.push("Exact publisher manifest is invalid: " + path); continue; }
      if (manifest.name !== evidence.expectedPackageName || (manifest.private === true) !== evidence.private) problems.push("Exact publisher identity or private status drifted: " + path);
    }
  }
  return problems;
}

export interface TicketImportantExactMutationAuthority {
  readonly catalogPath: string;
  readonly catalogContentHash: string;
  readonly caseId: string;
  readonly disposition: "move" | "remove";
  readonly sourcePath: string;
  readonly destinationPath: string | null;
  readonly sourcePreimage: Extract<TaxonomyLeafPreimage, { nodeKind: "file" }>;
}

export function ticketImportantExactMutationCases(repoRoot: string): readonly TicketImportantExactMutationAuthority[] {
  const catalogPath = TICKET_IMPORTANT_EXACT_MUTATIONS_CATALOG_PATH;
  const catalog = absolutePath(repoRoot, catalogPath);
  const catalogStat = lstatOrNull(catalog);
  if (!catalogStat) {
    const governed = TICKET_IMPORTANT_EXACT_GOVERNED_SOURCES.find((path) => lstatOrNull(absolutePath(repoRoot, path)) !== null);
    if (governed) throw new Error(`Ticket important exact mutation catalog is absent for governed source ${governed}`);
    return [];
  }
  if (!catalogStat.isFile() || catalogStat.isSymbolicLink()) throw new Error("Ticket important exact mutation catalog must be a regular file");
  const bytes = readFileSync(catalog);
  const root = planRecord(JSON.parse(bytes.toString("utf8")) as unknown, "ticket important exact mutation catalog", ["schemaVersion", "cases"]);
  if (root.schemaVersion !== 1 || !Array.isArray(root.cases)) throw new Error("Ticket important exact mutation catalog schema is invalid");
  const catalogContentHash = sha256(bytes);
  const rows = root.cases.map((value, index) => {
    const row = planRecord(value, `ticket important exact mutation catalog cases[${index}]`, ["id", "disposition", "sourcePath", "destinationPath", "sourcePreimage"]);
    if (row.disposition !== "move" && row.disposition !== "remove") throw new Error(`Ticket important exact mutation catalog case ${index} has an invalid disposition`);
    const sourcePreimage = parseLeafPreimage(row.sourcePreimage, `ticket important exact mutation catalog cases[${index}].sourcePreimage`);
    if (sourcePreimage.nodeKind !== "file" || row.disposition === "remove" && (sourcePreimage.size !== 0 || row.destinationPath !== null) || row.disposition === "move" && typeof row.destinationPath !== "string") throw new Error(`Ticket important exact mutation catalog case ${index} has invalid leaf evidence`);
    return { catalogPath, catalogContentHash, caseId: planString(row.id, `ticket important exact mutation catalog cases[${index}].id`), disposition: row.disposition, sourcePath: planPath(row.sourcePath, `ticket important exact mutation catalog cases[${index}].sourcePath`), destinationPath: row.destinationPath === null ? null : planPath(row.destinationPath, `ticket important exact mutation catalog cases[${index}].destinationPath`), sourcePreimage } as TicketImportantExactMutationAuthority;
  });
  if (new Set(rows.map((entry) => entry.caseId)).size !== rows.length || new Set(rows.map((entry) => entry.sourcePath)).size !== rows.length || rows.some((entry, index) => index > 0 && generatorPathCompare(requireTaxonomyValue(rows[index - 1],"rows[index - 1]").caseId, entry.caseId) >= 0)) throw new Error("Ticket important exact mutation catalog identities must be unique and bytewise sorted");
  return rows;
}

/** 🧩️ Resolves only a frozen exact ticket-important physical mutation with complete leaf evidence. */
export function ticketImportantExactMutationAuthority(repoRoot: string, sourcePath: string, sourcePreimage: TaxonomyLeafPreimage): TicketImportantExactMutationAuthority | null {
  return ticketImportantExactMutationCases(repoRoot).find((entry) => entry.sourcePath === sourcePath && canonicalJson(entry.sourcePreimage) === canonicalJson(sourcePreimage)) ?? null;
}

export function ticketManifestState(content: string | undefined): "closed" | "invalid" | "missing" | "open" {
  if (content === undefined) return "missing";
  try {
    const value = JSON.parse(content) as unknown;
    const status = typeof value === "object" && value !== null && !Array.isArray(value) ? (value as UnknownRecord).status : undefined;
    return status === "closed" || status === "open" ? status : "invalid";
  } catch { return "invalid"; }
}

export function validateTicketImportantDirectories(repoRoot: string, entries: Map<string, MutableInventoryEntry>, directoryKindByPath: ReadonlyMap<string, string>, fixedDirectoryContractByPath: ReadonlyMap<string, string>): void {
  const directories = [...entries.values()].filter((entry) => entry.nodeKind === "directory" && ["ticket-important", "ticket-important-history"].includes(directoryKindByPath.get(entry.sourcePath) ?? ""));
  const childrenByDirectory = new Map(directories.map((directory) => [directory.sourcePath, [] as MutableInventoryEntry[]]));
  for (const entry of entries.values()) childrenByDirectory.get(dirname(entry.sourcePath))?.push(entry);
  for (const directory of directories) {
    const kindId = directoryKindByPath.get(directory.sourcePath)!;
    const ownerPath = dirname(directory.sourcePath);
    const children = childrenByDirectory.get(directory.sourcePath)!;
    const leaf = children.length === 1 && children[0]?.nodeKind === "file" && basename(children[0].sourcePath) === "📝️.md" ? children[0] : undefined;
    const manifestPath = `${ownerPath}/🎫️ticket.json`;
    const manifest = entries.get(manifestPath);
    const manifestContent = manifest?.nodeKind === "file" && manifest.fixedContractId === "ticket-manifest" ? readFileSync(absolutePath(repoRoot, manifestPath), "utf8") : undefined;
    const manifestState = ticketManifestState(manifestContent);
    const ownerClaimed = fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug";
    const rawAbsent = !entries.has(`${ownerPath}/📌️important.md`);
    const lifecycleAdmitted = kindId === "ticket-important" && manifestState === "open";
    const historyAdmitted = kindId === "ticket-important-history" && (manifestState === "missing" || manifestState === "invalid" || manifestState === "closed" && Boolean(leaf && leaf.size > 0));
    if (ownerClaimed && leaf && rawAbsent && (lifecycleAdmitted || historyAdmitted)) continue;
    const problem = violation("projection-only-owner-invalid", directory.sourcePath, `${kindId} must be an exact ticket-owned lifecycle/history directory with one Markdown leaf and no raw sibling`);
    directory.violations.push(problem);
    for (const child of children) child.violations.push({ ...problem, path: child.sourcePath });
  }
}

/** 🛂️ Whether the direct children of one `🧬️mutations` root are payload owners that must carry their own canonical descriptor. A fixture mirror (`🧪️tests/🧫️fixtures/🧬️mutations`) and a codec projection (`🚪️io/<representation>/🧬️mutations`) consume the schema-side owner, which holds the descriptor. @see https://json-schema.org/draft-07/schema */
export function mutationOwnerRootHoldsDescriptors(root: string, taxonomy: Pick<LoadedTaxonomy["discoverySchema"], "testFixturesDirName" | "representationDirs">): boolean {
  const parent = basename(dirname(root));
  return parent !== taxonomy.testFixturesDirName && !taxonomy.representationDirs.includes(parent);
}

/** 🪢️ Validates each descriptor's authored schema pointer without relocating its payload file. */
export function validateMutationPayloadSchemas(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  const contract = taxonomy.discoverySchema.mutationPayloadSchemaAuthority;
  const descriptorKind = taxonomy.discoverySchema.fileKinds[contract.descriptorFileKindId]!;
  const descriptorFilename = `${descriptorKind.emoji}${descriptorKind.extensionChains[0]}`;
  const owners = new Map<string, { identity: string; children: MutableInventoryEntry[] }>();
  for (const entry of entries.values()) {
    const owner = entry.nodeKind === "directory" ? entry.sourcePath : dirname(entry.sourcePath), marker = owner.lastIndexOf("/🧬️mutations/");
    if (marker < 0) continue;
    const root = owner.slice(0, marker + "/🧬️mutations".length);
    if (!mutationOwnerRootHoldsDescriptors(root, taxonomy.discoverySchema)) continue;
    const identity = mutationOwnerIdentity(root, owner.slice(root.length + 1), taxonomy.discoverySchema);
    if (identity === null) continue;
    const state = owners.get(owner) ?? { identity, children: [] };
    state.children.push(entry);
    owners.set(owner, state);
  }
  for (const [owner, { identity, children }] of owners) {
    const descriptorPath = `${owner}/${descriptorFilename}`, descriptorEntry = entries.get(descriptorPath);
    const entry = descriptorEntry ?? entries.get(owner) ?? children[0]!;
    try {
      if (descriptorEntry?.nodeKind !== "file") throw new Error("Mutation owner requires one canonical regular-file descriptor");
      const content = (path: string): string => {
        const bytes = readFileSync(assertLexicalInputOutsideOpaque(repoRoot, path, "Mutation payload authority", true)), value = bytes.toString("utf8");
        if (!Buffer.from(value, "utf8").equals(bytes)) throw new Error("Payload authority must be exact UTF-8");
        return value;
      };
      for (const child of children) {
        if (child.nodeKind !== "file" || child.sourcePath === descriptorPath || !child.sourcePath.endsWith(".json")) continue;
        let other: unknown;
        try { other = JSON.parse(content(child.sourcePath)); } catch { continue; }
        if (other !== null && typeof other === "object" && !Array.isArray(other) && ["schemaVersion", contract.descriptorOwnerField, contract.descriptorIdentityField, contract.descriptorField].every((field) => Object.hasOwn(other, field))) throw new Error("Mutation owner contains a competing descriptor");
      }
      const descriptorContent = content(descriptorPath), descriptor = requireRecord(JSON.parse(descriptorContent), "Mutation payload descriptor"), pointer = descriptor[contract.descriptorField];
      if (!(Array.isArray(descriptor.requiredLanguageSurfaces) && descriptor.requiredLanguageSurfaces.includes("json-schema")) && !(typeof pointer === "string" && pointer.endsWith(".json"))) continue;
      if (jsonDocumentDuplicateKeys(descriptorContent).length > 0) throw new Error("Mutation payload descriptor has duplicate JSON members");
      if (descriptor.schemaVersion !== contract.descriptorSchemaVersion || descriptor[contract.descriptorOwnerField] !== owner || descriptor[contract.descriptorIdentityField] !== identity) throw new Error("Descriptor version and semantic identity must belong to the exact source owner");
      const problems = mutationPayloadSchemaProblems(owner, pointer, (path) => {
        const source = entries.get(path);
        return { kind: source?.nodeKind ?? "absent", ...(source?.nodeKind === "file" ? { content: content(path) } : {}), repositoryBoundary: isExcluded(path, taxonomy) };
      }, contract.jsonSchemaDialect);
      if (problems.length > 0) throw new Error(problems.join("; "));
    } catch (error) {
      entry.violations.push(violation("mutation-payload-schema-authority-invalid", entry.sourcePath, error instanceof Error ? error.message : String(error)));
    }
  }
}
