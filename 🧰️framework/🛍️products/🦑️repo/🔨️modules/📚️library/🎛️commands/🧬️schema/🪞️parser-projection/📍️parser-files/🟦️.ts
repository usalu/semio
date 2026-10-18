/** 📍️ Where the projected parsers of a schema leaf live: in the leaf itself, or in the `🚪️parsers` folder next to the file that declares the type of the export, so a leaf split into component files keeps one parser file per component. The leaf stays the public face: it re-exports every parser its folders hold.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { codePointOrder, escapeRegExp } from "../🔡️json-text/🟦️.ts";
import { PARSER_REGION, PARSER_REGION_END } from "../🚪️guard-helpers/🟦️.ts";

//#region 🔖️Files
/** 🚪️ The folder, next to a file that declares types, that holds the parsers projected for them. */
export const PARSER_DIRECTORY = "🚪️parsers";

const TYPESCRIPT_LEAF = "🟦️.ts";

/** 📚️ The leaf sources of one run and the files of each leaf's module tree that belong to its own document (the leaf first). */
export interface SourceTree {
  read(path: string): string;
  owned(path: string): readonly string[];
}

/** 🔎️ True when the file itself exports `parse<name>` (a re-export from elsewhere does not count). */
export const ownsParser = (source: string, name: string): boolean => new RegExp(`^export\\s+(?:(?:async\\s+)?function|const|let)\\s+parse${escapeRegExp(name)}\\b`, "m").test(source);

/** 🔎️ True when the file itself declares the type `name` (a re-export from elsewhere does not count). */
export const declaresHere = (source: string, name: string): boolean => new RegExp(`^\\s*(?:export\\s+)?(?:declare\\s+)?(?:interface|type|const|class)\\s+${escapeRegExp(name)}\\b`, "m").test(source);

/** 🔎️ The parsers a file exports, in file order. */
export const ownedParserNames = (source: string): string[] => [...source.matchAll(/^export\s+(?:(?:async\s+)?function|const|let)\s+parse([A-Z][A-Za-z0-9]*)\b/gmu)].map((match) => match[1]!);

/** 🧭️ The file path a specifier names from `from`, always with a leading `./` or `../`. */
export function specifierBetween(from: string, to: string): string {
  const path = relative(dirname(from), to).split("\\").join("/");
  return path.startsWith(".") ? path : `./${path}`;
}

/** 📍️ The files of one leaf that may hold its parsers, and where each parser belongs: the file that already defines it, else the parser folder of the file that declares its type, else the leaf. */
export class ParserFiles {
  constructor(private readonly sources: SourceTree, readonly leaf: string) {}

  get files(): readonly string[] {
    return this.sources.owned(this.leaf);
  }

  definingFile(name: string): string | undefined {
    return this.files.find((file) => ownsParser(this.sources.read(file), name));
  }

  typeFile(name: string): string {
    return this.files.find((file) => declaresHere(this.sources.read(file), name)) ?? this.leaf;
  }

  placementOf(name: string): string {
    const file = this.typeFile(name);
    return file === this.leaf ? this.leaf : join(dirname(file), PARSER_DIRECTORY, TYPESCRIPT_LEAF);
  }
}
//#endregion 🔖️Files

//#region 🔖️Regions
const HEADER = "/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.\n * @see ../🟦️.ts\n */\n";

/** 🚪️ A new parser file: its header and the parsers inside the parser region. */
export const parserFileText = (parsers: string): string => `${HEADER}\n${PARSER_REGION}\n${parsers}\n${PARSER_REGION_END}\n`;

/** 🚪️ `parsers` appended to the parser region of a parser file (inside it, before its end marker). */
export function appendParsers(source: string, parsers: string): string {
  const end = source.lastIndexOf(PARSER_REGION_END);
  if (!source.includes(PARSER_REGION)) return `${source.trimEnd()}\n\n${PARSER_REGION}\n${parsers}\n${PARSER_REGION_END}\n`;
  if (end < source.indexOf(PARSER_REGION)) return `${source.trimEnd()}\n\n${parsers}\n`;
  return `${source.slice(0, end).trimEnd()}\n\n${parsers}\n${source.slice(end)}`;
}

/** 🔗️ `source` with the statement that re-exports `names` from `specifier`: an existing statement for that specifier is rewritten, otherwise one is appended. */
export function withReexport(source: string, specifier: string, names: readonly string[]): string {
  const quoted = JSON.stringify(specifier);
  const statement = `export { ${[...names].sort(codePointOrder).join(", ")} } from ${quoted};`;
  const existing = new RegExp(`^export \\{[^}]*\\} from ${escapeRegExp(quoted)};[ \\t]*$`, "m");
  if (existing.test(source)) return source.replace(existing, () => statement);
  const tail = source.trimEnd();
  return `${tail}${/\nexport \{[^}]*\} from "[^"\n]+";$/u.test(tail) ? "\n" : "\n\n"}${statement}\n`;
}
//#endregion 🔖️Regions

//#region 🔖️Symbols
/** 🔗️ Where a name comes from: the file (or bare specifier) that exports it, and whether it is a type. */
export interface Binding {
  readonly from: string;
  readonly type: boolean;
}

const EXPORTED = /^export\s+(?:declare\s+)?(?:abstract\s+)?(interface|type|const|let|var|class|enum|(?:async\s+)?function)\s+([A-Za-z_$][\w$]*)/gmu;
const DECLARED = /^(?:export\s+)?(?:declare\s+)?(?:abstract\s+)?(?:interface|type|const|let|var|class|enum|(?:async\s+)?function)\s+([A-Za-z_$][\w$]*)/gmu;
const IMPORTS = /^import\s+(type\s+)?\{([^}]*)\}\s*from\s*"([^"\n]+)";?[ \t]*\n?/gmu;

/** 🔗️ The names a file exports by declaration, each marked as a type or a value. */
export function exportedNames(source: string): Map<string, boolean> {
  const found = new Map<string, boolean>();
  for (const match of source.matchAll(EXPORTED)) found.set(match[2]!, match[1] === "interface" || match[1] === "type");
  return found;
}

/** 🔗️ The names a file imports under their own name, with the specifier and whether the import is type-only. */
export function importedNames(source: string): Map<string, { readonly spec: string; readonly type: boolean }> {
  const found = new Map<string, { readonly spec: string; readonly type: boolean }>();
  for (const match of source.matchAll(IMPORTS)) {
    for (const item of match[2]!.split(",").map((part) => part.trim()).filter((part) => part !== "")) {
      const inline = item.startsWith("type ");
      const name = item.replace(/^type\s+/u, "");
      if (!/\sas\s/u.test(name)) found.set(name, { spec: match[3]!, type: match[1] !== undefined || inline });
    }
  }
  return found;
}

/** 🔗️ Every name a file declares or imports at its top level. */
export const boundNames = (source: string): Set<string> => new Set([...[...source.matchAll(DECLARED)].map((match) => match[1]!), ...importedNames(source).keys()]);

/** 🔗️ The table of the names the files of one module tree export (a declaration wins over an import of the same name). */
export function symbolTable(sources: SourceTree, files: readonly string[]): Map<string, Binding> {
  const table = new Map<string, Binding>();
  for (const file of files) for (const [name, type] of exportedNames(sources.read(file))) if (!table.has(name)) table.set(name, { from: file, type });
  for (const file of files) {
    for (const [name, bound] of importedNames(sources.read(file))) if (!table.has(name)) table.set(name, { from: bound.spec.startsWith(".") ? resolve(dirname(file), bound.spec) : bound.spec, type: bound.type });
  }
  return table;
}

const LITERAL = /"(?:[^"\\\n]|\\.)*"|'(?:[^'\\\n]|\\.)*'|`(?:[^`\\]|\\.)*`/gu;

/** ✂️ The identifiers a text mentions; with `literals` false the contents of string and template literals are left out. */
export const identifiersOf = (text: string, literals: boolean): Set<string> => new Set((literals ? text : text.replace(LITERAL, '""')).match(/[A-Za-z_$][\w$]*/gu) ?? []);

/** ✂️ A source without its import statements and its re-export statements. */
export const withoutModuleLines = (source: string): string => source.replace(IMPORTS, "").replace(/^export\s+(?:type\s+)?\{[^}]*\}\s*from\s*"[^"\n]+";?[ \t]*\n?/gmu, "");

/** 🧩️ The import statements `file` needs for the table names that `names` mention and the file neither declares nor imports; `skip` names are provided elsewhere. */
export function importStatements(file: string, source: string, names: Iterable<string>, table: ReadonlyMap<string, Binding>, skip: ReadonlySet<string> = new Set()): string[] {
  const bound = boundNames(source);
  const groups = new Map<string, Set<string>>();
  for (const name of names) {
    const binding = table.get(name);
    if (binding === undefined || bound.has(name) || skip.has(name) || binding.from === file) continue;
    const key = `${isAbsolute(binding.from) ? specifierBetween(file, binding.from) : binding.from}\u0000${binding.type ? "type " : ""}`;
    if (!groups.has(key)) groups.set(key, new Set());
    groups.get(key)!.add(name);
  }
  return [...groups.keys()].sort(codePointOrder).map((key) => {
    const [spec, kind] = key.split("\u0000") as [string, string];
    return `import ${kind}{ ${[...groups.get(key)!].sort(codePointOrder).join(", ")} } from ${JSON.stringify(spec)};`;
  });
}

/** ✂️ `source` without the imported `candidates` that nothing in the file mentions any more. */
export function pruneImports(source: string, candidates: ReadonlySet<string>): string {
  const used = identifiersOf(withoutModuleLines(source), true);
  return source.replace(IMPORTS, (whole, type: string | undefined, list: string, spec: string) => {
    const items = list.split(",").map((part) => part.trim()).filter((part) => part !== "");
    const kept = items.filter((item) => {
      const local = item.replace(/^type\s+/u, "").split(/\s+as\s+/u).at(-1)!.trim();
      return !candidates.has(local) || used.has(local);
    });
    return kept.length === items.length ? whole : kept.length === 0 ? "" : `import ${type ?? ""}{ ${kept.join(", ")} } from "${spec}";\n`;
  });
}
//#endregion 🔖️Symbols
