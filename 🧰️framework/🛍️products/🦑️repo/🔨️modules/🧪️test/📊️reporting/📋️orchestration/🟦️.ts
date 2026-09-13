import { type CoverageMetrics, type ImplementationCoverage, enforceMetricGates, formatMetrics, markOutputDir, readResults, renderJUnit, testCacheDir } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script, getRepoMetaDir } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, relative, sep } from "node:path";

/** 📊️ Where the latest run's reports land — a marked directory, so `clean test` can remove them. */
export function reportsDir(repoRoot: string): string {
  const dir = join(testCacheDir(repoRoot, "reports"), "latest");
  markOutputDir(repoRoot, dir, { testId: "run::latest", cacheKey: "reports" });
  return dir;
}

/** 📈️ Per-implementation source coverage, read from each language's own report directory. An
 * implementation that reported nothing is ABSENT rather than 100% — a blended repository percentage
 * must never be able to stand in for a language that produced no coverage at all. */
export function readImplementationCoverage(repoRoot: string): ImplementationCoverage[] {
  const roots: [string, string][] = [
    ["rust", "rust"],
    ["typescript", "js"],
    ["go", "go"],
    ["python", "py"],
    ["dotnet", "dotnet"],
  ];
  const out: ImplementationCoverage[] = [];
  for (const [implementation, dirName] of roots) {
    const dir = join(getRepoMetaDir(repoRoot), "📊️metrics", "coverage", dirName);
    if (!existsSync(dir)) continue;
    let linesFound = 0;
    let linesHit = 0;
    let branchesFound = 0;
    let branchesHit = 0;
    let sawBranches = false;
    const walk = (current: string): void => {
      for (const entry of readdirSync(current, { withFileTypes: true })) {
        const full = join(current, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (!/\.(lcov|info|cover)$/.test(entry.name)) continue;
        for (const line of readFileSync(full, "utf8").split(/\r?\n/)) {
          if (line.startsWith("LF:")) linesFound += Number(line.slice(3)) || 0;
          else if (line.startsWith("LH:")) linesHit += Number(line.slice(3)) || 0;
          else if (line.startsWith("BRF:")) {
            branchesFound += Number(line.slice(4)) || 0;
            sawBranches = true;
          } else if (line.startsWith("BRH:")) branchesHit += Number(line.slice(4)) || 0;
        }
      }
    };
    walk(dir);
    if (linesFound === 0) continue;
    out.push({
      implementation,
      lines: { covered: linesHit, total: linesFound, ratio: linesHit / linesFound },
      branches: sawBranches && branchesFound > 0 ? { covered: branchesHit, total: branchesFound, ratio: branchesHit / branchesFound } : null,
    });
  }
  return out;
}

/** 📊️ Re-renders the report artifacts from the last run's result stream. */
export class ReportScript extends Script {
  run(): void {
    const dir = reportsDir(this.repoRoot);
    const stream = join(dir, "📤️results.jsonl");
    const { results, problems } = readResults(stream);
    writeFileSync(join(dir, "📋️junit.xml"), renderJUnit(results));
    console.log(`[report] ${results.length} result(s) from ${relative(this.repoRoot, stream).split(sep).join("/")}`);
    for (const problem of problems) console.error(`[report] ${problem}`);
  }
}

export class MetricsScript extends Script {
  run(segments: string[]): void {
    const metricsPath = join(reportsDir(this.repoRoot), "📈️metrics.json");
    if (!existsSync(metricsPath)) {
      console.error(`[metrics] no run to report on — run \`bun ./📜️script.ts parity <level>\` first`);
      process.exit(1);
    }
    const metrics = JSON.parse(readFileSync(metricsPath, "utf8")) as CoverageMetrics;
    const perImplementation = readImplementationCoverage(this.repoRoot);
    console.log(formatMetrics(metrics, perImplementation));
    if (!segments.includes("--enforce")) return;
    const threshold = Number(segments[segments.indexOf("--threshold") + 1] ?? 95);
    const failures = enforceMetricGates(metrics, perImplementation, Number.isFinite(threshold) ? threshold : 95);
    for (const failure of failures) console.error(`[metrics] ${failure}`);
    if (failures.length > 0) process.exit(1);
  }
}
