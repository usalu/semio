import { BUNDLE_DATE_SECTION_RE } from "../🧾️bundle-model/🟦️.ts";

const EMOJI_CLUSTER_RE = /^(\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)/u;

const BUNDLE_SCOPE_RESERVED_RE = /🔀️|🚩️|📊️metric|🔢️/u;

const LABEL_TOKEN_BLOCKLIST = new Set(["uloc", "repo", "the", "and"]);

/** 🔢️Leading emoji clusters on a line. */
export function leadingEmojiClusterCount(line: string): number {
  let n = 0;
  let rest = line.trim();
  while (true) {
    const m = EMOJI_CLUSTER_RE.exec(rest);
    if (!m) break;
    n++;
    rest = rest.slice(m[0].length);
  }
  return n;
}

/** 🔖️Emoji clusters anywhere on a line (bundle labels interleave emoji + text). */
export function emojiClusterCountInLine(line: string): number {
  const re = /\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/gu;
  return [...line.matchAll(re)].length;
}

/** 🧹️Strips hand-written uloc and reserved bundle/tag emojis from a scope line. */
export function normalizeBundleScopeLabel(line: string): string {
  return line
    .trim()
    .replace(/📊️metric.*$/u, "")
    .replace(/🔀️|🚩️/gu, "")
    .trim();
}

/** 🏷️ASCII tokens from a bundle emoji label (for matching git paths internally). */
export function labelPathTokens(label: string): string[] {
  const text = normalizeBundleScopeLabel(label)
    .replace(/\p{Extended_Pictographic}/gu, " ")
    .trim()
    .toLowerCase();
  return text.split(/[^a-z0-9]+/).filter((t) => t.length >= 2 && !LABEL_TOKEN_BLOCKLIST.has(t));
}

/** 🚫️Validates bundle scope label before parse. */
export function bundleScopeLabelError(line: string): string | null {
  const raw = line.trim();
  if (raw.includes("|") || raw.includes("/")) {
    return "commit: bundle scope must be emoji + area name only — no paths or `|`";
  }
  if (BUNDLE_SCOPE_RESERVED_RE.test(raw)) {
    return "commit: bundle scope must not include 🔀️ 🚩️ 📊️metric or 🔢️ — script adds subject and metrics";
  }
  const norm = normalizeBundleScopeLabel(raw);
  if (!norm) return "commit: empty bundle scope after removing reserved emojis";
  const tokens = labelPathTokens(norm);
  if (tokens.length === 0) {
    return "commit: bundle scope needs an area name after emojis (e.g. 🏘️compose✍️sketchpad, 🥅️framework, 🖱️ui⚛️react)";
  }
  if (!isBundleScopeLine(norm)) {
    return `commit: invalid bundle scope: ${raw}`;
  }
  return null;
}

/** 📦️Bundle scope line: two+ emojis, or one emoji + lowercase technology slug (`🥅️framework`). */
export function isBundleScopeLine(line: string): boolean {
  const t = normalizeBundleScopeLabel(line);
  if (!t || BUNDLE_DATE_SECTION_RE.test(t)) return false;
  if (t.includes("/") || t.includes("|")) return false;
  if (BUNDLE_SCOPE_RESERVED_RE.test(t)) return false;
  if (emojiClusterCountInLine(t) >= 2) return true;
  if (emojiClusterCountInLine(t) !== 1) return false;
  const after = t.replace(/^\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/u, "");
  return /^[a-z][a-z0-9]{2,24}$/u.test(after);
}
