/** 🏛️ Repository contract
 *
 * Whole-repository contract gates: oracle purity, migration status and all-contract validation.
 */

//#region 🔌️Adapters
import { parseFeature } from "@semio-tech/framework/test/gherkin";
import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative, sep } from "node:path";
import { type BreachRecord } from "../../📚️library/🚨️policy/🔖️breach/🟦️.ts";
import { getRepoMetaDir } from "../../📚️library/🗂️workspaces/🧬️metadata/🟦️.ts";
import { isExcludedTestPath, isTestOraclePath, TEST_DOMAIN_REL_PATH, testFilenameForKind, testLocationPath, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { type DiscoveredCase, discoverTestCases, walkDirectories } from "../🧭️case-discovery/🟦️.ts";
import { oraclePackages } from "../🗃️oracle-registry/🟦️.ts";
import { PROFILE_MARKER } from "../🧮️mutation-catalog/🟦️.ts";
import { schemaModuleDirName } from "../📐️schema-contracts/🟦️.ts";
import { discoverTestContributions, loadOracleRegistry, type OracleRegistry } from "../🤝️contributions/🟦️.ts";
import { binaryProtocolDriftBreaches } from "../🌿️mutation-leaves/🟦️.ts";
import { dependencyEcosystemOf, externalOracleHostPackages, importProbe } from "../🔒️oracle-dependencies/🟦️.ts";
import { breach } from "../🚨️breaches/🟦️.ts";
import { inPackageTestImplementation } from "../🗺️test-layout/🟦️.ts";
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
  // 🧩️An owner's whole contribution directory is test-owned BY DEFINITION — that is what the
  // directory is for — so it is derived from the discovered manifests rather than listed by hand.
  const contributionRoots = registry.contributions.map((entry) => entry.manifestPath.slice(0, entry.manifestPath.lastIndexOf("/")));
  const hostRoots = [...contributionRoots, ...registry.oracles.map((entry) => entry.hostPath).filter((value): value is string => value !== undefined)];
  const caseDirs = new Set(allCases.map((entry) => entry.caseDir));
  // 🧩️Both halves of a host's third-party surface are probed: the packages a registered oracle
  // names, and the EXTERNAL distributions an owner puts on a generated host's import path. The
  // second half is how a Python or npm reference library reaches an adapter, so leaving it out
  // would make the purity rule enforceable in Rust only.
  // 🎯️An oracle must be independent of the PRODUCTION PATH UNDER TEST: the owner that registers it.
  // Whether the package may be production-reachable at all is the dependency gate's question, answered
  // at the declaration (the baseline must classify it test-only or record the debt), so an import here
  // counts only inside the registering owner's own production sources. A core-registry oracle has no
  // owner and is scanned repository-wide.
  const oracleOwners = new Map<string, string[]>();
  for (const contribution of registry.contributions) for (const oracle of contribution.oracles) oracleOwners.set(oracle.id, [...(oracleOwners.get(oracle.id) ?? []), contribution.owner]);
  const hostOwners = new Map<string, string[]>();
  for (const contribution of registry.contributions) for (const host of contribution.oracleHostPackages) if (host.path === undefined) hostOwners.set(`${dependencyEcosystemOf(host.implementation)}:${host.package}`, [...(hostOwners.get(`${dependencyEcosystemOf(host.implementation)}:${host.package}`) ?? []), contribution.owner]);
  const probes: { label: string; name: string; scope?: readonly string[]; pattern: RegExp; files: RegExp }[] = [
    ...registry.oracles.flatMap((entry) => oraclePackages(entry).map((name) => ({ label: entry.id, name, scope: oracleOwners.get(entry.id), ...importProbe(entry.ecosystem, name) }))),
    ...externalOracleHostPackages(registry).map((entry) => ({ label: `host package ${entry.ecosystem}:${entry.name}`, name: entry.name, scope: hostOwners.get(`${entry.ecosystem}:${entry.name}`), ...importProbe(entry.ecosystem, entry.name) })),
  ];
  // 🧩️A contribution directory is test-owned because of WHAT IT IS, which the taxonomy names, not
  // because its manifest happens to parse. Deriving ownership from the discovered manifests alone
  // made a directory production source the moment its JSON was absent or malformed — so an owner
  // adding one would see its own reference libraries reported as a production dependency.
  // 🧩️A contribution, a probe, a fixture generator and anything under a tests directory are all
  // test-owned by WHAT THEY ARE. A unit test beside production code is compiled only under `cfg(test)`
  // or a test runner, exactly as the dependency classifier already treats every `🧪️tests/` declaration;
  // a scan that disagreed reported a unit test's reference library as a production dependency.
  const taxonomy = testTaxonomy(repoRoot);
  const testOwnedDirs = [taxonomy.testProbeDirName, taxonomy.testGeneratorDirName, taxonomy.testsDirName];
  const inContributionDir = (rel: string): boolean => isTestOraclePath(taxonomy, rel) || rel.split("/").some((segment) => testOwnedDirs.includes(segment));
  // 🚫️The repository's OWN meta directory — tickets, caches, notes, metrics — is never production
  // source. Walking into it reported a scratch file inside somebody's ticket folder as a production
  // import of a registered oracle, which is a finding about a scratch file, not about what ships.
  const metaRoot = relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/");
  const isRepositoryMeta = (rel: string): boolean => rel === metaRoot || rel.startsWith(`${metaRoot}/`);
  const isTestOwned = (rel: string): boolean => isRepositoryMeta(rel) || caseDirs.has(rel) || inContributionDir(rel) || hostRoots.some((root) => rel === root || rel.startsWith(`${root}/`)) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`);
  // 🔒️Recorded, shrink-only production debt: a package that was ALREADY production-reachable before
  // it was registered as an oracle. The path is named in the registry entry so the debt is visible
  // in the report instead of silently excused, and `dependency` prints it every run.
  // 🔒️Debt is recorded against a PACKAGE at a path, not against an oracle id, so the same library
  // registered twice — once as an oracle, once as a host package — is excused once rather than
  // reported twice with only one of the two suppressible.
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
    const entries = readdirSync(abs, { withFileTypes: true }), siblingNames = entries.filter((entry) => entry.isFile()).map((entry) => entry.name);
    for (const entry of entries) {
      if (!entry.isFile() || !/\.(rs|ts|tsx|js|jsx|mjs|cjs|go|py|cs)$/.test(entry.name)) continue;
      if (inPackageTestImplementation(entry.name, siblingNames, taxonomy)) continue;
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
 * 🔒️ Every owner's declared migration status, collected from the owners themselves. The framework
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
  // 🚫️Self-check: discovery must never return a path the taxonomy excludes. The excluded set is
  // vocabulary, so this check names no area of its own.
  for (const leak of cases.filter((discovered) => isExcludedTestPath(repoRoot, discovered.caseDir))) {
    breaches.push(breach("testing/discovery", "excluded-path-leak", leak.caseDir, "Discovery returned a path the taxonomy excludes", "An excluded area is excluded in the discovery library itself, not by a caller's filter.", "Fix pathExclusions in 🔣️taxonomy.json."));
  }
  return breaches;
}

/** 🧾️ Repository-wide contract breaches. They read the FULL discovery and never a caller's selection, so narrowing a run to one case cannot change them.
 * A mutation vocabulary is the mutations facet of a schema module (`🧬️schema/🧬️mutations`); a fixture tree that mirrors the name holds vectors, not a vocabulary.
 * @see {@link oracleImportsInProduction} */
export function repositoryContractBreaches(repoRoot: string, registry: OracleRegistry = loadOracleRegistry(repoRoot), allCases: readonly DiscoveredCase[] = discoverTestCases(repoRoot), oracleHits: readonly { path: string; oracle: string }[] = oracleImportsInProduction(repoRoot, registry, allCases)): BreachRecord[] {
  const featureOwners = new Set(allCases.map((discovered) => discovered.owner));
  const breaches: BreachRecord[] = [];
  for (const hit of oracleHits) {
    breaches.push(breach("testing/dependency", "oracle-in-production", hit.path, `Production source imports the registered oracle ${hit.oracle}`, "An oracle is evidence a test host gathers. Once production code can reach it, the differential test compares an implementation with itself and the dependency stops being test-only.", "Move the usage into the oracle host, or remove the oracle from the registry."));
  }
  // 🔒️Shrink-only migration ratchet: the legacy backlog may only get smaller. Reported as one
  // ratcheted count per area rather than thousands of individual findings, so the signal stays
  // meaningful while Phase 6 migrates owners.
  // 🦠️A catalog-governed vocabulary declared in the TREE but in no manifest is more invisible still:
  // the catalog check above can only see what a manifest declares. Standards and owners with Gherkin
  // behaviour therefore stay in scope even when they have not registered anything yet.
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

  // 🦠️A declared mutation vocabulary that no feature claims is worse than an undeclared one: it reads
  // as covered surface in the manifest while nothing exercises it.
  //
  // 🔍️The claimed set is derived from a FULL discovery rather than from `cases`. Callers narrow the
  // sweep — every generated Nx target runs `--case <one>` — and a repository-wide question answered
  // over one caller's selection would report every other case's catalog as unclaimed.
  const claimed = new Set(allCases.map((discovered) => parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8")).mutationCatalog).filter((id): id is string => id !== null));
  for (const catalog of registry.mutationCatalogs) {
    if (claimed.has(catalog.id)) continue;
    breaches.push(breach("testing/contract", "mutation-catalog-unclaimed", catalog.id, `Mutation catalog ${catalog.id} (${catalog.kinds.length} kinds) is claimed by no feature`, "The catalog declares what an artifact can do to itself. Declared and unclaimed means the whole vocabulary is untested while the manifest suggests otherwise.", `Add a case whose feature tags @mutations-${catalog.id}, or remove the catalog.`));
  }

  // 🧬️v2: the three gates v1 structurally could not run — runtime dispatch against the owner manifest
  // against the claimed test inventory, the qualifying-oracle requirement, and fixture provenance.
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

/** 🧾️ Repository-wide contract sweep: the selected cases' own contracts plus every repository-wide gate. */
export function validateAllContracts(repoRoot: string, cases: readonly DiscoveredCase[] = discoverTestCases(repoRoot)): BreachRecord[] {
  const registry = loadOracleRegistry(repoRoot);
  return [...caseContractBreaches(repoRoot, cases, registry), ...repositoryContractBreaches(repoRoot, registry)];
}
//#endregion 🧾️Contract
