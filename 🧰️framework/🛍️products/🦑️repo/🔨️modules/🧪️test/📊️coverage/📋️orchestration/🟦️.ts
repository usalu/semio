import { matchesRow, readSelectors } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import { reportsDir } from "../../📊️reporting/📋️orchestration/🟦️.ts";
import {
  type RuntimeMutationInventory,
  buildCoverageMatrix,
  enforceReleaseGates,
  engineFamilyId,
  formatCoverageQuestions,
  isQualifiedProbe,
  isWildcardSubset,
  loadOracleRegistry,
  measureCoverage,
  owningSubsetOf,
  readResults,
  readRuntimeInventory,
} from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { join } from "node:path";

/** 🔬️ Lists and qualifies the external measurement probes the comparison pipeline invokes. */
export class ProbeScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const probes = registry.probes.filter((probe) => selectors.probe === null || probe.id === selectors.probe);
    if (segments.includes("--json")) {
      console.log(JSON.stringify(probes, null, 2));
      return;
    }
    for (const probe of probes) {
      const status = probe.qualification?.status ?? "unqualified";
      console.log(`[probe] ${probe.id.padEnd(28)} ${probe.kind.padEnd(17)} engine=${engineFamilyId(probe.engine)}@${probe.engine?.version ?? "*"} deterministic=${probe.deterministic} qualification=${status}`);
      for (const criterion of probe.qualification?.criteria ?? []) console.log(`[probe]   ${criterion.met ? "✔" : "✘"} ${criterion.id}${criterion.detail === undefined ? "" : ` — ${criterion.detail}`}`);
    }
    const unqualified = probes.filter((probe) => !isQualifiedProbe(probe));
    console.log(`[probe] ${probes.length} probe(s), ${unqualified.length} not yet qualified`);
    for (const probe of unqualified) console.log(`[probe] ${probe.id}: RUNS and REPORTS; no release gate may claim its strongest guarantee until the qualification spike passes`);
  }
}

/** 📊️ The full subset-scoped coverage matrix and its release gates. Never an artifact-level aggregate. */
export class MatrixScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const { results } = readResults(join(reportsDir(this.repoRoot), "📤️results.jsonl"));
    const baselineSha = process.env.SEMIO_BASELINE_SHA ?? "";
    const rows = buildCoverageMatrix(this.repoRoot, registry, results, baselineSha).filter((row) => matchesRow(row, selectors));
    const inventories = registry.mutationManifests.map((manifest) => readRuntimeInventory(this.repoRoot, manifest)).filter((inventory): inventory is RuntimeMutationInventory => inventory !== null);
    const measurements = measureCoverage(registry, rows, results, inventories);
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ baselineSha, rows, measurements }, null, 2));
      return;
    }
    for (const measurement of measurements) console.log(`[matrix] ${measurement.dimension.padEnd(32)} ${(measurement.ratio * 100).toFixed(2).padStart(6)}%  ${measurement.covered}/${measurement.total}`);
    console.log(formatCoverageQuestions(registry, rows, measurements));
    if (!segments.includes("--enforce")) return;
    const wildcardOwners = registry.mutationManifests.flatMap((manifest) => manifest.mutations.filter((mutation) => isWildcardSubset(owningSubsetOf(manifest, mutation)))).length;
    const deferred = registry.mutationCatalogs.reduce((total, catalog) => total + (catalog.deferredKinds ?? []).length, 0);
    const unregistered = measurements.find((measurement) => measurement.dimension === "runtimeMutationCoverage")?.missing.length ?? 0;
    const failures = enforceReleaseGates(measurements, { deferredMutations: deferred, skipped: 0, wildcardOwners, unregisteredRuntimeMutations: unregistered });
    for (const failure of failures) console.error(`[matrix] ${failure}`);
    if (failures.length > 0) process.exit(1);
  }
}
