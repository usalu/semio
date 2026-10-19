/** 📈️ Reporting
 *
 * Run summaries, JUnit and diff rendering, coverage metrics and the coverage matrix.
 */

//#region 🔌️Adapters
import { parseFeature } from "@semio-tech/framework/test/gherkin";
import { type ComparisonProfile, type Implementation, type TestEvidence, type EvidenceClass, type MutationOutcomeClass, type PlatformId, currentPlatform, EVIDENCE_CLASSES } from "@semio-tech/framework/test/adapter";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { type TestLevel } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { type ComparisonVerdict } from "../🟰️comparison/🟦️.ts";
import { type DiscoveredCase, repoRootFromHere } from "../🧭️case-discovery/🟦️.ts";
import { isWildcardSubsetFor, subsetCoordinate, surfaceCoordinate } from "../🪆️subset-policy/🟦️.ts";
import { engineFamilyId, isQualifyingOracleKind, type OracleKind, owningSubsetOf, type RuntimeMutationInventory } from "../📑️mutation-manifest/🟦️.ts";
import { type TestResult } from "../📤️results/🟦️.ts";
import { loadOracleRegistry, type OracleRegistry } from "../🤝️contributions/🟦️.ts";
import { testEvidenceProblems } from "../🧷️fixture-evidence/🟦️.ts";
import { type ClassifiedDependency } from "../🔒️oracle-dependencies/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📊️Reports
/** 📊️ Aggregated view of one run, per level, implementation and owner. */
export type RunSummary = Readonly<{
  level: TestLevel;
  cases: number;
  scenarios: number;
  executed: number;
  passed: number;
  failed: number;
  errored: number;
  byImplementation: Readonly<Record<string, { passed: number; failed: number; errored: number }>>;
  parity: readonly { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number }[];
  problems: readonly string[];
  /** 🔧️ Cases whose third-party CLI reference is not installed here, each with the command it lacked. Visible, never a pass. */
  oracleUnavailable: readonly { caseDir: string; oracle: string; missing: readonly string[] }[];
}>;

/** 📊️ Builds the owned run summary from raw host results plus the parity verdicts. */
export function summarizeRun(level: TestLevel, cases: number, scenarios: number, results: readonly TestResult[], parity: RunSummary["parity"], problems: readonly string[], oracleUnavailable: RunSummary["oracleUnavailable"] = []): RunSummary {
  const byImplementation: Record<string, { passed: number; failed: number; errored: number }> = {};
  for (const result of results) {
    const bucket = (byImplementation[result.implementation] ??= { passed: 0, failed: 0, errored: 0 });
    bucket[result.status] += 1;
  }
  return {
    level,
    cases,
    scenarios,
    executed: results.length,
    passed: results.filter((r) => r.status === "passed").length,
    failed: results.filter((r) => r.status === "failed").length,
    errored: results.filter((r) => r.status === "errored").length,
    byImplementation,
    parity,
    problems,
    oracleUnavailable,
  };
}

/** 📊️ Renders JUnit XML so any CI surface can consume the run without knowing this protocol. */
export function renderJUnit(results: readonly TestResult[]): string {
  const escape = (value: string): string => value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
  const suites = new Map<string, TestResult[]>();
  for (const result of results) {
    const key = `${result.owner}::${result.case}`;
    if (!suites.has(key)) suites.set(key, []);
    suites.get(key)!.push(result);
  }
  const body = [...suites.entries()]
    .map(([name, rows]) => {
      const cases = rows
        .map((row) => {
          const inner = row.status === "passed" ? "" : `<failure message="${escape(row.diagnostics.map((d) => d.message).join("; ") || row.status)}">${escape(row.diagnostics.map((d) => d.detail ?? d.message).join("\n"))}</failure>`;
          return `    <testcase classname="${escape(name)}" name="${escape(`${row.scenario}::${row.implementation}::${row.role}`)}" time="${(row.durationMs / 1000).toFixed(3)}">${inner}</testcase>`;
        })
        .join("\n");
      return `  <testsuite name="${escape(name)}" tests="${rows.length}" failures="${rows.filter((r) => r.status !== "passed").length}">\n${cases}\n  </testsuite>`;
    })
    .join("\n");
  return `<?xml version="1.0" encoding="UTF-8"?>\n<testsuites>\n${body}\n</testsuites>\n`;
}

/** 📊️ Renders a human-readable semantic diff for one failed parity comparison. */
export function renderDiff(testIdentity: string, verdict: ComparisonVerdict): string {
  const lines = [`# ${testIdentity}`, `profile: ${verdict.profile}`, `equal: ${verdict.equal}`, ""];
  for (const diff of verdict.diffs.slice(0, 200)) lines.push(`${diff.path}: ${diff.reason}\n  oracle:  ${JSON.stringify(diff.oracle)}\n  subject: ${JSON.stringify(diff.subject)}`);
  if (verdict.diffs.length > 200) lines.push(`… ${verdict.diffs.length - 200} further differences omitted`);
  return `${lines.join("\n")}\n`;
}
//#endregion 📊️Reports

//#region 📈️Metrics
/**
 * 📈️ The seven non-aggregate metrics of §13. A single well-covered implementation must never be able
 * to mask an untested one in another language, so every metric that CAN be per-implementation is.
 */
export type CoverageMetrics = Readonly<{
  /** 🎬️ Feature scenarios that were actually executed, over scenarios declared. */
  scenarioCoverage: { executed: number; declared: number; ratio: number; missing: readonly string[] };
  /** 🌐️ Claimed implementations that produced a result, over implementations claimed by an adapter. */
  implementationCoverage: Readonly<Record<string, { executed: number; declared: number; ratio: number }>>;
  /** ⚖️ Executed subject results that passed a parity comparison, per implementation. */
  parityCoverage: Readonly<Record<string, { compared: number; equal: number; ratio: number }>>;
  /** 🔮️ Cases backed by an approved oracle or an approved no-oracle decision, over cases. */
  oracleCoverage: { backed: number; cases: number; ratio: number; unbacked: readonly string[] };
  /** 🔒️ External dependencies that are not production-reachable, over external dependencies. */
  dependencyCleanCoverage: { clean: number; total: number; ratio: number };
}>;

/** 📈️ Per-implementation source coverage, so an aggregate percentage cannot hide an untested language. */
export type ImplementationCoverage = Readonly<{ implementation: string; lines: { covered: number; total: number; ratio: number }; branches: { covered: number; total: number; ratio: number } | null }>;

function ratio(part: number, whole: number): number {
  return whole === 0 ? 1 : Number((part / whole).toFixed(6));
}

/**
 * 📈️ Computes the run's non-aggregate metrics from the discovered cases and the emitted results.
 * Adapters, generated hosts, fixtures and oracle implementations are not repository source and are
 * therefore never counted as covered production code — only the claimed implementations are.
 */
export function computeCoverageMetrics(repoRoot: string, cases: readonly DiscoveredCase[], results: readonly TestResult[], parity: readonly { testId: string; equal: boolean }[], baseline: readonly ClassifiedDependency[]): CoverageMetrics {
  const registry = loadOracleRegistry(repoRoot);

  const declaredScenarios = new Set<string>();
  const backedCases: string[] = [];
  const unbackedCases: string[] = [];
  const declaredByImplementation = new Map<string, Set<string>>();
  for (const discovered of cases) {
    const feature = parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8"));
    const backed = (feature.oracle !== null && registry.oracles.some((entry) => entry.id === feature.oracle)) || (feature.noOracleDecision !== null && registry.noOracleDecisions.some((entry) => entry.id === feature.noOracleDecision));
    (backed ? backedCases : unbackedCases).push(discovered.caseDir);
    for (const scenario of feature.scenarios) {
      declaredScenarios.add(`${discovered.owner}::${discovered.case}::${scenario.id}`);
      for (const implementation of Object.keys(discovered.adapters)) {
        if (!declaredByImplementation.has(implementation)) declaredByImplementation.set(implementation, new Set());
        declaredByImplementation.get(implementation)!.add(`${discovered.owner}::${discovered.case}::${scenario.id}`);
      }
    }
  }

  const executedScenarios = new Set(results.map((result) => `${result.owner}::${result.case}::${result.scenario}`));
  const executedByImplementation = new Map<string, Set<string>>();
  for (const result of results) {
    if (!executedByImplementation.has(result.implementation)) executedByImplementation.set(result.implementation, new Set());
    executedByImplementation.get(result.implementation)!.add(`${result.owner}::${result.case}::${result.scenario}`);
  }

  const implementationCoverage: Record<string, { executed: number; declared: number; ratio: number }> = {};
  for (const [implementation, declared] of declaredByImplementation) {
    const executed = executedByImplementation.get(implementation)?.size ?? 0;
    implementationCoverage[implementation] = { executed, declared: declared.size, ratio: ratio(executed, declared.size) };
  }

  const parityByImplementation: Record<string, { compared: number; equal: number; ratio: number }> = {};
  for (const verdict of parity) {
    // 🧭️A verdict id is either a stable test id (`…::<implementation>::subject`) or a cross-subject
    // pair (`…::<a>~<b>`); both name every implementation they speak for.
    const implementations = verdict.testId.includes("~") ? (verdict.testId.split("::").pop() ?? "").split("~") : [verdict.testId.split("::").at(-2) ?? ""];
    for (const implementation of implementations.filter(Boolean)) {
      const bucket = (parityByImplementation[implementation] ??= { compared: 0, equal: 0, ratio: 1 });
      bucket.compared += 1;
      if (verdict.equal) bucket.equal += 1;
      bucket.ratio = ratio(bucket.equal, bucket.compared);
    }
  }

  const external = baseline.filter((entry) => entry.kinds.length > 0);
  return {
    scenarioCoverage: { executed: executedScenarios.size, declared: declaredScenarios.size, ratio: ratio(executedScenarios.size, declaredScenarios.size), missing: [...declaredScenarios].filter((id) => !executedScenarios.has(id)).sort() },
    implementationCoverage,
    parityCoverage: parityByImplementation,
    oracleCoverage: { backed: backedCases.length, cases: cases.length, ratio: ratio(backedCases.length, cases.length), unbacked: unbackedCases.sort() },
    dependencyCleanCoverage: { clean: external.filter((entry) => !entry.productionReachable).length, total: external.length, ratio: ratio(external.filter((entry) => !entry.productionReachable).length, external.length) },
  };
}

/**
 * 📈️ Enforces the non-aggregate gates. Scenario, implementation, parity and oracle coverage must all
 * be complete for the selected scope; source coverage is enforced per implementation against the
 * repository threshold, never as one blended percentage.
 */
export function enforceMetricGates(metrics: CoverageMetrics, perImplementation: readonly ImplementationCoverage[], sourceThreshold: number): string[] {
  const failures: string[] = [];
  if (metrics.scenarioCoverage.ratio < 1) failures.push(`scenario coverage ${metrics.scenarioCoverage.executed}/${metrics.scenarioCoverage.declared}: ${metrics.scenarioCoverage.missing.slice(0, 10).join(", ")}`);
  for (const [implementation, row] of Object.entries(metrics.implementationCoverage)) if (row.ratio < 1) failures.push(`implementation coverage ${implementation} ${row.executed}/${row.declared} — a claimed implementation did not run`);
  for (const [implementation, row] of Object.entries(metrics.parityCoverage)) if (row.ratio < 1) failures.push(`parity coverage ${implementation} ${row.equal}/${row.compared}`);
  if (metrics.oracleCoverage.ratio < 1) failures.push(`oracle coverage ${metrics.oracleCoverage.backed}/${metrics.oracleCoverage.cases}: ${metrics.oracleCoverage.unbacked.join(", ")}`);
  for (const row of perImplementation) {
    if (row.lines.ratio * 100 < sourceThreshold) failures.push(`${row.implementation} line coverage ${(row.lines.ratio * 100).toFixed(2)}% below ${sourceThreshold}%`);
    if (row.branches !== null && row.branches.ratio * 100 < sourceThreshold) failures.push(`${row.implementation} branch coverage ${(row.branches.ratio * 100).toFixed(2)}% below ${sourceThreshold}%`);
  }
  return failures;
}

/** 📈️ Renders the metrics as the repository dashboard row set of §22. */
export function formatMetrics(metrics: CoverageMetrics, perImplementation: readonly ImplementationCoverage[]): string {
  const lines = [
    `[metrics] scenario coverage      ${metrics.scenarioCoverage.executed}/${metrics.scenarioCoverage.declared} (${(metrics.scenarioCoverage.ratio * 100).toFixed(1)}%)`,
    `[metrics] oracle coverage        ${metrics.oracleCoverage.backed}/${metrics.oracleCoverage.cases} (${(metrics.oracleCoverage.ratio * 100).toFixed(1)}%)`,
    `[metrics] dependency-clean       ${metrics.dependencyCleanCoverage.clean}/${metrics.dependencyCleanCoverage.total} (${(metrics.dependencyCleanCoverage.ratio * 100).toFixed(1)}%)`,
  ];
  for (const [implementation, row] of Object.entries(metrics.implementationCoverage).sort()) lines.push(`[metrics] implementation ${implementation.padEnd(11)} ${row.executed}/${row.declared} scenarios`);
  for (const [implementation, row] of Object.entries(metrics.parityCoverage).sort()) lines.push(`[metrics] parity         ${implementation.padEnd(11)} ${row.equal}/${row.compared} comparisons equal`);
  for (const row of perImplementation) lines.push(`[metrics] source         ${row.implementation.padEnd(11)} lines ${(row.lines.ratio * 100).toFixed(2)}%${row.branches === null ? " (branches not reported)" : ` branches ${(row.branches.ratio * 100).toFixed(2)}%`}`);
  return lines.join("\n");
}
//#endregion 📈️Metrics

//#region 📈️CoverageV2
/** 📈️ Every dimension the release gate reads. A gate that reads nothing else cannot be satisfied by an aggregate. */
export const COVERAGE_DIMENSIONS = [
  "runtimeMutationCoverage",
  "subsetOwnershipCoverage",
  "externalOracleCoverage",
  "oracleEvidenceCoverage",
  "oracleCapabilityCoverage",
  "productionBridgeCoverage",
  "fixtureClassCoverage",
  "fixtureProvenanceCoverage",
  "expectedOutcomeCoverage",
  "inverseCoverage",
  "metamorphicCoverage",
  "comparisonMetricCoverage",
  "determinismCoverage",
  "implementationCoverage",
  "platformCoverage",
  "fixtureReproducibilityCoverage",
  "dependencyIsolationCoverage",
] as const;
export type CoverageDimension = (typeof COVERAGE_DIMENSIONS)[number];

/** 📈️ Dimensions that must be exactly 100% to release. */
export const RELEASE_GATED_DIMENSIONS: readonly CoverageDimension[] = ["runtimeMutationCoverage", "subsetOwnershipCoverage", "externalOracleCoverage", "oracleEvidenceCoverage", "productionBridgeCoverage", "fixtureProvenanceCoverage", "dependencyIsolationCoverage"];

/** 📊️ One report row, keyed by the FULL coordinate. Reporting at artifact level is what this shape prevents. */
export type CoverageRow = Readonly<{
  baselineSha: string;
  artifact: string;
  standard: string;
  subset: string;
  mutation: string;
  outcome: MutationOutcomeClass;
  fixture: string;
  fixtureClass?: EvidenceClass;
  oracle: string;
  oracleKind?: OracleKind;
  oracleEngineFamily?: string;
  subjectEngineFamily?: string;
  probes: readonly string[];
  implementation: Implementation | "";
  platform: PlatformId;
  comparisonProfile: string;
  toleranceOverridden?: boolean;
  status: "passed" | "failed" | "errored" | "missing";
  notes?: string;
}>;

/** 📈️ One dimension's measured ratio plus the exact coordinates that are missing. */
export type DimensionMeasurement = Readonly<{ dimension: CoverageDimension; covered: number; total: number; ratio: number; missing: readonly string[] }>;

function measure(dimension: CoverageDimension, covered: number, total: number, missing: readonly string[]): DimensionMeasurement {
  return { dimension, covered, total, ratio: total === 0 ? 1 : covered / total, missing: missing.slice(0, 200) };
}

/**
 * 📈️ Builds the complete v2 coverage matrix from the registry, the runtime inventories and the run's
 * results. The matrix is enumerated from the MANIFESTS, not from the results, so a mutation nobody
 * wrote a test for appears as a `missing` row instead of being absent from the denominator.
 */
export function buildCoverageMatrix(repoRoot: string, registry: OracleRegistry, results: readonly TestResult[], baselineSha: string): CoverageRow[] {
  const rows: CoverageRow[] = [];
  const platform = currentPlatform();
  const oracles = new Map(registry.oracles.map((oracle) => [oracle.id, oracle]));
  const fixturesByTarget = new Map<string, TestEvidence[]>();
  for (const contribution of registry.contributions) {
    for (const fixture of contribution.testEvidence) {
      // 🛡️A malformed fixture must not take the whole matrix down. Three fixtures registered without a
      // `target` threw here, and every caller that swallowed the throw reported coverage computed over a
      // silently smaller set — the provenance dimension read 300/300 against 303 registered fixtures,
      // because the three that FAILED it had been dropped from the denominator by the crash.
      if (fixture?.target?.artifact === undefined) continue;
      const key = `${surfaceCoordinate(fixture.target)}::${fixture.mutation ?? ""}::${fixture.outcome ?? ""}`;
      fixturesByTarget.set(key, [...(fixturesByTarget.get(key) ?? []), fixture]);
    }
  }
  const resultsByMutation = new Map<string, TestResult[]>();
  for (const result of results) {
    if (result.mutation === undefined) continue;
    const key = `${result.mutation}::${result.outcome ?? ""}`;
    resultsByMutation.set(key, [...(resultsByMutation.get(key) ?? []), result]);
  }

  for (const manifest of registry.mutationManifests) {
    for (const mutation of manifest.mutations) {
      const subset = owningSubsetOf(manifest, mutation);
      for (const outcome of mutation.outcomes) {
        const fixtureKey = `${surfaceCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset, surface: manifest.surface })}::${mutation.id}::${outcome}`;
        const fixtures = fixturesByTarget.get(fixtureKey) ?? [];
        const qualifying = registry.oracles.filter((oracle) => isQualifyingOracleKind(oracle.kind) && mutation.oracleRequirements.some((requirement) => oracle.capabilities.includes(requirement.capability)));
        const executions = resultsByMutation.get(`${mutation.id}::${outcome}`) ?? [];
        if (fixtures.length === 0 || executions.length === 0) {
          rows.push({
            baselineSha,
            artifact: manifest.artifact,
            standard: manifest.standard,
            subset,
            mutation: mutation.id,
            outcome,
            fixture: fixtures[0]?.id ?? "",
            fixtureClass: fixtures[0]?.class,
            oracle: qualifying[0]?.id ?? "",
            oracleKind: qualifying[0]?.kind,
            oracleEngineFamily: engineFamilyId(qualifying[0]?.engine),
            probes: [],
            implementation: "",
            platform,
            comparisonProfile: fixtures[0]?.comparisonProfile ?? "",
            toleranceOverridden: fixtures[0]?.toleranceOverride !== undefined,
            status: "missing",
            notes: fixtures.length === 0 ? "no fixture declares this mutation × outcome" : "no execution produced a result for this coordinate",
          });
          continue;
        }
        for (const fixture of fixtures) {
          for (const execution of executions) {
            const oracle = oracles.get(qualifying[0]?.id ?? "");
            rows.push({
              baselineSha,
              artifact: manifest.artifact,
              standard: manifest.standard,
              subset,
              mutation: mutation.id,
              outcome,
              fixture: fixture.id,
              fixtureClass: fixture.class,
              oracle: oracle?.id ?? "",
              oracleKind: oracle?.kind,
              oracleEngineFamily: engineFamilyId(oracle?.engine),
              probes: [],
              implementation: execution.implementation,
              platform: execution.platform ?? platform,
              comparisonProfile: fixture.comparisonProfile,
              toleranceOverridden: fixture.toleranceOverride !== undefined,
              status: execution.status,
            });
          }
        }
      }
    }
  }
  return rows;
}

/** 📈️ Measures all sixteen dimensions from the matrix, the registry and the run's results. */
export function measureCoverage(registry: OracleRegistry, rows: readonly CoverageRow[], results: readonly TestResult[], runtimeInventories: readonly RuntimeMutationInventory[], repoRoot: string = repoRootFromHere()): DimensionMeasurement[] {
  const manifestMutations = registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => ({ manifest, mutation })));
  const owned = new Set(manifestMutations.map(({ manifest, mutation }) => `${manifest.artifact}@${manifest.standard}::${mutation.id}`));
  const runtimeIds = runtimeInventories.flatMap((inventory) => inventory.mutations.map((mutation) => `${inventory.artifact}@${inventory.standard}::${mutation.id}`));
  // 🏭️A SUBSET WITH NO INVENTORY IS UNMEASURED, not covered. Pooling denominators across the registry
  // meant one subset that had run the bridge kept the denominator non-empty, and every subset that had
  // never run it simply contributed nothing — invisible rather than reported. Each such subset is
  // counted as one uncovered coordinate so it appears in the gate's `missing` list by name.
  const inventoried = new Set(runtimeInventories.map((inventory) => surfaceCoordinate(inventory)));
  const uninventoried = registry.mutationManifests.map((manifest) => surfaceCoordinate(manifest)).filter((coordinate) => !inventoried.has(coordinate)).map((coordinate) => `${coordinate} (no runtime inventory)`);

  const runtimeMissing = [...runtimeIds.filter((id) => !owned.has(id)), ...uninventoried];
  const wildcards = manifestMutations.filter(({ manifest, mutation }) => manifest.surface === undefined && isWildcardSubsetFor(repoRoot, manifest.artifact, manifest.standard, owningSubsetOf(manifest, mutation))).map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const withoutOracle = manifestMutations
    .filter(({ mutation }) => !mutation.oracleRequirements.every((requirement) => registry.oracles.some((oracle) => isQualifyingOracleKind(oracle.kind) && oracle.capabilities.includes(requirement.capability))))
    .map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const fixtureSubsets = new Set(registry.contributions.flatMap((contribution) => contribution.testEvidence).filter((fixture) => fixture?.target?.artifact !== undefined).map((fixture) => surfaceCoordinate(fixture.target)));
  // 🧪️EVIDENCE IS THE CONJUNCTION, not the fixture alone. Counting only "a fixture targets this subset"
  // let a mutation with NO discharged oracle — an `-uncarried` kind in a subset that happens to have
  // fixtures — count as measured, and evidence then exceeded registration: 213 against 210, which is
  // impossible by construction. A mutation is evidenced only when a qualifying oracle discharges it AND
  // something exists to run that oracle against.
  const oracled = new Set(withoutOracle);
  const withoutEvidence = manifestMutations
    .filter(({ manifest, mutation }) => oracled.has(`${manifest.artifact}::${mutation.id}`) || !fixtureSubsets.has(surfaceCoordinate(manifest)))
    .map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const requiredCapabilities = [...new Set(manifestMutations.flatMap(({ mutation }) => mutation.oracleRequirements.map((requirement) => requirement.capability)))];
  const unsupportedCapabilities = requiredCapabilities.filter((capability) => !registry.oracles.some((oracle) => isQualifyingOracleKind(oracle.kind) && oracle.capabilities.includes(capability)));

  const subjectResults = results.filter((result) => result.role === "subject");
  const replaying = subjectResults.filter((result) => result.productionDispatch?.invoked !== true).map((result) => result.testId);

  const fixtures = registry.contributions.flatMap((contribution) => contribution.testEvidence);
  const withoutProvenance = fixtures.filter((fixture) => testEvidenceProblems(fixture, repoRoot).length > 0).map((fixture) => fixture.id);
  const notReproducible = fixtures.filter((fixture) => fixture.class === "third-party-generated" && !fixture.reproducible).map((fixture) => fixture.id);
  const outcomeCoordinates = manifestMutations.flatMap(({ manifest, mutation }) => mutation.outcomes.map((outcome) => `${manifest.artifact}::${mutation.id}::${outcome}`));
  const coveredOutcomes = new Set(rows.filter((row) => row.status === "passed").map((row) => `${row.artifact}::${row.mutation}::${row.outcome}`));

  const inverseIds = new Set(results.filter((result) => result.scenario.startsWith("inverse-")).map((result) => result.scenario.slice("inverse-".length)));
  const metamorphicIds = new Set(results.filter((result) => result.scenario.startsWith("metamorphic-")).map((result) => result.scenario.slice("metamorphic-".length)));
  const allIds = manifestMutations.map(({ mutation }) => mutation.id);

  const determinismChecked = new Set(results.filter((result) => result.scenario.startsWith("determinism-")).map((result) => result.scenario.slice("determinism-".length)));
  const implementations = new Set(results.map((result) => result.implementation));
  const platforms = new Set(rows.map((row) => row.platform));
  const metricRows = rows.filter((row) => row.status !== "missing");
  const withMetric = metricRows.filter((row) => row.comparisonProfile.length > 0);
  const leakyOracles = registry.oracles.filter((oracle) => oracle.productionReachable === true && oracle.productionDebt === undefined).map((oracle) => oracle.id);
  const leakyProbes = registry.probes.filter((probe) => probe.productionReachable === true).map((probe) => probe.id);

  return [
    measure("runtimeMutationCoverage", runtimeIds.length - runtimeIds.filter((id) => !owned.has(id)).length, runtimeIds.length + uninventoried.length, runtimeMissing),
    measure("subsetOwnershipCoverage", manifestMutations.length - wildcards.length, manifestMutations.length, wildcards),
    measure("externalOracleCoverage", manifestMutations.length - withoutOracle.length, manifestMutations.length, withoutOracle),
    // 🧪️AN ORACLE THAT HAS NEVER BEEN RUN AGAINST ANYTHING PROVES NOTHING. `externalOracleCoverage`
    // asks only whether a qualifying oracle is REGISTERED for a mutation; it says nothing about whether
    // any artifact exists to run it on. 271 of 369 manifested mutations had zero fixtures targeting their
    // subset while counting as covered — the empty-denominator failure this protocol already forbids at
    // the dimension level, reappearing one level down at the mutation level.
    measure("oracleEvidenceCoverage", manifestMutations.length - withoutEvidence.length, manifestMutations.length, withoutEvidence),
    measure("oracleCapabilityCoverage", requiredCapabilities.length - unsupportedCapabilities.length, requiredCapabilities.length, unsupportedCapabilities),
    measure("productionBridgeCoverage", subjectResults.length - replaying.length, subjectResults.length, replaying),
    measure("fixtureClassCoverage", EVIDENCE_CLASSES.filter((klass) => fixtures.some((fixture) => fixture.class === klass)).length, EVIDENCE_CLASSES.length, EVIDENCE_CLASSES.filter((klass) => !fixtures.some((fixture) => fixture.class === klass))),
    measure("fixtureProvenanceCoverage", fixtures.length - withoutProvenance.length, fixtures.length, withoutProvenance),
    measure("expectedOutcomeCoverage", outcomeCoordinates.filter((coordinate) => coveredOutcomes.has(coordinate)).length, outcomeCoordinates.length, outcomeCoordinates.filter((coordinate) => !coveredOutcomes.has(coordinate))),
    measure("inverseCoverage", allIds.filter((id) => inverseIds.has(id)).length, allIds.length, allIds.filter((id) => !inverseIds.has(id))),
    measure("metamorphicCoverage", allIds.filter((id) => metamorphicIds.has(id)).length, allIds.length, allIds.filter((id) => !metamorphicIds.has(id))),
    measure("comparisonMetricCoverage", withMetric.length, metricRows.length, metricRows.filter((row) => row.comparisonProfile.length === 0).map((row) => `${row.artifact}::${row.mutation}`)),
    measure("determinismCoverage", allIds.filter((id) => determinismChecked.has(id)).length, allIds.length, allIds.filter((id) => !determinismChecked.has(id))),
    measure("implementationCoverage", implementations.size, Math.max(implementations.size, 1), []),
    measure("platformCoverage", platforms.size, Math.max(platforms.size, 1), []),
    measure("fixtureReproducibilityCoverage", fixtures.length - notReproducible.length, fixtures.length, notReproducible),
    measure("dependencyIsolationCoverage", registry.oracles.length + registry.probes.length - leakyOracles.length - leakyProbes.length, registry.oracles.length + registry.probes.length, [...leakyOracles, ...leakyProbes]),
  ];
}

/** 🚦️ The release gates. Every one is exact: `= 100%`, `= 0`, never "at least". */
export function enforceReleaseGates(measurements: readonly DimensionMeasurement[], counts: { deferredMutations: number; skipped: number; wildcardOwners: number; unregisteredRuntimeMutations: number }): string[] {
  const failures: string[] = [];
  const byDimension = new Map(measurements.map((measurement) => [measurement.dimension, measurement]));
  for (const dimension of RELEASE_GATED_DIMENSIONS) {
    const measurement = byDimension.get(dimension);
    if (measurement === undefined) {
      failures.push(`${dimension} was never measured — a gate that reads nothing cannot be satisfied`);
      continue;
    }
    // 🚦️An EMPTY denominator is not 100%. `measure` reports ratio 1 for an empty set so the display
    // reads "n/a" rather than a false 0%, but a release gate that accepted it would be satisfied by a
    // run in which nothing was measured at all — which is precisely the failure the gate exists for.
    if (measurement.total === 0) {
      failures.push(`${dimension} has an EMPTY denominator — nothing was measured, and an unmeasured dimension cannot be 100%`);
      continue;
    }
    if (measurement.ratio < 1) failures.push(`${dimension} is ${(measurement.ratio * 100).toFixed(2)}% (${measurement.covered}/${measurement.total}); release requires 100% — missing: ${measurement.missing.slice(0, 10).join(", ")}${measurement.missing.length > 10 ? ` (+${measurement.missing.length - 10} more)` : ""}`);
  }
  if (counts.deferredMutations !== 0) failures.push(`${counts.deferredMutations} deferred mutation(s); release requires 0`);
  if (counts.skipped !== 0) failures.push(`${counts.skipped} skip/quarantine(s); release requires 0`);
  if (counts.wildcardOwners !== 0) failures.push(`${counts.wildcardOwners} wildcard subset owner(s); release requires 0`);
  if (counts.unregisteredRuntimeMutations !== 0) failures.push(`${counts.unregisteredRuntimeMutations} unregistered runtime mutation(s); release requires 0`);
  return failures;
}

/** 📊️ Renders the matrix as the report's own questions, each answered by an explicit list. */
export function formatCoverageQuestions(registry: OracleRegistry, rows: readonly CoverageRow[], measurements: readonly DimensionMeasurement[]): string {
  const fixtures = registry.contributions.flatMap((contribution) => contribution.testEvidence);
  const byDimension = new Map(measurements.map((measurement) => [measurement.dimension, measurement]));
  const untested = (byDimension.get("runtimeMutationCoverage")?.missing ?? []).join(", ") || "none";
  const noOracle = (byDimension.get("externalOracleCoverage")?.missing ?? []).join(", ") || "none";
  const semioDerived = registry.oracles.filter((oracle) => oracle.kind === "cross-semio-implementation").map((oracle) => oracle.id).join(", ") || "none";
  const wildcard = (byDimension.get("subsetOwnershipCoverage")?.missing ?? []).join(", ") || "none";
  const provenance = (byDimension.get("fixtureProvenanceCoverage")?.missing ?? []).join(", ") || "none";
  const reproducibility = (byDimension.get("fixtureReproducibilityCoverage")?.missing ?? []).join(", ") || "none";
  const subsets = [...new Set(fixtures.map((fixture) => subsetCoordinate(fixture.target)))];
  const withoutRealWorld = subsets.filter((coordinate) => !fixtures.some((fixture) => subsetCoordinate(fixture.target) === coordinate && fixture.class === "real-world")).join(", ") || "none";
  const overridden = rows.filter((row) => row.toleranceOverridden === true).map((row) => `${row.mutation}/${row.fixture}`).join(", ") || "none";
  const sharedEngine = rows.filter((row) => row.oracleEngineFamily !== undefined && row.subjectEngineFamily !== undefined && row.oracleEngineFamily === row.subjectEngineFamily).map((row) => `${row.mutation} (${row.oracleEngineFamily})`).join(", ") || "none";
  const platformSpecific = (() => {
    const byCoordinate = new Map<string, Set<string>>();
    for (const row of rows) {
      const key = `${row.mutation}::${row.fixture}`;
      if (row.status === "failed") byCoordinate.set(key, (byCoordinate.get(key) ?? new Set()).add(row.platform));
    }
    const all = new Set(rows.map((row) => row.platform));
    return [...byCoordinate].filter(([, failing]) => failing.size < all.size).map(([key]) => key).join(", ") || "none";
  })();
  return [
    "[report] Which runtime mutations are untested?                       " + untested,
    "[report] Which mutations have no external oracle?                    " + noOracle,
    "[report] Which tests still use a Semio-derived oracle?               " + semioDerived,
    "[report] Which mutations remain under wildcard ownership?            " + wildcard,
    "[report] Which fixtures lack provenance?                             " + provenance,
    "[report] Which fixtures are not reproducible?                        " + reproducibility,
    "[report] Which subsets have no real-world fixture?                   " + withoutRealWorld,
    "[report] Which results are tolerance-sensitive (overridden)?         " + overridden,
    "[report] Which failures occur on only one platform?                  " + platformSpecific,
    "[report] Which oracle and subject share an underlying engine family? " + sharedEngine,
  ].join("\n");
}
//#endregion 📈️CoverageV2
