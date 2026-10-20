import { requireRecord, requireStringArray, requireString } from "@semio-tech/schema-validator-ts";
import { type SemanticDescendantKindNode } from "../🔣️taxonomy/🌳️descendants/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type SemanticProjectionCaptureField } from "../🔣️taxonomy/🧭️path-projection/🟦️.ts";
import { splitLeadingEmoji, emojiFold } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque } from "../📁️input/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { subsetIdForDirectoryName } from "../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { mutationCatalogSourceOwner, mutationOwnerIdentity, mutationOwnerRelativePath } from "../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { mutationCatalogSourceOwnersProblems } from "../../🔍️discovery/👥️mutation-owner-rules/🟦️.ts";
import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { basename, dirname } from "node:path";
import { pathBudgetCodes, pathBudgetFitsWithReserve, pathBudgetStatement, pathUtf16Units, type PathBudgetPolicy } from "../../🔍️discovery/📏️path-budget/🟦️.ts";
import { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";
import { foldMutationCaseName, mutationLeafCaseTableProblems, parseMutationLeafCaseTable } from "../../🔍️discovery/🗂️mutation-leaf-case-table/🟦️.ts";
import { type SemanticKindDescendantContract, type TaxonomyCapturedSourceRead, type TaxonomyInventoryEntry, type TaxonomyViolation, WINDOWS_RESERVED, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, isExcluded, violation } from "../🏷️file-kind-matching/🟦️.ts";
import type { CandidatePath, MutableInventoryEntry } from "../📸️repository-capture/🟦️.ts";

/** 🪟️ Evaluates checkout-hostile path strings without materializing filesystem nodes. */
export function taxonomyPlatformPathViolationCodes(path: string, policy: PathBudgetPolicy): readonly string[] {
  const rows: string[] = [...pathBudgetCodes(path, policy)];
  for (const segment of path.replaceAll("\\", "/").split("/")) {
    if (WINDOWS_RESERVED.test(segment)) rows.push("windows-reserved-name");
    if (/[. ]$/u.test(segment)) rows.push("trailing-dot-or-space");
  }
  return [...new Set(rows)];
}

export function pathPolicyViolations(path: string, taxonomy: LoadedTaxonomy): readonly TaxonomyViolation[] {
  const rows: TaxonomyViolation[] = [];
  for (const code of pathBudgetCodes(path, taxonomy.schema.collisionPolicy)) rows.push(violation(code, path, code === "path-too-long" ? `Path has ${pathUtf16Units(path)} UTF-16 units; the budget is ${taxonomy.schema.collisionPolicy.maxPathUtf16Units}` : `A path component exceeds ${taxonomy.schema.collisionPolicy.maxComponentUtf8Bytes} UTF-8 bytes`));
  for (const segment of path.split("/")) {
    if (taxonomy.schema.collisionPolicy.rejectWindowsReservedNames && WINDOWS_RESERVED.test(segment)) rows.push(violation("windows-reserved-name", path, `Path segment is Windows-reserved: ${segment}`));
    if (taxonomy.schema.collisionPolicy.rejectTrailingDotsAndSpaces && /[. ]$/.test(segment)) rows.push(violation("trailing-dot-or-space", path, `Path segment ends with a dot or space: ${segment}`));
  }
  return rows;
}

export function sourceTreeDigest(entries: readonly TaxonomyInventoryEntry[]): string {
  return sha256(canonicalJson(entries.map((entry) => ({ sourcePath: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, symlinkTarget: entry.symlinkTarget }))));
}

export function ancestorDirectoryKindIds(path: string, kinds: ReadonlyMap<string, string>): readonly string[] {
  const rows: string[] = [];
  let current = dirname(path);
  while (current && current !== ".") {
    const kindId = kinds.get(current);
    if (kindId) rows.push(kindId);
    current = dirname(current);
  }
  return rows;
}

/** 🪞️ The schema-side path a subset's `🧪️tests/🧫️fixtures/🧬️mutations` mirror stands for, so a registered domain owner's
 * `<domain>/<verb>` layout resolves identically on both sides; any other path is returned unchanged. */
export function mutationFixtureMirrorOwnerPath(path: string, taxonomy: LoadedTaxonomy): string {
  const contract = taxonomy.discoverySchema;
  const mirror = `/${contract.testsDirName}/${contract.testFixturesDirName}/${contract.schemaScopeOwnerLevels.mutationsFacetDirName}/`;
  let at = path.indexOf(mirror);
  while (at >= 0 && path.slice(0, at).endsWith(`/${contract.schemaScopeOwnerLevels.moduleMemberDirName}`)) at = path.indexOf(mirror, at + mirror.length);
  return at < 0 ? path : `${path.slice(0, at)}/${contract.mutationPayloadSchemaLocation.directoryName}/${contract.schemaScopeOwnerLevels.mutationsFacetDirName}/${path.slice(at + mirror.length)}`;
}

/** 🧬️ Mutation leaf owners proven by their own canonical descriptor: the configured schema version naming exactly this
 * directory as its `owner`, with a semantic identity. This structural authority replaces per-name leaf registries; the
 * identity's agreement with the directory stays governed by the mutation payload authority law. A
 * `🧪️tests/🧫️fixtures/🧬️mutations/<leaf>` mirror is proven through its schema-side owner, whose descriptor is read from disk when
 * the inventory scope excludes it. A `🧪️tests/🧫️fixtures/<leaf>` or `🧪️tests/🧫️fixtures/<leaf>-<scenario>` vector directory (leaf names
 * compared without U+FE0F) is proven through a sibling `🧬️schema/🧬️mutations/<leaf>` and returned itself, and so is each
 * of its scenario children holding a fixture bundle (`🦠️mutation`, `📸️snapshot`, `🔺️diff`); the set holds proven leaf
 * owners plus the vector and scenario directories they own. */
export function provenMutationLeafOwners(repoRoot: string, directories: readonly string[], admitted: ReadonlyMap<string, CandidatePath>, sourceRead: TaxonomyCapturedSourceRead, taxonomy: LoadedTaxonomy): ReadonlySet<string> {
  const contract = taxonomy.discoverySchema.mutationPayloadSchemaAuthority, descriptorKind = taxonomy.discoverySchema.fileKinds[contract.descriptorFileKindId]!;
  const descriptorName = `${descriptorKind.emoji}${descriptorKind.extensionChains[0]}`, proven = new Set<string>(), visited = new Set<string>();
  const prove = (owner: string): void => {
    const descriptorPath = `${owner}/${descriptorName}`, admittedDescriptor = admitted.has(descriptorPath);
    if (visited.has(owner) || !owner.includes("/🧬️mutations/")) return;
    visited.add(owner);
    if (isExcluded(descriptorPath, taxonomy) || !admittedDescriptor && !existsSync(absolutePath(repoRoot, descriptorPath))) return;
    let descriptor: unknown;
    try {
      const bytes = admittedDescriptor ? sourceRead(absolutePath(repoRoot, descriptorPath)) : readFileSync(assertLexicalInputOutsideOpaque(repoRoot, descriptorPath, "Mutation leaf descriptor", true));
      descriptor = JSON.parse(Buffer.from(bytes).toString("utf8"));
    } catch {
      return;
    }
    if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor)) return;
    const fields = descriptor as Record<string, unknown>;
    if (fields.schemaVersion === contract.descriptorSchemaVersion && fields[contract.descriptorOwnerField] === owner && typeof fields[contract.descriptorIdentityField] === "string" && fields[contract.descriptorIdentityField] !== "") proven.add(owner);
  };
  for (const path of admitted.keys()) if (basename(path) === descriptorName) prove(dirname(path));
  const leafNamesByRoot = new Map<string, readonly string[]>();
  let childNamesByParent: Map<string, string[]> | undefined;
  const leafNames = (root: string): readonly string[] => {
    const cached = leafNamesByRoot.get(root);
    if (cached) return cached;
    if (!childNamesByParent) {
      childNamesByParent = new Map();
      for (const directory of directories) {
        const parent = dirname(directory), siblings = childNamesByParent.get(parent);
        if (siblings) siblings.push(basename(directory));
        else childNamesByParent.set(parent, [basename(directory)]);
      }
    }
    const names = new Set(childNamesByParent.get(root) ?? []);
    if (!isExcluded(root, taxonomy) && existsSync(absolutePath(repoRoot, root))) for (const entry of readdirSync(absolutePath(repoRoot, root), { withFileTypes: true })) if (entry.isDirectory()) names.add(entry.name.normalize("NFC"));
    const sorted = [...names].sort();
    leafNamesByRoot.set(root, sorted);
    return sorted;
  };
  for (const directory of directories) {
    const owned = mutationFixtureMirrorOwnerPath(directory, taxonomy);
    if (owned !== directory) prove(owned);
    if (basename(dirname(directory)) !== taxonomy.discoverySchema.testFixturesDirName || basename(dirname(dirname(directory))) !== taxonomy.discoverySchema.testsDirName || basename(dirname(dirname(dirname(directory)))) === taxonomy.discoverySchema.schemaScopeOwnerLevels.moduleMemberDirName) continue;
    const root = `${dirname(dirname(dirname(directory)))}/${taxonomy.discoverySchema.mutationPayloadSchemaLocation.directoryName}/🧬️mutations`, vector = emojiFold(basename(directory));
    for (const leaf of leafNames(root)) {
      const folded = emojiFold(leaf);
      if (vector !== folded && !vector.startsWith(`${folded}-`)) continue;
      prove(`${root}/${leaf}`);
      if (proven.has(`${root}/${leaf}`)) proven.add(directory);
    }
  }
  const known = new Set(directories);
  for (const directory of directories) if (proven.has(dirname(directory)) && basename(dirname(dirname(directory))) === taxonomy.discoverySchema.testFixturesDirName && ["🦠️mutation", "📸️snapshot", "🔺️diff"].every((facet) => known.has(`${directory}/${facet}`))) proven.add(directory);
  return proven;
}

export function mutationDomainOwnerLocation(path: string, taxonomy: LoadedTaxonomy): { root: string; relativePath: string; identity: string } | null {
  for (const root of Object.keys(taxonomy.discoverySchema.mutationDomainOwners)) {
    if (!path.startsWith(`${root}/`)) continue;
    const relativePath = path.slice(root.length + 1).split("/").slice(0, 2).join("/");
    const identity = mutationOwnerIdentity(root, relativePath, taxonomy.discoverySchema);
    if (identity !== null) return { root, relativePath, identity };
  }
  return null;
}

interface MutationProjectionSource {
  readonly artifactRoot: string;
  readonly artifactId: string;
  readonly standardVersion: string;
  readonly standardDirectoryName: string;
  readonly subsetId: string;
  readonly subsetDirectoryName: string;
  readonly mutationId: string;
  readonly mutationDirectoryName: string;
  readonly sourceScenarioId: string;
  readonly sourceScenarioDirectoryName: string;
  readonly subsetRoot: string;
  readonly mutationRoot: string;
  readonly scenarioRoot: string;
}

interface MutationProjectionVector {
  readonly mutationId: string;
  readonly sourceMutationDirectoryName: string;
  readonly mutationDirectoryName: string;
  readonly scenarioId: string;
  readonly scenarioDirectoryName: string;
  readonly catalogOwner?: string;
  readonly catalogPath?: string;
}

function projectionDirectorySlug(name: string, kindId: string, taxonomy: LoadedTaxonomy): string | null {
  const kind = taxonomy.schema.semanticDirectoryKinds[kindId];
  if (!kind) return null;
  const leading = splitLeadingEmoji(name.normalize("NFC"));
  if (emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest)) return null;
  return leading.rest;
}

/** 🌳️ One append-only entry map's keys in code-unit order (every `<root>/` prefix is one contiguous range) with their insertion ordinals; entry maps only ever gain keys, so an unchanged size means unchanged keys. */
interface EntrySubtreeIndex {
  readonly size: number;
  readonly paths: readonly string[];
  readonly ordinals: ReadonlyMap<string, number>;
}

const entrySubtreeIndexes = new WeakMap<ReadonlyMap<string, unknown>, EntrySubtreeIndex>();

/** 🌲️ The entries at `root` and below it, in the map's insertion order, found by two binary searches instead of a scan of every entry. */
export function entrySubtree<T>(entries: ReadonlyMap<string, T>, root: string): T[] {
  let index = entrySubtreeIndexes.get(entries);
  if (!index || index.size !== entries.size) {
    const keys = [...entries.keys()];
    index = { size: keys.length, paths: [...keys].sort(), ordinals: new Map(keys.map((key, ordinal) => [key, ordinal])) };
    entrySubtreeIndexes.set(entries, index);
  }
  const prefix = `${root}/`, paths = index.paths, found = entries.has(root) ? [root] : [];
  let low = 0, high = paths.length;
  while (low < high) {
    const middle = (low + high) >>> 1;
    if (paths[middle]! < prefix) low = middle + 1;
    else high = middle;
  }
  for (let at = low; at < paths.length && paths[at]!.startsWith(prefix); at++) found.push(paths[at]!);
  const ordinals = index.ordinals;
  return found.sort((left, right) => ordinals.get(left)! - ordinals.get(right)!).map((path) => entries.get(path)!);
}

const memberNameFolds = new WeakMap<object, ReadonlyMap<string, readonly string[]>>();

/** 🔤️ One member registry's names grouped by emoji fold in declaration order, built once per registry so a fold lookup is O(1) instead of a scan of every member name. */
function memberNamesByFold(registry: { readonly memberNames: readonly string[] }): ReadonlyMap<string, readonly string[]> {
  const cached = memberNameFolds.get(registry);
  if (cached) return cached;
  const folds = new Map<string, string[]>();
  for (const name of registry.memberNames) {
    const key = emojiFold(name), rows = folds.get(key);
    if (rows) rows.push(name);
    else folds.set(key, [name]);
  }
  memberNameFolds.set(registry, folds);
  return folds;
}

function projectionSourceAt(
  path: string,
  scope: string | undefined,
  entries: ReadonlyMap<string, MutableInventoryEntry>,
  kinds: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): MutationProjectionSource | null {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const contract = ids;
  const domainOwner = mutationDomainOwnerLocation(path, taxonomy);
  if (!domainOwner && Object.keys(taxonomy.discoverySchema.mutationDomainOwners).some((root) => path.startsWith(`${root}/`))) return null;
  const physicalSegments = path.split("/"), domainIndex = domainOwner ? domainOwner.root.split("/").length : null;
  const segments = physicalSegments.filter((_segment, index) => index !== domainIndex);
  const physicalPrefix = (length: number): string => physicalSegments.slice(0, length + (domainIndex !== null && length > domainIndex ? 1 : 0)).join("/");
  if (segments.length <= contract.implementationSegments.length) return null;
  const start = segments.length - contract.implementationSegments.length;
  const artifactRoot = physicalPrefix(start);
  const ownerRegistry = requireTaxonomyValue(taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId],"source owner registry");
  const ownerMatches = memberNamesByFold(ownerRegistry).get(emojiFold(basename(artifactRoot))) ?? [];
  if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`)))) return null;
  const captures = new Map<SemanticProjectionCaptureField, string>();
  for (const [index, segment] of contract.implementationSegments.entries()) {

    const currentPath = physicalPrefix(start + index + 1);
    const current = entries.get(currentPath);
    if (!current || current.nodeKind !== "directory") return null;
    const canonicalName = basename(current.normalizedPath);
    if ("literal" in segment) {
      if (!("kindId" in segment) || canonicalName !== segment.literal || kinds.get(currentPath) !== segment.kindId) return null;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      if (segment.projectedMemberKindId !== ids.projectedMemberKindId) return null;
      const sourceName = basename(current.sourcePath).normalize("NFC");
      const slug = splitLeadingEmoji(sourceName).rest;
      if (!slug) return null;
      captures.set(segment.capture, domainOwner?.identity ?? slug);
      continue;
    }
    const sourceName = basename(current.sourcePath).normalize("NFC");
    if (segment.capture === "scenarioId" && pathEmojiStatuteFindings([{ path: sourceName, nodeKind: "directory" }], taxonomy.discoverySchema.pathEmojiPolicy.genericEmojiIdentities).length === 0 && new RegExp(requireTaxonomyValue(taxonomy.schema.semanticDirectoryKinds[segment.kindId],"directory kind").slugPattern, "u").test(splitLeadingEmoji(sourceName).rest)) {
      captures.set(segment.capture, splitLeadingEmoji(sourceName).rest);
      continue;
    }
    if ((segment.capture === "standardVersion" || segment.capture === "subsetId") && kinds.get(currentPath) !== segment.kindId && taxonomy.discoverySchema.subsetDirectoryOverrides?.[dirname(currentPath)] === undefined && splitLeadingEmoji(sourceName).emoji !== "" && pathEmojiStatuteFindings([{ path: sourceName, nodeKind: "directory" }], taxonomy.discoverySchema.pathEmojiPolicy.genericEmojiIdentities).length === 0 && new RegExp(requireTaxonomyValue(taxonomy.schema.semanticDirectoryKinds[segment.kindId],"directory kind").slugPattern, "u").test(splitLeadingEmoji(sourceName).rest)) {
      captures.set(segment.capture, splitLeadingEmoji(sourceName).rest);
      continue;
    }
    const contextualUnprefixed = segment.capture === "scenarioId" && !splitLeadingEmoji(sourceName).emoji && new RegExp(requireTaxonomyValue(taxonomy.schema.semanticDirectoryKinds[segment.kindId],"directory kind").slugPattern, "u").test(sourceName);
    if (kinds.get(currentPath) !== segment.kindId && !contextualUnprefixed) return null;
    const overriddenSubset = segment.capture === "subsetId" && taxonomy.discoverySchema.subsetDirectoryOverrides?.[dirname(currentPath)] !== undefined;
    const slug = contextualUnprefixed ? sourceName : overriddenSubset ? subsetIdForDirectoryName(dirname(currentPath), canonicalName, taxonomy.discoverySchema) : projectionDirectorySlug(canonicalName, segment.kindId, taxonomy);
    if (!slug) return null;
    captures.set(segment.capture, slug);
  }
  const standardVersion = captures.get("standardVersion");
  const subsetId = captures.get("subsetId");
  const mutationId = captures.get("mutationId");
  const sourceScenarioId = captures.get("scenarioId");
  if (!standardVersion || !subsetId || !mutationId || !sourceScenarioId) return null;
  const source = contract.implementationSegments.map((_segment, index) => physicalPrefix(start + index + 1));
  return {
    artifactRoot,
    artifactId: splitLeadingEmoji(basename(artifactRoot)).rest || basename(artifactRoot),
    standardVersion,
    standardDirectoryName: basename(requireTaxonomyValue(source[1],"source[1]")),
    subsetId,
    subsetDirectoryName: basename(requireTaxonomyValue(source[3],"source[3]")),
    mutationId,
    mutationDirectoryName: basename(requireTaxonomyValue(source[6],"source[6]")),
    sourceScenarioId,
    sourceScenarioDirectoryName: basename(requireTaxonomyValue(source[8],"source[8]")),
    subsetRoot: requireTaxonomyValue(source[3],"projection root"),
    mutationRoot: requireTaxonomyValue(source[6],"projection root"),
    scenarioRoot: requireTaxonomyValue(source[8],"projection root"),
  };
}

function projectionCatalogVectors(path: string, source: Pick<MutationProjectionSource, "standardDirectoryName" | "subsetDirectoryName">): { readonly vectors: readonly MutationProjectionVector[]; readonly error?: string } {
  let root: Record<string, unknown>;
  try {
    root = requireRecord(JSON.parse(readFileSync(path, "utf8")), "mutation projection catalog");
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  if (!Array.isArray(root.mutationCatalogs)) return { vectors: [], error: "mutationCatalogs must be an array" };
  const vectors: MutationProjectionVector[] = [];
  const seenSource = new Set<string>();
  const seenCanonical = new Set<string>();
  try {
    for (let catalogIndex = 0; catalogIndex < root.mutationCatalogs.length; catalogIndex++) {
      const catalog = requireRecord(root.mutationCatalogs[catalogIndex], `mutationCatalogs[${catalogIndex}]`);
      requireString(catalog.id, `mutationCatalogs[${catalogIndex}].id`);
      requireString(catalog.capability, `mutationCatalogs[${catalogIndex}].capability`);
      if (requireString(catalog.standardDirectoryName, `mutationCatalogs[${catalogIndex}].standardDirectoryName`) !== source.standardDirectoryName || requireString(catalog.subsetDirectoryName, `mutationCatalogs[${catalogIndex}].subsetDirectoryName`) !== source.subsetDirectoryName) throw new Error(`mutationCatalogs[${catalogIndex}] owner identity does not match its physical standard/subset`);
      requireStringArray(catalog.kinds, `mutationCatalogs[${catalogIndex}].kinds`);
      if (!Array.isArray(catalog.vectors)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors must be an array`);
      for (let vectorIndex = 0; vectorIndex < catalog.vectors.length; vectorIndex++) {
        const vector = requireRecord(catalog.vectors[vectorIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}]`);
        const mutationId = requireString(vector.mutationId, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationId`);
        const sourceMutationDirectoryName = requireString(vector.sourceMutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName`);
        if (sourceMutationDirectoryName !== sourceMutationDirectoryName.normalize("NFC") || sourceMutationDirectoryName.includes("/")) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName is not one exact NFC basename`);
        const mutationDirectoryName = requireString(vector.mutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationDirectoryName`).normalize("NFC");
        if (!Array.isArray(vector.scenarios)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}] has an invalid physical mutation identity`);
        const scenarioIds = new Set<string>();
        const scenarioDirectories = new Set<string>();
        for (let scenarioIndex = 0; scenarioIndex < vector.scenarios.length; scenarioIndex++) {
          const scenario = requireRecord(vector.scenarios[scenarioIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}]`);
          const scenarioId = requireString(scenario.id, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].id`);
          if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(scenarioId)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}] has an invalid logical scenario identity`);
          const physicalScenarioName = requireString(scenario.directoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName`);
          const scenarioDirectoryName = physicalScenarioName.normalize("NFC");
          const physicalScenarioId = splitLeadingEmoji(scenarioDirectoryName).rest;
          if (physicalScenarioName !== scenarioDirectoryName || physicalScenarioId === scenarioDirectoryName || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(physicalScenarioId) || /[\\/]/u.test(scenarioDirectoryName)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}] has an invalid physical scenario identity`);
          const scenarioDirectoryKey = scenarioDirectoryName.replaceAll("\uFE0F", "").toLocaleLowerCase("und");
          if (scenarioIds.has(scenarioId) || scenarioDirectories.has(scenarioDirectoryKey)) throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}] duplicates a scenario identity`);
          scenarioIds.add(scenarioId);
          scenarioDirectories.add(scenarioDirectoryKey);
          const sourceKey = `${mutationId}\u0000${sourceMutationDirectoryName}\u0000${scenarioId}`;
          const canonicalKey = `${mutationId}\u0000${mutationDirectoryName}\u0000${scenarioId}`;
          if (seenSource.has(sourceKey) || seenCanonical.has(canonicalKey)) throw new Error(`Duplicate physical vector identity ${sourceKey.replaceAll("\u0000", "/")}`);
          seenSource.add(sourceKey);
          seenCanonical.add(canonicalKey);
          vectors.push({ mutationId, sourceMutationDirectoryName, mutationDirectoryName, scenarioId, scenarioDirectoryName });
        }
      }
    }
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  return { vectors: vectors.sort((left, right) => left.sourceMutationDirectoryName.localeCompare(right.sourceMutationDirectoryName) || left.scenarioDirectoryName.localeCompare(right.scenarioDirectoryName)) };
}

function projectionCatalogEntryForSubset(entries: ReadonlyMap<string, MutableInventoryEntry>, subsetRoot: string, taxonomy: LoadedTaxonomy): MutableInventoryEntry | null {
  const oracleRoot = `${subsetRoot}/${taxonomy.discoverySchema.testOraclesDirName}`;
  const candidates = entrySubtree(entries, oracleRoot).filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && dirname(entry.sourcePath) === oracleRoot && basename(entry.normalizedPath) === "🔣️.json");
  return candidates.length === 1 ? requireTaxonomyValue(candidates[0],"candidates[0]") : null;
}

function projectionCatalogsForMutationSource(repoRoot: string, entries: ReadonlyMap<string, MutableInventoryEntry>, sourceOwner: string, taxonomy: LoadedTaxonomy): { owner: string; path: string; entry: MutableInventoryEntry | null; vectors: readonly MutationProjectionVector[]; error?: string }[] {
  const problems = mutationCatalogSourceOwnersProblems(taxonomy.discoverySchema);
  if (problems.length > 0) throw new Error(problems.join("\n"));
  const owners = [sourceOwner, ...Object.entries(taxonomy.discoverySchema.mutationCatalogSourceOwners).filter(([, source]) => source === sourceOwner).map(([owner]) => owner)];
  return owners.map((owner) => {
    const entry = projectionCatalogEntryForSubset(entries, owner, taxonomy);
    const path = entry?.sourcePath ?? `${owner}/${taxonomy.discoverySchema.testOraclesDirName}/🔣️.json`;
    const profile = { standardDirectoryName: owner.split("/").at(-3)!, subsetDirectoryName: basename(owner) };
    const catalog = entry || existsSync(absolutePath(repoRoot, path)) ? projectionCatalogVectors(absolutePath(repoRoot, path), profile) : { vectors: [], error: `catalog is missing at ${path}` };
    return { owner, path, entry, ...catalog, vectors: catalog.vectors.map((vector) => ({ ...vector, catalogOwner: owner, catalogPath: path })) };
  });
}

function mutationDescendantContract(taxonomy: LoadedTaxonomy, role: "implementation" | "fixture"): SemanticKindDescendantContract {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const contract = taxonomy.schema.semanticDescendantContracts[role === "implementation" ? ids.implementationDescendantContractId : ids.fixtureDescendantContractId];
  if (!contract || "contractKind" in contract || [...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].some((node) => !("kindId" in node))) throw new Error(`Mutation ${role} must reference one physical-kind exact descendant contract`);
  return contract as SemanticKindDescendantContract;
}

function projectionDescendantPath(node: SemanticDescendantKindNode, taxonomy: LoadedTaxonomy): string {
  const segments = node.pathSegments.map((segment) => segment.literal);
  if (node.nodeType === "file") {
    const kind = taxonomy.schema.fileKinds[node.kindId];
    if (!kind || kind.extensionChains.length !== 1) throw new Error(`Projection descendant kind ${node.kindId} is not a single physical leaf`);
    segments.push(`${kind.emoji}${kind.extensionChains[0]}`.normalize("NFC"));
  }
  return segments.join("/");
}

function canonicalProjectedMemberName(name: string, taxonomy: LoadedTaxonomy): string | null {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const sourceKind = requireTaxonomyValue(taxonomy.schema.semanticProjectedMemberKinds[ids.projectedMemberKindId],"projected member kind").sourceMemberKindId;
  const matches = memberNamesByFold(requireTaxonomyValue(taxonomy.schema.semanticDirectoryMemberKinds[sourceKind],"source member kind")).get(emojiFold(name.normalize("NFC"))) ?? [];
  return matches.length === 1 ? matches[0] ?? null : null;
}

function canonicalProjectedMutationOwner(name: string, identity: string, subsetRoot: string, taxonomy: LoadedTaxonomy): string | null {
  const sourceOwner = mutationCatalogSourceOwner(subsetRoot, taxonomy.discoverySchema);
  if (sourceOwner === null) return null;
  const root = `${sourceOwner}/🧬️schema/🧬️mutations`;
  if (!Object.hasOwn(taxonomy.discoverySchema.mutationDomainOwners, root)) {
    const normalized = name.normalize("NFC");
    const physicalId = splitLeadingEmoji(normalized).rest;
    return name === normalized && physicalId !== normalized && physicalId === identity && !/[\\/]/u.test(normalized) ? normalized : null;
  }
  const owner = mutationOwnerRelativePath(root, identity, taxonomy.discoverySchema);
  return owner && basename(owner) === name ? owner : null;
}

/** 🥒️ Whether a committed fixture bundle is covered without a per-case implementation: its subset ships the fixture runner that discovers every case at run time, an exhaustive mutation case declares the bundle in a feature step, or the case table of its leaf lists it. All three sources are the `mutationCatalogProjection.fixtureCoverage` schema. */
function fixtureCoverageOf(repoRoot: string, entries: ReadonlyMap<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy): (fixtureDirectory: string) => boolean {
  const coverage = taxonomy.schema.mutationCatalogProjection.fixtureCoverage, discovery = taxonomy.discoverySchema;
  const fixtureMarker = `/${discovery.testsDirName}/${discovery.testFixturesDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/`;
  const featureKind = requireTaxonomyValue(discovery.fileKinds[coverage.exhaustiveCase.featureFileKindId], "exhaustive case feature file kind");
  const featureName = `${featureKind.emoji}${featureKind.extensionChains[0]}`.normalize("NFC");
  const policy = { stepUriPrefix: coverage.exhaustiveCase.stepUriPrefix, bundleFacetDirectoryNames: coverage.exhaustiveCase.bundleFacetDirectoryNames };
  const tableKind = requireTaxonomyValue(discovery.fileKinds[coverage.leafCaseTable.fileKindId], "leaf case table file kind");
  const tableName = `${tableKind.emoji}${tableKind.extensionChains[0]}`.normalize("NFC");
  const leafTables = new Map<string, ReadonlySet<string> | null>();
  const leafTableCases = (subsetRoot: string, owner: string): ReadonlySet<string> | null => {
    const path = `${subsetRoot}/${discovery.schemaScopeOwnerLevels.facetDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/${owner}/${discovery.testsDirName}/${tableName}`;
    if (leafTables.has(path)) return leafTables.get(path)!;
    let found: ReadonlySet<string> | null = null;
    if (existsSync(absolutePath(repoRoot, path))) {
      const table = parseMutationLeafCaseTable(readFileSync(absolutePath(repoRoot, path), "utf8"), coverage.leafCaseTable);
      if ("cases" in table) found = new Set(table.cases.map(foldMutationCaseName));
    }
    leafTables.set(path, found);
    return found;
  };
  const subsets = new Map<string, Readonly<{ runner: boolean; declared: ReadonlySet<string> }>>();
  const subsetCoverage = (subsetRoot: string): Readonly<{ runner: boolean; declared: ReadonlySet<string> }> => {
    const cached = subsets.get(subsetRoot);
    if (cached) return cached;
    const runner = entries.get(`${subsetRoot}/${coverage.fixtureRunner.relativePath}`)?.nodeKind === "file", declared = new Set<string>();
    const testsRoot = `${subsetRoot}/${discovery.testsDirName}`;
    for (const entry of entrySubtree(entries, testsRoot)) {
      if (entry.nodeKind !== "file" || basename(entry.sourcePath).normalize("NFC") !== featureName || entry.sourcePath.startsWith(`${testsRoot}/${discovery.testFixturesDirName}/`)) continue;
      try {
        for (const bundle of mutationFixturesDeclaredByFeature(readFileSync(absolutePath(repoRoot, entry.sourcePath), "utf8"), policy)) declared.add(bundle);
      } catch {
        continue;
      }
    }
    const found = { runner, declared };
    subsets.set(subsetRoot, found);
    return found;
  };
  return (fixtureDirectory) => {
    const at = fixtureDirectory.indexOf(fixtureMarker);
    if (at < 0) return false;
    const subsetRoot = fixtureDirectory.slice(0, at), bundle = fixtureDirectory.slice(at + fixtureMarker.length), found = subsetCoverage(subsetRoot), split = bundle.lastIndexOf("/");
    return found.runner || found.declared.has(foldMutationCaseName(bundle)) || split > 0 && leafTableCases(subsetRoot, bundle.slice(0, split))?.has(foldMutationCaseName(bundle.slice(split + 1))) === true;
  };
}

function projectionBundleProblem(source: Pick<MutationProjectionSource, "scenarioRoot">, entries: ReadonlyMap<string, MutableInventoryEntry>, kinds: ReadonlyMap<string, string>, contract: SemanticKindDescendantContract, taxonomy: LoadedTaxonomy): string | null {
  const root = entries.get(source.scenarioRoot);
  if (!root) return "scenario root is absent";
  const actual = entrySubtree(entries, source.scenarioRoot);
  if (actual.length !== contract.realizedNodeCount) return `bundle has ${actual.length} nodes, expected ${contract.realizedNodeCount}`;
  if (actual.some((entry) => entry.nodeKind === "symlink")) return "bundle contains a symlink";
  const byKey = new Map<string, MutableInventoryEntry>();
  for (const entry of actual) {
    const relativePath = entry.normalizedPath === root.normalizedPath ? "" : entry.normalizedPath.startsWith(`${root.normalizedPath}/`) ? entry.normalizedPath.slice(root.normalizedPath.length + 1) : null;
    if (relativePath === null) return `bundle node normalizes outside its scenario: ${entry.sourcePath}`;
    const key = `${entry.nodeKind}\u0000${relativePath}`;
    if (byKey.has(key)) return `bundle normalization duplicates ${relativePath}`;
    byKey.set(key, entry);
  }
  const matches = (node: SemanticDescendantKindNode): boolean => {
    const entry = byKey.get(`${node.nodeType}\u0000${projectionDescendantPath(node, taxonomy)}`);
    if (!entry) return false;
    return node.nodeType === "file" ? entry.fileKind === node.kindId : node.pathSegments.length === 0 && entry.sourcePath === source.scenarioRoot && node.kindId === contract.rootDirectoryKindId || kinds.get(entry.sourcePath) === node.kindId;
  };
  const missing = contract.requiredNodes.filter((node) => !matches(node));
  if (missing.length > 0) return `bundle is missing ${projectionDescendantPath(requireTaxonomyValue(missing[0],"missing[0]"), taxonomy) || "scenario root"}`;
  for (const alternative of contract.exclusiveAlternatives) if (alternative.nodes.filter(matches).length !== 1) return `bundle must realize exactly one ${alternative.id} alternative`;
  const allowed = new Set([...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].filter(matches).map((node) => `${node.nodeType}\u0000${projectionDescendantPath(node, taxonomy)}`));
  const extra = [...byKey.keys()].find((key) => !allowed.has(key));
  return extra ? `bundle contains unregistered node ${extra.slice(extra.indexOf("\u0000") + 1)}` : null;
}

function setProjectedPath(entry: MutableInventoryEntry, destination: string, taxonomy: LoadedTaxonomy, authority: "exact-descendant" | "parent-rebase" = "exact-descendant"): void {
  entry.normalizedPath = destination.normalize("NFC");
  const superseded = new Set(["path-too-long", "path-component-too-long", "windows-reserved-name", "trailing-dot-or-space", ...(authority === "exact-descendant" ? ["directory-kind-ambiguous", "directory-kind-unresolved", "file-kind-ambiguous", "file-kind-unresolved", "semantic-stem-ambiguous", "semantic-stem-unresolved"] : [])]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}

export function normalizeMutationCasePairs(
  repoRoot: string,
  scope: string | undefined,
  entries: Map<string, MutableInventoryEntry>,
  kinds: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): void {
  const ids = taxonomy.schema.mutationCatalogProjection, covered = fixtureCoverageOf(repoRoot, entries, taxonomy);
  const implementationContract = mutationDescendantContract(taxonomy, "implementation");
  const fixtureContract = mutationDescendantContract(taxonomy, "fixture");
  const sources = [...entries.values()].filter((entry) => entry.nodeKind === "directory").map((entry) => projectionSourceAt(entry.sourcePath, scope, entries, kinds, taxonomy)).filter((entry): entry is MutationProjectionSource => entry !== null).sort((left, right) => left.scenarioRoot.localeCompare(right.scenarioRoot));
  const bySubset = new Map<string, MutationProjectionSource[]>();
  for (const source of sources) bySubset.set(source.subsetRoot, [...(bySubset.get(source.subsetRoot) ?? []), source]);
  for (const [subsetRoot, subsetSources] of [...bySubset.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    const catalogs = projectionCatalogsForMutationSource(repoRoot, entries, subsetRoot, taxonomy);
    const invalid = catalogs.filter((catalog) => catalog.error);
    if (invalid.length > 0) {
      for (const source of subsetSources) for (const catalog of invalid) entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-invalid", catalog.path, catalog.error!));
      continue;
    }
    const catalog = { vectors: catalogs.flatMap((entry) => entry.vectors) };
    const vectorsByMutation = new Map<string, MutationProjectionVector[]>();
    for (const vector of catalog.vectors) {
      const key = vector.mutationId;
      vectorsByMutation.set(key, [...(vectorsByMutation.get(key) ?? []), vector]);
    }
    const sourcesByMutation = new Map<string, MutationProjectionSource[]>();
    for (const source of subsetSources) {
      const key = source.mutationId;
      sourcesByMutation.set(key, [...(sourcesByMutation.get(key) ?? []), source]);
    }
    const consumed = new Set<string>();
    const canonicalizedMutationRoots = new Set<string>();
    for (const [mutationKey, mutationSources] of sourcesByMutation) {
      const vectors = vectorsByMutation.get(mutationKey) ?? [];
      const canonicalNames = [...new Set(vectors.map((vector) => canonicalProjectedMutationOwner(vector.mutationDirectoryName, vector.mutationId, subsetRoot, taxonomy)).filter((name): name is string => name !== null))];
      const mutationName = canonicalNames.length === 1 ? canonicalNames[0] : null;
      if (!mutationName || vectors.some((vector) => vector.mutationDirectoryName !== basename(mutationName) || mutationSources.some((source) => source.mutationDirectoryName !== vector.sourceMutationDirectoryName))) {
        for (const source of mutationSources) entries.get(source.scenarioRoot)?.violations.push(violation("projection-member-unresolved", source.mutationRoot, `Mutation member ${source.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const exact = new Map(vectors.map((vector) => [vector.scenarioDirectoryName, vector]));
      const assignments = new Map<MutationProjectionSource, MutationProjectionVector>();
      for (const source of mutationSources) {
        const vector = exact.get(source.sourceScenarioDirectoryName);
        if (vector) assignments.set(source, vector);
      }
      const unmatchedSources = mutationSources.filter((source) => !assignments.has(source));
      const matchedVectors = new Set(assignments.values());
      const unmatchedVectors = vectors.filter((vector) => !matchedVectors.has(vector));
      if (unmatchedSources.length === 1 && unmatchedVectors.length === 1) assignments.set(requireTaxonomyValue(unmatchedSources[0],"unmatchedSources[0]"), requireTaxonomyValue(unmatchedVectors[0],"unmatched projection vector"));
      if (assignments.size !== mutationSources.length || assignments.size !== vectors.length) {
        for (const source of mutationSources) entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-coverage", source.scenarioRoot, `Physical mutation ${mutationName} does not have an exact one-to-one vector registry`));
        continue;
      }
      for (const [source, vector] of assignments) {
        const vectorKey = `${vector.mutationId}\u0000${vector.sourceMutationDirectoryName}\u0000${vector.scenarioId}`;
        if (consumed.has(vectorKey)) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-duplicate", source.scenarioRoot, `Vector ${vectorKey.replaceAll("\u0000", "/")} owns more than one physical bundle`));
          continue;
        }
        const implementationProblem = projectionBundleProblem(source, entries, kinds, implementationContract, taxonomy);
        if (implementationProblem) {
          entries.get(source.scenarioRoot)?.violations.push(violation("mutation-implementation-invalid", source.scenarioRoot, implementationProblem));
          continue;
        }
        const implementationRoot = `${subsetRoot}/🧬️schema/🧬️mutations/${mutationName}/🧪️tests/${vector.scenarioDirectoryName}`.normalize("NFC");
        const sourceMutationRelativePath = source.mutationRoot.slice(`${subsetRoot}/🧬️schema/🧬️mutations/`.length);
        const fixtureSourceRoot = `${subsetRoot}/🧪️tests/🧫️fixtures/🧬️mutations/${sourceMutationRelativePath}/${source.sourceScenarioDirectoryName}`.normalize("NFC");
        const fixtureRoot = `${subsetRoot}/🧪️tests/🧫️fixtures/🧬️mutations/${mutationName}/${vector.scenarioDirectoryName}`.normalize("NFC");
        const fixtureSource = { ...source, scenarioRoot: fixtureSourceRoot };
        const fixtureProblem = projectionBundleProblem(fixtureSource, entries, kinds, fixtureContract, taxonomy);
        if (fixtureProblem) {
          entries.get(source.scenarioRoot)?.violations.push(violation("mutation-fixture-invalid", fixtureRoot, fixtureProblem));
          continue;
        }
        consumed.add(vectorKey);
        if (!pathBudgetFitsWithReserve(implementationRoot, implementationContract.pathBudgetReserve.utf16Units, taxonomy.schema.collisionPolicy) || !pathBudgetFitsWithReserve(fixtureRoot, fixtureContract.pathBudgetReserve.utf16Units, taxonomy.schema.collisionPolicy)) {
          entries.get(source.scenarioRoot)?.violations.push(violation("mutation-pair-path-budget", source.scenarioRoot, `Canonical mutation pair plus reserved descendant suffix exceeds ${pathBudgetStatement(taxonomy.schema.collisionPolicy)}`));
          continue;
        }
        const root = entries.get(source.scenarioRoot)!;
        root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
        const initialRoot = root.normalizedPath;
        for (const entry of entrySubtree(entries, source.scenarioRoot)) {
          const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
          setProjectedPath(entry, suffix ? `${implementationRoot}/${suffix}` : implementationRoot, taxonomy);
        }
        const fixture = entries.get(fixtureSourceRoot)!;
        fixture.violations = fixture.violations.filter((row) => row.code !== "directory-kind-unresolved");
        const initialFixtureRoot = fixture.normalizedPath;
        for (const entry of entrySubtree(entries, fixtureSourceRoot)) {
          const suffix = entry.normalizedPath === initialFixtureRoot ? "" : entry.normalizedPath.slice(initialFixtureRoot.length + 1);
          setProjectedPath(entry, suffix ? `${fixtureRoot}/${suffix}` : fixtureRoot, taxonomy);
        }
        if (!canonicalizedMutationRoots.has(source.mutationRoot)) {
          const mutation = entries.get(source.mutationRoot);
          const testsRoot = dirname(source.scenarioRoot);
          if (mutation) {
            const initialMutationRoot = mutation.normalizedPath;
            const canonicalMutationRoot = `${subsetRoot}/🧬️schema/🧬️mutations/${mutationName}`.normalize("NFC");
            const mutationEntries = entrySubtree(entries, source.mutationRoot).filter((entry) => entry.sourcePath !== testsRoot && !entry.sourcePath.startsWith(`${testsRoot}/`));
            mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
            for (const entry of mutationEntries) {
              const suffix = entry.normalizedPath === initialMutationRoot ? "" : entry.normalizedPath.startsWith(`${initialMutationRoot}/`) ? entry.normalizedPath.slice(initialMutationRoot.length + 1) : entry.sourcePath.slice(source.mutationRoot.length + 1);
              setProjectedPath(entry, suffix ? `${canonicalMutationRoot}/${suffix}` : canonicalMutationRoot, taxonomy, "parent-rebase");
            }
          }
          canonicalizedMutationRoots.add(source.mutationRoot);
        }
      }
    }
    const fixtureBase = `${subsetRoot}/🧪️tests/🧫️fixtures/🧬️mutations`;
    let physicalFixtures: Map<string, string> | undefined;
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\u0000${vector.sourceMutationDirectoryName}\u0000${vector.scenarioId}`;
      if (consumed.has(key)) continue;
      physicalFixtures ??= new Map(entrySubtree(entries, fixtureBase).filter((entry) => entry.nodeKind === "directory").map((entry) => [entry.sourcePath.replaceAll("\uFE0F", "").normalize("NFC"), entry.sourcePath]));
      const owner = canonicalProjectedMutationOwner(vector.mutationDirectoryName, vector.mutationId, subsetRoot, taxonomy);
      const found = [owner, vector.sourceMutationDirectoryName].filter((name): name is string => name !== null).map((name) => physicalFixtures!.get(`${fixtureBase}/${name}/${vector.scenarioDirectoryName}`.replaceAll("\uFE0F", "").normalize("NFC"))).find((path): path is string => path !== undefined);
      if (found === undefined || !covered(found)) continue;
      const problem = projectionBundleProblem({ scenarioRoot: found }, entries, kinds, fixtureContract, taxonomy);
      if (problem) entries.get(found)?.violations.push(violation("mutation-fixture-invalid", found, problem));
      else consumed.add(key);
    }
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\u0000${vector.sourceMutationDirectoryName}\u0000${vector.scenarioId}`;
      if (!consumed.has(key) && vector.catalogPath) entries.get(vector.catalogPath)?.violations.push(violation("projection-catalog-unrealized", vector.catalogPath, `Registered vector ${key.replaceAll("\u0000", "/")} has no physical bundle`));
    }
  }
}

export function validateMutationCasePairs(
  repoRoot: string,
  scope: string | undefined,
  entries: Map<string, MutableInventoryEntry>,
  _kinds: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): void {
  const expected = new Set<string>(), covered = fixtureCoverageOf(repoRoot, entries, taxonomy);
  const implementations = [...entries.values()].filter((entry) => entry.nodeKind === "directory").map((entry) => projectionSourceAt(entry.sourcePath, scope, entries, _kinds, taxonomy)).filter((entry): entry is MutationProjectionSource => entry !== null);
  for (const source of implementations) {
    const ownerRelativePath = source.mutationRoot.slice(`${source.subsetRoot}/🧬️schema/🧬️mutations/`.length);
    expected.add(`${source.subsetRoot}/🧪️tests/🧫️fixtures/🧬️mutations/${ownerRelativePath}/${source.sourceScenarioDirectoryName}`.normalize("NFC"));
  }
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "directory") continue;
    if (/\/🧪️tests\/🪆️[^/]+\/[^/]+\/🧪️[^/]+$/u.test(entry.sourcePath)) entry.violations.push(violation("mutation-projected-storage-forbidden", entry.sourcePath, "Mutation cases must use the subset-owned canonical implementation and fixture roots"));
    const looksLikeFixtureScenario = entry.sourcePath.includes("/🧪️tests/🧫️fixtures/🧬️mutations/") && entries.has(`${entry.sourcePath}/🦠️mutation`) && entries.has(`${entry.sourcePath}/📸️snapshot`) && entries.has(`${entry.sourcePath}/🔺️diff`) && entries.has(`${entry.sourcePath}/🎯️outcome`);
    if (looksLikeFixtureScenario && !expected.has(entry.sourcePath) && !covered(entry.sourcePath)) entry.violations.push(violation("mutation-fixture-unpaired", entry.sourcePath, "Fixture bundle has no canonical implementation case"));
  }
  const tableContract = taxonomy.schema.mutationCatalogProjection.fixtureCoverage.leafCaseTable, bundleFacets = taxonomy.schema.mutationCatalogProjection.fixtureCoverage.exhaustiveCase.bundleFacetDirectoryNames, discovery = taxonomy.discoverySchema;
  const tableKind = requireTaxonomyValue(discovery.fileKinds[tableContract.fileKindId], "leaf case table file kind");
  const tableName = `${tableKind.emoji}${tableKind.extensionChains[0]}`.normalize("NFC");
  const schemaMarker = `/${discovery.schemaScopeOwnerLevels.facetDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/`, tableSuffix = `/${discovery.testsDirName}/${tableName}`;
  for (const entry of entries.values()) {
    const at = entry.sourcePath.indexOf(schemaMarker);
    if (entry.nodeKind !== "file" || at < 0 || !entry.sourcePath.normalize("NFC").endsWith(tableSuffix)) continue;
    const owner = entry.sourcePath.slice(at + schemaMarker.length, entry.sourcePath.length - tableSuffix.length);
    const fixtureLeaf = `${entry.sourcePath.slice(0, at)}/${discovery.testsDirName}/${discovery.testFixturesDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/${owner}`;
    let table;
    try {
      table = parseMutationLeafCaseTable(readFileSync(absolutePath(repoRoot, entry.sourcePath), "utf8"), tableContract);
    } catch (error) {
      table = { error: `cannot be read: ${(error as Error).message}` };
    }
    const bundles = entrySubtree(entries, fixtureLeaf).filter((row) => row.nodeKind === "directory" && dirname(row.sourcePath) === fixtureLeaf && bundleFacets.every((facet) => entries.has(`${row.sourcePath}/${facet}`))).map((row) => basename(row.sourcePath));
    const problems = "error" in table ? [table.error] : entries.has(fixtureLeaf) || existsSync(absolutePath(repoRoot, fixtureLeaf)) ? mutationLeafCaseTableProblems(table.cases, bundles) : ["has no fixture leaf"];
    for (const problem of problems) entry.violations.push(violation("mutation-case-table-invalid", entry.sourcePath, `Case table ${problem}`));
  }
}
