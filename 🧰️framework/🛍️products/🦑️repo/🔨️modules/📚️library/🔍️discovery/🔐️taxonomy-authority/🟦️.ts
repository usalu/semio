import { parseGeneratorPreviewProgressPolicyV1 } from "../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { foldPathEmojiIdentity, pathEmojiStatuteFindings } from "../../../../../../🔨️modules/🪪️identity/🛣️path/🟦️.ts";
import { leadingEmojiIdentity } from "../../../../../../🔨️modules/🪪️identity/🧩️grapheme/🟦️.ts";
import { cargoRepositoryPackages } from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { basename, dirname, join, posix, relative, resolve } from "node:path";
import { type FixedContractScope, type SemanticDescendantNode, type SemanticPathProjectionCatalogContract, type SemanticPathProjectionReferenceConsumerForm, canonicalTaxonomyEmoji } from "../📜️taxonomy-contracts/🟦️.ts";
import { type SemanticPackageAdapter, type SemanticPackageProjectionCase, exactOwnerPath, parseCurrentJcoPackageDestination } from "../📦️package-projection-contracts/🟦️.ts";
import { type Taxonomy, validateFrozenCoordinateEvidenceContracts, validateFrozenMarkdownCoordinateEvidenceContracts, validateHistoricalDocumentEvidencePopulations } from "../📑️taxonomy-document/🟦️.ts";
import { cachedCatalogTaxonomy, cachedTaxonomy, readTaxonomyUnchecked, taxonomyWorkspaceRoot } from "../🧭️workspace-taxonomy-locator/🟦️.ts";
import { type SemanticDirectoryMatchContext, canonicalSemanticDirectoryName, resolveSemanticDirectoryKindId, semanticDirectoryKindIds } from "../📂️semantic-directory-kinds/🟦️.ts";
import { isDiscoverySkipDirectory, readdirVanishing, retryOnEintr } from "../🪧️owner-markers/🟦️.ts";
import { type RegistryCatalogInputView, registryCatalogGitlinkBoundaries } from "../📇️registry-catalog-inputs/🟦️.ts";
import { createTaxonomyPathMatcher, fixedContractFilename, parseFixedDirectoryContractSetScope, parseNamedFixedDirectoryContractSetScope, taxonomyPathPatternMatches } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { fileKindIdsByFilename, longestExtensionFileKindId, resolveScopedFileKindId, scopedFileKindIds } from "../📄️implementation-findings/🟦️.ts";
import { type ArtifactPathProjectionRoot, type SemanticPathProjectionAuthorityOptions, type SemanticProjectionAuthorityNode, type SemanticProjectionProfileIdentity, projectionByteCompare, projectionCanonicalKey } from "../🧱️path-projection-contracts/🟦️.ts";
import { OPAQUE_PATH_EXCLUSIONS, mutationCatalogSourceOwnersProblems, mutationDomainOwnersProblems } from "../👥️mutation-owner-rules/🟦️.ts";
import { type TaxonomyCliAttemptPreparationFacts, type TaxonomyCliWritePreparationFacts, generatorPreviewResourceLimits, generatorPreviewScriptArguments } from "../🛠️generator-commands/🟦️.ts";
import { exactOwnerGeneratorPrestate, exactOwnerRegularFile, generatorPreviewExecution, parseCurrentSourceApprovals, parseSemanticOwnedDocumentCorrections, semanticExactOwnedFileCatalog } from "../✅️owned-file-approvals/🟦️.ts";
import { exampleAssetPath, parseSemanticPackageBrowserProfile, semanticPackageProjectionAuthority, semanticPackageProjectionCatalog, semanticPackageSourceManifestIdentity } from "../🎁️package-projection/🟦️.ts";

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
  const kind = taxonomy.fileKinds[kindId];
  return kind ? kind.extensionChains.map((extension) => `${kind.emoji}${extension}`) : [];
}

/** 📄️ Returns the single canonical filename for a one-extension file kind. */
export function canonicalFilenameForKind(kindId: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const names = canonicalFilenamesForKind(kindId, taxonomy);
  if (names.length !== 1) throw new Error(`File kind ${JSON.stringify(kindId)} must have exactly one extension chain, got ${names.length}.`);
  return names[0]!;
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

/** 🧮️ Validates the schema scope-owner, export-resolution and dialect vocabulary families. */
function validateSchemaScopeVocabulary(taxonomy: Taxonomy): string[] {
  const problems: string[] = [];
  const dialect = taxonomy.mutationPayloadSchemaAuthority?.jsonSchemaDialect;
  if (typeof taxonomy.schemaJsonDialect !== "string" || taxonomy.schemaJsonDialect !== dialect) problems.push("schemaJsonDialect must be the single dialect mutationPayloadSchemaAuthority already declares.");
  const levels = taxonomy.schemaScopeOwnerLevels;
  if (!levels || typeof levels !== "object") problems.push("schemaScopeOwnerLevels must declare the eligible schema scope-owner levels.");
  else {
    if (levels.facetDirName !== "🧬️schema" || semanticDirectoryKindId(levels.facetDirName, taxonomy) !== levels.directoryKindId) problems.push("schemaScopeOwnerLevels must bind the registered 🧬️schema directory kind.");
    if (typeof taxonomy.schemaDefaultFacetKind !== "string" || !taxonomy.schemaFacetKinds?.[taxonomy.schemaDefaultFacetKind]) problems.push("schemaDefaultFacetKind must name a declared schemaFacetKinds entry.");
    else if ((taxonomy.schemaFacetKinds[taxonomy.schemaDefaultFacetKind]?.facetPathIdentities ?? []).length !== 0) problems.push("schemaDefaultFacetKind must not claim exact facet modules; it is the fallback kind.");
    if (!levels.levels || typeof levels.levels !== "object" || Object.keys(levels.levels).length === 0) problems.push("schemaScopeOwnerLevels.levels must be a non-empty object.");
    else for (const [levelId, level] of Object.entries(levels.levels)) {
      if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(levelId)) problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}] must be a kebab identifier.`);
      if (!Array.isArray(level.pathPatterns) || level.pathPatterns.length === 0 || level.pathPatterns.some((value) => typeof value !== "string" || !value)) problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}].pathPatterns must be a non-empty string array.`);
      else for (const value of level.pathPatterns) try { void taxonomyPathPatternMatches("", value); } catch { problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}] has an invalid path pattern ${JSON.stringify(value)}.`); }
      if (typeof level.reason !== "string" || level.reason.length < 16) problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}].reason must state why the level owns contracts.`);
    }
    if (!Array.isArray(levels.excludedOwnerPathPatterns) || levels.excludedOwnerPathPatterns.length === 0) problems.push("schemaScopeOwnerLevels.excludedOwnerPathPatterns must list the never-eligible owner shapes.");
    else for (const value of levels.excludedOwnerPathPatterns) try { void taxonomyPathPatternMatches("", value); } catch { problems.push(`schemaScopeOwnerLevels.excludedOwnerPathPatterns has an invalid path pattern ${JSON.stringify(value)}.`); }
    if (!Array.isArray(levels.fixtureOwnerPathPatterns) || levels.fixtureOwnerPathPatterns.length === 0) problems.push("schemaScopeOwnerLevels.fixtureOwnerPathPatterns must list the test and fixture collection roots.");
    else for (const value of levels.fixtureOwnerPathPatterns) try { void taxonomyPathPatternMatches("", value); } catch { problems.push(`schemaScopeOwnerLevels.fixtureOwnerPathPatterns has an invalid path pattern ${JSON.stringify(value)}.`); }
    if (Array.isArray(levels.fixtureOwnerPathPatterns) && Array.isArray(levels.excludedOwnerPathPatterns) && levels.fixtureOwnerPathPatterns.some((value) => levels.excludedOwnerPathPatterns.includes(value))) problems.push("schemaScopeOwnerLevels.fixtureOwnerPathPatterns and excludedOwnerPathPatterns must be disjoint: a collection is recognized by its own ancestors with the module-member carve-out, an excluded owner shape by an exact pattern.");
    if (typeof levels.fixtureOwnerReason !== "string" || levels.fixtureOwnerReason.length < 16) problems.push("schemaScopeOwnerLevels.fixtureOwnerReason must state why a collection is never a scope owner and why the schema documents inside it are findings.");
    if (typeof levels.moduleMemberDirName !== "string" || !(taxonomy.pluginChildDirs ?? []).includes(levels.moduleMemberDirName)) problems.push("schemaScopeOwnerLevels.moduleMemberDirName must be the declared module-member directory a collection pattern never claims.");
    if (typeof levels.mutationsFacetDirName !== "string" || !(taxonomy.schemaChildDirs ?? []).includes(levels.mutationsFacetDirName)) problems.push("schemaScopeOwnerLevels.mutationsFacetDirName must be a declared schema child directory.");
    if (typeof levels.inertSchemaDataKeyword !== "string" || !/^[a-z][A-Za-z0-9]*$/u.test(levels.inertSchemaDataKeyword)) problems.push("schemaScopeOwnerLevels.inertSchemaDataKeyword must name the case-file key that declares schema-shaped test inputs.");
  }
  const resolution = taxonomy.schemaExportResolution;
  if (!resolution || typeof resolution !== "object") problems.push("schemaExportResolution must declare how a consumer names one schema export.");
  else {
    const expected = ["catalogContractId", "catalogDocumentPath", "catalogPath", "exportIdPattern", "exportsKeyword", "forbiddenPlacementPatterns", "formatIds", "generator", "idBase", "idFacetFilenamePattern", "mutationAggregateFacetFilename", "mutationLeafFacetFilename", "mutationScopeSegment", "placementExceptions", "rootExportKeyword", "rustEntriesContractId", "scopeIdPattern", "scopeIdSeparator", "uriPattern", "uriScheme"];
    if (Object.keys(resolution).sort().join("\0") !== expected.join("\0")) problems.push(`schemaExportResolution must declare exactly ${expected.join(", ")}.`);
    if (resolution.uriScheme !== "schema" || !resolution.uriPattern.includes(`${resolution.uriScheme}://`)) problems.push("schemaExportResolution.uriPattern must bind its own declared scheme.");
    for (const [key, value] of [["uriPattern", resolution.uriPattern], ["idFacetFilenamePattern", resolution.idFacetFilenamePattern], ["scopeIdPattern", resolution.scopeIdPattern], ["exportIdPattern", resolution.exportIdPattern]] as const) {
      try { void new RegExp(value, "u"); } catch { problems.push(`schemaExportResolution.${key} is not a valid Unicode regular expression.`); }
    }
    if (typeof resolution.idBase !== "string" || !resolution.idBase.startsWith("https://") || !resolution.idBase.endsWith("/")) problems.push("schemaExportResolution.idBase must be an absolute https prefix ending in a separator.");
    if (resolution.scopeIdSeparator !== ".") problems.push("schemaExportResolution.scopeIdSeparator must be the dotted separator the descriptor ids already use.");
    if (resolution.rootExportKeyword !== "title" || resolution.exportsKeyword !== "$defs") problems.push("schemaExportResolution must read exports from the JSON Schema title and $defs keywords.");
    if (typeof resolution.catalogPath !== "string" || !resolution.catalogPath.endsWith(".json") || typeof resolution.catalogDocumentPath !== "string" || !resolution.catalogDocumentPath.endsWith(".md")) problems.push("schemaExportResolution must point at one generated JSON catalog and one generated Markdown index.");
    if (typeof resolution.generator !== "string" || !resolution.generator.includes("📜️script.ts schema generate")) problems.push("schemaExportResolution.generator must name the exact generating command.");
    if (!Array.isArray(resolution.forbiddenPlacementPatterns) || resolution.forbiddenPlacementPatterns.length === 0) problems.push("schemaExportResolution.forbiddenPlacementPatterns must list the retired schema placements.");
    if (!resolution.placementExceptions || typeof resolution.placementExceptions !== "object" || Array.isArray(resolution.placementExceptions)) problems.push("schemaExportResolution.placementExceptions must be an object of exact path to reason.");
    else for (const [path, reason] of Object.entries(resolution.placementExceptions)) if (typeof reason !== "string" || reason.length < 16 || path.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`schemaExportResolution.placementExceptions[${JSON.stringify(path)}] must name an exact path and a stated reason.`);
    const dataFormats = [...(taxonomy.schemaFacetKinds?.[taxonomy.schemaDefaultFacetKind]?.formats ?? [])].sort();
    if (!resolution.formatIds || typeof resolution.formatIds !== "object" || Array.isArray(resolution.formatIds)) problems.push("schemaExportResolution.formatIds must map every default-facet-kind format key to its ascii format id.");
    else {
      if (Object.keys(resolution.formatIds).sort().join("\0") !== dataFormats.join("\0")) problems.push(`schemaExportResolution.formatIds must name exactly the ${taxonomy.schemaDefaultFacetKind} formats ${dataFormats.join(", ")}.`);
      const ids = Object.values(resolution.formatIds);
      if (ids.some((value) => typeof value !== "string" || !/^[a-z][a-z0-9]*$/u.test(value)) || new Set(ids).size !== ids.length) problems.push("schemaExportResolution.formatIds values must be unique lowercase ascii format ids.");
    }
    if (typeof resolution.rustEntriesContractId !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(resolution.rustEntriesContractId)) problems.push("schemaExportResolution.rustEntriesContractId must be the kebab contract id of the Rust registry entry dump.");
    if (typeof resolution.mutationScopeSegment !== "string" || !new RegExp(resolution.scopeIdPattern, "u").test(resolution.mutationScopeSegment)) problems.push("schemaExportResolution.mutationScopeSegment must be one scope-id segment.");
    if (typeof resolution.mutationLeafFacetFilename !== "string" || !new RegExp(resolution.idFacetFilenamePattern, "u").test(resolution.mutationLeafFacetFilename)) problems.push("schemaExportResolution.mutationLeafFacetFilename must be a declared facet filename.");
    if (typeof resolution.mutationAggregateFacetFilename !== "string" || !new RegExp(resolution.idFacetFilenamePattern, "u").test(resolution.mutationAggregateFacetFilename)) problems.push("schemaExportResolution.mutationAggregateFacetFilename must be a declared facet filename.");
  }
  return problems;
}

/**
 * 🚦️ Internal-consistency audit of the vocabulary itself: the completeness/structural artifact lists must
 * stay in their superset relation, direct mutation ownership must name one Rust component kind and only optional facets,
 * every registry, kind mapping, exact contract, package grammar, and clean area must agree. Returns human
 * readable problems (empty = healthy) so a vocabulary edit can never silently blind or flood the rules that
 * consume it.
 */
export function validateTaxonomy(taxonomy: Taxonomy = readTaxonomyUnchecked()): string[] {
  const problems: string[] = [];
  const pathMatcher = createTaxonomyPathMatcher();
  const emojiVerdicts = new Map<string, boolean>();
  const canonicalEmoji = (value: unknown): value is string => {
    if (typeof value !== "string") return false;
    if (!emojiVerdicts.has(value)) emojiVerdicts.set(value, canonicalTaxonomyEmoji(value));
    return emojiVerdicts.get(value)!;
  };
  const document = taxonomy as unknown as Record<string, unknown>;
  const removedKeys = [
    "semanticManifestFilename", "subsetsManifestFilename", "packagingFileNames", "packagingFileSuffixes", "packagingDirNames",
    "surfaceSchemaSpecFilenames", "textSpecFilenames", "binarySpecFilenames", "artifactSchemaSpecFilenames", "exampleLeafFilenames",
    "exampleTestLeafFilenames", "semioDataLeafPrefix", "semioFileExtension", "artifactSpecFilenames", "windowEmptyFacetFilename",
    "taxonomyLeafFilenames", "entryFilenames", "storyLeafFilename", "requireEmojiPrefixWithVs16", "rootDataFileNames",
    "rootDocFileNames", "areaStates", "pluginTaxonomyStates", "repoWideFiles", "testFeatureFilename", "testAdapterFilenames",
    "testContributionFilename", "testOutputMarkerFilename", "testExcludedPathPrefixes", "testOracleRegistryPath", "testSchemaPath",
    "layeringGeneratedInventories", "semanticProjectionContracts", "projectedMemberKinds", "projectionContracts", "profileRenderers",
    "descendantContracts", "mutationCatalogProjectionContractId", "mutationCatalogKindBijection", "mutationChildDirs", "compositeMutationChildDirs",
  ];
  for (const key of removedKeys) if (key in document) problems.push(`${key} was removed by schema version 7; use kind IDs or exact contracts.`);
  if (taxonomy.schemaVersion !== 7) problems.push(`schemaVersion must be exactly 7, got ${JSON.stringify(taxonomy.schemaVersion)}.`);
  problems.push(...validateFrozenCoordinateEvidenceContracts(taxonomy.frozenCoordinateEvidenceContracts));
  problems.push(...validateFrozenMarkdownCoordinateEvidenceContracts(taxonomy.frozenMarkdownCoordinateEvidenceContracts));
  problems.push(...validateHistoricalDocumentEvidencePopulations(taxonomy.historicalDocumentEvidencePopulations));

  const record = (value: unknown, key: string): value is Record<string, unknown> => {
    const valid = typeof value === "object" && value !== null && !Array.isArray(value);
    if (!valid || Object.keys(value as Record<string, unknown>).length === 0) problems.push(`${key} must be a non-empty object.`);
    return valid;
  };
  if (record(taxonomy.physicalLeafRendering, "physicalLeafRendering")) {
    const expected = { direction: "forward-only", filename: "file-kind-emoji-and-extension-chain", sourceExtension: "longest-registered-chain", authoringExtension: "schema-ordered-primary", runtimeLookup: "canonical-only" };
    if (Object.keys(taxonomy.physicalLeafRendering).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, value]) => (taxonomy.physicalLeafRendering as unknown as Record<string, unknown>)[key] !== value)) problems.push("physicalLeafRendering must declare exact forward-only kind-only rendering, longest source extension, primary authoring extension, and canonical-only runtime lookup.");
  }
  if (record(taxonomy.referenceClosure, "referenceClosure")) {
    const expected = { scope: "repository-incoming-and-moved-outgoing", candidateSource: "git-tracked-and-untracked-plus-explicit-ticket", candidateAdmission: "opaque-first-no-follow", coordinateRoots: "verified-repository-ownership", unsupportedPathBearingForms: "error", frozenSourceCoordinates: "exact-digest-and-token-authority", frozenPlanCoordinates: "canonical-schema-v2-digest-and-typed-token-authority", preimageDrift: "reject", newIncomingReferences: "reject-or-rollback", ordering: "utf8-byte", historicalDocumentEvidence: "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded" };
    if (Object.keys(taxonomy.referenceClosure).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, value]) => (taxonomy.referenceClosure as unknown as Record<string, unknown>)[key] !== value)) problems.push("referenceClosure must declare repository-wide incoming and moved outgoing closure with opaque-first no-follow candidates, exact frozen coordinate authority, drift rejection, and byte ordering.");
  }
  const ids = (values: readonly string[] | undefined, registry: Readonly<Record<string, unknown>>, key: string): void => {
    if (!Array.isArray(values)) {
      problems.push(`${key} must be an array.`);
      return;
    }
    const seen = new Set<string>();
    for (const id of values) {
      if (seen.has(id)) problems.push(`${key} contains duplicate id ${JSON.stringify(id)}.`);
      seen.add(id);
      if (!(id in registry)) problems.push(`${key} references missing id ${JSON.stringify(id)}.`);
    }
  };
  const pattern = (value: string, key: string): void => {
    try {
      void new RegExp(value, "u");
    } catch {
      problems.push(`${key} is not a valid Unicode regular expression.`);
    }
  };
  const fullPattern = (value: unknown, key: string): void => {
    if (typeof value !== "string") {
      problems.push(`${key} must be a string.`);
      return;
    }
    pattern(value, key);
    if (!value.startsWith("^") || !value.endsWith("$")) problems.push(`${key} must be an anchored full-match pattern.`);
  };
  const pathPattern = (value: unknown, key: string): void => {
    if (typeof value !== "string" || !value || value !== value.normalize("NFC") || value.startsWith("/") || value.includes("\\") || /[{}!]/u.test(value) || /(^|\/)\*\*[^/]|[^/]\*\*(\/|$)/u.test(value)) {
      problems.push(`${key} must be one NFC workspace-relative v7 path pattern.`);
      return;
    }
    try {
      void pathMatcher.matches("", value);
    } catch {
      problems.push(`${key} is not a valid v7 path pattern.`);
    }
  };
  if (record(taxonomy.implementationLeafPolicy, "implementationLeafPolicy")) {
    const policy = taxonomy.implementationLeafPolicy;
    const ignoredPathPatterns = ["**/.git", "**/.next", "**/.nx", "**/.pytest_cache", "**/.venv", "**/.🧬semio", "**/__pycache__", "**/coverage", "**/dist", "**/node_modules", "**/storybook-static", "**/target", "**/📤️dist", "**/🗑️generated", "**/🔌️plugin-modules", "**/📦️packages/🦀️rust/pkg", "**/📦️packages/🟦️typescript/out", "**/📦️packages/🔷️dotnet/obj"];
    if (Object.keys(policy).sort().join("\0") !== "fileKindIds\0ignoredPathPatterns\0roles" || policy.roles.join("\0") !== "source" || policy.fileKindIds.join("\0") !== "css\0html" || policy.ignoredPathPatterns.join("\0") !== ignoredPathPatterns.join("\0")) problems.push("implementationLeafPolicy must cover every source role plus exact CSS/HTML authored kinds and only the declared generated, cache and tool-state paths.");
    ids(policy.fileKindIds, taxonomy.fileKinds, "implementationLeafPolicy.fileKindIds");
    for (const [index, value] of policy.ignoredPathPatterns.entries()) pathPattern(value, `implementationLeafPolicy.ignoredPathPatterns[${index}]`);
  }
  const workspacePath = (value: unknown, key: string): value is string => {
    const valid = typeof value === "string" && value.length > 0 && value === value.normalize("NFC") && !value.startsWith("/") && !value.endsWith("/") && !value.includes("\\")
      && !/[*?\[\]{}!]/u.test(value) && value.split("/").every((segment) => segment.length > 0 && segment !== "." && segment !== "..");
    if (!valid) problems.push(`${key} must be one exact NFC workspace-relative path.`);
    return valid;
  };
  const extensionChain = (value: unknown, key: string): void => {
    if (typeof value !== "string" || !/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(value)) problems.push(`${key} must be one lowercase dot-prefixed extension chain.`);
  };
  const exactKeys = (value: object, allowed: readonly string[], key: string): void => {
    const actual = Object.keys(value).sort();
    const expected = [...allowed].sort();
    if (actual.join("\0") !== expected.join("\0")) problems.push(`${key} must contain exactly ${expected.join(", ")}.`);
  };
  const kebabId = (value: string, key: string): void => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value)) problems.push(`${key} must be kebab-case.`);
  };

  if (record(taxonomy.fileKinds, "fileKinds")) {
    const canonical = new Map<string, string>();
    const extensionOwners = new Map<string, string>();
    for (const [id, spec] of Object.entries(taxonomy.fileKinds)) {
      if (!canonicalEmoji(spec.emoji)) problems.push(`fileKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
      if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`fileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
      for (const extension of spec.extensionChains ?? []) {
        if (!/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(extension)) problems.push(`fileKinds[${JSON.stringify(id)}] has invalid extension chain ${JSON.stringify(extension)}.`);
        const extensionOwner = extensionOwners.get(extension);
        if (extensionOwner) problems.push(`fileKinds ${JSON.stringify(extensionOwner)} and ${JSON.stringify(id)} both own physical extension ${JSON.stringify(extension)}.`);
        extensionOwners.set(extension, id);
        const filename = `${spec.emoji}${extension}`;
        const prior = canonical.get(filename);
        if (prior) problems.push(`fileKinds ${JSON.stringify(prior)} and ${JSON.stringify(id)} collide at ${JSON.stringify(filename)}.`);
        canonical.set(filename, id);
      }
    }
    for (const [id, extension] of [["png", ".png"], ["bmp", ".bmp"]] as const) {
      const kind = taxonomy.fileKinds[id];
      if (!kind || kind.role !== "asset" || kind.emoji !== "🖼️" || kind.extensionChains.join("\0") !== extension) problems.push(`fileKinds.${id} must be the canonical 🖼️${extension} asset kind.`);
    }
  }

  if (record(taxonomy.fileKindResolutionRules, "fileKindResolutionRules")) for (const [id, rule] of Object.entries(taxonomy.fileKindResolutionRules)) {
    extensionChain(rule.extensionChain, `fileKindResolutionRules[${JSON.stringify(id)}].extensionChain`);
    const kind = taxonomy.fileKinds[rule.fileKindId];
    if (!kind || !kind.extensionChains.includes(rule.extensionChain)) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}] must reference a kind owning its extension chain.`);
    if (rule.priority !== 0) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].priority must be zero for physical resolution.`);
    for (const removed of ["filenamePattern", "pathPattern", "parentKindIds", "ancestorKindIds"]) if (removed in rule) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].${removed} is forbidden; directories own semantics.`);
  }
  for (const [fileKindId, kind] of Object.entries(taxonomy.fileKinds)) for (const extension of kind.extensionChains) {
    const rules = Object.values(taxonomy.fileKindResolutionRules).filter((rule) => rule.extensionChain === extension && rule.fileKindId === fileKindId);
    if (rules.length !== 1) problems.push(`fileKindResolutionRules must own ${JSON.stringify(extension)} exactly once for ${JSON.stringify(fileKindId)}.`);
  }

  if (!(typeof taxonomy.scopedFileKinds === "object" && taxonomy.scopedFileKinds !== null && !Array.isArray(taxonomy.scopedFileKinds))) problems.push("scopedFileKinds must be an object.");
  else for (const [id, spec] of Object.entries(taxonomy.scopedFileKinds)) {
    pathPattern(spec.pathPattern, `scopedFileKinds[${JSON.stringify(id)}].pathPattern`);
    if (spec.parentDirectoryKindId !== undefined && !taxonomy.semanticDirectoryKinds[spec.parentDirectoryKindId]) problems.push(`scopedFileKinds[${JSON.stringify(id)}].parentDirectoryKindId must reference a semantic directory kind.`);
    if (!canonicalEmoji(spec.emoji)) problems.push(`scopedFileKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
    if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`scopedFileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
    for (const extension of spec.extensionChains ?? []) extensionChain(extension, `scopedFileKinds[${JSON.stringify(id)}].extensionChains`);
    if (!["source", "schema", "specification", "configuration", "documentation", "test", "asset", "generated", "marker", "evidence"].includes(spec.role)) problems.push(`scopedFileKinds[${JSON.stringify(id)}].role is invalid.`);
    fullPattern(spec.sourceFilenamePattern, `scopedFileKinds[${JSON.stringify(id)}].sourceFilenamePattern`);
    if (!spec.authority || !spec.reason || !spec.verification) problems.push(`scopedFileKinds[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (!(spec.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(spec.expires))) problems.push(`scopedFileKinds[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }

  if (record(taxonomy.semanticDirectoryKinds, "semanticDirectoryKinds")) {
    const parentKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds, ...taxonomy.fixedDirectoryContracts };
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryKinds)) {
      if (!canonicalEmoji(spec.emoji)) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
      if (typeof spec.slugPattern !== "string") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern must be a string.`);
      else pattern(spec.slugPattern, `semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern`);
      if (typeof spec.allowEmojiOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].allowEmojiOnly must be boolean.`);
      if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].inferWithoutEmoji must be boolean when present.`);
      if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].projectionOnly must be boolean when present.`);
      ids(spec.parentKindIds ?? [], parentKinds, `semanticDirectoryKinds[${JSON.stringify(id)}].parentKindIds`);
    }
  }
  const taxonomyCliArtifactDirectoryKinds: Readonly<Record<string, Readonly<{ name: string; emoji: string; slugPattern: string; parentKindIds?: readonly string[] }>>> = {
    "taxonomy-transaction": { name: "🧾️taxonomy-transaction", emoji: "🧾️", slugPattern: "^taxonomy-transaction$" },
    "transaction-digest": { name: `🔖️${"0".repeat(64)}`, emoji: "🔖️", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-transaction"] },
    "transaction-attempts": { name: "🔂️attempts", emoji: "🔂️", slugPattern: "^attempts$", parentKindIds: ["transaction-digest"] },
    "transaction-attempt": { name: "🔢️000001", emoji: "🔢️", slugPattern: "^[0-9]{6}$", parentKindIds: ["transaction-attempts"] },
    "transaction-stage": { name: "🚧️stage", emoji: "🚧️", slugPattern: "^stage$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-backup": { name: "💾️backup", emoji: "💾️", slugPattern: "^backup$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-lease": { name: "🔒️lease", emoji: "🔒️", slugPattern: "^lease$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-attempt-preparation": { name: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^prepare-[0-9]{6}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-attempts"] },
    "transaction-edit-preparation": { name: "🚧️edit-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^edit-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-stage"] },
    "transaction-edit-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-edit-preparation"] },
    "transaction-journal-write": { name: "🚧️journal", emoji: "🚧️", slugPattern: "^journal$", parentKindIds: ["transaction-stage"] },
    "transaction-json-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-journal-write", "transaction-lease-preparation", "transaction-lease"] },
    "transaction-lease-preparation": { name: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", emoji: "🚧️", slugPattern: "^lease-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}-(preparing|stale)$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-preparation": { name: "🚧️backup-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^backup-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-write-preparation": { name: "🚧️write-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup-preparation"] },
    "transaction-restore-preparation": { name: "🚧️restore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "🚧️", slugPattern: "^restore-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "taxonomy-inventory-data": { name: "📊️taxonomy-inventory", emoji: "📊️", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-data": { name: "📊️taxonomy-plan", emoji: "📊️", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-data": { name: "📊️taxonomy-apply", emoji: "📊️", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-data": { name: "📊️taxonomy-verification", emoji: "📊️", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-summary": { name: "📓️taxonomy-inventory", emoji: "📓️", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-summary": { name: "📓️taxonomy-plan", emoji: "📓️", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-summary": { name: "📓️taxonomy-apply", emoji: "📓️", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-summary": { name: "📓️taxonomy-verification", emoji: "📓️", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-shards": { name: "📊️shards", emoji: "📊️", slugPattern: "^shards$", parentKindIds: ["taxonomy-inventory-data"] },
    "taxonomy-inventory-shard-digest": { name: `🔖️${"0".repeat(64)}`, emoji: "🔖️", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-inventory-shards"] },
  };
  for (const [id, expected] of Object.entries(taxonomyCliArtifactDirectoryKinds)) {
    const spec = taxonomy.semanticDirectoryKinds[id];
    if (!spec) {
      problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must declare the permanent taxonomy CLI artifact directory.`);
      continue;
    }
    exactKeys(spec, expected.parentKindIds ? ["emoji", "slugPattern", "allowEmojiOnly", "parentKindIds"] : ["emoji", "slugPattern", "allowEmojiOnly"], `semanticDirectoryKinds[${JSON.stringify(id)}]`);
    if (spec.emoji !== expected.emoji || spec.slugPattern !== expected.slugPattern || spec.allowEmojiOnly || JSON.stringify(spec.parentKindIds) !== JSON.stringify(expected.parentKindIds)) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must remain the exact taxonomy CLI artifact contract.`);
    if (semanticDirectoryKindId(expected.name, taxonomy, { parentKindId: expected.parentKindIds?.[0] }) !== id) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] does not resolve its canonical taxonomy CLI artifact directory uniquely.`);
  }
  const editCandidate = taxonomy.scopedFileKinds["transaction-edit-candidate"];
  if (!editCandidate || editCandidate.pathPattern !== "**/🚧️stage/🚧️edit-*/*.edit" || editCandidate.parentDirectoryKindId !== "transaction-edit-preparation" || editCandidate.extensionChains.join("\0") !== ".edit" || editCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.edit$") problems.push("scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority.");
  const editPreimage = taxonomy.scopedFileKinds["transaction-edit-preimage"];
  if (!editPreimage || editPreimage.pathPattern !== "**/🚧️stage/🚧️edit-*/*.pre" || editPreimage.parentDirectoryKindId !== "transaction-edit-preparation" || editPreimage.extensionChains.join("\0") !== ".pre" || editPreimage.sourceFilenamePattern !== "^[0-9a-f]{24}\\.pre$") problems.push("scopedFileKinds.transaction-edit-preimage must remain the exact transaction edit displaced-preimage authority.");
  const editWriteCandidate = taxonomy.scopedFileKinds["transaction-edit-write-candidate"];
  if (!editWriteCandidate || editWriteCandidate.pathPattern !== "**/🚧️stage/🚧️edit-*/🚧️write-*/🚧️.edit" || editWriteCandidate.parentDirectoryKindId !== "transaction-edit-write-preparation" || editWriteCandidate.extensionChains.join("\0") !== ".edit" || editWriteCandidate.sourceFilenamePattern !== "^🚧️\\.edit$") problems.push("scopedFileKinds.transaction-edit-write-candidate must remain the exact unpublished edit-writer authority.");
  const backupWriteCandidate = taxonomy.scopedFileKinds["transaction-backup-write-candidate"];
  if (!backupWriteCandidate || backupWriteCandidate.pathPattern !== "**/💾️backup/🚧️backup-*/🚧️write-*/🚧️.backup" || backupWriteCandidate.parentDirectoryKindId !== "transaction-backup-write-preparation" || backupWriteCandidate.extensionChains.join("\0") !== ".backup" || backupWriteCandidate.sourceFilenamePattern !== "^🚧️\\.backup$") problems.push("scopedFileKinds.transaction-backup-write-candidate must remain the exact unpublished backup-writer authority.");
  const jsonPrevious = taxonomy.scopedFileKinds["transaction-json-previous"];
  if (!jsonPrevious || jsonPrevious.pathPattern !== "**/🚧️write-*/⏮️.json" || jsonPrevious.parentDirectoryKindId !== "transaction-json-write-preparation" || jsonPrevious.extensionChains.join("\0") !== ".json" || jsonPrevious.sourceFilenamePattern !== "^⏮️\\.json$") problems.push("scopedFileKinds.transaction-json-previous must remain the exact displaced canonical JSON authority.");
  const backupCandidate = taxonomy.scopedFileKinds["transaction-backup-candidate"];
  if (!backupCandidate || backupCandidate.pathPattern !== "**/💾️backup/🚧️restore-*/*.backup" || backupCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || backupCandidate.extensionChains.join("\0") !== ".backup" || backupCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.backup$") problems.push("scopedFileKinds.transaction-backup-candidate must remain the exact transaction restore preimage authority.");
  const postimageCandidate = taxonomy.scopedFileKinds["transaction-postimage-candidate"];
  if (!postimageCandidate || postimageCandidate.pathPattern !== "**/💾️backup/🚧️restore-*/*.post" || postimageCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || postimageCandidate.extensionChains.join("\0") !== ".post" || postimageCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.post$") problems.push("scopedFileKinds.transaction-postimage-candidate must remain the exact transaction restore postimage authority.");
  if (semanticDirectoryKindId("🚧️write-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease-preparation" }) !== "transaction-json-write-preparation" || semanticDirectoryKindId("🚧️write-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease" }) !== "transaction-json-write-preparation") problems.push("semanticDirectoryKinds.transaction-json-write-preparation must resolve below all three exact parent kinds.");
  for (const [name, expectedKindId] of [["🚧️stage", "transaction-stage"], ["💾️backup", "transaction-backup"], ["🔒️lease", "transaction-lease"]] as const) if (semanticDirectoryKindId(name, taxonomy, { parentKindId: "transaction-attempt-preparation" }) !== expectedKindId) problems.push(`semanticDirectoryKinds.${expectedKindId} must resolve below transaction-attempt-preparation.`);
  const attemptPreparation = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", children: [{ name: "🚧️stage", nodeKind: "directory" }, { name: "💾️backup", nodeKind: "directory" }, { name: "🔒️lease", nodeKind: "directory" }, { name: "🔣️.json", nodeKind: "file" }] } as const;
  if (taxonomyCliAttemptPreparationsProblems([attemptPreparation], taxonomy).length > 0 || taxonomyCliAttemptPreparationsProblems([attemptPreparation, attemptPreparation], taxonomy).length === 0) problems.push("transaction-attempt-preparation must retain its validate-all exact child and duplicate-sibling authority.");
  const leasePreparation = { parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", leafNames: ["🔣️.json"], writePreparations: [] } as const;
  if (taxonomyCliLeaseDirectoryProblems(leasePreparation, taxonomy).length > 0) problems.push("transaction-lease-preparation must admit one complete canonical JSON lease before publication.");

  if (record(taxonomy.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds")) {
    const allDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
    const ownerMembers = new Set<string>();
    const ownerPatterns = new Set<string>();
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryMemberKinds)) {
      ids(spec.ownerKindIds, allDirectoryKinds, `semanticDirectoryMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      exactKeys(spec, spec.memberSlugPattern === undefined ? ["ownerKindIds", "memberNames", "source"] : ["ownerKindIds", "memberNames", "memberSlugPattern", "source"], `semanticDirectoryMemberKinds[${JSON.stringify(id)}]`);
      if (!Array.isArray(spec.memberNames) || (spec.memberNames.length === 0 && spec.memberSlugPattern === undefined)) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].memberNames must be non-empty without a memberSlugPattern.`);
      if (spec.memberSlugPattern !== undefined) {
        const key = `semanticDirectoryMemberKinds[${JSON.stringify(id)}].memberSlugPattern`;
        fullPattern(spec.memberSlugPattern, key);
        if (typeof spec.memberSlugPattern === "string") {
          try {
            if (new RegExp(spec.memberSlugPattern, "u").test("")) problems.push(`${key} must not match an empty slug.`);
          } catch {}
          for (const owner of spec.ownerKindIds ?? []) {
            const collision = `${owner}\0${spec.memberSlugPattern}`;
            if (ownerPatterns.has(collision)) problems.push(`semanticDirectoryMemberKinds pattern rows collide for owner ${JSON.stringify(owner)} and pattern ${JSON.stringify(spec.memberSlugPattern)}.`);
            ownerPatterns.add(collision);
          }
        }
      }
      for (const name of spec.memberNames ?? []) {
        if (typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name) || !canonicalEmoji(leadingEmojiIdentity(name).first)) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}] has invalid exact member ${JSON.stringify(name)}.`);
        for (const owner of spec.ownerKindIds ?? []) {
          const key = `${owner}\0${name}`;
          if (ownerMembers.has(key)) problems.push(`semanticDirectoryMemberKinds collide for owner ${JSON.stringify(owner)} and member ${JSON.stringify(name)}.`);
          ownerMembers.add(key);
        }
      }
      if (spec.source !== "registry") problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].source must be registry.`);
    }
  }

  //#region 🪞️SemanticPathProjection
  const projectionDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
  if (record(taxonomy.semanticProjectedMemberKinds, "semanticProjectedMemberKinds")) {
    for (const [id, spec] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      kebabId(id, `semanticProjectedMemberKinds id ${JSON.stringify(id)}`);
      exactKeys(spec, ["ownerKindIds", "projectionContractId", "sourceMemberKindId", "identityField"], `semanticProjectedMemberKinds[${JSON.stringify(id)}]`);
      ids(spec.ownerKindIds, projectionDirectoryKinds, `semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      if (spec.ownerKindIds.length === 0) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds must be non-empty.`);
      if (!taxonomy.semanticPathProjectionContracts[spec.projectionContractId] && spec.projectionContractId !== taxonomy.mutationCatalogProjection.contractId) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].projectionContractId is missing.`);
      if (!taxonomy.semanticDirectoryMemberKinds[spec.sourceMemberKindId]) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].sourceMemberKindId is missing.`);
      const expectedIdentityField = spec.projectionContractId === taxonomy.mutationCatalogProjection.contractId ? "mutationDirectoryName" : spec.projectionContractId === "artifact-editor-command-bundle-v1" ? "commandDirectoryName" : null;
      if (spec.identityField !== expectedIdentityField) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].identityField does not match its projection contract.`);
    }
    const visiting = new Set<string>();
    const visited = new Set<string>();
    const visit = (id: string): void => {
      if (visiting.has(id)) {
        problems.push(`semanticProjectedMemberKinds contains an owner cycle at ${JSON.stringify(id)}.`);
        return;
      }
      if (visited.has(id)) return;
      visiting.add(id);
      for (const owner of taxonomy.semanticProjectedMemberKinds[id]?.ownerKindIds ?? []) if (taxonomy.semanticProjectedMemberKinds[owner]) visit(owner);
      visiting.delete(id);
      visited.add(id);
    };
    for (const id of Object.keys(taxonomy.semanticProjectedMemberKinds)) visit(id);
  }

  if (record(taxonomy.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers")) for (const [id, renderer] of Object.entries(taxonomy.semanticPathProjectionProfileRenderers)) {
    kebabId(id, `semanticPathProjectionProfileRenderers id ${JSON.stringify(id)}`);
    exactKeys(renderer, ["direction", "captureFields", "directoryKindId", "template", "tupleCollisionFields"], `semanticPathProjectionProfileRenderers[${JSON.stringify(id)}]`);
    if (renderer.direction !== "forward-only") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].direction must be forward-only.`);
    if (renderer.captureFields.join("\0") !== "standardVersion\0subsetId") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].captureFields must be exactly standardVersion, subsetId.`);
    if (!taxonomy.semanticDirectoryKinds[renderer.directoryKindId]) problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].directoryKindId is missing.`);
    if (renderer.template !== "🪆️{standardVersion}-{subsetId}") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].template must be the exact forward profile renderer.`);
    if (renderer.tupleCollisionFields.join("\0") !== "artifactId\0standardVersion\0subsetId") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].tupleCollisionFields must be exactly artifactId, standardVersion, subsetId.`);
  }

  const validateDescendantNode = (node: SemanticDescendantNode, key: string, rootKindId: string): string | null => {
    const authorityKey = node.nodeType === "file" && !("kindId" in node) ? "fixedFilenameContractId" : "kindId";
    exactKeys(node, ["pathSegments", "nodeType", authorityKey, ...("sourceFilename" in node ? ["sourceFilename"] : [])], key);
    const validateSegments = (segments: readonly Readonly<{ kindId: string; literal: string }>[] | unknown, field: string): void => {
      if (!Array.isArray(segments)) {
        problems.push(`${key}.${field} must be an array.`);
        return;
      }
      let parentKindId = rootKindId;
      for (const [index, segment] of segments.entries()) {
        exactKeys(segment, ["kindId", "literal"], `${key}.${field}[${index}]`);
        if (!taxonomy.semanticDirectoryKinds[segment.kindId]) problems.push(`${key}.${field}[${index}].kindId is missing.`);
        else if (semanticDirectoryKindId(segment.literal, taxonomy, { parentKindId }) !== segment.kindId) problems.push(`${key}.${field}[${index}].literal does not resolve uniquely to its kind.`);
        parentKindId = segment.kindId;
      }
    };
    validateSegments(node.pathSegments, "pathSegments");
    if (node.nodeType === "directory") {
      if (!projectionDirectoryKinds[node.kindId]) problems.push(`${key}.kindId is not a directory kind.`);
      const expected = node.pathSegments.at(-1)?.kindId ?? rootKindId;
      if (node.kindId !== expected) problems.push(`${key}.kindId must equal its realized directory kind ${JSON.stringify(expected)}.`);
    } else if (node.nodeType === "file") {
      if ("kindId" in node && !taxonomy.fileKinds[node.kindId]) problems.push(`${key}.kindId is not a file kind.`);
      else if ("kindId" in node && node.sourceFilename !== undefined && (node.sourceFilename !== node.sourceFilename.normalize("NFC") || /[\\/]/u.test(node.sourceFilename) || !node.sourceFilename.startsWith(taxonomy.fileKinds[node.kindId]!.emoji) || fileKindIdForSourcePath(node.sourceFilename, taxonomy) !== node.kindId)) problems.push(`${key}.sourceFilename must be one NFC basename resolving to kindId.`);
      else if (!("kindId" in node) && !taxonomy.fixedFilenameContracts[node.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId is missing.`);
    } else problems.push(`${key}.nodeType must be directory or file.`);
    try {
      return `${node.nodeType}:${semanticDescendantNodeRelativePath(node, taxonomy)}`;
    } catch {
      return null;
    }
  };

  if (record(taxonomy.semanticDescendantContracts, "semanticDescendantContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticDescendantContracts)) {
    kebabId(id, `semanticDescendantContracts id ${JSON.stringify(id)}`);
    if ("contractKind" in contract) {
      exactKeys(contract, ["contractKind", "rootDirectoryKindId", "catalogContractId", "leafFileKindId", "rendering", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
      if (contract.contractKind !== "catalog") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].contractKind must be catalog.`);
      if (!taxonomy.semanticDirectoryKinds[contract.rootDirectoryKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
      const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
      if (!catalog || !("contractKind" in catalog) || catalog.contractKind !== "distributed-json-manifest-catalog") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].catalogContractId must reference one distributed JSON manifest catalog.`);
      if (!taxonomy.fileKinds[contract.leafFileKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].leafFileKindId is missing.`);
      if (contract.rendering !== "semantic-member-directory-and-physical-kind-leaf") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rendering is invalid.`);
      exactKeys(contract.pathBudgetReserve, ["derivation", "bytes"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
      if (contract.pathBudgetReserve.derivation !== "longest-rendered-catalog-descendant-suffix") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
      if (!Number.isSafeInteger(contract.pathBudgetReserve.bytes) || contract.pathBudgetReserve.bytes <= 0 || contract.pathBudgetReserve.bytes >= taxonomy.collisionPolicy.maxPathBytes) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.bytes must be a positive safe integer below maxPathBytes.`);
      continue;
    }
    exactKeys(contract, ["rootDirectoryKindId", "requiredNodes", "exclusiveAlternatives", "realizedNodeCount", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
    if (!projectionDirectoryKinds[contract.rootDirectoryKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
    if (!Array.isArray(contract.requiredNodes) || contract.requiredNodes.length === 0) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be non-empty.`);
    const requiredKeys = (contract.requiredNodes ?? []).map((node, index) => validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes[${index}]`, contract.rootDirectoryKindId)).filter((key): key is string => key !== null);
    if (new Set(requiredKeys).size !== requiredKeys.length) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be unique.`);
    if (!Array.isArray(contract.exclusiveAlternatives)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives must be an array.`);
    const alternativeIds = new Set<string>();
    const alternativeKeys: string[] = [];
    for (const [groupIndex, alternative] of (contract.exclusiveAlternatives ?? []).entries()) {
      exactKeys(alternative, ["id", "mode", "nodes"], `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}]`);
      kebabId(alternative.id, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].id`);
      if (alternativeIds.has(alternative.id)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] has duplicate alternative id ${JSON.stringify(alternative.id)}.`);
      alternativeIds.add(alternative.id);
      if (alternative.mode !== "exactly-one") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].mode must be exactly-one.`);
      if (!Array.isArray(alternative.nodes) || alternative.nodes.length < 2) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes must contain at least two nodes.`);
      for (const [nodeIndex, node] of (alternative.nodes ?? []).entries()) {
        const nodeKey = validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes[${nodeIndex}]`, contract.rootDirectoryKindId);
        if (nodeKey) alternativeKeys.push(nodeKey);
      }
    }
    const allKeys = [...requiredKeys, ...alternativeKeys];
    if (new Set(allKeys).size !== allKeys.length) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] descendant nodes must not overlap.`);
    const requiredRealizedKeys = new Set(requiredKeys);
    for (const key of requiredKeys) if (key.startsWith("file:")) {
      const segments = key.slice(5).split("/");
      segments.pop();
      let directory = "";
      for (const segment of segments) {
        directory = directory ? `${directory}/${segment}` : segment;
        requiredRealizedKeys.add(`directory:${directory}`);
      }
    }
    if (contract.realizedNodeCount !== requiredRealizedKeys.size + (contract.exclusiveAlternatives?.length ?? 0)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].realizedNodeCount must equal realized required destination nodes plus one node per exclusive group.`);
    exactKeys(contract.pathBudgetReserve, ["derivation", "bytes"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
    if (contract.pathBudgetReserve.derivation !== "longest-canonical-descendant-suffix") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
    const derivedBytes = Math.max(0, ...allKeys.map((key) => new TextEncoder().encode(`/${key.slice(key.indexOf(":") + 1)}`).length));
    if (contract.pathBudgetReserve.bytes !== derivedBytes) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.bytes must equal derived longest suffix ${derivedBytes}.`);
    if (contract.pathBudgetReserve.bytes >= taxonomy.collisionPolicy.maxPathBytes) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve must be below maxPathBytes.`);
    if (id === "mutation-implementation-case-v1") {
      const expectedRequired = ["directory:", "file:🦀️.rs"].sort();
      if ([...requiredKeys].sort().join("\0") !== expectedRequired.join("\0") || alternativeKeys.length !== 0 || contract.realizedNodeCount !== 2 || contract.pathBudgetReserve.bytes !== 11) problems.push("semanticDescendantContracts.mutation-implementation-case-v1 must encode the exact two-node Rust implementation case and 11-byte reserve.");
    }
    if (id === "mutation-fixture-bundle-v1") {
      const expectedRequired = ["directory:", "directory:🦠️mutation", "file:🦠️mutation/🔣️.json", "directory:📸️snapshot", "directory:📸️snapshot/⬅️before", "file:📸️snapshot/⬅️before/🔣️.json", "directory:📸️snapshot/➡️after", "file:📸️snapshot/➡️after/🔣️.json", "directory:🔺️diff", "directory:🎯️outcome", "file:🎯️outcome/🔣️.json"].sort();
      const expectedAlternatives = ["file:🔺️diff/🔣️.json", "file:🔺️diff/🚫️.absent"].sort();
      if ([...requiredKeys].sort().join("\0") !== expectedRequired.join("\0") || [...alternativeKeys].sort().join("\0") !== expectedAlternatives.join("\0") || contract.realizedNodeCount !== 12) problems.push("semanticDescendantContracts.mutation-fixture-bundle-v1 must encode the exact 12-node data bundle and exclusive diff alternatives.");
    }
    if (id === "draw-editor-command-bundle-v1") {
      const manifestSuffix = "/📦️packages/🦀️rust/Cargo.toml";
      const actual = new Map((contract.requiredNodes ?? []).flatMap((node): (readonly [string, string])[] => {
        try {
          return [[`${node.nodeType}:${semanticDescendantNodeRelativePath(node, taxonomy)}`, node.nodeType === "directory" ? "directory" : "kindId" in node ? `${node.kindId}:${node.sourceFilename ?? ""}` : `fixed:${node.fixedFilenameContractId}`]];
        } catch {
          return [];
        }
      }));
      const expected = new Map<string, string>([["directory:", "directory"], ["file:🦀️.rs", "rust-source:🦀️.rs"]]);
      for (const owner of [...actual.keys()].filter((key) => key.startsWith("file:") && key.endsWith(manifestSuffix)).map((key) => key.slice("file:".length, -manifestSuffix.length))) {
        for (const [name, authority] of [["🦀️.rs", "rust-source:🦀️.rs"], ["📦️packages/🦀️rust/Cargo.toml", "fixed:cargo-manifest"], ["📦️packages/🦀️rust/📋️project.json", "fixed:build-project-manifest"], ["📦️packages/🦀️rust/📜️script.ts", "fixed:root-script"]] as const) expected.set(`file:${owner}/${name}`, authority);
        const segments = `${owner}/📦️packages/🦀️rust`.split("/");
        for (let index = 1; index <= segments.length; index++) expected.set(`directory:${segments.slice(0, index).join("/")}`, "directory");
      }
      if (alternativeKeys.length !== 0 || actual.size !== expected.size || [...expected].some(([key, authority]) => actual.get(key) !== authority)) problems.push("semanticDescendantContracts.draw-editor-command-bundle-v1 must encode one root Rust command declaration and only declaration-only nested Rust packages.");
    }
  }

  if (record(taxonomy.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionCatalogContracts)) {
    kebabId(id, `semanticPathProjectionCatalogContracts id ${JSON.stringify(id)}`);
    if ("contractKind" in contract && contract.contractKind === "distributed-json-manifest-catalog") {
      exactKeys(contract, ["contractKind", "ownerArtifactMemberName", "profileVectors", "modelManifestSchema", "modelManifestSourceFilename", "modelIdentityField", "memberIdentityField", "memberVersionField", "requiredMemberVersion", "requiredModelManifest", "categoryRules", "coverage", "unknownCategoryPolicy", "unownedModelPolicy"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (!contract.ownerArtifactMemberName || !contract.modelManifestSchema || !contract.modelManifestSourceFilename || contract.modelIdentityField !== "id" || contract.memberIdentityField !== "id" || contract.memberVersionField !== "version" || !contract.requiredMemberVersion || contract.requiredModelManifest !== true) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must declare exact non-empty manifest authority fields.`);
      if (!Array.isArray(contract.profileVectors) || contract.profileVectors.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].profileVectors must be non-empty.`);
      const profiles = new Set<string>();
      for (const [index, vector] of (contract.profileVectors ?? []).entries()) {
        const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].profileVectors[${index}]`;
        exactKeys(vector, ["artifactId", "standardVersion", "subsetId"], scope);
        const key = [vector.artifactId, vector.standardVersion, vector.subsetId].join("\0");
        if (vector.artifactId !== contract.ownerArtifactMemberName || !vector.standardVersion || !vector.subsetId || profiles.has(key) || [vector.standardVersion, vector.subsetId].some((value) => value !== value.normalize("NFC") || /[\\/]/u.test(value))) problems.push(`${scope} must be one unique NFC owner profile tuple.`);
        profiles.add(key);
      }
      if (!Array.isArray(contract.categoryRules) || contract.categoryRules.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules must be non-empty.`);
      const categoryNames = new Set<string>();
      for (const [index, rule] of (contract.categoryRules ?? []).entries()) {
        const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules[${index}]`;
        exactKeys(rule, rule.sourceShape === "direct-semantic-json" ? ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "memberDirectoryEmoji"] : ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "fixedSourceFilename"], scope);
        if (categoryNames.has(rule.sourceDirectoryName)) problems.push(`${scope}.sourceDirectoryName is duplicated.`);
        categoryNames.add(rule.sourceDirectoryName);
        if (!taxonomy.semanticDirectoryKinds[rule.directoryKindId]) problems.push(`${scope}.directoryKindId is missing.`);
        if (!rule.manifestSchema) problems.push(`${scope}.manifestSchema must be non-empty.`);
        if (rule.sourceShape === "direct-semantic-json") {
          if (!/^\p{Extended_Pictographic}\uFE0F$/u.test(rule.memberDirectoryEmoji)) problems.push(`${scope}.memberDirectoryEmoji must be one emoji plus U+FE0F.`);
        } else if (rule.sourceShape === "nested-fixed-json") {
          if (!rule.fixedSourceFilename) problems.push(`${scope}.fixedSourceFilename must be non-empty.`);
        } else problems.push(`${scope}.sourceShape is invalid.`);
      }
      if (contract.coverage !== "every-source-file-and-destination-node-exactly-once" || contract.unknownCategoryPolicy !== "problem" || contract.unownedModelPolicy !== "problem") problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must fail closed over exact source and destination ownership.`);
      continue;
    }
    if ("contractKind" in contract && contract.contractKind === "exact-owner-vectors") {
      exactKeys(contract, ["contractKind", "required", "allowEmpty", "identityFields", "coverage", "vectors"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (contract.required !== true || contract.allowEmpty !== false || contract.identityFields.join("\0") !== "artifactId\0standardVersion\0subsetId\0commandDirectoryName" || contract.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(contract.vectors) || contract.vectors.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact non-empty command owner-vector contract.`);
      const owners = new Set<string>();
      for (const [index, vector] of (contract.vectors ?? []).entries()) {
        exactKeys(vector, ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}]`);
        const owner = [vector.artifactId, vector.standardVersion, vector.subsetId, vector.commandDirectoryName].join("\0");
        if (!vector.artifactId || !vector.standardVersion || !vector.subsetId || !vector.commandDirectoryName || owners.has(owner)) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}] must be one unique non-empty owner tuple.`);
        owners.add(owner);
      }
      continue;
    }
    exactKeys(contract, ["registryField", "required", "allowEmpty", "runtimeKindsField", "runtimeKindsRelation", "mutationIdField", "sourceMutationDirectoryNameField", "mutationDirectoryNameField", "scenariosField", "scenarioIdField", "scenarioDirectoryNameField", "sourceBundleUniquenessFields", "canonicalBundleUniquenessFields", "coverage"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
    const expected: SemanticPathProjectionCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
    if (JSON.stringify(contract) !== JSON.stringify(expected)) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact independent physical vectors contract.`);
  }

  if (record(taxonomy.semanticPathProjectionContracts, "semanticPathProjectionContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionContracts)) {
    kebabId(id, `semanticPathProjectionContracts id ${JSON.stringify(id)}`);
    exactKeys(contract, ["sourceOwnerKindId", ...(contract.sourceArtifactMemberName === undefined ? [] : ["sourceArtifactMemberName"]), "sourceSegments", "profileRendererId", "destinationOwnerKindId", "destinationSegments", "descendantContractId", "catalogContractId", "rationaleRule"], `semanticPathProjectionContracts[${JSON.stringify(id)}]`);
    if (typeof contract.sourceArtifactMemberName !== "string" || !taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]?.memberNames.includes(contract.sourceArtifactMemberName)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceArtifactMemberName must be one exact source-owner member.`);
    if (!taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceOwnerKindId is missing.`);
    if (!taxonomy.semanticDirectoryMemberKinds[contract.destinationOwnerKindId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationOwnerKindId is missing.`);
    if (!taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].profileRendererId is missing.`);
    if (!taxonomy.semanticDescendantContracts[contract.descendantContractId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].descendantContractId is missing.`);
    if (!taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].catalogContractId is missing.`);
    if (!["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(contract.rationaleRule)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].rationaleRule is invalid.`);
    const captures = new Set<string>();
    let sourceParentKindId: string | undefined;
    for (const [index, segment] of contract.sourceSegments.entries()) {
      const value = segment as unknown as Record<string, unknown>;
      const hasProjected = typeof value.projectedMemberKindId === "string";
      const hasMember = typeof value.memberKindId === "string";
      exactKeys(value, hasProjected ? ["projectedMemberKindId", "capture"] : hasMember ? ["memberKindId", "literal"] : ["kindId", "literal" in value ? "literal" : "capture"], `semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}]`);
      const kindId = (hasProjected ? value.projectedMemberKindId : hasMember ? value.memberKindId : value.kindId) as string;
      const kind = hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : hasMember ? taxonomy.semanticDirectoryMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId];
      if (!kind) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}] references a missing kind.`);
      if (typeof value.literal === "string") {
        const validLiteral = hasMember
          ? Boolean(taxonomy.semanticDirectoryMemberKinds[kindId]?.ownerKindIds.includes(sourceParentKindId ?? "") && taxonomy.semanticDirectoryMemberKinds[kindId]?.memberNames.includes(value.literal))
          : semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: sourceParentKindId }) === kindId;
        if (!validLiteral) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].literal does not resolve to its kind.`);
      }
      if (typeof value.capture === "string") {
        if (!["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"].includes(value.capture)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].capture is invalid.`);
        if (captures.has(value.capture)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] captures ${JSON.stringify(value.capture)} more than once.`);
        captures.add(value.capture);
      }
      sourceParentKindId = kindId;
    }
    const requiredCaptures = contract.rationaleRule === "artifact-example-model-catalog-projection-v1" ? ["standardVersion", "subsetId"] : ["standardVersion", "subsetId", "commandDirectoryName"];
    if (captures.size !== requiredCaptures.length || requiredCaptures.some((field) => !captures.has(field))) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must capture exactly ${requiredCaptures.join(", ")}.`);
    let renderedProfiles = 0;
    let destinationParentKindId: string | undefined;
    for (const [index, segment] of contract.destinationSegments.entries()) {
      const value = segment as unknown as Record<string, unknown>;
      const hasProjected = typeof value.projectedMemberKindId === "string";
      const operation = "literal" in value ? "literal" : "render" in value ? "render" : "copy";
      exactKeys(value, [hasProjected ? "projectedMemberKindId" : "kindId", operation], `semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}]`);
      const kindId = (hasProjected ? value.projectedMemberKindId : value.kindId) as string;
      if (!(hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId])) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}] references a missing kind.`);
      if (typeof value.literal === "string" && semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: destinationParentKindId }) !== kindId) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].literal does not resolve to its kind.`);
      if (value.render !== undefined) {
        if (value.render !== "profile") problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].render must be profile.`);
        renderedProfiles += 1;
      }
      if (typeof value.copy === "string" && !captures.has(value.copy)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].copy must reference a source capture.`);
      destinationParentKindId = kindId;
    }
    if (renderedProfiles !== 1) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render exactly one profile segment.`);
    const projectedReferences = [...contract.sourceSegments, ...contract.destinationSegments].flatMap((segment) => "projectedMemberKindId" in segment ? [segment.projectedMemberKindId] : []);
    for (const [projectedId, projected] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      const references = projectedReferences.filter((candidate) => candidate === projectedId).length;
      if (projected.projectionContractId === id && references !== 2) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must reference projected member ${JSON.stringify(projectedId)} exactly once in source and destination.`);
    }
    if (contract.sourceArtifactMemberName) {
      try {
        if (artifactPathProjectionCatalogRoots(`🗿️artifacts/${contract.sourceArtifactMemberName}`, id, taxonomy).length === 0) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render at least one exact forward owner profile.`);
      } catch (error) { problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] has invalid forward profile authority: ${error instanceof Error ? error.message : String(error)}`); }
    }
  }

  if (record(taxonomy.semanticPolicyStateCoordinateContracts, "semanticPolicyStateCoordinateContracts")) {
    if (Object.keys(taxonomy.semanticPolicyStateCoordinateContracts).join("\0") !== "dependency-freeze-users-v1") problems.push("semanticPolicyStateCoordinateContracts requires the exact dependency policy-state contract.");
    for (const [id, contract] of Object.entries(taxonomy.semanticPolicyStateCoordinateContracts)) {
      exactKeys(contract, ["contractKind", "statePath", "stateSchemaVersion", "sourceDisposition", "ownerProjectionContractId", "packageIds", "manifestFilename", "dependencyEvidenceField", "coordinatePointer", "preserveNonCoordinateBytes"], `semanticPolicyStateCoordinateContracts.${id}`);
      if (contract.contractKind !== "dependency-freeze-user-coordinates" || contract.statePath !== "🔒️dependencies.json" || contract.stateSchemaVersion !== 2 || contract.sourceDisposition !== "authored-policy-state" || contract.ownerProjectionContractId !== "nested-cargo-packages-v1" || !taxonomy.semanticPackageProjectionContracts[contract.ownerProjectionContractId] || JSON.stringify(contract.packageIds) !== JSON.stringify(["jcoprobe-guest"]) || contract.manifestFilename !== "Cargo.toml" || contract.dependencyEvidenceField !== "witDependency" || contract.coordinatePointer !== "/entries/*/users/*" || contract.preserveNonCoordinateBytes !== true) problems.push(`semanticPolicyStateCoordinateContracts.${id} has invalid exact user-coordinate authority.`);
    }
  }
  if (record(taxonomy.semanticPackageProjectionContracts, "semanticPackageProjectionContracts")) {
    if (Object.keys(taxonomy.semanticPackageProjectionContracts).join("\0") !== "nested-cargo-packages-v1") problems.push("semanticPackageProjectionContracts requires the exact nested Cargo contract.");
    for (const [id, contract] of Object.entries(taxonomy.semanticPackageProjectionContracts)) {
      exactKeys(contract, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "packageIds", "sourceLeafCounts", "purityCount", "adapterCount", "derivedLeafCount", "joinedPathBindingCounts", "generatedSourceRetirementCounts", "authoredFragmentCounts", "rationaleRule"], `semanticPackageProjectionContracts.${id}`);
      if (contract.contractKind !== "exact-nested-cargo-package-catalog" || !exampleAssetPath(contract.authorityCatalogPath, taxonomy) || !/^[0-9a-f]{64}$/u.test(contract.authorityCatalogSha256) || JSON.stringify(contract.packageIds) !== JSON.stringify(["wgpu-renderer", "jcoprobe-guest"]) || JSON.stringify(contract.sourceLeafCounts) !== JSON.stringify([32, 4]) || contract.purityCount !== 27 || contract.adapterCount !== 5 || contract.derivedLeafCount !== 1 || JSON.stringify(contract.joinedPathBindingCounts) !== JSON.stringify([1, 0]) || JSON.stringify(contract.generatedSourceRetirementCounts) !== JSON.stringify([1, 0]) || JSON.stringify(contract.authoredFragmentCounts) !== JSON.stringify([31, 0]) || contract.rationaleRule !== "nested-cargo-package-projection-v1") problems.push(`semanticPackageProjectionContracts.${id} has invalid exact authority.`);
    }
  }

  if (record(taxonomy.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts")) {
    const ownerTuples = new Set<string>();
    for (const [id, contract] of Object.entries(taxonomy.semanticOwnedFileProjectionContracts)) {
      const scope = `semanticOwnedFileProjectionContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      if (!contract || typeof contract !== "object" || Array.isArray(contract)) {
        problems.push(`${scope} must be an object.`);
        continue;
      }
      if (contract.contractKind === "exact-owner-path-catalog") {
        exactKeys(contract, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "sourceFileKindId", "sourceBasenames", "destinationDirectoryKinds", "allowedDispositions", "ownerEvidenceKinds", "referenceOwnerIds", "generatorOwnerIds", "authoredDocumentCorrections", "rationaleRule", ...(Object.hasOwn(contract, "currentSourceRevisions") ? ["currentSourceRevisions"] : [])], scope);
        if (Object.hasOwn(contract, "currentSourceRevisions")) {
          try {
            const revisions = parseCurrentSourceApprovals(contract.currentSourceRevisions);
            for (const [revisionId, revision] of Object.entries(revisions)) if (!exampleAssetPath(revision.approvalPath, taxonomy)) problems.push(scope + ".currentSourceRevisions." + revisionId + ".approvalPath must be beneath " + taxonomy.exampleAssetsDirName + ".");
          } catch (error) { problems.push(scope + ".currentSourceRevisions: " + (error instanceof Error ? error.message : String(error))); }
        }
        if (id !== "readme-license-owner-leaves-v1" || contract.rationaleRule !== "readme-license-owner-projection-v1") problems.push(scope + " must be the exact README/LICENSE owner projection contract.");
        if (!exampleAssetPath(contract.authorityCatalogPath, taxonomy)) problems.push(scope + ".authorityCatalogPath must be one repository-local path beneath " + taxonomy.exampleAssetsDirName + ".");
        if (typeof contract.authorityCatalogSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(contract.authorityCatalogSha256)) problems.push(scope + ".authorityCatalogSha256 must be one SHA-256 digest.");
        if (!taxonomy.fileKinds[contract.sourceFileKindId] || contract.sourceFileKindId !== "markdown") problems.push(scope + ".sourceFileKindId must be markdown.");
        if (!Array.isArray(contract.sourceBasenames) || contract.sourceBasenames.join("\0") !== "LICENSE.md\0README.md") problems.push(scope + ".sourceBasenames must be exactly LICENSE.md and README.md.");
        if (!contract.destinationDirectoryKinds || typeof contract.destinationDirectoryKinds !== "object" || Array.isArray(contract.destinationDirectoryKinds)) problems.push(scope + ".destinationDirectoryKinds must be an object.");
        else {
          exactKeys(contract.destinationDirectoryKinds, ["license", "readme"], scope + ".destinationDirectoryKinds");
          const expectedDestinations = {
            license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" },
            readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" },
          } as const;
          for (const kind of ["license", "readme"] as const) {
            const destination = contract.destinationDirectoryKinds[kind];
            exactKeys(destination, ["directoryKindId", "directoryName", "filename"], scope + ".destinationDirectoryKinds." + kind);
            if (JSON.stringify(destination) !== JSON.stringify(expectedDestinations[kind])) problems.push(scope + ".destinationDirectoryKinds." + kind + " is not canonical.");
            const directory = taxonomy.semanticDirectoryKinds[destination.directoryKindId];
            if (!directory || directory.projectionOnly !== true || semanticDirectoryKindId(destination.directoryName, taxonomy) !== destination.directoryKindId || fileKindIdForSourcePath(destination.filename, taxonomy) !== contract.sourceFileKindId) problems.push(scope + ".destinationDirectoryKinds." + kind + " does not resolve through the registered projection-only directory and Markdown leaf kinds.");
          }
        }
        const exactArrays = [
          [contract.allowedDispositions, "attribution-relocate\0configurable-owner-license-relocate\0fixed\0generated-evidence-relocate\0owner-documentation-relocate", "allowedDispositions"],
          [contract.ownerEvidenceKinds, "configurable-owner-license\0ordinary-owner-doc\0package-publication\0third-party-attribution\0ticket-evidence\0ticket-scratch", "ownerEvidenceKinds"],
        ] as const;
        for (const [values, expected, field] of exactArrays) if (!Array.isArray(values) || values.join("\0") !== expected) problems.push(scope + "." + field + " is not the declared domain vocabulary.");
        for (const [values, field] of [[contract.referenceOwnerIds, "referenceOwnerIds"], [contract.generatorOwnerIds, "generatorOwnerIds"]] as const) if (!Array.isArray(values) || new Set(values).size !== values.length || values.some(id => typeof id !== "string" || !/^[a-z][a-z0-9-]*$/u.test(id))) problems.push(scope + "." + field + " must declare unique domain registry identities.");
        if (contract.generatorOwnerIds.some(id => !taxonomy.generatorContracts[id])) problems.push(scope + " has an unregistered generator owner.");
        try { parseSemanticOwnedDocumentCorrections(contract.authoredDocumentCorrections); } catch (error) { problems.push(scope + ".authoredDocumentCorrections: " + (error instanceof Error ? error.message : String(error))); }
        if (taxonomy.fixedFilenameContracts["root-script"]?.pathPattern !== "**/📜️script.ts") problems.push(scope + ".authoredDocumentCorrections requires the canonical root-script filename contract.");
        continue;
      }
      if (contract.contractKind === "semantic-facet-primary-file") {
        exactKeys(contract, ["contractKind", "sourceRoot", "sourceFilename", "fileKindAuthority", "sourceDisposition", "directoryCaptures", "ownerPathPatterns", "authoringCommand", "referenceConsumer", "rationaleRule"], scope);
        if (id !== "artifact-empty-facet-primary-markdown-v1" || contract.rationaleRule !== id || contract.sourceRoot !== "✏️s/🔌️plugins" || contract.sourceFilename !== "📌️.empty.md" || contract.fileKindAuthority !== "windowEmptyFacetFileKindId" || contract.sourceDisposition !== "authored" || taxonomy.windowEmptyFacetFileKindId !== "markdown") problems.push(`${scope} must use the exact authored empty-facet primary-leaf grammar.`);
        if (record(contract.directoryCaptures, `${scope}.directoryCaptures`)) for (const [capture, spec] of Object.entries(contract.directoryCaptures)) {
          if (!record(spec, `${scope}.directoryCaptures.${capture}`)) continue;
          exactKeys(spec, ["kindIds", ...(spec.names ? ["names"] : [])], `${scope}.directoryCaptures.${capture}`);
          if (!/^[a-zA-Z]+$/u.test(capture) || !Array.isArray(spec.kindIds) || spec.kindIds.length === 0 || new Set(spec.kindIds).size !== spec.kindIds.length || spec.kindIds.some((kindId) => !taxonomy.semanticDirectoryKinds[kindId] && !taxonomy.semanticDirectoryMemberKinds[kindId])) problems.push(`${scope}.directoryCaptures.${capture} must resolve registered semantic directory kinds.`);
          if (spec.names && (!Array.isArray(spec.names) || spec.names.length === 0 || new Set(spec.names).size !== spec.names.length || spec.names.some((name) => typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name)))) problems.push(`${scope}.directoryCaptures.${capture}.names must be exact NFC directory names.`);
        }
        if (record(contract.ownerPathPatterns, `${scope}.ownerPathPatterns`)) {
          if (Object.keys(contract.ownerPathPatterns).join("\0") !== "plugin-commands\0artifact-surface\0artifact-mode\0artifact-window\0engine-mode\0engine-window\0extension-window") problems.push(`${scope}.ownerPathPatterns must contain the seven semantic owner contexts.`);
          const patterns = new Set<string>();
          for (const [form, pattern] of Object.entries(contract.ownerPathPatterns)) {
            if (typeof pattern !== "string" || pattern !== pattern.normalize("NFC") || !pattern.startsWith("{plugin}/") || patterns.has(pattern)) { problems.push(`${scope}.ownerPathPatterns.${form} is not a unique rooted owner grammar.`); continue; }
            patterns.add(pattern);
            for (const segment of pattern.split("/")) {
              const capture = /^\{([a-zA-Z]+)\}$/u.exec(segment)?.[1];
              if (capture ? !contract.directoryCaptures?.[capture] : !segment || /[\\*?{}\0]/u.test(segment) || !semanticDirectoryKindId(segment, taxonomy)) problems.push(`${scope}.ownerPathPatterns.${form} has an unregistered directory segment.`);
            }
          }
        }
        if (record(contract.authoringCommand, `${scope}.authoringCommand`)) {
          exactKeys(contract.authoringCommand, ["scriptPath", "command", "writeDisposition"], `${scope}.authoringCommand`);
          if (contract.authoringCommand.scriptPath !== "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts" || !Array.isArray(contract.authoringCommand.command) || contract.authoringCommand.command.join("\0") !== "new\0surface" || contract.authoringCommand.writeDisposition !== "create-if-absent") problems.push(`${scope}.authoringCommand must be the create-if-absent surface author.`);
        }
        if (record(contract.referenceConsumer, `${scope}.referenceConsumer`)) {
          exactKeys(contract.referenceConsumer, ["path", "ownerRoot", "adapter", "region", "lineTemplate"], `${scope}.referenceConsumer`);
          if (contract.referenceConsumer.path !== "🌎️hub/🧩️compositions/🔋️energy/📦️packages/🦀️rust/🦀️.rs" || contract.referenceConsumer.ownerRoot !== "✏️s/🔌️plugins/🔋️energy" || contract.referenceConsumer.adapter !== "rust" || contract.referenceConsumer.region !== "✏️👁️Surfaces" || contract.referenceConsumer.lineTemplate !== "// `{filename}` (`🎚️config`/`🎮️commands`/`👥️presence`/`🫧️transient` at every surface/mode level) need") problems.push(`${scope}.referenceConsumer must be the exact energy surface-region prose form.`);
        }
        continue;
      }
      if (contract.contractKind === "owner-primary-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "sourceFileKindId", "sourceFilename", "destinationFilename", "rationaleRule"], scope);
        if (id !== "ticket-document-primary-markdown-v1" || contract.ownerFixedDirectoryContractId !== "ticket-slug" || contract.sourceFileKindId !== "markdown" || contract.sourceFilename !== "ticket.md" || contract.rationaleRule !== id) problems.push(`${scope} must use the exact ticket document primary-leaf grammar.`);
        if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
        if (!taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId || contract.destinationFilename !== canonicalFilenameForKind(contract.sourceFileKindId, taxonomy)) problems.push(`${scope}.destinationFilename must be the registered primary file-kind leaf.`);
        const tuple = [contract.ownerFixedDirectoryContractId, contract.sourceFilename, contract.contractKind].join("\0");
        if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
        ownerTuples.add(tuple);
        continue;
      }
      if (contract.contractKind === "owner-sibling-manifest-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], scope);
        if (contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.emptyContentRule !== "zero-byte" || contract.rationaleRule !== "ticket-important-markdown-projection-v1") problems.push(`${scope} must use the exact active ticket important owner-file grammar.`);
        if (!Array.isArray(contract.allowedStatuses) || contract.allowedStatuses.join("\0") !== "closed\0open") problems.push(`${scope}.allowedStatuses must be exactly closed, open.`);
        if (contract.statusDispositions && typeof contract.statusDispositions === "object" && !Array.isArray(contract.statusDispositions)) exactKeys(contract.statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${scope}.statusDispositions`);
        else problems.push(`${scope}.statusDispositions must be an object.`);
        if (JSON.stringify(contract.statusDispositions) !== JSON.stringify({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" })) problems.push(`${scope}.statusDispositions must be the exact lifecycle mapping.`);
        if (!taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId]) problems.push(`${scope}.requiredSiblingFixedFilenameContractId is missing.`);
      } else if (contract.contractKind === "owner-optional-sibling-manifest-file") {
        exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "optionalSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "admittedDispositions", "rationaleRule"], scope);
        if (contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.rationaleRule !== "ticket-important-history-markdown-v1") problems.push(`${scope} must use the exact ticket important history grammar.`);
        if (!Array.isArray(contract.admittedDispositions) || contract.admittedDispositions.join("\0") !== "closed-nonzero\0invalid-manifest\0missing-manifest") problems.push(`${scope}.admittedDispositions must be the exact history mapping.`);
        if (!taxonomy.fixedFilenameContracts[contract.optionalSiblingFixedFilenameContractId]) problems.push(`${scope}.optionalSiblingFixedFilenameContractId is missing.`);
      } else problems.push(`${scope}.contractKind is invalid.`);
      if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId]) problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
      if (typeof contract.sourceFilename !== "string" || !taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.sourceFilename must resolve to sourceFileKindId.`);
      const destinationKind = taxonomy.semanticDirectoryKinds[contract.destinationDirectoryKindId];
      if (!destinationKind || destinationKind.projectionOnly !== true || typeof contract.destinationDirectoryName !== "string" || semanticDirectoryKindId(contract.destinationDirectoryName, taxonomy, { parentKindId: "ticket-slug" }) !== contract.destinationDirectoryKindId) problems.push(`${scope}.destination directory must resolve to one ticket-scoped projectionOnly kind.`);
      if (contract.destinationFilename !== "📝️.md" || typeof contract.destinationFilename !== "string" || fileKindIdForSourcePath(contract.destinationFilename, taxonomy) !== contract.sourceFileKindId) problems.push(`${scope}.destinationFilename must be the exact Markdown physical leaf.`);
      for (const [field, value] of [["sourceFilename", contract.sourceFilename], ["destinationDirectoryName", contract.destinationDirectoryName], ["destinationFilename", contract.destinationFilename]] as const) if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /[\\/]/u.test(value)) problems.push(`${scope}.${field} must be one non-empty NFC name.`);
      const siblingContractId = contract.contractKind === "owner-sibling-manifest-file" ? contract.requiredSiblingFixedFilenameContractId : contract.optionalSiblingFixedFilenameContractId;
      const tuple = [contract.ownerFixedDirectoryContractId, siblingContractId, contract.sourceFilename, contract.contractKind].join("\0");
      if (ownerTuples.has(tuple)) problems.push(`${scope} overlaps another owned-file projection contract.`);
      ownerTuples.add(tuple);
    }
    if (Object.keys(taxonomy.semanticOwnedFileProjectionContracts).join("\0") !== "artifact-empty-facet-primary-markdown-v1\0readme-license-owner-leaves-v1\0ticket-document-primary-markdown-v1\0ticket-important-history-markdown-v1\0ticket-important-markdown-v1") problems.push("semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active ticket-important, and history ticket-important contracts.");
  }

  if (record(taxonomy.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts")) {
    const identities = new Set<string>();
    const patterns = new Set<string>();
    const adapters = ["rust", "typescript", "json", "toml"];
    const forms: readonly SemanticPathProjectionReferenceConsumerForm[] = ["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:category-glob", "artifact-catalog-prose:catalog-grammar"];
    for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts)) {
      const scope = `semanticPathProjectionReferenceConsumerContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      exactKeys(contract, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], scope);
      const projection = taxonomy.semanticPathProjectionContracts[contract.projectionContractId];
      if (!projection || !["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(projection.rationaleRule)) problems.push(`${scope}.projectionContractId must reference one CAD or Draw artifact projection.`);
      if (contract.consumerIdentity !== id || identities.has(contract.consumerIdentity)) problems.push(`${scope}.consumerIdentity must equal its unique registry id.`);
      identities.add(contract.consumerIdentity);
      if (contract.ownership !== "external") problems.push(`${scope}.ownership must be external.`);
      if (!contract.sourcePathPattern.startsWith("^") || !contract.sourcePathPattern.endsWith("$") || contract.sourcePathPattern !== contract.sourcePathPattern.normalize("NFC") || /\uFE0E/u.test(contract.sourcePathPattern)) problems.push(`${scope}.sourcePathPattern must be one anchored NFC regex without VS15.`);
      try {
        const expression = new RegExp(contract.sourcePathPattern, "u");
        if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0 || contract.sourcePathIdentities.some((path) => !path || path !== path.normalize("NFC") || /\uFE0E/u.test(path) || !expression.test(path)) || new Set(contract.sourcePathIdentities).size !== contract.sourcePathIdentities.length) problems.push(`${scope}.sourcePathIdentities must be unique exact NFC paths admitted by sourcePathPattern.`);
      } catch {
        problems.push(`${scope}.sourcePathPattern must compile with Unicode semantics.`);
      }
      const patternKey = `${contract.projectionContractId}\0${contract.sourcePathPattern}`;
      if (patterns.has(patternKey)) problems.push(`${scope}.sourcePathPattern is duplicated for its projection.`);
      patterns.add(patternKey);
      if (!Array.isArray(contract.adapters) || contract.adapters.length === 0 || contract.adapters.some((adapter) => !adapters.includes(adapter)) || new Set(contract.adapters).size !== contract.adapters.length) problems.push(`${scope}.adapters must be unique supported reference adapters.`);
      if (!Array.isArray(contract.supportedForms) || contract.supportedForms.length === 0 || contract.supportedForms.some((form) => !forms.includes(form)) || new Set(contract.supportedForms).size !== contract.supportedForms.length) problems.push(`${scope}.supportedForms must be unique supported structural forms.`);
      if (!Array.isArray(contract.staleMarkers) || contract.staleMarkers.length === 0 || contract.staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC") || /\uFE0E/u.test(marker)) || new Set(contract.staleMarkers).size !== contract.staleMarkers.length) problems.push(`${scope}.staleMarkers must be unique non-empty NFC markers without VS15.`);
      if (contract.supportedForms.some((form) => form.startsWith("artifact-catalog-")) && (projection?.rationaleRule !== "artifact-example-model-catalog-projection-v1" || !contract.adapters.some((adapter) => adapter === "rust" || adapter === "typescript"))) problems.push(`${scope} artifact-catalog forms require the CAD projection and a Rust or TypeScript adapter.`);
    }
    const required = ["cad-editor-interaction", "cad-editor-runtime", "cad-interaction-spec", "cad-spatial-kernel-geometry", "draw-dependency-registry", "draw-package-cargo", "draw-package-library", "draw-workspace-cargo", "draw-workspace-script"];
    if ([...identities].sort().join("\0") !== required.join("\0")) problems.push("semanticPathProjectionReferenceConsumerContracts must encode the nine exact current external CAD/Draw consumers.");
    const rows = Object.values(taxonomy.semanticPathProjectionReferenceConsumerContracts);
    for (const [index, left] of rows.entries()) for (const right of rows.slice(index + 1)) {
      if (left.projectionContractId !== right.projectionContractId || !left.supportedForms.some((form) => right.supportedForms.includes(form))) continue;
      const leftPattern = new RegExp(left.sourcePathPattern, "u");
      const rightPattern = new RegExp(right.sourcePathPattern, "u");
      if (left.sourcePathIdentities.some((path) => rightPattern.test(path)) || right.sourcePathIdentities.some((path) => leftPattern.test(path))) problems.push(`semanticPathProjectionReferenceConsumerContracts ${JSON.stringify(left.consumerIdentity)} and ${JSON.stringify(right.consumerIdentity)} overlap for one supported form.`);
    }
  }

  const projectionIds = taxonomy.mutationCatalogProjection;
  if (!projectionIds || typeof projectionIds !== "object") problems.push("mutationCatalogProjection must be an object.");
  else {
    exactKeys(projectionIds, ["contractKind", "contractId", "sourceOwnerKindId", "projectedMemberKindId", "implementationSegments", "fixtureSegments", "implementationDescendantContractId", "fixtureDescendantContractId", "catalogContractId", "coverage"], "mutationCatalogProjection");
    const projected = taxonomy.semanticProjectedMemberKinds[projectionIds.projectedMemberKindId];
    if (projectionIds.contractKind !== "canonical-mutation-case-pair" || projectionIds.contractId !== "canonical-mutation-case-pair-v1" || projectionIds.coverage !== "every-catalog-vector-has-one-implementation-and-one-fixture-bundle") problems.push("mutationCatalogProjection must declare the canonical pair identity and closed coverage.");
    if (!projected || projected.projectionContractId !== projectionIds.contractId) problems.push("mutationCatalogProjection projected member and pair IDs do not agree.");
    if (!taxonomy.semanticDirectoryMemberKinds[projectionIds.sourceOwnerKindId] || !taxonomy.semanticDescendantContracts[projectionIds.implementationDescendantContractId] || !taxonomy.semanticDescendantContracts[projectionIds.fixtureDescendantContractId] || !taxonomy.semanticPathProjectionCatalogContracts[projectionIds.catalogContractId]) problems.push("mutationCatalogProjection contract IDs do not agree with its pair registries.");
  }
  const exactImplementation = [{ kindId: "standards", literal: "🏅️standards" }, { kindId: "standard", capture: "standardVersion" }, { kindId: "subsets", literal: "🪆️subsets" }, { kindId: "subset", capture: "subsetId" }, { kindId: "schema", literal: "🧬️schema" }, { kindId: "schema", literal: "🧬️mutations" }, { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" }, { kindId: "tests", literal: "🧪️tests" }, { kindId: "test-case", capture: "scenarioId" }];
  const exactFixture = [{ kindId: "standards", literal: "🏅️standards" }, { kindId: "standard", capture: "standardVersion" }, { kindId: "subsets", literal: "🪆️subsets" }, { kindId: "subset", capture: "subsetId" }, { kindId: "tests", literal: "🧪️tests" }, { kindId: "fixtures", literal: "🧫️fixtures" }, { kindId: "schema", literal: "🧬️mutations" }, { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" }, { kindId: "test-case", capture: "scenarioId" }];
  if (JSON.stringify(projectionIds?.implementationSegments) !== JSON.stringify(exactImplementation) || JSON.stringify(projectionIds?.fixtureSegments) !== JSON.stringify(exactFixture)) problems.push("mutationCatalogProjection must encode the exact canonical implementation and fixture root grammars.");
  const implementation = taxonomy.semanticDescendantContracts["mutation-implementation-case-v1"], fixture = taxonomy.semanticDescendantContracts["mutation-fixture-bundle-v1"];
  if (!implementation || "contractKind" in implementation || implementation.realizedNodeCount !== 2 || implementation.exclusiveAlternatives.length !== 0 || implementation.pathBudgetReserve.bytes !== 11) problems.push("semanticDescendantContracts.mutation-implementation-case-v1 must encode two nodes and the derived 11-byte reserve.");
  if (!fixture || "contractKind" in fixture || fixture.realizedNodeCount !== 12 || fixture.exclusiveAlternatives.length !== 1 || fixture.exclusiveAlternatives[0]?.id !== "diff-leaf" || fixture.pathBudgetReserve.bytes !== 42) problems.push("semanticDescendantContracts.mutation-fixture-bundle-v1 must encode 12 nodes, one diff alternative, and the derived 42-byte reserve.");
  //#endregion 🪞️SemanticPathProjection

  if (taxonomy.fixedDirectoryContractSets !== undefined && !(taxonomy.fixedDirectoryContractSets && typeof taxonomy.fixedDirectoryContractSets === "object" && !Array.isArray(taxonomy.fixedDirectoryContractSets))) problems.push("fixedDirectoryContractSets must be an object.");
  else for (const [id, values] of Object.entries(taxonomy.fixedDirectoryContractSets ?? {})) {
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`fixedDirectoryContractSets[${JSON.stringify(id)}] must use a literal set ID.`);
    try { parseFixedDirectoryContractSetScope({ kind: "fixed-directory-contract-set", fixedDirectoryContractIds: values }, taxonomy.fixedDirectoryContracts); } catch (error) { problems.push(`fixedDirectoryContractSets[${JSON.stringify(id)}]: ${error instanceof Error ? error.message : String(error)}`); }
  }
  const fixedScope = (scope: FixedContractScope, contractPathPattern: string, key: string, filenameContract: boolean): void => {
    if (!(typeof scope === "object" && scope !== null && !Array.isArray(scope)) || typeof scope.kind !== "string") {
      problems.push(`${key} must be a tagged fixed-contract scope.`);
      return;
    }
    if (scope.kind === "exact-path") {
      exactKeys(scope, ["kind", "path"], key);
      pathPattern(scope.path, `${key}.path`);
      if (scope.path !== contractPathPattern || /[*?\[\]{}]/u.test(scope.path)) problems.push(`${key}.path must equal the exact wildcard-free contract path.`);
    } else if (scope.kind === "repository-root") {
      exactKeys(scope, ["kind"], key);
      if (contractPathPattern.includes("/") || /[*?\[\]{}]/u.test(contractPathPattern)) problems.push(`${key} repository-root contract must be one exact basename.`);
    } else if (scope.kind === "package-root") {
      exactKeys(scope, ["kind", "ecosystemId"], key);
      if (!filenameContract) problems.push(`${key} cannot use package-root scope.`);
      if (!scope.ecosystemId || !taxonomy.ecosystems[scope.ecosystemId]) problems.push(`${key}.ecosystemId must reference an ecosystem.`);
    } else if (scope.kind === "directory-kind") {
      exactKeys(scope, ["kind", "directoryKindId"], key);
      if (!scope.directoryKindId || !taxonomy.semanticDirectoryKinds[scope.directoryKindId]) problems.push(`${key}.directoryKindId must reference a semantic directory kind.`);
    } else if (scope.kind === "fixed-directory-contract") {
      exactKeys(scope, ["kind", "fixedDirectoryContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use fixed-directory-contract scope.`);
      if (!scope.fixedDirectoryContractId || !taxonomy.fixedDirectoryContracts[scope.fixedDirectoryContractId]) problems.push(`${key}.fixedDirectoryContractId must reference a fixed directory contract.`);
    } else if (scope.kind === "fixed-directory-contract-set") {
      if (!filenameContract) problems.push(`${key} cannot use fixed-directory-contract-set scope.`);
      try { parseFixedDirectoryContractSetScope(scope, taxonomy.fixedDirectoryContracts); } catch (error) { problems.push(`${key}: ${error instanceof Error ? error.message : String(error)}`); }
    } else if (scope.kind === "named-fixed-directory-contract-set") {
      if (!filenameContract) problems.push(`${key} cannot use named-fixed-directory-contract-set scope.`);
      try { parseNamedFixedDirectoryContractSetScope(scope, taxonomy.fixedDirectoryContracts, taxonomy.fixedDirectoryContractSets ?? {}); } catch (error) { problems.push(`${key}: ${error instanceof Error ? error.message : String(error)}`); }
    } else if (scope.kind === "sibling-fixed-filename-contract") {
      exactKeys(scope, ["kind", "fixedFilenameContractId"], key);
      if (!filenameContract) problems.push(`${key} cannot use sibling-fixed-filename-contract scope.`);
      if (!scope.fixedFilenameContractId || !taxonomy.fixedFilenameContracts[scope.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId must reference a fixed filename contract.`);
    } else if (scope.kind === "path-pattern") exactKeys(scope, ["kind"], key);
    else problems.push(`${key}.kind is invalid.`);
  };

  if (record(taxonomy.fixedFilenameContracts, "fixedFilenameContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
    pathPattern(contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].pathPattern`);
    if (typeof contract.pathPattern === "string" && /[*?\[\]{}]/u.test(fixedContractFilename(contract))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].pathPattern must end in one exact literal basename.`);
    if (!contract.authority || !contract.reason || !contract.verification) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (contract.configurability !== "unconfigurable") problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
    fixedScope(contract.scope, contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].scope`, true);
    if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires))) problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }
  const cargoTargetEvidence = taxonomy.semanticDirectoryKinds["ticket-cargo-target-evidence"];
  if (!cargoTargetEvidence || cargoTargetEvidence.emoji !== "🧪️" || cargoTargetEvidence.slugPattern !== "^target-[a-z0-9]+(?:-[a-z0-9]+)*$" || cargoTargetEvidence.allowEmojiOnly || cargoTargetEvidence.parentKindIds !== undefined) problems.push("semanticDirectoryKinds.ticket-cargo-target-evidence must remain the exact ticket-local Cargo target authority.");
  const cargoCacheTag = taxonomy.fixedFilenameContracts["cargo-cache-tag"];
  if (!cargoCacheTag || cargoCacheTag.pathPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/CACHEDIR.TAG" || cargoCacheTag.authority !== "Cargo" || cargoCacheTag.scope.kind !== "directory-kind" || cargoCacheTag.scope.directoryKindId !== "ticket-cargo-target-evidence") problems.push("fixedFilenameContracts.cargo-cache-tag must remain conjunctively scoped to a governed ticket path and the ticket-cargo-target-evidence directory kind.");
  const ticketCargoPattern = "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**/";
  for (const triple of ["wasm32-unknown-unknown", "wasm32-wasip2"] as const) {
    const directoryId = `cargo-target-triple-${triple}`;
    const directory = taxonomy.fixedDirectoryContracts[directoryId];
    if (!directory || directory.pathPattern !== `${ticketCargoPattern}${triple}` || directory.authority !== "Cargo" || directory.scope.kind !== "directory-kind" || directory.scope.directoryKindId !== "ticket-cargo-target-evidence") problems.push(`fixedDirectoryContracts.${directoryId} must remain the exact governed ticket Cargo target-triple authority.`);
    const cacheId = `cargo-cache-tag-${triple}`;
    const cache = taxonomy.fixedFilenameContracts[cacheId];
    if (!cache || cache.pathPattern !== `${ticketCargoPattern}${triple}/CACHEDIR.TAG` || cache.authority !== "Cargo" || cache.scope.kind !== "fixed-directory-contract" || cache.scope.fixedDirectoryContractId !== directoryId) problems.push(`fixedFilenameContracts.${cacheId} must remain conjunctively scoped through its exact Cargo target-triple contract.`);
  }
  const nxManifestScopes = [
    ["build-owned-node-package-manifest", "**/package.json", "Build Tools and Node.js"],
    ["build-owned-typescript-config", "**/tsconfig.json", "Build Tools and TypeScript"],
  ] as const;
  for (const [id, pattern, authority] of nxManifestScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern || contract.authority !== authority || contract.scope.kind !== "sibling-fixed-filename-contract" || contract.scope.fixedFilenameContractId !== "build-project-manifest") problems.push(`fixedFilenameContracts.${id} must remain conjunctively scoped through an adjacent exact build project manifest.`);
  }
  const bunPublicationScopes = [
    ["bun-package-readme", "**/README.md"],
    ["bun-package-license", "**/LICENSE.md"],
  ] as const;
  for (const [id, pattern] of bunPublicationScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern || contract.authority !== "Bun package publisher" || contract.scope.kind !== "package-root" || contract.scope.ecosystemId !== "🟦️typescript" || contract.verification !== "bun pm pack --dry-run --ignore-scripts") problems.push(`fixedFilenameContracts.${id} must remain scoped to an exact publishable Bun package root.`);
  }
  const ticketCargoManifest = taxonomy.fixedFilenameContracts["ticket-cargo-manifest"];
  if (!ticketCargoManifest || ticketCargoManifest.pathPattern !== `${ticketCargoPattern}Cargo.toml` || ticketCargoManifest.authority !== "Cargo" || ticketCargoManifest.scope.kind !== "path-pattern") problems.push("fixedFilenameContracts.ticket-cargo-manifest must remain scoped to governed canonical or embedded ticket paths.");
  const ticketCargoLock = taxonomy.fixedFilenameContracts["ticket-cargo-lock"];
  if (!ticketCargoLock || ticketCargoLock.pathPattern !== `${ticketCargoPattern}Cargo.lock` || ticketCargoLock.authority !== "Cargo" || ticketCargoLock.scope.kind !== "path-pattern") problems.push("fixedFilenameContracts.ticket-cargo-lock must remain scoped to governed canonical or embedded ticket paths.");
  const rootCargoLock = taxonomy.fixedFilenameContracts["root-cargo-lock"];
  if (!rootCargoLock || rootCargoLock.pathPattern !== "Cargo.lock" || rootCargoLock.authority !== "Cargo" || rootCargoLock.scope.kind !== "repository-root") problems.push("fixedFilenameContracts.root-cargo-lock must remain the exact repository-root Cargo lock authority.");

  if (record(taxonomy.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts")) {
    const identities = new Map<string, string>();
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameRejectionContracts)) {
      exactKeys(contract, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts[${JSON.stringify(id)}]`);
      if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities must be non-empty.`);
      for (const identity of contract.sourcePathIdentities ?? []) {
        pathPattern(identity, `fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities`);
        if (/[*?\[]/u.test(identity)) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}] identities must be exact paths.`);
        if (identities.has(identity)) problems.push(`Fixed filename rejection identity ${JSON.stringify(identity)} is duplicated by ${JSON.stringify(identities.get(identity))} and ${JSON.stringify(id)}.`);
        identities.set(identity, id);
      }
      if (!["normalize", "relocate"].includes(contract.disposition)) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].disposition is invalid.`);
      if (!contract.reason) problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].reason must be non-empty.`);
    }
  }

  if (record(taxonomy.fixedDirectoryContracts, "fixedDirectoryContracts")) for (const [id, contract] of Object.entries(taxonomy.fixedDirectoryContracts)) {
    pathPattern(contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].pathPattern`);
    if (!contract.authority || !contract.reason || !contract.verification) problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (contract.configurability !== "unconfigurable") problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
    if (contract.descendants !== undefined && contract.descendants !== "reserved") problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].descendants must be reserved when present.`);
    fixedScope(contract.scope, contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].scope`, false);
    if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires))) problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }

  if (record(taxonomy.configurableEntryContracts, "configurableEntryContracts")) for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) {
    if (!taxonomy.fileKinds[contract.fileKindId]) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].fileKindId is missing.`);
    else if (!canonicalFilenamesForKind(contract.fileKindId, taxonomy).includes(contract.filename)) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].filename is not canonical for its file kind.`);
    if (!taxonomy.ecosystems[contract.ecosystemId]) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].ecosystemId is missing.`);
    if (!Array.isArray(contract.configurationSources) || contract.configurationSources.length === 0) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].configurationSources must be non-empty.`);
  }

  if (record(taxonomy.packageGlueGrammar, "packageGlueGrammar")) for (const [id, grammar] of Object.entries(taxonomy.packageGlueGrammar)) {
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp", "tex"].includes(grammar.analyzer)) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].analyzer is invalid.`);
    if (!Number.isSafeInteger(grammar.maxDelegationStatements) || grammar.maxDelegationStatements < 0) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].maxDelegationStatements is invalid.`);
    if (!Array.isArray(grammar.allowedRoles) || grammar.allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role))) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].allowedRoles is invalid.`);
  }

  if (record(taxonomy.packageBoundaryRules, "packageBoundaryRules")) for (const [ecosystemId, rule] of Object.entries(taxonomy.packageBoundaryRules)) {
    if (!taxonomy.ecosystems[ecosystemId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] has no ecosystem.`);
    if (rule.manifestContractId && !taxonomy.fixedFilenameContracts[rule.manifestContractId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].manifestContractId is missing.`);
    ids(rule.entryContractIds, taxonomy.configurableEntryContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].entryContractIds`);
    ids(rule.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFixedContractIds`);
    ids(rule.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFileKindIds`);
    ids(rule.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedDirectoryKindIds`);
    if (!taxonomy.packageGlueGrammar[rule.glueGrammarId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].glueGrammarId is missing.`);
    if (rule.recursive !== true || rule.uncertainRole !== "problem" || rule.implementationRole !== "problem") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] must recursively block uncertain and implementation roles.`);
  }

  if (record(taxonomy.packageBoundaryProfiles, "packageBoundaryProfiles")) for (const [id, profile] of Object.entries(taxonomy.packageBoundaryProfiles)) {
    exactKeys(profile, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles[${JSON.stringify(id)}]`);
    if (profile.admission !== "blocked-until-language-directory-registered") problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].admission is invalid.`);
    ids(profile.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFileKindIds`);
    ids(profile.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedDirectoryKindIds`);
    ids(profile.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFixedContractIds`);
    if (!taxonomy.packageGlueGrammar[profile.glueGrammarId]) problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].glueGrammarId is missing.`);
    if (profile.recursive !== true || profile.uncertainRole !== "problem" || profile.implementationRole !== "problem" || !profile.reason) problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}] must fail closed with a reason.`);
  }

  if (record(taxonomy.packageSourceDispositions, "packageSourceDispositions")) {
    const expected = new Map<string, "fixed" | "configurable">();
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
      const kindId = fileKindIdForSourcePath(fixedContractFilename(contract), taxonomy);
      if (kindId && taxonomy.fileKinds[kindId]?.role === "source") expected.set(id, "fixed");
    }
    for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) if (taxonomy.fileKinds[contract.fileKindId]?.role === "source") expected.set(id, "configurable");
    for (const [id, disposition] of Object.entries(taxonomy.packageSourceDispositions)) if (disposition.contractKind === "fixed" && disposition.grammarId && taxonomy.fixedFilenameContracts[id]) expected.set(id, "fixed");
    for (const missing of [...expected.keys()].filter((id) => !taxonomy.packageSourceDispositions[id])) problems.push(`packageSourceDispositions is missing source-format contract ${JSON.stringify(missing)}.`);
    for (const [id, disposition] of Object.entries(taxonomy.packageSourceDispositions)) {
      exactKeys(disposition, ["contractKind", "disposition", "validator", ...(disposition.grammarId === undefined ? [] : ["grammarId"]), "authority", "verification"], `packageSourceDispositions[${JSON.stringify(id)}]`);
      if (!expected.has(id)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] does not name a source-format fixed/configurable contract.`);
      else if (expected.get(id) !== disposition.contractKind) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].contractKind does not match its registry.`);
      if (!["adapter-source", "tool-metadata"].includes(disposition.disposition)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].disposition is invalid.`);
      const TOOL_CONFIG_VALIDATORS: Readonly<Record<string, string>> = { "vitest-configuration": "vitest-config-entry", "tool-config-tailwind": "tailwind-config", "tool-config-postcss": "postcss-config", "tool-config-eslint": "eslint-config", "tool-config-dependency-cruiser": "dependency-cruiser-config", "pytest-configuration": "root-pytest-config", "eslint-configuration": "root-eslint-config", "vscode-test-configuration": "vscode-test-cli-config" };
      const configValidatorOwner = TOOL_CONFIG_VALIDATORS[disposition.validator];
      if (!["package-glue", "command-router", ...Object.keys(TOOL_CONFIG_VALIDATORS)].includes(disposition.validator) || (disposition.disposition === "adapter-source") !== (disposition.validator === "package-glue") || (configValidatorOwner !== undefined && id !== configValidatorOwner)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] disposition/validator pair is invalid.`);
      if (disposition.grammarId !== undefined && !taxonomy.packageGlueGrammar[disposition.grammarId]) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].grammarId is missing.`);
      if (!disposition.authority || !disposition.verification) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] must declare authority and verification.`);
    }
  }

  if (record(taxonomy.generatorContracts, "generatorContracts")) {
    const contractIds = Object.keys(taxonomy.generatorContracts);
    if (contractIds.join("\0") !== [...contractIds].sort().join("\0")) problems.push("generatorContracts ids must be lexically ordered.");
    const opaqueRoots = Object.values(taxonomy.pathExclusions ?? {}).map((entry) => entry.path.replace(/\/$/u, ""));
    const exactTouchesOpaque = (value: string): boolean => opaqueRoots.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
    // 🚧️ A pattern crosses an opaque boundary when it is lexically under (or over) an opaque root, or
    // when a wildcard sits at or above that root's own depth and could therefore expand into it. An
    // exact literal path that merely SHARES a first segment with a deeper opaque root — `♻️mit-bestand/
    // 📋️bericht/…` beside the `♻️mit-bestand/🔎️recherche/` gitlink — reaches nothing opaque and is
    // admitted; anything glob-shaped above the root still is not.
    const patternTouchesOpaque = (value: string): boolean => {
      if (exactTouchesOpaque(value)) return true;
      const segments = value.split("/");
      return opaqueRoots.some((opaque) => segments.slice(0, opaque.split("/").length).some((segment) => /[*?\[]/u.test(segment)));
    };
    const nxTarget = (value: unknown, key: string): value is string => {
      const valid = typeof value === "string" && /^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*:[a-z0-9][a-z0-9._-]*$/u.test(value);
      if (!valid) problems.push(`${key} must be one exact build project:target identity.`);
      return valid;
    };
    const outputOwners: { id: string; path: string }[] = [];
    const targets = new Map<string, string>();
    for (const [id, contract] of Object.entries(taxonomy.generatorContracts)) {
      if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`generatorContracts id ${JSON.stringify(id)} must be kebab-case.`);
      const allowedKeys = new Set(["ownership", "ownerPath", "target", "previewTarget", "previewArguments", "previewProgress", "previewLimits", "checkTarget", "nativeConsumers", "inputPatterns", "inputDiscovery", "compilerInputManifest", "packageGeneration", "currentPackageDestination", "projectionActivation", "outputRoots", "reason"]);
      for (const key of Object.keys(contract)) if (!allowedKeys.has(key)) problems.push(`generatorContracts[${JSON.stringify(id)}].${key} is forbidden.`);
      const ownership = contract.ownership as string;
      if (!["owned", "external"].includes(ownership)) problems.push(`generatorContracts[${JSON.stringify(id)}].ownership must be owned or external.`);
      if (!contract.reason) problems.push(`generatorContracts[${JSON.stringify(id)}].reason must be non-empty.`);
      const runnable = ownership === "owned";
      if (contract.nativeConsumers !== undefined) {
        if (!runnable || !Array.isArray(contract.nativeConsumers) || contract.nativeConsumers.length === 0 || new Set(contract.nativeConsumers).size !== contract.nativeConsumers.length) problems.push(`generatorContracts[${JSON.stringify(id)}].nativeConsumers requires distinct Cargo project roots on an owned generator.`);
        else for (const consumer of contract.nativeConsumers) if (workspacePath(consumer, `generatorContracts[${JSON.stringify(id)}].nativeConsumers`) && exactTouchesOpaque(consumer)) problems.push(`generatorContracts[${JSON.stringify(id)}].nativeConsumers crosses an opaque boundary.`);
      }
      const targetKnown = runnable;
      if (targetKnown) {
        if (workspacePath(contract.ownerPath, `generatorContracts[${JSON.stringify(id)}].ownerPath`) && exactTouchesOpaque(contract.ownerPath)) problems.push(`generatorContracts[${JSON.stringify(id)}].ownerPath crosses an opaque boundary.`);
        if (nxTarget(contract.target, `generatorContracts[${JSON.stringify(id)}].target`)) {
          const prior = targets.get(contract.target);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.target)}.`);
          targets.set(contract.target, id);
        }
      } else if (contract.ownerPath !== null || contract.target !== null) problems.push(`generatorContracts[${JSON.stringify(id)}] unowned/external classifications must have null ownerPath and target.`);
      if (runnable) {
        if (contract.previewTarget === undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is required for owned contracts.`);
        else if (nxTarget(contract.previewTarget, `generatorContracts[${JSON.stringify(id)}].previewTarget`) && typeof contract.target === "string") {
          try { generatorPreviewScriptArguments(contract); }
          catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}]: ${error instanceof Error ? error.message : String(error)}`); }
          const prior = targets.get(contract.previewTarget);
          if (prior) problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.previewTarget)}.`);
          targets.set(contract.previewTarget, id);
        }
      } else if (contract.previewTarget !== undefined) problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is forbidden for external contracts.`);
      if (contract.previewProgress !== undefined) { try { if (!runnable || contract.previewTarget === undefined) throw new Error("Declared progress requires owned preview authority"); parseGeneratorPreviewProgressPolicyV1(contract.previewProgress); } catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}].previewProgress: ${(error as Error).message}`); } }
      if (contract.previewArguments !== undefined && (!runnable || contract.previewTarget === undefined)) problems.push(`generatorContracts[${JSON.stringify(id)}].previewArguments requires an owned preview target.`);
      try { generatorPreviewResourceLimits(contract); } catch (error) { problems.push(`generatorContracts[${JSON.stringify(id)}].previewLimits: ${(error as Error).message}`); }
      if (contract.compilerInputManifest !== undefined) {
        const authority = contract.compilerInputManifest;
        if (!runnable || !authority || typeof authority !== "object" || Array.isArray(authority) || JSON.stringify(Object.keys(authority).sort()) !== JSON.stringify(["kind", "manifestOutputPath", "manifestSchemaPath", "maxFiles", "staticAuthorityPath"])) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest requires one closed owned authority.`);
        else {
          const literal = (path: unknown): path is string => typeof path === "string" && path === path.normalize("NFC") && path.length > 0 && !path.startsWith("/") && !/[\\*?\[\]\u0000-\u001f]/u.test(path) && !path.split("/").some(part => !part || part === "." || part === "..");
          if (authority.kind !== "compiler-input-manifest-v1" || !literal(authority.manifestOutputPath) || !literal(authority.manifestSchemaPath) || !literal(authority.staticAuthorityPath) || !Number.isSafeInteger(authority.maxFiles) || authority.maxFiles < 1 || authority.maxFiles > 10000) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest has invalid bounded fields.`);
          if (!contract.outputRoots?.some(root => authority.manifestOutputPath === root.path || authority.manifestOutputPath.startsWith(`${root.path}/`))) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest manifest is outside its output roots.`);
          for (const path of [authority.manifestSchemaPath, authority.staticAuthorityPath]) if (!contract.inputPatterns?.includes(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].compilerInputManifest authority is not one exact input: ${JSON.stringify(path)}.`);
        }
      }
      if (contract.checkTarget !== undefined) {
        if (!targetKnown || !nxTarget(contract.checkTarget, `generatorContracts[${JSON.stringify(id)}].checkTarget`)) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget requires a known target.`);
        else if (typeof contract.target === "string" && contract.checkTarget.slice(0, contract.checkTarget.lastIndexOf(":")) !== contract.target.slice(0, contract.target.lastIndexOf(":"))) problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget must belong to the target project.`);
      }
      if (id === "jco-package-adapter" && contract.currentPackageDestination === undefined) problems.push(`generatorContracts.${id} requires explicit current package destination authority.`);
      if (["jco-package-adapter", "wgpu-frame-worker"].includes(id) && contract.projectionActivation !== undefined) problems.push(`generatorContracts.${id} must use its current catalog instead of historical projection activation.`);
      if (contract.currentPackageDestination !== undefined) {
        try {
          const current = parseCurrentJcoPackageDestination(contract.currentPackageDestination);
          if (!runnable || id !== "jco-package-adapter" || contract.outputRoots.length !== 1 || contract.outputRoots[0]?.path !== current.adapterPath || contract.outputRoots[0]?.inclusion !== "tracked" || [current.cargoManifestPath, current.cargoLockPath, current.componentPath, current.witPath].some((path) => !contract.inputPatterns.includes(path))) problems.push(`generatorContracts.${id} current JCO ownership or inputs disagree.`);
        } catch (error) { problems.push(`generatorContracts.${id}.currentPackageDestination: ${error instanceof Error ? error.message : String(error)}`); }
      }
      if (id === "wgpu-frame-worker" && contract.packageGeneration === undefined) problems.push(`generatorContracts.${id} requires explicit package generation authority.`);
      if (contract.projectionActivation !== undefined) {
        const activation = contract.projectionActivation;
        exactKeys(activation, ["kind", "projectionContractId", "packageId", "sourceManifestPath", "destinationManifestPath"], `generatorContracts.${id}.projectionActivation`);
        if (!runnable || !["jco-package-adapter", "wgpu-frame-worker"].includes(id) || activation.kind !== "canonical-or-planned-package" || activation.projectionContractId !== "nested-cargo-packages-v1" || activation.packageId !== (id === "jco-package-adapter" ? "jcoprobe-guest" : "wgpu-renderer") || !exactOwnerPath(activation.sourceManifestPath) || !exactOwnerPath(activation.destinationManifestPath) || activation.sourceManifestPath === activation.destinationManifestPath) problems.push(`generatorContracts.${id}.projectionActivation has no exact package authority.`);
      }
      if (contract.packageGeneration !== undefined) {
        const generation = contract.packageGeneration;
        exactKeys(generation, ["kind", "previewInput", "catalogPath", "catalogSha256", "browserProfile"], `generatorContracts.${id}.packageGeneration`);
        const protocol = generation.previewInput;
        if (!runnable || id !== "wgpu-frame-worker" || contract.inputDiscovery || generation.kind !== "wgpu-package-artifacts" || !protocol || Object.keys(protocol).sort().join("\0") !== "maxBytes\0maxOperations\0protocol" || protocol.protocol !== "package-projected-inputs-v1" || protocol.maxBytes !== 67108864 || protocol.maxOperations !== 200000) problems.push(`generatorContracts.${id}.packageGeneration requires its exact bounded package protocol.`);
        try {
          const profile = parseSemanticPackageBrowserProfile(generation.browserProfile, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
          if (generation.catalogPath !== profile.ownerPath + "/🪪️package-catalog.json" || !/^[0-9a-f]{64}$/u.test(generation.catalogSha256)) problems.push(`generatorContracts.${id}.packageGeneration requires its exact current catalog and digest.`);
          if (profile.entries.some((entry) => !contract.outputRoots.some((output) => output.path === `${profile.ownerPath}/${entry.outputRelativePath}` && output.inclusion === entry.inclusion))) problems.push(`generatorContracts.${id}.packageGeneration browser outputs disagree with ownership.`);
        } catch (error) { problems.push(`generatorContracts.${id}.packageGeneration: ${error instanceof Error ? error.message : String(error)}`); }
      }
      if (contract.inputDiscovery !== undefined) {
        const input = contract.inputDiscovery;
        exactKeys(input, ["kind", "previewInput", "descriptorRelativePath", "exampleDirectoryName", "exampleFileKindId", "implementationEntryPaths", "workspaceImports"], `generatorContracts[${JSON.stringify(id)}].inputDiscovery`);
        if (!runnable || id !== "plugin-registry" || input.kind !== "registry-catalog") problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery requires the owned registry-catalog authority.`);
        if (!input.previewInput || JSON.stringify(Object.keys(input.previewInput).sort()) !== JSON.stringify(["maxBytes", "maxOperations", "protocol"]) || input.previewInput.protocol !== "registry-projected-inputs-v1" || input.previewInput.maxBytes !== 67108864 || input.previewInput.maxOperations !== 200000) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery requires the bounded registry-projected-inputs-v1 preview protocol.`);
        if (input.descriptorRelativePath !== "../../🔣️.json" || input.exampleDirectoryName !== "📚️examples" || !taxonomy.artifactChildDirs.includes(input.exampleDirectoryName) || input.exampleFileKindId !== taxonomy.exampleFileKinds["🦀️rust"]) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery must preserve the exact descriptor and example vocabulary.`);
        if (!Array.isArray(input.implementationEntryPaths) || input.implementationEntryPaths.length === 0 || input.implementationEntryPaths.join("\0") !== [...new Set(input.implementationEntryPaths)].sort().join("\0")) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery implementation entries must be nonempty, unique and ordered.`);
        else for (const path of input.implementationEntryPaths) if (!workspacePath(path, `generatorContracts[${JSON.stringify(id)}].inputDiscovery entry`) || exactTouchesOpaque(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery entry crosses an opaque boundary.`);
        if (!input.workspaceImports || typeof input.workspaceImports !== "object" || Array.isArray(input.workspaceImports)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery.workspaceImports must be an object.`);
        else for (const [name, binding] of Object.entries(input.workspaceImports)) {
          exactKeys(binding, ["manifestPath", "entryPath"], `generatorContracts[${JSON.stringify(id)}].inputDiscovery.workspaceImports[${JSON.stringify(name)}]`);
          if (!/^@[a-z0-9-]+\/[a-z0-9-]+$/u.test(name)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace import must be an exact package name.`);
          for (const path of [binding.manifestPath, binding.entryPath]) if (!workspacePath(path, `generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace binding`) || exactTouchesOpaque(path)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputDiscovery workspace binding crosses an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.inputPatterns) || (runnable && contract.inputPatterns.length === 0) || (!runnable && contract.inputPatterns.length > 0)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be non-empty only for owned contracts.`);
      if (Array.isArray(contract.inputPatterns)) {
        if (contract.inputPatterns.join("\0") !== [...contract.inputPatterns].sort().join("\0") || new Set(contract.inputPatterns).size !== contract.inputPatterns.length) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be unique and lexically ordered.`);
        for (const [index, input] of contract.inputPatterns.entries()) {
          pathPattern(input, `generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}]`);
          if (typeof input === "string" && patternTouchesOpaque(input)) problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}] can cross an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.outputRoots) || contract.outputRoots.length === 0) problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be non-empty.`);
      if (Array.isArray(contract.outputRoots)) {
        const outputPaths = contract.outputRoots.map((output) => output.path);
        if (outputPaths.join("\0") !== [...outputPaths].sort().join("\0") || new Set(outputPaths).size !== outputPaths.length) problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be unique and lexically ordered.`);
        for (const [index, output] of contract.outputRoots.entries()) {
          const key = `generatorContracts[${JSON.stringify(id)}].outputRoots[${index}]`;
          if (workspacePath(output.path, `${key}.path`) && exactTouchesOpaque(output.path)) problems.push(`${key}.path crosses an opaque boundary.`);
          if (!["tracked", "ignored"].includes(output.inclusion)) problems.push(`${key}.inclusion must be tracked or ignored.`);
          if (output.inclusion === "tracked" && typeof output.path === "string" && output.path.split("/").some((part: string) => part.replace("\uFE0F", "") === "🤖generated")) problems.push(`${key} declares a tracked output inside 🤖️generated, which .gitignore ignores and the indexed-generated-output law never indexes: no clone has it.`);
          if (contract.ownership === "external" && output.inclusion !== "tracked") problems.push(`${key} is an external input that is not tracked: nothing produces it, so no clone has it.`);
          if (output.producer !== undefined) {
            exactKeys(output.producer, ["ownerPath", "target"], `${key}.producer`);
            if (!runnable) problems.push(`${key}.producer requires an owned generator.`);
            if (workspacePath(output.producer?.ownerPath, `${key}.producer.ownerPath`) && exactTouchesOpaque(output.producer.ownerPath)) problems.push(`${key}.producer crosses an opaque boundary.`);
            if (nxTarget(output.producer?.target, `${key}.producer.target`)) {
              const prior = targets.get(output.producer.target);
              if (prior && prior !== id) problems.push(`${key}.producer belongs to another generator contract.`);
              targets.set(output.producer.target, id);
              if (output.producer.target === contract.target && output.producer.ownerPath !== contract.ownerPath) problems.push(`${key}.producer disagrees with the generator owner.`);
            }
          }
          if (runnable && contract.inputPatterns.some((input) => pathMatcher.matches(output.path, input))) problems.push(`${key}.path is also declared as an input.`);
          outputOwners.push({ id, path: output.path });
        }
      }
    }
    for (let left = 0; left < outputOwners.length; left += 1) for (let right = left + 1; right < outputOwners.length; right += 1) {
      const a = outputOwners[left]!;
      const b = outputOwners[right]!;
      if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) problems.push(`generatorContracts ${JSON.stringify(a.id)} and ${JSON.stringify(b.id)} have overlapping output roots ${JSON.stringify(a.path)} and ${JSON.stringify(b.path)}.`);
    }
    const unsettled = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => !["owned", "external"].includes(contract.ownership as string)).map(([id]) => id);
    if (unsettled.length > 0) problems.push(`generatorContracts must contain zero unknown or unsafe contracts; found ${unsettled.join(", ")}.`);
    for (const removed of ["ownerless-ui-icons", "root-layering-declarations"]) if (taxonomy.generatorContracts[removed]) problems.push(`generatorContracts.${removed} is false ownership and must remain absent.`);
    const foreignMetadataRoots = Object.entries(taxonomy.fixedDirectoryContracts ?? {}).filter(([, contract]) => contract.scope.kind === "repository-root" && contract.pathPattern.startsWith(".") && !contract.pathPattern.includes("/")).map(([id, contract]) => [id, contract.pathPattern] as const);
    const ralphFileContracts: Readonly<Record<string, string>> = {
      "ralph-config": ".ralph-tui/config.toml",
      "ralph-lock": ".ralph-tui/ralph.lock",
      "ralph-prd-json": ".ralph-tui/prd/*/prd.json",
      "ralph-prd-markdown": ".ralph-tui/prd/*/prd.md",
      "ralph-progress": ".ralph-tui/progress.md",
      "ralph-session-meta": ".ralph-tui/session-meta.json",
      "ralph-session": ".ralph-tui/session.json",
    };
    for (const [id, expected] of Object.entries(ralphFileContracts)) {
      const contract = taxonomy.fixedFilenameContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI" || contract.scope.kind !== "path-pattern") problems.push(`fixedFilenameContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    const ralphDirectoryContracts: Readonly<Record<string, string>> = { "ralph-metadata": ".ralph-tui", "ralph-prd-root": ".ralph-tui/prd", "ralph-prd-identifier": ".ralph-tui/prd/*" };
    for (const [id, expected] of Object.entries(ralphDirectoryContracts)) {
      const contract = taxonomy.fixedDirectoryContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI") problems.push(`fixedDirectoryContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    for (const [id, contract] of [...Object.entries(taxonomy.fixedFilenameContracts), ...Object.entries(taxonomy.fixedDirectoryContracts)]) for (const [, root] of foreignMetadataRoots) if (contract.pathPattern.startsWith(`${root}/`) && contract.pathPattern.includes("**")) problems.push(`Foreign metadata contract ${JSON.stringify(id)} must not use a recursive wildcard.`);
    for (const output of outputOwners) for (const [contractId, root] of foreignMetadataRoots) if (output.path === root || output.path.startsWith(`${root}/`)) problems.push(`generatorContracts ${JSON.stringify(output.id)} declares output ${JSON.stringify(output.path)} inside the foreign metadata directory ${JSON.stringify(root)} owned by ${JSON.stringify(contractId)}; a foreign tool's own state is never a repository output.`);
    const fixedRootManifests: Readonly<Record<string, string>> = { "root-package": "package.json", "root-cargo": "Cargo.toml", "root-go-work": "go.work" };
    for (const [id, expected] of Object.entries(fixedRootManifests)) if (taxonomy.fixedFilenameContracts[id]?.pathPattern !== expected) problems.push(`fixedFilenameContracts.${id} must remain the authored root manifest contract ${JSON.stringify(expected)}.`);
    const generatedRootManifests = outputOwners.filter((output) => ["package.json", "Cargo.toml", "go.work"].includes(output.path));
    if (generatedRootManifests.length > 0) problems.push("Root Bun, Cargo, and Go manifests are authored fixed contracts, not generator outputs.");
  }

  if (record(taxonomy.pathExclusions, "pathExclusions")) {
    const entries = Object.entries(taxonomy.pathExclusions);
    if (entries.map(([id]) => id).join("\0") !== OPAQUE_PATH_EXCLUSIONS.map(([id]) => id).join("\0")) problems.push(`pathExclusions must contain exactly ordered ${OPAQUE_PATH_EXCLUSIONS.map(([id]) => JSON.stringify(id)).join(", ")} contracts.`);
    for (const [id, path] of OPAQUE_PATH_EXCLUSIONS) {
      const contract = taxonomy.pathExclusions[id];
      if (!contract || contract.path !== path || contract.mode !== "opaque" || !contract.reason) problems.push(`pathExclusions.${id} must be the exact opaque ${JSON.stringify(path)} contract.`);
    }
  }
  if (taxonomy.unicodeNormalization?.form !== "NFC" || taxonomy.unicodeNormalization?.caseFold !== "lower" || taxonomy.unicodeNormalization?.locale !== "und") problems.push("unicodeNormalization must be NFC/lower/und.");
  if (taxonomy.variationSelectorPolicy?.selector !== "\uFE0F" || taxonomy.variationSelectorPolicy?.requiredAfterEmoji !== true || taxonomy.variationSelectorPolicy?.comparison !== "ignore-selector") problems.push("variationSelectorPolicy is invalid.");
  if (taxonomy.pathEmojiPolicy?.inventory !== "git-visible" || taxonomy.pathEmojiPolicy?.identity !== "single-emoji-grapheme" || taxonomy.pathEmojiPolicy?.siblingNamespace !== "files-and-directories") problems.push("pathEmojiPolicy inventory, identity, and sibling namespace are invalid.");
  if (taxonomy.pathEmojiPolicy?.genericEmojiIdentities?.map(foldPathEmojiIdentity).join("\0") !== "📁\0📂\0📄") problems.push("pathEmojiPolicy.genericEmojiIdentities must reject the three generic file/folder glyphs.");
  if (!Array.isArray(taxonomy.pathEmojiPolicy?.reservedSubtreeDirectoryNames) || new Set(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.map(foldPathEmojiIdentity)).size !== taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.length) problems.push("pathEmojiPolicy.reservedSubtreeDirectoryNames must be a unique array.");
  const comparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (taxonomy.collisionPolicy?.comparisons?.join("\0") !== comparisons.join("\0")) problems.push(`collisionPolicy.comparisons must be exactly ${comparisons.join(", ")}.`);
  if (taxonomy.collisionPolicy?.maxPathBytes !== 240 || taxonomy.collisionPolicy?.rejectWindowsReservedNames !== true || taxonomy.collisionPolicy?.rejectTrailingDotsAndSpaces !== true) problems.push("collisionPolicy platform constraints must retain maxPathBytes 240 and reject reserved/trailing names.");
  if (taxonomy.areaEnforcement?.requiredState !== "clean" || taxonomy.areaEnforcement?.undeclaredAreas !== "enforce") problems.push("areaEnforcement must enforce clean declared and undeclared areas.");
  ids(taxonomy.areaEnforcement?.opaquePathExclusionIds, taxonomy.pathExclusions, "areaEnforcement.opaquePathExclusionIds");
  if (taxonomy.areaEnforcement?.opaquePathExclusionIds?.join("\0") !== OPAQUE_PATH_EXCLUSIONS.map(([id]) => id).join("\0")) problems.push(`areaEnforcement.opaquePathExclusionIds must be exactly [${OPAQUE_PATH_EXCLUSIONS.map(([id]) => JSON.stringify(id)).join(", ")}].`);
  const exemptAreaRoots = [...(taxonomy.pathEmojiPolicy?.reservedSubtreeDirectoryNames ?? []), ...Object.values(taxonomy.pathExclusions ?? {}).map((exclusion) => exclusion.path.replaceAll("\\", "/").replace(/\/+$/u, ""))];
  if (record(taxonomy.areas, "areas")) for (const [area, state] of Object.entries(taxonomy.areas)) {
    if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/")) problems.push("Opaque compose prefixes must exist only in pathExclusions.");
    if (state === "exempt") {
      if (!exemptAreaRoots.some((root) => root.length > 0 && (area === root || area.startsWith(`${root}/`)))) problems.push(`areas[${JSON.stringify(area)}] may only be "exempt" inside a reserved subtree directory or a declared path exclusion, which discovery already skips.`);
    } else if (state !== "clean") problems.push(`areas[${JSON.stringify(area)}] must be "clean" or "exempt".`);
  }
  if (record(taxonomy.dependencyDirections, "dependencyDirections") && record(taxonomy.dependencyDirections.roles, "dependencyDirections.roles") && record(taxonomy.dependencyDirections.rules, "dependencyDirections.rules")) {
    for (const [role, contribution] of Object.entries(taxonomy.dependencyDirections.roles)) {
      if (!role || !record(contribution, `dependencyDirections.roles.${role}`)) continue;
      if (!Array.isArray(contribution.ownerPaths) || !Array.isArray(contribution.externalPackages)) problems.push(`dependencyDirections.roles.${role} requires ownerPaths and externalPackages arrays.`);
      else {
        for (const path of contribution.ownerPaths) workspacePath(path, `dependencyDirections.roles.${role}.ownerPaths`);
        for (const name of contribution.externalPackages) if (typeof name !== "string" || !/^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*$/u.test(name)) problems.push(`dependencyDirections.roles.${role} has an invalid external package.`);
      }
    }
    for (const [name, rule] of Object.entries(taxonomy.dependencyDirections.rules)) {
      if (!name || !record(rule, `dependencyDirections.rules.${name}`)) continue;
      for (const [field, roles] of [["fromRoles", rule.fromRoles], ["toRoles", rule.toRoles]] as const) {
        if (!Array.isArray(roles) || !roles.length) problems.push(`dependencyDirections.rules.${name}.${field} requires nonempty roles.`);
        else for (const role of roles) if (!taxonomy.dependencyDirections.roles[role]) problems.push(`dependencyDirections.rules.${name}.${field} references unknown role ${JSON.stringify(role)}.`);
      }
    }
  }
  if (record(taxonomy.cargoDependencyDirections, "cargoDependencyDirections") && record(taxonomy.cargoDependencyDirections.rules, "cargoDependencyDirections.rules")) {
    const roles = taxonomy.cargoDependencyDirections.roles;
    const ownerRoles = taxonomy.cargoDependencyDirections.ownerRoles;
    if (!Array.isArray(roles) || !roles.length || roles.some((role) => typeof role !== "string" || !role) || new Set(roles).size !== roles.length) problems.push("cargoDependencyDirections requires distinct declared semantic roles.");
    else for (const [name, rule] of Object.entries(taxonomy.cargoDependencyDirections.rules)) {
      if (!name || !record(rule, `cargoDependencyDirections.rules.${name}`)) continue;
      for (const selected of [rule.fromRoles, rule.toRoles]) if (!Array.isArray(selected) || new Set(selected).size !== selected.length || selected.some((role) => !roles.includes(role))) problems.push(`cargoDependencyDirections.rules.${name} references an unknown semantic role.`);
      if (Object.keys(rule).some(key => !["fromRoles", "toRoles", "fromOwnerPaths", "toOwnerSegments"].includes(key))) problems.push("Cargo direction rules must use only declared selector fields.");
      if ((!rule.fromRoles?.length && !rule.fromOwnerPaths?.length) || (!rule.toRoles?.length && !rule.toOwnerSegments?.length)) problems.push("Cargo direction rules require nonempty source and target selectors.");
      if (rule.toOwnerSegments !== undefined && (!Array.isArray(rule.toOwnerSegments) || !rule.toOwnerSegments.length || new Set(rule.toOwnerSegments).size !== rule.toOwnerSegments.length || rule.toOwnerSegments.some((segment: unknown) => typeof segment !== "string" || !segment || /[/\\\\]/u.test(segment) || segment === "." || segment === ".."))) problems.push("Cargo direction rules require distinct exact target owner segments.");
      if (rule.fromOwnerPaths !== undefined) {
        if (!Array.isArray(rule.fromOwnerPaths) || !rule.fromOwnerPaths.length || new Set(rule.fromOwnerPaths).size !== rule.fromOwnerPaths.length) problems.push(`cargoDependencyDirections.rules.${name} requires distinct source owner patterns.`);
        else for (const path of rule.fromOwnerPaths) {
          if (typeof path !== "string" || !path) problems.push(`cargoDependencyDirections.rules.${name} requires readable source owner patterns.`);
          try { new RegExp(path, "u"); } catch { problems.push(`cargoDependencyDirections.rules.${name} has an invalid source owner pattern.`); }
        }
      }
    }
    if (!Array.isArray(ownerRoles) || !ownerRoles.length || new Set(ownerRoles.map((rule) => rule.path)).size !== ownerRoles.length) problems.push("cargoDependencyDirections requires distinct physical owner role classifications.");
    else for (const rule of ownerRoles) {
      if (typeof rule.path !== "string" || !rule.path || !Array.isArray(rule.roles) || !rule.roles.length || new Set(rule.roles).size !== rule.roles.length || rule.roles.some((role: string) => !roles?.includes(role))) problems.push("cargoDependencyDirections owner classifications require readable paths and known distinct roles.");
      try { new RegExp(rule.path, "u"); } catch { problems.push(`cargoDependencyDirections owner classification has an invalid path pattern: ${JSON.stringify(rule.path)}.`); }
    }
    if (JSON.stringify(taxonomy.roles) !== JSON.stringify(roles)) problems.push("Package discovery and Cargo direction must use the same semantic role vocabulary.");
  }
  if (record(taxonomy.areaLayers, "areaLayers")) for (const [area, layer] of Object.entries(taxonomy.areaLayers)) {
    if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/")) problems.push("Opaque compose prefixes must not appear in areaLayers.");
    if (!["framework", "implementation", "repo-wide"].includes(layer)) problems.push(`areaLayers[${JSON.stringify(area)}] is invalid.`);
  }

  for (const [lang, ecosystem] of Object.entries(taxonomy.ecosystems ?? {})) {
    const oldShape = ecosystem as unknown as Record<string, unknown>;
    for (const key of ["manifestFilename", "moduleRootFilename", "entryFilenames", "leafFilename", "sourceExtension", "packagingDirNames"]) if (key in oldShape) problems.push(`ecosystems[${JSON.stringify(lang)}].${key} was removed.`);
    if (!['manifest', 'boundary-only'].includes(ecosystem.packageIdentity)) problems.push(`ecosystems[${JSON.stringify(lang)}].packageIdentity is invalid.`);
    if (ecosystem.packageIdentity === "manifest" && (!ecosystem.manifestContractId || !ecosystem.marker)) problems.push(`ecosystems[${JSON.stringify(lang)}] manifest identity requires a manifest contract and marker.`);
    if (ecosystem.packageIdentity === "boundary-only" && (ecosystem.manifestContractId !== null || ecosystem.marker !== null)) problems.push(`ecosystems[${JSON.stringify(lang)}] boundary-only identity cannot declare a manifest or marker.`);
    if (ecosystem.manifestContractId && !taxonomy.fixedFilenameContracts[ecosystem.manifestContractId]) problems.push(`ecosystems[${JSON.stringify(lang)}].manifestContractId is missing.`);
    if (ecosystem.moduleRootContractId && !taxonomy.fixedFilenameContracts[ecosystem.moduleRootContractId]) problems.push(`ecosystems[${JSON.stringify(lang)}].moduleRootContractId is missing.`);
    if (!taxonomy.fileKinds[ecosystem.componentFileKindId]) problems.push(`ecosystems[${JSON.stringify(lang)}].componentFileKindId is missing.`);
    ids(ecosystem.sourceFileKindIds, taxonomy.fileKinds, `ecosystems[${JSON.stringify(lang)}].sourceFileKindIds`);
    ids(ecosystem.entryContractIds, taxonomy.configurableEntryContracts, `ecosystems[${JSON.stringify(lang)}].entryContractIds`);
    ids(ecosystem.packagingDirectoryKindIds ?? [], taxonomy.semanticDirectoryKinds, `ecosystems[${JSON.stringify(lang)}].packagingDirectoryKindIds`);
    if (!taxonomy.packageBoundaryRules[lang]) problems.push(`ecosystems[${JSON.stringify(lang)}] has no packageBoundaryRules entry.`);
  }
  for (const [target, spec] of Object.entries(taxonomy.targets ?? {})) {
    const oldShape = spec as unknown as Record<string, unknown>;
    for (const key of ["leafFilename", "entryFilenames"]) if (key in oldShape) problems.push(`targets[${JSON.stringify(target)}].${key} was removed.`);
    if (!taxonomy.ecosystems[spec.lang]) problems.push(`targets[${JSON.stringify(target)}].lang is missing.`);
    if (!taxonomy.fileKinds[spec.componentFileKindId]) problems.push(`targets[${JSON.stringify(target)}].componentFileKindId is missing.`);
    ids(spec.entryContractIds, taxonomy.configurableEntryContracts, `targets[${JSON.stringify(target)}].entryContractIds`);
  }

  const mappings: [string, Readonly<Record<string, string>>][] = [
    ["componentFileKinds", taxonomy.componentFileKinds], ["exampleFileKinds", taxonomy.exampleFileKinds],
    ["exampleTestFileKinds", taxonomy.exampleTestFileKinds], ["testAdapterFileKinds", taxonomy.testAdapterFileKinds],
    ["artifactSpecFileKinds", taxonomy.artifactSpecFileKinds], ["artifactSchemaSpecFileKinds", taxonomy.artifactSchemaSpecFileKinds],
    ["surfaceSchemaSpecFileKinds", taxonomy.surfaceSchemaSpecFileKinds],
  ];
  for (const [key, mapping] of mappings) for (const [owner, kindId] of Object.entries(mapping ?? {})) if (!taxonomy.fileKinds[kindId]) problems.push(`${key}[${JSON.stringify(owner)}] references missing kind ${JSON.stringify(kindId)}.`);
  ids(taxonomy.testImplementationFileKindIds, taxonomy.fileKinds, "testImplementationFileKindIds");
  if (!Array.isArray(taxonomy.testLegacyDirectoryNames) || taxonomy.testLegacyDirectoryNames.some((name) => typeof name !== "string" || !name || /[\\/]/u.test(name))) problems.push("testLegacyDirectoryNames must contain non-empty directory names.");
  if (!Array.isArray(taxonomy.testFixtureLegacyDirectoryNames) || taxonomy.testFixtureLegacyDirectoryNames.some((name) => typeof name !== "string" || !name || /[\\/]/u.test(name))) problems.push("testFixtureLegacyDirectoryNames must contain non-empty directory names.");
  if (taxonomy.testExamplesDirName !== "📚️examples") problems.push('testExamplesDirName must select the canonical "📚️examples" collection.');
  if (taxonomy.testOraclesDirName !== "🔮️oracles") problems.push('testOraclesDirName must select the canonical "🔮️oracles" collection.');
  for (const [key, values] of [["testObsoleteCategoryStems", taxonomy.testObsoleteCategoryStems], ["testObsoleteTestEmojiCategoryStems", taxonomy.testObsoleteTestEmojiCategoryStems]] as const) {
    if (!Array.isArray(values) || values.some((name) => typeof name !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(name) || name !== name.normalize("NFC")) || new Set(values).size !== values.length) problems.push(`${key} must contain unique normalized lowercase stems.`);
  }
  if (typeof taxonomy.testRunnerConfigurationCaseName !== "string" || !taxonomy.testRunnerConfigurationCaseName || /[\\/]/u.test(taxonomy.testRunnerConfigurationCaseName)) problems.push("testRunnerConfigurationCaseName must be one directory name.");
  if (!Array.isArray(taxonomy.testSubjectBoundaryOwnerKinds) || taxonomy.testSubjectBoundaryOwnerKinds.some((name) => typeof name !== "string" || !taxonomy.testOwnerKinds.includes(name))) problems.push("testSubjectBoundaryOwnerKinds must name owner collections from testOwnerKinds.");
  if (!Array.isArray(taxonomy.testDeliveryScopeDirectoryNames) || taxonomy.testDeliveryScopeDirectoryNames.some((name) => typeof name !== "string" || !name || /[\\/]/u.test(name))) problems.push("testDeliveryScopeDirectoryNames must contain non-empty directory names.");
  if (!Array.isArray(taxonomy.testJavaScriptFrameworkModules) || taxonomy.testJavaScriptFrameworkModules.some((name) => typeof name !== "string" || !name)) problems.push("testJavaScriptFrameworkModules must contain non-empty module specifiers.");
  if (!Array.isArray(taxonomy.testAssertionModules) || taxonomy.testAssertionModules.some((name) => typeof name !== "string" || !name)) problems.push("testAssertionModules must contain non-empty module specifiers.");
  try { new RegExp(taxonomy.testSelfTestDeclarationPattern, "u"); } catch { problems.push("testSelfTestDeclarationPattern must be a valid Unicode regular expression."); }
  if (!Array.isArray(taxonomy.testLegacyFilenamePatterns)) problems.push("testLegacyFilenamePatterns must be an array.");
  else for (const [index, entry] of taxonomy.testLegacyFilenamePatterns.entries()) {
    if (!entry || typeof entry.id !== "string" || !entry.id || typeof entry.pattern !== "string" || !entry.pattern) problems.push(`testLegacyFilenamePatterns[${index}] must declare an id and pattern.`);
    else try { new RegExp(entry.pattern, "u"); } catch { problems.push(`testLegacyFilenamePatterns[${index}].pattern is not a valid Unicode regular expression.`); }
  }
  if (!Array.isArray(taxonomy.testInPackageImplementations)) problems.push("testInPackageImplementations must be an array.");
  else for (const [index, entry] of taxonomy.testInPackageImplementations.entries()) {
    if (!entry || typeof entry.id !== "string" || !entry.id || typeof entry.implementation !== "string" || !Object.values(taxonomy.testImplementationIds).includes(entry.implementation) || typeof entry.rationale !== "string" || !entry.rationale) problems.push(`testInPackageImplementations[${index}] must declare an id, a registered implementation, and a rationale.`);
    else if (!taxonomy.testImplementationFileKindIds.includes(entry.fileKindId) || typeof entry.filenameSuffix !== "string" || !taxonomy.fileKinds[entry.fileKindId]?.extensionChains.some((extension) => entry.filenameSuffix.endsWith(extension) && entry.filenameSuffix.length > extension.length)) problems.push(`testInPackageImplementations[${index}] must name a test implementation file kind and a suffix that extends one of its extensions.`);
    else if (taxonomy.testLegacyFilenamePatterns.some((legacy) => new RegExp(legacy.pattern, "u").test(`x${entry.filenameSuffix}`))) problems.push(`testInPackageImplementations[${index}].filenameSuffix is also a legacy test filename pattern.`);
  }
  for (const [key, kindId] of [
    ["semanticManifestFileKindId", taxonomy.semanticManifestFileKindId], ["subsetsManifestFileKindId", taxonomy.subsetsManifestFileKindId],
    ["storyFileKindId", taxonomy.storyFileKindId], ["testFeatureFileKindId", taxonomy.testFeatureFileKindId],
    ["testContributionFileKindId", taxonomy.testContributionFileKindId], ["testOutputMarkerFileKindId", taxonomy.testOutputMarkerFileKindId],
    ["windowEmptyFacetFileKindId", taxonomy.windowEmptyFacetFileKindId], ["mutationComponentFileKindId", taxonomy.mutationComponentFileKindId], ["mutationDescriptorFileKindId", taxonomy.mutationDescriptorFileKindId], ["testOracleRegistryLocation.fileKindId", taxonomy.testOracleRegistryLocation?.fileKindId],
    ["testSchemaLocation.fileKindId", taxonomy.testSchemaLocation?.fileKindId],
  ] as const) if (!kindId || !taxonomy.fileKinds[kindId]) problems.push(`${key} references a missing file kind.`);
  if (taxonomy.semanticManifestFilenameOverrides !== undefined && record(taxonomy.semanticManifestFilenameOverrides, "semanticManifestFilenameOverrides")) {
    for (const [owner, name] of Object.entries(taxonomy.semanticManifestFilenameOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`semanticManifestFilenameOverrides[${JSON.stringify(owner)}] must name one exact repository-relative collection owner.`);
      const identity = leadingEmojiIdentity(name);
      const statuteProblems = pathEmojiStatuteFindings([{ path: name, nodeKind: "file" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities);
      if (!canonicalEmoji(identity.first) || identity.rest !== "manifest.json" || statuteProblems.some((problem) => problem.kind !== "duplicate") || fileKindIdForSourcePath(name, taxonomy) !== taxonomy.semanticManifestFileKindId) problems.push(`semanticManifestFilenameOverrides[${JSON.stringify(owner)}] must name one canonical semantic emoji followed by "manifest.json" in the semantic manifest file kind.`);
    }
  }
  if (taxonomy.subsetDirectoryOverrides !== undefined && record(taxonomy.subsetDirectoryOverrides, "subsetDirectoryOverrides")) {
    const subsetSlug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
    const anyId = taxonomy.subsetAnyId ?? "*";
    for (const [owner, directories] of Object.entries(taxonomy.subsetDirectoryOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner.split("/").some((segment) => !segment || segment === "." || segment === "..") || posix.basename(owner) !== taxonomy.subsetsDirName) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] must name one exact repository-relative ${JSON.stringify(taxonomy.subsetsDirName)} owner.`);
      if (!record(directories, `subsetDirectoryOverrides[${JSON.stringify(owner)}]`)) continue;
      const physicalNames = new Set<string>();
      for (const [id, name] of Object.entries(directories)) {
        if (id !== anyId && !subsetSlug.test(id)) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}][${JSON.stringify(id)}] must use the canonical logical subset id grammar.`);
        const expectedRest = id === anyId ? "any" : id;
        const identity = leadingEmojiIdentity(name);
        const statuteProblems = pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities);
        if (!canonicalEmoji(identity.first) || identity.rest !== expectedRest || statuteProblems.some((problem) => problem.kind !== "duplicate")) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}][${JSON.stringify(id)}] must name one canonical semantic emoji followed by ${JSON.stringify(expectedRest)}.`);
        if (physicalNames.has(foldPathEmojiIdentity(name))) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] has duplicate physical directory ${JSON.stringify(name)}.`);
        physicalNames.add(foldPathEmojiIdentity(name));
      }
      if (!Object.hasOwn(directories, anyId)) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] must declare the unconstrained subset id ${JSON.stringify(anyId)}.`);
    }
  }
  ids(taxonomy.textSpecFileKinds, taxonomy.fileKinds, "textSpecFileKinds");
  ids(taxonomy.binarySpecFileKinds, taxonomy.fileKinds, "binarySpecFileKinds");
  ids(taxonomy.rootDataContractIds, taxonomy.fixedFilenameContracts, "rootDataContractIds");
  ids(taxonomy.rootDocumentContractIds, taxonomy.fixedFilenameContracts, "rootDocumentContractIds");
  ids(taxonomy.repoWideContractIds, taxonomy.fixedFilenameContracts, "repoWideContractIds");

  for (const [formatId, format] of Object.entries(taxonomy.schemaFormats ?? {})) {
    if ("leafFilename" in (format as unknown as Record<string, unknown>) || "extension" in (format as unknown as Record<string, unknown>)) problems.push(`schemaFormats[${JSON.stringify(formatId)}] contains removed filename fields.`);
    if (!taxonomy.fileKinds[format.fileKindId]) problems.push(`schemaFormats[${JSON.stringify(formatId)}].fileKindId is missing.`);
    if (!["snake", "camel", "kebab"].includes(format.fieldCasing)) problems.push(`schemaFormats[${JSON.stringify(formatId)}].fieldCasing is invalid.`);
  }
  const declaredFacetKindPaths = new Set<string>();
  for (const [kindId, kind] of Object.entries(taxonomy.schemaFacetKinds ?? {})) {
    if (!kind.formats.includes(kind.normativeFormat)) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].formats must include its normative format.`);
    for (const formatId of kind.formats) if (!taxonomy.schemaFormats[formatId]) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}] references missing format ${JSON.stringify(formatId)}.`);
    if (!Array.isArray(kind.facetPathIdentities)) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].facetPathIdentities must be an array of exact facet module paths.`);
    else for (const facetPath of kind.facetPathIdentities) {
      if (typeof facetPath !== "string" || !facetPath || facetPath !== facetPath.normalize("NFC") || facetPath.startsWith("/") || /[\\*?]/u.test(facetPath) || facetPath.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].facetPathIdentities must name exact repository-relative facet modules.`);
      else if (declaredFacetKindPaths.has(facetPath)) problems.push(`schemaFacetKinds declares ${JSON.stringify(facetPath)} under more than one facet kind.`);
      else declaredFacetKindPaths.add(facetPath);
    }
  }
  problems.push(...validateSchemaScopeVocabulary(taxonomy));

  fullPattern(taxonomy.mutationDirectoryPattern, "mutationDirectoryPattern");
  problems.push(...mutationDomainOwnersProblems(taxonomy.mutationDomainOwners, taxonomy.pathEmojiPolicy.genericEmojiIdentities));
  problems.push(...mutationCatalogSourceOwnersProblems(taxonomy));
  const mutationBehaviorFacetDirs = ["🦠️mutation", "🔺️diff", "↩️inverse"];
  if (taxonomy.mutationBehaviorFacetDirs?.join("\0") !== mutationBehaviorFacetDirs.join("\0")) problems.push(`mutationBehaviorFacetDirs must contain exactly ${mutationBehaviorFacetDirs.join(", ")} in canonical order.`);
  const mutationOrganizationalFacetDirs = ["🧩️plan", "📝️text", "💾️binary", "🧬️schema"];
  if (taxonomy.mutationOrganizationalFacetDirs?.join("\0") !== mutationOrganizationalFacetDirs.join("\0")) problems.push(`mutationOrganizationalFacetDirs must contain exactly ${mutationOrganizationalFacetDirs.join(", ")} in canonical order.`);
  for (const dir of [...(taxonomy.mutationBehaviorFacetDirs ?? []), ...(taxonomy.mutationOrganizationalFacetDirs ?? [])]) if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) problems.push(`taxonomyLeafParentDirs must include mutation facet ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.mutationBehaviorFacetDirs ?? []) if (dir !== "🦠️mutation" && !taxonomy.mutationDirectLeafForbiddenRegionMarkers?.[dir]) problems.push(`mutationDirectLeafForbiddenRegionMarkers must name the forbidden inline region marker for behavior facet ${JSON.stringify(dir)}.`);
  if (taxonomy.mutationDirectLeafForbiddenRegionMarkers?.["🔺️diff"] !== "🔖️Diff" || taxonomy.mutationDirectLeafForbiddenRegionMarkers?.["↩️inverse"] !== "🔖️Inverse") problems.push('mutationDirectLeafForbiddenRegionMarkers must map "🔺️diff" to "🔖️Diff" and "↩️inverse" to "🔖️Inverse".');
  if (taxonomy.mutationComponentFileKindId !== "rust-source") problems.push('mutationComponentFileKindId must select the direct owner "rust-source" kind.');
  if (taxonomy.mutationDescriptorFileKindId !== "json") problems.push('mutationDescriptorFileKindId must select the language-neutral "json" kind.');
  if (record(taxonomy.mutationPayloadSchemaLocation, "mutationPayloadSchemaLocation")) {
    const location = taxonomy.mutationPayloadSchemaLocation;
    if (Object.keys(location).sort().join("\0") !== "directoryKindId\0directoryName\0fileKindId" || location.directoryKindId !== "schema" || location.directoryName !== "🧬️schema" || location.fileKindId !== "json" || semanticDirectoryKindId(location.directoryName, taxonomy) !== location.directoryKindId) problems.push("mutationPayloadSchemaLocation must select the registered schema directory and JSON physical leaf.");
  }

  if (record(taxonomy.mutationPayloadSchemaAuthority, "mutationPayloadSchemaAuthority")) {
    const expected = { contractKind: "descriptor-linked-mutation-payload-schema", ownerAuthority: "mutationOwnerIdentity", descriptorFileKindId: "json", descriptorField: "payloadSchema", descriptorSchemaVersion: 1, descriptorCardinality: "one-canonical-no-competing-descriptor", descriptorOwnerField: "owner", descriptorIdentityField: "semanticKind", jsonSchemaDialect: "http://json-schema.org/draft-07/schema#", targetAuthority: "owner-relative-regular-json-schema" };
    const authority = taxonomy.mutationPayloadSchemaAuthority;
    if (Object.keys(authority).length !== Object.keys(expected).length || Object.entries(expected).some(([key, value]) => authority[key as keyof typeof authority] !== value)) problems.push("mutationPayloadSchemaAuthority must bind exact mutation descriptors to regular owner-contained JSON Schema files.");
  }

  const directoryValues = [
    taxonomy.packagesDirName, taxonomy.targetsDirName, taxonomy.elementsDirName, taxonomy.artifactsDirName, taxonomy.modesDirName,
    taxonomy.windowsDirName, taxonomy.standardsDirName, taxonomy.subsetsDirName, taxonomy.viewerDirName, taxonomy.editorDirName,
    taxonomy.exampleAssetsDirName, taxonomy.exampleTestsDirName, taxonomy.testExamplesDirName, taxonomy.testOraclesDirName, ...taxonomy.artifactChildDirs, ...taxonomy.newArtifactChildDirs,
    ...taxonomy.standardChildDirs, ...taxonomy.subsetChildDirs, ...taxonomy.surfaceChildDirs, ...taxonomy.modeChildDirs,
    ...taxonomy.windowChildDirs, ...taxonomy.taxonomyLeafParentDirs, ...taxonomy.pluginChildDirs, ...taxonomy.osChildDirs,
    ...taxonomy.rootDataDirNames, ...taxonomy.schemaChildDirs, ...taxonomy.representationDirs, ...taxonomy.ioDirectionDirs,
    ...taxonomy.ioSemanticCollectionDirNames, ...Object.values(taxonomy.ioDirectionChildDirs), ...taxonomy.mutationBehaviorFacetDirs, ...taxonomy.mutationOrganizationalFacetDirs,
  ];
  if (!Array.isArray(taxonomy.nativeRepresentationDirs) || taxonomy.nativeRepresentationDirs.length === 0 || new Set(taxonomy.nativeRepresentationDirs).size !== taxonomy.nativeRepresentationDirs.length || taxonomy.nativeRepresentationDirs.some(directory => !taxonomy.representationDirs.includes(directory))) problems.push("nativeRepresentationDirs must be a non-empty unique subset of representationDirs.");
  for (const directory of new Set(directoryValues)) if (!semanticDirectoryKindId(directory, taxonomy)) problems.push(`Semantic directory ${JSON.stringify(directory)} is not uniquely registered.`);
  for (const dir of taxonomy.artifactComponentDirs) if (!taxonomy.artifactChildDirs.includes(dir)) problems.push(`artifactChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.windowRequiredChildDirs) if (!taxonomy.windowChildDirs.includes(dir)) problems.push(`windowChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.surfaceRequiredChildDirs) if (!taxonomy.surfaceChildDirs.includes(dir)) problems.push(`surfaceChildDirs must include ${JSON.stringify(dir)}.`);
  return problems;
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
    if (resolve(execFileSync("git", ["rev-parse", "--show-toplevel"], gitOptions).trim()) !== root) return [];
    const admitted = execFileSync("git", ["--literal-pathspecs", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", row.sourceRoot], gitOptions).split("\0").filter(Boolean).sort(projectionByteCompare);
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
                catch { problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must route exactly to its declared script invocation in its owner project.`); }
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
  return Object.values(taxonomy.pathExclusions).some((exclusion) => {
    const prefix = exclusion.path.replace(/^\.\//u, "").replace(/\/+$/u, "");
    return rel === prefix || rel.startsWith(`${prefix}/`);
  });
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
    const absolute = checked(path);
    if (kinds.has(path)) return kinds.get(path)!;
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
