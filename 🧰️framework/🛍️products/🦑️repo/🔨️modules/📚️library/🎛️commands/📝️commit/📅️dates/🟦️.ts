import type { GitDeltaSum } from "../🧾️bundle-model/🟦️.ts";
import { formatBundleMetricSuffixes } from "../📊️metrics/🎨️format/🟦️.ts";

const BUNDLE_COMMIT_TIMESTAMP_LINE_RE = /^🎆️\d{2}🌙️\d{2}☀️\d{2}⏰️/u;

/** 🎆️Calendar day from a micro-commit subject (`🎆️YY🌙️MM☀️DD`). */
export function extractBundleDateLineFromSubject(subject: string): string | null {
  const m = /🎆️\d{2}🌙️\d{2}☀️\d{2}/u.exec(subject.trim());
  return m?.[0] ?? null;
}

/** 📅️Calendar day from the micro-commit body timestamp line (`🎆️YY🌙️MM☀️DD⏰️…`). */
export function extractBundleDateLineFromCommitBody(body: string): string | null {
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    if (!BUNDLE_COMMIT_TIMESTAMP_LINE_RE.test(line)) continue;
    const m = /^🎆️\d{2}🌙️\d{2}☀️\d{2}/u.exec(line);
    return m?.[0] ?? null;
  }
  return null;
}

/** 📆️Calendar day for bundle per-day uloc (body timestamp, else subject). */
export function extractBundleDateLineFromCommit(subject: string, body: string): string | null {
  return extractBundleDateLineFromCommitBody(body) ?? extractBundleDateLineFromSubject(subject);
}

/** 🧹️Strips hand-written per-day uloc from a date section line. */
export function normalizeBundleDateLine(line: string): string {
  return line
    .trim()
    .replace(/📊️metric.*$/u, "")
    .trim();
}

/** 🗓️Bundle squash only: `🎆️YY🌙️MM☀️DD` + full per-day `📊️metric…` suffixes. */
export function formatBundleDateLine(dateLine: string, ulocDelta: GitDeltaSum, sizeDelta: GitDeltaSum, ulocBloc: number, sizeBloc: number): string {
  return `${normalizeBundleDateLine(dateLine)}${formatBundleMetricSuffixes(ulocDelta, sizeDelta, ulocBloc, sizeBloc)}`;
}
