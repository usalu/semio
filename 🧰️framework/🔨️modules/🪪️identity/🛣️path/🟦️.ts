/** 🛣️Explicit path identity evaluation without workspace policy. */
import { leadingEmojiIdentity } from "../🧩️grapheme/🟦️.ts";

/** 📂️One path presented to the language-neutral path-emoji statutes. */
export interface PathEmojiEntry {
  readonly path: string;
  readonly nodeKind: "directory" | "file";
  readonly reserved?: boolean;
}

export type PathEmojiFindingKind = "missing" | "generic" | "presentation" | "spacing" | "duplicate" | "multiple" | "reserved-emoji";

/** ⚠️One deterministic path-emoji statute finding. */
export interface PathEmojiFinding {
  readonly kind: PathEmojiFindingKind;
  readonly path: string;
  readonly sibling?: string;
  readonly emoji?: string;
}

/** 🪞️Folds presentation selectors for logical path-identity comparisons. */
export function foldPathEmojiIdentity(value: string): string {
  return value.normalize("NFC").replaceAll("\uFE0E", "").replaceAll("\uFE0F", "");
}

/** 📖️Identifies reserved documentation names independently of their optional format extension. */
export function reservedDocumentationBasename(name: string): string | null {
  const identity = leadingEmojiIdentity(name);
  return /^(?:README(?:\.[^/]+)?|LICENSE(?:\.[^/]+)?|AGENTS\.md)$/u.test(identity.rest) ? identity.rest : null;
}

/** ⚖️Evaluates path emoji rules in one namespace shared by file and directory siblings. */
export function pathEmojiStatuteFindings(entries: readonly PathEmojiEntry[], genericEmojiIdentities: readonly string[]): PathEmojiFinding[] {
  const generic = new Set(genericEmojiIdentities.map(foldPathEmojiIdentity));
  const seen = new Map<string, PathEmojiEntry>();
  const findings: PathEmojiFinding[] = [];
  const sorted = [...entries].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
  for (const entry of sorted) {
    const name = entry.path.split("/").at(-1) ?? "";
    const identity = leadingEmojiIdentity(name);
    if (entry.nodeKind === "file" && reservedDocumentationBasename(name)) {
      if (identity.emoji) findings.push({ kind: "reserved-emoji", path: entry.path, emoji: identity.emoji });
      continue;
    }
    if (entry.reserved) continue;
    if (!identity.emoji) {
      findings.push({ kind: "missing", path: entry.path });
      continue;
    }
    if (identity.emoji !== identity.first || /[\p{Extended_Pictographic}\p{Emoji_Presentation}\u20E3]/u.test(identity.rest)) findings.push({ kind: "multiple", path: entry.path, emoji: identity.emoji });
    if (generic.has(foldPathEmojiIdentity(identity.first))) findings.push({ kind: "generic", path: entry.path, emoji: identity.first });
    const firstCodePoint = [...identity.first][0] ?? "";
    if (firstCodePoint && !/\p{Emoji_Presentation}/u.test(firstCodePoint) && !identity.first.includes("\uFE0F")) findings.push({ kind: "presentation", path: entry.path, emoji: identity.first });
    if (/^\s/u.test(identity.rest)) findings.push({ kind: "spacing", path: entry.path, emoji: identity.emoji });
    const parent = entry.path.includes("/") ? entry.path.slice(0, entry.path.lastIndexOf("/")) : "";
    const key = `${parent}\0${foldPathEmojiIdentity(identity.first)}`;
    const previous = seen.get(key);
    if (previous) findings.push({ kind: "duplicate", path: entry.path, sibling: previous.path, emoji: foldPathEmojiIdentity(identity.first) });
    else seen.set(key, entry);
  }
  return findings;
}

