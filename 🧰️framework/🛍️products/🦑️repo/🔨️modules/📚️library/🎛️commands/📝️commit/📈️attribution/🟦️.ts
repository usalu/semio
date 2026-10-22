import type { CommitBundleSection, GitDeltaSum } from "../🧾️bundle-model/🟦️.ts";
import { git, gitRepoRoot } from "../🌳️git/🟦️.ts";
import { accumulateRangeUlocDeltas, accumulateSizeDeltasFromPaths, accumulateUlocDeltasByRow, gitDeltaLineTotal, gitRangeNumstat, pathsFromNumstatRow, pathUnderPrefixes, sumGitLangDeltas } from "../📊️metrics/🔀️deltas/🟦️.ts";
import { buildBundlePathPrefixSets, resolveBundleIndicesForNumstatRow } from "../🧭️path-prefixes/🟦️.ts";
import { extractBundleDateLineFromCommit } from "../📅️dates/🟦️.ts";
import { shouldSkipPathForUloc } from "../📊️metrics/🛤️path-rules/🟦️.ts";
import { normalizeBundleScopeLabel } from "../🏷️labels/🟦️.ts";
import { formatBundleMetricSuffixes, METRIC_KIND_SIZE, METRIC_KIND_ULOC } from "../📊️metrics/🎨️format/🟦️.ts";
import { buildCommitMetricsForRange, gitDeltaSumsEqual, sumMicroCommitLangMetrics, validateCommitMetricsDeltaSum } from "../📊️metrics/🧮️summary/🟦️.ts";
import type { MetricsRunner } from "../📊️metrics/🏷️types/🟦️.ts";
import { createDefaultMetricsRunner } from "../📊️metrics/💾️scan/🟦️.ts";
import {refuseCommitV1} from "../📖️texts/🟦️.ts";

function addGitDeltaSums(a: GitDeltaSum, b: GitDeltaSum): GitDeltaSum {
  return { added: a.added + b.added, removed: a.removed + b.removed, edited: a.edited + b.edited };
}

export type BundleDateDeltasMap = Map<number, Map<string, GitDeltaSum>>;

function gitCommitShasInRange(root: string, base: string, head: string): string[] {
  const r = git(root, ["rev-list", "--reverse", `${base}..${head}`]);
  if (!r.ok || !r.out) return [];
  return r.out.split("\n").filter(Boolean);
}

function addUlocDeltaToBundleDateMap(map: BundleDateDeltasMap, chunk: GitDeltaSum, dateLine: string, bi: number): void {
  if (gitDeltaLineTotal(chunk) === 0) return;
  const bundleMap = map.get(bi)!;
  const prev = bundleMap.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
  bundleMap.set(dateLine, addGitDeltaSums(prev, chunk));
}

/** 📊️Per-bundle per-day git deltas: sum each micro-commit parent..sha row on its body 🎆️ day (sums to range partition). */
export function buildBundleDateDeltasMap(root: string, base: string, head: string, bundles: CommitBundleSection[]): BundleDateDeltasMap {
  root = gitRepoRoot(root);
  const map: BundleDateDeltasMap = new Map();
  for (let i = 0; i < bundles.length; i++) map.set(i, new Map());
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  for (const sha of gitCommitShasInRange(root, base, head)) {
    const parent = `${sha}^`;
    const subject = git(root, ["log", "-1", "--format=%s", sha]).out;
    const body = git(root, ["log", "-1", "--format=%B", sha]).out;
    const dateLine = extractBundleDateLineFromCommit(subject, body);
    if (!dateLine) continue;
    const rows = gitRangeNumstat(root, parent, sha);
    const rowDeltas = accumulateUlocDeltasByRow(root, rows, parent, sha);
    for (const [index, row] of rows.entries()) {
      const rowPaths = pathsFromNumstatRow(row.path);
      if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
      const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
      if (owners.length === 0) {
        throw refuseCommitV1("failed","attribution.changed-path-is-not-attributed",{path:String(row.path)});
      }
      if (owners.length > 1) {
        const names = owners.map((i) => bundles[i]!.label).join(", ");
        throw refuseCommitV1("failed","attribution.changed-path-matches-multiple-bundles",{names:String(names),path:String(row.path)});
      }
      addUlocDeltaToBundleDateMap(map, sumGitLangDeltas(rowDeltas[index]!), dateLine, owners[0]!);
    }
  }
  return map;
}

function addSizeRowToBundleDateMap(map: BundleDateDeltasMap, row: { path: string }, dateLine: string, bi: number, root: string, oldRev: string, newRev: string): void {
  const chunk = sumGitLangDeltas(accumulateSizeDeltasFromPaths(root, [{ path: row.path }], oldRev, newRev));
  if (gitDeltaLineTotal(chunk) === 0) return;
  const bundleMap = map.get(bi)!;
  const prev = bundleMap.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
  bundleMap.set(dateLine, addGitDeltaSums(prev, chunk));
}

/** 📊️Per-bundle per-day byte deltas from micro-commit parent..sha rows. */
export function buildBundleDateSizeDeltasMap(root: string, base: string, head: string, bundles: CommitBundleSection[]): BundleDateDeltasMap {
  root = gitRepoRoot(root);
  const map: BundleDateDeltasMap = new Map();
  for (let i = 0; i < bundles.length; i++) map.set(i, new Map());
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  for (const sha of gitCommitShasInRange(root, base, head)) {
    const parent = `${sha}^`;
    const subject = git(root, ["log", "-1", "--format=%s", sha]).out;
    const body = git(root, ["log", "-1", "--format=%B", sha]).out;
    const dateLine = extractBundleDateLineFromCommit(subject, body);
    if (!dateLine) continue;
    for (const row of gitRangeNumstat(root, parent, sha)) {
      const rowPaths = pathsFromNumstatRow(row.path);
      if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
      const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
      if (owners.length === 0) {
        throw refuseCommitV1("failed","attribution.changed-path-is-not-attributed",{path:String(row.path)});
      }
      if (owners.length > 1) {
        const names = owners.map((i) => bundles[i]!.label).join(", ");
        throw refuseCommitV1("failed","attribution.changed-path-matches-multiple-bundles",{names:String(names),path:String(row.path)});
      }
      addSizeRowToBundleDateMap(map, row, dateLine, owners[0]!, root, parent, sha);
    }
  }
  return map;
}

function bundleGitDeltasForPaths(root: string, base: string, head: string, pathPrefixes: string[]): { added: number; removed: number; edited: number } {
  return sumGitLangDeltas(accumulateRangeUlocDeltas(root, base, head, pathPrefixes));
}

function bundleSizeDeltasForPaths(root: string, base: string, head: string, pathPrefixes: string[]): GitDeltaSum {
  const rows = gitRangeNumstat(root, base, head);
  const assigned = pathPrefixes.length > 0 ? rows.filter((r) => pathsFromNumstatRow(r.path).some((p) => pathUnderPrefixes(p, pathPrefixes))) : [];
  return sumGitLangDeltas(accumulateSizeDeltasFromPaths(root, assigned, base, head));
}

export function formatBundleHeaderLine(label: string, ulocTotal: GitDeltaSum, sizeTotal: GitDeltaSum, ulocBloc: number, sizeBloc: number): string {
  return `${normalizeBundleScopeLabel(label)}${formatBundleMetricSuffixes(ulocTotal, sizeTotal, ulocBloc, sizeBloc)}`;
}

/** 📊️Orders bundles by descending 🟰️ (➕️+✏️+➖️) from assigned path diffs. */
export function sortCommitBundlesByEditTotal(root: string, base: string, head: string, bundles: CommitBundleSection[], pathAssignments: string[][]): { bundles: CommitBundleSection[]; pathAssignments: string[][]; pathPrefixSets: string[][] } {
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  const ranked = bundles.map((bundle, i) => ({
    bundle,
    paths: pathAssignments[i] ?? [],
    prefixes: prefixSets[i] ?? [],
    total: gitDeltaLineTotal(bundleGitDeltasForPaths(root, base, head, prefixSets[i] ?? [])),
  }));
  ranked.sort((a, b) => b.total - a.total);
  return {
    bundles: ranked.map((r) => r.bundle),
    pathAssignments: ranked.map((r) => r.paths),
    pathPrefixSets: ranked.map((r) => r.prefixes),
  };
}

export function formatGitDeltaSumBrief(d: GitDeltaSum): string {
  return `➕️${d.added}✏️${d.edited}➖️${d.removed}🟰️${gitDeltaLineTotal(d)}`;
}

function assertGitDeltaSumsEqual(a: GitDeltaSum, b: GitDeltaSum, message: string): void {
  if (gitDeltaSumsEqual(a, b)) return;
  throw new Error(`${message} — ${formatGitDeltaSumBrief(a)} vs ${formatGitDeltaSumBrief(b)}`);
}

export function rangeGitDeltaTotal(root: string, base: string, head: string): GitDeltaSum {
  return sumGitLangDeltas(accumulateRangeUlocDeltas(root, base, head));
}

/** 📊️Partition WIP-range numstat across bundles (one bundle per row). */
export function partitionRangeDeltasByBundle(root: string, base: string, head: string, bundles: CommitBundleSection[], prefixSets: string[][]): { bundleTotals: GitDeltaSum[]; rangeTotal: GitDeltaSum } {
  const bundleTotals = bundles.map(() => ({ added: 0, removed: 0, edited: 0 }));
  let rangeTotal: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  const rows = gitRangeNumstat(root, base, head);
  const rowDeltas = accumulateUlocDeltasByRow(root, rows, base, head);
  for (const [index, row] of rows.entries()) {
    const rowPaths = pathsFromNumstatRow(row.path);
    if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
    const chunk = sumGitLangDeltas(rowDeltas[index]!);
    if (gitDeltaLineTotal(chunk) === 0) continue;
    rangeTotal = addGitDeltaSums(rangeTotal, chunk);
    const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
    if (owners.length === 0) {
      throw refuseCommitV1("failed","attribution.changed-path-is-not-attributed",{path:String(row.path)});
    }
    if (owners.length > 1) {
      const names = owners.map((i) => bundles[i]!.label).join(", ");
      throw refuseCommitV1("failed","attribution.changed-path-matches-multiple-bundles",{names:String(names),path:String(row.path)});
    }
    const bi = owners[0]!;
    bundleTotals[bi] = addGitDeltaSums(bundleTotals[bi]!, chunk);
  }
  return { bundleTotals, rangeTotal };
}

/** 📊️Partition WIP-range byte deltas across bundles (one bundle per row). */
export function partitionRangeSizeDeltasByBundle(root: string, base: string, head: string, bundles: CommitBundleSection[], prefixSets: string[][]): { bundleTotals: GitDeltaSum[]; rangeTotal: GitDeltaSum } {
  const bundleTotals = bundles.map(() => ({ added: 0, removed: 0, edited: 0 }));
  let rangeTotal: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const row of gitRangeNumstat(root, base, head)) {
    const rowPaths = pathsFromNumstatRow(row.path);
    if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
    const chunk = sumGitLangDeltas(accumulateSizeDeltasFromPaths(root, [{ path: row.path }], base, head));
    if (gitDeltaLineTotal(chunk) === 0) continue;
    rangeTotal = addGitDeltaSums(rangeTotal, chunk);
    const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
    if (owners.length === 0) {
      throw refuseCommitV1("failed","attribution.changed-path-is-not-attributed",{path:String(row.path)});
    }
    if (owners.length > 1) {
      const names = owners.map((i) => bundles[i]!.label).join(", ");
      throw refuseCommitV1("failed","attribution.changed-path-matches-multiple-bundles",{names:String(names),path:String(row.path)});
    }
    const bi = owners[0]!;
    bundleTotals[bi] = addGitDeltaSums(bundleTotals[bi]!, chunk);
  }
  return { bundleTotals, rangeTotal };
}

export function validateBundleDayDeltasAttribution(bundles: CommitBundleSection[], prefixSets: string[][], dateDeltas: BundleDateDeltasMap, bundleTotals: GitDeltaSum[], kindToken = "📃uloc", additive = true): void {
  for (let bi = 0; bi < bundles.length; bi++) {
    const bundle = bundles[bi]!;
    const total = bundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 };
    const listedDates = new Set(bundle.dates.map((s) => s.dateLine));
    let daySum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
    for (const dateLine of listedDates) {
      const d = dateDeltas.get(bi)?.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
      daySum = addGitDeltaSums(daySum, d);
    }
    const perDay = dateDeltas.get(bi);
    if (perDay) {
      for (const [dateLine, d] of perDay) {
        if (listedDates.has(dateLine)) continue;
        if (gitDeltaLineTotal(d) === 0) continue;
        throw refuseCommitV1("missing","attribution.has-changes-on-but-that",{label:String(bundle.label),dateLine:String(dateLine),delta:String(formatGitDeltaSumBrief(d))});
      }
    }
    if (additive && (daySum.added !== total.added || daySum.edited !== total.edited || daySum.removed !== total.removed)) {
      throw refuseCommitV1("failed","attribution.per-day-metric-for-does",{kindToken:String(kindToken),label:String(bundle.label),daySum:String(formatGitDeltaSumBrief(daySum)),total:String(formatGitDeltaSumBrief(total))},);
    }
  }
}

/** 🚫️All bundle-commit metrics constraints (days→bundle, bundles→range, languages→range). */
export function validateBundleCommitAttribution(root: string, base: string, head: string, bundles: CommitBundleSection[], metricsRunner?: MetricsRunner): void {
  root = gitRepoRoot(root);
  const runner = metricsRunner ?? createDefaultMetricsRunner();
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  const { bundleTotals: ulocPartitioned, rangeTotal: ulocRangeTotal } = partitionRangeDeltasByBundle(root, base, head, bundles, prefixSets);
  const { bundleTotals: sizePartitioned, rangeTotal: sizeRangeTotal } = partitionRangeSizeDeltasByBundle(root, base, head, bundles, prefixSets);
  const dateDeltas = buildBundleDateDeltasMap(root, base, head, bundles);
  const dateSizeDeltas = buildBundleDateSizeDeltasMap(root, base, head, bundles);
  validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, ulocPartitioned, METRIC_KIND_ULOC.token);
  validateBundleDayDeltasAttribution(bundles, prefixSets, dateSizeDeltas, sizePartitioned, METRIC_KIND_SIZE.token, false);
  for (let bi = 0; bi < bundles.length; bi++) {
    let allDays: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
    const perDay = dateDeltas.get(bi);
    if (perDay) {
      for (const d of perDay.values()) allDays = addGitDeltaSums(allDays, d);
    }
    assertGitDeltaSumsEqual(allDays, ulocPartitioned[bi] ?? { added: 0, removed: 0, edited: 0 }, `commit: all micro-commit days for ${bundles[bi]!.label} do not add up to the bundle uloc total`);
  }
  let bundleSum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const t of ulocPartitioned) bundleSum = addGitDeltaSums(bundleSum, t);
  assertGitDeltaSumsEqual(bundleSum, ulocRangeTotal, "commit: all bundle header uloc totals do not add up to the WIP range — fix bundle attribution");
  let bundleSizeSum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const t of sizePartitioned) bundleSizeSum = addGitDeltaSums(bundleSizeSum, t);
  assertGitDeltaSumsEqual(bundleSizeSum, sizeRangeTotal, "commit: all bundle header size totals do not add up to the WIP range — fix bundle attribution");
  const metrics = buildCommitMetricsForRange(root, base, head, undefined, runner);
  validateCommitMetricsDeltaSum(metrics);
  const ulocLangTotal = sumMicroCommitLangMetrics(metrics.uloc);
  assertGitDeltaSumsEqual({ added: ulocLangTotal.added, edited: ulocLangTotal.edited, removed: ulocLangTotal.removed }, ulocRangeTotal, "commit: footer per-language 📃uloc does not add up to the WIP range total");
  const sizeLangTotal = sumMicroCommitLangMetrics(metrics.size);
  assertGitDeltaSumsEqual({ added: sizeLangTotal.added, edited: sizeLangTotal.edited, removed: sizeLangTotal.removed }, sizeRangeTotal, "commit: footer per-language 💾size does not add up to the WIP range total");
}
