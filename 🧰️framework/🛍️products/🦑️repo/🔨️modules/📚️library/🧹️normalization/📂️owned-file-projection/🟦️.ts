import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type SemanticPathProjectionContract } from "../🔣️taxonomy/🧭️path-projection/🟦️.ts";
import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { assertLexicalInputOutsideOpaque, lstatOrNull, assertNoFollowAncestors } from "../📁️input/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { semanticOwnedInputFileSnapshot, semanticExactOwnedFileProjectionAuthority } from "../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { artifactPathProjectionCatalogRoots, renderArtifactPathProjectionRoot } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { reservedDocumentationBasename } from "@semio-tech/framework/identity/path";
import { basename, dirname, posix } from "node:path";
import { semanticArtifactEmptyFacetProjectionAuthority, semanticOwnedFileHistoryProjectionAuthority, semanticOwnedFileProjectionAuthority, semanticOwnedPrimaryFileProjectionAuthority } from "../../🔍️discovery/🗃️owned-file-projection/🟦️.ts";
import { semanticPathProjectionAuthority } from "../../🔍️discovery/🪞️path-projection/🟦️.ts";
import { type TaxonomyInventoryEntry, type TaxonomyMoveSourceAuthority, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { collisionKey } from "../🏭️generator-inputs/🟦️.ts";
import { type MutableInventoryEntry, packageLocation } from "../📸️repository-capture/🟦️.ts";
import { entrySubtree } from "../👥️mutation-projection/🟦️.ts";
import { type ArtifactProjectionLocation, type ArtifactProjectionRationale, type ExactOwnedCatalogSnapshot, applyArtifactProjectionPath, artifactProjectionAuthorityNodes, artifactProjectionContracts, artifactProjectionDirectoryMappings, exactOwnedCatalogSnapshot, exactOwnedCurrentRawPath, exactOwnedCurrentRevisions, exactOwnedEvidenceProblems, ticketImportantExactMutationAuthority } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { canonicalDirectoryName, inventoryLeafPreimage, leafPreimage } from "../🧐️preimage-assertions/🟦️.ts";

export function artifactProjectionSourceLocation(path: string, contract: SemanticPathProjectionContract & { readonly sourceArtifactMemberName: string }, taxonomy: LoadedTaxonomy): ArtifactProjectionLocation | null {
  const segments = path.split("/");
  for (let artifactIndex = 0; artifactIndex < segments.length; artifactIndex++) {
    if (segments[artifactIndex] !== contract.sourceArtifactMemberName) continue;
    const owner = requireTaxonomyValue(taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId],"source owner registry");
    if (!owner?.memberNames.includes(requireTaxonomyValue(segments[artifactIndex],"segments[artifactIndex]")) || artifactIndex === 0 || segments[artifactIndex - 1] !== canonicalDirectoryName(taxonomy, "artifacts", "artifacts")) continue;
    const sourceNames = segments.slice(artifactIndex + 1, artifactIndex + 1 + contract.sourceSegments.length);
    if (sourceNames.length !== contract.sourceSegments.length) continue;
    const matches = contract.sourceSegments.every((segment, index) => "literal" in segment ? sourceNames[index] === segment.literal : sourceNames[index] !== "");
    if (!matches) continue;
    return { artifactRoot: segments.slice(0, artifactIndex + 1).join("/"), sourceRoot: segments.slice(0, artifactIndex + 1 + contract.sourceSegments.length).join("/") };
  }
  return null;
}

export function projectArtifactCatalogs(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  const projectedSources = new Map<string, string>();
  const projectedDestinations = new Map<string, string>();
  const occupiedPaths = [...entries.keys()];
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const candidates = [...entries.values()]
      .filter((entry) => entry.nodeKind === "directory")
      .map((entry) => ({ entry, location: artifactProjectionSourceLocation(entry.sourcePath, contract, taxonomy) }))
      .filter((row): row is { readonly entry: MutableInventoryEntry; readonly location: ArtifactProjectionLocation } => row.location !== null && row.entry.sourcePath === row.location.sourceRoot)
      .sort((left, right) => generatorPathCompare(left.entry.sourcePath, right.entry.sourcePath));
    for (const { entry: root, location } of candidates) {
      const nodes = artifactProjectionAuthorityNodes(repoRoot, location.sourceRoot, entries, taxonomy);
      const authority = semanticPathProjectionAuthority({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot, nodes, occupiedPaths, occupiedExclusionRoot: location.sourceRoot }, taxonomy.discoverySchema);
      if (authority.problems.length > 0) {
        root.violations.push(violation("projection-authority-invalid", root.sourcePath, `${id}: ${authority.problems.join(" | ")}`));
        continue;
      }
      const accepted = authority.mappings.map((mapping) => ({ sourcePath: normalizeRelative(mapping.sourcePath), destinationPath: normalizeRelative(mapping.destinationPath) }));
      for (const mapping of accepted) {
        const source = entries.get(mapping.sourcePath);
        const priorSource = projectedSources.get(mapping.sourcePath);
        const priorDestination = projectedDestinations.get(mapping.destinationPath);
        if (!source || source.nodeKind !== "file" || priorSource && priorSource !== mapping.destinationPath || priorDestination && priorDestination !== mapping.sourcePath) {
          root.violations.push(violation("projection-mapping-collision", mapping.sourcePath, `${id} does not own one unique admitted source/destination pair`));
          continue;
        }
        projectedSources.set(mapping.sourcePath, mapping.destinationPath);
        projectedDestinations.set(mapping.destinationPath, mapping.sourcePath);
        applyArtifactProjectionPath(source, mapping.destinationPath, taxonomy);
      }
      for (const [sourcePath, destinationPath] of artifactProjectionDirectoryMappings(location.sourceRoot, authority.destinationRoot, accepted, entries)) {
        const source = entries.get(sourcePath);
        if (source) applyArtifactProjectionPath(source, destinationPath, taxonomy);
      }
    }
    const artifactRoots = [...entries.values()].filter((entry) => entry.nodeKind === "directory" && basename(entry.sourcePath) === contract.sourceArtifactMemberName && basename(posix.dirname(entry.sourcePath)) === canonicalDirectoryName(taxonomy, "artifacts", "artifacts"));
    for (const artifact of artifactRoots) {
      const registered = new Map(artifactPathProjectionCatalogRoots(artifact.sourcePath, id, taxonomy.discoverySchema).map((root) => [root.destinationRoot, root]));
      const destinations = entrySubtree(entries, artifact.sourcePath).filter((entry) => {
        if (entry.nodeKind !== "directory" || entry.sourcePath === artifact.sourcePath) return false;
        const names = entry.sourcePath.slice(artifact.sourcePath.length + 1).split("/");
        return names.length === contract.destinationSegments.length && contract.destinationSegments.every((segment, index) => "literal" in segment ? names[index] === segment.literal : names[index] !== "");
      });
      for (const destination of destinations) {
        const root = registered.get(destination.sourcePath);
        if (!root) {
          destination.violations.push(violation("projection-destination-unregistered", destination.sourcePath, `${id} destination has no exact forward profile vector`));
          continue;
        }
        const nodes = artifactProjectionAuthorityNodes(repoRoot, root.destinationRoot, entries, taxonomy);
        const admitted = new Set(nodes.map(({ path }) => path));
        const incomplete = nodes.some((node) => node.nodeKind === "directory" && readdirSync(assertLexicalInputOutsideOpaque(repoRoot, node.path, "Canonical artifact catalog", true)).some((name) => !admitted.has(`${node.path}/${name}`)));
        const authority = semanticPathProjectionAuthority({ artifactRoot: artifact.sourcePath, contractId: id, sourceRoot: root.sourceRoot, nodes, layout: "destination" }, taxonomy.discoverySchema);
        if (incomplete || authority.problems.length > 0) {
          destination.violations.push(violation("projection-authority-invalid", destination.sourcePath, `${id}: ${incomplete ? "Canonical catalog has an unadmitted physical child" : authority.problems.join(" | ")}`));
          continue;
        }
        for (const node of nodes) {
          const entry = entries.get(node.path);
          if (entry) applyArtifactProjectionPath(entry, entry.sourcePath, taxonomy);
        }
      }
    }
  }
}

export function artifactCatalogProjectionRationale(sourcePath: string, destinationPath: string, taxonomy: LoadedTaxonomy): ArtifactProjectionRationale | null {
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const location = artifactProjectionSourceLocation(sourcePath, contract, taxonomy);
    if (!location) continue;
    const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot }, taxonomy.discoverySchema);
    if (rendered.problems.length === 0 && (destinationPath === rendered.destinationRoot || destinationPath.startsWith(`${rendered.destinationRoot}/`))) return contract.rationaleRule;
  }
  return null;
}

export function exactOwnedFileResolution(repoRoot: string, entry: TaxonomyInventoryEntry, snapshot: ExactOwnedCatalogSnapshot, taxonomy: LoadedTaxonomy): Readonly<{ result: ReturnType<typeof semanticExactOwnedFileProjectionAuthority>; sourceAuthority?: TaxonomyMoveSourceAuthority }> {
  const catalog = snapshot.catalog, owner = catalog?.entries.find(row => row.sourcePath === entry.sourcePath || row.destinationPath === entry.sourcePath);
  if (!catalog || !owner) throw new Error("Owner projection policy does not govern this source: " + entry.sourcePath);
  const source = assertLexicalInputOutsideOpaque(repoRoot, owner.sourcePath, "Owner source", true), destination = assertLexicalInputOutsideOpaque(repoRoot, owner.destinationPath, "Owner destination", true);
  const destinationPreimage = lstatOrNull(destination)?.isFile() ? leafPreimage(destination) : undefined;
  const facts = { path: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, sourcePresent: lstatOrNull(source) !== null, destinationPresent: lstatOrNull(destination) !== null, destinationPreimage, occupiedPaths: exactOwnedDestinationOccupancy(repoRoot, owner.destinationPath) };
  if (!exactOwnedCurrentRawPath(entry.sourcePath, snapshot, taxonomy)) return { result: semanticExactOwnedFileProjectionAuthority(catalog, facts) };
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (contract?.contractKind !== "exact-owner-path-catalog" || !contract.currentSourceRevisions || !taxonomy.input || !snapshot.input) throw new Error("Current source is missing its declared policy inputs");
  const selected = Object.values(contract.currentSourceRevisions).find(row => row.sourcePath === owner.sourcePath);
  if (!selected || entry.sourcePath !== owner.sourcePath) throw new Error("Current source does not match its stable policy owner");
  const sourceInput = semanticOwnedInputFileSnapshot(repoRoot, entry.sourcePath);
  if (!sourceInput || canonicalJson(inventoryLeafPreimage(entry)) !== canonicalJson({ nodeKind: "file", contentHash: sourceInput.contentHash, mode: sourceInput.mode, size: sourceInput.size })) throw new Error("Current source changed since inventory: " + entry.sourcePath);
  const approval = semanticOwnedInputFileSnapshot(repoRoot, selected.approvalPath);
  if (!approval) throw new Error("Current source approval is absent: " + selected.approvalPath);
  const result = semanticExactOwnedFileProjectionAuthority(catalog, facts, { contract, revisions: contract.currentSourceRevisions, expectations: [approval] }), authority = result.currentSource;
  if (result.disposition !== "project" || result.problems.length || authority?.disposition !== "revised" || !authority.revisionId || !authority.revisionDigest || authority.sourcePath !== owner.sourcePath) return { result };
  const inputs: TaxonomyMoveSourceAuthority["inputs"] = ([{ role: "schema", input: taxonomy.input }, { role: "catalog", input: snapshot.input }, { role: "approval", input: approval }] as const).map(({ role, input }) => ({ role, path: input.path, preimage: { nodeKind: "file" as const, contentHash: input.contentHash, mode: input.mode, size: input.size } })).sort((left, right) => generatorPathCompare(left.path, right.path));
  return { result, sourceAuthority: { kind: "exact-owner-current-source-approval-v1", revisionId: authority.revisionId, revisionDigest: authority.revisionDigest, inputs } };
}

function exactOwnedDestinationOccupancy(repoRoot: string, path: string): readonly string[] {
  const owner = dirname(dirname(path));
  const segments = [basename(dirname(path)), basename(path)];
  const occupied: string[] = [];
  let parent = owner;
  for (let index = 0; index < segments.length; index++) {
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, parent, "Exact owner destination", true);
    const stat = lstatOrNull(absolute);
    if (!stat) break;
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Exact owner destination has an occupied non-directory parent: " + parent);
    const matches = readdirSync(absolute).filter((child) => collisionKey(child, "vs16-fold") === collisionKey(requireTaxonomyValue(segments[index],"segments[index]"), "vs16-fold"));
    for (const match of matches) {
      const found = [owner, ...segments.slice(0, index), match, ...segments.slice(index + 1)].join("/");
      if (match !== segments[index] || index === segments.length - 1) occupied.push(found);
    }
    if (!matches.includes(requireTaxonomyValue(segments[index],"segments[index]"))) break;
    parent += "/" + segments[index];
  }
  return occupied;
}

export function projectExactOwnedFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  const snapshot = exactOwnedCatalogSnapshot(repoRoot, taxonomy), catalog = snapshot.catalog;
  if (!catalog) {
    for (const row of Object.values(exactOwnedCurrentRevisions(taxonomy) ?? {})) {
      const entry = entries.get(row.sourcePath);
      if (!entry) continue;
      entry.normalizedPath = entry.sourcePath;
      entry.violations.push(violation("owner-leaf-authority-invalid", entry.sourcePath, "Current source requires its exact original catalog"));
    }
    return;
  }
  const governed = new Map(catalog.entries.flatMap((entry) => [[entry.sourcePath, entry], [entry.destinationPath, entry]] as const));
  for (const entry of entries.values()) {
    const owner = governed.get(entry.sourcePath);
    if (!owner || entry.nodeKind === "directory") continue;
    if (reservedDocumentationBasename(basename(entry.sourcePath))) {
      entry.normalizedPath = entry.sourcePath;
      continue;
    }
    let problems: readonly string[] = [], disposition = "problem";
    try {
      const { result } = exactOwnedFileResolution(repoRoot, entry, snapshot, taxonomy);
      problems = [...result.problems, ...exactOwnedEvidenceProblems(repoRoot, owner, catalog)];
      disposition = result.disposition;
    } catch (error) { problems = [error instanceof Error ? error.message : String(error)]; }
    if (problems.length > 0 || disposition === "problem") {
      entry.normalizedPath = entry.sourcePath;
      entry.violations.push(violation("owner-leaf-authority-invalid", entry.sourcePath, problems.join(" | ")));
      continue;
    }
    applyArtifactProjectionPath(entry, owner.destinationPath, taxonomy);
    entry.fileKind = "markdown";
    entry.semanticStem = null;
    entry.packageRole = packageLocation(entry.sourcePath, taxonomy) ? "configuration" : "not-package";
    entry.violations = entry.violations.filter((problem) => !["package-role-unresolved", "package-implementation-file", "package-implementation-destination-unresolved"].includes(problem.code));
    if (owner.fixedContractId) entry.fixedContractId = owner.fixedContractId;
  }
  const destinations = new Map(catalog.entries.filter((entry) => entry.disposition !== "fixed").map((entry) => [dirname(entry.destinationPath), entry]));
  for (const directory of entries.values()) {
    if (directory.nodeKind !== "directory" || !["📃️readme", "⚖️license"].includes(basename(directory.sourcePath))) continue;
    const owner = destinations.get(directory.sourcePath);
    let valid = false;
    if (owner) {
      const sourcePresent = lstatOrNull(absolutePath(repoRoot, owner.sourcePath)) !== null;
      const absolute = assertLexicalInputOutsideOpaque(repoRoot, directory.sourcePath, "Exact owner directory", true);
      const names = readdirSync(absolute);
      const leaf = lstatOrNull(absolutePath(repoRoot, owner.destinationPath));
      const generatedPair = owner.generatorOwnerId !== null && sourcePresent && Boolean(leaf?.isFile()) && canonicalJson(leafPreimage(absolutePath(repoRoot, owner.destinationPath))) === canonicalJson({ nodeKind: "file", contentHash: owner.preimage.sha256, mode: Number.parseInt(owner.preimage.mode, 8), size: owner.preimage.size });
      valid = (!sourcePresent || generatedPair) && names.length === 1 && names[0] === "📝️.md" && Boolean(leaf?.isFile() && !leaf.isSymbolicLink());
    }
    if (valid) applyArtifactProjectionPath(directory, directory.sourcePath, taxonomy);
    else {
      const problem = violation("projection-only-owner-invalid", directory.sourcePath, "Owner documentation directories require an exact registered owner, one Markdown leaf, and no raw sibling");
      directory.violations.push(problem);
      for (const child of entries.values()) if (dirname(child.sourcePath) === directory.sourcePath) child.violations.push({ ...problem, path: child.sourcePath });
    }
  }
}

export function projectArtifactEmptyFacetFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): void {
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "file" || basename(entry.sourcePath) !== "📌️.empty.md") continue;
    const authority = semanticArtifactEmptyFacetProjectionAuthority({ sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "" }, taxonomy.discoverySchema);
    if (authority.disposition !== "project" || !authority.destinationPath) continue;
    applyArtifactProjectionPath(entry, authority.destinationPath, taxonomy);
    const owner = absolutePath(repoRoot, dirname(entry.sourcePath));
    assertNoFollowAncestors(repoRoot, owner, "Artifact empty-facet owner", true);
    const destinationName = basename(authority.destinationPath);
    if (readdirSync(owner).some((name) => collisionKey(name, "vs16-fold") === collisionKey(destinationName, "vs16-fold"))) entry.violations.push(violation("projection-destination-occupied", entry.sourcePath, "Artifact empty-facet primary destination is occupied or folded-colliding"));
  }
}

export function projectTicketDocumentFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, fixedDirectoryContractByPath: ReadonlyMap<string, string>, taxonomy: LoadedTaxonomy): void {
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "file" || basename(entry.sourcePath) !== "ticket.md") continue;
    const ownerPath = dirname(entry.sourcePath);
    const authority = semanticOwnedPrimaryFileProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug" ? ["ticket-slug"] : [], sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "" }, taxonomy.discoverySchema);
    if (authority.disposition !== "project" || !authority.destinationPath) continue;
    applyArtifactProjectionPath(entry, authority.destinationPath, taxonomy);
    const owner = absolutePath(repoRoot, ownerPath);
    assertNoFollowAncestors(repoRoot, owner, "Ticket document owner", true);
    const destinationName = basename(authority.destinationPath);
    if (readdirSync(owner).some((name) => collisionKey(name, "vs16-fold") === collisionKey(destinationName, "vs16-fold"))) entry.violations.push(violation("projection-destination-occupied", entry.sourcePath, "Ticket document primary destination is occupied or folded-colliding"));
  }
}

export function projectTicketImportantFiles(repoRoot: string, entries: Map<string, MutableInventoryEntry>, fixedDirectoryContractByPath: ReadonlyMap<string, string>, taxonomy: LoadedTaxonomy): void {
  for (const entry of [...entries.values()].filter((candidate) => candidate.nodeKind === "file" && basename(candidate.sourcePath) === "📌️important.md").sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath))) {
    const ownerPath = dirname(entry.sourcePath);
    const manifestPath = `${ownerPath}/🎫️ticket.json`;
    const manifest = entries.get(manifestPath);
    const manifestClaimed = manifest?.nodeKind === "file" && manifest.fixedContractId === "ticket-manifest";
    const manifestContent = manifestClaimed ? readFileSync(absolutePath(repoRoot, manifestPath), "utf8") : undefined;
    if (manifestContent !== undefined) {
      const active = semanticOwnedFileProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug" ? ["ticket-slug"] : [], manifestPath, manifestFixedFilenameContractIds: ["ticket-manifest"], manifestContent, sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "", sourceByteLength: entry.size }, taxonomy.discoverySchema);
      if (active.disposition === "project" && active.destinationPath) {
        applyArtifactProjectionPath(entry, active.destinationPath, taxonomy);
        continue;
      }
      if (active.disposition === "remove") {
        applyArtifactProjectionPath(entry, entry.sourcePath, taxonomy);
        continue;
      }
    }
    const history = semanticOwnedFileHistoryProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: fixedDirectoryContractByPath.get(ownerPath) === "ticket-slug" ? ["ticket-slug"] : [], manifestPath: manifestClaimed ? manifestPath : undefined, manifestFixedFilenameContractIds: manifestClaimed ? ["ticket-manifest"] : [], manifestContent, sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "", sourceByteLength: entry.size }, taxonomy.discoverySchema);
    if (history.disposition === "project" && history.destinationPath) {
      applyArtifactProjectionPath(entry, history.destinationPath, taxonomy);
      continue;
    }
    const exact = ticketImportantExactMutationAuthority(repoRoot, entry.sourcePath, inventoryLeafPreimage(entry));
    if (exact?.disposition === "move" && exact.destinationPath) applyArtifactProjectionPath(entry, exact.destinationPath, taxonomy);
    else if (exact?.disposition === "remove") applyArtifactProjectionPath(entry, entry.sourcePath, taxonomy);
  }
}
