/** 🏗️ Canonical 🛣️path source service. */
import { posix } from "node:path";

export const SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

/** 🔢️ Orders two well-formed strings exactly as their UTF-8 bytes (Unicode code point order) without allocating: UTF-16 code units already agree except that surrogates (U+10000 and above) must sort after U+E000–U+FFFF. */
export function utf8Compare(left: string, right: string): number {
  const length = Math.min(left.length, right.length);
  for (let index = 0; index < length; index++) {
    const a = left.charCodeAt(index), b = right.charCodeAt(index);
    if (a === b) continue;
    const ordered = (a >= 0xd800 ? (a <= 0xdfff ? a + 0x2000 : a - 0x800) : a) - (b >= 0xd800 ? (b <= 0xdfff ? b + 0x2000 : b - 0x800) : b);
    return ordered < 0 ? -1 : 1;
  }
  return left.length === right.length ? 0 : left.length < right.length ? -1 : 1;
}

export function normalizeRelative(value: string): string {
  return sourceRelative(value).normalize("NFC");
}

export function sourceRelative(value: string): string {
  const slash = value.replaceAll("\\", "/").replace(/^\.\//, "");
  const normalized = posix.normalize(slash);
  if (normalized === ".") return "";
  if (normalized === ".." || normalized.startsWith("..") || normalized.startsWith("/") || normalized.includes("\u0000")) throw new Error(`Path escapes repository scope: ${value}`);
  return normalized.replace(/\/$/, "");
}

export function inScope(path: string, scope?: string): boolean {
  if (!scope) return true;
  const normalizedScope = normalizeRelative(scope);
  const normalizedPath = normalizeRelative(path);
  return normalizedPath === normalizedScope || normalizedPath.startsWith(`${normalizedScope}/`) || normalizedScope.startsWith(`${normalizedPath}/`);
}

export function emojiFold(value: string): string {
  return value.normalize("NFC").replaceAll("\uFE0F", "");
}

function graphemes(value: string): readonly string[] {
  return [...SEGMENTER.segment(value)].map((entry) => entry.segment);
}

export function isEmojiGrapheme(value: string): boolean {
  return /[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(value);
}

export function splitLeadingEmoji(value: string): { emoji: string; rest: string } {
  const first = SEGMENTER.segment(value)[Symbol.iterator]().next().value?.segment;
  if (!first || !isEmojiGrapheme(first)) return { emoji: "", rest: value };
  return { emoji: first, rest: value.slice(first.length) };
}
