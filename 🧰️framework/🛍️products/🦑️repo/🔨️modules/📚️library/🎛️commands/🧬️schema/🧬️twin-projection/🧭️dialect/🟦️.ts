/** 🧭️ The dialect of a twin family: how its wire readers are spelled, where its contract lives and which sentence opens each facet. The engine knows no domain; a family declares its dialect in a `🧬️twin-dialect/🔣️.json` beside its schemas.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts
 */
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, sep } from "node:path";
import type { Node, SchemaModule } from "../🧩️model/🟦️.ts";

/** 🗂️ The folder that declares the dialect of every module below it. */
export const DIALECT_DIRECTORY = "🧬️twin-dialect";

/** 🏷️ How a facet folder opens its twin. */
export interface FacetStyle {
  readonly emoji: string;
  readonly summary: string;
}

/** 🧭️ How a family spells its twins. */
export interface Dialect {
  readonly enabled: boolean;
  readonly contractModule: string;
  readonly contractFile?: string;
  readonly readerPrefix: string;
  readonly readerType: string;
  readonly jsonType: string;
  readonly generatedBy: string;
  readonly recordPrefixPattern: RegExp;
  readonly facets: ReadonlyMap<string, FacetStyle>;
  readonly leaf: { readonly summary: string };
}

/** 🔤️ The wire reader a family spells for a reader kind (`Object` to `normWireObject`). */
export const readerName = (dialect: Dialect, kind: string): string => `${dialect.readerPrefix}${kind}`;

/** 📖️ Reads a dialect document. */
export function parseDialect(document: Node, directory: string): Dialect {
  const ts = (document.typescript ?? {}) as Node;
  return {
    enabled: document.projection !== "off",
    contractModule: ts.contractModule,
    contractFile: typeof ts.contractFile === "string" ? join(directory, ts.contractFile) : undefined,
    readerPrefix: ts.readerPrefix,
    readerType: ts.readerType,
    jsonType: ts.jsonType,
    generatedBy: ts.generatedBy,
    recordPrefixPattern: new RegExp(ts.recordPrefix ?? "^$", "u"),
    facets: new Map(Object.entries<FacetStyle>(document.facets ?? {})),
    leaf: document.leaf,
  };
}

const declared = new Map<string, Dialect | null>();

/** 🔎️ The dialect that governs a module: the nearest ancestor folder that declares one, or `undefined` when none does. */
export function dialectOf(module: SchemaModule): Dialect | undefined {
  for (let dir = module.dir; ; dir = dirname(dir)) {
    if (!declared.has(dir)) {
      const path = join(dir, DIALECT_DIRECTORY, "🔣️.json");
      declared.set(dir, existsSync(path) ? parseDialect(JSON.parse(readFileSync(path, "utf8")), dir) : null);
    }
    const dialect = declared.get(dir);
    if (dialect !== null && dialect !== undefined) return dialect;
    if (dirname(dir) === dir) return undefined;
  }
}

/** 🗂️ The facet folder of a module: its text lane, else its own folder name, else the artifact root. */
export function facetOf(module: SchemaModule): string {
  const segments = module.dir.split(sep);
  if (segments.includes("📝️text")) return "📝️text";
  const last = segments.at(-1) ?? "";
  return last === "🧬️schema" ? (segments.at(-2) === "✳️any" ? "🧬️schema" : last) : last;
}
