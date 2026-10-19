/** 🦠️ Test platform: mutation catalog laws
 *
 * The case-above-subset, binary protocol, mutation fixture, completeness, physical vector registry and real-world artifact fixture laws of the test platform.
 */

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join } from "node:path";
import { CORE_COMPARISON_PROFILES, mutationCatalogProblems, mutationCoverageBreaches, mutationVectorRegistryBreaches, resolveTestInputs, testInputUrisIn, isExcludedTestPath, testFilenameForKind, testTaxonomy, caseAboveSubsetBreaches, mutationFixtureBreaches, binaryProtocolDriftBreaches } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { parseFeature } from "@semio-tech/framework/test/gherkin";
import { repoCases, repoRegistry, repoRoot } from "../🧰️repository-scan/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧪️Tests
describe("🪆️ case above subset", () => {
  const featureFilename = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testFeatureFileKindId);
  const discoveredAt = (owner: string): import("../../📦️packages/🟦️typescript/🟦️.ts").DiscoveredCase =>
    ({ owner, ownerName: "📦️artifact", case: "mutate-thing", caseDir: `${owner}/🧪️tests/mutate-thing`, featurePath: `${owner}/🧪️tests/mutate-thing/${featureFilename}`, adapters: {}, sharedFixtureDir: null, projectName: "test-synthetic-000000-mutate-thing" } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").DiscoveredCase);
  const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️only", kinds: ["set-name"], vectors: [] };
  const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [] } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;
  const featureTagged = (tag: string): import("@semio-tech/framework/test/gherkin").ParsedFeature => parseFeature([`@capability-thing-mutate`, `@no-oracle-none`, tag, "Feature: Mutate a thing", "  @id-a", "  @level-quick", "  @mode-conformance", "  Scenario: a", "    Given a thing", "    Then it changed"].join("\n"));
  const featureUntagged: import("@semio-tech/framework/test/gherkin").ParsedFeature = parseFeature(["@capability-thing-mutate", "@no-oracle-none", "Feature: Round trip", "  @id-a", "  @level-quick", "  @mode-conformance", "  Scenario: a", "    Given a thing", "    Then it round-trips"].join("\n"));

  test("a case sitting above the one real subset its catalog names is a breach", () => {
    const breaches = caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureTagged("@mutations-thing-v1"), registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("case-above-subset");
    expect(breaches[0]!.summary).toContain("only");
  });

  test("a case already living under the catalog's own subset is left alone", () => {
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact/🏅️standards/🔖️1/🪆️subsets/✳️only"), featureTagged("@mutations-thing-v1"), registry)).toEqual([]);
  });

  test("a case with no @mutations- tag is never a candidate, whatever its adapter names", () => {
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureUntagged, registry)).toEqual([]);
  });

  test("a catalog naming no subsetDirectoryName yields no verdict rather than a guess", () => {
    const { subsetDirectoryName: _drop, ...unprofiled } = catalog;
    const unprofiledRegistry = { ...registry, mutationCatalogs: [unprofiled] } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureTagged("@mutations-thing-v1"), unprofiledRegistry)).toEqual([]);
  });

  test("a catalog id the registry does not declare yields no verdict — unknown-mutation-catalog is that breach's own job", () => {
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureTagged("@mutations-does-not-exist"), registry)).toEqual([]);
  });

  test("no live case sits above the subset its mutation catalog names", () => {
    const liveRegistry = repoRegistry;
    const scopes = repoCases
      .flatMap((discovered) => caseAboveSubsetBreaches(discovered, parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8")), liveRegistry))
      .map((entry) => entry.scope);
    expect(scopes).toEqual([]);
  }, 30_000);
});

describe("📡️ binary protocol records", () => {
  test("every live mutation vocabulary's 📡️.protocol.semio declares exactly one record per leaf kind, each at its own tag", () => {
    expect(binaryProtocolDriftBreaches(repoRoot, repoRegistry).map((row) => `${row.id} ${row.scope}: ${row.summary}`)).toEqual([]);
  }, 60_000);
});

describe("🧫️ mutation without fixture", () => {
  type Mutation = import("../../📦️packages/🟦️typescript/🟦️.ts").ManifestMutation;
  type Manifest = import("../../📦️packages/🟦️typescript/🟦️.ts").MutationManifest;
  type Catalog = import("../../📦️packages/🟦️typescript/🟦️.ts").MutationCatalog;
  type Fixture = import("@semio-tech/framework/test/adapter").TestEvidence;
  type Registry = import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;

  const mutation = (id: string, capability: string): Mutation => ({ id, capability, outcomes: ["applied"], productionDispatch: { operation: id, bridgeVersion: 1 }, oracleRequirements: [{ capability, qualifyingKind: "third-party-library" as const }] });
  const manifestWith = (mutations: readonly Mutation[]): Manifest => ({ schema: "semio.repository-test.mutation-manifest/v2", artifact: "s.norm.test-fixture", standard: "1", subset: "only", mutations });
  const fixture = (mutationId: string, subset = "only"): Fixture =>
    ({ id: `${mutationId}-fixture`, class: "handcrafted", target: { artifact: "s.norm.test-fixture", standard: "1", subset }, mutation: mutationId, units: { length: "unitless", angle: "degree" }, files: [], provenance: { license: "MIT", source: "handcrafted" }, comparisonProfile: "ordered-json-v1", reproducible: true } as unknown as Fixture);
  const catalog = (capability: string, mutationIds: readonly string[]): Catalog =>
    ({ id: `${capability}-catalog`, capability, kinds: mutationIds, vectors: mutationIds.map((id) => ({ mutationId: id, sourceMutationDirectoryName: id, mutationDirectoryName: id, scenarios: [{ id: "direct-behavior", directoryName: "🧪️direct-behavior" }] })) } as unknown as Catalog);
  const registryWith = (manifests: readonly Manifest[], fixtures: readonly Fixture[] = [], catalogs: readonly Catalog[] = []): Registry =>
    ({
      schemaVersion: 2,
      oracles: [],
      probes: [],
      noOracleDecisions: [],
      comparisonProfiles: [...CORE_COMPARISON_PROFILES],
      comparisonPipelines: [],
      toleranceProfiles: [],
      oracleHostPackages: [],
      mutationCatalogs: catalogs,
      mutationManifests: manifests,
      testEvidence: fixtures,
      contributions: [{ owner: "test/owner", manifestPath: "test/owner/🔮️oracles/🔣️.json", oracles: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], subjectHostPackages: [], mutationCatalogs: catalogs, mutationManifests: manifests, testEvidence: fixtures, probes: [], problems: [] }],
    }) as unknown as Registry;

  test("a mutation with neither a v2 fixture nor a v1 vector is a breach", () => {
    const breaches = mutationFixtureBreaches(registryWith([manifestWith([mutation("change-a", "test-fixture-1-mutate")])]));
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("mutation-without-fixture");
    expect(breaches[0]!.summary).toContain("change-a");
  });

  test("a registered v2 TestEvidence naming this mutation and target discharges it", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate")]);
    expect(mutationFixtureBreaches(registryWith([manifest], [fixture("change-a")]))).toEqual([]);
  });

  test("a v1 physical vector sharing the mutation's own capability discharges it too", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate")]);
    expect(mutationFixtureBreaches(registryWith([manifest], [], [catalog("test-fixture-1-mutate", ["change-a"])]))).toEqual([]);
  });

  test("a fixture registered for a DIFFERENT mutation of the same target does not discharge this one", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate"), mutation("change-b", "test-fixture-1-mutate")]);
    const breaches = mutationFixtureBreaches(registryWith([manifest], [fixture("change-a")]));
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.summary).toContain("change-b");
  });

  test("a kind the capability's catalog lists in deferredKinds is left to mutation-kinds-deferred, and only that kind", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate"), mutation("change-b", "test-fixture-1-mutate")]);
    const deferring = { ...catalog("test-fixture-1-mutate", []), kinds: ["change-b"], deferredKinds: ["change-a"] } as unknown as Catalog;
    expect(mutationFixtureBreaches(registryWith([manifest], [], [deferring])).map((row) => row.summary)).toEqual([expect.stringContaining("change-b")]);
    const elsewhere = { ...catalog("unrelated-capability", []), kinds: ["change-b"], deferredKinds: ["change-a"] } as unknown as Catalog;
    expect(mutationFixtureBreaches(registryWith([manifest], [], [elsewhere])).length).toBe(2);
  });

  test("a vector registered under a DIFFERENT catalog's capability does not discharge it — capability is the only correlation this rule trusts", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate")]);
    expect(mutationFixtureBreaches(registryWith([manifest], [], [catalog("unrelated-capability", ["change-a"])])).length).toBe(1);
  });

  test("a fixture registered for the same mutation id but a DIFFERENT subset does not discharge it", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate")]);
    expect(mutationFixtureBreaches(registryWith([manifest], [fixture("change-a", "elsewhere")])).length).toBe(1);
  });

  test("a malformed manifest is skipped here — mutation-manifest-invalid is that breach's own job", () => {
    const malformed = { schema: "semio.repository-test.mutation-manifest/v2", artifact: "", standard: "1", subset: "only", mutations: [] } as unknown as Manifest;
    expect(mutationFixtureBreaches(registryWith([malformed]))).toEqual([]);
  });

  test("the live registry retains the independent Stdio declaration census and has no declared fixture debt", () => {
    const liveRegistry = repoRegistry, taxonomy = testTaxonomy(repoRoot), owner = "✏️s/🔌️plugins/🗄️stdio";
    const physical: { path: string; value: Record<string, any> }[] = [];
    const walk = (path: string): void => {
      for (const entry of readdirSync(join(repoRoot, path), { withFileTypes: true })) {
        const child = `${path}/${entry.name}`;
        if (isExcludedTestPath(repoRoot, child)) continue;
        if (entry.isDirectory()) walk(child);
        else if (entry.isFile() && entry.name.endsWith(".json")) {
          const content = readFileSync(join(repoRoot, child), "utf8");
          if (!/"(?:mutationManifests|testEvidence)"\s*:/u.test(content)) continue;
          const value = JSON.parse(content);
          if (Array.isArray(value.mutationManifests) || Array.isArray(value.testEvidence)) physical.push({ path: child, value });
        }
      }
    };
    walk(owner);
    expect(physical.length).toBeGreaterThan(0);
    const declarationIds = (value: { mutationManifests?: readonly any[] }) => (value.mutationManifests ?? []).map((manifest) => ({ artifact: manifest.artifact, standard: manifest.standard, subset: manifest.subset, mutations: manifest.mutations.map((mutation: { id: string }) => mutation.id) }));
    const witnesses = new Set(["s.stdio.binary", "s.stdio.wav", "s.stdio.mp3", "s.stdio.txt"]);
    const observedWitnesses = new Set<string>();
    for (const row of physical) {
      const matches = liveRegistry.contributions.filter((contribution) => contribution.manifestPath === row.path);
      expect(matches.length, row.path).toBe(1);
      const live = matches[0]!;
      expect(basename(dirname(row.path)), row.path).toBe(taxonomy.testOraclesDirName);
      expect(declarationIds(live), row.path).toEqual(declarationIds(row.value));
      expect(live.testEvidence.map((fixture) => fixture.id), row.path).toEqual((row.value.testEvidence ?? []).map((fixture: { id: string }) => fixture.id));
      for (const manifest of row.value.mutationManifests ?? []) if (witnesses.has(manifest.artifact)) observedWitnesses.add(manifest.artifact);
    }
    const live = liveRegistry.contributions.filter((contribution) => contribution.owner === owner || contribution.owner.startsWith(`${owner}/`));
    expect(live.flatMap((contribution) => contribution.mutationManifests).flatMap((manifest) => manifest.mutations).length).toBe(physical.flatMap((row) => row.value.mutationManifests ?? []).flatMap((manifest) => manifest.mutations).length);
    expect(live.flatMap((contribution) => contribution.testEvidence).length).toBe(physical.flatMap((row) => row.value.testEvidence ?? []).length);
    expect([...observedWitnesses].sort()).toEqual([...witnesses].sort());
    expect(mutationFixtureBreaches(liveRegistry)).toEqual([]);
  }, 30_000);
});
//#endregion 🧪️Tests

describe("🦠️ mutation completeness gate", () => {
  const owner = "🧪️synthetic/📦️artifact";
  const featureFilename = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testFeatureFileKindId);
  const discovered = { owner, ownerName: "📦️artifact", case: "mutate-thing", caseDir: `${owner}/🧪️tests/mutate-thing`, featurePath: `${owner}/🧪️tests/mutate-thing/${featureFilename}`, adapters: {}, sharedFixtureDir: null, projectName: "test-synthetic-000000-mutate-thing" } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").DiscoveredCase;
  const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: ["set-name", "remove-item"], vectors: [] };
  const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [] } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;
  const feature = (scenarioIds: readonly string[], tag = "@mutations-thing-v1"): import("@semio-tech/framework/test/gherkin").ParsedFeature =>
    parseFeature([`@capability-thing-mutate`, `@no-oracle-none`, tag, "Feature: Mutate a thing", ...scenarioIds.flatMap((id) => [`  @id-${id}`, "  @level-exhaustive", "  @mode-differential", `  Scenario: ${id}`, "    Given a thing", "    Then it changed"])].join("\n"));

  test("a feature covering every declared kind twice reports nothing", () => {
    expect(mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "mutate-remove-item", "inverse-remove-item"]), registry)).toEqual([]);
  });

  test("an untested mutation kind is a breach naming the kind", () => {
    const ids = mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "inverse-remove-item"]), registry);
    expect(ids.map((entry) => entry.id)).toContain("mutation-kind-uncovered");
    expect(ids.some((entry) => entry.summary.includes("remove-item"))).toBe(true);
  });

  test("a kind that is applied but never inverted is a breach of its own", () => {
    const ids = mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "mutate-remove-item"]), registry);
    expect(ids.map((entry) => entry.id)).toContain("mutation-inverse-uncovered");
  });

  test("exercising a kind the catalog does not declare is a breach, so the declared set cannot drift", () => {
    const ids = mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "mutate-remove-item", "inverse-remove-item", "mutate-invented-kind"]), registry);
    expect(ids.map((entry) => entry.id)).toContain("mutation-kind-undeclared");
  });

  test("claiming a catalog that is not declared anywhere is a breach", () => {
    expect(mutationCoverageBreaches(discovered, feature([], "@mutations-does-not-exist"), registry).map((entry) => entry.id)).toContain("unknown-mutation-catalog");
  });

  test("a feature that claims no catalog is left alone", () => {
    expect(mutationCoverageBreaches(discovered, parseFeature("@capability-thing-mutate\nFeature: Something else\n  @id-a\n  @level-quick\n  @mode-conformance\n  Scenario: a\n    Given x\n    Then y"), registry)).toEqual([]);
  });

  test("deferring kinds is recorded rather than silently accepted", () => {
    const deferring = { ...registry, mutationCatalogs: [{ ...catalog, deferredKinds: ["rotate-item"] }] } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;
    expect(mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "mutate-remove-item", "inverse-remove-item"]), deferring).map((entry) => entry.id)).toContain("mutation-kinds-deferred");
  });
});

describe("🧬️ physical mutation vector registry", () => {
  const scenario = { id: "changes-the-value", directoryName: "🧪️changes-the-value" };
  const vector = { mutationId: "change-value", sourceMutationDirectoryName: "change-value", mutationDirectoryName: "🪄️change-value", scenarios: [scenario] };
  const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: ["runtime-only-operation"], vectors: [vector] };

  test("physical vectors are strict and independent of runtime capability kinds", () => {
    expect(mutationCatalogProblems(catalog, "artifact/🏅️standards/🔖️1/🪆️subsets/✳️any")).toEqual([]);
    expect(mutationCatalogProblems({ ...catalog, vectors: [{ mutationId: vector.mutationId, mutationDirectoryName: vector.mutationDirectoryName, scenarios: vector.scenarios }] })).toContain("vectors[0].sourceMutationDirectoryName must be non-empty NFC");
    expect(mutationCatalogProblems({ ...catalog, vectors: [{ ...vector, scenarios: [{ ...scenario, directoryName: "changes-the-value" }] }] })).toContain("vectors[0].scenarios[0].directoryName must be one canonical NFC test-case identity");
    expect(mutationCatalogProblems({ ...catalog, vectors: [vector, { ...vector, mutationDirectoryName: "🧭️change-value" }] })).toContain("vectors mutationId change-value is duplicated");
    expect(mutationCatalogProblems({ ...catalog, standardDirectoryName: "🔖️2" }, "artifact/🏅️standards/🔖️1/🪆️subsets/✳️any")).toContain("catalog profile does not match its contribution owner");
  });

  test("a nested facet owner inherits its containing subset profile", () => {
    const owner = "artifact/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config";
    expect(mutationCatalogProblems(catalog, owner)).toEqual([]);
    expect(mutationCatalogProblems({ ...catalog, subsetDirectoryName: "✳️other" }, owner)).toContain("catalog profile does not match its contribution owner");
    expect(mutationCatalogProblems(catalog, "artifact/🏅️standards/🔖️1/🪆️subsets/✳️anywhere/✏️editor/🎚️config")).toContain("catalog profile does not match its contribution owner");
  });

  test("a profile directory is any canonical semantic emoji plus its slug, never a fixed prefix", () => {
    for (const [standardDirectoryName, subsetDirectoryName] of [["🔖️v1", "✉️base"], ["9️⃣89a", "🧱️base"], ["4️⃣1.4", "♾️any"], ["🔖️ap214", "1️⃣cc1"]] as const) {
      expect(mutationCatalogProblems({ ...catalog, standardDirectoryName, subsetDirectoryName }, `artifact/🏅️standards/${standardDirectoryName}/🪆️subsets/${subsetDirectoryName}`)).toEqual([]);
    }
    expect(mutationCatalogProblems({ ...catalog, subsetDirectoryName: "any" }, "artifact/🏅️standards/🔖️1/🪆️subsets/any")).toContain("subsetDirectoryName must be one canonical semantic emoji followed by a subset slug");
    expect(mutationCatalogProblems({ ...catalog, standardDirectoryName: "1" }, "artifact/🏅️standards/1/🪆️subsets/✳️any")).toContain("standardDirectoryName must be one canonical semantic emoji followed by a standard slug");
  });

  test("an owner with no standards/subsets coordinates declares a catalog without them", () => {
    const facetOwner = "🧰️framework/🛍️products/💻️os/🎚️config";
    const { standardDirectoryName: _standard, subsetDirectoryName: _subset, ...profileless } = catalog;
    expect(mutationCatalogProblems(profileless, facetOwner)).toEqual([]);
    expect(mutationCatalogProblems(catalog, facetOwner)).toContain("standardDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
    expect(mutationCatalogProblems(catalog, facetOwner)).toContain("subsetDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
    expect(mutationCatalogProblems(profileless, "artifact/🏅️standards/🔖️1/🪆️subsets/✳️any")).toContain("standardDirectoryName must be a non-empty string");
  });

  test("all governed catalogs register the physical tree exactly once", () => {
    const registry = repoRegistry;
    const vectors = registry.mutationCatalogs.flatMap((entry) => entry.vectors);

    expect(registry.mutationCatalogs.length).toBeGreaterThan(0);
    expect(vectors.length).toBeGreaterThan(0);
    expect(vectors.every((entry) => entry.scenarios.length > 0)).toBe(true);
    expect(vectors.flatMap((entry) => entry.scenarios).length).toBeGreaterThanOrEqual(vectors.length);
    const directories = vectors.map((entry) => entry.mutationDirectoryName);
    expect(new Set(directories).size).toBeLessThanOrEqual(directories.length);

    const breaches = mutationVectorRegistryBreaches(repoRoot, registry);
    const known = new Set(["mutation-vector-catalog-invalid", "mutation-vector-mixed-state", "mutation-vector-duplicate-owner", "mutation-vector-bundle-invalid", "mutation-vector-source-id-mismatch", "mutation-vector-missing", "mutation-vector-unregistered"]);
    for (const entry of breaches) expect(known.has(entry.id), `unknown vector breach id ${entry.id}`).toBe(true);
    for (const entry of breaches) expect(entry.scope.length).toBeGreaterThan(0);
  }, 120_000);
});

describe("🧫️ real-world artifact fixtures", () => {
  const owner = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base";
  const thesis = `${owner}/🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf`;
  const featureFilename = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testFeatureFileKindId);
  const discovered = { owner, ownerName: "📄️pdf", case: "c", caseDir: `${owner}/🧪️tests/c`, featurePath: `${owner}/🧪️tests/c/${featureFilename}`, adapters: {}, sharedFixtureDir: null, projectName: "p" } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").DiscoveredCase;

  test("asset:// resolves against the owner assets and pins the real artifact's digest", () => {
    expect(existsSync(join(repoRoot, thesis))).toBe(true);
    const uri = "asset://🎓️bachelor-thesis/🎓️bachelor-thesis.pdf";
    const { inputs, missing } = resolveTestInputs(repoRoot, discovered, [uri]);
    expect(missing).toEqual([]);
    expect(inputs[0].scope).toBe("asset");
    expect(inputs[0].path).toBe(thesis);
    expect(inputs[0].digest.length).toBeGreaterThan(0);
  });

  test("asset:// cannot escape the owner assets", () => {
    expect(resolveTestInputs(repoRoot, discovered, ["asset://../../../../../../etc/hosts"]).missing.length).toBe(1);
  });

  test("the three fixture schemes are all extracted from a feature's text", () => {
    expect(testInputUrisIn(parseFeature("@capability-x\nFeature: f\n  @id-s\n  @level-quick\n  @mode-conformance\n  Scenario: s\n    Given asset://a/b.pdf and shared://c.png and local://d.csv\n    Then y"))).toEqual(["asset://a/b.pdf", "local://d.csv", "shared://c.png"]);
  });

  test("a test vector cannot be addressed as a production asset", () => {
    const root = mkdtempSync(join(tmpdir(), "projected-uri-"));
    const projectedOwner = "artifact";
    const projected = "🧪️tests/🪆️1-any/🌾change-humidification-required-kg-h/🧪️raises-required-humidification-to-3-point-5-kg-per-hour/🦠️mutation/🔣️.json";
    const projectedCase = { ...discovered, owner: projectedOwner };
    try {
      mkdirSync(dirname(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")), { recursive: true });
      writeFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")));
      mkdirSync(join(root, projectedOwner, ...projected.split("/").slice(0, -1)), { recursive: true });
      writeFileSync(join(root, projectedOwner, projected), "{}\n");
      const uri = `asset://${projected}`;
      expect(testInputUrisIn(parseFeature(`@capability-x\nFeature: f\n  @id-s @level-quick @mode-conformance\n  Scenario: s\n    Given ${uri}\n`))).toEqual([uri]);
      expect(resolveTestInputs(root, projectedCase, [uri])).toMatchObject({ missing: [uri], inputs: [] });
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
