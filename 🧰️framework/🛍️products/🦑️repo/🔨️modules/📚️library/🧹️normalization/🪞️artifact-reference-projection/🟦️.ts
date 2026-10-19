import { requireRecord, type UnknownRecord } from "@semio-tech/schema-validator-ts";
import { report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type SemanticDistributedJsonManifestCatalogContract, type SemanticExactOwnerVectorsCatalogContract } from "../🔣️taxonomy/🧭️path-projection/🟦️.ts";
import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, createLexicalInputSession, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { semanticExactOwnedDocumentCorrectionAuthority } from "../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { renderArtifactPathProjectionRoot } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { semanticPackageJoinedPathReferenceAuthority, semanticPackageAuthoredFragmentReferences, type SemanticPackageProjectionCase, type OwnerProjectionPolicy } from "../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";
import { type SemanticFacetPrimaryFileProjectionContract, type SemanticPathProjectionReferenceConsumerForm } from "../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";
import { basename, dirname, posix } from "node:path";
import { createTaxonomyPathMatcher } from "../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import { semanticPathProjectionAuthority, semanticPathProjectionReferenceConsumers } from "../../🔍️discovery/🪞️path-projection/🟦️.ts";
import { type ReferenceEdit, type TaxonomyEvidenceRemoval, type TaxonomyInventory, type TaxonomyInventoryEntry, type TaxonomyMove, type TaxonomyPlanOptions, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { areaId, checkCancellation, isExcluded, ownerId, stableViolations, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type ReferenceToken, lineLocation } from "../🔤️reference-tokenizers/🟦️.ts";
import { applyEditsToContent, opaqueLeafNames, referenceAdapter, referenceEditConflict, referenceResolutionReach, unsupportedReferenceTokens } from "../✂️reference-edits/🟦️.ts";
import { jsonStringCoordinates } from "../➰️symlink-targets/🟦️.ts";
import { inventoryEntriesBySourcePath, inventoryLeafStamps, leafStampCurrent, referenceEditIdentity, taxonomyFixturePayloadPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { generatorContractsForOutputPath } from "../📸️repository-capture/🟦️.ts";
import { type ArtifactProjectionRationale, artifactProjectionAuthorityNodes, artifactProjectionContracts, exactOwnedFileCatalog, nestedCargoCatalog } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { inventoryLeafPreimage } from "../🧐️preimage-assertions/🟦️.ts";
import { artifactProjectionSourceLocation } from "../📂️owned-file-projection/🟦️.ts";
import { frozenEvidenceContractIndex, frozenEvidenceRecord, frozenPlanCoordinateAuthority, isFrozenSourceCoordinateToken } from "../❄️frozen-evidence/🟦️.ts";
import { type ReferencePathIndex, indexedReferenceTokens, rustReferenceInterpretationCovers, splitTokenSuffix } from "../🦀️rust-reference-graph/🟦️.ts";
import { referencePathIndex, resolveReferenceTokenPath, rewriteReferenceToken, rewriteReferenceValue, visitReferenceTokenReach } from "../🧭️reference-resolution/🟦️.ts";
import { type ReferenceAnalysis, ancestorInTargets, referenceDigestMayResolveInto, referenceDigestResolvesInto, referenceTokenStore, referenceTokens, textualPath } from "../🧮️reference-digests/🟦️.ts";
import { storedReferenceAnalysis } from "../🔬️reference-analysis/🟦️.ts";
import { incomingReferenceSnapshot } from "../📥️incoming-references/🟦️.ts";

interface ArtifactReferenceProjection {
  readonly id: string;
  readonly artifactRoot: string;
  readonly sourceRoot: string;
  readonly destinationRoot: string;
  readonly rationaleRule: ArtifactProjectionRationale;
  readonly catalog: SemanticDistributedJsonManifestCatalogContract | SemanticExactOwnerVectorsCatalogContract;
  readonly mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[];
  readonly authorityProblems: readonly string[];
}

function artifactReferenceProjections(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): readonly ArtifactReferenceProjection[] {
  const rows: ArtifactReferenceProjection[] = [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const catalog = requireTaxonomyValue(taxonomy.schema.semanticPathProjectionCatalogContracts[contract.catalogContractId],"projection catalog");
    if (!("contractKind" in catalog)) continue;
    for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "directory")) {
      const location = artifactProjectionSourceLocation(entry.sourcePath, contract, taxonomy);
      if (!location || location.sourceRoot !== entry.sourcePath) continue;
      const mappings = moves.filter((move) => move.rationaleRule === contract.rationaleRule && move.sourcePath.startsWith(`${location.sourceRoot}/`)).map(({ sourcePath, destinationPath }) => ({ sourcePath, destinationPath }));
      if (mappings.length === 0) continue;
      const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot }, taxonomy.discoverySchema);
      const authority = semanticPathProjectionAuthority({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot, nodes: artifactProjectionAuthorityNodes(inventory.repoRoot, location.sourceRoot, entries, taxonomy) }, taxonomy.discoverySchema);
      const orderedMappings = mappings.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
      const mappingProblems = canonicalJson(orderedMappings) === canonicalJson(authority.mappings) ? [] : ["Planned artifact mappings do not equal the schema projection authority"];
      rows.push({ id, artifactRoot: location.artifactRoot, sourceRoot: location.sourceRoot, destinationRoot: rendered.destinationRoot, rationaleRule: contract.rationaleRule, catalog, mappings: orderedMappings, authorityProblems: [...rendered.problems, ...authority.problems, ...mappingProblems] });
    }
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourceRoot, right.sourceRoot) || left.id.localeCompare(right.id));
}

/** 🗿️ Returns the canonical artifact-owned suffix used to bind structural reference tokens. */
export function artifactProjectionTail(path: string): string {
  const marker = "🗿️artifacts/";
  const index = path.indexOf(marker);
  return index < 0 ? path : path.slice(index);
}

function artifactReferenceForm(token: ReferenceToken): SemanticPathProjectionReferenceConsumerForm | null {
  if (token.rewriteKind === "artifact-catalog-glob") return "artifact-catalog-glob";
  if (token.rewriteKind !== "artifact-catalog-prose") return null;
  const form = token.rewriteData?.form;
  return form === "root-marker" || form === "relative-root" || form === "category-glob" || form === "catalog-grammar" ? `artifact-catalog-prose:${form}` : null;
}

function registeredArtifactConsumers(context: ArtifactReferenceProjection, referencePath: string, token: ReferenceToken, taxonomy: LoadedTaxonomy): readonly string[] {
  const form = artifactReferenceForm(token);
  if (!form || !["rust", "typescript", "json", "toml"].includes(token.adapter)) return [];
  return semanticPathProjectionReferenceConsumers(context.id, referencePath, token.adapter as "rust" | "typescript" | "json" | "toml", form, taxonomy.discoverySchema).map((row) => row.id);
}

function catalogProjectionForToken(referencePath: string, token: ReferenceToken, contexts: readonly ArtifactReferenceProjection[], taxonomy: LoadedTaxonomy): ArtifactReferenceProjection | string | null {
  const selectors = token.rewriteData?.selectors ? JSON.parse(token.rewriteData.selectors) as string[] : [];
  const matches = contexts.filter((context) => {
    if (context.rationaleRule !== "artifact-example-model-catalog-projection-v1") return false;
    const authorized = referencePath === context.artifactRoot || referencePath.startsWith(`${context.artifactRoot}/`) || registeredArtifactConsumers(context, referencePath, token, taxonomy).length === 1;
    const selectorMatches = selectors.length === 0 || selectors.some((selector) => selector.includes(artifactProjectionTail(context.sourceRoot)));
    return authorized && selectorMatches;
  });
  if (matches.length === 1) return matches[0] ?? null;
  if (matches.length > 1) return `Reference form matches multiple artifact projection owners: ${matches.map((row) => row.id).join(", ")}`;
  const cad = contexts.filter((context) => context.rationaleRule === "artifact-example-model-catalog-projection-v1");
  if (cad.length > 0) return selectors.length > 0 ? "Artifact selector or reference file does not match a registered source owner" : "Artifact catalog prose occurs outside an authorized owner or consumer location";
  return null;
}

function renderCatalogGlob(referencePath: string, token: ReferenceToken, context: ArtifactReferenceProjection): string | Readonly<{ problem: string }> {
  if (context.catalog.contractKind !== "distributed-json-manifest-catalog") return { problem: `${context.id} has no distributed catalog grammar` };
  const selectors = JSON.parse(token.rewriteData?.selectors ?? "[]") as string[];
  if (selectors.length === 0 || selectors.some((selector) => typeof selector !== "string")) return { problem: "Artifact catalog glob has no exact literal selectors" };
  const pathMatcher = createTaxonomyPathMatcher();
  const sourceTail = artifactProjectionTail(context.sourceRoot);
  const baseRelative = posix.relative(dirname(referencePath), context.destinationRoot);
  const base = baseRelative.startsWith(".") ? baseRelative : `./${baseRelative}`;
  const zeroSource = [/\/\*\*\/🔣️extension\.json$/u, /\/\*\*\/🏷️properties\/\*\.json$/u, /\/\*\*\/🔧️properties\/\*\.json$/u];
  const rendered: string[] = [];
  for (const selector of selectors) {
    const tailIndex = selector.indexOf(sourceTail);
    if (tailIndex < 0) return { problem: `Artifact selector does not contain its registered source owner: ${selector}` };
    const suffix = selector.slice(tailIndex + sourceTail.length);
    const sourcePattern = `${context.sourceRoot}${suffix}`;
    const admitted = context.mappings.filter((mapping) => pathMatcher.matches(mapping.sourcePath, sourcePattern));
    if (admitted.length === 0) {
      if (zeroSource.some((pattern) => pattern.test(suffix))) continue;
      return { problem: `Nonempty artifact selector has no exact authority mapping: ${selector}` };
    }
    if (suffix.endsWith(`/${context.catalog.modelManifestSourceFilename}`)) {
      rendered.push(`${base}/*/🔣️.json`);
      continue;
    }
    const rules = context.catalog.categoryRules.filter((rule) => suffix.includes(`/${rule.sourceDirectoryName}/`));
    if (rules.length !== 1) return { problem: `Artifact selector has no unique registered category: ${selector}` };
    rendered.push(`${base}/**/${requireTaxonomyValue(rules[0],"rules[0]").sourceDirectoryName}/${requireTaxonomyValue(rules[0],"rules[0]").sourceShape === "nested-fixed-json" ? "**" : "*"}/🔣️.json`);
  }
  if (rendered.length === 0) {
    if (!/^\s*[A-Za-z_$][\w$]*\s*:\s*import\.meta\.glob\([^\r\n]+\),?\s*(?:as\s+[^;]+)?;?\s*(?:\r?\n)?$/u.test(token.value)) return { problem: "Zero-source artifact selectors cannot be removed without owning their complete object member" };
    return "";
  }
  if (rendered.length !== selectors.length) return { problem: "Artifact selector list mixes registered and zero-source selectors" };
  let result = token.value;
  for (let index = 0; index < selectors.length; index++) {
    const quoted = JSON.stringify(selectors[index]);
    if (!result.includes(quoted)) return { problem: "Artifact selector raw literal is not canonical JSON-compatible syntax" };
    result = result.replace(quoted, JSON.stringify(rendered[index]));
  }
  return result;
}

function artifactStructuralReferenceRewrite(referencePath: string, token: ReferenceToken, contexts: readonly ArtifactReferenceProjection[], taxonomy: LoadedTaxonomy): Readonly<{ newValue?: string; problem?: string }> | null {
  if (token.rewriteKind !== "artifact-catalog-glob" && token.rewriteKind !== "artifact-catalog-prose") return null;
  const selected = catalogProjectionForToken(referencePath, token, contexts, taxonomy);
  if (typeof selected === "string") return { problem: selected };
  if (!selected) return null;
  if (token.rewriteKind === "artifact-catalog-glob") {
    const rendered = renderCatalogGlob(referencePath, token, selected);
    return typeof rendered === "string" ? { newValue: rendered } : rendered;
  }
  if (selected.catalog.contractKind !== "distributed-json-manifest-catalog") return { problem: `${selected.id} cannot render catalog prose` };
  const root = posix.relative(selected.artifactRoot, selected.destinationRoot);
  if (token.rewriteData?.form === "root-marker") return { newValue: `${root}/` };
  if (token.rewriteData?.form === "relative-root") {
    const source = posix.relative(selected.artifactRoot, selected.sourceRoot);
    return token.value === source || token.value.endsWith(`/${source}`) ? { newValue: `${token.value.slice(0, token.value.length - source.length)}${root}` } : { problem: `${selected.id} root join ${token.value} does not end at its artifact source root` };
  }
  if (token.rewriteData?.form === "category-glob") {
    const category = token.rewriteData.category ?? "";
    return selected.catalog.categoryRules.some((rule) => rule.sourceDirectoryName === category) ? { newValue: `${root}/*/${category}/${token.rewriteData.members === "json" ? "*/🔣️.json" : ""}` } : { problem: `${selected.id} has no catalog category ${category}` };
  }
  if (token.rewriteData?.form === "catalog-grammar") {
    const members = selected.catalog.categoryRules.map((rule) => `${rule.sourceDirectoryName}/<member>/🔣️.json`).sort(generatorPathCompare);
    return { newValue: `${root}/<model>/{${members.join(",")},🔣️.json}` };
  }
  return { problem: `Unknown artifact catalog prose form ${token.rewriteData?.form ?? ""}` };
}

function externalProjectionReferenceEntries(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], contexts: readonly ArtifactReferenceProjection[], taxonomy: LoadedTaxonomy): readonly TaxonomyInventoryEntry[] {
  const active = new Set(contexts.map((context) => context.id));
  const admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
  const rows: TaxonomyInventoryEntry[] = [];
  for (const path of Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts)
    .filter((contract) => active.has(contract.projectionContractId))
    .flatMap((contract) => contract.sourcePathIdentities)
    .filter((path, index, values) => values.indexOf(path) === index && !admitted.has(path))
    .sort(generatorPathCompare)) {
    if (isExcluded(path, taxonomy)) throw new Error(`Declared projection consumer crosses opaque path ${path}`);
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Declared projection consumer", true);
    const stat = lstatOrNull(absolute);
    if (!stat) continue;
    if (!stat.isFile() || stat.isSymbolicLink()) throw new Error(`Declared projection consumer is not a regular file: ${path}`);
    const bytes = readFileSync(absolute);
    rows.push({ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] });
  }
  return rows;
}

interface ExactOwnerMoveIndex {
  readonly moving: ReadonlySet<string>;
  readonly owners: readonly OwnerProjectionPolicy["entries"][number][];
  readonly authoredMoves: ReadonlyMap<string, TaxonomyMove>;
  readonly markdownSources: ReadonlySet<string>;
}

const exactOwnerMoveIndexes = new WeakMap<readonly TaxonomyMove[], WeakMap<OwnerProjectionPolicy, ExactOwnerMoveIndex>>();

/** 🧭️ The README-owner moves and catalog cases every exact-owner token check consults, derived once per move list and catalog. */
function exactOwnerMoveIndex(catalog: OwnerProjectionPolicy, moves: readonly TaxonomyMove[]): ExactOwnerMoveIndex {
  let byCatalog = exactOwnerMoveIndexes.get(moves);
  if (!byCatalog) exactOwnerMoveIndexes.set(moves, byCatalog = new WeakMap());
  const cached = byCatalog.get(catalog);
  if (cached) return cached;
  const moving = new Set(moves.filter((move) => move.rationaleRule === "readme-license-owner-projection-v1").map((move) => move.sourcePath)), authoredMoves = new Map<string, TaxonomyMove>();
  for (const move of moves) if (moving.has(move.sourcePath) && !authoredMoves.has(move.sourcePath)) authoredMoves.set(move.sourcePath, move);
  const index = { moving, owners: catalog.entries.filter((entry) => moving.has(entry.sourcePath)), authoredMoves, markdownSources: new Set(catalog.entries.filter((entry) => entry.referenceOwnerIds.includes("markdown-relative-reference-adapter")).map((entry) => entry.sourcePath)) };
  byCatalog.set(catalog, index);
  return index;
}

function exactOwnedReferenceTokens(path: string, content: string, catalog: OwnerProjectionPolicy | null, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): Readonly<{ tokens: readonly ReferenceToken[]; problems: readonly string[] }> {
  if (!catalog) return { tokens: [], problems: [] };
  const { owners, authoredMoves } = exactOwnerMoveIndex(catalog, moves);
  const tokens: ReferenceToken[] = [], problems: string[] = [];
  const authoredMove = authoredMoves.get(path);
  const authoredContract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (authoredMove && authoredContract?.contractKind === "exact-owner-path-catalog") {
    const result = semanticExactOwnedDocumentCorrectionAuthority(catalog, authoredContract, { path, finalPath: authoredMove.destinationPath, content, mode: authoredMove.sourcePreimage.mode, moving: true });
    problems.push(...result.problems);
    for (const splice of result.splices) tokens.push({ adapter: "markdown", structuredLocation: lineLocation(content, splice.start, "exact-owner-authored-" + splice.correctionId), start: splice.start, end: splice.end, value: splice.oldValue, targetValues: [path], rewriteKind: "exact-owner-reference", rewriteData: { newValue: splice.newValue } });
  }
  for (const id of ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader"]) {
    const selected = owners.filter((entry) => entry.referenceOwnerIds.includes(id));
    if (selected.length === 0 || requireTaxonomyValue(catalog.referenceOwners[id],"catalog.referenceOwners[id]").ownerPath !== path) continue;
    const forms = id === "repo-cli-dev-docs-go" ? [
      { oldValue: 'filepath.Join(rootDir, technology.Root, "README.md")', newValue: 'filepath.Join(rootDir, technology.Root, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'filepath.Join(rootDir, bundle.Root, "README.md")', newValue: 'filepath.Join(rootDir, bundle.Root, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'filepath.Join(bundleRoot, "README.md")', newValue: 'filepath.Join(bundleRoot, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'filepath.Join(rootDir, name, "README.md")', newValue: 'filepath.Join(rootDir, name, "📃️readme", "📝️.md")', count: 2 },
      { oldValue: 'd.Name() == "README.md"', newValue: 'd.Name() == "📝️.md" && filepath.Base(filepath.Dir(path)) == "📃️readme"', count: 1 },
    ] : [{ oldValue: 'std::fs::read_to_string("README.md")', newValue: 'std::fs::read_to_string("📃️readme/📝️.md")', count: 1 }];
    for (const [index, form] of forms.entries()) {
      const count = (value: string): number => content.split(value).length - 1;
      if (count(form.oldValue) + count(form.newValue) !== form.count) {
        problems.push(id + " concrete consumer form " + index + " drifted");
        continue;
      }
      for (let start = content.indexOf(form.oldValue); start >= 0; start = content.indexOf(form.oldValue, start + form.oldValue.length)) {
        tokens.push({ adapter: id === "repo-cli-dev-docs-go" ? "go" : "rust", structuredLocation: lineLocation(content, start, "exact-owner-" + id + "-" + index), start, end: start + form.oldValue.length, value: form.oldValue, targetValues: [requireTaxonomyValue(selected[0],"selected[0]").sourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue: form.newValue } });
      }
    }
  }
  return { tokens, problems };
}

function externalExactOwnedReferenceEntries(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], catalog: OwnerProjectionPolicy | null, taxonomy: LoadedTaxonomy): readonly TaxonomyInventoryEntry[] {
  if (!catalog || !moves.some((move) => move.rationaleRule === "readme-license-owner-projection-v1")) return [];
  const admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
  const moving = new Set(moves.map((move) => move.sourcePath));
  const required = new Set(catalog.entries.filter((entry) => moving.has(entry.sourcePath)).flatMap((entry) => entry.referenceOwnerIds).filter((id) => ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader"].includes(id)).map((id) => requireTaxonomyValue(catalog.referenceOwners[id],"catalog.referenceOwners[id]").ownerPath));
  const paths = new Set(required);
  for (const entry of catalog.entries) {
    const source = assertLexicalInputOutsideOpaque(inventory.repoRoot, entry.sourcePath, "Exact owner reference", true);
    const active = lstatOrNull(source) ? entry.sourcePath : entry.destinationPath;
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, active, "Exact owner reference", true);
    if (lstatOrNull(absolute)?.isFile()) paths.add(active);
  }
  const rows: TaxonomyInventoryEntry[] = [];
  for (const path of [...paths].sort(generatorPathCompare)) {
    if (admitted.has(path)) continue;
    if (isExcluded(path, taxonomy)) throw new Error("Exact owner consumer crosses an opaque path: " + path);
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Exact owner consumer", true), stat = lstatOrNull(absolute);
    if (!stat?.isFile()) {
      if (required.has(path)) throw new Error("Exact owner consumer is absent or not a regular file: " + path);
      continue;
    }
    const bytes = readFileSync(absolute);
    rows.push({ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] });
  }
  return rows;
}

function exactOwnedMarkdownTarget(referencePath: string, token: ReferenceToken, catalog: OwnerProjectionPolicy | null, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): string | null {
  if (token.rewriteKind === "exact-owner-reference" || token.adapter !== "markdown" || !catalog) return null;
  const index = exactOwnerMoveIndex(catalog, moves);
  if (!index.markdownSources.has(referencePath) || !index.moving.has(referencePath)) return null;
  const value = splitTokenSuffix(token.value).path;
  if (!value || /^(?:[a-z][a-z0-9+.-]*:|\/|#|\$|\{)/iu.test(value) || /[*{}]/u.test(value)) return null;
  try {
    const path = normalizeRelative(posix.join(dirname(referencePath), decodeURI(value)));
    return isExcluded(path, taxonomy) ? null : path;
  } catch { return null; }
}

const artifactEmptyFacetReferenceMoves = new WeakMap<readonly TaxonomyMove[], WeakMap<LoadedTaxonomy, TaxonomyMove | null>>();

/** 🫙️ The first move the artifact empty-facet reference consumer rewrites for, found once per move list and taxonomy. */
function artifactEmptyFacetReferenceMove(moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): TaxonomyMove | undefined {
  let byTaxonomy = artifactEmptyFacetReferenceMoves.get(moves);
  if (!byTaxonomy) artifactEmptyFacetReferenceMoves.set(moves, byTaxonomy = new WeakMap());
  if (byTaxonomy.has(taxonomy)) return byTaxonomy.get(taxonomy) ?? undefined;
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  const move = contract?.contractKind !== "semantic-facet-primary-file" ? undefined : moves.find((candidate) => candidate.rationaleRule === contract.rationaleRule && candidate.sourcePath.startsWith(`${contract.referenceConsumer.ownerRoot}/`));
  byTaxonomy.set(taxonomy, move ?? null);
  return move;
}

function externalArtifactEmptyFacetReferenceEntries(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): readonly TaxonomyInventoryEntry[] {
  if (!artifactEmptyFacetReferenceMove(moves, taxonomy)) return [];
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"] as SemanticFacetPrimaryFileProjectionContract;
  const path = contract.referenceConsumer.path;
  if (inventory.entries.some((entry) => entry.sourcePath === path)) return [];
  const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Artifact empty-facet consumer", true), stat = lstatOrNull(absolute);
  if (!stat) return [];
  if (!stat.isFile()) throw new Error(`Artifact empty-facet consumer must be a regular file: ${path}`);
  const bytes = readFileSync(absolute);
  return [{ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] }];
}

function artifactEmptyFacetReferenceTokens(path: string, content: string, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): Readonly<{ tokens: readonly ReferenceToken[]; problems: readonly string[] }> {
  const move = artifactEmptyFacetReferenceMove(moves, taxonomy);
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  if (!move || contract?.contractKind !== "semantic-facet-primary-file" || contract.referenceConsumer.path !== path) return { tokens: [], problems: [] };
  const consumer = contract.referenceConsumer;
  const boundaries = [...content.matchAll(/^\/\/#(region|endregion) ([^\r\n]+)$/gmu)].filter((match) => match[2] === consumer.region);
  const invalid = { tokens: [], problems: ["Artifact empty-facet energy surface-region prose drifted"] };
  if (boundaries.length !== 2 || boundaries[0]![1] !== "region" || boundaries[1]![1] !== "endregion") return invalid;
  const oldValue = consumer.lineTemplate.replace("{filename}", contract.sourceFilename);
  const fileKind = taxonomy.schema.fileKinds[taxonomy.schema.windowEmptyFacetFileKindId]!;
  const newValue = consumer.lineTemplate.replace("{filename}", `${fileKind.emoji}${fileKind.extensionChains[0]}`);
  const lines = [...content.matchAll(/^[^\r\n]*$/gmu)].filter((match) => match.index! > boundaries[0]!.index! && match.index! < boundaries[1]!.index! && (match[0] === oldValue || match[0] === newValue));
  if (lines.length !== 1 || content.split(contract.sourceFilename).length - 1 !== Number(lines[0]![0] === oldValue)) return invalid;
  if (lines[0]![0] === newValue) return { tokens: [], problems: [] };
  const start = lines[0]!.index!;
  return { tokens: [{ adapter: "rust", structuredLocation: lineLocation(content, start, "artifact-empty-facet-energy-surfaces"), start, end: start + oldValue.length, value: oldValue, targetValues: [move.sourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue } }], problems: [] };
}

function nestedCargoReferenceTokens(path: string, content: string, packages: readonly SemanticPackageProjectionCase[]): Readonly<{ tokens: readonly ReferenceToken[]; problems: readonly string[] }> {
  const row = packages.find((entry) => entry.mappings.some((mapping) => mapping.sourcePath === path));
  if (!row) return { tokens: [], problems: [] };
  const mapping = row.mappings.find((entry) => entry.sourcePath === path)!;
  if (sha256(content) !== mapping.sourceHash || Buffer.byteLength(content) !== mapping.sourceSize) return { tokens: [], problems: ["Nested Cargo reference source preimage drift: " + path] };
  const tokens: ReferenceToken[] = [], problems: string[] = [];
  const authored = semanticPackageAuthoredFragmentReferences({ path, content, layout: "source" }, row);
  problems.push(...authored.problems);
  for (const reference of authored.references) tokens.push({ adapter: referenceAdapter(path), structuredLocation: lineLocation(content, reference.start, "nested-cargo-authored-fragment"), start: reference.start, end: reference.end, value: reference.oldValue, targetValues: [reference.targetSourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue: reference.newValue } });
  for (const binding of row.joinedPathBindings.filter((binding) => path === row.sourceRoot + "/" + binding.consumerRelativePath)) {
    const authority = semanticPackageJoinedPathReferenceAuthority({ path, content, layout: "source" }, binding, row);
    problems.push(...authority.problems);
    for (const reference of authority.references) tokens.push({ adapter: "typescript", structuredLocation: lineLocation(content, reference.start, "nested-cargo-joined-path"), start: reference.start, end: reference.end, value: reference.oldValue, targetValues: [reference.targetSourcePath], rewriteKind: "exact-owner-reference", rewriteData: { newValue: reference.newValue } });
  }
  const add = (needle: string, value: string, newValue: string, target: string, adapter: "rust" | "toml" | "json" | "typescript") => {
    const start = content.indexOf(needle);
    if (start < 0 || content.indexOf(needle, start + needle.length) >= 0) { problems.push("Nested Cargo exact reference is missing or repeated: " + needle); return; }
    const offset = start + needle.indexOf(value);
    tokens.push({ adapter, structuredLocation: lineLocation(content, offset, "nested-cargo-reference"), start: offset, end: offset + value.length, value, targetValues: [target], rewriteKind: "exact-owner-reference", rewriteData: { newValue } });
  };
  for (const splice of row.sourceSplices) if (path === splice.sourcePath) add(splice.oldValue, splice.oldValue, splice.newValue, splice.sourcePath, "rust");
  if (path === row.sourceRoot + "/Cargo.toml") {
    if (row.id === "jcoprobe-guest") add('path = "🦀️.rs"', "🦀️.rs", "📚️library/🦀️.rs", row.sourceRoot + "/🦀️.rs", "toml");
    else {
      add('build = "build.rs"', "build.rs", "🏗️builder/🦀️.rs", row.sourceRoot + "/build.rs", "toml");
      add('path = "🦀️lib.rs"', "🦀️lib.rs", "📚️library/🦀️.rs", row.sourceRoot + "/🦀️lib.rs", "toml");
      add('path = "📦️bin.rs"', "📦️bin.rs", "💾️binary/🦀️.rs", row.sourceRoot + "/📦️bin.rs", "toml");
    }
  }
  if (row.id === "wgpu-renderer" && path === row.sourceRoot + "/package.json") {
    add('".": "./🟦️.ts"', ".", ".", row.sourceRoot, "json");
    add('".": "./🟦️.ts"', "./🟦️.ts", "./🟦️typescript/📚️library/🟦️.ts", row.sourceRoot + "/🟦️.ts", "json");
    add('"directory": "framework/product/os/module/renderer/wgpu"', "framework/product/os/module/renderer/wgpu", row.destinationRoot, row.sourceRoot, "json");
  }
  if (row.id === "wgpu-renderer" && path === row.sourceRoot + "/📋️project.json") {
    const named = content.match(/"namedInputs"\s*:\s*\{\s*"default"\s*:\s*(\[[^\]]+\])/u);
    if (!named || !Array.isArray(JSON.parse(named[1]!))) problems.push("Nested Cargo exact Nx default inputs drifted");
    else add(named[0], named[1]!, JSON.stringify([...JSON.parse(named[1]!), `{workspaceRoot}/${row.semanticOwnerRoot}/**/*`]), row.sourceRoot, "json");
  }
  if (row.id === "wgpu-renderer" && path === row.sourceRoot + "/📜️script.ts") {
    const config = row.mappings.find((entry) => entry.sourcePath === row.sourceRoot + "/🧪️vitest.config.ts")!;
    const calls = [...content.matchAll(/^    await runVitest\(this\.root, [^\r\n]+\);$/gmu)];
    if (calls.length !== 3 || calls.some((call) => !call[0].endsWith(', "🧪️vitest.config.ts");'))) problems.push("Nested Cargo exact Vitest call authority drifted");
    else for (const call of calls) {
      add(call[0], basename(config.sourcePath), posix.relative(row.destinationRoot, config.destinationPath), config.sourcePath, "typescript");
      for (const selector of row.mappings.filter((entry) => entry.sourcePath.endsWith(".test.ts") && call[0].includes(JSON.stringify(basename(entry.sourcePath))))) add(call[0], basename(selector.sourcePath), posix.relative(row.destinationRoot, selector.destinationPath), selector.sourcePath, "typescript");
    }
  }
  if (row.id === "jcoprobe-guest" && path === row.sourceRoot + "/🦀️.rs") {
    const target = row.sourceRoot + "/🧬️schema/📜️world.wit", rendered = "../📦️packages/🦀️rust/🧬️schema/📜️world.wit";
    add('path: "🧬️schema/📜️world.wit"', "🧬️schema/📜️world.wit", "🧬️schema/📜️world.wit", target, "rust");
    add("`👽️guest/🧬️schema/📜️world.wit`", "👽️guest/🧬️schema/📜️world.wit", rendered, target, "rust");
  }
  return { tokens, problems };
}

interface DependencyPolicyStateTokens {
  readonly contentHash: string;
  readonly active: boolean;
  readonly manifestSources: ReadonlySet<string>;
  readonly tokens: readonly ReferenceToken[];
  readonly problems: readonly string[];
}

/** 🔒️ Rewrites only manifest-user coordinates, never dependency approvals or generated metadata. */
function dependencyPolicyStateTokens(repoRoot: string, path: string, content: string, packages: readonly SemanticPackageProjectionCase[], moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy): DependencyPolicyStateTokens {
  const contract = taxonomy.discoverySchema.semanticPolicyStateCoordinateContracts["dependency-freeze-users-v1"]!;
  const owners = packages.filter((row) => contract.packageIds.some((id) => id === row.id));
  const active = path === contract.statePath && owners.length > 0, tokens: ReferenceToken[] = [], problems: string[] = [], manifestSources = new Set<string>();
  const finish = (): DependencyPolicyStateTokens => ({ contentHash: sha256(content), active, manifestSources: problems.length ? new Set() : manifestSources, tokens: problems.length ? [] : tokens, problems });
  if (!active) return finish();
  try {
    const baseline = requireRecord(JSON.parse(content), "Dependency policy state"), coordinates = jsonStringCoordinates(content);
    if (baseline.schemaVersion !== contract.stateSchemaVersion || typeof baseline.generatedAt !== "string" || typeof baseline.commit !== "string" || !Array.isArray(baseline.entries) || coordinates.length === 0) throw new Error("Dependency policy state requires exact schema, metadata, entries and unambiguous JSON coordinates");
    const identities = new Set<string>();
    const entries = baseline.entries.map((value, index) => {
      const entry = requireRecord(value, `Dependency policy entry ${index}`), key = `${entry.ecosystem}:${entry.name}`;
      if (typeof entry.ecosystem !== "string" || typeof entry.name !== "string" || !entry.name || typeof entry.version !== "string" || !Array.isArray(entry.kinds) || entry.kinds.some((kind) => !["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"].includes(String(kind))) || typeof entry.productionReachable !== "boolean" || !Array.isArray(entry.users) || entry.users.some((user) => typeof user !== "string") || new Set(entry.users).size !== entry.users.length || identities.has(key)) throw new Error("Dependency policy state has missing users, duplicate identities/users or invalid classifications");
      identities.add(key);
      return entry as UnknownRecord & { users: string[]; kinds: string[] };
    });
    for (const row of owners) {
      const source = row.sourceRoot + "/" + contract.manifestFilename, destination = row.destinationRoot + "/" + contract.manifestFilename;
      const mapping = row.mappings.find((entry) => entry.sourcePath === source), approved = moves.filter((move) => move.sourcePath === source && move.destinationPath === destination);
      const declaration = requireRecord(row.requiredManifestEvidence[contract.dependencyEvidenceField], "Dependency policy declaration");
      const bytes = readFileSync(assertLexicalInputOutsideOpaque(repoRoot, source, "Dependency policy manifest", true));
      if (!mapping || approved.length !== 1 || approved[0]!.sourcePreimage.nodeKind !== "file" || approved[0]!.sourcePreimage.contentHash !== mapping.sourceHash || approved[0]!.sourcePreimage.size !== mapping.sourceSize || sha256(bytes) !== mapping.sourceHash || bytes.byteLength !== mapping.sourceSize || typeof declaration.name !== "string" || typeof declaration.version !== "string") throw new Error("Dependency policy manifest is not one exact source-preimage-proven move");
      const owned = entries.flatMap((entry, index) => entry.users.includes(source) ? [{ entry, index }] : []);
      if (owned.length !== 1) throw new Error("Dependency policy manifest requires exactly one unambiguous approved user entry");
      if (entries.some((entry) => entry.users.some((user) => user.startsWith(row.sourceRoot + "/") && user !== source))) throw new Error("Dependency policy contains an ambiguous source/canonical or unproved package owner");
      const { entry, index } = owned[0]!;
      if (entry.ecosystem !== "rust" || entry.name !== declaration.name || entry.version !== declaration.version || !entry.kinds.includes("production-runtime") || entry.productionReachable !== true) throw new Error("Dependency policy manifest would introduce an unapproved dependency or classification");
      const pointer = `/entries/${index}/users/${entry.users.indexOf(source)}`, coordinate = coordinates.find((coordinate) => coordinate.pointer === pointer && coordinate.value === source);
      const occurrences = (value: unknown): number => typeof value === "string" ? Number(value === source) : Array.isArray(value) ? value.reduce((count, child) => count + occurrences(child), 0) : value && typeof value === "object" ? Object.entries(value).reduce((count, [key, child]) => count + Number(key === source) + occurrences(child), 0) : 0;
      if (!coordinate || occurrences(baseline) !== 1) throw new Error("Dependency policy requires one exact unescaped users token and no undeclared source-coordinate fields");
      manifestSources.add(source);
      tokens.push({ adapter: "json", structuredLocation: `${pointer}@${coordinate.start}`, start: coordinate.start, end: coordinate.end, value: source, targetValues: [source], rewriteKind: "exact-owner-reference", rewriteData: { newValue: destination } });
    }
  } catch (error) { problems.push(error instanceof Error ? error.message : String(error)); }
  return finish();
}

export function buildReferenceEdits(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions, known: ReferencePathIndex, removals: readonly TaxonomyEvidenceRemoval[]): { readonly edits: readonly ReferenceEdit[]; readonly editTargets: ReadonlyMap<string, string>; readonly resultHashes: ReadonlyMap<string, string>; readonly resultSizes: ReadonlyMap<string, number>; readonly unresolved: readonly TaxonomyViolation[] } {
  const incoming = incomingReferenceSnapshot(inventory, taxonomy, options);
  const ownerCatalog = exactOwnedFileCatalog(inventory.repoRoot, taxonomy);
  const ownerConsumers = externalExactOwnedReferenceEntries(inventory, moves, ownerCatalog, taxonomy);
  const facetConsumers = externalArtifactEmptyFacetReferenceEntries(inventory, moves, taxonomy);
  known = referencePathIndex([...known.exact, ...incoming.paths, ...ownerConsumers.map((entry) => entry.sourcePath), ...facetConsumers.map((entry) => entry.sourcePath), ...(ownerCatalog?.entries.flatMap((entry) => [entry.sourcePath, entry.destinationPath]) ?? [])], inventory.repoRoot, incoming.coordinateRoots, undefined, options.cancelFile, new Set(inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath).map((entry) => entry.sourcePath)));
  const moveBySource = new Map(moves.map((move) => [move.sourcePath, move]));
  const destinationBySource = new Map(inventory.entries
    .filter((entry) => entry.sourcePath !== entry.normalizedPath && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0)
    .map((entry) => [entry.sourcePath, entry.normalizedPath]));
  const edits: ReferenceEdit[] = [];
  const editTargets = new Map<string, string>();
  const unresolved: TaxonomyViolation[] = [];
  const packageCatalog = nestedCargoCatalog(inventory.repoRoot, taxonomy.discoverySchema);
  const packages = packageCatalog?.packages.filter((row) => row.mappings.some((mapping) => moves.some((move) => move.sourcePath === mapping.sourcePath && move.destinationPath === mapping.destinationPath))) ?? [];
  const policyStates = new Map<string, DependencyPolicyStateTokens>();
  for (const contract of Object.values(taxonomy.discoverySchema.semanticPolicyStateCoordinateContracts)) if (packages.some((row) => contract.packageIds.some((id) => id === row.id))) {
    try {
      const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, contract.statePath, "Dependency policy state", true), stat = lstatOrNull(absolute);
      if (!stat) continue;
      if (!stat.isFile()) throw new Error("Dependency policy state must be a no-follow regular file");
      const state = dependencyPolicyStateTokens(inventory.repoRoot, contract.statePath, incoming.contents.get(contract.statePath) ?? readFileSync(absolute, "utf8"), packages, moves, taxonomy);
      policyStates.set(contract.statePath, state);
      for (const problem of state.problems) unresolved.push(violation("policy-state-authority-invalid", contract.statePath, problem));
    } catch (error) { unresolved.push(violation("policy-state-authority-invalid", contract.statePath, error instanceof Error ? error.message : String(error))); }
  }
  for (const row of packages) {
    if (row.id === "wgpu-renderer") unresolved.push(violation("nested-cargo-generation-unresolved", row.sourceRoot, "WGPU package generation and transaction acceptance must be complete before projection"));
    for (const consumer of packageCatalog!.referenceConsumers.filter((entry) => entry.packageId === row.id && entry.ownership === "generated")) {
      if (policyStates.get(consumer.path)?.manifestSources.has(row.sourceRoot + "/Cargo.toml")) continue;
      const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, consumer.path, "Nested Cargo generated consumer", true), stat = lstatOrNull(absolute);
      if (!stat) continue;
      const content = stat.isFile() ? readFileSync(absolute, "utf8") : "";
      const transform = packageCatalog!.referenceTokenTransforms[consumer.transformId]!;
      if (!stat.isFile() || !transform || content.replaceAll(transform.destinationToken, "").includes(transform.sourceToken)) unresolved.push(violation("nested-cargo-generated-consumer-unresolved", consumer.path, `Generated ${row.id} consumer needs an exact regeneration authority before projection`));
    }
  }
  const resultHashes = new Map<string, string>();
  const resultSizes = new Map<string, number>();
  const accountedIncoming = new Set<string>();
  const artifactContexts = artifactReferenceProjections(inventory, moves, taxonomy);
  for (const context of artifactContexts) for (const problem of context.authorityProblems) unresolved.push(violation("projection-reference-authority-invalid", context.sourceRoot, `${context.id}: ${problem}`));
  const generatedOwnerSources = new Set(ownerCatalog?.entries.filter((entry) => entry.generatorOwnerId !== null).map((entry) => entry.sourcePath) ?? []);
  for (const removal of removals) if (removal.authority.kind === "nested-cargo-generated-source") {
    const authority = removal.authority, catalogContract = requireTaxonomyValue(taxonomy.schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"],"nested Cargo contract");
    const row = packageCatalog?.packages.find((row) => row.id === authority.packageId), mapping = row?.mappings.find((mapping) => mapping.sourcePath === removal.sourcePath);
    const entry = inventory.entries.find((entry) => entry.sourcePath === removal.sourcePath);
    if (!mapping || mapping.disposition !== "generated" || authority.catalogPath !== catalogContract.authorityCatalogPath || authority.catalogContentHash !== catalogContract.authorityCatalogSha256 || authority.destinationPath !== mapping.destinationPath || removal.preimage.nodeKind !== "file" || removal.preimage.contentHash !== mapping.sourceHash || removal.preimage.size !== mapping.sourceSize || !entry || canonicalJson(inventoryLeafPreimage(entry)) !== canonicalJson(removal.preimage) || !row!.generatedSourceRetirements.some((item) => item.sourcePath === removal.sourcePath && item.destinationPath === authority.destinationPath && item.generatorContractId === authority.generatorContractId && item.sourceMode === removal.preimage.mode)) throw new Error("Generated source reference exclusion lacks exact retirement authority: " + removal.sourcePath);
    generatedOwnerSources.add(removal.sourcePath);
  }
  const candidates = [...new Map([...inventory.entries, ...incoming.entries, ...externalProjectionReferenceEntries(inventory, moves, artifactContexts, taxonomy), ...ownerConsumers, ...facetConsumers].map((entry) => [entry.sourcePath, entry])).values()].filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && !taxonomyFixturePayloadPath(entry.sourcePath, taxonomy.discoverySchema) && !generatedOwnerSources.has(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
  const store = referenceTokenStore(inventory.repoRoot), stamps = inventoryLeafStamps.get(inventory), admit = createLexicalInputSession(), reach = referenceResolutionReach(destinationBySource.keys()), opaqueLeaves = opaqueLeafNames(taxonomy);
  const facetContract = taxonomy.schema.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  const contentGated = new Set<string>([
    ...frozenEvidenceContractIndex(taxonomy).keys(),
    ...(ownerCatalog ? ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader"].flatMap((id) => ownerCatalog.referenceOwners[id] ? [ownerCatalog.referenceOwners[id].ownerPath] : []) : []),
    ...(facetContract?.contractKind === "semantic-facet-primary-file" ? [facetContract.referenceConsumer.path] : []),
    ...packages.flatMap((row) => row.mappings.map((mapping) => mapping.sourcePath)),
    ...policyStates.keys(),
  ]);
  for (const [index, entry] of candidates.entries()) {
    checkCancellation(inventory.repoRoot, options.cancelFile);

    if (isExcluded(entry.sourcePath, taxonomy)) continue;
    const incomingContent = incoming.contents.get(entry.sourcePath);
    let loaded: string | undefined;
    const load = (): string => {
      if (loaded !== undefined) return loaded;
      const text = incomingContent ?? readFileSync(admit(inventory.repoRoot, entry.sourcePath, "Reference preimage", true), "utf8");
      if (sha256(text) !== entry.contentHash || Buffer.byteLength(text) !== entry.size) throw new Error(`Reference preimage changed since inventory: ${entry.sourcePath}`);
      return loaded = text;
    };
    const finalReferencePath = moveBySource.get(entry.sourcePath)?.destinationPath ?? entry.normalizedPath;
    let analysis: ReferenceAnalysis;
    try {
      if (incomingContent !== undefined || !leafStampCurrent(inventory.repoRoot, entry.sourcePath, stamps?.get(entry.sourcePath), admit)) load();
      analysis = storedReferenceAnalysis(store, inventory.repoRoot, entry.sourcePath, entry.contentHash, opaqueLeaves, () => ({ text: load(), matches: true }));
    } catch (error) {
      unresolved.push(violation("reference-preimage-unreadable", entry.sourcePath, error instanceof Error ? error.message : String(error)));
      continue;
    }
    const staying = finalReferencePath === entry.sourcePath && artifactContexts.length === 0, rootedNullLeaves = !staying || ancestorInTargets(entry.sourcePath, destinationBySource);
    const digest = analysis.digest;
    const unsupportedMayResolve = (): boolean => referenceDigestMayResolveInto({ ...digest, rustManifest: false, supported: { leaves: [], nullLeaf: false, python: false } }, reach, rootedNullLeaves);
    const reachable = (token: ReferenceToken): boolean => !staying || rootedNullLeaves || visitReferenceTokenReach(token, known.repoRoot, (leaf, python) => leaf !== null && reach.leaves.has(leaf) || python && reach.python);
    if (staying && !contentGated.has(entry.sourcePath) && (!referenceDigestMayResolveInto(digest, reach, rootedNullLeaves) || !digest.rustManifest && !unsupportedMayResolve() && !referenceDigestResolvesInto(entry.sourcePath, digest, known, destinationBySource, reachable))) {
      report(options.progress, "plan", "references", index + 1, candidates.length, entry.sourcePath);
      continue;
    }
    let content: string;
    try { content = load(); } catch (error) {
      unresolved.push(violation("reference-preimage-unreadable", entry.sourcePath, error instanceof Error ? error.message : String(error)));
      continue;
    }
    const contentBytes = Buffer.from(content);
    const frozenRecord = frozenEvidenceRecord(entry.sourcePath, contentBytes, taxonomy);
    if (frozenRecord?.problem) {
      unresolved.push(violation("frozen-coordinate-evidence-invalid", entry.sourcePath, frozenRecord.problem));
      continue;
    }
    const frozenEvidence = frozenRecord?.coordinates ?? null;
    const fileEdits: ReferenceEdit[] = [];
    const fileTargets = new Map<string, string>();
    const ownedTokens = exactOwnedReferenceTokens(entry.sourcePath, content, ownerCatalog, moves, taxonomy);
    const facetTokens = artifactEmptyFacetReferenceTokens(entry.sourcePath, content, moves, taxonomy);
    const packageTokens = nestedCargoReferenceTokens(entry.sourcePath, content, packages);
    const policyTokens = policyStates.get(entry.sourcePath);
    if (policyTokens?.active && policyTokens.contentHash !== entry.contentHash) { unresolved.push(violation("policy-state-authority-invalid", entry.sourcePath, "Dependency policy preimage changed during planning")); continue; }
    const exactTokens = { tokens: [...ownedTokens.tokens, ...facetTokens.tokens, ...packageTokens.tokens, ...(policyTokens?.tokens ?? [])], problems: [...ownedTokens.problems, ...facetTokens.problems, ...packageTokens.problems] };
    for (const problem of exactTokens.problems) unresolved.push(violation("owner-reference-authority-invalid", entry.sourcePath, problem));
    const genericTokens = (analysis.standalone ? indexedReferenceTokens(entry.sourcePath, content, known, analysis.standalone) : referenceTokens(entry.sourcePath, content, known)).filter((token) => !exactTokens.tokens.some((exact) => exact.start <= token.start && exact.end >= token.end));
    const tokens = [...new Map([...genericTokens, ...exactTokens.tokens].map((token) => [`${token.start}\u0000${token.end}\u0000${token.value}\u0000${(token.targetValues ?? []).join("\u0000")}`, token])).values()].sort((left, right) => left.start - right.start || left.end - right.end || left.structuredLocation.localeCompare(right.structuredLocation));
    const supported = tokens.map((token) => ({ token, target: exactOwnedMarkdownTarget(entry.sourcePath, token, ownerCatalog, moves, taxonomy) ?? (reachable(token) ? resolveReferenceTokenPath(entry.sourcePath, token, known) : null) }));
    for (const { token, target: oldTarget } of supported) {
      if (token.unsupportedReason && token.physicalTargets !== undefined && (finalReferencePath !== entry.sourcePath || token.physicalTargets.some((target) => destinationBySource.has(target)))) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${token.unsupportedReason}`));
        continue;
      }
      if (oldTarget && isFrozenSourceCoordinateToken(entry.sourcePath, contentBytes, token, oldTarget, taxonomy, inventory.repoRoot)) continue;
      const rebase = exactOwnedMarkdownTarget(entry.sourcePath, token, ownerCatalog, moves, taxonomy);
      const destination = oldTarget ? destinationBySource.get(oldTarget) ?? (rebase || finalReferencePath !== entry.sourcePath ? oldTarget : undefined) : undefined;
      const artifactRewrite = artifactStructuralReferenceRewrite(finalReferencePath, token, artifactContexts, taxonomy);
      if (artifactRewrite?.problem) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${artifactRewrite.problem}`));
        continue;
      }
      const artifactProjectionActive = artifactContexts.length > 0 && (token.rewriteKind === "path-prefix" || token.rewriteKind === "artifact-catalog-glob" || token.rewriteKind === "artifact-catalog-prose");
      if (token.unsupportedReason && artifactProjectionActive) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${token.unsupportedReason}`));
        continue;
      }
      if ((!oldTarget || !destination) && artifactRewrite?.newValue === undefined) continue;
      if (frozenEvidence !== null) {
        unresolved.push(violation("frozen-coordinate-evidence-unowned", entry.sourcePath, `${token.structuredLocation} is outside the exact digest-bound coordinate authority`));
        continue;
      }
      if (policyTokens?.active && !policyTokens.tokens.some((approved) => approved.start === token.start && approved.end === token.end && approved.value === token.value)) {
        unresolved.push(violation("policy-state-coordinate-unowned", entry.sourcePath, `${token.structuredLocation} is outside the exact users-only policy authority`));
        continue;
      }
      if (frozenPlanCoordinateAuthority(entry.sourcePath, contentBytes).planLike) {
        unresolved.push(violation("immutable-plan-reference-unowned", entry.sourcePath, `${token.structuredLocation} is not an exact typed coordinate in a canonical digest-verified plan`));
        continue;
      }
      const newValue = requireTaxonomyValue(token.rewriteKind === "exact-owner-reference" ? token.rewriteData!.newValue : artifactRewrite?.newValue ?? (oldTarget && destination ? rewriteReferenceToken(finalReferencePath, entry.sourcePath, token, oldTarget, destination, inventory.repoRoot, inventory.entries) : token.rewriteData!.newValue),"token.rewriteKind === \"exact-owner-reference\" ? token.rewriteData!.newValue : artifactRewrite?.newValue ?? (oldTarget && destination ? rewriteReferenceToken(finalReferencePath, entry.sourcePath, token, oldTarget, destination, inventory.repoRoot, inventory.entries) : token.rewriteData!.newValue)");
      if (oldTarget) accountedIncoming.add(`${oldTarget}\u0000${entry.sourcePath}`);
      if (newValue === token.value) continue;
      const edit = {
        path: finalReferencePath,
        adapter: token.adapter,
        structuredLocation: token.structuredLocation,
        oldValue: token.value,
        newValue,
        preimage: { nodeKind: "file" as const, contentHash: entry.contentHash, mode: entry.mode, size: entry.size },
      } satisfies ReferenceEdit;
      fileEdits.push(edit);
      if (oldTarget) fileTargets.set(referenceEditIdentity(edit), oldTarget);
    }
    const rustSupported = supported.filter(({ token }) => token.adapter === "rust");
    let coverers: Map<string, ReferenceToken[]> | undefined, exactOwnerCoverers: ReferenceToken[] = [];
    for (const candidate of !staying || unsupportedMayResolve() ? analysis.unsupported ?? unsupportedReferenceTokens(content, referenceAdapter(entry.sourcePath)) : []) {
      if (!reachable(candidate)) continue;
      if (candidate.adapter === "rust" && rustSupported.some(({ token }) => rustReferenceInterpretationCovers(token, candidate))) continue;
      const oldTarget = resolveReferenceTokenPath(entry.sourcePath, candidate, known);
      if (!oldTarget || !destinationBySource.has(oldTarget) && finalReferencePath === entry.sourcePath || isFrozenSourceCoordinateToken(entry.sourcePath, contentBytes, candidate, oldTarget, taxonomy, inventory.repoRoot)) continue;
      if (frozenEvidence !== null) {
        unresolved.push(violation("frozen-coordinate-evidence-unowned", entry.sourcePath, `${candidate.structuredLocation} is outside the exact digest-bound coordinate authority`));
        continue;
      }
      if (frozenPlanCoordinateAuthority(entry.sourcePath, contentBytes).planLike) {
        unresolved.push(violation("immutable-plan-reference-unowned", entry.sourcePath, `${candidate.structuredLocation} is not an exact typed coordinate in a canonical digest-verified plan`));
        continue;
      }
      if (!coverers) {
        coverers = new Map();
        exactOwnerCoverers = supported.filter(({ token }) => token.rewriteKind === "exact-owner-reference").map(({ token }) => token);
        for (const { token, target } of supported) if (target !== null) { const rows = coverers.get(target); if (rows) rows.push(token); else coverers.set(target, [token]); }
      }
      const covers = (token: ReferenceToken): boolean => token.start <= candidate.start && token.end >= candidate.end;
      const covered = (coverers.get(oldTarget)?.some(covers) ?? false) || exactOwnerCoverers.some(covers);
      const destination = destinationBySource.get(oldTarget) ?? oldTarget;
      const unchanged = rewriteReferenceValue(finalReferencePath, candidate.value, oldTarget, destination, entry.sourcePath, inventory.repoRoot) === candidate.value;
      if (!covered && !unchanged) unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${candidate.adapter} ${candidate.structuredLocation} contains unsupported path-bearing token ${JSON.stringify(candidate.value)} targeting ${oldTarget}`));
    }
    if (fileEdits.length > 0) {
      const deduplicated = [...new Map(fileEdits.map((edit) => [`${edit.structuredLocation}:${edit.newValue}`, { ...edit, structuredLocation: Buffer.from(edit.structuredLocation, "utf16le").toString("utf16le"), oldValue: Buffer.from(edit.oldValue, "utf16le").toString("utf16le"), newValue: Buffer.from(edit.newValue, "utf16le").toString("utf16le") }])).values()].sort(referenceEditCompare);
      const conflict = referenceEditConflict(content, deduplicated);
      if (conflict) {
        unresolved.push(violation("reference-edit-conflict", entry.sourcePath, conflict));
        continue;
      }
      edits.push(...deduplicated);
      for (const edit of deduplicated) {
        const target = fileTargets.get(referenceEditIdentity(edit));
        if (target) editTargets.set(referenceEditIdentity(edit), target);
      }
      const rendered = applyEditsToContent(content, deduplicated);
      resultHashes.set(finalReferencePath, sha256(rendered));
      resultSizes.set(finalReferencePath, Buffer.byteLength(rendered));
    }
    report(options.progress, "plan", "references", index + 1, candidates.length, entry.sourcePath);
  }
  const entryBySource = inventoryEntriesBySourcePath(inventory);
  for (const move of moves) {
    const entry = entryBySource.get(move.sourcePath);
    if (entry?.fileKind) {
      const role = taxonomy.schema.fileKinds[entry.fileKind]?.role;
      const unaccounted = entry.referencesIn.filter((source) => !accountedIncoming.has(`${entry.sourcePath}\u0000${source}`));
      if (role === "binary" && unaccounted.length > 0) unresolved.push(violation("opaque-reference-rewrite-unresolved", entry.sourcePath, `Binary target has unsupported incoming references from ${unaccounted.join(", ")}`));
      if (role === "generated" && entry.referencesIn.length > 0) unresolved.push(violation("generated-reference-rewrite-unresolved", entry.sourcePath, "Generated target requires an explicit regeneration contract before its incoming references can move"));
    }
  }
  store?.flush();
  const semanticLocationMatches = (actual: string, expected: string): boolean => actual === expected || actual.startsWith(`${expected}:`) || actual.startsWith(`${expected}@`);
  return { edits: edits.sort(referenceEditCompare), editTargets, resultHashes, resultSizes, unresolved: stableViolations(unresolved) };
}

export function referenceEditCompare(a: ReferenceEdit, b: ReferenceEdit): number {
  return generatorPathCompare(a.path, b.path) || generatorPathCompare(a.structuredLocation, b.structuredLocation) || generatorPathCompare(a.oldValue, b.oldValue) || generatorPathCompare(a.newValue, b.newValue);
}
