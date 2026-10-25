import { gitDeltaLineTotal } from "../🔀️deltas/🟦️.ts";
import type { MetricKind, MetricKindId } from "../🏷️types/🟦️.ts";

/** 🐙️Appends `➕️` `✏️` `➖️` and total `🟰️` (sum of the three) when non-zero. */
export function appendGitDeltaSuffix(line: string, d: { added: number; removed: number; edited: number }): string {
  if (d.added > 0) line += `➕️${formatMetricLocCount(d.added)}`;
  if (d.edited > 0) line += `✏️${formatMetricLocCount(d.edited)}`;
  if (d.removed > 0) line += `➖️${formatMetricLocCount(d.removed)}`;
  const sum = gitDeltaLineTotal(d);
  if (sum > 0) line += `🟰️${formatMetricLocCount(sum)}`;
  return line;
}

/** 📊️Full inline `📊️metric…` suffixes (uloc + size) from bloc and git deltas. */
export function formatBundleMetricSuffixes(
  ulocDelta: { added: number; removed: number; edited: number },
  sizeDelta: { added: number; removed: number; edited: number },
  ulocBloc: number,
  sizeBloc: number,
  langEmoji?: string,
  langSlug?: string,
): string {
  return formatMetricBody({ kind: "uloc", code: ulocBloc, ...ulocDelta, langEmoji, langSlug }) + formatMetricBody({ kind: "size", code: sizeBloc, ...sizeDelta, langEmoji, langSlug });
}

/** 🎁️Full `📊️uloc💯️…` metrics line from bloc and git deltas (bundle header, footer, languages). */
export function formatBundleUlocSuffix(d: { added: number; removed: number; edited: number }, code: number, langEmoji?: string, langSlug?: string): string {
  return formatMetricBody({ kind: "uloc", code, ...d, langEmoji, langSlug });
}

/** 🪧️Rounds to `digits` significant figures (preserves sign; 0 stays 0). */
export function roundSignificant(n: number, digits = 3): number {
  if (!Number.isFinite(n) || n === 0) return 0;
  const sign = n < 0 ? -1 : 1;
  const abs = Math.abs(n);
  const exp = Math.floor(Math.log10(abs));
  const factor = 10 ** (digits - 1 - exp);
  return (sign * Math.round(abs * factor)) / factor;
}

/** 🔤️Renders a coefficient already near 3 significant figures (no scientific notation). */
function formatSignificantCoeff(n: number): string {
  if (!Number.isFinite(n) || n === 0) return "0";
  const rounded = roundSignificant(n, 3);
  if (rounded === 0) return formatTinyPositive(Math.abs(n));
  if (Number.isInteger(rounded) || Math.abs(rounded - Math.round(rounded)) < 1e-12) return String(Math.round(rounded));
  let s = rounded.toPrecision(3);
  if (/e/i.test(s)) s = expandScientificDecimal(Number(s));
  return s.replace(/(\.\d*?)0+$/, "$1").replace(/\.$/, "");
}

/** 🌱️Expands a tiny positive value without scientific notation (1 significant digit). */
function formatTinyPositive(n: number): string {
  const one = roundSignificant(n, 1);
  if (one === 0) {
    const place = 10 ** Math.floor(Math.log10(n));
    return expandScientificDecimal(place);
  }
  return expandScientificDecimal(one);
}

/** 🔬️Decimal string for a finite number without `e` notation. */
function expandScientificDecimal(n: number): string {
  if (!Number.isFinite(n) || n === 0) return "0";
  const abs = Math.abs(n);
  if (abs >= 1e-6 && abs < 1e21) {
    const decimals = Math.max(0, Math.min(16, -Math.floor(Math.log10(abs)) + 2));
    return (
      (n < 0 ? "-" : "") +
      abs
        .toFixed(decimals)
        .replace(/(\.\d*?)0+$/, "$1")
        .replace(/\.$/, "")
    );
  }
  let s = abs
    .toFixed(20)
    .replace(/(\.\d*?)0+$/, "$1")
    .replace(/\.$/, "");
  return n < 0 ? `-${s}` : s;
}

/** 🔢️Formats metric counts with 3 significant digits and k/M (e.g. 422377 → 422k, 1300000 → 1.3M). */
export function formatMetricLocCount(n: number): string {
  const v = Math.abs(n);
  if (!Number.isFinite(v) || v <= 0) return "0";
  let rounded = roundSignificant(v, 3);
  if (rounded === 0) rounded = roundSignificant(v, 1);
  if (rounded === 0) return formatTinyPositive(v);
  if (rounded >= 1_000_000) return `${formatSignificantCoeff(rounded / 1_000_000)}M`;
  if (rounded >= 1_000) return `${formatSignificantCoeff(rounded / 1_000)}k`;
  return formatSignificantCoeff(rounded);
}

/** ➗️Formats net/previous as a percentage (*100) with 3 significant digits; never formats a positive value as 0. */
export function formatMetricRatio(n: number): string {
  if (!Number.isFinite(n) || n <= 0) return "0";
  // Caller passes a fraction (net/previous); display as percent.
  const pct = n * 100;
  let v = roundSignificant(pct, 3);
  if (v === 0) v = roundSignificant(pct, 1);
  if (v === 0) return formatTinyPositive(pct);
  if (v >= 1) return formatSignificantCoeff(v);
  // Short decimals by magnitude for sub-1% values.
  let places: number;
  if (v >= 0.1) places = 3;
  else if (v >= 0.01) places = 2;
  else if (v >= 0.001) places = 3;
  else places = 4;
  const factor = 10 ** places;
  const capped = Math.round(v * factor) / factor;
  if (capped === 0) return formatSignificantCoeff(1 / factor);
  return formatSignificantCoeff(capped);
}

/** 🏷️Lowercase metrics slug for a language bucket (`TypeScript` → `typescript`). */
export function langMetricsSlug(lang: string): string {
  const fixed: Record<string, string> = {
    TypeScript: "typescript",
    JavaScript: "javascript",
    "C#": "csharp",
    JSON: "json",
    YAML: "yaml",
    TOML: "toml",
    Markdown: "markdown",
    Dockerfile: "dockerfile",
    Makefile: "makefile",
    "Bourne Shell": "shell",
    "Bourne Again Shell": "shell",
    PowerShell: "powershell",
    TeX: "tex",
    HTML: "html",
    CSS: "css",
    SQL: "sql",
    CSV: "csv",
    XML: "xml",
    Go: "go",
    Python: "python",
    Rust: "rust",
    Shell: "shell",
    Docker: "docker",
  };
  if (fixed[lang]) return fixed[lang];
  return lang
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "")
    .slice(0, 24);
}

/** 🧮️Formats byte counts with 3 significant digits and B/KB/MB/GB. */
export function formatMetricSizeCount(n: number): string {
  const v = Math.abs(n);
  if (!Number.isFinite(v) || v <= 0) return "0";
  let rounded = roundSignificant(v, 3);
  if (rounded === 0) rounded = roundSignificant(v, 1);
  if (rounded === 0) return formatTinyPositive(v);
  if (rounded >= 1_000_000_000) return `${formatSignificantCoeff(rounded / 1_000_000_000)}GB`;
  if (rounded >= 1_000_000) return `${formatSignificantCoeff(rounded / 1_000_000)}MB`;
  if (rounded >= 1_000) return `${formatSignificantCoeff(rounded / 1_000)}KB`;
  return `${formatSignificantCoeff(rounded)}B`;
}

export const METRIC_KIND_ULOC: MetricKind = { id: "uloc", token: "📃uloc", formatCount: formatMetricLocCount };

export const METRIC_KIND_SIZE: MetricKind = { id: "size", token: "💾size", formatCount: formatMetricSizeCount };

export const METRIC_KINDS: Record<MetricKindId, MetricKind> = { uloc: METRIC_KIND_ULOC, size: METRIC_KIND_SIZE };

/** 🚧️Commit metrics block header. */
export const COMMIT_METRIC_HEADER = "📊️metric";

/** 💯️Emoji for the aggregate total row (internal sums only). */
export const COMMIT_METRIC_TOTAL_EMOJI = "🔢️";

export type MetricBodyInput = {
  kind: MetricKindId;
  code: number;
  added: number;
  edited: number;
  removed: number;
  langEmoji?: string;
  langSlug?: string;
};

export type UlocMetricsBodyInput = MetricBodyInput & { kind?: MetricKindId };

/** 📏️Renders one explicit `📊️metric…` line; omits empty segments. */
export function formatMetricBody(input: MetricBodyInput): string {
  const { kind, code, added, edited, removed, langEmoji, langSlug } = input;
  const metricKind = METRIC_KINDS[kind];
  let line = COMMIT_METRIC_HEADER;
  if (langEmoji && langSlug) line += `${langEmoji}${langSlug}`;
  line += metricKind.token;
  const bloc = Math.max(0, Math.round(code));
  if (bloc > 0) line += `💯️${metricKind.formatCount(bloc)}`;
  const net = added - removed;
  if (net !== 0) {
    const absNet = Math.abs(net);
    line += net > 0 ? `📈️${metricKind.formatCount(absNet)}` : `📉️${metricKind.formatCount(absNet)}`;
    const previous = bloc - net;
    if (previous > 0) line += `➗️${formatMetricRatio(absNet / previous)}`;
  }
  if (added > 0) line += `➕️${metricKind.formatCount(added)}`;
  if (edited > 0) line += `✏️${metricKind.formatCount(edited)}`;
  if (removed > 0) line += `➖️${metricKind.formatCount(removed)}`;
  const sum = gitDeltaLineTotal({ added, edited, removed });
  if (sum > 0) line += `🟰️${metricKind.formatCount(sum)}`;
  return line;
}

/** 🖌️Renders one explicit `📊️metric📃uloc…` line (uloc alias). */
export function formatUlocMetricsBody(input: Omit<MetricBodyInput, "kind"> & { kind?: MetricKindId }): string {
  return formatMetricBody({ kind: input.kind ?? "uloc", code: input.code, added: input.added, edited: input.edited, removed: input.removed, langEmoji: input.langEmoji, langSlug: input.langSlug });
}
