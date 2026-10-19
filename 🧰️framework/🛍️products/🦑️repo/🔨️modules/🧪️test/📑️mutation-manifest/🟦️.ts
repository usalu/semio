/** 📑️ Mutation manifest
 *
 * Mutation manifests, runtime inventories and their equality.
 */

//#region 🔌️Adapters
import { type SubsetTarget, type MutationOutcomeClass, type EngineFamily, type EvidenceInvariants, MUTATION_OUTCOME_CLASSES } from "@semio-tech/framework/test/adapter";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { digest } from "../🔢️digests/🟦️.ts";
import { canonicalize, isPlainObject } from "../🟰️comparison/🟦️.ts";
import { testCacheDir } from "../⚡️output-cache/🟦️.ts";
import { subsetCoordinatesOfOwner, surfaceCoordinate, type SurfaceTarget } from "../🪆️subset-policy/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Manifest
/** 🎯️ The semantic class a fixture DECLARES for its mutation. "Any non-crash result" is not a class. */



/** 🚦️ Whether a raw value is one of the protocol's outcome classes. */
export function isMutationOutcomeClass(value: unknown): value is MutationOutcomeClass {
  return (MUTATION_OUTCOME_CLASSES as readonly unknown[]).includes(value);
}

/**
 * ✅️ The only oracle kinds that can DISCHARGE a mutation's external-oracle requirement.
 *
 * `verified-native-second-implementation` is the one exception to "a second implementation never
 * discharges" (see `SUPPLEMENTAL_ORACLE_KINDS` below), and it exists ONLY for a format no third party
 * can, even in principle, implement — `isSemioNativeArtifact` draws that boundary, and
 * `nativeSecondImplementationBreaches` makes the claim EARNED rather than merely asserted: a false or
 * lazy claim under this kind fails loudly, by design.
 */
export const QUALIFYING_ORACLE_KINDS = ["third-party-library", "third-party-cli", "standards-reference-tool", "verified-native-second-implementation"] as const;
export type QualifyingOracleKind = (typeof QUALIFYING_ORACLE_KINDS)[number];

/**
 * ➕️ Required supplements that can never substitute for a qualifying oracle. `cross-semio-implementation`
 * names a second implementation written INSIDE this repository from this repository's own schemas: it
 * catches transcription errors and cannot catch a misread specification, because both halves read the
 * same one.
 */
export const SUPPLEMENTAL_ORACLE_KINDS = ["metamorphic", "inverse", "round-trip", "property", "cross-semio-implementation"] as const;
export type SupplementalOracleKind = (typeof SUPPLEMENTAL_ORACLE_KINDS)[number];

export type OracleKind = QualifyingOracleKind | SupplementalOracleKind;

/** ✅️ Whether an oracle kind discharges an external-oracle requirement. */
export function isQualifyingOracleKind(kind: OracleKind | undefined): kind is QualifyingOracleKind {
  return kind !== undefined && (QUALIFYING_ORACLE_KINDS as readonly string[]).includes(kind);
}

/**
 * 🚫️ `s.stdio.*` names a real interchange format this repository DECODES — a genuine third party can,
 * in principle, implement one, so it always owes a real `third-party-library` / `third-party-cli` /
 * `standards-reference-tool` reference. `s.stdio.semio` is the one exception: it IS the format this
 * repository defines, so no vendor implements it by construction. This is the categorical boundary
 * `verified-native-second-implementation` may never cross — encoded here as the actual check
 * `nativeSecondImplementationBreaches` runs, not merely documented in a comment beside it.
 */
export function isSemioNativeArtifact(artifact: string): boolean {
  return artifact.length > 0 && !(artifact.startsWith("s.stdio.") && artifact !== "s.stdio.semio");
}

/**
 * 🌱️ The structured, machine-checked evidence a `verified-native-second-implementation` oracle entry
 * must carry. Every field is required; a field this function cannot itself verify (`specificationSource`,
 * the survey's `reason` prose) is still required to be PRESENT and non-empty rather than omitted — an
 * unfalsifiable claim recorded in the open is a far better failure mode than a silent gap, which is
 * exactly the failure this rule family exists to prevent.
 */
export type NativeSecondImplementationEvidence = Readonly<{
  /** 🎯️ The `mutationManifests[].artifact` id this claims native status for — cross-checked against `isSemioNativeArtifact` and against a real manifest this same contribution owns. */
  format: string;
  /** 🔍️ The recorded negative search: what was checked, and why each candidate considered was declined. */
  noThirdPartySurvey: Readonly<{ ecosystemsSearched: readonly string[]; candidatesConsidered: readonly Readonly<{ package: string; reason: string }>[] }>;
  /** 🗣️ The production subject's own implementation language — this reference is checked to differ from it. */
  subjectImplementationLanguage: string;
  /** 🗣️ The language this reference is actually written in. */
  secondImplementationLanguage: string;
  /** 📖️ The written specification this was authored FROM — never the subject's own source. */
  specificationSource: string;
  /** 🧫️ Fixture-backed vector evidence: how many committed vectors, and which capabilities they exercise. */
  fixtureCoverage: Readonly<{ vectors: number; capabilitiesCovered: readonly string[] }>;
}>;

/** ⚙️ The kernel a reference or a probe actually sits on — the unit independence is accounted in. */


/** ⚙️ Stable identity of one engine family, ignoring the wrapper that exposes it. */
export function engineFamilyId(engine: EngineFamily | undefined): string {
  return engine === undefined ? "unknown" : engine.family;
}

/** 🎯️ What a mutation needs from an external reference before it may be registered or released. */
export type OracleRequirement = Readonly<{ capability: string; qualifyingKind: QualifyingOracleKind; distinctEngineFamilies?: number; oracle?: string }>;

/** 🦠️ One mutation as its domain owner declares it — the single source feeding production and tests. */
export type ManifestMutation = Readonly<{
  id: string;
  capability: string;
  subset?: string;
  compound?: readonly string[];
  selectorSchema?: string;
  payloadSchema?: string;
  resultSchema?: string;
  outcomes: readonly MutationOutcomeClass[];
  productionDispatch: Readonly<{ operation: string; bridgeVersion: number; variant?: string }>;
  oracleRequirements: readonly OracleRequirement[];
  invariants?: EvidenceInvariants;
  normativeTopologyCounts?: boolean;
  carriers?: readonly string[];
  notes?: string;
  comparisonPipeline?: string;
}>;

/** 🧷️ Named invariants a mutation or fixture must keep, locally and in its enclosing artifact. */


/** 🧬️ One owner's authoritative mutation inventory for one artifact/standard/subset. */
export type MutationManifest = Readonly<{
  schema: "semio.repository-test.mutation-manifest/v2";
  artifact: string;
  standard: string;
  subset: string;
  standardDirectoryName?: string;
  subsetDirectoryName?: string;
  /** 🎚️ The editor or viewer state lane this manifest measures, as its owner's path below the subset (`✏️editor/🎚️config`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient`). Absent: the document dispatch. */
  surface?: string;
  mutations: readonly ManifestMutation[];
}>;

/** 🏭️ What production dispatch actually offers, emitted by running production code — never by parsing it. */
export type RuntimeMutationInventory = Readonly<{
  schema: "semio.repository-test.runtime-inventory/v2";
  artifact: string;
  standard: string;
  subset: string;
  surface?: string;
  bridgeVersion: number;
  producedBy?: string;
  mutations: readonly Readonly<{ id: string; variant: string; verb?: string; entity?: string; record?: string; outcomes: readonly MutationOutcomeClass[] }>[];
}>;

export const MANIFEST_MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
const CAPABILITY_ID_RE = /^[a-z0-9]+(?:[.-][a-z0-9]+)*$/;

/** 🧾️ Validates one manifest strictly. A manifest is the authority, so a malformed one is fatal. */
export function mutationManifestProblems(value: unknown, owner?: string): string[] {
  if (!isPlainObject(value)) return ["manifest is not an object"];
  const problems: string[] = [];
  if (value.schema !== "semio.repository-test.mutation-manifest/v2") problems.push('schema must be "semio.repository-test.mutation-manifest/v2"');
  for (const key of ["artifact", "standard", "subset"] as const) {
    if (typeof value[key] !== "string" || (value[key] as string).length === 0) problems.push(`${key} must be a non-empty string`);
  }
  const subset = typeof value.subset === "string" ? value.subset : "";
  if (subset.length === 0) problems.push("subset must be a non-empty string");
  const coordinates = owner === undefined ? null : subsetCoordinatesOfOwner(owner);
  if (coordinates !== null) {
    if (typeof value.standardDirectoryName === "string" && value.standardDirectoryName !== coordinates.standardDirectoryName) problems.push("standardDirectoryName does not match the owner path");
    if (typeof value.subsetDirectoryName === "string" && value.subsetDirectoryName !== coordinates.subsetDirectoryName) problems.push("subsetDirectoryName does not match the owner path");
    if (typeof value.standard === "string" && value.standard !== coordinates.standard) problems.push(`standard ${JSON.stringify(value.standard)} does not match the owner path standard ${JSON.stringify(coordinates.standard)}`);
  }
  if (!Array.isArray(value.mutations) || value.mutations.length === 0) {
    problems.push("mutations must be a non-empty array");
    return problems;
  }
  const seen = new Set<string>();
  for (const [index, raw] of value.mutations.entries()) {
    if (!isPlainObject(raw)) {
      problems.push(`mutations[${index}] is not an object`);
      continue;
    }
    const id = typeof raw.id === "string" ? raw.id : "";
    if (!MANIFEST_MUTATION_ID_RE.test(id)) problems.push(`mutations[${index}].id must be kebab-case`);
    if (seen.has(id)) problems.push(`mutations[${index}].id ${id} is duplicated — a mutation has exactly one owner`);
    seen.add(id);
    if (typeof raw.capability !== "string" || !CAPABILITY_ID_RE.test(raw.capability)) problems.push(`mutations[${index}].capability must be a dotted/kebab id`);
    if (raw.compound !== undefined && (!Array.isArray(raw.compound) || raw.compound.length < 2)) problems.push(`mutations[${index}].compound must name at least two subsets`);
    if (!Array.isArray(raw.outcomes) || raw.outcomes.length === 0 || raw.outcomes.some((outcome) => !(MUTATION_OUTCOME_CLASSES as readonly string[]).includes(String(outcome)))) {
      problems.push(`mutations[${index}].outcomes must be a non-empty array of ${MUTATION_OUTCOME_CLASSES.join("|")}`);
    } else if (new Set(raw.outcomes as string[]).size !== (raw.outcomes as string[]).length) problems.push(`mutations[${index}].outcomes must be unique`);
    if (!isPlainObject(raw.productionDispatch) || typeof raw.productionDispatch.operation !== "string" || typeof raw.productionDispatch.bridgeVersion !== "number") {
      problems.push(`mutations[${index}].productionDispatch must declare operation and bridgeVersion`);
    }
    if (!Array.isArray(raw.oracleRequirements) || raw.oracleRequirements.length === 0) {
      problems.push(`mutations[${index}].oracleRequirements must name at least one qualifying capability`);
    } else {
      for (const [requirementIndex, requirement] of raw.oracleRequirements.entries()) {
        if (!isPlainObject(requirement)) {
          problems.push(`mutations[${index}].oracleRequirements[${requirementIndex}] is not an object`);
          continue;
        }
        if (typeof requirement.capability !== "string" || !CAPABILITY_ID_RE.test(requirement.capability)) problems.push(`mutations[${index}].oracleRequirements[${requirementIndex}].capability must be a dotted/kebab id`);
        if (!(QUALIFYING_ORACLE_KINDS as readonly string[]).includes(String(requirement.qualifyingKind))) problems.push(`mutations[${index}].oracleRequirements[${requirementIndex}].qualifyingKind must be one of ${QUALIFYING_ORACLE_KINDS.join("|")}`);
      }
    }
  }
  return problems;
}

/** 🪆️ The subset one manifest mutation is owned by — its own override, else the manifest's. */
export function owningSubsetOf(manifest: MutationManifest, mutation: ManifestMutation): string {
  return mutation.subset ?? manifest.subset;
}

/** 🎯️ The full subset target of one manifest mutation. */
export function manifestTarget(manifest: MutationManifest, mutation: ManifestMutation): SubsetTarget {
  return { artifact: manifest.artifact, standard: manifest.standard, subset: owningSubsetOf(manifest, mutation), compound: mutation.compound };
}

/** #⃣ Content digest of one manifest — part of every run key, so a manifest edit invalidates caches. */
export function mutationManifestDigest(manifest: MutationManifest): string {
  return digest(JSON.stringify(canonicalize(manifest)));
}

/**
 * 🏭️ Where a generated runtime inventory is cached. A subset's inventory is produced by RUNNING the
 * production bridge; it is cache state, never committed source, so a stale checked-in copy can never
 * be mistaken for what the runtime offers today.
 */
export function runtimeInventoryPath(repoRoot: string, target: SurfaceTarget): string {
  return join(testCacheDir(repoRoot, "results"), "🏭️inventory", `${target.artifact}@${target.standard}@${target.subset}${target.surface === undefined ? "" : `@${target.surface}`}.json`.replace(/[^A-Za-z0-9@._-]+/g, "_"));
}

/** 🏭️ Reads a generated runtime inventory, or `null` when the bridge has not been run for that subset. */
export function readRuntimeInventory(repoRoot: string, target: SurfaceTarget): RuntimeMutationInventory | null {
  const path = runtimeInventoryPath(repoRoot, target);
  if (!existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as RuntimeMutationInventory;
    return parsed.schema === "semio.repository-test.runtime-inventory/v2" ? parsed : null;
  } catch {
    return null;
  }
}

/** 🏭️ Writes a bridge-produced runtime inventory into the cache. */
export function writeRuntimeInventory(repoRoot: string, inventory: RuntimeMutationInventory): string {
  const path = runtimeInventoryPath(repoRoot, inventory);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(inventory, null, 2)}\n`);
  return path;
}

/** 🧾️ The three-way difference the equality gate reports on. Every non-empty field is a hard failure. */
export type InventoryEquality = Readonly<{
  target: string;
  runtimeOnly: readonly string[];
  manifestOnly: readonly string[];
  testOnly: readonly string[];
  outcomeMismatches: readonly { mutation: string; runtime: readonly string[]; manifest: readonly string[] }[];
  variantMismatches: readonly { mutation: string; runtime: string; manifest: string }[];
  runtimeMissing: boolean;
}>;

/**
 * 🧾️ Compares production dispatch, the owner manifest and the claimed test inventory EXACTLY. This is
 * what closes v1's blind spot: a mutation can be reachable in production and absent from the catalog
 * without any v1 gate proving the omission, because v1 never consulted dispatch at all.
 */
export function compareInventories(manifest: MutationManifest, runtime: RuntimeMutationInventory | null, claimedTestKinds: readonly string[]): InventoryEquality {
  const target = surfaceCoordinate(manifest);
  const manifestIds = new Set(manifest.mutations.map((mutation) => mutation.id));
  const testIds = new Set(claimedTestKinds);
  if (runtime === null) {
    return { target, runtimeOnly: [], manifestOnly: [], testOnly: [...testIds].filter((id) => !manifestIds.has(id)).sort(), outcomeMismatches: [], variantMismatches: [], runtimeMissing: true };
  }
  const runtimeIds = new Set(runtime.mutations.map((mutation) => mutation.id));
  const outcomeMismatches: { mutation: string; runtime: readonly string[]; manifest: readonly string[] }[] = [];
  const variantMismatches: { mutation: string; runtime: string; manifest: string }[] = [];
  for (const mutation of manifest.mutations) {
    const runtimeRow = runtime.mutations.find((row) => row.id === mutation.id);
    if (runtimeRow === undefined) continue;
    const runtimeOutcomes = [...runtimeRow.outcomes].sort();
    const manifestOutcomes = [...mutation.outcomes].sort();
    if (JSON.stringify(runtimeOutcomes) !== JSON.stringify(manifestOutcomes)) outcomeMismatches.push({ mutation: mutation.id, runtime: runtimeOutcomes, manifest: manifestOutcomes });
    const declaredVariant = mutation.productionDispatch.variant;
    if (declaredVariant !== undefined && declaredVariant !== runtimeRow.variant) variantMismatches.push({ mutation: mutation.id, runtime: runtimeRow.variant, manifest: declaredVariant });
  }
  return {
    target,
    runtimeOnly: [...runtimeIds].filter((id) => !manifestIds.has(id)).sort(),
    manifestOnly: [...manifestIds].filter((id) => !runtimeIds.has(id)).sort(),
    testOnly: [...testIds].filter((id) => !manifestIds.has(id)).sort(),
    outcomeMismatches,
    variantMismatches,
    runtimeMissing: false,
  };
}
//#endregion 🧬️Manifest
