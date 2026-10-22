import { join } from "node:path";
import { existsSync, readFileSync, statSync } from "node:fs";
import { gitRepoRoot, gitTrackedPaths } from "../../🌳️git/🟦️.ts";
import { accumulateRangeSizeDeltas, accumulateRangeUlocDeltas, accumulateStagedSizeDeltas, accumulateStagedUlocDeltas, accumulateUlocDeltasFromPaths, gitDeltaLineTotal, gitRangeNumstat, pathUnderPrefixes } from "../🔀️deltas/🟦️.ts";
import { classifyPathForMetrics, langMetricsEmoji, MAX_METRICS_FILE_BYTES, shouldSkipPathForUloc } from "../🛤️path-rules/🟦️.ts";
import { countUnifiedLocForFile } from "../🔢️uloc/🟦️.ts";
import type { MetricKindId, MetricsRunner, MicroCommitLangMetrics, UlocByLanguage } from "../🏷️types/🟦️.ts";
import { createDefaultMetricsRunner } from "../💾️scan/🟦️.ts";
import { COMMIT_METRIC_TOTAL_EMOJI, formatMetricBody, langMetricsSlug, METRIC_KINDS } from "../🎨️format/🟦️.ts";
import {refuseCommitV1} from "../../📖️texts/🟦️.ts";

/** 📂️Sums unified LOC for tracked paths under prefixes (bundle-scoped 💯️). */
export function countUnifiedLocUnderPathPrefixes(root: string, prefixes: string[]): number {
  const repoRoot = gitRepoRoot(root);
  let total = 0;
  for (const rel of gitTrackedPaths(root)) {
    if (prefixes.length > 0 && !pathUnderPrefixes(rel, prefixes)) continue;
    if (shouldSkipPathForUloc(repoRoot, rel)) continue;
    const lang = classifyPathForMetrics(rel);
    if (!lang) continue;
    const fp = join(repoRoot, rel);
    if (!existsSync(fp)) continue;
    try {
      const st = statSync(fp);
      if (!st.isFile() || st.size > MAX_METRICS_FILE_BYTES) continue;
    } catch {
      continue;
    }
    let data: string;
    try {
      const buf = readFileSync(fp);
      if (buf.includes(0)) continue;
      data = buf.toString("utf8");
    } catch {
      continue;
    }
    const loc = countUnifiedLocForFile(rel, data);
    if (loc > 0) total += loc;
  }
  return total;
}

/** 📂️Sums on-disk bytes for tracked paths under prefixes (bundle-scoped size 💯️). */
export function countBytesUnderPathPrefixes(root: string, prefixes: string[]): number {
  const repoRoot = gitRepoRoot(root);
  let total = 0;
  for (const rel of gitTrackedPaths(root)) {
    if (prefixes.length > 0 && !pathUnderPrefixes(rel, prefixes)) continue;
    if (shouldSkipPathForUloc(repoRoot, rel)) continue;
    const lang = classifyPathForMetrics(rel);
    if (!lang) continue;
    const fp = join(repoRoot, rel);
    if (!existsSync(fp)) continue;
    try {
      const st = statSync(fp);
      if (!st.isFile() || st.size <= 0) continue;
      total += st.size;
    } catch {
      continue;
    }
  }
  return total;
}

function sortMetricLanguages(codeByLang: UlocByLanguage, deltas: Map<string, { edited: number }>): string[] {
  const langs = new Set<string>();
  for (const [lang, n] of Object.entries(codeByLang)) {
    if (n > 0) langs.add(lang);
  }
  for (const lang of deltas.keys()) langs.add(lang);
  return [...langs].sort((a, b) => (codeByLang[b] ?? 0) - (codeByLang[a] ?? 0) || a.localeCompare(b));
}

function buildLangMetricsFromDeltas(codeByLang: UlocByLanguage, deltas: Map<string, { added: number; removed: number; edited: number }>): MicroCommitLangMetrics[] {
  const rows: MicroCommitLangMetrics[] = [];
  for (const lang of sortMetricLanguages(codeByLang, deltas)) {
    const d = deltas.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    const code = codeByLang[lang] ?? 0;
    if (code === 0 && d.edited === 0 && d.added === 0 && d.removed === 0) continue;
    rows.push({
      lang,
      emoji: langMetricsEmoji(lang),
      code,
      edited: d.edited,
      added: d.added,
      removed: d.removed,
    });
  }
  return rows;
}

function buildMicroCommitMetricsFromDeltas(codeByLang: UlocByLanguage, deltas: Map<string, { added: number; removed: number; edited: number }>): MicroCommitLangMetrics[] {
  return buildLangMetricsFromDeltas(codeByLang, deltas);
}

export type CommitMetricsBundle = {
  uloc: MicroCommitLangMetrics[];
  size: MicroCommitLangMetrics[];
};

/** 📊️Builds uloc + size metrics for staged changes. */
export function buildCommitMetrics(root: string, runner: MetricsRunner = createDefaultMetricsRunner()): CommitMetricsBundle {
  const repoRoot = gitRepoRoot(root);
  const ulocDeltas = accumulateStagedUlocDeltas(repoRoot);
  const sizeDeltas = accumulateStagedSizeDeltas(repoRoot);
  const ulocByLang = runner.countRepoUlocByLanguage(repoRoot);
  const sizeByLang = runner.countRepoSizeByLanguage(repoRoot);
  return {
    uloc: buildLangMetricsFromDeltas(ulocByLang, ulocDeltas),
    size: buildLangMetricsFromDeltas(sizeByLang, sizeDeltas),
  };
}

/** 📊️Builds uloc + size metrics for a git revision range (optional path prefixes). */
export function buildCommitMetricsForRange(root: string, base: string, head = "HEAD", pathPrefixes?: string[], runner: MetricsRunner = createDefaultMetricsRunner()): CommitMetricsBundle {
  const repoRoot = gitRepoRoot(root);
  const numstat = gitRangeNumstat(repoRoot, base, head);
  const ulocDeltas = accumulateUlocDeltasFromPaths(repoRoot, numstat, base, head, pathPrefixes);
  const sizeDeltas = accumulateRangeSizeDeltas(repoRoot, base, head, pathPrefixes);
  const ulocByLang = runner.countRepoUlocByLanguage(repoRoot);
  const sizeByLang = runner.countRepoSizeByLanguage(repoRoot);
  return {
    uloc: buildLangMetricsFromDeltas(ulocByLang, ulocDeltas),
    size: buildLangMetricsFromDeltas(sizeByLang, sizeDeltas),
  };
}

/** 📊️Builds micro-commit uloc metrics: repo uloc per language + staged git deltas. */
export function buildMicroCommitMetrics(root: string, runner: MetricsRunner = createDefaultMetricsRunner()): MicroCommitLangMetrics[] {
  const repoRoot = gitRepoRoot(root);
  const deltas = accumulateStagedUlocDeltas(repoRoot);
  const codeByLang = runner.countRepoUlocByLanguage(repoRoot);
  return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}

/** 📊️Builds uloc metrics for a git revision range (optional path prefixes). */
export function buildMicroCommitMetricsForRange(root: string, base: string, head = "HEAD", pathPrefixes?: string[], runner: MetricsRunner = createDefaultMetricsRunner()): MicroCommitLangMetrics[] {
  const repoRoot = gitRepoRoot(root);
  const deltas = accumulateRangeUlocDeltas(repoRoot, base, head, pathPrefixes);
  const codeByLang = runner.countRepoUlocByLanguage(repoRoot);
  return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}

/** 📊️Sums per-language metric rows into one total. */
export function sumMicroCommitLangMetrics(metrics: MicroCommitLangMetrics[]): MicroCommitLangMetrics {
  const total: MicroCommitLangMetrics = {
    lang: "Total",
    emoji: COMMIT_METRIC_TOTAL_EMOJI,
    code: 0,
    edited: 0,
    added: 0,
    removed: 0,
  };
  for (const m of metrics) {
    total.code += m.code;
    total.edited += m.edited;
    total.added += m.added;
    total.removed += m.removed;
  }
  return total;
}

/** 📊️Formats one per-language metric row for a kind. */
export function formatCommitMetricLine(kind: MetricKindId, m: MicroCommitLangMetrics): string {
  return formatMetricBody({
    kind,
    code: m.code,
    added: m.added,
    edited: m.edited,
    removed: m.removed,
    langEmoji: m.emoji,
    langSlug: langMetricsSlug(m.lang),
  });
}

/** 📊️Formats one per-language `📊️metric🦀️rust📃uloc…` row. */
export function formatMicroCommitMetricLine(m: MicroCommitLangMetrics): string {
  return formatCommitMetricLine("uloc", m);
}

function commitMetricRowHasActivity(m: MicroCommitLangMetrics): boolean {
  return m.code > 0 || m.edited > 0 || m.added > 0 || m.removed > 0;
}

/** 📊️Renders commit metrics: repo totals per kind, then per-language uloc+size pairs. */
export function formatCommitMetricsLines(bundle: CommitMetricsBundle): string[] {
  const { uloc, size } = bundle;
  if (uloc.length === 0 && size.length === 0) return [];
  const lines: string[] = [];
  const ulocTotal = sumMicroCommitLangMetrics(uloc);
  const sizeTotal = sumMicroCommitLangMetrics(size);
  if (commitMetricRowHasActivity(ulocTotal)) {
    lines.push(formatMetricBody({ kind: "uloc", code: ulocTotal.code, added: ulocTotal.added, edited: ulocTotal.edited, removed: ulocTotal.removed }));
  }
  if (commitMetricRowHasActivity(sizeTotal)) {
    lines.push(formatMetricBody({ kind: "size", code: sizeTotal.code, added: sizeTotal.added, edited: sizeTotal.edited, removed: sizeTotal.removed }));
  }
  const sizeByLang = new Map(size.map((m) => [m.lang, m]));
  const langs = sortMetricLanguages(Object.fromEntries(uloc.map((m) => [m.lang, m.code])), new Map(uloc.map((m) => [m.lang, { edited: m.edited }])));
  for (const lang of langs) {
    const u = uloc.find((m) => m.lang === lang);
    const s = sizeByLang.get(lang);
    if (u && commitMetricRowHasActivity(u)) lines.push(formatCommitMetricLine("uloc", u));
    if (s && commitMetricRowHasActivity(s)) lines.push(formatCommitMetricLine("size", s));
  }
  for (const s of size) {
    if (langs.includes(s.lang)) continue;
    if (commitMetricRowHasActivity(s)) lines.push(formatCommitMetricLine("size", s));
  }
  return lines;
}

/** 📊️Renders the unified LOC block for uloc rows alone. */
export function formatMicroCommitMetricsLines(metrics: MicroCommitLangMetrics[]): string[] {
  return formatCommitMetricsLines({ uloc: metrics, size: [] });
}

/** ✅️Whether two git delta sums match on ➕️ ✏️ ➖️ (🟰️ follows). */
export function gitDeltaSumsEqual(a: { added: number; removed: number; edited: number }, b: { added: number; removed: number; edited: number }): boolean {
  return a.added === b.added && a.edited === b.edited && a.removed === b.removed;
}

/** 🚫️Per-language ➕️✏️➖️ must sum to the footer total line for each metric kind. */
export function validateMicroCommitLangMetricsDeltaSum(metrics: MicroCommitLangMetrics[], kind: MetricKindId = "uloc"): void {
  if (metrics.length === 0) return;
  const total = sumMicroCommitLangMetrics(metrics);
  let added = 0;
  let edited = 0;
  let removed = 0;
  for (const m of metrics) {
    added += m.added;
    edited += m.edited;
    removed += m.removed;
  }
  if (!gitDeltaSumsEqual({ added, edited, removed }, total)) {
    const kindToken = METRIC_KINDS[kind].token;
    throw refuseCommitV1("failed","metrics.per-language-deltas-do-not",{kindToken:String(kindToken),added:String(added),edited:String(edited),removed:String(removed),sum:String(gitDeltaLineTotal({ added, edited, removed })),footerAdded:String(total.added),footerEdited:String(total.edited),footerRemoved:String(total.removed),footerSum:String(gitDeltaLineTotal(total))},);
  }
}

/** 🚫️Per-language delta sums for uloc and size footers. */
export function validateCommitMetricsDeltaSum(bundle: CommitMetricsBundle): void {
  validateMicroCommitLangMetricsDeltaSum(bundle.uloc, "uloc");
  validateMicroCommitLangMetricsDeltaSum(bundle.size, "size");
}
