//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, sep } from "node:path";

/** 🧭️ Repo-relative, forward-slashed path — the shape every discovered record carries. */
const relativeToRepo = (root: string, target: string): string => relative(root, target).split(sep).join("/");
import { COMPARISON_PROFILES, canonicalize, oracleImportsInProduction, computeCoverageMetrics, enforceMetricGates, validateCaseContract, cleanTestOutputs, compareProjections, digest, discoverTestCases, fixtureUrisIn, isExcludedTestPath, loadMigrationBaseline, loadOracleRegistry, markOutputDir, parseFeature, MIGRATION_STATUSES, projectionHash, ratchetDependencies, readOutputMarker, repoRootFromHere, setDigest, surveyUnmanagedTests, testCacheDir, testProjectName, testTaxonomy, validateAllContracts, validateResult } from "./📦️index.ts";
//#endregion 🔌️Adapters

const repoRoot = repoRootFromHere();

//#region 🧪️Tests
describe("🔣️ contract", () => {
  test("the taxonomy exposes every frozen test vocabulary key", () => {
    const taxonomy = testTaxonomy(repoRoot);
    expect(taxonomy.testsDirName).toBe("🧪️tests");
    expect(taxonomy.testFeatureFilename).toBe("component.feature");
    expect(Object.values(taxonomy.testAdapterFilenames).sort()).toEqual(["🐍️component.py", "🐹️component.go", "🔷️component.cs", "🟦️component.ts", "🦀️component.rs"].sort());
    expect(taxonomy.testOutputChildDirs).toEqual(["work", "hosts", "oracles", "results", "diffs", "reports"]);
  });

  test("compose/ is excluded by the discovery library itself, not by a workflow path filter", () => {
    expect(isExcludedTestPath(repoRoot, "compose")).toBe(true);
    expect(isExcludedTestPath(repoRoot, "compose/client/lib/rs")).toBe(true);
    expect(isExcludedTestPath(repoRoot, "✏️s/🔌️plugins/🗄️stdio")).toBe(false);
  });

  test("no legacy area is excluded — only compose is a permanent exemption", () => {
    for (const legacy of ["♻️mit-bestand", "🌎️hub", "✏️s/🔨️modules"]) expect(isExcludedTestPath(repoRoot, legacy)).toBe(false);
  });

  test("project names are deterministic and CLI-safe", () => {
    const name = testProjectName("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf", "create-minimal-pdf");
    expect(name).toBe(testProjectName("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf", "create-minimal-pdf"));
    expect(name).toMatch(/^[a-z0-9-]+$/);
  });
});

describe("🥒️ feature profile", () => {
  const source = `@capability-thing @oracle-pdf-writer @comparison-semantic-pdf-v1
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
    expect(compareProjections("semantic-pdf-v1", oracle, subject).equal).toBe(true);
    expect(compareProjections("semantic-pdf-v1", oracle, { ...subject, pageCount: 2 }).equal).toBe(false);
  });

  test("utf8-text-v1 normalizes line endings and trailing whitespace only", () => {
    expect(compareProjections("utf8-text-v1", "a\r\nb  \n", "a\nb\n").equal).toBe(true);
    expect(compareProjections("utf8-text-v1", "a", "b").equal).toBe(false);
  });

  test("every declared profile is implemented and produces a stable projection hash", () => {
    for (const profile of COMPARISON_PROFILES) {
      expect(compareProjections(profile, { a: 1 }, { a: 1 }).equal).toBe(true);
      expect(projectionHash(profile, { a: 1 })).toBe(projectionHash(profile, { a: 1 }));
    }
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
  test("discovery finds the committed cases and never returns a compose path", () => {
    const cases = discoverTestCases(repoRoot);
    expect(cases.length).toBeGreaterThan(0);
    expect(cases.every((entry) => !entry.owner.startsWith("compose/") && !entry.caseDir.includes("compose/"))).toBe(true);
    expect(cases.some((entry) => entry.case === "host-protocol-parity")).toBe(true);
  });

  test("discovery is idempotent", () => {
    expect(JSON.stringify(discoverTestCases(repoRoot))).toBe(JSON.stringify(discoverTestCases(repoRoot)));
  });

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
      for (const status of Object.values(baseline.ownerStatus)) expect(MIGRATION_STATUSES).toContain(status);
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
    const sentinelFixture = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📄️protocol-vector.txt");
    const before = readFileSync(sentinelFixture, "utf8");
    const report = cleanTestOutputs(repoRoot, { dry: true });
    for (const row of report.removals) {
      expect(row.path.startsWith(".🧬semio/🦑️repo/⚡️cache/tests/")).toBe(true);
      expect(row.path.includes("compose/")).toBe(false);
    }
    expect(readFileSync(sentinelFixture, "utf8")).toBe(before);
    expect(existsSync(join(repoRoot, "compose"))).toBe(true);
  });

  test("marking a directory outside the test cache root is refused", () => {
    expect(() => markOutputDir(repoRoot, join(repoRoot, "✏️s"), { testId: "x", cacheKey: "y" })).toThrow();
  });
});

describe("🔒️ dependency ratchet", () => {
  const registry = { schemaVersion: 1, oracles: [{ id: "pdf-writer", ecosystem: "rust", package: "pdf-writer", capabilities: ["pdf-create"], comparisonProfiles: ["semantic-pdf-v1" as const], license: "MIT", testOnly: true as const }], noOracleDecisions: [] };
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
      expect(entry.users.every((user) => !user.startsWith("compose/"))).toBe(true);
    }
    for (const oracle of loadOracleRegistry(repoRoot).oracles) {
      const entry = baseline.entries.find((candidate) => candidate.name === oracle.package);
      expect(entry?.kinds).toEqual(["test-oracle"]);
      expect(entry?.productionReachable).toBe(false);
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
    const registry = { schemaVersion: 1, oracles: [], noOracleDecisions: [{ id: "vectors-only", capabilities: ["x"], rationale: "a rationale long enough to satisfy the schema minimum length", substitutes: ["specification-vectors"] }] };
    const feature = "@capability-x @no-oracle-vectors-only @comparison-ordered-json-v1\nFeature: F\n  @id-s @level-quick @mode-differential\n  Scenario: S\n    Given a value\n";
    const dir = join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test", "🧪️tests", "differential-without-oracle");
    rmSync(join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test"), { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "component.feature"), feature);
    writeFileSync(join(dir, "🟦️component.ts"), "export default {};\n");
    const discovered = {
      owner: relativeToRepo(repoRoot, join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test")),
      ownerName: "🧪️contract-self-test",
      case: "differential-without-oracle",
      caseDir: relativeToRepo(repoRoot, dir),
      featurePath: `${relativeToRepo(repoRoot, dir)}/component.feature`,
      adapters: { typescript: `${relativeToRepo(repoRoot, dir)}/🟦️component.ts` },
      sharedFixtureDir: null,
      localFixtureDir: null,
      projectName: "self-test",
    };
    const breaches = validateCaseContract(repoRoot, discovered, registry);
    expect(breaches.some((breach) => breach.id === "differential-without-evidence")).toBe(true);
    rmSync(join(testCacheDir(repoRoot, "work"), "🧪️contract-self-test"), { recursive: true, force: true });
  });
});


describe("🚫️ oracle purity", () => {
  test("no production source imports a registered oracle", () => {
    expect(oracleImportsInProduction(repoRoot).map((hit) => `${hit.path} → ${hit.oracle}`)).toEqual([]);
  }, 60_000);

  test("narrowing a run to one case must not make other cases' adapters look like production source", () => {
    // 🧭️Regression: the exclusion set was once derived from the CALLER's selected cases, so
    // `contract --case X` reported every other case's adapter as a production oracle import.
    const single = discoverTestCases(repoRoot).filter((entry) => entry.case === "compile-style-variants");
    expect(single).toHaveLength(1);
    expect(validateAllContracts(repoRoot, single).filter((breach) => breach.id === "oracle-in-production")).toEqual([]);
  }, 60_000);
});

//#endregion 🧪️Tests
