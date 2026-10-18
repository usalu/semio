import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { normalizeRelative, utf8Compare } from "../🛣️path/🟦️.ts";
import { basename, posix } from "node:path";
import type { ReferenceEdit, TaxonomyReferenceAdapter } from "../📜️plan-contracts/🟦️.ts";
import { type ReferenceToken, lineLocation } from "../🔤️reference-tokenizers/🟦️.ts";

export function addUniqueIndex(index: Map<string, string | null>, key: string, value: string): void {
  if (!key) return;
  const existing = index.get(key);
  if (existing === undefined) index.set(key, value);
  else if (existing !== value) index.set(key, null);
}

/** 🍁️ The NFC final segment every candidate built from `path` must end with, or null when normalization can replace it (an empty, `.` or `..` tail) so no leaf can be promised. */
export function referenceLeaf(path: string): string | null {
  const normalized = posix.normalize(path.replaceAll("\\", "/")).replace(/\/+$/u, ""), leaf = normalized.slice(normalized.lastIndexOf("/") + 1);
  return leaf === "" || leaf === "." || leaf === ".." ? null : leaf.normalize("NFC");
}

/** 🎯️ The final segments a token must name to resolve into a target set: each target's own, NFC and extensionless leaf (exactly the keys
 * {@link referenceLeafNames} admits), plus whether a dotted module token could reach a Python target. */
export interface ReferenceResolutionReach { readonly leaves: ReadonlySet<string>; readonly python: boolean }

export function referenceResolutionReach(targets: Iterable<string>): ReferenceResolutionReach {
  const leaves = new Set<string>(), add = (key: string): void => { leaves.add(key.slice(key.lastIndexOf("/") + 1)); };
  let python = false;
  for (const target of targets) {
    const normalized = target.normalize("NFC");
    add(target);
    add(normalized);
    add(normalized.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, ""));
    python ||= normalized.endsWith(".py");
  }
  return { leaves, python };
}

/** 🪪️ What a token can resolve to, as {@link resolveReferenceTokenPath} reads it: the final segments its physical targets or candidate
 * values name, whether some value has no promisable final segment, and whether some value can take the Python module route. */
export interface ReferenceReachSignature { readonly leaves: Set<string>; nullLeaf: boolean; python: boolean }

/** 🧲️ False only when no token behind a signature can resolve into the reach's target set. */
export function referenceSignatureMayResolveInto(signature: Readonly<{ leaves: Iterable<string>; nullLeaf: boolean; python: boolean }>, reach: ReferenceResolutionReach): boolean {
  if (signature.nullLeaf || signature.python && reach.python) return true;
  for (const leaf of signature.leaves) if (reach.leaves.has(leaf)) return true;
  return false;
}

const opaqueLeafNameLists = new WeakMap<object, readonly string[]>();

/** 🚧️ The last segment of every opaque exclusion: a path can only normalize into an exclusion if it names that segment itself, because the referencing file is never inside one. */
export function opaqueLeafNames(taxonomy: Pick<LoadedTaxonomy, "exclusions">): readonly string[] {
  let names = opaqueLeafNameLists.get(taxonomy.exclusions);
  if (!names) opaqueLeafNameLists.set(taxonomy.exclusions, names = taxonomy.exclusions.map((entry) => normalizeRelative(entry.path).split("/").filter(Boolean).at(-1) ?? "").filter(Boolean));
  return names;
}

/** 📐️ The `[index, matched text, captured value]` of every match of `/"((?:\\.|[^"\\\r\n])+)"|'…'|`…`/gu` in one linear pass: from each quote, the
 * body runs to the first unescaped same quote; an escape cannot cross a line terminator and the body cannot hold `\r`/`\n` or be empty. */
function quotedLiteralMatches(content: string): [number, string, string][] {
  const rows: [number, string, string][] = [];
  for (let index = 0; index < content.length; index++) {
    const quote = content.charAt(index);

    if (quote !== '"' && quote !== "'" && quote !== "`") continue;
    let cursor = index + 1, closed = false;
    while (cursor < content.length) {
      const character = content[cursor]!;
      if (character === quote) { closed = cursor > index + 1; break; }
      if (character === "\r" || character === "\n") break;
      if (character === "\\") {
        const escaped = content.codePointAt(cursor + 1);
        if (escaped === undefined || escaped === 0x0a || escaped === 0x0d || escaped === 0x2028 || escaped === 0x2029) break;
        cursor += escaped > 0xffff ? 3 : 2;
        continue;
      }
      cursor++;
    }
    if (!closed) continue;
    rows.push([index, content.slice(index, cursor + 1), content.slice(index + 1, cursor)]);
    index = cursor;
  }
  return rows;
}

const PATH_RUN_EXCLUDED = new Set([..."\"'`()],;"]), PATH_LOOKAHEAD = new Set([..."\"'`),;]"]), PATH_PREFIXES = new Set([..."(=,:"]);

const REGEX_WHITESPACE = new Set([..."\t\n\v\f\r \u00a0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000\ufeff"]);

const LINE_TERMINATORS = new Set(["\n", "\r", "\u2028", "\u2029"]);

/**
 * 📏️ The `[index, matched text, captured value]` of every match of the unsupported path grammar
 * `/(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s"'`),;\]])/gmu` in one linear pass. From
 * each start (a line start, or just after one of `\s ( = , :`), the slashed form is the maximal run of path characters when it holds a `/` with a
 * character on both sides and ends at input end or a lookahead character; otherwise the dotted form is the maximal `[A-Za-z0-9_.@-]` run when
 * it ends there too and its trailing `[A-Za-z0-9.]` suffix holds a `.` after its first character and before its last.
 */
function pathLiteralMatches(content: string): [number, string, string][] {
  const rows: [number, string, string][] = [];
  const lookahead = (index: number): boolean => index >= content.length || PATH_LOOKAHEAD.has(content[index]!) || REGEX_WHITESPACE.has(content[index]!);
  const word = (code: number): boolean => code >= 48 && code <= 57 || code >= 65 && code <= 90 || code >= 97 && code <= 122 || code === 95 || code === 46 || code === 64 || code === 45;
  const suffix = (code: number): boolean => code >= 48 && code <= 57 || code >= 65 && code <= 90 || code >= 97 && code <= 122 || code === 46;
  let runFrom = -1, runTo = -1, wordFrom = -1, wordTo = -1, suffixLast = -1, suffixFirst = -1, slashAt = -1, dotAt = -1;
  const next = (at: number, from: number, code: number): number => {
    if (at >= from || at === Infinity) return at;
    for (let index = from; index < content.length; index++) if (content.charCodeAt(index) === code) return index;
    return Infinity;
  };
  const valueEnd = (start: number): number => {
    if (runFrom < 0 || start < runFrom || start > runTo) {
      let end = start;
      while (end < content.length && !PATH_RUN_EXCLUDED.has(content[end]!) && !REGEX_WHITESPACE.has(content[end]!)) end++;
      runFrom = start; runTo = end;
    }
    const end = runTo;
    if (end > start && lookahead(end) && (slashAt = next(slashAt, start + 1, 47)) < end - 1) return end;
    if (wordFrom < 0 || start < wordFrom || start > wordTo) {
      let last = start;
      while (last < content.length && word(content.charCodeAt(last))) last++;
      wordFrom = start; wordTo = last;
    }
    const last = wordTo;
    if (last - start < 3 || !lookahead(last)) return -1;
    if (suffixLast !== last) {
      let first = last;
      while (first > 0 && suffix(content.charCodeAt(first - 1))) first--;
      suffixLast = last; suffixFirst = first;
    }
    return (dotAt = next(dotAt, Math.max(suffixFirst, start + 1), 46)) < last - 1 ? last : -1;
  };
  for (let position = 0; position < content.length;) {
    let start = -1, end = -1;
    if (position === 0 || LINE_TERMINATORS.has(content[position - 1]!)) { end = valueEnd(position); if (end >= 0) start = position; }
    if (start < 0 && (PATH_PREFIXES.has(content[position]!) || REGEX_WHITESPACE.has(content[position]!))) { end = valueEnd(position + 1); if (end >= 0) start = position + 1; }
    if (start < 0) { position++; continue; }
    rows.push([position, content.slice(position, end), content.slice(start, end)]);
    position = end;
  }
  return rows;
}

export function unsupportedReferenceTokens(content: string, adapter: TaxonomyReferenceAdapter): readonly ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const matches of [quotedLiteralMatches(content), pathLiteralMatches(content)]) {
    for (const [index, text, value] of matches) {
      if (!value || /^\/\//u.test(value) || /^[\\/]+$/u.test(value) || /^(?:\/{2,}|\.{1,2}\/?|\*+)$/u.test(value) || !/[\\/]/u.test(value) && !/^\.{1,2}$/u.test(value) && !/\.[A-Za-z0-9][A-Za-z0-9.-]*$/u.test(value)) continue;
      const start = index + text.indexOf(value);
      const decoded = value.replace(/\\(?:u\{([a-f0-9]{1,6})\}|u([a-f0-9]{4})|x([a-f0-9]{2})|([\\/"'`]))/giu, (sequence, point: string | undefined, unit: string | undefined, byte: string | undefined, escaped: string | undefined) => {
        const numeric = point ?? unit ?? byte;
        if (!numeric) return escaped ?? sequence;
        const code = Number.parseInt(numeric, 16);
        return code <= 0x10ffff ? String.fromCodePoint(code) : sequence;
      });
      rows.push({ adapter, structuredLocation: lineLocation(content, start, "unsupported-path-syntax"), start, end: start + value.length, value, ...(decoded === value ? {} : { targetValues: [decoded] }) });
    }
  }
  return rows;
}

export function referenceAdapter(path: string): TaxonomyReferenceAdapter {
  const lower = path.toLocaleLowerCase("und");
  if (lower.endsWith(".feature")) return "gherkin";
  if (lower.endsWith(".rs")) return "rust";
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower)) return "typescript";
  if (/\.(?:go|mod|work)$/u.test(lower)) return "go";
  if (lower.endsWith(".py")) return "python";
  if (/\.(?:cs|fs|vb|csproj|fsproj|vbproj|sln|props|targets)$/u.test(lower)) return "dotnet";
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename(path) === "CMakeLists.txt") return "native";
  if (lower.endsWith(".jsonc")) return "jsonc";
  if (lower.endsWith(".json")) return "json";
  if (lower.endsWith(".toml")) return "toml";
  if (/\.ya?ml$/u.test(lower)) return "yaml";
  if (/\.(?:xml|html|htm)$/u.test(lower)) return "xml";
  return "markdown";
}

/** 📍️ The code-unit offset a reference edit's structured location ends with. */
function referenceEditOffset(edit: ReferenceEdit): number {
  const value = edit.structuredLocation.match(/@(\d+)$/)?.[1];
  if (value === undefined) throw new Error(`Reference edit lacks a structured offset at ${edit.path}:${edit.structuredLocation}`);
  return Number.parseInt(value, 10);
}

/** 🧱️ Names why a file's planned edits cannot all apply to its preimage in one pass (a preimage that differs from the bytes at its
 * offset, or two edits sharing bytes), or `null` when {@link applyEditsToContent} is exact; the planner reports it as a violation. */
export function referenceEditConflict(content: string, edits: readonly ReferenceEdit[]): string | null {
  const spans = edits.map((edit) => ({ edit, start: referenceEditOffset(edit) })).sort((a, b) => a.start - b.start || utf8Compare(a.edit.structuredLocation, b.edit.structuredLocation));
  for (let index = 0; index < spans.length; index++) {
    const { edit, start } = spans[index]!, previous = spans[index - 1];
    if (content.slice(start, start + edit.oldValue.length) !== edit.oldValue) return `${edit.structuredLocation}: planned preimage ${JSON.stringify(edit.oldValue)} differs from the file bytes at its offset`;
    if (previous && start < previous.start + previous.edit.oldValue.length) return `${edit.structuredLocation} (${JSON.stringify(edit.oldValue)} -> ${JSON.stringify(edit.newValue)}) overlaps ${previous.edit.structuredLocation} (${JSON.stringify(previous.edit.oldValue)} -> ${JSON.stringify(previous.edit.newValue)}); one byte span cannot take two rewrites`;
  }
  return null;
}

export function applyEditsToContent(content: string, edits: readonly ReferenceEdit[]): string {
  let result = content;
  const sorted = edits.map((edit) => ({ edit, start: referenceEditOffset(edit) })).sort((a, b) => b.start - a.start || b.edit.structuredLocation.localeCompare(a.edit.structuredLocation));
  const pieces: string[] = [];
  let floor = content.length, index = 0;
  for (; index < sorted.length; index++) {
    const { edit, start } = sorted[index]!, end = start + edit.oldValue.length;
    if (end > floor) break;
    if (content.slice(start, end) !== edit.oldValue) throw new Error(`Reference edit preimage mismatch at ${edit.path}:${edit.structuredLocation}`);
    pieces.push(content.slice(end, floor), edit.newValue);
    floor = start;
  }
  pieces.push(content.slice(0, floor));
  result = pieces.reverse().join("");
  for (; index < sorted.length; index++) {
    const { edit, start } = sorted[index]!, end = start + edit.oldValue.length;
    if (result.slice(start, end) !== edit.oldValue) throw new Error(`Reference edit preimage mismatch at ${edit.path}:${edit.structuredLocation}`);
    result = `${result.slice(0, start)}${edit.newValue}${result.slice(end)}`;
  }
  return result;
}
