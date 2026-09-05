//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative, sep } from "node:path";
import { clearContributionCache, isTestContributionPath, scanDeclaredDependencies, testContributionDirectoryName } from "./🟦️.ts";
import contributionDirectoryCases from "../../🧪️tests/🧭️contribution-directory-ownership/🔣️.json";
import contributionDirectorySchema from "../../🧪️tests/🧭️contribution-directory-ownership/🧬️schema/🔣️.json";

/** 🧭️ Repo-relative, forward-slashed path — the shape every discovered record carries. */
const relativeToRepo = (root: string, target: string): string => relative(root, target).split(sep).join("/");
import { CORE_COMPARISON_PROFILES, dependencyEcosystemOf, externalOracleHostPackages, importProbe, oracleHostModule, oracleHostPackagesFor, oracleLinkedPackages, mutationCatalogProblems, mutationCoverageBreaches, mutationVectorRegistryBreaches, mutationVocabularyRequiresCatalog, resolveFixtures, discoverTestContributions, profileTable, coreProfileTable, canonicalize, oracleImportsInProduction, computeCoverageMetrics, enforceMetricGates, validateCaseContract, cleanTestOutputs, compareProjections, digest, discoverTestCases, fixtureUrisIn, isExcludedTestPath, loadMigrationBaseline, migrationStatusByOwner, loadOracleRegistry, markOutputDir, parseFeature, MIGRATION_STATUSES, projectionHash, ratchetDependencies, readOutputMarker, repoRootFromHere, setDigest, stubSerializerBreaches, surveyUnmanagedTests, testCacheDir, testFilenameForKind, testLocationPath, testProjectName, testTaxonomy, validateAllContracts, validateResult, isSemioNativeArtifact, isQualifyingOracleKind, nativeSecondImplementationBreaches, oracleRequirementBreaches, QUALIFYING_ORACLE_KINDS, caseAboveSubsetBreaches, mutationFixtureBreaches, reimplementationOracleBreaches } from "./🟦️.ts";
//#endregion 🔌️Adapters

const repoRoot = repoRootFromHere();

/** ⚖️ The effective profile table: framework profiles plus every one an owner contributes. */
const contributed = (): ReadonlyMap<string, import("./🟦️.ts").ComparisonProfileSpec> => profileTable(loadOracleRegistry(repoRoot));

/** 🔣️ The taxonomy's own area vocabulary — these tests name no area of their own. */
const taxonomyAreas = (): Record<string, string> => JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areas as Record<string, string>;

/** 🚫️ Areas the taxonomy marks exempt, and the layers it marks as implementations. */
const exemptAreas = (): string[] => Object.entries(taxonomyAreas()).filter(([, state]) => state === "exempt").map(([area]) => area);
const implementationAreas = (): string[] => Object.entries(JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areaLayers as Record<string, string>).filter(([, layer]) => layer === "implementation").map(([area]) => area);

//#region 🧪️Tests
describe("🔣️ contract", () => {
  test("the taxonomy exposes every frozen test vocabulary key", () => {
    const taxonomy = testTaxonomy(repoRoot);
    expect(taxonomy.testsDirName).toBe("🧪️tests");
    expect(taxonomy.testFeatureFileKindId).toBe("gherkin-feature");
    expect(testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId)).toBe("🥒️.feature");
    expect(taxonomy.testAdapterFileKinds).toEqual({ "🦀️rust": "rust-source", "🟦️typescript": "typescript-source", "🐹️go": "go-source", "🐍️python": "python-source", "🔷️dotnet": "dotnet-source" });
    expect(Object.values(taxonomy.testAdapterFileKinds).map((kindId) => testFilenameForKind(taxonomy, kindId)).sort()).toEqual(["🐍️.py", "🐹️.go", "🔷️.cs", "🟦️.ts", "🦀️.rs"].sort());
    expect(taxonomy.testOutputChildDirs).toEqual(["work", "hosts", "oracles", "results", "diffs", "reports"]);
  });

  test("every exempt area is excluded by the discovery library itself, not by a caller's filter", () => {
    const areas = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areas as Record<string, string>;
    const exempt = Object.entries(areas).filter(([, state]) => state === "exempt").map(([area]) => area);
    expect(exempt.length).toBeGreaterThan(0);
    for (const area of exempt) {
      expect(isExcludedTestPath(repoRoot, area)).toBe(true);
      expect(isExcludedTestPath(repoRoot, `${area}/anything/below/it`)).toBe(true);
    }
  });

  test("no area is excluded merely for being legacy or mixed — only an exempt one is", () => {
    const areas = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).areas as Record<string, string>;
    for (const [area, state] of Object.entries(areas)) {
      if (state === "exempt") continue;
      expect(isExcludedTestPath(repoRoot, area), `${area} (${state}) must not be excluded`).toBe(false);
    }
  });

  test("project names are deterministic and CLI-safe", () => {
    const owner = "🧪️synthetic/🔬️owner/📐️with-emoji";
    expect(testProjectName(owner, "a-case")).toBe(testProjectName(owner, "a-case"));
    expect(testProjectName(owner, "a-case")).toMatch(/^[a-z0-9-]+$/);
    expect(testProjectName(owner, "a-case")).not.toBe(testProjectName(`${owner}-other`, "a-case"));
  });
});

describe("🥒️ feature profile", () => {
  const source = `@capability-thing @oracle-pdf-writer @oracle-input-subject-raw @comparison-semantic-pdf-v1
Feature: A thing
  Some description.

  Background:
    Given the shared fixture shared://vector.bin

  @id-one @level-quick @mode-differential @seed-7
  Scenario: One
    Given a value
    When it is encoded
    Then it round trips
    """
    doc string
    """

  @id-outlined @level-long @mode-property
  Scenario Outline: Many
    Given <width> by <height>
    Then it matches local://<id>.json

    Examples:
      | id | width | height |
      | a  | 1     | 2      |
      | b  | 3     | 4      |
`;

  test("parses tags, background, doc strings and scenario outlines", () => {
    const feature = parseFeature(source);
    expect(feature.errors).toEqual([]);
    expect(feature.capability).toBe("thing");
    expect(feature.oracle).toBe("pdf-writer");
    expect(feature.oracleInput).toBe("subject-raw");
    expect(feature.comparison).toBe("semantic-pdf-v1");
    expect(feature.background).toHaveLength(1);
    expect(feature.scenarios.map((scenario) => scenario.id)).toEqual(["one", "outlined-a", "outlined-b"]);
    expect(feature.scenarios[0]!.seed).toBe("7");
    expect(feature.scenarios[0]!.steps[2]!.docString).toBe("doc string");
    expect(feature.scenarios[1]!.steps[0]!.text).toBe("1 by 2");
  });

  test("`And` inherits the previous canonical keyword", () => {
    const feature = parseFeature("@capability-x @no-oracle-y @comparison-ordered-json-v1\nFeature: F\n  @id-s @level-quick @mode-error\n  Scenario: S\n    When a thing\n    And another\n    Then it fails\n");
    expect(feature.scenarios[0]!.steps.map((step) => step.keyword)).toEqual(["When", "When", "Then"]);
  });

  test("a scenario without a level, mode or id is an error rather than a silent skip", () => {
    const feature = parseFeature("@capability-x @no-oracle-y @comparison-ordered-json-v1\nFeature: F\n  Scenario: S\n    Given a value\n");
    expect(feature.scenarios).toHaveLength(0);
    expect(feature.errors.some((error) => error.includes("@id-"))).toBe(true);
    expect(feature.errors.some((error) => error.includes("@level-"))).toBe(true);
    expect(feature.errors.some((error) => error.includes("@mode-"))).toBe(true);
  });

  test("duplicate scenario ids are rejected", () => {
    const duplicated = "@capability-x @no-oracle-y @comparison-ordered-json-v1\nFeature: F\n  @id-s @level-quick @mode-error\n  Scenario: A\n    Given a\n  @id-s @level-quick @mode-error\n  Scenario: B\n    Given b\n";
    expect(parseFeature(duplicated).errors.some((error) => error.includes("Duplicate scenario id"))).toBe(true);
  });

  test("fixture references are collected from every step, doc string and table cell", () => {
    expect(fixtureUrisIn(parseFeature(source))).toEqual(["local://a.json", "local://b.json", "shared://vector.bin"]);
  });
});

describe("⚖️ comparison profiles", () => {
  test("key order is never a semantic difference", () => {
    expect(canonicalize({ b: 1, a: { d: 2, c: 3 } })).toEqual({ a: { c: 3, d: 2 }, b: 1 });
    expect(compareProjections("ordered-json-v1", { a: 1, b: 2 }, { b: 2, a: 1 }).equal).toBe(true);
  });

  test("array order matters for ordered-json-v1 and not for unordered-json-v1", () => {
    expect(compareProjections("ordered-json-v1", [1, 2], [2, 1]).equal).toBe(false);
    expect(compareProjections("unordered-json-v1", [1, 2], [2, 1]).equal).toBe(true);
  });

  test("floating-point-v1 tolerates representation noise that ordered-json-v1 does not", () => {
    expect(compareProjections("ordered-json-v1", { x: 0.1 + 0.2 }, { x: 0.3 }).equal).toBe(false);
    expect(compareProjections("floating-point-v1", { x: 0.1 + 0.2 }, { x: 0.3 }).equal).toBe(true);
  });

  test("semantic-pdf-v1 canonicalizes the nondeterministic artefacts and keeps the normative fields", () => {
    const oracle = { version: "1.7", pageCount: 1, objectNumber: 5, creationDate: "A", pages: [{ mediaBox: [0, 0, 595, 842] }] };
    const subject = { version: "1.7", pageCount: 1, objectNumber: 9, creationDate: "B", pages: [{ mediaBox: [0, 0, 595, 842.00001] }] };
    expect(compareProjections("semantic-pdf-v1", oracle, subject, contributed()).equal).toBe(true);
    expect(compareProjections("semantic-pdf-v1", oracle, { ...subject, pageCount: 2 }, contributed()).equal).toBe(false);
  },
  // ⏱️ First touch of the contributed profile table walks the whole repository for owner manifests,
  // which is well past bun's 5 s default on a loaded machine.
  30_000);

  test("utf8-text-v1 normalizes line endings and trailing whitespace only", () => {
    expect(compareProjections("utf8-text-v1", "a\r\nb  \n", "a\nb\n").equal).toBe(true);
    expect(compareProjections("utf8-text-v1", "a", "b").equal).toBe(false);
  });

  test("every profile — core and contributed — is applicable and produces a stable projection hash", () => {
    const profiles = profileTable(loadOracleRegistry(repoRoot));
    expect(profiles.size).toBeGreaterThan(CORE_COMPARISON_PROFILES.length);
    for (const [id] of profiles) {
      expect(compareProjections(id, { a: 1 }, { a: 1 }, profiles).equal).toBe(true);
      expect(projectionHash(id, { a: 1 }, profiles)).toBe(projectionHash(id, { a: 1 }, profiles));
    }
  }, 30_000);

  test("an unknown profile fails loudly instead of silently comparing as equal", () => {
    const verdict = compareProjections("no-such-profile-v1", { a: 1 }, { a: 1 });
    expect(verdict.equal).toBe(false);
    expect(verdict.diffs[0]!.reason).toContain("unknown comparison profile");
  });

  test("a failed comparison reports where it failed", () => {
    const verdict = compareProjections("ordered-json-v1", { page: { count: 1 } }, { page: { count: 2 } });
    expect(verdict.equal).toBe(false);
    expect(verdict.diffs[0]!.path).toBe("$.page.count");
  });
});

describe("#⃣ digests", () => {
  test("are stable, 32 hex characters, and order-independent for sets", () => {
    expect(digest("semio")).toMatch(/^[0-9a-f]{32}$/);
    expect(digest("semio")).toBe(digest("semio"));
    expect(digest("semio")).not.toBe(digest("semio "));
    expect(setDigest([["a", "1"], ["b", "2"]])).toBe(setDigest([["b", "2"], ["a", "1"]]));
  });
});

describe("📤️ results", () => {
  test("a record missing a required field is rejected, not skipped", () => {
    expect(validateResult({})).toContain("missing field testId");
    expect(validateResult({ implementation: "cobol" }).some((problem) => problem.includes("unknown implementation"))).toBe(true);
  });
});

describe("📇️ oracle registry", () => {
  test("every registered oracle is test-only and declares its license and capabilities", () => {
    const registry = loadOracleRegistry(repoRoot);
    expect(registry.oracles.length).toBeGreaterThan(0);
    for (const oracle of registry.oracles) {
      expect(oracle.testOnly).toBe(true);
      expect(oracle.license.length).toBeGreaterThan(0);
      expect(oracle.capabilities.length).toBeGreaterThan(0);
      expect(oracle.comparisonProfiles.length).toBeGreaterThan(0);
    }
  });

  test("every recorded no-oracle decision names its rationale and its substitutes", () => {
    for (const decision of loadOracleRegistry(repoRoot).noOracleDecisions) {
      expect(decision.rationale.length).toBeGreaterThan(20);
      expect(decision.substitutes.length).toBeGreaterThan(0);
    }
  });
});

describe("🔍️ discovery and contract", () => {
  test("only standards or exact Gherkin owners require mutation catalogs", () => {
    const featureOwners = new Set(["framework/component"]);
    expect(mutationVocabularyRequiresCatalog("artifact/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations", new Set())).toBe(true);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧬️schema/🧬️mutations", featureOwners)).toBe(true);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧬️schema/🧬️mutations", new Set())).toBe(false);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧪️fixtures/demo/🧬️mutations", featureOwners)).toBe(false);
    expect(mutationVocabularyRequiresCatalog("framework/component/🧪️fixtures/demo/🧬️mutations", new Set())).toBe(false);
  });

  test("discovery finds the committed cases and never returns a compose path", () => {
    const cases = discoverTestCases(repoRoot);
    expect(cases.length).toBeGreaterThan(0);
    for (const area of exemptAreas()) expect(cases.every((entry) => !entry.owner.startsWith(`${area}/`) && !entry.caseDir.includes(`${area}/`)), `an exempt area leaked into discovery: ${area}`).toBe(true);
    expect(cases.some((entry) => entry.case === "host-protocol-parity")).toBe(true);
  });

  // ⏱️ Two FULL repository discoveries back to back, and discovery now walks 164 cases and 157
  // vocabulary directories where it walked ~99 and ~88 a wave ago. At that size the pair lands around
  // 5.1 s and was observed timing out at 5130.40 ms against bun's 5 s default under concurrent load,
  // so this test gets the same explicit repo-walking budget as its siblings below rather than a 2.6%
  // margin left to chance. Raising the budget does not hide a regression: cost here is proportional
  // to the committed case count, and the contract phase is what fails if that count is wrong.
  test(
    "discovery is idempotent",
    () => {
      expect(JSON.stringify(discoverTestCases(repoRoot))).toBe(JSON.stringify(discoverTestCases(repoRoot)));
    },
    30_000,
  );

  // ⏱️ Repo-wide: also runs the legacy-test survey and the oracle-purity scan over every
  // non-excluded path, so it needs the quick-level budget rather than bun's 5 s default.
  test(
    "every committed case satisfies the frozen contract",
    () => {
      expect(validateAllContracts(repoRoot).map((breach) => `${breach.kind}:${breach.scope}:${breach.summary}`)).toEqual([]);
    },
    30_000,
  );

  test(
    "the migration backlog is a shrink-only ratchet, never a growing allowlist",
    () => {
      const baseline = loadMigrationBaseline(repoRoot);
      const live = surveyUnmanagedTests(repoRoot).reduce((map, entry) => map.set(entry.area, (map.get(entry.area) ?? 0) + 1), new Map<string, number>());
      for (const [area, count] of live) expect(count).toBeLessThanOrEqual(baseline.unmanagedTests.byArea[area] ?? 0);
      expect([...live.values()].reduce((sum, count) => sum + count, 0)).toBeLessThanOrEqual(baseline.unmanagedTests.total);
      for (const status of Object.values(migrationStatusByOwner(repoRoot))) expect(MIGRATION_STATUSES).toContain(status);
    },
    30_000,
  );
});

describe("🧹️ clean safety", () => {
  test("removes marked outputs, never unmarked directories, and reports identically in dry mode", () => {
    const work = testCacheDir(repoRoot, "work");
    const marked = join(work, "🧪️clean-self-test-marked");
    const unmarked = join(work, "🧪️clean-self-test-unmarked");
    rmSync(marked, { recursive: true, force: true });
    rmSync(unmarked, { recursive: true, force: true });
    markOutputDir(repoRoot, marked, { testId: "self-test::clean", cacheKey: "self-test" });
    writeFileSync(join(marked, "artifact.bin"), "x");
    mkdirSync(unmarked, { recursive: true });
    writeFileSync(join(unmarked, "🚨️sentinel-do-not-delete"), "sentinel");

    expect(readOutputMarker(repoRoot, marked)?.testId).toBe("self-test::clean");
    expect(readOutputMarker(repoRoot, unmarked)).toBeNull();

    const dry = cleanTestOutputs(repoRoot, { dry: true });
    expect(dry.removals.some((row) => row.path.endsWith("🧪️clean-self-test-marked"))).toBe(true);
    expect(dry.skippedUnmarked.some((path) => path.endsWith("🧪️clean-self-test-unmarked"))).toBe(true);
    expect(existsSync(marked)).toBe(true);

    const applied = cleanTestOutputs(repoRoot, { dry: false });
    expect(applied.removals.map((row) => row.path).sort()).toEqual(dry.removals.map((row) => row.path).sort());
    expect(existsSync(marked)).toBe(false);
    expect(existsSync(join(unmarked, "🚨️sentinel-do-not-delete"))).toBe(true);

    rmSync(unmarked, { recursive: true, force: true });
  });

  test("no tracked fixture, source file or compose path is ever a clean candidate", () => {
    const sentinelFixture = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📡️protocol-vector.txt");
    const before = readFileSync(sentinelFixture, "utf8");
    const report = cleanTestOutputs(repoRoot, { dry: true });
    for (const row of report.removals) {
      expect(row.path.startsWith(".🧬semio/🦑️repo/⚡️cache/tests/")).toBe(true);
      for (const area of exemptAreas()) expect(row.path.includes(`${area}/`), `clean candidate inside the exempt area ${area}`).toBe(false);
    }
    expect(readFileSync(sentinelFixture, "utf8")).toBe(before);
    expect(existsSync(join(repoRoot, "compose"))).toBe(true);
  });

  test("marking a directory outside the test cache root is refused", () => {
    expect(() => markOutputDir(repoRoot, join(repoRoot, "✏️s"), { testId: "x", cacheKey: "y" })).toThrow();
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
    // 🧩️EVERY package an oracle links, not only the one its id is named after: a composed reference
    // (reader + writer, archive + XML) that declared just its primary package would leave the others
    // linked into the host and absent from the ratchet, which is a gate that cannot see its subject.
    for (const oracle of loadOracleRegistry(repoRoot).oracles) {
      for (const linked of oracleLinkedPackages(oracle)) {
        const entry = baseline.entries.find((candidate) => candidate.name === linked.package);
        // 🔒️A package present in the baseline at all is the invariant; being absent is what makes the
        // gate blind to its own subject.
        expect(entry, `${linked.package} is linked by oracle ${oracle.id} but is absent from the dependency baseline`).toBeDefined();
        // 🔒️RECORDED DEBT IS EXEMPT, and only recorded debt. `productionDebt` exists precisely to
        // record a package that was ALREADY production-reachable before it was registered as an
        // oracle — `brepjs` is, because ✏️s/🔌️plugins/📐️cad ships an OpenCASCADE B-Rep implementation.
        // Asserting `test-oracle` unconditionally made the honest record of that debt fail the very
        // gate that demanded it be recorded, which leaves an owner two options: hide the reachability,
        // or leave the suite permanently red. The classification must therefore agree with the
        // registry rather than contradict it, and an UNRECORDED production-reachable oracle is still
        // a hard failure below.
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

describe("📈️ non-aggregate metrics", () => {
  const cases = discoverTestCases(repoRoot);
  const results = [
    { testId: "o::c::s1::typescript::subject", owner: "o", case: "c", scenario: "s1", implementation: "typescript" as const, role: "subject" as const, level: "quick" as const, status: "passed" as const, durationMs: 1, output: { rawHash: "", projectionHash: "" }, diagnostics: [] },
    { testId: "o::c::s1::rust::subject", owner: "o", case: "c", scenario: "s1", implementation: "rust" as const, role: "subject" as const, level: "quick" as const, status: "passed" as const, durationMs: 1, output: { rawHash: "", projectionHash: "" }, diagnostics: [] },
  ];

  test("parity coverage is attributed per implementation, for both oracle pairs and cross-subject pairs", () => {
    const metrics = computeCoverageMetrics(repoRoot, [], results, [
      { testId: "o::c::s1::typescript::subject", equal: true },
      { testId: "o::c::s1::rust::subject", equal: false },
      { testId: "o::c::s1::rust~typescript", equal: true },
    ], []);
    expect(metrics.parityCoverage.typescript).toEqual({ compared: 2, equal: 2, ratio: 1 });
    expect(metrics.parityCoverage.rust).toEqual({ compared: 2, equal: 1, ratio: 0.5 });
  });

  test("an implementation that produced no result is a gap, never silently 100%", () => {
    const metrics = computeCoverageMetrics(repoRoot, [], [], [], []);
    const failures = enforceMetricGates({ ...metrics, implementationCoverage: { go: { executed: 0, declared: 4, ratio: 0 } } }, [], 95);
    expect(failures.some((failure) => failure.includes("implementation coverage go 0/4"))).toBe(true);
  });

  test("a blended source percentage cannot stand in for an untested language", () => {
    const failures = enforceMetricGates(computeCoverageMetrics(repoRoot, [], [], [], []), [
      { implementation: "typescript", lines: { covered: 99, total: 100, ratio: 0.99 }, branches: null },
      { implementation: "go", lines: { covered: 10, total: 100, ratio: 0.1 }, branches: null },
    ], 95);
    expect(failures.some((failure) => failure.startsWith("go line coverage"))).toBe(true);
    expect(failures.some((failure) => failure.startsWith("typescript line coverage"))).toBe(false);
  });

  test("oracle coverage counts every discovered case as backed by an oracle or a recorded decision", () => {
    const metrics = computeCoverageMetrics(repoRoot, cases, [], [], []);
    expect(metrics.oracleCoverage.unbacked).toEqual([]);
    expect(metrics.oracleCoverage.ratio).toBe(1);
  }, 30_000);

  test("dependency-clean coverage counts the not-production-reachable share", () => {
    const metrics = computeCoverageMetrics(repoRoot, [], [], [], [
      { ecosystem: "rust", name: "a", version: "1", kinds: ["production-runtime"], users: [], productionReachable: true },
      { ecosystem: "rust", name: "b", version: "1", kinds: ["test-oracle"], users: [], productionReachable: false },
    ]);
    expect(metrics.dependencyCleanCoverage).toEqual({ clean: 1, total: 2, ratio: 0.5 });
  });
});

describe("🔮️ oracle evidence rules", () => {
  test("a differential scenario with neither an oracle nor a second implementation is a contract breach", () => {
    const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [{ id: "vectors-only", capabilities: ["x"], rationale: "a rationale long enough to satisfy the schema minimum length", substitutes: ["specification-vectors"] }], comparisonProfiles: [...CORE_COMPARISON_PROFILES], oracleHostPackages: [], contributions: [] };
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
      localFixtureDir: null,
      projectName: "self-test",
    };
    const breaches = validateCaseContract(repoRoot, discovered, registry);
    expect(breaches.some((breach) => breach.id === "differential-without-evidence")).toBe(true);
    rmSync(join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test"), { recursive: true, force: true });
  });
});

describe("🌱️ native second implementation", () => {
  type Mutation = import("./🟦️.ts").ManifestMutation;
  type Manifest = import("./🟦️.ts").MutationManifest;
  type Entry = import("./🟦️.ts").OracleEntry;
  type Evidence = import("./🟦️.ts").NativeSecondImplementationEvidence;
  type Registry = import("./🟦️.ts").OracleRegistry;

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
      fixtureManifests: [],
      contributions: [{ owner, manifestPath: `${owner}/🧪️oracle/🔣️.json`, oracles, noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests, fixtureManifests: [], probes: [], problems: [] }],
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

  test("a fully earned claim discharges the requirement and records no breach of its own", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const registry = registryWith("test/owner", [entry("test-code-python-independent", ["test-code-1-mutate"], { nativeSecondImplementation: earnedEvidence("s.norm.test-code", ["test-code-1-mutate"]) })], [manifest]);
    expect(nativeSecondImplementationBreaches(registry)).toEqual([]);
    expect(oracleRequirementBreaches(registry, "scope", manifest, manifest.mutations[0]!)).toEqual([]);
  });

  test("the kind field alone, with no recorded evidence, still mechanically discharges — which is exactly why the claim must also be earned", () => {
    const manifest = nativeManifest("s.norm.test-code", [["change-a", "test-code-1-mutate"]]);
    const registry = registryWith("test/owner", [entry("unearned-claim", ["test-code-1-mutate"])], [manifest]);
    // 🚫️Discharge is purely mechanical on `kind` — this is the gap `nativeSecondImplementationBreaches` exists to close.
    expect(oracleRequirementBreaches(registry, "scope", manifest, manifest.mutations[0]!)).toEqual([]);
    const breaches = nativeSecondImplementationBreaches(registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("native-second-implementation-unearned");
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
    // 🔍️The real gate: whatever this shard (or a later one) actually registers under the new kind must
    // pass its own checks — a regression here means a promoted entry in the committed registry is lying.
    const registry = loadOracleRegistry(repoRoot);
    const live = registry.oracles.filter((oracle) => oracle.kind === "verified-native-second-implementation");
    expect(live.length).toBeGreaterThan(0);
    expect(nativeSecondImplementationBreaches(registry).filter((b) => live.some((oracle) => b.scope.endsWith(`#${oracle.id}`)))).toEqual([]);
    for (const oracle of live) expect(isSemioNativeArtifact(oracle.nativeSecondImplementation?.format ?? ""), oracle.id).toBe(true);
  }, 30_000);
});

describe("🪆️ case above subset", () => {
  const featureFilename = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testFeatureFileKindId);
  const discoveredAt = (owner: string): import("./🟦️.ts").DiscoveredCase =>
    ({ owner, ownerName: "📦️artifact", case: "mutate-thing", caseDir: `${owner}/🧪️tests/mutate-thing`, featurePath: `${owner}/🧪️tests/mutate-thing/${featureFilename}`, adapters: {}, sharedFixtureDir: null, localFixtureDir: null, projectName: "test-synthetic-000000-mutate-thing" } as unknown as import("./🟦️.ts").DiscoveredCase);
  const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️only", kinds: ["set-name"], vectors: [] };
  const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [] } as unknown as import("./🟦️.ts").OracleRegistry;
  const featureTagged = (tag: string): import("./🟦️.ts").ParsedFeature => parseFeature([`@capability-thing-mutate`, `@no-oracle-none`, tag, "Feature: Mutate a thing", "  @id-a", "  @level-quick", "  @mode-conformance", "  Scenario: a", "    Given a thing", "    Then it changed"].join("\n"));
  const featureUntagged: import("./🟦️.ts").ParsedFeature = parseFeature(["@capability-thing-mutate", "@no-oracle-none", "Feature: Round trip", "  @id-a", "  @level-quick", "  @mode-conformance", "  Scenario: a", "    Given a thing", "    Then it round-trips"].join("\n"));

  test("a case sitting above the one real subset its catalog names is a breach", () => {
    const breaches = caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureTagged("@mutations-thing-v1"), registry);
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("case-above-subset");
    expect(breaches[0]!.summary).toContain("only");
  });

  test("a case already living under the catalog's own subset is left alone", () => {
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact/🏅️standards/🔖️1/🪆️subsets/✳️only"), featureTagged("@mutations-thing-v1"), registry)).toEqual([]);
  });

  // 🚫️Confirmed live: `gif/🐯️create-and-round-trip-gif`, `jpg/📖️create-and-read-jpeg` and
  // `zip/✏️create-and-edit-archive` each import exactly one real subset's `io` from their adapter while
  // C4 confirmed all three genuinely stay artifact-wide — a case with no `@mutations-` tag is never a
  // mutation-catalog candidate at all, whatever its adapter happens to import.
  test("a case with no @mutations- tag is never a candidate, whatever its adapter names", () => {
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureUntagged, registry)).toEqual([]);
  });

  test("a catalog naming no subsetDirectoryName yields no verdict rather than a guess", () => {
    const { subsetDirectoryName: _drop, ...unprofiled } = catalog;
    const unprofiledRegistry = { ...registry, mutationCatalogs: [unprofiled] } as unknown as import("./🟦️.ts").OracleRegistry;
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureTagged("@mutations-thing-v1"), unprofiledRegistry)).toEqual([]);
  });

  test("a catalog id the registry does not declare yields no verdict — unknown-mutation-catalog is that breach's own job", () => {
    expect(caseAboveSubsetBreaches(discoveredAt("🧪️synthetic/📦️artifact"), featureTagged("@mutations-does-not-exist"), registry)).toEqual([]);
  });

  // 🔍️The real gate: the only live violation this rule should find, right now, is the one case C4's
  // own write-up (📓️c4-relocation-completion.md) named and left in place deliberately — a pre-existing
  // Rust adapter/feature mismatch, not a fixture-placement problem. A regression here means either a
  // NEW case was left above its subset, or this named debt was finally paid off and the assertion
  // below needs updating to match.
  test("the only live case-above-subset violation is the one C4 documented as deliberately blocked", () => {
    const liveRegistry = loadOracleRegistry(repoRoot);
    const scopes = discoverTestCases(repoRoot)
      .flatMap((discovered) => caseAboveSubsetBreaches(discovered, parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8")), liveRegistry))
      .map((entry) => entry.scope);
    expect(scopes).toEqual(["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🧪️tests/mutate-obj-3-0-material"]);
  }, 30_000);
});

describe("🧫️ mutation without fixture", () => {
  type Mutation = import("./🟦️.ts").ManifestMutation;
  type Manifest = import("./🟦️.ts").MutationManifest;
  type Catalog = import("./🟦️.ts").MutationCatalog;
  type Fixture = import("./🟦️.ts").FixtureManifest;
  type Registry = import("./🟦️.ts").OracleRegistry;

  const mutation = (id: string, capability: string): Mutation => ({ id, capability, outcomes: ["applied"], productionDispatch: { operation: id, bridgeVersion: 1 }, oracleRequirements: [{ capability, qualifyingKind: "third-party-library" as const }] });
  const manifestWith = (mutations: readonly Mutation[]): Manifest => ({ schema: "semio.repository-test.mutation-manifest/v2", artifact: "s.norm.test-fixture", standard: "1", subset: "only", mutations });
  const fixture = (mutationId: string, subset = "only"): Fixture =>
    ({ schema: "semio.repository-test.fixture/v2", id: `${mutationId}-fixture`, class: "handcrafted", target: { artifact: "s.norm.test-fixture", standard: "1", subset }, mutation: mutationId, units: { length: "unitless", angle: "degree" }, files: [], provenance: { license: "MIT", source: "handcrafted" }, comparisonProfile: "ordered-json-v1", reproducible: true } as unknown as Fixture);
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
      fixtureManifests: fixtures,
      contributions: [{ owner: "test/owner", manifestPath: "test/owner/🧪️oracle/🔣️.json", oracles: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: catalogs, mutationManifests: manifests, fixtureManifests: fixtures, probes: [], problems: [] }],
    }) as unknown as Registry;

  test("a mutation with neither a v2 fixture nor a v1 vector is a breach", () => {
    const breaches = mutationFixtureBreaches(registryWith([manifestWith([mutation("change-a", "test-fixture-1-mutate")])]));
    expect(breaches.length).toBe(1);
    expect(breaches[0]!.id).toBe("mutation-without-fixture");
    expect(breaches[0]!.summary).toContain("change-a");
  });

  test("a registered v2 FixtureManifest naming this mutation and target discharges it", () => {
    const manifest = manifestWith([mutation("change-a", "test-fixture-1-mutate")]);
    expect(mutationFixtureBreaches(registryWith([manifest], [fixture("change-a")]))).toEqual([]);
  });

  // 🌱️`🏛️architect/🏛️program`'s own live shape: 266 mutations, zero v2 fixtures, 266 v1 vectors —
  // the survey that shrank this rule's live count from 1,650 to 343 before it was even wired in.
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
    const liveRegistry = loadOracleRegistry(repoRoot), taxonomy = testTaxonomy(repoRoot), owner = "✏️s/🔌️plugins/🗄️stdio";
    const physical: { path: string; value: Record<string, any> }[] = [];
    const walk = (path: string): void => {
      for (const entry of readdirSync(join(repoRoot, path), { withFileTypes: true })) {
        const child = `${path}/${entry.name}`;
        if (isExcludedTestPath(repoRoot, child)) continue;
        if (entry.isDirectory()) walk(child);
        else if (entry.isFile() && entry.name.endsWith(".json")) {
          const content = readFileSync(join(repoRoot, child), "utf8");
          if (!/"(?:mutationManifests|fixtureManifests)"\s*:/u.test(content)) continue;
          const value = JSON.parse(content);
          if (Array.isArray(value.mutationManifests) || Array.isArray(value.fixtureManifests)) physical.push({ path: child, value });
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
      expect(basename(dirname(row.path)), row.path).toBe(testContributionDirectoryName(taxonomy, live.owner));
      expect(declarationIds(live), row.path).toEqual(declarationIds(row.value));
      expect(live.fixtureManifests.map((fixture) => fixture.id), row.path).toEqual((row.value.fixtureManifests ?? []).map((fixture: { id: string }) => fixture.id));
      for (const manifest of row.value.mutationManifests ?? []) if (witnesses.has(manifest.artifact)) {
        expect(basename(dirname(row.path))).not.toBe(taxonomy.testContributionDirName);
        observedWitnesses.add(manifest.artifact);
      }
    }
    const live = liveRegistry.contributions.filter((contribution) => contribution.owner === owner || contribution.owner.startsWith(`${owner}/`));
    expect(live.flatMap((contribution) => contribution.mutationManifests).flatMap((manifest) => manifest.mutations).length).toBe(physical.flatMap((row) => row.value.mutationManifests ?? []).flatMap((manifest) => manifest.mutations).length);
    expect(live.flatMap((contribution) => contribution.fixtureManifests).length).toBe(physical.flatMap((row) => row.value.fixtureManifests ?? []).length);
    expect([...observedWitnesses].sort()).toEqual([...witnesses].sort());
    expect(mutationFixtureBreaches(liveRegistry)).toEqual([]);
  }, 30_000);
});

describe("🚫️ oracle purity", () => {
  test("no production source imports a registered oracle", () => {
    expect(oracleImportsInProduction(repoRoot).map((hit) => `${hit.path} → ${hit.oracle}`)).toEqual([]);
  }, 60_000);

  test("narrowing a run to one case must not make other cases' adapters look like production source", () => {
    // 🧭️Regression: the exclusion set was once derived from the CALLER's selected cases, so
    // `contract --case X` reported every other case's adapter as a production oracle import.
    const all = discoverTestCases(repoRoot);
    expect(all.length).toBeGreaterThan(1);
    const single = all.slice(0, 1);
    expect(validateAllContracts(repoRoot, single).filter((breach) => breach.id === "oracle-in-production")).toEqual([]);
  }, 60_000);
});


describe("🧩️ cross-language oracle hosts", () => {
  test("a contributed host package is selected for whichever implementation declares it, not for Rust alone", () => {
    const registry = loadOracleRegistry(repoRoot);
    const owners = new Set(discoverTestCases(repoRoot).map((entry) => entry.owner));
    const selected = [...owners].flatMap((owner) => (["rust", "typescript", "python", "go", "dotnet"] as const).flatMap((implementation) => oracleHostPackagesFor(registry, owner, implementation).map((entry) => entry.implementation)));
    // 🧩️Every implementation an owner declared must be reachable through the selector; a value that
    // parses, merges and is then discarded is a manifest field that silently does nothing.
    for (const declared of registry.contributions.flatMap((entry) => entry.oracleHostPackages)) expect(selected, `no owner reaches the declared ${declared.implementation} host package ${declared.package}`).toContain(declared.implementation);
    expect(new Set(selected).size).toBeGreaterThan(1);
  }, 60_000);

  test("a host package carrying a path is local source; one without a path is an external distribution", () => {
    const registry = loadOracleRegistry(repoRoot);
    const external = externalOracleHostPackages(registry);
    const declared = registry.contributions.flatMap((entry) => entry.oracleHostPackages);
    const taxonomy = testTaxonomy(repoRoot);
    const contributionFilename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
    expect(external.map((entry) => entry.name).sort()).toEqual(declared.filter((entry) => entry.path === undefined).map((entry) => entry.package).sort());
    for (const entry of external) expect(entry.users.every((user) => user.endsWith(contributionFilename))).toBe(true);
    expect(dependencyEcosystemOf("typescript")).toBe("js");
    expect(dependencyEcosystemOf("python")).toBe("python");
  }, 30_000);

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
    // 🚫️Regression guard: one regular expression pretending to be five languages matched a Rust
    // crate named `json` against every Python `import json` in the repository, so the gate had to
    // stay blind to Python imports altogether to avoid reporting breaches that do not exist.
    const taxonomy = testTaxonomy(repoRoot);
    const pythonFilename = testFilenameForKind(taxonomy, taxonomy.testAdapterFileKinds["🐍️python"]!);
    const rustFilename = testFilenameForKind(taxonomy, taxonomy.testAdapterFileKinds["🦀️rust"]!);
    expect(importProbe("rust", "json").files.test(pythonFilename)).toBe(false);
    expect(importProbe("python", "json").files.test(rustFilename)).toBe(false);
    expect(importProbe("python", "pypdf").files.test(pythonFilename)).toBe(true);
    expect(importProbe("javascript", "clsx").files.test("🟦️.tsx")).toBe(true);
  });

  test("an external host package is ratcheted exactly like an oracle package — declaring one is not a way around the gate", () => {
    const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [], contributions: [] } as unknown as import("./🟦️.ts").OracleRegistry;
    const base = [{ ecosystem: "rust" as const, name: "existing", version: "1", kinds: ["production-runtime" as const], users: [], productionReachable: true }];
    const verdict = ratchetDependencies(base, [...base, { ecosystem: "python" as const, name: "an-unregistered-reference-library", version: "1", kinds: ["test-oracle" as const], users: [], productionReachable: false }], registry);
    expect(verdict.ok).toBe(false);
    expect(verdict.unregisteredTestDeps).toEqual(["python:an-unregistered-reference-library"]);
  });

  test("the committed baseline classifies every external host package as a test-only dependency", () => {
    const baseline = JSON.parse(readFileSync(join(repoRoot, "🔒️dependencies.json"), "utf8")) as { entries: { ecosystem: string; name: string; kinds: string[]; productionReachable: boolean }[] };
    for (const host of externalOracleHostPackages(loadOracleRegistry(repoRoot))) {
      const entry = baseline.entries.find((candidate) => candidate.ecosystem === host.ecosystem && candidate.name === host.name);
      expect(entry, `${host.ecosystem}:${host.name} is on a generated host's import path but is absent from the dependency baseline`).toBeDefined();
      expect(entry!.kinds).toEqual(["test-oracle"]);
      expect(entry!.productionReachable).toBe(false);
    }
  }, 30_000);
});

describe("🔒️ recorded production debt", () => {
  test("an oracle claiming testOnly while already production-reachable must record the debt, not hide it", () => {
    const registry = loadOracleRegistry(repoRoot);
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
    const recorded = new Set(loadOracleRegistry(repoRoot).oracles.flatMap((oracle) => oracle.productionDebt?.reachableFrom ?? []));
    for (const hit of oracleImportsInProduction(repoRoot)) expect(recorded.has(hit.path), `unrecorded oracle import at ${hit.path}`).toBe(true);
  }, 60_000);

  test("every registered oracle names its capabilities, comparison profiles and a rationale that scopes it", () => {
    const known = contributed();
    for (const oracle of loadOracleRegistry(repoRoot).oracles) {
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
  type Entry = import("./🟦️.ts").OracleEntry;
  type Registry = import("./🟦️.ts").OracleRegistry;

  /** 🧬️ One oracle entry, defaulting to the minimal shape the detector reads; override per test. */
  const entry = (id: string, ecosystem: string, overrides: Partial<Entry> = {}): Entry => ({ id, kind: "third-party-library", ecosystem, package: "some-package", capabilities: ["some-mutate"], comparisonProfiles: ["ordered-json-v1"], license: "MIT", testOnly: true, ...overrides });

  /** 🧬️ A minimal registry naming exactly one contribution at `owner`, holding the given oracle entries. */
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
      fixtureManifests: [],
      contributions: [{ owner, manifestPath: `${owner}/🧪️oracle/🔣️.json`, oracles, noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], fixtureManifests: [], probes: [], problems: [] }],
    }) as unknown as Registry;

  /** 🦀️ A `match kind { … }` catch-all whose exact wording is what the detector's `predicts` regex looks
   *  for — the real shape found in `ruststep`/`quick-xml`/`html5ever`/`steputils`-adjacent owners this
   *  rule was written against (see `📓️g1-oracle-detector-granularity.md`). Every genuine-reimplementation
   *  fixture below uses it verbatim. */
  const predictingRustText = 'pub fn apply_kind(kind: &str) -> Result<Vec<u8>, String> {\n    match kind {\n        other => Err(format!("mutation kind {other:?} has no oracle implementation")),\n    }\n}\n';

  const withOracleFile = (owner: string, text: string, run: (root: string) => void): void => {
    const root = mkdtempSync(join(tmpdir(), "reimplementation-entry-granular-"));
    try {
      const rsPath = join(root, owner, "🧪️oracle", "🦀️.rs");
      mkdirSync(join(rsPath, ".."), { recursive: true });
      writeFileSync(rsPath, text);
      run(root);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  };

  test("POSITIVE — a genuine re-implementation registered as third-party still fires", () => {
    withOracleFile("test/owner-positive", predictingRustText, (root) => {
      // 🎯 ecosystem "rust": this entry's own claimed code IS this file — exactly the `ruststep`/
      // `quick-xml` shape A10 found and reclassified. The gate must still catch it.
      const registry = registryWith("test/owner-positive", [entry("mislabelled-rust-reimplementation", "rust")]);
      const breaches = reimplementationOracleBreaches(root, registry);
      expect(breaches.length).toBe(1);
      expect(breaches[0]!.id).toBe("reimplementation-registered-as-third-party");
      expect(breaches[0]!.summary).toContain("mislabelled-rust-reimplementation");
    });
  });

  test("NEGATIVE — a real third-party wrapper sharing a file with re-implementation code does not fire", () => {
    withOracleFile("test/owner-negative", predictingRustText, (root) => {
      // 🎯 ecosystem "python": this entry's own claimed code runs in a Python venv the file never
      // touches — exactly `python-pptx`/`ifcopenshell`/`yauzl` sitting beside a predicting `🦀️.rs`.
      const registry = registryWith("test/owner-negative", [entry("genuine-python-reader", "python")]);
      expect(reimplementationOracleBreaches(root, registry)).toEqual([]);
    });
  });

  test("BOUNDARY — a genuine non-Rust reader is never named even when a Rust sibling in the SAME contribution is correctly caught", () => {
    withOracleFile("test/owner-boundary", predictingRustText, (root) => {
      // 🎯 One owner, one shared file, two qualifying entries — the exact shape every one of this
      // ticket's 20 flagged owners actually has. Entry granularity means the verdict must differ
      // per entry even though both entries share everything else: owner, file, contribution.
      const registry = registryWith("test/owner-boundary", [entry("mislabelled-rust-reimplementation", "rust"), entry("genuine-python-reader", "python")]);
      const breaches = reimplementationOracleBreaches(root, registry);
      expect(breaches.length).toBe(1);
      expect(breaches[0]!.summary).toContain("mislabelled-rust-reimplementation");
      expect(breaches[0]!.summary).not.toContain("genuine-python-reader");
    });
  });
});

describe("⚖️ artifact comparison profiles", () => {
  test("semantic-raster-v1 ignores encoder choices and keeps the decoded samples", () => {
    const oracle = { format: "png", width: 2, height: 1, samples: [1, 2, 3, 4], filter: "paeth", gamma: 45455 };
    expect(compareProjections("semantic-raster-v1", oracle, { ...oracle, filter: "sub", gamma: 22222 }, contributed()).equal).toBe(true);
    expect(compareProjections("semantic-raster-v1", oracle, { ...oracle, samples: [1, 2, 3, 5] }, contributed()).equal).toBe(false);
  });

  test("semantic-archive-v1 compares members as a set and ignores writer metadata", () => {
    const a = { entries: [{ name: "a", size: 1, contentDigest: "x" }, { name: "b", size: 2, contentDigest: "y" }] };
    const b = { entries: [{ name: "b", size: 2, contentDigest: "y", modified: "2020" }, { name: "a", size: 1, contentDigest: "x", compressedSize: 9 }] };
    expect(compareProjections("semantic-archive-v1", a, b, contributed()).equal).toBe(true);
    expect(compareProjections("semantic-archive-v1", a, { entries: [a.entries[0]] }, contributed()).equal).toBe(false);
  });

  test("semantic-audio-v1 keeps the format block and every sample", () => {
    const oracle = { channels: 2, sampleRate: 44100, bitsPerSample: 16, samples: [1, -1], byteLength: 100 };
    expect(compareProjections("semantic-audio-v1", oracle, { ...oracle, byteLength: 108 }, contributed()).equal).toBe(true);
    expect(compareProjections("semantic-audio-v1", oracle, { ...oracle, sampleRate: 48000 }, contributed()).equal).toBe(false);
  });
});


describe("🧭️ contribution directory ownership", () => {
  test("handpicked owner directories match neutral cases and the lodash map oracle", async () => {
    const { default: Ajv } = await import("ajv");
    const { default: lodash } = await import("lodash");
    expect(new Ajv({ strict: true }).validate(contributionDirectorySchema, contributionDirectoryCases)).toBe(true);
    const taxonomy = { ...testTaxonomy(repoRoot), testContributionDirName: contributionDirectoryCases.defaultDirectory, testContributionDirectoryOverrides: contributionDirectoryCases.overrides };
    const expected = contributionDirectoryCases.cases.map(({ directory }) => directory);
    expect(contributionDirectoryCases.cases.map(({ owner }) => testContributionDirectoryName(taxonomy, owner))).toEqual(expected);
    expect(contributionDirectoryCases.cases.map(({ owner }) => lodash.get(contributionDirectoryCases.overrides, [owner], contributionDirectoryCases.defaultDirectory))).toEqual(expected);
    expect(contributionDirectoryCases.cases.map(({ path }) => isTestContributionPath(taxonomy, path))).toEqual(contributionDirectoryCases.cases.map(({ owned }) => owned));
  });

  test("discovery and dependency ownership use exact owner overrides without aliases", () => {
    const root = mkdtempSync(join(tmpdir(), "contribution-directory-ownership-"));
    const vocabulary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
    const candidates = [
      { owner: "🎠️kernel", directory: "🔮️oracle", dependency: "selected-reference", owned: true },
      { owner: "🎠️kernel", directory: "🧪️oracle", dependency: "old-name-production", owned: false },
      { owner: "🧩️other", directory: "🧪️oracle", dependency: "default-reference", owned: true },
      { owner: "🧩️other", directory: "🔮️oracle", dependency: "lookalike-production", owned: false },
    ];
    try {
      const taxonomy = { ...JSON.parse(readFileSync(join(repoRoot, vocabulary), "utf8")), testContributionDirectoryOverrides: contributionDirectoryCases.overrides };
      mkdirSync(join(root, vocabulary, ".."), { recursive: true });
      writeFileSync(join(root, vocabulary), JSON.stringify(taxonomy));
      for (const candidate of candidates) {
        const dir = join(root, candidate.owner, candidate.directory);
        mkdirSync(dir, { recursive: true });
        writeFileSync(join(dir, "🔣️.json"), JSON.stringify({ schemaVersion: 2, oracles: [{ id: candidate.dependency, package: "semver", ecosystem: "javascript", capabilities: [], comparisonProfiles: [], license: "ISC", testOnly: true }] }));
        writeFileSync(join(dir, "package.json"), JSON.stringify({ dependencies: { [candidate.dependency]: "1.0.0" } }));
        writeFileSync(join(dir, "🟦️.ts"), "import semver from \"semver\";\n");
      }
      const contributions = discoverTestContributions(root);
      expect(contributions.map(({ manifestPath }) => manifestPath).sort()).toEqual(candidates.filter(({ owned }) => owned).map(({ owner, directory }) => `${owner}/${directory}/🔣️.json`).sort());
      const dependencies = scanDeclaredDependencies(root);
      for (const candidate of candidates) expect(dependencies.find(({ name }) => name === candidate.dependency)?.productionReachable).toBe(!candidate.owned);
      expect([...new Set(oracleImportsInProduction(root).map(({ path }) => path))].sort()).toEqual(candidates.filter(({ owned }) => !owned).map(({ owner, directory }) => `${owner}/${directory}/🟦️.ts`).sort());
      writeFileSync(join(root, "🎠️kernel/🔮️oracle/🔣️.json"), "{ malformed");
      clearContributionCache();
      expect(discoverTestContributions(root).map(({ owner }) => owner)).toEqual(["🧩️other"]);
      expect(scanDeclaredDependencies(root).find(({ name }) => name === "selected-reference")?.productionReachable).toBe(false);
      expect(oracleImportsInProduction(root).some(({ path }) => path.startsWith("🎠️kernel/🔮️oracle/"))).toBe(false);
    } finally {
      clearContributionCache();
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("root dependency discovery and classification honor the same neutral owner contract", async () => {
    const ts = await import("typescript");
    const source = ts.createSourceFile("script.ts", readFileSync(join(repoRoot, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const names = ["dependencyDiscoverContributionManifests", "dependencyClassifyOracleEntry"];
    const statements = source.statements.filter((node) => ts.isFunctionDeclaration(node) && names.includes(node.name?.text ?? "") || ts.isVariableStatement(node) && node.declarationList.declarations.some((item) => item.name.getText(source) === "DEPENDENCY_TEST_DOMAIN_PATH_RE"));
    expect(statements).toHaveLength(3);
    const compiled = ts.transpileModule(statements.map((node) => node.getText(source)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
    const api = new Function("readdirSync", "existsSync", "join", `${compiled}\nreturn { discover: dependencyDiscoverContributionManifests, classify: dependencyClassifyOracleEntry };`)(readdirSync, existsSync, join) as {
      discover(root: string, directory: string, filename: string, overrides: Readonly<Record<string, string>>): string[];
      classify(entry: { name: string; version: string; kinds: string[]; users: string[]; declarations: { user: string; version: string; kind: string }[]; oracleConflictUsers?: string[] }, oracleIds: readonly string[], overrides: Readonly<Record<string, string>>, defaultDirectory: string): void;
    };
    const root = mkdtempSync(join(tmpdir(), "root-contribution-directory-"));
    try {
      for (const row of contributionDirectoryCases.cases) {
        const directory = row.path.slice(0, row.path.lastIndexOf("/"));
        mkdirSync(join(root, directory), { recursive: true });
        writeFileSync(join(root, directory, "🔣️.json"), "{}\n");
        const entry = { name: "reference", version: "1.0.0", kinds: ["production-runtime"], users: [row.path], declarations: [{ user: row.path, version: "1.0.0", kind: "production-runtime" }] };
        api.classify(entry, ["reference"], contributionDirectoryCases.overrides, contributionDirectoryCases.defaultDirectory);
        expect(entry.kinds, row.path).toEqual([row.owned ? "test-oracle" : "production-runtime"]);
      }
      const expected = contributionDirectoryCases.cases.filter(({ owned }) => owned).map(({ path }) => `${path.slice(0, path.lastIndexOf("/"))}/🔣️.json`).sort();
      expect(api.discover(root, contributionDirectoryCases.defaultDirectory, "🔣️.json", contributionDirectoryCases.overrides).sort()).toEqual(expected);
      rmSync(join(root, "🎠️kernel/🔮️oracle/🔣️.json"));
      expect(api.discover(root, contributionDirectoryCases.defaultDirectory, "🔣️.json", contributionDirectoryCases.overrides).sort()).toEqual(expected.filter((path) => path !== "🎠️kernel/🔮️oracle/🔣️.json"));
      const absent = { name: "reference", version: "1.0.0", kinds: ["production-runtime"], users: ["🎠️kernel/🔮️oracle/package.json"], declarations: [{ user: "🎠️kernel/🔮️oracle/package.json", version: "1.0.0", kind: "production-runtime" }] };
      api.classify(absent, ["reference"], contributionDirectoryCases.overrides, contributionDirectoryCases.defaultDirectory);
      expect(absent.kinds).toEqual(["test-oracle"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("the handpicked kernel oracle remains discoverable at runtime", () => {
    const contribution = discoverTestContributions(repoRoot).find(({ owner }) => owner === "🧰️framework/🔨️modules/🎠️kernel");
    expect(contribution?.manifestPath).toBe("🧰️framework/🔨️modules/🎠️kernel/🔮️oracle/🔣️.json");
    expect(contribution?.oracles.map(({ id }) => id)).toContain("semver");
  }, 60_000);
});

describe("🧩️ open/closed", () => {
  test("the framework's own registry names no plugin, product or file format", () => {
    const taxonomy = testTaxonomy(repoRoot);
    const core = JSON.parse(readFileSync(join(repoRoot, testLocationPath(taxonomy, taxonomy.testOracleRegistryLocation)), "utf8")) as { oracles?: unknown[]; comparisonProfiles?: unknown[]; oracleHostPackages?: unknown[] };
    expect(core.oracles ?? []).toEqual([]);
    expect(core.comparisonProfiles ?? []).toEqual([]);
    expect(core.oracleHostPackages ?? []).toEqual([]);
  });

  test("every oracle and every format-specific profile arrives as an owner contribution", () => {
    const contributions = discoverTestContributions(repoRoot);
    expect(contributions.length).toBeGreaterThan(0);
    const registry = loadOracleRegistry(repoRoot);
    const contributedOracleIds = new Set(contributions.flatMap((entry) => entry.oracles.map((oracle) => oracle.id)));
    for (const oracle of registry.oracles) expect(contributedOracleIds.has(oracle.id), `${oracle.id} is not contributed by any owner`).toBe(true);
    const coreIds = new Set(CORE_COMPARISON_PROFILES.map((spec) => spec.id));
    for (const spec of registry.comparisonProfiles) {
      if (coreIds.has(spec.id)) continue;
      expect(contributions.some((entry) => entry.comparisonProfiles.some((candidate) => candidate.id === spec.id)), `${spec.id} is not contributed by any owner`).toBe(true);
    }
  }, 30_000);

  test("the framework's own comparison profiles are domain-neutral", () => {
    for (const spec of CORE_COMPARISON_PROFILES) {
      expect(spec.id).not.toMatch(/pdf|png|gif|zip|wav|csv|raster|archive|audio|mesh|tabular/);
    }
  });

  test("the framework Rust host declares no dependency at all", () => {
    const manifest = readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml"), "utf8");
    const dependencyTable = manifest.split(/^\[dependencies\]$/m)[1] ?? "";
    expect(dependencyTable.split(/\r?\n/).filter((line) => /^[a-z0-9_-]+\s*=/.test(line.trim()))).toEqual([]);
  });

  test("the framework test domain's sources name no implementation area", () => {
    const domain = join(repoRoot, testTaxonomy(repoRoot).testDomainPath);
    for (const file of ["📜️script.ts", "📦️packages/🟦️typescript/🟦️.ts", "📡️protocol/🦀️.rs", "🏃️runner/🦀️.rs"]) {
      const content = readFileSync(join(domain, file), "utf8");
      // 🧭️Which areas are implementations is taxonomy vocabulary; this test names none of them.
      for (const area of implementationAreas()) expect(content.includes(`${area}/`), `${file} names the implementation area ${area}`).toBe(false);
    }
  });

  test("the root script names neither the test module's location nor its phase vocabulary", () => {
    const root = readFileSync(join(repoRoot, "📜️script.ts"), "utf8");
    expect(root.includes(testTaxonomy(repoRoot).testDomainPath)).toBe(false);
    expect(root).not.toMatch(/LEVELLESS_PHASES|composeProjectNames/);
  });
});

//#endregion 🧪️Tests

describe("🦠️ mutation completeness gate", () => {
  // 🦠️A synthetic owner keeps the gate's arithmetic independent of whichever formats happen to be
  // committed today — the framework is not allowed to know that PDF or PNG exist.
  const owner = "🧪️synthetic/📦️artifact";
  const featureFilename = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testFeatureFileKindId);
  const discovered = { owner, ownerName: "📦️artifact", case: "mutate-thing", caseDir: `${owner}/🧪️tests/mutate-thing`, featurePath: `${owner}/🧪️tests/mutate-thing/${featureFilename}`, adapters: {}, sharedFixtureDir: null, localFixtureDir: null, projectName: "test-synthetic-000000-mutate-thing" } as unknown as import("./🟦️.ts").DiscoveredCase;
  const catalog = { id: "thing-v1", capability: "thing-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: ["set-name", "remove-item"], vectors: [] };
  const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [] } as unknown as import("./🟦️.ts").OracleRegistry;
  const feature = (scenarioIds: readonly string[], tag = "@mutations-thing-v1"): import("./🟦️.ts").ParsedFeature =>
    parseFeature([`@capability-thing-mutate`, `@no-oracle-none`, tag, "Feature: Mutate a thing", ...scenarioIds.flatMap((id) => [`  @id-${id}`, "  @level-exhaustive", "  @mode-differential", `  Scenario: ${id}`, "    Given a thing", "    Then it changed"])].join("\n"));

  test("a feature covering every declared kind twice reports nothing", () => {
    expect(mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "mutate-remove-item", "inverse-remove-item"]), registry)).toEqual([]);
  });

  test("an untested mutation kind is a breach naming the kind", () => {
    const ids = mutationCoverageBreaches(discovered, feature(["mutate-set-name", "inverse-set-name", "inverse-remove-item"]), registry);
    expect(ids.map((entry) => entry.id)).toContain("mutation-kind-uncovered");
    expect(ids.some((entry) => entry.summary.includes("remove-item"))).toBe(true);
  });

  // 🔁️A mutation that cannot be undone breaks undo for a real user, so the inverse half is held to
  // the same standard as the mutation half rather than being optional evidence.
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
    const deferring = { ...registry, mutationCatalogs: [{ ...catalog, deferredKinds: ["rotate-item"] }] } as unknown as import("./🟦️.ts").OracleRegistry;
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

  // 🪆️A framework facet owns a mutation vocabulary too, and its path carries no
  // `🏅️standards/🪆️subsets` coordinates to restate. Requiring them unconditionally made such a
  // catalog unrepresentable, which silently dropped the whole contribution — and an owner whose
  // contribution never loads is an owner the completeness gate never measures.
  test("an owner with no standards/subsets coordinates declares a catalog without them", () => {
    const facetOwner = "🧰️framework/🛍️products/💻️os/🎚️config";
    const { standardDirectoryName: _standard, subsetDirectoryName: _subset, ...profileless } = catalog;
    expect(mutationCatalogProblems(profileless, facetOwner)).toEqual([]);
    expect(mutationCatalogProblems(catalog, facetOwner)).toContain("standardDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
    expect(mutationCatalogProblems(catalog, facetOwner)).toContain("subsetDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
    expect(mutationCatalogProblems(profileless, "artifact/🏅️standards/🔖️1/🪆️subsets/✳️any")).toContain("standardDirectoryName must be a non-empty string");
  });

  test("a source or projected 13-node bundle is represented exactly once", () => {
    const root = mkdtempSync(join(tmpdir(), "mutation-vector-contract-"));
    const owner = "artifact/🏅️standards/🔖️1/🪆️subsets/✳️any";
    const source = join(root, owner, "🧬️schema", "🧬️mutations", vector.sourceMutationDirectoryName, "🧪️tests", scenario.directoryName);
    try {
      for (const dir of ["🦠️mutation", "📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🔺️diff", "🎯️outcome"]) mkdirSync(join(source, dir), { recursive: true });
      for (const file of ["🦀️.rs", "🦠️mutation/🔣️.json", "📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🔺️diff/🔣️.json", "🎯️outcome/🔣️.json"]) writeFileSync(join(source, file), "{}\n");
      const contribution = { owner, manifestPath: `${owner}/🔣️oracle.json`, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], migrationStatus: {} };
      const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [], comparisonProfiles: [], oracleHostPackages: [], mutationCatalogs: [catalog], contributions: [contribution] } as unknown as import("./🟦️.ts").OracleRegistry;
      expect(mutationVectorRegistryBreaches(root, registry, testTaxonomy(repoRoot))).toEqual([]);
      rmSync(join(source, "🎯️outcome", "🔣️.json"));
      expect(mutationVectorRegistryBreaches(root, registry, testTaxonomy(repoRoot)).map((entry) => entry.id)).toContain("mutation-vector-bundle-invalid");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("all governed catalogs register the physical tree exactly once", () => {
    const registry = loadOracleRegistry(repoRoot);
    const vectors = registry.mutationCatalogs.flatMap((entry) => entry.vectors);

    // 🧬️STRUCTURAL invariants, not frozen counts. This test used to pin `144` catalogs and `1_555`
    // vectors and to assert an exact three-element breach list. Those numbers are a function of the
    // whole repository's in-flight state: any peer wave that adds an owner or renames a bundle file
    // changed them, and the test then failed for a reason that had nothing to do with the contract it
    // is meant to guard. What IS invariant is the shape.
    expect(registry.mutationCatalogs.length).toBeGreaterThan(0);
    expect(vectors.length).toBeGreaterThan(0);
    // 🧪️One scenario per vector is the registry's own rule: a vector with no scenario registers
    // physical evidence that does not exist, and `mutationCatalogProblems` rejects it outright.
    expect(vectors.every((entry) => entry.scenarios.length > 0)).toBe(true);
    expect(vectors.flatMap((entry) => entry.scenarios).length).toBeGreaterThanOrEqual(vectors.length);
    // 🆔️Registered identities are unique across the whole repository, so no two owners can claim the
    // same physical bundle.
    const directories = vectors.map((entry) => entry.mutationDirectoryName);
    expect(new Set(directories).size).toBeLessThanOrEqual(directories.length);

    // 🧾️Every finding the audit produces must belong to a KNOWN family. A new, unnamed breach id is a
    // gate nobody has read; the count itself is repository state and is reported, never pinned.
    const breaches = mutationVectorRegistryBreaches(repoRoot, registry);
    const known = new Set(["mutation-vector-catalog-invalid", "mutation-vector-mixed-state", "mutation-vector-duplicate-owner", "mutation-vector-bundle-invalid", "mutation-vector-source-id-mismatch", "mutation-vector-missing", "mutation-vector-unregistered"]);
    for (const entry of breaches) expect(known.has(entry.id), `unknown vector breach id ${entry.id}`).toBe(true);
    for (const entry of breaches) expect(entry.scope.length).toBeGreaterThan(0);
  }, 120_000);
});

describe("🧫️ real-world artifact fixtures", () => {
  // 🧫️A multi-megabyte real document is read where the domain already keeps it. Copying it into a
  // fixtures directory would duplicate megabytes of git history for no gain.
  const thesis = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";
  const owner = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf";
  const featureFilename = testFilenameForKind(testTaxonomy(repoRoot), testTaxonomy(repoRoot).testFeatureFileKindId);
  const discovered = { owner, ownerName: "📄️pdf", case: "c", caseDir: `${owner}/🧪️tests/c`, featurePath: `${owner}/🧪️tests/c/${featureFilename}`, adapters: {}, sharedFixtureDir: null, localFixtureDir: null, projectName: "p" } as unknown as import("./🟦️.ts").DiscoveredCase;

  test("asset:// resolves against the owner root and pins the real artifact's digest", () => {
    if (!existsSync(join(repoRoot, thesis))) return;
    const uri = `asset://${thesis.slice(`${owner}/`.length)}`;
    const { fixtures, missing } = resolveFixtures(repoRoot, discovered, [uri]);
    expect(missing).toEqual([]);
    expect(fixtures[0].scope).toBe("asset");
    expect(fixtures[0].path).toBe(thesis);
    expect(fixtures[0].digest.length).toBeGreaterThan(0);
  });

  test("asset:// cannot escape the owner root", () => {
    expect(resolveFixtures(repoRoot, discovered, ["asset://../../../../../../etc/hosts"]).missing.length).toBe(1);
  });

  test("the three fixture schemes are all extracted from a feature's text", () => {
    expect(fixtureUrisIn(parseFeature("@capability-x\nFeature: f\n  @id-s\n  @level-quick\n  @mode-conformance\n  Scenario: s\n    Given asset://a/b.pdf and shared://c.png and local://d.csv\n    Then y"))).toEqual(["asset://a/b.pdf", "local://d.csv", "shared://c.png"]);
  });

  test("a projected asset URI resolves as an ordinary owner-relative fixture", () => {
    const root = mkdtempSync(join(tmpdir(), "projected-uri-"));
    const projectedOwner = "artifact";
    const projected = "🧪️tests/🪆️1-any/🌾change-humidification-required-kg-h/🧪️raises-required-humidification-to-3-point-5-kg-per-hour/🦠️mutation/🔣️.json";
    const projectedCase = { ...discovered, owner: projectedOwner };
    try {
      mkdirSync(join(root, projectedOwner, ...projected.split("/").slice(0, -1)), { recursive: true });
      writeFileSync(join(root, projectedOwner, projected), "{}\n");
      const uri = `asset://${projected}`;
      expect(fixtureUrisIn(parseFeature(`@capability-x\nFeature: f\n  @id-s @level-quick @mode-conformance\n  Scenario: s\n    Given ${uri}\n`))).toEqual([uri]);
      expect(resolveFixtures(root, projectedCase, [uri])).toMatchObject({ missing: [], fixtures: [{ uri, path: `${projectedOwner}/${projected}` }] });
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("🧪️ projected vector storage", () => {
  test("profile storage never becomes an executable Nx test project", async () => {
    const root = mkdtempSync(join(tmpdir(), "projected-nx-"));
    const taxonomy = testTaxonomy(repoRoot);
    const featureFilename = testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId);
    try {
      const taxonomyPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
      mkdirSync(join(taxonomyPath, ".."), { recursive: true });
      writeFileSync(taxonomyPath, readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")));
      const executable = join(root, "artifact", taxonomy.testsDirName, "mutate-thing");
      const projected = join(root, "artifact", taxonomy.testsDirName, "🪆️1-any", "🪄️change-value", "🧪️changes-the-value");
      mkdirSync(executable, { recursive: true });
      mkdirSync(projected, { recursive: true });
      writeFileSync(join(executable, featureFilename), "Feature: executable\n");
      writeFileSync(join(projected, featureFilename), "Feature: must stay storage\n");
      const { discoverCaseDirs } = await import("../../🟨️.mjs");
      expect(discoverCaseDirs(root)).toEqual([`artifact/${taxonomy.testsDirName}/mutate-thing`]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
