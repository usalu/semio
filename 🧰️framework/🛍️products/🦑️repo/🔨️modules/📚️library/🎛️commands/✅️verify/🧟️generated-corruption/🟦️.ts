/** 🧭️ Generated-corruption census behind `verify generated-corruption`.
 * @see 📜️script.ts
 */
import { readFileSync, readdirSync, type Dirent } from "node:fs";
import { createRequire } from "node:module";
import { extname, join, relative } from "node:path";

//#region 🔖️GeneratedCorruptionCensus
/** 🚫️Directories the corruption census never walks: caches, vendored trees and build scratch own no repo source. */
const GENERATED_CORRUPTION_SKIP_DIRS = new Set(["node_modules", ".git", "target", "🎯️target", "⚡️cache", ".venv", ".nx", ".turbo", ".git-rewrite"]);

/** 🧩️The extensions `tsc` parses — the only files where a spliced emoji becomes a TS1127/TS1434 parse error. */
const GENERATED_CORRUPTION_EXTENSIONS = new Set([".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs"]);

/** 🏷️jco stamps every world-level binding file with this first line; it selects generator output without guessing file names. */
const JCO_WORLD_HEADER = "// world ";

/** 📥️`export type * as <Alias> from './interfaces/<x>.js'; // import <wit-id>` — one imported interface of a jco world. */
const JCO_IMPORT_ALIAS_LINE = /^export type \* as (\S+) from '[^']+'; \/\/ import (\S+)\s*$/;

/** 📤️`export * as <alias> from './interfaces/<x>.js'; // export <wit-id>` — one exported interface of a jco world. */
const JCO_EXPORT_ALIAS_LINE = /^export \* as (\S+) from '[^']+'; \/\/ export (\S+)\s*$/;

/** 🔬️An emoji (with its optional VS16 / ZWJ / skin-tone tail) wedged between two ASCII identifier characters. */
const SPLICED_EMOJI_IDENTIFIER = /[A-Za-z0-9_$](\p{Extended_Pictographic}(?:️|︎|‍\p{Extended_Pictographic}|[\u{1F3FB}-\u{1F3FF}])*)[A-Za-z0-9_$]/gu;

/** 🔎️One spliced-emoji identifier, located precisely enough to be fixed without re-running the census. */
type GeneratedCorruptionFinding = {
  readonly kind: "jco-alias" | "code-position";
  readonly file: string;
  readonly line: number;
  readonly detail: string;
};

/** 📊️The whole census: what was looked at, what validated the jco naming rule, and every surviving finding. */
type GeneratedCorruptionCensus = {
  readonly filesScanned: number;
  readonly jcoFilesScanned: number;
  readonly jcoAliasesValidated: number;
  readonly findings: readonly GeneratedCorruptionFinding[];
};

/** 🐫️jco's identifier casing for one kebab/snake WIT segment. */
function jcoPascalSegment(segment: string): string {
  return segment
    .split(/[-_]/)
    .filter((part) => part.length > 0)
    .map((part) => part[0]!.toUpperCase() + part.slice(1))
    .join("");
}

/**
 * 🧬️Re-derives the world-level namespace alias jco emits for one WIT interface id, from the id alone:
 * `wasi:cli/environment@0.2.9` → `WasiCliEnvironment029` for an import, and the bare camelCase interface
 * name (`plugin`) for an export. This is the source of truth the corruption census checks bindings against.
 */
function jcoWorldAlias(witId: string, direction: "import" | "export"): string {
  const [head, version = ""] = witId.split("@");
  const segments = head!.split(/[:/]/).filter((segment) => segment.length > 0);
  if (direction === "export") {
    const last = jcoPascalSegment(segments[segments.length - 1] ?? "");
    return last.slice(0, 1).toLowerCase() + last.slice(1);
  }
  return segments.map(jcoPascalSegment).join("") + version.replace(/[.\-]/g, "");
}

/** 🧾️`ScriptKind` per extension, so JSX and TypeScript syntax are parsed by the rules their file actually uses. */
function typescriptScriptKindFor(fileName: string, ts: TypeScriptParserApi): number {
  if (fileName.endsWith(".tsx")) return ts.ScriptKind.TSX;
  if (fileName.endsWith(".jsx")) return ts.ScriptKind.JSX;
  if (/\.(js|mjs|cjs)$/.test(fileName)) return ts.ScriptKind.JS;
  return ts.ScriptKind.TS;
}

/** 🧰️The sliver of the TypeScript compiler this census uses: a parse, and the errors that parse produced. */
type TypeScriptParserApi = {
  readonly version: string;
  readonly ScriptTarget: { readonly Latest: number };
  readonly ScriptKind: { readonly TS: number; readonly TSX: number; readonly JS: number; readonly JSX: number };
  createSourceFile(fileName: string, text: string, target: number, setParentNodes: boolean, scriptKind: number): { readonly parseDiagnostics?: readonly { readonly start: number }[] };
};

/**
 * 🔬️Walks every TS/JS file the repo owns and reports emoji spliced into ASCII identifiers — the signature of
 * the 2026-09-03 rename-plan codemod incident. Two independent checks, neither of them a heuristic: jco
 * world bindings are validated against {@link jcoWorldAlias}, and every other candidate file is handed to
 * the repo's own TypeScript parser, so an emoji counts as spliced only where the parser reports an error at
 * that exact offset. That is what separates corruption from this repo's legitimate emoji-in-path/label
 * idiom, which parses cleanly inside strings, comments, regex literals and JSX text. Generated output is
 * walked deliberately: it is what the incident corrupted.
 */
export function censusGeneratedCorruption(root: string): GeneratedCorruptionCensus {
  const findings: GeneratedCorruptionFinding[] = [];
  const ts = createRequire(join(root, "package.json"))("typescript") as TypeScriptParserApi;
  let filesScanned = 0;
  let jcoFilesScanned = 0;
  let jcoAliasesValidated = 0;

  const walk = (directory: string): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(directory, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const absolute = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!GENERATED_CORRUPTION_SKIP_DIRS.has(entry.name)) walk(absolute);
        continue;
      }
      if (!entry.isFile() || !GENERATED_CORRUPTION_EXTENSIONS.has(extname(entry.name))) continue;
      let source: string;
      try {
        source = readFileSync(absolute, "utf8");
      } catch {
        continue;
      }
      filesScanned += 1;
      const file = relative(root, absolute);
      if (source.startsWith(JCO_WORLD_HEADER)) {
        jcoFilesScanned += 1;
        source.split("\n").forEach((text, offset) => {
          const asImport = JCO_IMPORT_ALIAS_LINE.exec(text);
          const asExport = asImport ? null : JCO_EXPORT_ALIAS_LINE.exec(text);
          const match = asImport ?? asExport;
          if (!match) return;
          const expected = jcoWorldAlias(match[2]!, asImport ? "import" : "export");
          if (match[1] === expected) {
            jcoAliasesValidated += 1;
            return;
          }
          findings.push({ kind: "jco-alias", file, line: offset + 1, detail: `alias ${JSON.stringify(match[1])} ≠ ${JSON.stringify(expected)} derived from ${match[2]}` });
        });
        continue;
      }
      SPLICED_EMOJI_IDENTIFIER.lastIndex = 0;
      if (!SPLICED_EMOJI_IDENTIFIER.test(source)) continue;
      const parsed = ts.createSourceFile(absolute, source, ts.ScriptTarget.Latest, false, typescriptScriptKindFor(entry.name, ts));
      const errorOffsets = (parsed.parseDiagnostics ?? []).map((diagnostic) => diagnostic.start);
      if (errorOffsets.length === 0) continue;
      const lineStarts = [0];
      for (let at = source.indexOf("\n"); at >= 0; at = source.indexOf("\n", at + 1)) lineStarts.push(at + 1);
      SPLICED_EMOJI_IDENTIFIER.lastIndex = 0;
      let match: RegExpExecArray | null;
      while ((match = SPLICED_EMOJI_IDENTIFIER.exec(source)) !== null) {
        const from = match.index;
        const to = SPLICED_EMOJI_IDENTIFIER.lastIndex;
        if (SPLICED_EMOJI_IDENTIFIER.lastIndex === match.index) SPLICED_EMOJI_IDENTIFIER.lastIndex += 1;
        if (!errorOffsets.some((offset) => offset >= from && offset <= to)) continue;
        let start = from;
        while (start > 0 && /[A-Za-z0-9_$]/.test(source[start - 1]!)) start -= 1;
        let end = to;
        while (end < source.length && /[A-Za-z0-9_$]/.test(source[end]!)) end += 1;
        let line = 1;
        while (line < lineStarts.length && lineStarts[line]! <= from) line += 1;
        findings.push({ kind: "code-position", file, line, detail: `spliced ${match[1]} in identifier ${JSON.stringify(source.slice(start, end))} (TypeScript cannot parse it)` });
      }
    }
  };

  walk(root);
  return { filesScanned, jcoFilesScanned, jcoAliasesValidated, findings };
}
//#endregion 🔖️GeneratedCorruptionCensus
