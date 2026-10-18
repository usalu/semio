import { join } from "node:path";
import { mkdirSync } from "node:fs";
import { coverageDir, coverageSlug } from "../../🏃️process/📋️context/🔣️policy/🟦️.ts";

/** 📊️Whether instrumented coverage collection is on — auto-set by [[resolveTestLevel]] at the `exhaustive` level; `SEMIO_COVERAGE=0` opts out. */
export function coverageEnabled(): boolean {
  return process.env.SEMIO_COVERAGE === "1";
}

/**
 * 📊️Central, authoritative exclusion list applied at repo-wide aggregation — generated code, vendored/emitted
 * assets, and paths that are GPU-only or Electron-shell and thus unmeasurable by a headless line-coverage
 * runner. Per-tool excludes (vitest `coverage.exclude`, coverage.py `omit`, …) may mirror this for speed, but
 * this list is what decides what counts toward the repo-wide percentage. Every entry has a reason in
 * [[COVERAGE_EXCLUDE_REASONS]] so exclusions stay auditable rather than a silent denominator shrink.
 */
export const COVERAGE_EXCLUDE_GLOBS: readonly string[] = [
  "**/generated/**",
  "framework/module/asset/metabolism/icon/generated/**",
  "**/pkg/**",
  ".storybook/**",
  "**/*.stories.*",
  "**/*.spec.ts",
  "**/*.test.ts",
  "**/*.tex",
  "**/*.wgsl",
  "**/*.svg",
  "elements/client/lib/geometry/topologic/**",
  "framework/product/os/module/renderer/wgpu/**",
  "framework/module/ui/wgpu/**",
  "**/dist/**",
  "**/node_modules/**",
  "**/target/**",
];

/** 📊️One-line rationale per [[COVERAGE_EXCLUDE_GLOBS]] entry — printed alongside the coverage summary. */
export const COVERAGE_EXCLUDE_REASONS: Readonly<Record<string, string>> = {
  "**/generated/**": "Emitted lookup tables/codegen output, not hand-authored logic.",
  "framework/module/asset/metabolism/icon/generated/**": "~22k LOC generated icon table.",
  "**/pkg/**": "wasm-bindgen build output.",
  ".storybook/**": "Covered by Playwright specs, not unit line coverage.",
  "**/*.stories.*": "Storybook fixtures, exercised visually not via line coverage.",
  "**/*.spec.ts": "Playwright specs — browser-process coverage is a separate mechanism.",
  "**/*.test.ts": "Standalone test files measure themselves, not the source under test.",
  "**/*.tex": "LaTeX templates — not executable.",
  "**/*.wgsl": "GPU shader source — not measurable by CPU line coverage.",
  "**/*.svg": "Vector assets — not executable.",
  "elements/client/lib/geometry/topologic/**": "Vendored third-party C++.",
  "framework/product/os/module/renderer/wgpu/**": "GPU rendering internals — smoke-testable only; a headless-GPU harness is out of scope here.",
  "framework/module/ui/wgpu/**": "GPU rendering internals — smoke-testable only; a headless-GPU harness is out of scope here.",
  "**/dist/**": "Build output.",
  "**/node_modules/**": "Third-party dependency code.",
  "**/target/**": "Cargo build output.",
};

function coverageGlobToRegExp(glob: string): RegExp {
  const escaped = glob
    .replace(/[.+^${}()|[\]\\]/g, "\\$&")
    .replace(/\*\*/g, "\x00")
    .replace(/\*/g, "[^/]*")
    .replace(/\x00/g, ".*");
  return new RegExp(`^${escaped}$`);
}

/** 📊️Whether a repo-relative path (forward-slash, no leading `./`) matches any [[COVERAGE_EXCLUDE_GLOBS]] entry. */
export function isCoverageExcluded(relPath: string): boolean {
  const normalized = relPath.replace(/\\/g, "/").replace(/^\.\//, "");
  return COVERAGE_EXCLUDE_GLOBS.some((glob) => coverageGlobToRegExp(glob).test(normalized));
}

/** 📊️`go test` coverage args, appended alongside `goLevelTestArgs` when [[coverageEnabled]]; the text profile is converted to LCOV via [[goProfileToLcov]] at aggregation. */
export function goCoverageArgs(repoRoot: string, moduleLabel: string): string[] {
  if (!coverageEnabled()) return [];
  const file = join(coverageDir(repoRoot, "go"), `${coverageSlug(moduleLabel)}.cover`);
  return ["-covermode=atomic", "-coverpkg=./...", `-coverprofile=${file}`];
}

/** 📊️pytest coverage args, appended alongside `pytestLevelArgs` when [[coverageEnabled]]; `pytest-cov`'s `lcov` reporter writes LCOV directly, no conversion needed. */
export function pytestCoverageArgs(repoRoot: string, moduleLabel: string): string[] {
  if (!coverageEnabled()) return [];
  const file = join(coverageDir(repoRoot, "py"), `${coverageSlug(moduleLabel)}.lcov`);
  return ["--cov", `--cov-report=lcov:${file}`];
}

/** 📊️dotnet test coverage args (coverlet via `--collect`), appended when [[coverageEnabled]]; each project gets its own results-directory subfolder so concurrent projects don't clobber one another's report file. */
export function dotnetCoverageArgs(repoRoot: string, moduleLabel: string): string[] {
  if (!coverageEnabled()) return [];
  const dir = join(coverageDir(repoRoot, "dotnet"), coverageSlug(moduleLabel));
  mkdirSync(dir, { recursive: true });
  return ["--collect", "XPlat Code Coverage;Format=lcov", "--results-directory", dir];
}

export type CoverageSummary = {
  linesFound: number;
  linesHit: number;
  pct: number;
  perFile: { path: string; linesFound: number; linesHit: number; pct: number }[];
};

/** 📊️Reduces a merged coverage map to a repo-wide percentage plus a worst-offenders table, after dropping [[isCoverageExcluded]] paths. */
export function summarizeCoverage(merged: Map<string, Map<number, number>>): CoverageSummary {
  let linesFound = 0;
  let linesHit = 0;
  const perFile: CoverageSummary["perFile"] = [];
  for (const [path, lines] of merged) {
    if (isCoverageExcluded(path)) continue;
    const found = lines.size;
    const hit = [...lines.values()].filter((count) => count > 0).length;
    linesFound += found;
    linesHit += hit;
    perFile.push({ path, linesFound: found, linesHit: hit, pct: found === 0 ? 100 : (hit / found) * 100 });
  }
  perFile.sort((a, b) => a.pct - b.pct || b.linesFound - a.linesFound);
  return { linesFound, linesHit, pct: linesFound === 0 ? 0 : (linesHit / linesFound) * 100, perFile };
}

/** 📊️Prints the worst-covered files and hard-fails (`process.exit(1)`) below `thresholdPct`. */
export function enforceCoverageThreshold(summary: CoverageSummary, thresholdPct: number, worstCount = 25): void {
  console.log(`[coverage] ${summary.linesHit}/${summary.linesFound} lines (${summary.pct.toFixed(2)}%), threshold ${thresholdPct}%.`);
  if (summary.pct < thresholdPct) {
    console.log(`[coverage] worst ${Math.min(worstCount, summary.perFile.length)} files:`);
    for (const file of summary.perFile.slice(0, worstCount)) {
      console.log(`  ${file.pct.toFixed(1)}% (${file.linesHit}/${file.linesFound})  ${file.path}`);
    }
    console.error(`[coverage] ${summary.pct.toFixed(2)}% < ${thresholdPct}% — exhaustive gate failed.`);
    process.exit(1);
  }
}
