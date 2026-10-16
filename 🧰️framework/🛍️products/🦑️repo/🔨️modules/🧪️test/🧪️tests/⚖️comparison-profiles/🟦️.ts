/** ⚖️ Test platform: comparison laws
 *
 * The feature profile, comparison profiles, digests, results, pipelines, artifact profiles and non-aggregate metric laws of the test platform.
 */

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { CORE_COMPARISON_PROFILES, profileTable, canonicalize, computeCoverageMetrics, enforceMetricGates, compareProjections, digest, testInputUrisIn, projectionHash, setDigest, executePipeline, pipelineRoleArtifacts, testCacheDir, validateResult } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { parseFeature } from "@semio-tech/framework/test/gherkin";
import { contributed, repoCases, repoRegistry, repoRoot } from "../🧰️repository-scan/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧪️Tests
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

  test("an outline row without a kebab-case id cell is an error rather than an index-named scenario", () => {
    const outline = (header: string, row: string): string => `@capability-x @no-oracle-y @comparison-ordered-json-v1\nFeature: F\n  @id-read @level-quick @mode-differential\n  Scenario Outline: Read <${header}>\n    Given <${header}>\n    Examples:\n      | ${header} |\n      | ${row} |\n`;
    for (const [header, row] of [["format", "stl"], ["id", "600FF"], ["id", ""]] as const) {
      const feature = parseFeature(outline(header, row));
      expect(feature.scenarios).toHaveLength(0);
      expect(feature.errors.some((error) => error.includes("needs a kebab-case `id` cell"))).toBe(true);
    }
    expect(parseFeature(outline("id", "stl")).scenarios.map((scenario) => scenario.id)).toEqual(["read-stl"]);
  });

  test("duplicate scenario ids are rejected", () => {
    const duplicated = "@capability-x @no-oracle-y @comparison-ordered-json-v1\nFeature: F\n  @id-s @level-quick @mode-error\n  Scenario: A\n    Given a\n  @id-s @level-quick @mode-error\n  Scenario: B\n    Given b\n";
    expect(parseFeature(duplicated).errors.some((error) => error.includes("Duplicate scenario id"))).toBe(true);
  });

  test("fixture references are collected from every step, doc string and table cell", () => {
    expect(testInputUrisIn(parseFeature(source))).toEqual(["local://a.json", "local://b.json", "shared://vector.bin"]);
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
    const profiles = profileTable(repoRegistry);
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

describe("📈️ non-aggregate metrics", () => {
  const cases = repoCases;
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

describe("⚖️ comparison pipelines", () => {
  test("a pipeline runs its probes over the pair's artifacts by role, and fails on an unproduced role or an invalid report", () => {
    const dir = join(testCacheDir(repoRoot, "work"), "⚖️pipeline-self-test");
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
    const probeScript = join(dir, "probe.ts");
    writeFileSync(probeScript, 'const [probe, ...rest] = process.argv.slice(2);\nconst inputs = rest.filter((_, index) => rest[index - 1] === "--input");\nif (probe === "broken") { console.log("not json"); process.exit(1); }\nconst texts = inputs.map((path) => require("node:fs").readFileSync(path, "utf8"));\nconsole.log(JSON.stringify({ schema: "semio.repository-test.probe-report/v2", probe, status: "ok", measurements: { equal: texts.every((text) => text === texts[0]), count: texts.length } }, null, 2));\n');
    const expected = join(dir, "expected.txt");
    const same = join(dir, "same.txt");
    const other = join(dir, "other.txt");
    writeFileSync(expected, "a");
    writeFileSync(same, "a");
    writeFileSync(other, "b");
    const probe = (id: string) => ({ id, kind: "external-process" as const, ecosystem: "javascript", package: "self-test", capabilities: ["self.test"], outputSchema: "semio.repository-test.probe-report/v2", deterministic: true, license: "MIT", testOnly: true as const, command: ["bun", probeScript, id], qualification: { status: "qualified" as const, evidence: "self-test" } });
    const probes = new Map([["text-compare", probe("text-compare")], ["broken", probe("broken")]]);
    type Stage = { probe: string; inputs: string[]; assertions: Record<string, unknown> };
    const pipeline = { id: "self-test-v1", stages: [{ probe: "text-compare", inputs: ["expected-text", "actual-text"], assertions: { equal: true, count: 2 } }] as Stage[] };
    const run = (artifacts: Record<string, string>, stages: Stage[] = pipeline.stages) => executePipeline(repoRoot, { ...pipeline, stages }, probes, new Map(Object.entries(artifacts)), { budgetMs: 60_000 });
    const equal = run({ "expected-text": expected, "actual-text": same });
    expect(equal.problems).toEqual([]);
    expect(equal.verdict.equal).toBe(true);
    expect(run({ "expected-text": expected, "actual-text": other }).verdict.equal).toBe(false);
    const unproduced = run({ "expected-text": expected });
    expect(unproduced.verdict.equal).toBe(false);
    expect(unproduced.problems.join("\n")).toContain("no artifact for role(s) actual-text");
    const invalid = run({ "expected-text": expected, "actual-text": same }, [{ probe: "broken", inputs: ["expected-text"], assertions: {} }]);
    expect(invalid.verdict.equal).toBe(false);
    expect(invalid.problems.join("\n")).toContain("without a valid report");
    expect(pipelineRoleArtifacts([{ role: "oracle", artifacts: [{ role: "x", path: "a", mediaType: "text/plain", sha256: "0" }] }, { role: "subject", artifacts: [{ role: "x", path: "b", mediaType: "text/plain", sha256: "0" }] }]).problems).toEqual(["artifact role x is produced by more than one result"]);
    rmSync(dir, { recursive: true, force: true });
  }, 120_000);
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
//#endregion 🧪️Tests
