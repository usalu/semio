/** 🔮️ Test platform: oracle governance laws
 *
 * The oracle registry, dependency ratchet, evidence rules, native second implementation, purity, cross-language hosts, production debt, serializer stub and re-implementation laws of the test platform.
 */

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { oracleHostPython, provisionPythonInterpreter, pythonSiteDirectories } from "../../🖥️host/🏗️materialization/🟦️.ts";
import { CORE_COMPARISON_PROFILES, dependencyEcosystemOf, externalOracleHostPackages, importProbe, oracleHostModule, oracleHostPackagesFor, oracleLinkedPackages, validateCaseContract, ratchetDependencies, stubSerializerBreaches, subjectHostPackagesFor, testCacheDir, testFilenameForKind, testTaxonomy, caseContractBreaches, isSemioNativeArtifact, isQualifyingOracleKind, nativeSecondImplementationBreaches, oracleRequirementBreaches, QUALIFYING_ORACLE_KINDS, noOracleMisuseBreaches, reimplementationOracleBreaches } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { contributed, exemptAreas, relativeToRepo, repoCases, repoOracleHits, repoRegistry, repoRoot, repoWideBreaches, withTestArtifacts } from "../🧰️repository-scan/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧪️Tests
describe("📇️ oracle registry", () => {
  test("every registered oracle is test-only and declares its license and capabilities", () => {
    const registry = repoRegistry;
    expect(registry.oracles.length).toBeGreaterThan(0);
    for (const oracle of registry.oracles) {
      expect(oracle.testOnly).toBe(true);
      expect(oracle.license.length).toBeGreaterThan(0);
      expect(oracle.capabilities.length).toBeGreaterThan(0);
      expect(oracle.comparisonProfiles.length).toBeGreaterThan(0);
    }
  });

  test("every recorded no-oracle decision names its rationale and its substitutes", () => {
    for (const decision of repoRegistry.noOracleDecisions) {
      expect(decision.rationale.length).toBeGreaterThan(20);
      expect(decision.substitutes.length).toBeGreaterThan(0);
    }
  });
});

describe("🔒️ dependency ratchet", () => {
  const registry = { schemaVersion: 1, oracles: [{ id: "pdf-writer", ecosystem: "rust", package: "pdf-writer", capabilities: ["pdf-create"], comparisonProfiles: ["semantic-pdf-v1"], license: "MIT", testOnly: true as const }], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], contributions: [] };
  const base = [{ ecosystem: "rust" as const, name: "existing", version: "1", kinds: ["production-runtime" as const], users: [], productionReachable: true }];

  test("a new production-reachable dependency is always forbidden", () => {
    const verdict = ratchetDependencies(base, [...base, { ecosystem: "js" as const, name: "left-pad", version: "1", kinds: ["production-runtime" as const], users: [], productionReachable: true }], registry);
    expect(verdict.ok).toBe(false);
    expect(verdict.newProduction).toEqual(["js:left-pad"]);
  });

  test("a new test oracle is permitted only when the registry claims it", () => {
    const registered = ratchetDependencies(base, [...base, { ecosystem: "rust" as const, name: "pdf-writer", version: "0.15", kinds: ["test-oracle" as const], users: [], productionReachable: false }], registry);
    expect(registered.ok).toBe(true);
    const unregistered = ratchetDependencies(base, [...base, { ecosystem: "rust" as const, name: "some-other-pdf-crate", version: "1", kinds: ["test-oracle" as const], users: [], productionReachable: false }], registry);
    expect(unregistered.ok).toBe(false);
    expect(unregistered.unregisteredTestDeps).toEqual(["rust:some-other-pdf-crate"]);
  });

  test("removing a dependency always passes", () => {
    const verdict = ratchetDependencies(base, [], registry);
    expect(verdict.ok).toBe(true);
    expect(verdict.removed).toEqual(["rust:existing"]);
  });

  test("the committed baseline classifies every ecosystem it tracks and keeps oracles out of production", () => {
    const baseline = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { ecosystem: string; name: string; kinds: string[]; productionReachable: boolean; users: string[] }[] };
    expect(baseline.entries.length).toBeGreaterThan(0);
    for (const entry of baseline.entries) {
      expect(["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"]).toContain(entry.kinds[0]);
      for (const area of exemptAreas()) expect(entry.users.every((user) => !user.startsWith(`${area}/`)), `${entry.name} is attributed to the exempt area ${area}`).toBe(true);
    }
    for (const oracle of repoRegistry.oracles) {
      for (const linked of oracleLinkedPackages(oracle)) {
        const entry = baseline.entries.find((candidate) => candidate.name === linked.package);
        expect(entry, `${linked.package} is linked by oracle ${oracle.id} but is absent from the dependency baseline`).toBeDefined();
        if (oracle.productionDebt === undefined) {
          expect(entry?.kinds, `${linked.package} is linked by oracle ${oracle.id} and records no productionDebt, so it must classify as test-oracle`).toEqual(["test-oracle"]);
          expect(entry?.productionReachable).toBe(false);
        } else {
          expect(oracle.productionDebt.reachableFrom.length, `${oracle.id} records productionDebt with no reachableFrom path`).toBeGreaterThan(0);
          expect(oracle.productionDebt.owner.length, `${oracle.id} records productionDebt with no owning path`).toBeGreaterThan(0);
          expect(oracle.productionDebt.plan.length, `${oracle.id} records productionDebt with no retirement plan`).toBeGreaterThan(0);
        }
      }
    }
  });
});

describe("🔮️ oracle evidence rules", () => {
  test("a differential scenario with neither an oracle nor a second implementation is a contract breach", () => withTestArtifacts(() => {
    const registry: import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry = { schemaVersion: 1, oracles: [], probes: [], noOracleDecisions: [{ id: "vectors-only", capabilities: ["x"], rationale: "a rationale long enough to satisfy the schema minimum length", substitutes: ["specification-vectors"] }], comparisonProfiles: [...CORE_COMPARISON_PROFILES], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], testEvidence: [], contributions: [] };
    const feature = "@capability-x @no-oracle-vectors-only @comparison-ordered-json-v1\nFeature: F\n  @id-s @level-quick @mode-differential\n  Scenario: S\n    Given a value\n";
    const taxonomy = testTaxonomy(repoRoot);
    const featureFilename = testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId);
    const adapterFilename = testFilenameForKind(taxonomy, taxonomy.testAdapterFileKinds["🟦️typescript"]!);
    const dir = join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test", "🧪️tests", "differential-without-oracle");
    rmSync(join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test"), { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, featureFilename), feature);
    writeFileSync(join(dir, adapterFilename), "export default {};\n");
    const discovered = {
      owner: relativeToRepo(repoRoot, join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test")),
      ownerName: "🧪️contract-self-test",
      case: "differential-without-oracle",
      caseDir: relativeToRepo(repoRoot, dir),
      featurePath: `${relativeToRepo(repoRoot, dir)}/${featureFilename}`,
      adapters: { typescript: `${relativeToRepo(repoRoot, dir)}/${adapterFilename}` },
      sharedFixtureDir: null,
     
      projectName: "self-test",
    };
    const breaches = validateCaseContract(repoRoot, discovered, registry);
    expect(breaches.some((breach) => breach.id === "differential-without-evidence")).toBe(true);
    rmSync(join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test"), { recursive: true, force: true });
  }));

  test("a declared differential row whose adapter cannot run is a contract breach: no entry point, or no adapter in the oracle's language", () => withTestArtifacts(() => {
    const oracle = { id: "reader-x", kind: "third-party-library" as const, ecosystem: "javascript", package: "reader-x", capabilities: ["x"], comparisonProfiles: ["ordered-json-v1"], license: "MIT", testOnly: true as const };
    const registry: import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry = { schemaVersion: 1, oracles: [oracle], probes: [], noOracleDecisions: [], comparisonProfiles: [...CORE_COMPARISON_PROFILES], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], testEvidence: [], contributions: [] };
    const feature = "@capability-x @oracle-reader-x @comparison-ordered-json-v1\nFeature: F\n  @id-s @level-quick @mode-differential\n  Scenario: S\n    Given a value\n";
    const taxonomy = testTaxonomy(repoRoot);
    const featureFilename = testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId);
    const filenameOf = (implementation: string): string => testFilenameForKind(taxonomy, taxonomy.testAdapterFileKinds[Object.entries(taxonomy.testImplementationIds).find(([, id]) => id === implementation)![0]]!);
    const root = join(testCacheDir(repoRoot, "work"), "🧪️contract-entry-self-test");
    rmSync(root, { recursive: true, force: true });
    const caseWith = (name: string, adapters: Readonly<Record<string, string>>) => {
      const dir = join(root, "🧪️tests", name);
      mkdirSync(dir, { recursive: true });
      writeFileSync(join(dir, featureFilename), feature);
      for (const [implementation, source] of Object.entries(adapters)) writeFileSync(join(dir, filenameOf(implementation)), source);
      return validateCaseContract(repoRoot, { owner: relativeToRepo(repoRoot, root), ownerName: "🧪️contract-entry-self-test", case: name, caseDir: relativeToRepo(repoRoot, dir), featurePath: `${relativeToRepo(repoRoot, dir)}/${featureFilename}`, adapters: Object.fromEntries(Object.keys(adapters).map((implementation) => [implementation, `${relativeToRepo(repoRoot, dir)}/${filenameOf(implementation)}`])), sharedFixtureDir: null, projectName: name }, registry).map((breach) => breach.id);
    };
    const script = caseWith("script-without-entry", { python: "import json\nprint(json.dumps({}))\n", typescript: "export default defineTestAdapter({ implementation: \"typescript\" });\n" });
    expect(script.filter((id) => id === "adapter-entry-point-missing")).toEqual(["adapter-entry-point-missing"]);
    expect(script).not.toContain("oracle-adapter-missing");
    const subjectOnly = caseWith("oracle-language-missing", { rust: "pub fn adapter() -> Adapter {\n    Adapter::new(\"rust\")\n}\n" });
    expect(subjectOnly).toContain("oracle-adapter-missing");
    expect(subjectOnly).not.toContain("adapter-entry-point-missing");
    const wired = caseWith("wired", { rust: "pub fn adapter() -> Adapter {\n    Adapter::new(\"rust\")\n}\n", typescript: "export default defineTestAdapter({ implementation: \"typescript\" });\n" });
    expect(wired.filter((id) => id === "adapter-entry-point-missing" || id === "oracle-adapter-missing")).toEqual([]);
    rmSync(root, { recursive: true, force: true });
  }));
});

describe("🌱️ native second implementation", () => {
  type Mutation = import("../../📦️packages/🟦️typescript/🟦️.ts").ManifestMutation;
  type Manifest = import("../../📦️packages/🟦️typescript/🟦️.ts").MutationManifest;
  type Entry = import("../../📦️packages/🟦️typescript/🟦️.ts").OracleEntry;
  type Evidence = import("../../📦️packages/🟦️typescript/🟦️.ts").NativeSecondImplementationEvidence;
  type Registry = import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;

  /** 🧬️ A manifest with one mutation per given [id, capability] pair, standing in for a subset's own vocabulary. */
  const nativeManifest = (artifact: string, pairs: readonly (readonly [string, string])[]): Manifest => ({
    schema: "semio.repository-test.mutation-manifest/v2",
    artifact,
    standard: "1",
    subset: "any",
    mutations: pairs.map(
      ([id, capability]): Mutation => ({
        id,
        capability,
        outcomes: ["applied"],
        productionDispatch: { operation: id, bridgeVersion: 1 },
        oracleRequirements: [{ capability, qualifyingKind: "verified-native-second-implementation" as const }],
      }),
    ),
  });

  /** 🌱️ Evidence that clears every check — the baseline every negative test below mutates exactly one field of. */
  const earnedEvidence = (format: string, capabilitiesCovered: readonly string[]): Evidence => ({
    format,
    noThirdPartySurvey: { ecosystemsSearched: ["python/pypi", "js/npm"], candidatesConsidered: [{ package: "unrelated-formulae-lib", reason: "implements design-code formulae, not the document interchange format" }] },
    subjectImplementationLanguage: "rust",
    secondImplementationLanguage: "python",
    specificationSource: "26/00/00/TEST-TICKET 📓️taxonomy.md — verb table and derivation rules",
    fixtureCoverage: { vectors: 3, capabilitiesCovered },
  });

  const entry = (id: string, capabilities: readonly string[], overrides: Partial<Entry> = {}): Entry => ({ id, kind: "verified-native-second-implementation", ecosystem: "python", package: "", capabilities, comparisonProfiles: ["ordered-json-v1"], license: "AGPL-3.0-only", testOnly: true, ...overrides });

  const registryWith = (owner: string, oracles: readonly Entry[], mutationManifests: readonly Manifest[]): Registry =>
    ({
      schemaVersion: 2,
      oracles,
      probes: [],
      noOracleDecisions: [],
      comparisonProfiles: [...CORE_COMPARISON_PROFILES],
      comparisonPipelines: [],
      toleranceProfiles: [],
      oracleHostPackages: [],
      mutationCatalogs: [],
      mutationManifests,
      testEvidence: [],
      contributions: [{ owner, manifestPath: `${owner}/🔮️oracles/🔣️.json`, oracles, noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], subjectHostPackages: [], mutationCatalogs: [], mutationManifests, testEvidence: [], probes: [], problems: [] }],
    }) as unknown as Registry;

  test("isSemioNativeArtifact refuses every s.stdio.* interchange format except s.stdio.semio itself", () => {
    for (const format of ["s.stdio.png", "s.stdio.pdf", "s.stdio.tiff", "s.stdio.gltf", "s.stdio.ifc", "s.stdio.step", "s.stdio.docx", "s.stdio.xlsx"]) expect(isSemioNativeArtifact(format), format).toBe(false);
    for (const format of ["s.stdio.semio", "s.norm.din16798", "s.block.5d", "s.puzzle.3d", "s.cad.cad", "os.config"]) expect(isSemioNativeArtifact(format), format).toBe(true);
    expect(isSemioNativeArtifact("")).toBe(false);
  });

  test("verified-native-second-implementation is a qualifying kind", () => {
    expect(QUALIFYING_ORACLE_KINDS).toContain("verified-native-second-implementation");
    expect(isQualifyingOracleKind("verified-native-second-implementation")).toBe(true);
  });

  test("a surveyed decision keeps a mutation's reference gap visible as medium, and an unsurveyed one still blocks", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const survey = { ecosystemsSearched: ["pypi", "crates.io"], candidatesConsidered: [{ package: "networkx", ecosystem: "pypi", verdict: "cannot-express-the-mutation" as const, reason: "models graphs but has no notion of the test-code document it would have to edit" }], whyNoneQualifies: "The format is defined by this repository, so no third party implements its mutation semantics." };
    const decision = { id: "test-code-gap", capabilities: ["test-code-1-mutate"], rationale: "The vocabulary is this repository's own.", substitutes: ["specification-vectors"], coversMutations: true, referenceSurvey: survey };
    const justified = { ...registryWith("test/owner", [], [manifest]), noOracleDecisions: [decision] } as unknown as Registry;
    const gap = oracleRequirementBreaches(justified, "scope", manifest, manifest.mutations[0]!);
    expect(gap.map((row) => [row.id, row.priority])).toEqual([["justified-reference-gap", "medium"]]);
    expect(noOracleMisuseBreaches(justified)).toEqual([]);
    const unsurveyed = { ...registryWith("test/owner", [], [manifest]), noOracleDecisions: [{ ...decision, referenceSurvey: undefined }] } as unknown as Registry;
    expect(oracleRequirementBreaches(unsurveyed, "scope", manifest, manifest.mutations[0]!).map((row) => row.id)).toEqual(["missing-external-oracle"]);
    expect(noOracleMisuseBreaches(unsurveyed).map((row) => row.id)).toEqual(["no-oracle-covers-mutation"]);
  });

  test("a fully earned claim discharges the requirement and records no breach of its own", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const registry = registryWith("test/owner", [entry("test-code-python-independent", ["test-code-1-mutate"], { nativeSecondImplementation: earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]) })], [manifest]);
    expect(nativeSecondImplementationBreaches(registry)).toEqual([]);
    expect(oracleRequirementBreaches(registry, "scope", manifest, manifest.mutations[0]!)).toEqual([]);
  });

  test("the kind field alone, with no recorded evidence, still mechanically discharges — which is exactly why the claim must also be earned", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const registry = registryWith("test/owner", [entry("unearned-claim", ["test-code-1-mutate"])], [manifest]);
    expect(oracleRequirementBreaches(registry, "scope", manifest, manifest.mutations[0]!)).toEqual([]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
  });

  test("an evidence record missing its schema-required format reports a breach instead of crashing contract validation", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const registry = registryWith("test/owner", [entry("missing-format", ["test-code-1-mutate"], { nativeSecondImplementation: {} as never })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches).toHaveLength(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
    expect(breaches[0]!.summary).toContain("format");
  });

  test("a real interchange format is refused however good the survey reads", () => {
    const manifest = nativeManifest("s.stdio.png", [["change-a", "png-1-2-mutate"]]);
    const registry = registryWith("test/owner", [entry("png-claims-native", ["png-1-2-mutate"], { nativeSecondImplementation: earnedEvidence("s.stdio.png", ["png-1-2-mutate"]) })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-not-native");
  });

  test("format naming no manifest this owner actually owns cannot be verified", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const registry = registryWith("test/owner", [entry("wrong-format", ["test-code-1-mutate"], { nativeSecondImplementation: earnedEvidence("s.norm.somewhere-else", ["test-code-1-mutate"]) })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
  });

  test("a partial second implementation must stay cross-semio-implementation — it cannot discharge the whole vocabulary", () => {
    const manifest = nativeManifest("s.norm.test-code", [
      ["change-a", "test-code-1-mutate"],
      ["change-b", "test-code-2-mutate"],
    ]);
    const registry = registryWith("test/owner", [entry("partial-coverage", ["test-code-1-mutate"], { nativeSecondImplementation: earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]) })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-partial-coverage");
  });

  test("a rationale-free or candidate-free negative search is not credible", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const base = earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]);
    for (const survey of [{ ecosystemsSearched: [], candidatesConsidered: base.noThirdPartySurvey.candidatesConsidered }, { ecosystemsSearched: base.noThirdPartySurvey.ecosystemsSearched, candidatesConsidered: [] }, { ecosystemsSearched: base.noThirdPartySurvey.ecosystemsSearched, candidatesConsidered: [{ package: "x", reason: "no" }] }]) {
      const registry = registryWith("test/owner", [entry("bad-survey", ["test-code-1-mutate"], { nativeSecondImplementation: { ...base, noThirdPartySurvey: survey } })], [manifest]);
      const breaches = nativeSecondImplementationBreaches(registry);
      expect(breaches.length, JSON.stringify(survey)).toBe(1);
      expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
    }
  });

  test("a same-language or transliterated reference is refused — it would catch a typo and nothing else", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const base = earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]);
    const registry = registryWith("test/owner", [entry("same-language", ["test-code-1-mutate"], { nativeSecondImplementation: { ...base, secondImplementationLanguage: "Rust" } })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-same-language");
  });

  test("an empty specificationSource cannot demonstrate the reference was read from a written specification", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const base = earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]);
    const registry = registryWith("test/owner", [entry("no-spec", ["test-code-1-mutate"], { nativeSecondImplementation: { ...base, specificationSource: "" } })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
  });

  test("zero committed fixture vectors is not fixture-tested evidence, whatever else the entry claims", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const base = earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]);
    const registry = registryWith("test/owner", [entry("no-fixtures", ["test-code-1-mutate"], { nativeSecondImplementation: { ...base, fixtureCoverage: { vectors: 0, capabilitiesCovered: [] } } })], [manifest]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
  });

  test("every registered verified-native-second-implementation entry in the live registry is earned", () => {
    const registry = repoRegistry;
    const live = registry.oracles.filter((oracle) => oracle.kind === "verified-native-second-implementation");
    expect(live.length).toBeGreaterThan(0);
    expect(nativeSecondImplementationBreaches(registry).filter((b) => live.some((oracle) => b.scope.endsWith(`#${oracle.id}`)))).toEqual([]);
    for (const oracle of live) expect(isSemioNativeArtifact(oracle.nativeSecondImplementation?.format ?? ""), oracle.id).toBe(true);
  }, 30_000);
});

describe("🚫️ oracle purity", () => {
  test("no production source imports a registered oracle", () => {
    expect(repoOracleHits.map((hit) => `${hit.path} → ${hit.oracle}`)).toEqual([]);
  });

  test("narrowing a run to one case must not make other cases' adapters look like production source", () => {
    expect(repoCases.length).toBeGreaterThan(1);
    expect(caseContractBreaches(repoRoot, repoCases.slice(0, 1), repoRegistry).filter((breach) => breach.id === "oracle-in-production")).toEqual([]);
    expect(repoWideBreaches.filter((breach) => breach.id === "oracle-in-production").map((breach) => breach.scope)).toEqual(repoOracleHits.map((hit) => hit.path));
  });
});

describe("🧩️ cross-language oracle hosts", () => {
  test("a subject host links the packages the owner and its ancestors declare for its language, merged per package and refused from two paths", () => {
    type Row = { implementation: "rust" | "python"; package: string; path: string; rationale: string };
    const contribution = (owner: string, subjectHostPackages: readonly Row[]) => ({ owner, manifestPath: `${owner}/🔮️oracles/🔣️.json`, oracles: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], subjectHostPackages, mutationCatalogs: [], mutationManifests: [], testEvidence: [], probes: [], problems: [] });
    const registry = (...contributions: ReturnType<typeof contribution>[]) => ({ schemaVersion: 2, oracles: [], probes: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], testEvidence: [], contributions });
    const base: Row = { implementation: "rust", package: "family-base", path: "plugin/artifact/base", rationale: "the subset builds on the family base" };
    const kernel: Row = { implementation: "rust", package: "kernel", path: "framework/kernel", rationale: "the subject dispatches through the kernel" };
    const tree = registry(contribution("plugin/artifact", [base]), contribution("plugin/artifact/subset", [kernel, base, { implementation: "python", package: "py-base", path: "py", rationale: "another language" }]), contribution("plugin/sibling", [{ ...kernel, package: "sibling" }]));
    expect(subjectHostPackagesFor(tree, "plugin/artifact/subset/case", "rust")).toEqual([base, kernel]);
    expect(subjectHostPackagesFor(tree, "plugin/artifact/subset/case", "python").map((row) => row.package)).toEqual(["py-base"]);
    expect(subjectHostPackagesFor(tree, "other", "rust")).toEqual([]);
    expect(() => subjectHostPackagesFor(registry(contribution("plugin", [base]), contribution("plugin/artifact", [{ ...base, path: "elsewhere" }])), "plugin/artifact/case", "rust")).toThrow("contributed from two paths");
  });

  test("a contributed host package is selected for whichever implementation declares it, not for Rust alone", () => {
    const registry = repoRegistry;
    const owners = new Set(repoCases.map((entry) => entry.owner));
    const selected = [...owners].flatMap((owner) => (["rust", "typescript", "python", "go", "dotnet"] as const).flatMap((implementation) => oracleHostPackagesFor(registry, owner, implementation).map((entry) => entry.implementation)));
    for (const declared of registry.contributions.flatMap((entry) => entry.oracleHostPackages)) expect(selected, `no owner reaches the declared ${declared.implementation} host package ${declared.package}`).toContain(declared.implementation);
    expect(new Set(selected).size).toBeGreaterThan(1);
  }, 60_000);

  test("a host package carrying a path is local source; one without a path is an external distribution", () => {
    const registry = repoRegistry;
    const external = externalOracleHostPackages(registry);
    const declared = registry.contributions.flatMap((entry) => entry.oracleHostPackages);
    const taxonomy = testTaxonomy(repoRoot);
    const contributionFilename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
    expect(external.map((entry) => `${entry.ecosystem}:${entry.name}`).sort()).toEqual([...new Set(declared.filter((entry) => entry.path === undefined).map((entry) => `${dependencyEcosystemOf(entry.implementation)}:${entry.package}`))].sort());
    for (const entry of external) expect(entry.users.every((user) => user.endsWith(contributionFilename))).toBe(true);
    expect(dependencyEcosystemOf("typescript")).toBe("js");
    expect(dependencyEcosystemOf("python")).toBe("python");
  }, 30_000);

  test("a cache-local Python host environment reuses every distribution its base interpreter provides instead of hiding it", () => withTestArtifacts(() => {
    const base = oracleHostPython(repoRoot);
    const read = (interpreter: string, program: string): string[] => {
      const probe = Bun.spawnSync([interpreter, "-c", program], { cwd: repoRoot, env: { ...process.env, PYTHONDONTWRITEBYTECODE: "1" }, stdout: "pipe", stderr: "pipe" });
      expect(probe.exitCode, Buffer.from(probe.stderr).toString()).toBe(0);
      return Buffer.from(probe.stdout).toString().trim().split("\n");
    };
    const [numpyVersion, pillowVersion] = read(base, "import importlib.metadata as meta; print(meta.version('numpy')); print(meta.version('pillow'))");
    const { interpreter, problems } = provisionPythonInterpreter(repoRoot, base, [{ implementation: "python", package: "numpy", version: numpyVersion, module: "numpy" }]);
    expect(problems).toEqual([]);
    expect(interpreter).not.toBe(base);
    const [numpyFile, pillowFile, prefix] = read(interpreter, "import numpy, PIL, sys; print(numpy.__file__); print(PIL.__file__); print(sys.prefix)");
    const sites = pythonSiteDirectories(repoRoot, base)!;
    for (const file of [numpyFile!, pillowFile!]) {
      expect(file.startsWith(prefix!), `${file} was installed into the environment instead of reused from ${base}`).toBe(false);
      expect(sites.some((site) => file.startsWith(site)), `${file} does not come from ${base}'s site directories ${sites.join(", ")}`).toBe(true);
    }
    expect(read(interpreter, "import importlib.metadata as meta; print(meta.version('pillow'))")).toEqual([pillowVersion]);
  }), 120_000);

  test("the import name defaults to the distribution name and is overridable", () => {
    expect(oracleHostModule({ implementation: "python", package: "ply-rs" })).toBe("ply_rs");
    expect(oracleHostModule({ implementation: "python", package: "Pillow", module: "PIL" })).toBe("PIL");
  });

  test("every ecosystem's own import syntax is what the purity gate looks for", () => {
    expect(importProbe("python", "pypdf").pattern.test("import pypdf\n")).toBe(true);
    expect(importProbe("python", "pypdf").pattern.test("from pypdf import PdfReader\n")).toBe(true);
    expect(importProbe("python", "pypdf").pattern.test("from pypdf.generic import NameObject\n")).toBe(true);
    expect(importProbe("python", "pypdf").pattern.test("# pypdf is mentioned in a comment\n")).toBe(false);
    expect(importProbe("rust", "lopdf").pattern.test("use lopdf::Document;")).toBe(true);
    expect(importProbe("javascript", "semver").pattern.test('import semver from "semver";')).toBe(true);
    expect(importProbe("javascript", "semver").pattern.test('const semver = require("semver");')).toBe(true);
    expect(importProbe("dotnet", "SixLabors").pattern.test("using SixLabors.ImageSharp;")).toBe(true);
  });

  test("an ecosystem's import syntax is only looked for in that ecosystem's files", () => {
    const taxonomy = testTaxonomy(repoRoot);
    const pythonFilename = testFilenameForKind(taxonomy, taxonomy.testAdapterFileKinds["🐍️python"]!);
    const rustFilename = testFilenameForKind(taxonomy, taxonomy.testAdapterFileKinds["🦀️rust"]!);
    expect(importProbe("rust", "json").files.test(pythonFilename)).toBe(false);
    expect(importProbe("python", "json").files.test(rustFilename)).toBe(false);
    expect(importProbe("python", "pypdf").files.test(pythonFilename)).toBe(true);
    expect(importProbe("javascript", "clsx").files.test("🟦️.tsx")).toBe(true);
  });

  test("an external host package is ratcheted exactly like an oracle package — declaring one is not a way around the gate", () => {
    const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [], contributions: [] } as unknown as import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;
    const base = [{ ecosystem: "rust" as const, name: "existing", version: "1", kinds: ["production-runtime" as const], users: [], productionReachable: true }];
    const verdict = ratchetDependencies(base, [...base, { ecosystem: "python" as const, name: "an-unregistered-reference-library", version: "1", kinds: ["test-oracle" as const], users: [], productionReachable: false }], registry);
    expect(verdict.ok).toBe(false);
    expect(verdict.unregisteredTestDeps).toEqual(["python:an-unregistered-reference-library"]);
  });

  test("the committed baseline classifies every external host package as a test-only dependency", () => {
    const baseline = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { ecosystem: string; name: string; kinds: string[]; productionReachable: boolean }[] };
    for (const host of externalOracleHostPackages(repoRegistry)) {
      const entry = baseline.entries.find((candidate) => candidate.ecosystem === host.ecosystem && candidate.name === host.name);
      expect(entry, `${host.ecosystem}:${host.name} is on a generated host's import path but is absent from the dependency baseline`).toBeDefined();
      if (repoRegistry.oracles.some((oracle) => oracle.productionDebt !== undefined && oracleLinkedPackages(oracle).some((linked) => linked.package === host.name))) continue;
      expect(entry!.kinds).toEqual(["test-oracle"]);
      expect(entry!.productionReachable).toBe(false);
    }
  }, 30_000);
});

describe("🔒️ recorded production debt", () => {
  test("an oracle claiming testOnly while already production-reachable must record the debt, not hide it", () => {
    const registry = repoRegistry;
    const baseline = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { name: string; productionReachable: boolean }[] };
    for (const oracle of registry.oracles) {
      const entry = baseline.entries.find((candidate) => candidate.name === oracle.package);
      if (entry?.productionReachable !== true) continue;
      expect(oracle.productionDebt, `${oracle.package} is production-reachable but records no debt`).toBeDefined();
      expect(oracle.productionDebt!.reachableFrom.length).toBeGreaterThan(0);
      expect(oracle.productionDebt!.plan.length).toBeGreaterThan(20);
    }
  });

  test("only the recorded paths are excused — any other production import is still a breach", () => {
    const recorded = new Set(repoRegistry.oracles.flatMap((oracle) => oracle.productionDebt?.reachableFrom ?? []));
    for (const hit of repoOracleHits) expect(recorded.has(hit.path), `unrecorded oracle import at ${hit.path}`).toBe(true);
  }, 60_000);

  test("every registered oracle names its capabilities, comparison profiles and a rationale that scopes it", () => {
    const known = contributed();
    for (const oracle of repoRegistry.oracles) {
      expect(oracle.capabilities.length).toBeGreaterThan(0);
      expect(oracle.comparisonProfiles.every((profile) => known.has(profile)), `${oracle.id} names a profile nobody defines`).toBe(true);
      expect((oracle.rationale ?? "").length).toBeGreaterThan(80);
    }
  }, 30_000);
});

describe("🫥️ hidden serializer stubs", () => {
  test("serde Value coercion into an unrelated target is rejected while explicit field mapping is accepted", () => {
    const root = mkdtempSync(join(tmpdir(), "stub-serializer-contract-"));
    const taxonomyPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    const serializer = join(root, "artifact/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs");
    try {
      mkdirSync(join(taxonomyPath, ".."), { recursive: true });
      writeFileSync(taxonomyPath, readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")));
      mkdirSync(join(serializer, ".."), { recursive: true });
      writeFileSync(serializer, "pub fn serialize(snapshot: &Source) -> Target { let value = serde_json::to_value(snapshot).unwrap(); serde_json::from_value(value).unwrap() }\n");
      expect(stubSerializerBreaches(root).map((entry) => entry.id)).toEqual(["stub-serializer"]);
      writeFileSync(serializer, "pub fn serialize(snapshot: &Source) -> Target { Target { rows: snapshot.rows.iter().map(|row| Row { id: row.id.clone() }).collect() } }\n");
      expect(stubSerializerBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("🎯 reimplementation-registered-as-third-party is entry-granular, not file-granular", () => {
  type Entry = import("../../📦️packages/🟦️typescript/🟦️.ts").OracleEntry;
  type Registry = import("../../📦️packages/🟦️typescript/🟦️.ts").OracleRegistry;

  /** ⛔️ One oracle entry, defaulting to the minimal shape the detector reads; override per test. */
  const entry = (id: string, ecosystem: string, overrides: Partial<Entry> = {}): Entry => ({ id, kind: "third-party-library", ecosystem, package: "some-package", capabilities: ["some-mutate"], comparisonProfiles: ["ordered-json-v1"], license: "MIT", testOnly: true, ...overrides });

  /** 📋️ A minimal registry naming exactly one contribution at `owner`, holding the given oracle entries. */
  const registryWith = (owner: string, oracles: readonly Entry[]): Registry =>
    ({
      schemaVersion: 2,
      oracles,
      probes: [],
      noOracleDecisions: [],
      comparisonProfiles: [...CORE_COMPARISON_PROFILES],
      comparisonPipelines: [],
      toleranceProfiles: [],
      oracleHostPackages: [],
      mutationCatalogs: [],
      mutationManifests: [],
      testEvidence: [],
      contributions: [{ owner, manifestPath: `${owner}/🔮️oracles/🔣️.json`, oracles, noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], subjectHostPackages: [], mutationCatalogs: [], mutationManifests: [], testEvidence: [], probes: [], problems: [] }],
    }) as unknown as Registry;

  /** 🦀️ A `match kind { … }` catch-all whose exact wording is what the detector's `predicts` regex looks
   *  for — the real shape found in `ruststep`/`quick-xml`/`html5ever`/`steputils`-adjacent owners this
   *  rule was written against (see `📓️g1-oracle-detector-granularity.md`). Every genuine-reimplementation
   *  fixture below uses it verbatim. */
  const predictingRustText = 'pub fn apply_kind(kind: &str) -> Result<Vec<u8>, String> {\n    match kind {\n        other => Err(format!("mutation kind {other:?} has no oracle implementation")),\n    }\n}\n';

  const withOracleFile = (owner: string, text: string, run: (root: string) => void): void => {
    const root = mkdtempSync(join(tmpdir(), "reimplementation-entry-granular-"));
    try {
      const rsPath = join(root, owner, "🔮️oracles", "🦀️.rs");
      mkdirSync(join(rsPath, ".."), { recursive: true });
      writeFileSync(rsPath, text);
      run(root);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  };

  test("POSITIVE — a genuine re-implementation registered as third-party still fires", () => {
    withOracleFile("test/owner-positive", predictingRustText, (root) => {
      const registry = registryWith("test/owner-positive", [entry("mislabelled-rust-reimplementation", "rust")]);
      const breaches = reimplementationOracleBreaches(root, registry);
      expect(breaches.length).toBe(1);
      expect(breaches[0]!.id).toBe("reimplementation-registered-as-third-party");
      expect(breaches[0]!.summary).toContain("mislabelled-rust-reimplementation");
    });
  });

  test("NEGATIVE — a real third-party wrapper sharing a file with re-implementation code does not fire", () => {
    withOracleFile("test/owner-negative", predictingRustText, (root) => {
      const registry = registryWith("test/owner-negative", [entry("genuine-python-reader", "python")]);
      expect(reimplementationOracleBreaches(root, registry)).toEqual([]);
    });
  });

  test("BOUNDARY — a genuine non-Rust reader is never named even when a Rust sibling in the SAME contribution is correctly caught", () => {
    withOracleFile("test/owner-boundary", predictingRustText, (root) => {
      const registry = registryWith("test/owner-boundary", [entry("mislabelled-rust-reimplementation", "rust"), entry("genuine-python-reader", "python")]);
      const breaches = reimplementationOracleBreaches(root, registry);
      expect(breaches.length).toBe(1);
      expect(breaches[0]!.summary).toContain("mislabelled-rust-reimplementation");
      expect(breaches[0]!.summary).not.toContain("genuine-python-reader");
    });
  });
});
//#endregion 🧪️Tests
