import { cargoRepositoryPackages } from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import type { SemanticDescendantNode } from "../📜️taxonomy-contracts/🟦️.ts";
import { type SemanticPackageAdapter, type SemanticPackageProjectionCase, parseCurrentJcoPackageDestination } from "../📦️package-projection-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { readTaxonomyUnchecked, taxonomyWorkspaceRoot } from "../🧭️workspace-taxonomy-locator/🟦️.ts";
import { cachedCatalogTaxonomy, cachedTaxonomy } from "../🧭️workspace-taxonomy-locator/⚡️cache/🟦️.ts";
import { taxonomyFileKindFilenamesV1, taxonomySingleFileKindFilenameV1 } from "../📄️file-kind-query/🟦️.ts";
import { type SemanticDirectoryMatchContext, canonicalSemanticDirectoryName, resolveSemanticDirectoryKindId, semanticDirectoryKindIds } from "../📂️semantic-directory-kinds/🟦️.ts";
import { isDiscoverySkipDirectory, readdirVanishing, retryOnEintr } from "../🪧️owner-markers/🟦️.ts";
import { type RegistryCatalogInputView, registryCatalogGitlinkBoundaries } from "../📇️registry-catalog-inputs/🟦️.ts";
import { fixedContractFilename } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { fileKindIdsByFilename, longestExtensionFileKindId, resolveScopedFileKindId, scopedFileKindIds } from "../📄️implementation-findings/🟦️.ts";
import { type ArtifactPathProjectionRoot, type SemanticPathProjectionAuthorityOptions, type SemanticProjectionAuthorityNode, type SemanticProjectionProfileIdentity, projectionByteCompare, projectionCanonicalKey } from "../🧱️path-projection-contracts/🟦️.ts";
import type { TaxonomyCliAttemptPreparationFacts, TaxonomyCliWritePreparationFacts } from "../🛠️generator-commands/🟦️.ts";
import { exactOwnerGeneratorPrestate, exactOwnerRegularFile, generatorPreviewExecution, semanticExactOwnedFileCatalog } from "../✅️owned-file-approvals/🟦️.ts";
import { semanticPackageProjectionAuthority, semanticPackageProjectionCatalog, semanticPackageSourceManifestIdentity } from "../🎁️package-projection/🟦️.ts";
import { auditSchemaVersionAndEvidence } from "../📆️audit-schema-version-and-evidence/🟦️.ts";
import { auditLeafAndReferencePolicies } from "../📎️audit-leaf-and-reference-policies/🟦️.ts";
import { auditFileKinds } from "../📔️audit-file-kinds/🟦️.ts";
import { auditScopedAndDirectoryKinds } from "../📗️audit-scoped-and-directory-kinds/🟦️.ts";
import { auditTransactionArtifacts } from "../🚥️audit-transaction-artifacts/🟦️.ts";
import { auditMemberAndDescendantContracts } from "../🛤️audit-member-and-descendant-contracts/🟦️.ts";
import { auditProjectionAndPackageContracts } from "../🚏️audit-projection-and-package-contracts/🟦️.ts";
import { auditOwnedFileProjection } from "../🔑️audit-owned-file-projection/🟦️.ts";
import { auditReferenceConsumersAndMutationCatalog } from "../🗞️audit-reference-consumers-and-catalog/🟦️.ts";
import { auditFixedContracts } from "../🔩️audit-fixed-contracts/🟦️.ts";
import { auditPackageBoundaryContracts } from "../🚛️audit-package-boundary-contracts/🟦️.ts";
import { auditGeneratorContracts } from "../⚗️audit-generator-contracts/🟦️.ts";
import { auditPathPoliciesAndAreas } from "../🛑️audit-path-policies-and-areas/🟦️.ts";
import { auditDependencyDirections } from "../🚇️audit-dependency-directions/🟦️.ts";
import { auditEcosystemsAndTargets } from "../🌄️audit-ecosystems-and-targets/🟦️.ts";
import { auditTestVocabulary } from "../🎲️audit-test-vocabulary/🟦️.ts";
import { auditManifestAndRootContracts } from "../📙️audit-manifest-and-root-contracts/🟦️.ts";
import { auditSchemaFormatsAndFacets } from "../📘️audit-schema-formats-and-facets/🟦️.ts";
import { auditMutationVocabulary } from "../💉️audit-mutation-vocabulary/🟦️.ts";
import { auditRepresentationAndChildDirectories } from "../🛖️audit-representation-and-child-dirs/🟦️.ts";
import { createTaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";
import { runToolSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

/** 📇️ Validates catalog vocabulary without reading unrelated generator outputs or workspace diagnostics. */
export function loadCatalogTaxonomy(): Taxonomy {
  if (cachedCatalogTaxonomy.current) return cachedCatalogTaxonomy.current;
  const taxonomy = readTaxonomyUnchecked();
  const problems = validateTaxonomy(taxonomy);
  if (problems.length > 0) throw new Error(`Invalid taxonomy schema:\n${problems.map((problem) => `- ${problem}`).join("\n")}`);
  cachedCatalogTaxonomy.current = taxonomy;
  return taxonomy;
}

/** 📖️ Reads, strictly validates, and caches the incompatible version-7 taxonomy and workspace contracts. */
export function loadTaxonomy(): Taxonomy {
  if (cachedTaxonomy.current) return cachedTaxonomy.current;
  const taxonomy = loadCatalogTaxonomy();
  const workspaceRoot = taxonomyWorkspaceRoot();
  const problems = workspaceRoot ? validateGeneratorContractsAgainstWorkspace(workspaceRoot, taxonomy) : ["generatorContracts workspace root could not be resolved."];
  if (problems.length > 0) throw new Error(`Invalid taxonomy schema:\n${problems.map((problem) => `- ${problem}`).join("\n")}`);
  cachedTaxonomy.current = taxonomy;
  return cachedTaxonomy.current;
}

/** 📄️ Returns every canonical kind-only filename for one file kind. */
export function canonicalFilenamesForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  return taxonomyFileKindFilenamesV1(kindId,taxonomy);
}

/** 📄️ Returns the single canonical filename for a one-extension file kind. */
export function canonicalFilenameForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  return taxonomySingleFileKindFilenameV1(kindId,taxonomy);
}

/** 🍃️ Renders the schema-ordered primary physical leaf for new authored content. */
export function canonicalPrimaryFilenameForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const filename = canonicalFilenamesForKind(kindId, taxonomy)[0];
  if (!filename) throw new Error(`File kind ${JSON.stringify(kindId)} must declare a primary extension chain.`);
  return filename;
}

/** 🧩️ Resolves a source path to one physical file kind through the longest registered extension chain, once per taxonomy and filename. */
export function fileKindIdForSourcePath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const filename = normalized.slice(normalized.lastIndexOf("/") + 1).toLowerCase();
  let memo = fileKindIdsByFilename.get(taxonomy);
  if (!memo) fileKindIdsByFilename.set(taxonomy, memo = new Map());
  if (memo.has(filename)) return memo.get(filename)!;
  const kindId = longestExtensionFileKindId(filename, taxonomy);
  memo.set(filename, kindId);
  return kindId;
}

/** 🎟️ Resolves an owner-scoped evidence suffix without admitting it as a global file kind, once per taxonomy, path and parent kind. */
export function scopedFileKindIdForSourcePath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: Readonly<{ parentDirectoryKindId?: string }> = {}): string | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  let memo = scopedFileKindIds.get(taxonomy);
  if (!memo) scopedFileKindIds.set(taxonomy, memo = new Map());
  const key = `${normalized}\0${context.parentDirectoryKindId ?? ""}`;
  if (memo.has(key)) return memo.get(key)!;
  const kindId = resolveScopedFileKindId(normalized, taxonomy, context);
  memo.set(key, kindId);
  return kindId;
}

/** 🧪️ Validates every unpublished attempt preparation before any recovery mutation. */
export function taxonomyCliAttemptPreparationsProblems(facts: readonly Readonly<TaxonomyCliAttemptPreparationFacts>[], taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const names = new Set<string>();
  const ordinals = new Set<string>();
  for (const preparation of [...facts].sort((left, right) => projectionByteCompare(left.directoryName, right.directoryName))) {
    const kindId = semanticDirectoryKindId(preparation.directoryName, taxonomy, { parentKindId: preparation.parentKindId });
    if (kindId !== "transaction-attempt-preparation") problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} must resolve below transaction-attempts.`);
    if (names.has(preparation.directoryName)) problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} is duplicated.`);
    names.add(preparation.directoryName);
    const ordinal = preparation.directoryName.match(/^🚧️prepare-([0-9]{6})-/u)?.[1];
    if (!ordinal) problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} must carry one six-digit ordinal.`);
    else if (ordinals.has(ordinal)) problems.push(`Attempt preparation ordinal ${ordinal} is duplicated.`);
    else ordinals.add(ordinal);
    const childNames = new Set<string>();
    for (const child of [...preparation.children].sort((left, right) => projectionByteCompare(`${left.name}\0${left.nodeKind}`, `${right.name}\0${right.nodeKind}`))) {
      if (childNames.has(child.name)) problems.push(`Attempt preparation child ${JSON.stringify(child.name)} is duplicated.`);
      childNames.add(child.name);
      const expectedKind = child.name === "🚧️stage" ? "transaction-stage" : child.name === "💾️backup" ? "transaction-backup" : child.name === "🔒️lease" ? "transaction-lease" : null;
      if (expectedKind) {
        if (child.nodeKind !== "directory" || semanticDirectoryKindId(child.name, taxonomy, { parentKindId: kindId ?? undefined }) !== expectedKind) problems.push(`Attempt preparation child ${JSON.stringify(child.name)} must be its exact no-follow directory kind.`);
      } else if (child.name === canonicalFilenameForKind("json", taxonomy)) {
        if (child.nodeKind !== "file") problems.push("Attempt preparation journal must be an exact regular file.");
      } else problems.push(`Attempt preparation child ${JSON.stringify(child.name)} is not admitted.`);
    }
  }
  return problems;
}

/** 📝️ Validates a Windows-safe JSON-write exchange beneath the exact journal or lease-preparation owner. */
export function taxonomyCliJsonWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-json-write-preparation") problems.push("JSON write preparation must resolve below transaction-journal-write, transaction-lease-preparation, or transaction-lease.");
  const allowed = new Set(["🔣️.json", "⏮️.json"]);
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("JSON write preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    if (!allowed.has(leaf)) problems.push(`JSON write preparation leaf ${JSON.stringify(leaf)} is not admitted.`);
    if (leaf === "⏮️.json" && scopedFileKindIdForSourcePath(`🚧️journal/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== "transaction-json-previous") problems.push("JSON write displaced previous leaf has no scoped exchange authority.");
  }
  return problems;
}

/** 🔒️ Validates canonical, preparing, and stale-quarantined lease publication states. */
export function taxonomyCliLeaseDirectoryProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-lease" && kindId !== "transaction-lease-preparation") problems.push("Lease directory must resolve as a canonical lease or token-bound preparation below its exact parent.");
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Lease leaves must be unique.");
  for (const leaf of facts.leafNames) if (leaf !== canonicalFilenameForKind("json", taxonomy)) problems.push(`Lease leaf ${JSON.stringify(leaf)} is not the canonical JSON record.`);
  if (facts.writePreparations.length > 1) problems.push("Lease directory may contain at most one JSON-write preparation.");
  for (const writer of facts.writePreparations) problems.push(...taxonomyCliJsonWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  const publishedState = kindId === "transaction-lease" || facts.directoryName.endsWith("-stale");
  const hasCanonical = facts.leafNames.includes(canonicalFilenameForKind("json", taxonomy));
  const exchangeWithoutCanonical = facts.writePreparations.length === 1 && [...facts.writePreparations[0]!.leafNames].sort(projectionByteCompare).join("\0") === ["⏮️.json", "🔣️.json"].sort(projectionByteCompare).join("\0");
  if (publishedState && !hasCanonical && !exchangeWithoutCanonical) problems.push("Published or stale lease must retain canonical JSON or the exact displaced-previous exchange state.");
  return problems;
}

/** 🎯 Resolves an output path to its one exact or ancestor-root generator classification. */
export function generatorContractIdsForOutputPath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  return Object.entries(taxonomy.generatorContracts).filter(([, contract]) => contract.outputRoots.some((root) => normalized === root.path || normalized.startsWith(`${root.path}/`))).map(([id]) => id);
}

/** 📁️ Resolves one canonical semantic directory name to its unique global or owner-local registry identifier, once per taxonomy, name and context. */
export function semanticDirectoryKindId(name: string, taxonomy: Taxonomy = loadTaxonomy(), context: SemanticDirectoryMatchContext = {}): string | null {
  let memo = semanticDirectoryKindIds.get(taxonomy);
  if (!memo) semanticDirectoryKindIds.set(taxonomy, memo = new Map());
  const key = `${name}\0${context.parentKindId ?? ""}\0${(context.ancestorKindIds ?? []).join("\0")}`;
  if (memo.has(key)) return memo.get(key)!;
  const kindId = resolveSemanticDirectoryKindId(name, taxonomy, context);
  memo.set(key, kindId);
  return kindId;
}

/** 🎨️ Renders one profile without exposing a reverse string parser. */
export function renderSemanticProjectionProfile(contractId: string, identity: SemanticProjectionProfileIdentity, taxonomy: Taxonomy = loadTaxonomy()): string {
  const contract = taxonomy.semanticPathProjectionContracts[contractId];
  const renderer = contract && taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId];
  if (!contract || !renderer || !identity.artifactId) throw new Error(`Unknown or incomplete semantic projection ${JSON.stringify(contractId)}.`);
  const { standard, subset } = taxonomy.semanticDirectoryKinds;
  if (!standard || !subset) throw new Error("Standard and subset semantic directory contracts required");
  if (!new RegExp(standard.slugPattern, "u").test(identity.standardVersion) || !new RegExp(subset.slugPattern, "u").test(identity.subsetId)) throw new Error("Projection profile captures do not satisfy standard/subset slug contracts.");
  const rendered = renderer.template.replace("{standardVersion}", identity.standardVersion).replace("{subsetId}", identity.subsetId);
  const profileIndex = contract.destinationSegments.findIndex((segment) => "render" in segment && segment.render === "profile");
  const parent = profileIndex > 0 ? contract.destinationSegments[profileIndex - 1] : undefined;
  const parentKindId = parent && "kindId" in parent ? parent.kindId : undefined;
  if (semanticDirectoryKindId(rendered, taxonomy, { parentKindId }) !== renderer.directoryKindId) throw new Error(`Projection profile renderer produced invalid directory ${JSON.stringify(rendered)}.`);
  return rendered;
}

/** 🪆️ Parses exact source segments and renders a destination profile without reverse-splitting it. */
export function renderArtifactPathProjectionRoot(options: Pick<SemanticPathProjectionAuthorityOptions, "artifactRoot" | "contractId" | "sourceRoot">, taxonomy: Taxonomy = loadTaxonomy()): ArtifactPathProjectionRoot {
  const problems: string[] = [];
  const contract = taxonomy.semanticPathProjectionContracts[options.contractId];
  if (!contract || !contract.sourceArtifactMemberName) return { captures: {}, destinationRoot: "", problems: [`Unknown artifact projection contract ${JSON.stringify(options.contractId)}.`] };
  const sourceOwner = taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  const artifactDirectoryName = basename(options.artifactRoot);
  if (artifactDirectoryName !== contract.sourceArtifactMemberName || !sourceOwner?.memberNames.includes(artifactDirectoryName)) problems.push(`Artifact root does not match sourceArtifactMemberName ${JSON.stringify(contract.sourceArtifactMemberName)}.`);
  if (options.sourceRoot !== options.sourceRoot.normalize("NFC") || options.artifactRoot !== options.artifactRoot.normalize("NFC") || /\uFE0E/u.test(`${options.artifactRoot}\0${options.sourceRoot}`)) problems.push("Projection roots must be NFC and must not contain VS15.");
  const prefix = `${options.artifactRoot}/`;
  const sourceNames = options.sourceRoot.startsWith(prefix) ? options.sourceRoot.slice(prefix.length).split("/") : [];
  if (sourceNames.length !== contract.sourceSegments.length) problems.push("Source root does not have the exact projection grammar length.");
  const captures: Record<string, string> = {};
  let parentKindId: string | undefined;
  for (const [index, segment] of contract.sourceSegments.entries()) {
    const name = sourceNames[index] ?? "";
    if ("memberKindId" in segment) {
      const member = taxonomy.semanticDirectoryMemberKinds[segment.memberKindId];
      if (name !== segment.literal || !member?.ownerKindIds.includes(parentKindId ?? "") || !member.memberNames.includes(canonicalSemanticDirectoryName(name, taxonomy))) problems.push(`Source segment ${index} does not match exact member registry ${JSON.stringify(segment.memberKindId)}.`);
      parentKindId = segment.memberKindId;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      const projected = taxonomy.semanticProjectedMemberKinds[segment.projectedMemberKindId];
      const source = projected && taxonomy.semanticDirectoryMemberKinds[projected.sourceMemberKindId];
      const canonical = canonicalSemanticDirectoryName(name, taxonomy);
      if (!projected || projected.projectionContractId !== options.contractId || !projected.ownerKindIds.includes(parentKindId ?? "") || !source?.memberNames.includes(canonical) || canonical !== name) problems.push(`Source segment ${index} does not match projected member ${JSON.stringify(segment.projectedMemberKindId)}.`);
      captures[segment.capture] = name;
      parentKindId = segment.projectedMemberKindId;
      continue;
    }
    if ("literal" in segment) {
      if (name !== segment.literal || semanticDirectoryKindId(name, taxonomy, { parentKindId }) !== segment.kindId) problems.push(`Source segment ${index} does not match kind ${JSON.stringify(segment.kindId)}.`);
    } else {
      const kind = taxonomy.semanticDirectoryKinds[segment.kindId];
      if (!kind || semanticDirectoryKindId(name, taxonomy, { parentKindId }) !== segment.kindId || !name.startsWith(kind.emoji)) problems.push(`Source capture ${JSON.stringify(segment.capture)} does not match kind ${JSON.stringify(segment.kindId)}.`);
      else captures[segment.capture] = name.slice(kind.emoji.length);
    }
    parentKindId = segment.kindId;
  }
  const standardVersion = captures.standardVersion ?? "";
  const subsetId = captures.subsetId ?? "";
  let destinationParentKindId: string | undefined;
  const destinationNames: string[] = [];
  for (const segment of contract.destinationSegments) {
    if ("projectedMemberKindId" in segment) {
      const value = captures[segment.copy];
      if (!value) problems.push(`Destination copy ${JSON.stringify(segment.copy)} is missing.`);
      else destinationNames.push(value);
      destinationParentKindId = segment.projectedMemberKindId;
      continue;
    }
    if ("literal" in segment) destinationNames.push(segment.literal);
    else if ("render" in segment) {
      try {
        destinationNames.push(renderSemanticProjectionProfile(options.contractId, { artifactId: artifactDirectoryName, standardVersion, subsetId }, taxonomy));
      } catch (error) {
        problems.push(error instanceof Error ? error.message : String(error));
      }
    } else {
      const value = captures[segment.copy];
      if (!value) problems.push(`Destination copy ${JSON.stringify(segment.copy)} is missing.`);
      else destinationNames.push(value);
    }
    destinationParentKindId = segment.kindId;
  }
  return { captures, destinationRoot: problems.length === 0 ? `${options.artifactRoot}/${destinationNames.join("/")}` : "", problems };
}

/** 🗺️ Enumerates exact artifact roots by forward-rendering registered profile tuples. */
export function artifactPathProjectionCatalogRoots(artifactRoot: string, contractId: string, taxonomy: Taxonomy = loadTaxonomy()): readonly Readonly<{ sourceRoot: string; destinationRoot: string }>[] {
  const contract = taxonomy.semanticPathProjectionContracts[contractId];
  const catalog = contract && taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
  if (!contract?.sourceArtifactMemberName || !catalog || !("contractKind" in catalog)) return [];
  const vectors = catalog.contractKind === "distributed-json-manifest-catalog" ? catalog.profileVectors : catalog.vectors;
  const roots = vectors.filter((vector) => vector.artifactId === basename(artifactRoot)).map((vector) => {
    const captures = vector as Readonly<Record<string, string>>;
    const sourceRoot = `${artifactRoot}/${contract.sourceSegments.map((segment) => "literal" in segment ? segment.literal : "projectedMemberKindId" in segment ? captures[segment.capture] : `${taxonomy.semanticDirectoryKinds[segment.kindId]?.emoji ?? ""}${captures[segment.capture] ?? ""}`).join("/")}`;
    const rendered = renderArtifactPathProjectionRoot({ artifactRoot, contractId, sourceRoot }, taxonomy);
    if (rendered.problems.length > 0) throw new Error(`Invalid forward profile vector for ${contractId}: ${rendered.problems.join(" | ")}`);
    return { sourceRoot, destinationRoot: rendered.destinationRoot };
  }).sort((left, right) => projectionByteCompare(left.destinationRoot, right.destinationRoot));
  for (const comparison of ["nfc", "case-fold", "vs16-fold"] as const) if (new Set(roots.map(({ destinationRoot }) => projectionCanonicalKey(destinationRoot, comparison))).size !== roots.length) throw new Error(`Forward profile vectors collide for ${contractId} under ${comparison}`);
  return roots;
}

/** 📏️ Derives a descendant node's canonical relative suffix from directory and physical file kinds. */
export function semanticDescendantNodeRelativePath(node: SemanticDescendantNode, taxonomy: Taxonomy = loadTaxonomy()): string {
  const parent = node.pathSegments.map((segment) => segment.literal);
  if (node.nodeType === "directory") return parent.join("/");
  if ("kindId" in node) return [...parent, canonicalPrimaryFilenameForKind(node.kindId, taxonomy)].join("/");
  const contract = taxonomy.fixedFilenameContracts[node.fixedFilenameContractId];
  if (!contract) throw new Error(`Unknown fixed filename contract ${JSON.stringify(node.fixedFilenameContractId)}.`);
  return [...parent, fixedContractFilename(contract)].join("/");
}

/**
 * 🚦️ Internal-consistency audit of the vocabulary itself: the completeness/structural artifact lists must
 * stay in their superset relation, direct mutation ownership must name one Rust component kind and only optional facets,
 * every registry, kind mapping, exact contract, package grammar, and clean area must agree. Returns human
 * readable problems (empty = healthy) so a vocabulary edit can never silently blind or flood the rules that
 * consume it.
 */
export function validateTaxonomy(taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const audit = createTaxonomyAudit(taxonomy, { semanticDirectoryKindId, fileKindIdForSourcePath, canonicalFilenameForKind, canonicalFilenamesForKind, taxonomyCliAttemptPreparationsProblems, taxonomyCliLeaseDirectoryProblems, artifactPathProjectionCatalogRoots, semanticDescendantNodeRelativePath });
  auditSchemaVersionAndEvidence(audit);
  auditLeafAndReferencePolicies(audit);
  auditFileKinds(audit);
  auditScopedAndDirectoryKinds(audit);
  auditTransactionArtifacts(audit);
  auditMemberAndDescendantContracts(audit);
  auditProjectionAndPackageContracts(audit);
  auditOwnedFileProjection(audit);
  auditReferenceConsumersAndMutationCatalog(audit);
  auditFixedContracts(audit);
  auditPackageBoundaryContracts(audit);
  auditGeneratorContracts(audit);
  auditPathPoliciesAndAreas(audit);
  auditDependencyDirections(audit);
  auditEcosystemsAndTargets(audit);
  auditTestVocabulary(audit);
  auditManifestAndRootContracts(audit);
  auditSchemaFormatsAndFacets(audit);
  auditMutationVocabulary(audit);
  auditRepresentationAndChildDirectories(audit);
  return audit.problems;
}

/** 🧩️ Renders unchanged frozen adapter bytes only after validating the complete current no-follow package. */
export function semanticPackageAdapterPreview(repoRoot: string, packageId: "jcoprobe-guest", taxonomy: Taxonomy = loadTaxonomy()): readonly SemanticPackageAdapter[] {
  const current = parseCurrentJcoPackageDestination(taxonomy.generatorContracts["jco-package-adapter"]?.currentPackageDestination);
  const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy), frozen = catalog?.packages.find((row) => row.id === packageId);
  if (!catalog || !frozen || frozen.mappings.length !== 4 || frozen.adapters.length !== 1 || frozen.adapters[0]?.id !== "jco-library") throw new Error("JCO frozen adapter evidence is unavailable");
  const destinations = [current.cargoLockPath, current.cargoManifestPath, current.componentPath, current.witPath];
  const row: SemanticPackageProjectionCase = { ...frozen, semanticOwnerRoot: current.semanticOwnerRoot, destinationRoot: current.packageRoot, mappings: frozen.mappings.map((mapping, index) => ({ ...mapping, destinationPath: destinations[index]! })), adapters: [{ ...frozen.adapters[0], path: current.adapterPath, targetPaths: [current.componentPath] }] };
  const nodes: SemanticProjectionAuthorityNode[] = row.mappings.map((mapping) => {
    if (exactOwnerRegularFile(repoRoot, mapping.destinationPath) !== "file") throw new Error("Current JCO input is absent or not a no-follow regular file: " + mapping.destinationPath);
    return { path: mapping.destinationPath, nodeKind: "file", content: readFileSync(join(repoRoot, mapping.destinationPath), "utf8") };
  });
  const state = exactOwnerRegularFile(repoRoot, current.adapterPath);
  if (state === "invalid") throw new Error("Current JCO adapter destination is not a no-follow regular file");
  nodes.push({ path: current.adapterPath, nodeKind: "file", content: state === "file" ? readFileSync(join(repoRoot, current.adapterPath), "utf8") : row.adapters[0]!.content });
  const authority = semanticPackageProjectionAuthority({ packageId, nodes, layout: "destination" }, { ...catalog, packages: [row] }, taxonomy);
  if (authority.problems.length) throw new Error(authority.problems.join(" | "));
  return authority.adapters;
}

/** 🌗️ Recognizes pending source-layout outputs without conferring source-content mutation authority. */
export function semanticPackageSourceOutputPhase(repoRoot: string, generatorId: string, taxonomy: Taxonomy): readonly string[] {
  if (generatorId !== "wgpu-frame-worker") return [];
  try {
    const contract = taxonomy.generatorContracts[generatorId], row = semanticPackageProjectionCatalog(repoRoot, taxonomy)?.packages.find((entry) => entry.id === "wgpu-renderer");
    if (!row || contract?.packageGeneration?.kind !== "wgpu-package-artifacts" || contract.projectionActivation?.packageId !== row.id) return [];
    const root = resolve(repoRoot), gitOptions = { cwd: root, encoding: "utf8" as const, timeout: 5_000, maxBuffer: 1_048_576, stdio: ["ignore", "pipe", "pipe"] as ["ignore", "pipe", "pipe"] };
    if (resolve(runToolSyncV1("git", ["rev-parse", "--show-toplevel"], { ...gitOptions, check: true }).stdout.trim()) !== root) return [];
    const admitted = runToolSyncV1("git", ["--literal-pathspecs", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", row.sourceRoot], { ...gitOptions, check: true }).stdout.split("\0").filter(Boolean).sort(projectionByteCompare);
    const expected = row.mappings.map((mapping) => mapping.sourcePath).sort(projectionByteCompare);
    if (JSON.stringify(admitted) !== JSON.stringify(expected) || admitted.some((path) => exactOwnerRegularFile(root, path) !== "file")) return [];
    const view = registryCatalogInputView(root, taxonomy);
    const destinations = new Set([...row.mappings.map((mapping) => mapping.destinationPath), ...row.adapters.map((adapter) => adapter.path), ...row.derivedLeaves.map((leaf) => leaf.path), ...contract.outputRoots.map((output) => output.path)]);
    if ([...destinations].some((path) => view.kind(path) !== null)) return [];
    const manifest = (name: string): string => {
      const mapping = row.mappings.find((entry) => entry.sourcePath === row.sourceRoot + "/" + name);
      if (!mapping) throw new Error("Source package manifest mapping is absent");
      return readFileSync(join(root, mapping.sourcePath), "utf8");
    };
    if (!semanticPackageSourceManifestIdentity({ cargoManifestContent: manifest("Cargo.toml"), nodeManifestContent: manifest("package.json"), projectManifestContent: manifest("📋️project.json") }, row)) return [];
    if (exactOwnerRegularFile(root, "Cargo.toml") !== "file" || exactOwnerRegularFile(root, "package.json") !== "file") return [];
    const cargo = readFileSync(join(root, "Cargo.toml"), "utf8"), node = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
    if (!cargoRepositoryPackages(root).some(member => member.directory === row.sourceRoot) || !Array.isArray(node.workspaces) || !node.workspaces.includes(row.sourceRoot)) return [];
    return contract.outputRoots.map((output) => output.path);
  } catch { return []; }
}

function nestedCargoGeneratedPrestate(repoRoot: string, path: string, generatorId: string, taxonomy: Taxonomy): boolean {
  if (!["jco-package-adapter", "external-cargo-locks", "wgpu-frame-worker"].includes(generatorId)) return false;
  try {
    if (generatorId === "wgpu-frame-worker") return semanticPackageSourceOutputPhase(repoRoot, generatorId, taxonomy).includes(path);
    if (generatorId === "jco-package-adapter") return semanticPackageAdapterPreview(repoRoot, "jcoprobe-guest", taxonomy).some((adapter) => adapter.path === path);
    const row = semanticPackageProjectionCatalog(repoRoot, taxonomy)?.packages.find((entry) => entry.id === "jcoprobe-guest");
    const mapping = row?.mappings.find((entry) => entry.destinationPath === path && basename(entry.sourcePath) === "Cargo.lock");
    return Boolean(mapping && exactOwnerRegularFile(repoRoot, mapping.sourcePath) === "file" && createHash("sha256").update(readFileSync(join(repoRoot, mapping.sourcePath))).digest("hex") === mapping.sourceHash);
  } catch { return false; }
}

export function validateGeneratorContractsAgainstWorkspace(repoRoot: string, taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const problems: string[] = [];
  const root = resolve(repoRoot);
  const catalog = semanticExactOwnedFileCatalog(root, taxonomy);
  for (const [id, contract] of Object.entries(taxonomy.generatorContracts ?? {})) {
    for (const consumer of contract.nativeConsumers ?? []) if (!existsSync(join(root, consumer, "Cargo.toml"))) problems.push(`generatorContracts[${JSON.stringify(id)}].nativeConsumers has no Cargo project at ${JSON.stringify(consumer)}.`);
    if (contract.target) {
      if (!contract.ownerPath) {
        problems.push(`generatorContracts[${JSON.stringify(id)}] has a target without an ownerPath.`);
      } else {
        const manifestPath = join(root, contract.ownerPath, "📋️project.json");
        if (!existsSync(manifestPath)) {
          problems.push(`generatorContracts[${JSON.stringify(id)}] owner project is missing at ${JSON.stringify(relative(root, manifestPath))}.`);
        } else {
          try {
            const project = JSON.parse(readFileSync(manifestPath, "utf8")) as { name?: unknown; targets?: Record<string, { executor?: unknown; options?: Record<string, unknown> }> };
            const separator = contract.target.lastIndexOf(":");
            const projectName = contract.target.slice(0, separator);
            const targetName = contract.target.slice(separator + 1);
            if (project.name !== projectName || !project.targets?.[targetName]) problems.push(`generatorContracts[${JSON.stringify(id)}].target ${JSON.stringify(contract.target)} is absent from its owner project.`);
            if (contract.previewTarget) {
              const previewSeparator = contract.previewTarget.lastIndexOf(":");
              const previewProject = contract.previewTarget.slice(0, previewSeparator);
              const previewName = contract.previewTarget.slice(previewSeparator + 1);
              const preview = project.targets?.[previewName];
              if (project.name !== previewProject || !preview) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget ${JSON.stringify(contract.previewTarget)} is absent from its owner project.`);
              else {
                try { generatorPreviewExecution(contract, preview); }
                catch (cause) { problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must route exactly to its declared script invocation in its owner project: ${cause instanceof Error ? cause.message : String(cause)}`); }
              }
            }
            if (contract.checkTarget) {
              const checkSeparator = contract.checkTarget.lastIndexOf(":");
              const checkProject = contract.checkTarget.slice(0, checkSeparator);
              const checkName = contract.checkTarget.slice(checkSeparator + 1);
              if (project.name !== checkProject || !project.targets?.[checkName]) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget ${JSON.stringify(contract.checkTarget)} is absent from its owner project.`);
            }
          } catch {
            problems.push(`generatorContracts[${JSON.stringify(id)}] owner project is not valid JSON.`);
          }
        }
      }
    }
    for (const output of contract.outputRoots ?? []) {
      if (output.producer) {
        try {
          const producer = output.producer, project = JSON.parse(readFileSync(join(root, producer.ownerPath, "📋️project.json"), "utf8"));
          const separator = producer.target.lastIndexOf(":");
          if (project.name !== producer.target.slice(0, separator) || !project.targets?.[producer.target.slice(separator + 1)]) problems.push(`generatorContracts[${JSON.stringify(id)}] output ${JSON.stringify(output.path)} has no declared Nx producer.`);
        } catch { problems.push(`generatorContracts[${JSON.stringify(id)}] output ${JSON.stringify(output.path)} producer project is unreadable.`); }
      }
      const owners = generatorContractIdsForOutputPath(output.path, taxonomy);
      if (owners.length !== 1 || owners[0] !== id) problems.push(`generatorContracts[${JSON.stringify(id)}] output ${JSON.stringify(output.path)} does not have exactly one owner.`);
      if (output.inclusion === "tracked" && !existsSync(join(root, output.path)) && !exactOwnerGeneratorPrestate(root, output.path, id, catalog) && !nestedCargoGeneratedPrestate(root, output.path, id, taxonomy)) problems.push(`generatorContracts[${JSON.stringify(id)}] tracked output ${JSON.stringify(output.path)} is missing.`);
    }
  }
  return problems;
}

/** 🚫️ Tests a repo-relative path against opaque prefixes without touching the candidate path. */
export function pathIsExcluded(repoRoot: string, candidate: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const rel = relative(resolve(repoRoot), resolve(candidate)).replaceAll("\\", "/").replace(/^\.\//u, "");
  if (rel === ".." || rel.startsWith("..")) return false;
  const prefixes = opaquePrefixes(taxonomy);
  for (let end = rel.indexOf("/"); ; end = rel.indexOf("/", end + 1)) {
    if (prefixes.has(end < 0 ? rel : rel.slice(0, end))) return true;
    if (end < 0) return false;
  }
}

const opaquePrefixCache = new WeakMap<object, { readonly size: number; readonly prefixes: ReadonlySet<string> }>();

/** 🚫️ The normalized opaque path prefixes of a taxonomy, derived once per exclusion table so a membership test costs one lookup per path segment instead of one pattern rewrite per exclusion. */
function opaquePrefixes(taxonomy: Taxonomy): ReadonlySet<string> {
  const exclusions = taxonomy.pathExclusions, size = Object.keys(exclusions).length, cached = opaquePrefixCache.get(exclusions);
  if (cached && cached.size === size) return cached.prefixes;
  const prefixes = new Set(Object.values(exclusions).map((exclusion) => exclusion.path.replace(/^\.\//u, "").replace(/\/+$/u, "")));
  opaquePrefixCache.set(exclusions, { size, prefixes });
  return prefixes;
}

/** 🛡️ Creates one no-follow, opaque-first catalog filesystem view. */
export function registryCatalogInputView(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): RegistryCatalogInputView {
  const kinds = new Map<string, "file" | "directory" | "symlink" | null>();
  const gitlinkBoundaries = registryCatalogGitlinkBoundaries(repoRoot);
  const checked = (path: string): string => {
    const absolute = resolve(repoRoot, path || ".");
    const normalized = relative(repoRoot, absolute).replaceAll("\\", "/");
    if (normalized !== path || pathIsExcluded(repoRoot, absolute, taxonomy)) throw new Error(`Registry catalog input is outside its nonopaque owner: ${path}`);
    return absolute;
  };
  const kind = (path: string): "file" | "directory" | "symlink" | null => {
    if (kinds.has(path)) return kinds.get(path)!;
    const absolute = checked(path);
    const parent = dirname(path) === "." ? "" : dirname(path);
    if (path && parent !== path) {
      const parentKind = kind(parent);
      if (parentKind === "symlink") throw new Error(`Registry catalog ancestor is a symlink: ${parent}`);
      if (parentKind !== "directory") return null;
    }
    let value: "file" | "directory" | "symlink" | null;
    try {
      const stat = retryOnEintr(() => lstatSync(absolute));
      value = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : stat.isFile() ? "file" : null;
      if (value === null) throw new Error(`Registry catalog input is not a regular node: ${path}`);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      value = null;
    }
    kinds.set(path, value);
    return value;
  };
  return {
    kind,
    entries(path) {
      const absolute = checked(path);
      const nodeKind = kind(path);
      if (nodeKind === null) return [];
      if (nodeKind !== "directory") throw new Error(`Registry catalog directory is ${nodeKind}: ${path}`);
      if (gitlinkBoundaries.has(path.normalize("NFC"))) return [];
      return readdirVanishing(absolute).filter((name) => !isDiscoverySkipDirectory(name) && !pathIsExcluded(repoRoot, join(absolute, name), taxonomy)).flatMap((name) => {
        const childPath = relative(repoRoot, join(absolute, name)).replaceAll("\\", "/"), childKind = kind(childPath);
        return childKind === null ? [] : [{ name, nodeKind: childKind }];
      });
    },
    readText(path) {
      if (kind(path) !== "file") throw new Error(`Registry catalog content input is missing or a symlink: ${path}`);
      return new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(checked(path)));
    },
    readBytes(path) {
      if (kind(path) !== "file") throw new Error(`Registry catalog content input is missing or a symlink: ${path}`);
      return readFileSync(checked(path));
    },
  };
}
