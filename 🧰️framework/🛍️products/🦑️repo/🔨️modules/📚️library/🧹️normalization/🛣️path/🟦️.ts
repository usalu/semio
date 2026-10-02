/** 🏗️ Canonical 🛣️path source service. */
import { posix } from "node:path";

export const SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });

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
