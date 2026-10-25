/** 🏛️ Repository contract
 *
 * Whole-repository contract gates: oracle purity, migration status and all-contract validation.
 */

//#region 🔌️Adapters
import { parseFeature } from "../../../../../📦️packages/🟦️typescript/🧪️test/🥒️gherkin/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative, sep } from "node:path";
import { type BreachRecord } from "@semio-tech/repo-lib/policy";
import { getRepoMetaDir } from "../../📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { isExcludedTestPath, isTestOraclePath, TEST_DOMAIN_REL_PATH, testFilenameForKind, testLocationPath, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { type DiscoveredCase, discoverTestCases, walkDirectories } from "../🧭️case-discovery/🟦️.ts";
import { oraclePackages } from "../🗃️oracle-registry/🟦️.ts";
import { PROFILE_MARKER } from "../🧮️mutation-catalog/🟦️.ts";
import { schemaModuleDirName } from "../📐️schema-contracts/🟦️.ts";
import { discoverTestContributions, loadOracleRegistry, type OracleRegistry } from "../🤝️contributions/🟦️.ts";
import { binaryProtocolDriftBreaches } from "../🌿️mutation-leaves/🟦️.ts";
import { dependencyEcosystemOf, externalOracleHostPackages, importProbe } from "../🔒️oracle-dependencies/🟦️.ts";
import { breach } from "../🚨️breaches/🟦️.ts";
import { capabilityManifestBreaches, fixtureProvenanceBreaches, isolationBreaches, mutationFixtureBreaches, mutationInventoryBreaches, nativeSecondImplementationBreaches, noOracleMisuseBreaches, registryRecordBreaches, reimplementationOracleBreaches, stubDeserializerBreaches, stubSerializerBreaches } from "../🚧️mutation-gates/🟦️.ts";
import { validateCaseContract } from "../📃️case-contract/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧾️Contract
/**
 * 🚫️ Production source files that import a registered oracle package. An oracle is evidence a test
 * host gathers; the moment production code can reach it, the differential test compares an
 * implementation with itself and the dependency stops being test-only.
 *
 * The exclusion set is derived from a FULL repository discovery, never from the caller's selected
 * scope — otherwise narrowing a run to one case would make every other case's adapter look like
 * production source importing an oracle.
 */
export function oracleImportsInProduction(repoRoot: string, registry: OracleRegistry = loadOracleRegistry(repoRoot), allCases: readonly DiscoveredCase[] = discoverTestCases(repoRoot)): { path: string; oracle: string }[] {
  const contributionRoots = registry.contributions.map((entry) => entry.manifestPath.slice(0, entry.manifestPath.lastIndexOf("/")));
  const hostRoots = [...contributionRoots, ...registry.oracles.map((entry) => entry.hostPath).filter((value): value is string => value !== undefined)];
  const caseDirs = new Set(allCases.map((entry) => entry.caseDir));
  const oracleOwners = new Map<string, string[]>();
  for (const contribution of registry.contributions) for (const oracle of contribution.oracles) oracleOwners.set(oracle.id, [...(oracleOwners.get(oracle.id) ?? []), contribution.owner]);
  const hostOwners = new Map<string, string[]>();
  for (const contribution of registry.contributions) for (const host of contribution.oracleHostPackages) if (host.path === undefined) hostOwners.set(`${dependencyEcosystemOf(host.implementation)}:${host.package}`, [...(hostOwners.get(`${dependencyEcosystemOf(host.implementation)}:${host.package}`) ?? []), contribution.owner]);
  const probes: { label: string; name: string; scope?: readonly string[]; pattern: RegExp; files: RegExp }[] = [
    ...registry.oracles.flatMap((entry) => oraclePackages(entry).map((name) => ({ label: entry.id, name, scope: oracleOwners.get(entry.id), ...importProbe(entry.ecosystem, name) }))),
    ...externalOracleHostPackages(registry).map((entry) => ({ label: `host package ${entry.ecosystem}:${entry.name}`, name: entry.name, scope: hostOwners.get(`${entry.ecosystem}:${entry.name}`), ...importProbe(entry.ecosystem, entry.name) })),
  ];
  const taxonomy = testTaxonomy(repoRoot);
  const testOwnedDirs = [taxonomy.testProbeDirName, taxonomy.testGeneratorDirName, taxonomy.testsDirName];
  const inContributionDir = (rel: string): boolean => isTestOraclePath(taxonomy, rel) || rel.split("/").some((segment) => testOwnedDirs.includes(segment));
  const metaRoot = relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/");
  const isRepositoryMeta = (rel: string): boolean => rel === metaRoot || rel.startsWith(`${metaRoot}/`);
  const isTestOwned = (rel: string): boolean => isRepositoryMeta(rel) || caseDirs.has(rel) || inContributionDir(rel) || hostRoots.some((root) => rel === root || rel.startsWith(`${root}/`)) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`);
  const recordedDebt = new Set(registry.oracles.flatMap((entry) => (entry.productionDebt?.reachableFrom ?? []).flatMap((path) => oraclePackages(entry).map((name) => `${path}::${name}`))));
  const hits: { path: string; oracle: string }[] = [];
  const reported = new Set<string>();
  const scopes = probes.every((probe) => probe.scope !== undefined) ? [...new Set(probes.flatMap((probe) => probe.scope ?? []))] : undefined;
  const inScope = (rel: string): boolean => scopes === undefined || scopes.some((owner) => rel === owner || rel.startsWith(`${owner}/`));
  const aboveScope = (rel: string): boolean => scopes !== undefined && (rel === "" || scopes.some((owner) => owner.startsWith(`${rel}/`)));
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (isTestOwned(rel)) return "skip";
    if (!inScope(rel)) return aboveScope(rel) ? "enter" : "skip";
    const entries = readdirSync(abs, { withFileTypes: true });
    for (const entry of entries) {
      if (!entry.isFile() || !/\.(rs|ts|tsx|js|jsx|mjs|cjs|go|py|cs)$/.test(entry.name)) continue;
      const filePath = `${rel}/${entry.name}`;
      let content: string;
      try {
        content = readFileSync(join(repoRoot, filePath), "utf8");
      } catch {
        continue;
      }
      for (const probe of probes) {
        if (recordedDebt.has(`${filePath}::${probe.name}`)) continue;
        if (!probe.files.test(entry.name)) continue;
        if (probe.scope !== undefined && !probe.scope.some((owner) => filePath.startsWith(`${owner}/`))) continue;
        if (reported.has(`${filePath}::${probe.name}`) || !probe.pattern.test(content)) continue;
        reported.add(`${filePath}::${probe.name}`);
        hits.push({ path: filePath, oracle: probe.label });
      }
    }
    return "enter";
  });
  return hits;
}


/** 🔒️ Ordered migration ladder every owner walks; an owner advances only on machine-readable evidence. */
export const MIGRATION_STATUSES = ["discovered", "surveyed", "contract-ready", "oracle-green", "subject-green", "parity-green", "coverage-green", "dependency-clean", "legacy-removed", "ci-enforced", "complete"] as const;
export type MigrationStatus = (typeof MIGRATION_STATUSES)[number];

/**
 * 🚥️ Every owner's declared migration status, collected from the owners themselves. The framework
 * holds no list of owners, so an owner that is deleted takes its status with it.
 */
export function migrationStatusByOwner(repoRoot: string): Record<string, string> {
  const collected: Record<string, string> = {};
  const core = (() => {
    try {
      const taxonomy = testTaxonomy(repoRoot);
      return JSON.parse(readFileSync(join(repoRoot, testLocationPath(taxonomy, taxonomy.testOracleRegistryLocation)), "utf8")) as { migrationStatus?: Record<string, string> };
    } catch {
      return {};
    }
  })();
  for (const [owner, status] of Object.entries(core.migrationStatus ?? {})) collected[owner] = status;
  for (const contribution of discoverTestContributions(repoRoot)) for (const [owner, status] of Object.entries(contribution.migrationStatus ?? {})) collected[owner] = status;
  return collected;
}

/**
 * 🎯️ Limits mutation-catalog enforcement to artifact standards and owners that expose Gherkin
 * behaviour. Framework-native Rust tests also use `🧬️mutations` for embedded fixture and module
 * trees, but without a standard profile or an owner-level feature there is no artifact contract for a
 * catalog to measure. Standards remain unconditional, while exact feature ownership keeps unprofiled
 * artifact surfaces in scope without letting an unrelated descendant feature claim a fixture tree.
 */
export function mutationVocabularyRequiresCatalog(vocabularyRel: string, featureOwners: ReadonlySet<string>): boolean {
  const owner = dirname(dirname(vocabularyRel));
  return owner.includes(PROFILE_MARKER) || featureOwners.has(owner);
}

/** 🧾️ Contract breaches of the selected cases alone: each case's own contract and the discovery self-check. */
export function caseContractBreaches(repoRoot: string, cases: readonly DiscoveredCase[], registry: OracleRegistry = loadOracleRegistry(repoRoot)): BreachRecord[] {
  const breaches = cases.flatMap((discovered) => validateCaseContract(repoRoot, discovered, registry));
  for (const leak of cases.filter((discovered) => isExcludedTestPath(repoRoot, discovered.caseDir))) {
    breaches.push(breach("testing/discovery", "excluded-path-leak", leak.caseDir, "Discovery returned a path the taxonomy excludes", "An excluded area is excluded in the discovery library itself, not by a caller's filter.", "Fix pathExclusions in 🔣️taxonomy.json."));
  }
  return breaches;
}

/** 🦑️ Repository-wide contract breaches. They read the FULL discovery and never a caller's selection, so narrowing a run to one case cannot change them.
 * A mutation vocabulary is the mutations facet of a schema module (`🧬️schema/🧬️mutations`); a fixture tree that mirrors the name holds vectors, not a vocabulary.
 * @see {@link oracleImportsInProduction} */
export function repositoryContractBreaches(repoRoot: string, registry: OracleRegistry = loadOracleRegistry(repoRoot), allCases: readonly DiscoveredCase[] = discoverTestCases(repoRoot), oracleHits: readonly { path: string; oracle: string }[] = oracleImportsInProduction(repoRoot, registry, allCases)): BreachRecord[] {
  const featureOwners = new Set(allCases.map((discovered) => discovered.owner));
  const breaches: BreachRecord[] = [];
  for (const hit of oracleHits) {
    breaches.push(breach("testing/dependency", "oracle-in-production", hit.path, `Production source imports the registered oracle ${hit.oracle}`, "An oracle is evidence a test host gathers. Once production code can reach it, the differential test compares an implementation with itself and the dependency stops being test-only.", "Move the usage into the oracle host, or remove the oracle from the registry."));
  }
  const vocabularyDir = testTaxonomy(repoRoot).testMutationVocabularyDirName;
  const schemaDir = schemaModuleDirName(repoRoot);
  {
    walkDirectories(repoRoot, (abs, rel) => {
      if (isExcludedTestPath(repoRoot, rel)) return "skip";
      if (basename(abs) !== vocabularyDir) return "enter";
      if (basename(dirname(abs)) !== schemaDir) return "skip";
      const owner = dirname(dirname(rel));
      const claimed = registry.contributions.some((entry) => entry.owner === owner && entry.mutationCatalogs.length > 0);
      if (!claimed && mutationVocabularyRequiresCatalog(rel, featureOwners)) {
        const taxonomy = testTaxonomy(repoRoot);
        breaches.push(breach("testing/contract", "unregistered-mutation-vocabulary", rel, `A mutation vocabulary is declared here but no catalog registers it`, "The completeness gate measures a feature against a declared catalog. A vocabulary with no catalog is not measured at all — it looks finished and is untested.", `Add a ${taxonomy.testOraclesDirName}/${testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId)} beside it declaring a mutationCatalog, and a case that claims it.`));
      }
      return "skip";
    });
  }

  const claimed = new Set(allCases.map((discovered) => parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8")).mutationCatalog).filter((id): id is string => id !== null));
  for (const catalog of registry.mutationCatalogs) {
    if (claimed.has(catalog.id)) continue;
    breaches.push(breach("testing/contract", "mutation-catalog-unclaimed", catalog.id, `Mutation catalog ${catalog.id} (${catalog.kinds.length} kinds) is claimed by no feature`, "The catalog declares what an artifact can do to itself. Declared and unclaimed means the whole vocabulary is untested while the manifest suggests otherwise.", `Add a case whose feature tags @mutations-${catalog.id}, or remove the catalog.`));
  }

  for (const contribution of registry.contributions) {
    for (const problem of contribution.problems) {
      breaches.push(breach("testing/contract", "contribution-manifest-invalid", contribution.manifestPath, problem, "An owner contribution is the platform's only view of that owner's oracles, probes, catalogs, manifests and fixtures. A malformed one is not skipped surface — it is surface the gates cannot see at all.", "Correct the contribution record."));
    }
  }
  breaches.push(...mutationInventoryBreaches(repoRoot, registry));
  breaches.push(...capabilityManifestBreaches(registry));
  breaches.push(...binaryProtocolDriftBreaches(repoRoot, registry));
  breaches.push(...stubSerializerBreaches(repoRoot));
  breaches.push(...stubDeserializerBreaches(repoRoot));
  breaches.push(...reimplementationOracleBreaches(repoRoot, registry));
  breaches.push(...nativeSecondImplementationBreaches(registry));
  breaches.push(...registryRecordBreaches(registry));
  breaches.push(...noOracleMisuseBreaches(registry));
  breaches.push(...fixtureProvenanceBreaches(repoRoot, registry));
  breaches.push(...mutationFixtureBreaches(registry));
  breaches.push(...isolationBreaches(registry));

  return breaches;
}

/** 🧬️ Repository-wide contract sweep: the selected cases' own contracts plus every repository-wide gate. */
export function validateAllContracts(repoRoot: string, cases: readonly DiscoveredCase[] = discoverTestCases(repoRoot)): BreachRecord[] {
  const registry = loadOracleRegistry(repoRoot);
  return [...caseContractBreaches(repoRoot, cases, registry), ...repositoryContractBreaches(repoRoot, registry)];
}
//#endregion 🧾️Contract
