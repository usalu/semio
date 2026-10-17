import { rustAttributes, rustFindTopLevel, rustPathAttributes, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokens, rustVisibility, type RustToken } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

/** 🧪️ Whether one attribute text is a `cfg(...)` that only holds under `cfg(test)`: `test` itself, an `all(...)` with such an argument, or an `any(...)` whose every argument is such. */
export function cfgImpliesTest(attribute: string): boolean {
  const match = /^cfg\s*\((.*)\)$/su.exec(attribute.trim());
  return match !== null && predicateImpliesTest(match[1]!.trim());
}

function predicateImpliesTest(predicate: string): boolean {
  if (/^test$/u.test(predicate)) return true;
  const call = /^(all|any)\s*\((.*)\)$/su.exec(predicate);
  if (!call) return false;
  const args: string[] = [];
  let depth = 0;
  let start = 0;
  const body = call[2]!;
  for (let index = 0; index < body.length; index += 1) {
    const char = body[index]!;
    if (char === "(") depth += 1;
    else if (char === ")") depth -= 1;
    else if (char === "," && depth === 0) { args.push(body.slice(start, index).trim()); start = index + 1; }
  }
  args.push(body.slice(start).trim());
  const real = args.filter(Boolean);
  return call[1] === "all" ? real.some(predicateImpliesTest) : real.length > 0 && real.every(predicateImpliesTest);
}

/** 🌳️ One `mod` item of a Rust source file with exact character offsets. */
export type RustModuleNode = {
  readonly name: string;
  readonly visibility: string;
  readonly start: number;
  readonly declStart: number;
  readonly end: number;
  readonly attributeTexts: readonly string[];
  readonly pathAttribute: string | null;
  readonly conditionalTest: boolean;
  readonly inline: boolean;
  readonly bodyStart: number;
  readonly bodyEnd: number;
  readonly children: readonly RustModuleNode[];
  readonly items: readonly RustSourceItem[];
};

/** 🧱️ One non-module item of a Rust module body. */
export type RustSourceItem = {
  readonly kind: "use" | "extern-crate" | "fn" | "const" | "static" | "struct" | "enum" | "type" | "trait" | "impl" | "macro" | "other";
  readonly name: string | null;
  readonly visibility: string;
  readonly start: number;
  readonly declStart: number;
  readonly end: number;
  readonly attributeTexts: readonly string[];
};

/** 🧾️ Parsed outer structure of one Rust source file. */
export type RustSourceTree = {
  readonly source: string;
  readonly innerAttributes: readonly { readonly text: string; readonly start: number; readonly end: number }[];
  readonly modules: readonly RustModuleNode[];
  readonly items: readonly RustSourceItem[];
};

const ITEM_KEYWORDS: ReadonlySet<string> = new Set(["fn", "const", "static", "struct", "enum", "type", "trait", "impl", "macro_rules", "union"]);
const QUALIFIERS: ReadonlySet<string> = new Set(["async", "unsafe", "extern", "default", "const"]);

/** 🔤️ Slices a token range back to its source text. */
export function rustSourceRange(source: string, tokens: readonly RustToken[], from: number, to: number): string {
  return source.slice(tokens[from]!.start, tokens[to - 1]!.end);
}

/** 🌳️ Parses the module and item skeleton of a Rust file; bodies of fn, impl and type items stay opaque. */
export function parseRustSourceTree(source: string): RustSourceTree {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const innerAttributes: { text: string; start: number; end: number }[] = [];
  let first = 0;
  while (tokens[first]?.text === "#" && tokens[first + 1]?.text === "!" && tokens[first + 2]?.text === "[") {
    const close = pairs.get(first + 2);
    if (close === undefined) break;
    innerAttributes.push({ text: source.slice(tokens[first]!.start, tokens[close]!.end), start: tokens[first]!.start, end: tokens[close]!.end });
    first = close + 1;
  }
  const parseBody = (from: number, to: number): { modules: RustModuleNode[]; items: RustSourceItem[] } => {
    const modules: RustModuleNode[] = [];
    const items: RustSourceItem[] = [];
    for (let index = from; index < to;) {
      const attributes = rustAttributes(tokens, pairs, index, to);
      const visibility = rustVisibility(tokens, pairs, attributes.next);
      let cursor = visibility.next;
      const attributeTexts = attributes.ranges.map(([a, b]) => rustSourceRange(source, tokens, a, b));
      const itemStart = tokens[index]!.start;
      const declStart = tokens[attributes.next]?.start ?? itemStart;
      if (tokens[cursor]?.text === "mod" && tokens[cursor + 1]?.kind === "identifier") {
        const name = tokens[cursor + 1]!.text;
        const afterName = cursor + 2;
        const paths = rustPathAttributes(tokens, pairs, attributes);
        const pathAttribute = paths.length ? paths[paths.length - 1]!.path : null;
        const conditionalTest = attributeTexts.some(cfgImpliesTest);
        if (tokens[afterName]?.text === ";") {
          modules.push({ name, visibility: visibility.value, start: itemStart, declStart, end: tokens[afterName]!.end, attributeTexts, pathAttribute, conditionalTest, inline: false, bodyStart: tokens[afterName]!.end, bodyEnd: tokens[afterName]!.end, children: [], items: [] });
          index = afterName + 1;
          continue;
        }
        const close = pairs.get(afterName);
        if (tokens[afterName]?.text === "{" && close !== undefined) {
          const body = parseBody(afterName + 1, close);
          modules.push({ name, visibility: visibility.value, start: itemStart, declStart, end: tokens[close]!.end, attributeTexts, pathAttribute, conditionalTest, inline: true, bodyStart: tokens[afterName]!.end, bodyEnd: tokens[close]!.start, children: body.modules, items: body.items });
          index = close + 1;
          continue;
        }
      }
      let end: number;
      const boundary = rustFindTopLevel(tokens, pairs, cursor, to, new Set([";", "{"]));
      if (boundary < 0) end = to;
      else if (tokens[boundary]!.text === ";") end = boundary + 1;
      else {
        end = (pairs.get(boundary) ?? to - 1) + 1;
        if (tokens[end]?.text === ";") end += 1;
      }
      while (QUALIFIERS.has(tokens[cursor]?.text ?? "") && !(tokens[cursor]?.text === "const" && tokens[cursor + 1]?.kind === "identifier" && tokens[cursor + 2]?.text === ":")) cursor += 1;
      const keyword = tokens[cursor]?.text ?? "";
      let kind: RustSourceItem["kind"] = "other";
      let name: string | null = null;
      if (keyword === "use") kind = "use";
      else if (keyword === "extern" || (tokens[cursor]?.text === "crate" && tokens[cursor - 1]?.text === "extern")) kind = "extern-crate";
      else if (keyword === "macro_rules") { kind = "macro"; name = tokens[cursor + 2]?.text ?? null; }
      else if (ITEM_KEYWORDS.has(keyword) || keyword === "union") {
        kind = keyword === "union" ? "struct" : (keyword as RustSourceItem["kind"]);
        name = tokens[cursor + 1]?.kind === "identifier" ? tokens[cursor + 1]!.text : null;
      }
      if (kind === "other" && tokens[index]?.text === "extern" && tokens[index + 1]?.text === "crate") { kind = "extern-crate"; name = tokens[index + 2]?.text ?? null; }
      if (visibility.next < to && tokens[visibility.next]?.text === "extern" && tokens[visibility.next + 1]?.text === "crate") { kind = "extern-crate"; name = tokens[visibility.next + 2]?.text ?? null; }
      items.push({ kind, name, visibility: visibility.value, start: itemStart, declStart, end: tokens[end - 1]!.end, attributeTexts });
      index = Math.max(end, index + 1);
    }
    return { modules, items };
  };
  const top = parseBody(first, tokens.length);
  return { source, innerAttributes, modules: top.modules, items: top.items };
}

/** 🧭️ Resolves a module by its path of names, or null. */
export function findRustModule(modules: readonly RustModuleNode[], path: readonly string[]): RustModuleNode | null {
  let level: readonly RustModuleNode[] = modules;
  let found: RustModuleNode | null = null;
  for (const segment of path) {
    found = level.find((node) => node.name === segment) ?? null;
    if (!found) return null;
    level = found.children;
  }
  return found;
}

/** 🏷️ Extends an item start back over its contiguous leading comment lines. */
export function rustItemStartWithComments(source: string, start: number): number {
  let cursor = start;
  for (;;) {
    const lineStart = source.lastIndexOf("\n", cursor - 2) + 1;
    const previous = source.slice(lineStart, cursor - 1);
    if (cursor <= 0 || !/^\s*\/\/(?!!)/u.test(previous) || previous.trim() === "") return cursor;
    cursor = lineStart;
  }
}

/** 📜️ Extends the start of an item over the contiguous outer doc comment lines (`///`) directly above it, so removing the item never leaves a dangling doc comment. */
export function rustOuterDocStart(source: string, start: number): number {
  let cursor = start;
  const lineStart = source.lastIndexOf("\n", cursor - 1) + 1;
  if (!/^[ \t]*$/u.test(source.slice(lineStart, cursor))) return start;
  cursor = lineStart;
  while (cursor > 0) {
    const previousStart = source.lastIndexOf("\n", cursor - 2) + 1;
    if (!/^[ \t]*\/\/\/(?!\/)/u.test(source.slice(previousStart, cursor - 1))) break;
    cursor = previousStart;
  }
  return cursor;
}

/** 🩹 Turns outer doc comments (`///`) that no longer document an item into plain comments: a run followed, past blank lines and plain comments, only by the end of the text or a closing brace would not parse. */
export function demoteDanglingOuterDocs(source: string): string {
  const lines = source.split("\n");
  const isDoc = (line: string): boolean => /^[ \t]*\/\/\/(?!\/)/u.test(line);
  const isSkippable = (line: string): boolean => line.trim() === "" || (/^[ \t]*\/\//u.test(line) && !isDoc(line));
  let changed = false;
  for (let index = 0; index < lines.length; index += 1) {
    if (!isDoc(lines[index]!)) continue;
    let end = index;
    while (end + 1 < lines.length && isDoc(lines[end + 1]!)) end += 1;
    let next = end + 1;
    while (next < lines.length && isSkippable(lines[next]!)) next += 1;
    if (next >= lines.length || /^[ \t]*[})\]]/u.test(lines[next]!)) {
      for (let i = index; i <= end; i += 1) lines[i] = lines[i]!.replace(/\/\/\//u, "//");
      changed = true;
    }
    index = end;
  }
  return changed ? lines.join("\n") : source;
}

/** 🔍️ Reads the string value of a `#[path = "…"]` attribute text. */
export function rustPathAttributeValue(attributeText: string): string | null {
  const match = /^path\s*=\s*"((?:[^"\\]|\\.)*)"$/u.exec(attributeText);
  return match ? match[1]! : null;
}

export { rustStringValue };

/** 🍃 Leaf names a `use` statement binds. */
export function useLeafNames(statement: string): string[] {
  const tokens = rustTokens(statement);
  const pairs = rustTokenPairs(tokens);
  let start = 0;
  if (tokens[start]?.text === "pub") {
    start += 1;
    if (tokens[start]?.text === "(") start = (pairs.get(start) ?? start) + 1;
  }
  if (tokens[start]?.text !== "use") return [];
  const names: string[] = [];
  const visit = (from: number, to: number): void => {
    let brace = -1;
    for (let k = from; k < to; k += 1) if (tokens[k]!.text === "{") { brace = k; break; }
    if (brace >= 0) {
      const close = pairs.get(brace)!;
      for (const [a, b] of rustTokenSegments(tokens, pairs, brace + 1, close, ",")) {
        if (tokens[a]?.text === "self") names.push(tokens[a + 2]?.text === "as" ? tokens[a + 3]!.text : (tokens[brace - 2]?.text ?? ""));
        else visit(a, b);
      }
      return;
    }
    const as = tokens.findIndex((token, index) => index >= from && index < to && token.text === "as");
    if (as >= 0) names.push(tokens[as + 1]!.text);
    else if (tokens[to - 1]?.kind === "identifier" && tokens[to - 1]!.text !== "self") names.push(tokens[to - 1]!.text);
  };
  const end = tokens.findLastIndex((token) => token.text === ";");
  visit(start + 1, end < 0 ? tokens.length : end);
  return names.filter(Boolean);
}

/** 🔤️ All identifier texts of a source. */
export function identifierSet(source: string): Set<string> {
  const set = new Set<string>();
  for (const token of rustTokens(source)) if (token.kind === "identifier") set.add(token.text);
  return set;
}

/** 🔗️ Identifier texts that can name a module-level item: field accesses (`.name`), field, parameter or binding names (`name:`) and non-leading path segments (`a::name`, unless `a` is `crate`, `self` or `super`) never reference an item. */
export function referencedItemNames(source: string): Set<string> {
  const tokens = rustTokens(source);
  const set = new Set<string>();
  tokens.forEach((token, index) => {
    if (token.kind !== "identifier") return;
    if (tokens[index - 1]?.text === "." || tokens[index + 1]?.text === ":") return;
    if (tokens[index - 1]?.text === "::" && !["crate", "self", "super"].includes(tokens[index - 2]?.text ?? "")) return;
    set.add(token.text);
  });
  return set;
}


/** ✂️ Prunes a `use` statement to the given leaf names; nested groups are flattened to full paths below their longest shared leading path (so a leading `super::super` run stays one prefix that path absolutising can resolve), and glob leaves are dropped. */
export function prunedUse(statement: string, leaves: readonly string[]): string {
  const text = statement.replace(/\/\*[\s\S]*?\*\//gu, " ").replace(/\/\/[^\n]*/gu, " ").trim();
  const head = /^((?:pub(?:\([^)]*\))?\s+)?use\s+)/u.exec(text);
  if (!head || !text.includes("{")) return text;
  const body = text.slice(head[0].length).replace(/;\s*$/u, "");
  const entries: { segments: string[]; alias: string | null }[] = [];
  let cursor = 0;
  const skip = (): void => { while (cursor < body.length && /\s/u.test(body[cursor]!)) cursor += 1; };
  const parseTree = (prefix: readonly string[]): void => {
    const segments = [...prefix];
    for (;;) {
      skip();
      if (body[cursor] === "{") {
        cursor += 1;
        for (;;) {
          skip();
          if (cursor >= body.length) return;
          if (body[cursor] === "}") { cursor += 1; return; }
          const before = cursor;
          parseTree(segments);
          skip();
          if (body[cursor] === ",") cursor += 1;
          else if (cursor === before) cursor += 1;
        }
      }
      if (body[cursor] === "*") { cursor += 1; return; }
      const word = /^(?:::)?[A-Za-z_][A-Za-z0-9_]*|^::/u.exec(body.slice(cursor));
      if (!word) return;
      cursor += word[0].length;
      segments.push(word[0].replace(/^::/u, "") || "");
      skip();
      if (body.startsWith("::", cursor)) { cursor += 2; continue; }
      let alias: string | null = null;
      const asMatch = /^as\s+([A-Za-z_][A-Za-z0-9_]*)/u.exec(body.slice(cursor));
      if (asMatch) { alias = asMatch[1]!; cursor += asMatch[0].length; }
      entries.push({ segments: segments.filter(Boolean), alias });
      return;
    }
  };
  parseTree([]);
  const binding = (entry: { segments: string[]; alias: string | null }): string => entry.alias ?? (entry.segments.at(-1) === "self" ? entry.segments.at(-2)! : entry.segments.at(-1)!);
  const kept = entries.filter((entry) => leaves.includes(binding(entry)));
  if (!kept.length) return "";
  const render = (entry: { segments: string[]; alias: string | null }, from: number): string => {
    const isSelf = entry.segments.at(-1) === "self";
    const path = (isSelf ? entry.segments.slice(0, -1) : entry.segments).slice(from).join("::");
    return `${path || "self"}${entry.alias ? ` as ${entry.alias}` : ""}`;
  };
  if (kept.length === 1) return `${head[1]}${render(kept[0]!, 0)};`;
  let shared = 0;
  while (kept.every((entry) => entry.segments.length > shared + 1 && entry.segments[shared] === kept[0]!.segments[shared])) shared += 1;
  if (shared > 0) return `${head[1]}${kept[0]!.segments.slice(0, shared).join("::")}::{${kept.map((entry) => render(entry, shared)).join(", ")}};`;
  return `${head[1]}{${kept.map((entry) => render(entry, 0)).join(", ")}};`;
}
