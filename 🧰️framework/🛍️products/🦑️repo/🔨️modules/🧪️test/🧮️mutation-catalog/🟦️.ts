/** 🧮️ Mutation catalog
 *
 * Mutation vector and catalog records with the profile-slug rules and catalog problems.
 */

//#region 🔌️Adapters
import { basename } from "node:path";
import { loadCatalogTaxonomy, mutationCatalogSourceOwner, mutationOwnerRelativePath } from "../../📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { pathEmojiStatuteFindings } from "../../../../../📦️packages/🟦️typescript/🪪️identity/🛣️path/🟦️.ts";
import { leadingEmojiIdentity } from "../../../../../📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts";
import { type MutationCatalogAuthority } from "../🗂️taxonomy/🟦️.ts";
import { isPlainObject } from "../🟰️comparison/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📇️Registry
/**
 * 🦠️ One owner's declared mutation vocabulary for an artifact: the complete list of mutation kinds
 * the implementation can apply. This is the ground truth the completeness gate holds a feature
 * against, so "exhaustive" becomes a machine-checked claim instead of a hand-counted one.
 *
 * The framework never parses implementation source to learn this list — that would make it depend on
 * a language and on the shape of somebody else's enum. The OWNER declares it here and proves the
 * declaration honest with its own adjacent test.
 */
/** 🧪️ One canonical projected scenario identity. */
export type MutationVectorScenario = Readonly<{ id: string; directoryName: string }>;

/** 🧬️ One physical mutation directory and every scenario bundle it contributes to projection. */
export type MutationVector = Readonly<{ mutationId: string; sourceMutationDirectoryName: string; mutationDirectoryName: string; scenarios: readonly MutationVectorScenario[] }>;

/** 📚️ Runtime capability vocabulary plus its independent physical projection registry. */
export type MutationCatalog = Readonly<{
  id: string;
  capability: string;
  standardDirectoryName?: string;
  subsetDirectoryName?: string;
  kinds: readonly string[];
  vectors: readonly MutationVector[];
  deferredKinds?: readonly string[];
}>;

const MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

/** 📍️The path segment that marks an owner as carrying standards/subsets profile coordinates. */
export const PROFILE_MARKER = "/🏅️standards/";

/** 🏷️The logical slug a standard or subset directory renders after its semantic emoji (`🔖️1.1`, `9️⃣89a`, `✉️base`, `1️⃣cc1`). */
const PROFILE_SLUG_RE = /^[a-z0-9][a-z0-9.-]*$/u;

/**
 * 🪆️Whether a standard or subset directory name is one taxonomy-admitted semantic directory identity.
 * The emoji is the owner's own choice (`subsetDirectoryOverrides`, distinct sibling emojis), so only the
 * shape is judged here; `ownerContainsProfile` binds the exact spelling to the owner's real path.
 */
export function profileDirectoryIsCanonical(name: string, taxonomy: MutationCatalogAuthority = loadCatalogTaxonomy()): boolean {
  const identity = leadingEmojiIdentity(name);
  return identity.emoji.length > 0 && PROFILE_SLUG_RE.test(identity.rest) && pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length === 0;
}

/** ⏮️Whether the first standards marker in an owner contains these exact subset coordinates. */
function ownerContainsProfile(owner: string, standardDirectoryName: string, subsetDirectoryName: string): boolean {
  const markerIndex = owner.indexOf(PROFILE_MARKER);
  if (markerIndex < 0) return false;
  const profile = `${PROFILE_MARKER}${standardDirectoryName}/🪆️subsets/${subsetDirectoryName}`;
  const suffix = owner.slice(markerIndex);
  return suffix === profile || suffix.startsWith(`${profile}/`);
}

/** 🧾️ Validates the strict catalog record without consulting implementation source or runtime kinds. */
export function mutationCatalogProblems(value: unknown, owner?: string, taxonomy: MutationCatalogAuthority = loadCatalogTaxonomy()): string[] {
  if (!isPlainObject(value)) return ["catalog is not an object"];
  const problems: string[] = [];
  const allowed = new Set(["id", "capability", "standardDirectoryName", "subsetDirectoryName", "kinds", "vectors", "deferredKinds"]);
  for (const key of Object.keys(value)) if (!allowed.has(key)) problems.push(`unknown catalog field ${key}`);
  for (const key of ["id", "capability"] as const) {
    if (typeof value[key] !== "string" || value[key].length === 0) problems.push(`${key} must be a non-empty string`);
    else if (value[key] !== value[key].normalize("NFC")) problems.push(`${key} must be NFC`);
  }
  const profiled = owner === undefined ? value.standardDirectoryName !== undefined || value.subsetDirectoryName !== undefined : owner.includes(PROFILE_MARKER);
  const standardDirectoryName = typeof value.standardDirectoryName === "string" ? value.standardDirectoryName : "";
  const subsetDirectoryName = typeof value.subsetDirectoryName === "string" ? value.subsetDirectoryName : "";
  if (profiled) {
    for (const key of ["standardDirectoryName", "subsetDirectoryName"] as const) {
      if (typeof value[key] !== "string" || value[key].length === 0) problems.push(`${key} must be a non-empty string`);
      else if (value[key] !== value[key].normalize("NFC")) problems.push(`${key} must be NFC`);
    }
    if (!profileDirectoryIsCanonical(standardDirectoryName, taxonomy)) problems.push("standardDirectoryName must be one canonical semantic emoji followed by a standard slug");
    if (!profileDirectoryIsCanonical(subsetDirectoryName, taxonomy)) problems.push("subsetDirectoryName must be one canonical semantic emoji followed by a subset slug");
    if (owner !== undefined && !ownerContainsProfile(owner, standardDirectoryName, subsetDirectoryName)) problems.push("catalog profile does not match its contribution owner");
  } else {
    if (value.standardDirectoryName !== undefined) problems.push("standardDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
    if (value.subsetDirectoryName !== undefined) problems.push("subsetDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
  }
  if (!Array.isArray(value.kinds) || value.kinds.length === 0 || value.kinds.some((kind) => typeof kind !== "string" || kind.length === 0)) problems.push("kinds must be a non-empty string array");
  else if (new Set(value.kinds).size !== value.kinds.length) problems.push("kinds must be unique");
  if (value.deferredKinds !== undefined && (!Array.isArray(value.deferredKinds) || value.deferredKinds.some((kind) => typeof kind !== "string" || kind.length === 0) || new Set(value.deferredKinds).size !== value.deferredKinds.length)) problems.push("deferredKinds must be a unique string array");
  if (!Array.isArray(value.vectors)) {
    problems.push("vectors must be an array");
    return problems;
  }
  const mutationIds = new Set<string>();
  const mutationDirectories = new Set<string>();
  const sourceOwner = owner === undefined ? undefined : mutationCatalogSourceOwner(owner, taxonomy);
  if (sourceOwner === null) problems.push("catalog source ownership is invalid");
  const mutationRoot = sourceOwner === undefined || sourceOwner === null ? null : `${sourceOwner}/🧬️schema/🧬️mutations`;
  const grouped = mutationRoot !== null && Object.hasOwn(taxonomy.mutationDomainOwners, mutationRoot);
  for (const [vectorIndex, candidate] of value.vectors.entries()) {
    if (!isPlainObject(candidate)) {
      problems.push(`vectors[${vectorIndex}] is not an object`);
      continue;
    }
    for (const key of Object.keys(candidate)) if (!["mutationId", "sourceMutationDirectoryName", "mutationDirectoryName", "scenarios"].includes(key)) problems.push(`vectors[${vectorIndex}] has unknown field ${key}`);
    const mutationId = typeof candidate.mutationId === "string" ? candidate.mutationId : "";
    const sourceMutationDirectoryName = typeof candidate.sourceMutationDirectoryName === "string" ? candidate.sourceMutationDirectoryName : "";
    const mutationDirectoryName = typeof candidate.mutationDirectoryName === "string" ? candidate.mutationDirectoryName : "";
    if (!MUTATION_ID_RE.test(mutationId)) problems.push(`vectors[${vectorIndex}].mutationId must be kebab-case`);
    if (mutationIds.has(mutationId)) problems.push(`vectors mutationId ${mutationId} is duplicated`);
    mutationIds.add(mutationId);
    if (sourceMutationDirectoryName.length === 0 || sourceMutationDirectoryName !== sourceMutationDirectoryName.normalize("NFC")) problems.push(`vectors[${vectorIndex}].sourceMutationDirectoryName must be non-empty NFC`);
    if (mutationDirectoryName.length === 0 || mutationDirectoryName !== mutationDirectoryName.normalize("NFC")) problems.push(`vectors[${vectorIndex}].mutationDirectoryName must be non-empty NFC`);
    for (const [field, name] of Object.entries({ sourceMutationDirectoryName, mutationDirectoryName })) if (/[\\/]/u.test(name) || name === "." || name === "..") problems.push(`vectors[${vectorIndex}].${field} must be one basename`);
    const registeredOwner = grouped ? mutationOwnerRelativePath(mutationRoot!, mutationId, taxonomy) : null;
    if (grouped) {
      if (registeredOwner === null || basename(registeredOwner) !== sourceMutationDirectoryName || basename(registeredOwner) !== mutationDirectoryName) problems.push(`vectors[${vectorIndex}] has no exact registered domain-operation owner for mutationId ${mutationId}`);
    } else {
      if ((sourceMutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/)?.[0] ?? "") !== mutationId) problems.push(`vectors[${vectorIndex}].sourceMutationDirectoryName does not render mutationId ${mutationId}`);
      if ((mutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/)?.[0] ?? "") !== mutationId) problems.push(`vectors[${vectorIndex}].mutationDirectoryName does not render mutationId ${mutationId}`);
    }
    const directoryKey = registeredOwner ?? mutationDirectoryName;
    if (mutationDirectories.has(directoryKey)) problems.push(`vectors mutationDirectoryName ${directoryKey} is duplicated`);
    mutationDirectories.add(directoryKey);
    if (!Array.isArray(candidate.scenarios) || candidate.scenarios.length === 0) {
      problems.push(`vectors[${vectorIndex}].scenarios must be a non-empty array`);
      continue;
    }
    const scenarioIds = new Set<string>();
    const scenarioDirectories = new Set<string>();
    for (const [scenarioIndex, scenario] of candidate.scenarios.entries()) {
      if (!isPlainObject(scenario)) {
        problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}] is not an object`);
        continue;
      }
      for (const key of Object.keys(scenario)) if (!["id", "directoryName"].includes(key)) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}] has unknown field ${key}`);
      const id = typeof scenario.id === "string" ? scenario.id : "";
      const directoryName = typeof scenario.directoryName === "string" ? scenario.directoryName : "";
      if (!MUTATION_ID_RE.test(id)) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}].id must be kebab-case`);
      if (scenarioIds.has(id)) problems.push(`vectors[${vectorIndex}] scenario id ${id} is duplicated`);
      scenarioIds.add(id);
      const physicalId = leadingEmojiIdentity(directoryName).rest;
      if (!MUTATION_ID_RE.test(physicalId) || physicalId === directoryName || pathEmojiStatuteFindings([{ path: directoryName, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length > 0 || /[\\/]/u.test(directoryName) || directoryName === "." || directoryName === "..") problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName must be one canonical NFC test-case identity`);
      if (directoryName !== directoryName.normalize("NFC")) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName must be NFC`);
      const directoryKey = directoryName.replaceAll("\uFE0F", "").toLocaleLowerCase("und");
      if (scenarioDirectories.has(directoryKey)) problems.push(`vectors[${vectorIndex}] scenario directory ${directoryName} is duplicated`);
      scenarioDirectories.add(directoryKey);
    }
  }
  return problems;
}

/**
 * 📖️ Reads an owner's catalogs and REPORTS what is malformed instead of throwing.
 *
 * Throwing here was a blinding failure: `discoverTestContributions` walks the whole repository, so
 * one bad manifest aborted the scan and every other owner's catalogs, oracles, probes and fixtures
 * became invisible — the registry loaded empty and every gate that reads it reported green over
 * nothing. A malformed catalog is a contract BREACH, and a breach must be visible beside all the
 * others, not be the reason none of them can be seen.
 */
export function strictMutationCatalogs(value: unknown, owner: string, manifestPath: string, problems: string[], taxonomy: MutationCatalogAuthority): MutationCatalog[] {
  if (value === undefined) return [];
  if (!Array.isArray(value)) {
    problems.push("mutationCatalogs must be an array");
    return [];
  }
  const accepted: MutationCatalog[] = [];
  for (const [index, catalog] of value.entries()) {
    const found = mutationCatalogProblems(catalog, owner, taxonomy);
    if (found.length > 0) {
      for (const problem of found) problems.push(`mutationCatalogs[${index}] ${problem}`);
      continue;
    }
    accepted.push(catalog as MutationCatalog);
  }
  return accepted;
}
//#endregion 📇️Registry
