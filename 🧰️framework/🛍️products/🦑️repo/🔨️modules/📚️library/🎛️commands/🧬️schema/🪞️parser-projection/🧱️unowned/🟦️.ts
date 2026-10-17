/** 🧱️ The registry of hand-written parsers that no schema can own yet: each one validates something the normative JSON Schema does not state (the owned in-memory words behind a transport, an iterative walk that never recurses), so the projection must neither regenerate it nor accept new drift next to it. The registry is data (`🔣️.json`); the freshness gate fails on a drifted parser that is not listed and on a listed parser that no longer drifts, so the list only shrinks.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { asDict, isDict } from "../🔡️json-text/🟦️.ts";

//#region 🔖️Registry
/** 🧱️ One hand-written parser (`path` is its repository-relative leaf) and why its schema cannot own it (a key of `reasons`). */
export interface UnownedParser {
  readonly path: string;
  readonly name: string;
  readonly reason: string;
}

/** 🧱️ The registry file: the reasons with their explanation, and the listed parsers. */
export interface UnownedRegistry {
  readonly reasons: Readonly<Record<string, string>>;
  readonly parsers: readonly UnownedParser[];
}

/** 🔑️ The key a parser is looked up by. */
export const unownedKey = (path: string, name: string): string => `${path}\u0000${name}`;

/** 📥️ Reads and validates the registry; a missing reason or a malformed row is refused. */
export function loadUnowned(location: URL = new URL("./🔣️.json", import.meta.url)): UnownedRegistry {
  const document: unknown = JSON.parse(readFileSync(fileURLToPath(location), "utf8"));
  const reasons = asDict(isDict(document) ? document.reasons : undefined);
  const rows = isDict(document) && Array.isArray(document.parsers) ? document.parsers : [];
  const parsers = rows.map((row, at): UnownedParser => {
    const entry = asDict(row);
    if (typeof entry.path !== "string" || typeof entry.name !== "string" || typeof entry.reason !== "string" || typeof reasons[entry.reason] !== "string") throw new Error(`[schema parsers] unowned parser ${at} is malformed or names a reason the registry does not explain.`);
    return { path: entry.path, name: entry.name, reason: entry.reason };
  });
  return { reasons: reasons as Readonly<Record<string, string>>, parsers };
}
//#endregion 🔖️Registry
