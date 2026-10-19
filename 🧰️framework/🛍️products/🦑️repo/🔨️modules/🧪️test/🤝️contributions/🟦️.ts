/** 🤝️ Test contributions
 *
 * Contribution records discovered below owners, the merged oracle registry and its table accessors.
 */

//#region 🔌️Adapters
import { type Implementation, type TestEvidence } from "@semio-tech/framework/test/adapter";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join, relative, sep } from "node:path";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { testFixture as protocolSchema } from "@semio-tech/framework/test/schema";
import { packagesForOwner } from "../🕸️dependencies/🟨️.mjs";
import { type ComparisonProfileSpec, CORE_COMPARISON_PROFILES, isExcludedTestPath, testFilenameForKind, testLocationPath, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { isPlainObject } from "../🟰️comparison/🟦️.ts";
import { walkDirectories } from "../🧭️case-discovery/🟦️.ts";
import { type NoOracleDecision, type OracleEntry, type OracleHostPackage, type SubjectHostPackage } from "../🗃️oracle-registry/🟦️.ts";
import { type MutationCatalog, strictMutationCatalogs } from "../🧮️mutation-catalog/🟦️.ts";
import { type ManifestMutation, type MutationManifest } from "../📑️mutation-manifest/🟦️.ts";
import { type ProbeEntry } from "../🔬️probes/🟦️.ts";
import { CORE_TOLERANCE_PROFILES, type ToleranceProfile } from "../📏️tolerances/🟦️.ts";
import { type ComparisonPipeline } from "../🔀️comparison-pipelines/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📇️Registry
/**
 * 🧩️ One owner's contribution to the test platform. This is the OPEN half of open/closed: an owner
 * that needs a reference implementation, a native oracle crate or a format-specific comparison
 * profile declares them here, and the framework discovers the manifest without ever naming the
 * owner, the plugin or the format.
 */
export type TestContribution = Readonly<{
  owner: string;
  manifestPath: string;
  oracles: readonly OracleEntry[];
  noOracleDecisions: readonly NoOracleDecision[];
  comparisonProfiles: readonly ComparisonProfileSpec[];
  oracleHostPackages: readonly OracleHostPackage[];
  /** 🧱️ Packages the subject host links beside this owner's package; never the oracle's. */
  subjectHostPackages: readonly SubjectHostPackage[];
  /** 🦠️ The mutation vocabularies this owner claims exhaustive coverage of. */
  mutationCatalogs: readonly MutationCatalog[];
  /** 🧬️ The authoritative subset-scoped mutation manifests this owner declares. */
  mutationManifests: readonly MutationManifest[];
  /** 🧫️ The provenance-carrying fixture manifests this owner commits. */
  testEvidence: readonly TestEvidence[];
  /** 🔬️ External measurement tools this owner contributes to the comparison pipeline. */
  probes: readonly ProbeEntry[];
  /** ⚖️ Multi-artifact comparison pipelines this owner contributes. */
  comparisonPipelines: readonly ComparisonPipeline[];
  /** 📏️ Scale-relative tolerance profiles this owner contributes. */
  toleranceProfiles: readonly ToleranceProfile[];
  /** 🧾️ What is malformed in this manifest. Reported by the contract phase; never thrown, because a throw during a repository-wide walk hides every other owner. */
  problems: readonly string[];
  /** 🔒️ Where this owner stands on the migration ladder, declared by the owner itself. */
  migrationStatus?: Readonly<Record<string, string>>;
}>;

export type OracleRegistry = Readonly<{
  schemaVersion: number;
  oracles: readonly OracleEntry[];
  probes: readonly ProbeEntry[];
  noOracleDecisions: readonly NoOracleDecision[];
  comparisonProfiles: readonly ComparisonProfileSpec[];
  comparisonPipelines: readonly ComparisonPipeline[];
  toleranceProfiles: readonly ToleranceProfile[];
  oracleHostPackages: readonly OracleHostPackage[];
  mutationCatalogs: readonly MutationCatalog[];
  mutationManifests: readonly MutationManifest[];
  testEvidence: readonly TestEvidence[];
  contributions: readonly TestContribution[];
}>;

/** 🧬️ The protocol definition every record of each contribution array must satisfy — the schema is the authority, the TS types mirror it. */
const CONTRIBUTION_RECORD_DEFINITIONS: Readonly<Record<string, string>> = {
  oracles: "OracleRegistryEntry",
  probes: "ProbeRegistryEntry",
  noOracleDecisions: "NoOracleDecision",
  comparisonProfiles: "ComparisonProfileSpec",
  comparisonPipelines: "ComparisonPipeline",
  toleranceProfiles: "ToleranceProfile",
  oracleHostPackages: "OracleHostPackage",
  subjectHostPackages: "SubjectHostPackage",
  mutationManifests: "MutationManifest",
  testEvidence: "TestEvidence",
};

/**
 * 🧬️ Validates every record of one parsed contribution against its protocol definition.
 *
 * Records were cast, never checked, so an oracle without `comparisonProfiles` or a decision spelled
 * `capability` reached every gate as `undefined` and crashed or silently passed them. Each finding
 * names the array, the record's id and the JSON pointer inside it.
 */
export function contributionSchemaProblems(parsed: Record<string, unknown>): string[] {
  return Object.entries(CONTRIBUTION_RECORD_DEFINITIONS).flatMap(([key, definition]) => {
    const records = parsed[key];
    if (records === undefined) return [];
    if (!Array.isArray(records)) return [`${key} must be an array`];
    return records.flatMap((record, index) => validateJsonSchemaSubset({ $ref: `#/$defs/${definition}` }, record, protocolSchema).map((error) => `${key}[${index}]${isPlainObject(record) && typeof record.id === "string" ? ` ${record.id}` : ""}: ${error.trimStart()} (${definition})`));
  });
}

function readContribution(repoRoot: string, owner: string, manifestPath: string): TestContribution | null {
  let parsed: Record<string, unknown>;
  try {
    parsed = JSON.parse(readFileSync(join(repoRoot, manifestPath), "utf8")) as Record<string, unknown>;
  } catch {
    return null;
  }
  const manifestDir = dirname(manifestPath);
  const problems: string[] = contributionSchemaProblems(parsed);
  const mutationCatalogs = strictMutationCatalogs(parsed.mutationCatalogs, owner, manifestPath, problems, testTaxonomy(repoRoot));
  return {
    owner,
    manifestPath,
    problems,
    oracles: (parsed.oracles as OracleEntry[] | undefined) ?? [],
    probes: (parsed.probes as ProbeEntry[] | undefined) ?? [],
    noOracleDecisions: (parsed.noOracleDecisions as NoOracleDecision[] | undefined) ?? [],
    comparisonProfiles: (parsed.comparisonProfiles as ComparisonProfileSpec[] | undefined) ?? [],
    comparisonPipelines: (parsed.comparisonPipelines as ComparisonPipeline[] | undefined) ?? [],
    toleranceProfiles: (parsed.toleranceProfiles as ToleranceProfile[] | undefined) ?? [],
    oracleHostPackages: (parsed.oracleHostPackages as OracleHostPackage[] | undefined) ?? [],
    subjectHostPackages: (parsed.subjectHostPackages as SubjectHostPackage[] | undefined) ?? [],
    mutationCatalogs,
    mutationManifests: (parsed.mutationManifests as MutationManifest[] | undefined) ?? [],
    testEvidence: ((parsed.testEvidence as TestEvidence[] | undefined) ?? []).map((fixture) => ({ ...fixture, manifestDir: fixture.manifestDir ?? manifestDir })),
    migrationStatus: (parsed.migrationStatus as Record<string, string> | undefined) ?? {},
  };
}

/**
 * 🧩️ Discovers each owner's contribution at its taxonomy-declared directory and JSON kind leaf.
 * An owner-specific handpicked directory replaces the default only at that exact owner.
 */
export function discoverTestContributions(repoRoot: string): TestContribution[] {
  const cached = contributionCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const taxonomy = testTaxonomy(repoRoot);
  const found: TestContribution[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    const owner = relative(repoRoot, dirname(abs)).split(sep).join("/") || ".";
    if (basename(abs) !== taxonomy.testOraclesDirName) return "enter";
    const manifest = join(abs, testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId));
    if (existsSync(manifest)) {
      const contribution = readContribution(repoRoot, owner, relative(repoRoot, manifest).split(sep).join("/"));
      if (contribution !== null) found.push(contribution);
    }
    return "skip";
  });
  found.sort((a, b) => a.owner.localeCompare(b.owner));
  contributionCache.set(repoRoot, found);
  return found;
}

/** 🧩️ Discovery walks the whole repository, and the registry is consulted per case and per role, so
 * the result is memoized for the life of the process. `clearContributionCache` exists for tests that
 * add or remove a manifest mid-run. */
const contributionCache = new Map<string, TestContribution[]>();

/**
 * 🧩️ Forgets the memoized contribution scan — of ONE root when it is named, of every root otherwise.
 *
 * A caller that mutated one synthetic repository names it: discarding every root's scan on its behalf
 * makes the next reader of a repository nobody touched pay a full repository walk again, which is how a
 * test budget ends up measuring the filesystem instead of the rule it asserts.
 */
export function clearContributionCache(repoRoot?: string): void {
  if (repoRoot === undefined) contributionCache.clear();
  else contributionCache.delete(repoRoot);
}

/**
 * 📇️ The effective registry: the framework's own core manifest merged with every discovered owner
 * contribution. The framework manifest holds only what is domain-neutral; nothing in this function
 * knows that PDF, PNG or any other format exists.
 */
export function loadOracleRegistry(repoRoot: string): OracleRegistry {
  const core = (() => {
    try {
      const taxonomy = testTaxonomy(repoRoot);
      return JSON.parse(readFileSync(join(repoRoot, testLocationPath(taxonomy, taxonomy.testOracleRegistryLocation)), "utf8")) as Partial<OracleRegistry> & { schemaVersion: number };
    } catch {
      return { schemaVersion: 2 } as Partial<OracleRegistry> & { schemaVersion: number };
    }
  })();
  const contributions = discoverTestContributions(repoRoot);
  return {
    schemaVersion: core.schemaVersion ?? 2,
    oracles: [...(core.oracles ?? []), ...contributions.flatMap((entry) => entry.oracles)],
    probes: [...(core.probes ?? []), ...contributions.flatMap((entry) => entry.probes)],
    noOracleDecisions: [...(core.noOracleDecisions ?? []), ...contributions.flatMap((entry) => entry.noOracleDecisions)],
    comparisonProfiles: [...CORE_COMPARISON_PROFILES, ...(core.comparisonProfiles ?? []), ...contributions.flatMap((entry) => entry.comparisonProfiles)],
    comparisonPipelines: [...(core.comparisonPipelines ?? []), ...contributions.flatMap((entry) => entry.comparisonPipelines)],
    toleranceProfiles: [...(core.toleranceProfiles ?? []), ...contributions.flatMap((entry) => entry.toleranceProfiles)],
    oracleHostPackages: [...(core.oracleHostPackages ?? []), ...contributions.flatMap((entry) => entry.oracleHostPackages)],
    mutationCatalogs: [...(core.mutationCatalogs ?? []), ...contributions.flatMap((entry) => entry.mutationCatalogs)],
    mutationManifests: [...(core.mutationManifests ?? []), ...contributions.flatMap((entry) => entry.mutationManifests)],
    testEvidence: [...(core.testEvidence ?? []), ...contributions.flatMap((entry) => entry.testEvidence)],
    contributions,
  };
}

/** ⚖️ The effective profile table: core profiles plus every contributed one, keyed by id. */
export function profileTable(registry: OracleRegistry): ReadonlyMap<string, ComparisonProfileSpec> {
  return new Map(registry.comparisonProfiles.map((spec) => [spec.id, spec]));
}

/** 🧩️ The native oracle packages one owner's adapters may reach, walking up to the nearest contributor. */
export function oracleHostPackagesFor(registry: OracleRegistry, owner: string, implementation: Implementation): OracleHostPackage[] {
  return packagesForOwner(registry.contributions, owner, implementation);
}

/**
 * 🧱️ The packages the generated SUBJECT host links beside the owner's own package: what the owner and each ancestor
 * contribution declares for `implementation`, one row per package (identical rows from two ancestors merge; a package
 * declared from two paths is refused).
 */
export function subjectHostPackagesFor(registry: OracleRegistry, owner: string, implementation: Implementation): SubjectHostPackage[] {
  const segments = owner.split("/");
  const ancestors = new Set(segments.map((_, index) => segments.slice(0, index + 1).join("/")));
  const merged = new Map<string, SubjectHostPackage>();
  for (const entry of registry.contributions.filter((contribution) => ancestors.has(contribution.owner) || contribution.owner === ".").flatMap((contribution) => contribution.subjectHostPackages).filter((candidate) => candidate.implementation === implementation)) {
    const previous = merged.get(entry.package);
    if (previous !== undefined && previous.path !== entry.path) throw new Error(`subject host package ${entry.package} is contributed from two paths (${previous.path} and ${entry.path})`);
    if (previous === undefined) merged.set(entry.package, entry);
  }
  return [...merged.values()].sort((left, right) => left.package.localeCompare(right.package));
}
//#endregion 📇️Registry

//#region 🧬️Manifest
/** 🧬️ Every manifest in the registry that owns a mutation with this id, for duplicate-owner detection. */
export function manifestsOwning(registry: OracleRegistry, mutationId: string): { manifest: MutationManifest; mutation: ManifestMutation }[] {
  const found: { manifest: MutationManifest; mutation: ManifestMutation }[] = [];
  for (const manifest of registry.mutationManifests) for (const mutation of manifest.mutations) if (mutation.id === mutationId) found.push({ manifest, mutation });
  return found;
}
//#endregion 🧬️Manifest

//#region 📏️Tolerance
/** 📏️ The effective profile table: core profiles plus every contributed one, keyed by id. */
export function toleranceProfileTable(registry: OracleRegistry): ReadonlyMap<string, ToleranceProfile> {
  return new Map([...CORE_TOLERANCE_PROFILES, ...registry.toleranceProfiles].map((profile) => [profile.id, profile]));
}
//#endregion 📏️Tolerance

//#region ⚖️Pipeline
/** ⚖️ The effective pipeline table: every contributed pipeline, keyed by id. */
export function pipelineTable(registry: OracleRegistry): ReadonlyMap<string, ComparisonPipeline> {
  return new Map(registry.comparisonPipelines.map((pipeline) => [pipeline.id, pipeline]));
}

/** 🔬️ The effective probe table, keyed by id. */
export function probeTable(registry: OracleRegistry): ReadonlyMap<string, ProbeEntry> {
  return new Map(registry.probes.map((probe) => [probe.id, probe]));
}
//#endregion ⚖️Pipeline
