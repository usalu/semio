import { basename, dirname } from "node:path";
import { requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { type ReferenceToken, lineLocation, regexTokens } from "../🔤️reference-tokenizers/🟦️.ts";

export type TicketImportantProseReference = Readonly<{ start: number; end: number; structuredLocation: string; value: "📌️important.md" }>;

/** 📌️ Admits only the exact ticket-leaf prose form used by governed handoff generators. */
export function ticketImportantProseReferenceAuthority(content: string): readonly TicketImportantProseReference[] {
  const rows: TicketImportantProseReference[] = [];
  for (const match of content.matchAll(/\bsee\s+(📌️important\.md)(?=$|[\s"'`,.;:!?)}\]])/gu)) {
    if (match.index === undefined) continue;
    const start = match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]"));
    rows.push({ start, end: start + requireTaxonomyValue(match[1],"match[1]").length, structuredLocation: lineLocation(content, start, "typescript-ticket-leaf-prose"), value: "📌️important.md" });
  }
  return rows;
}

/** 📖️ Resolves unescaped single-backtick paths only inside the leading JSDoc block. */
export function typescriptLeadingDocumentationReferenceAuthority(content: string): readonly Readonly<{ start: number; end: number; value: string; structuredLocation: string }>[] {
  const start = content.match(/^\s*\/\*\*/u)?.[0].length;
  if (start === undefined) return [];
  const end = content.indexOf("*/", start);
  if (end < 0) return [];
  const rows: { start: number; end: number; value: string; structuredLocation: string }[] = [];
  for (const match of content.slice(start, end).matchAll(/(?<![\\`])`([^`\\\r\n]+)`(?!`)/gu)) {
    if (!/[/.]/u.test(requireTaxonomyValue(match[1],"match[1]")) || /\s/u.test(requireTaxonomyValue(match[1],"match[1]"))) continue;
    const offset = start + match.index! + 1;
    rows.push({ start: offset, end: offset + requireTaxonomyValue(match[1],"match[1]").length, value: requireTaxonomyValue(match[1],"required reference value"), structuredLocation: lineLocation(content, offset, "typescript-leading-jsdoc-path") });
  }
  return rows;
}

/** 💬️ Resolves unescaped single-backtick paths inside EVERY comment in a TypeScript file — every `/* … *\/`/`/** … *\/` block (not only the leading JSDoc {@link typescriptLeadingDocumentationReferenceAuthority} already covers) and every whole-line `//` comment — mirroring `rustTokens`'s unconditional `rust-comment-path` scan so both languages give a moved file's design-rationale prose the same live-reference treatment. A trailing end-of-line `//` after real code is deliberately NOT scanned (mirrors the Rust scanner's own whole-line restriction), which keeps this from ever touching a `//` inside a string literal on a code line. */
function typescriptCommentPathReferenceAuthority(content: string): readonly Readonly<{ start: number; end: number; value: string; structuredLocation: string }>[] {
  const rows: { start: number; end: number; value: string; structuredLocation: string }[] = [];
  for (const block of content.matchAll(/\/\*[\s\S]*?\*\//gu)) {
    if (block.index === undefined) continue;
    for (const match of block[0].matchAll(/(?<![\\`])`([^`\\\r\n]+)`(?!`)/gu)) {
      if (!/[/.]/u.test(requireTaxonomyValue(match[1],"match[1]")) || /\s/u.test(requireTaxonomyValue(match[1],"match[1]"))) continue;
      const offset = block.index + match.index! + 1;
      rows.push({ start: offset, end: offset + requireTaxonomyValue(match[1],"match[1]").length, value: requireTaxonomyValue(match[1],"required reference value"), structuredLocation: lineLocation(content, offset, "typescript-comment-block-path") });
    }
  }
  for (const match of content.matchAll(/(?:^|\n)[ \t]*\/\/([^\r\n]*)/gu)) {
    if (match.index === undefined) continue;
    const fragment = requireTaxonomyValue(match[1],"match[1]"), fragmentStart = match.index + match[0].indexOf(fragment);
    for (const quoted of fragment.matchAll(/`([^`]+)`/gu)) {
      if (quoted.index === undefined || !/[/.]/u.test(requireTaxonomyValue(quoted[1],"quoted[1]")) || /\s/u.test(requireTaxonomyValue(quoted[1],"quoted[1]"))) continue;
      const start = fragmentStart + quoted.index + quoted[0].indexOf(requireTaxonomyValue(quoted[1],"quoted[1]"));
      rows.push({ start, end: start + requireTaxonomyValue(quoted[1],"quoted[1]").length, value: requireTaxonomyValue(quoted[1],"required reference value"), structuredLocation: lineLocation(content, start, "typescript-comment-line-path") });
    }
  }
  return rows;
}

interface TypeScriptCollectionToken {
  readonly text: string;
  readonly start: number;
  readonly end: number;
  readonly kind: "identifier" | "string" | "template" | "number" | "punctuation" | "regex";
  readonly group: number;
  readonly scope: number;
}

interface TypeScriptCollectionSyntax {
  readonly tokens: readonly TypeScriptCollectionToken[];
  readonly pairs: ReadonlyMap<number, number>;
}

const TYPESCRIPT_COLLECTION_OPERATOR = /(?:\.\.\.|===|!==|>>>=|>>>|>>=|<<=|\*\*=|&&=|\|\|=|\?\?=|=>|==|!=|<=|>=|\+\+|--|\+=|-=|\*=|\/=|%=|&=|\|=|\^=|&&|\|\||\?\?|\?\.|\*\*|<<|>>)/uy;

/** 🔬️ Reads bounded source tokens and balanced delimiters without evaluating code or decoding editable spans. */
function typescriptCollectionSyntax(content: string): TypeScriptCollectionSyntax | null {
  const tokens: TypeScriptCollectionToken[] = [], pairs = new Map<number, number>(), stack: number[] = [];
  let cursor = 0;
  while (cursor < content.length) {
    if (/\s/u.test(requireTaxonomyValue(content[cursor],"content[cursor]"))) { cursor++; continue; }
    if (content.startsWith("//", cursor)) { const end = content.indexOf("\n", cursor + 2); cursor = end < 0 ? content.length : end + 1; continue; }
    if (content.startsWith("/*", cursor)) { const end = content.indexOf("*/", cursor + 2); if (end < 0) return null; cursor = end + 2; continue; }
    const start = cursor, first = requireTaxonomyValue(content[cursor],"content[cursor]");
    let kind: TypeScriptCollectionToken["kind"] = "punctuation";
    if (first === '"' || first === "'" || first === "\u0060") {
      kind = first === "\u0060" ? "template" : "string";
      cursor++;
      while (cursor < content.length && content[cursor] !== first) {
        if (content[cursor] === "\\") { cursor += 2; continue; }
        if (kind === "string" && /[\r\n]/u.test(requireTaxonomyValue(content[cursor],"content[cursor]"))) return null;
        cursor++;
      }
      if (cursor >= content.length) return null;
      cursor++;
    } else if (/[A-Za-z_$]/u.test(first)) {
      kind = "identifier";
      while (cursor < content.length && /[A-Za-z0-9_$]/u.test(requireTaxonomyValue(content[cursor],"content[cursor]"))) cursor++;
    } else if (/[0-9]/u.test(first)) {
      kind = "number";
      while (cursor < content.length && /[A-Za-z0-9_.]/u.test(requireTaxonomyValue(content[cursor],"content[cursor]"))) cursor++;
    } else if (first === "/" && (!tokens.length || /^(?:[([{=:;,!?]|=>|&&|\|\||return|throw|case)$/u.test(tokens.at(-1)!.text))) {
      kind = "regex";
      let characterClass = false;
      cursor++;
      while (cursor < content.length) {
        if (content[cursor] === "\\") { cursor += 2; continue; }
        if (/[\r\n]/u.test(requireTaxonomyValue(content[cursor],"content[cursor]"))) return null;
        if (content[cursor] === "[") characterClass = true;
        if (content[cursor] === "]") characterClass = false;
        if (content[cursor] === "/" && !characterClass) break;
        cursor++;
      }
      if (cursor >= content.length) return null;
      cursor++;
      while (/[A-Za-z]/u.test(content[cursor] ?? "")) cursor++;
    } else {
      TYPESCRIPT_COLLECTION_OPERATOR.lastIndex = cursor;
      const operator = TYPESCRIPT_COLLECTION_OPERATOR.exec(content)?.[0];
      if (!operator && !/[{}()[\].,;:?~!+\-*/%<>=&|^]/u.test(first)) return null;
      cursor += operator?.length ?? 1;
    }
    const text = content.slice(start, cursor), group = stack.at(-1) ?? -1, scope = [...stack].reverse().find((index) => requireTaxonomyValue(tokens[index],"tokens[index]").text === "{") ?? -1;
    const index = tokens.length;
    tokens.push({ text, start, end: cursor, kind, group, scope });
    if (kind !== "punctuation") continue;
    if (["(", "[", "{"].includes(text)) stack.push(index);
    else if ([")", "]", "}"].includes(text)) {
      const open = stack.pop();
      if (open === undefined || "([{".indexOf(requireTaxonomyValue(tokens[open],"tokens[open]").text) !== ")]}".indexOf(text)) return null;
      pairs.set(open, index);
      pairs.set(index, open);
    }
  }
  return stack.length ? null : { tokens, pairs };
}

/** 🧩️ Splits only top-level token segments, keeping nested calls, arrays, and blocks opaque. */
function typescriptCollectionSegments(syntax: TypeScriptCollectionSyntax, start: number, end: number): readonly (readonly [number, number])[] {
  const rows: [number, number][] = [];
  let left = start;
  for (let cursor = start; cursor < end; cursor++) {
    const close = syntax.pairs.get(cursor);
    if (close !== undefined && close > cursor) { cursor = close; continue; }
    if (requireTaxonomyValue(syntax.tokens[cursor],"syntax.tokens[cursor]").text === ",") { rows.push([left, cursor]); left = cursor + 1; }
  }
  if (left < end) rows.push([left, end]);
  return rows;
}

/** 🪆️ Exposes simple template expressions for binding checks and rejects unparsed nested expression syntax. */
function typescriptCollectionEmbeddedExpressions(syntax: TypeScriptCollectionSyntax): readonly TypeScriptCollectionSyntax[] | null {
  const expressions: TypeScriptCollectionSyntax[] = [];
  for (const token of syntax.tokens) if (token.kind === "template") {
    for (let cursor = 1; cursor < token.text.length - 1; cursor++) {
      if (token.text[cursor] === "\\") { cursor++; continue; }
      if (!token.text.startsWith("$" + "{", cursor)) continue;
      const end = token.text.indexOf("}", cursor + 2), source = token.text.slice(cursor + 2, end);
      if (end < 0 || /[{}\u0060]/u.test(source)) return null;
      const expression = typescriptCollectionSyntax(source);
      if (!expression) return null;
      expressions.push(expression);
      cursor = end;
    }
  }
  return expressions;
}

/** 🧷️ Detects direct, property, computed-property, and destructuring writes to a proven binding. */
function typescriptCollectionChangedBinding(syntax: TypeScriptCollectionSyntax, name: string, declaration = -1): boolean {
  const { tokens, pairs } = syntax;
  const assignment = (text?: string): boolean => /^(?:=|\+=|-=|\*=|\/=|%=|&=|\|=|\^=|&&=|\|\|=|\?\?=|\*\*=|<<=|>>=|>>>=|\+\+|--)$/u.test(text ?? "");
  return tokens.some((token, index) => {
    if (token.kind !== "identifier" || token.text !== name || index === declaration) return false;
    if (assignment(tokens[index + 1]?.text) || ["++", "--"].includes(tokens[index - 1]?.text ?? "")) return true;
    let cursor = index + 1;
    while (tokens[cursor]?.text === "." && tokens[cursor + 1]?.kind === "identifier" || tokens[cursor]?.text === "[" && pairs.has(cursor)) cursor = requireTaxonomyValue(tokens[cursor],"tokens[cursor]").text === "[" ? pairs.get(cursor)! + 1 : cursor + 2;
    if (cursor > index + 1 && assignment(tokens[cursor]?.text)) return true;
    for (let group = token.group; group >= 0; group = requireTaxonomyValue(tokens[group],"tokens[group]").group) if (assignment(tokens[(pairs.get(group) ?? -2) + 1]?.text)) return true;
    return false;
  });
}

/** 🚧️ Prevents a rejected for-of proof from falling through the independent map-only authority. */
function typescriptCollectionHasForOf(content: string): boolean {
  const firstFor = content.search(/\bfor\b/u);
  if (firstFor < 0 || !/\bof\b/u.test(content.slice(firstFor + 3))) return false;
  const syntax = typescriptCollectionSyntax(content);
  if (!syntax) return true;
  const { tokens, pairs } = syntax;
  return tokens.some((token, index) => {
    if (token.text !== "for") return false;
    const open = index + (tokens[index + 1]?.text === "await" ? 2 : 1), close = pairs.get(open);
    return tokens[open]?.text === "(" && close !== undefined && tokens.slice(open + 1, close).some((part) => part.kind === "identifier" && part.text === "of" && part.group === open);
  });
}

/** 🛡️ Proves immutable for-of reader bindings and emits only exact unescaped physical leaf spans. */
function typescriptPathCollectionReferenceAuthority(content: string): ReferenceToken[] {
  if (!content.includes("for") || !content.includes("readFileSync") || !content.includes("node:path") || !content.includes("node:fs")) return [];
  const syntax = typescriptCollectionSyntax(content);
  if (!syntax) return [];
  const embedded = typescriptCollectionEmbeddedExpressions(syntax);
  if (!embedded) return [];
  const { tokens, pairs } = syntax;
  type Declaration = { name: string; index: number; kind: string; start: number; end: number; scope: number; exported: boolean };
  const declarations = new Map<string, Declaration[]>(), bindings = new Map<string, number[]>(), imports = new Map<string, { index: number; module: string; name: string }>(), functions: [number, number][] = [];
  const bind = (index: number): void => { if (tokens[index]?.kind === "identifier") bindings.set(tokens[index].text, [...(bindings.get(tokens[index].text) ?? []), index]); };
  const endOf = (start: number, stops: readonly string[]): number => {
    for (let cursor = start; cursor < tokens.length; cursor++) {
      if (stops.includes(requireTaxonomyValue(tokens[cursor],"tokens[cursor]").text)) return cursor;
      const close = pairs.get(cursor);
      if (close !== undefined && close > cursor) cursor = close;
    }
    return tokens.length;
  };
  for (let index = 0; index < tokens.length; index++) {
    const token = requireTaxonomyValue(tokens[index],"tokens[index]");

    if (["const", "let", "var"].includes(token.text)) {
      let cursor = index + 1;
      while (cursor < tokens.length) {
        const name = requireTaxonomyValue(tokens[cursor],"tokens[cursor]");
        if (name.kind !== "identifier") { const close = pairs.get(cursor); if (close !== undefined) for (let part = cursor + 1; part < close; part++) bind(part); break; }
        bind(cursor);
        const equal = endOf(cursor + 1, ["=", ",", ";", "of", "in", ")"]);
        const end = tokens[equal]?.text === "=" ? endOf(equal + 1, [",", ";", ")"]) : equal;
        const row = { name: name.text, index: cursor, kind: token.text, start: tokens[equal]?.text === "=" ? equal + 1 : end, end, scope: token.scope, exported: tokens[index - 1]?.text === "export" };
        declarations.set(name.text, [...(declarations.get(name.text) ?? []), row]);
        if (tokens[end]?.text !== ",") break;
        cursor = end + 1;
      }
    }
    let importOpen = token.text === "import" ? index + 1 : -1;
    if (tokens[importOpen]?.kind === "identifier") { bind(importOpen); importOpen = tokens[importOpen + 1]?.text === "," ? importOpen + 2 : -1; }
    if (tokens[importOpen]?.text === "*" && tokens[importOpen + 1]?.text === "as") bind(importOpen + 2);
    if (tokens[importOpen]?.text === "{") {
      const close = pairs.get(importOpen), module = close === undefined ? undefined : tokens[close + 2];
      if (close !== undefined && tokens[close + 1]?.text === "from" && module?.kind === "string" && !module.text.includes("\\")) {
        for (const [start, end] of typescriptCollectionSegments(syntax, importOpen + 1, close)) {
          const local = end - start === 3 && requireTaxonomyValue(tokens[start + 1],"tokens[start + 1]").text === "as" ? start + 2 : end - start === 1 ? start : -1;
          if (local >= 0) { bind(local); imports.set(requireTaxonomyValue(tokens[local],"tokens[local]").text, { index: local, module: module.text.slice(1, -1), name: requireTaxonomyValue(tokens[start],"tokens[start]").text }); }
        }
      }
    }
    if (["function", "class"].includes(token.text)) bind(index + 1);
    if (token.text === "(") {
      const close = pairs.get(index), previous = tokens[index - 1]?.text, next = close === undefined ? undefined : tokens[close + 1]?.text;
      if (close !== undefined && (next === "=>" || (next === "{" || next === ":") && (previous === undefined || !["for", "if", "while", "switch", "with"].includes(previous)))) {
        for (const [start, end] of typescriptCollectionSegments(syntax, index + 1, close)) {
          const limit = Math.min(endOf(start, [":", "="]), end);
          for (let cursor = start; cursor < limit; cursor++) bind(cursor);
        }
        if (next === "{") functions.push([close + 1, pairs.get(close + 1)!]);
      }
    }
    if (token.text === "=>") {
      if (tokens[index - 1]?.kind === "identifier") bind(index - 1);
      functions.push([index + 1, tokens[index + 1]?.text === "{" ? pairs.get(index + 1)! : endOf(index + 1, [";", ",", ")", "]"])]);
    }
  }
  const changed = (name: string, declaration = -1): boolean => typescriptCollectionChangedBinding(syntax, name, declaration) || embedded.some((expression) => typescriptCollectionChangedBinding(expression, name));
  const immutable = (row?: Declaration): row is Declaration => Boolean(row && row.kind === "const" && bindings.get(row.name)?.length === 1 && !changed(row.name, row.index));
  const unique = (name: string): Declaration | undefined => declarations.get(name)?.length === 1 ? declarations.get(name)![0] : undefined;
  const ancestor = (outer: number, inner: number): boolean => { for (let scope = inner; scope >= 0; scope = requireTaxonomyValue(tokens[scope],"tokens[scope]").scope) if (scope === outer) return true; return outer === -1; };
  const staticValue = (start: number, end: number, scope: number, seen = new Set<string>()): string | null => {
    if (end - start !== 1) return null;
    const token = requireTaxonomyValue(tokens[start],"tokens[start]");
    if (token.kind === "string") return token.text.includes("\\") ? null : token.text.slice(1, -1);
    if (token.kind === "identifier") {
      const row = unique(token.text);
      return immutable(row) && row.end < start && ancestor(row.scope, scope) && !seen.has(row.name) ? staticValue(row.start, row.end, row.scope, new Set([...seen, row.name])) : null;
    }
    if (token.kind !== "template" || token.text.includes("\\")) return null;
    let valid = true;
    const value = token.text.slice(1, -1).replace(/\$\{([A-Za-z_$][\w$]*)\}/gu, (_match, name: string) => {
      const row = unique(name);
      const part = immutable(row) && row.end < start && ancestor(row.scope, scope) && !seen.has(name) ? staticValue(row.start, row.end, row.scope, new Set([...seen, name])) : null;
      if (part === null) valid = false;
      return part ?? "";
    });
    return valid && !value.includes("$" + "{") ? value : null;
  };
  const imported = (index: number, module: string, name: string): boolean => {
    const token = requireTaxonomyValue(tokens[index],"tokens[index]"), owner = imports.get(token?.text);
    return Boolean(token?.kind === "identifier" && ![".", "?."].includes(tokens[index - 1]?.text ?? "") && owner?.module === module && owner.name === name && bindings.get(token.text)?.length === 1 && !changed(token.text, owner.index));
  };
  const rootValue = (index: number): boolean => {
    const row = unique(requireTaxonomyValue(tokens[index],"tokens[index]")?.text);
    return immutable(row) && row.end < index && ancestor(row.scope, requireTaxonomyValue(tokens[index],"tokens[index]").scope) && tokens.slice(row.start, row.end).map((token) => token.text).join(" ") === "process . cwd ( )" && !bindings.has("process") && !changed("process");
  };
  const loops: { index: number; item: number; collection: number; body: number; end: number }[] = [];
  for (let index = 0; index < tokens.length; index++) if (requireTaxonomyValue(tokens[index],"tokens[index]").text === "for" && tokens[index + 1]?.text === "(" && tokens[index + 2]?.text === "const" && tokens[index + 3]?.kind === "identifier" && tokens[index + 4]?.text === "of" && tokens[index + 5]?.kind === "identifier" && pairs.get(index + 1) === index + 6 && tokens[index + 7]?.text === "{") loops.push({ index, item: index + 3, collection: index + 5, body: index + 7, end: pairs.get(index + 7)! });
  const relativeLeaf = (value: string): boolean => Boolean(value) && !value.startsWith("/") && !/^[A-Za-z]:/u.test(value) && !/[\\\u0000]/u.test(value) && !value.split("/").some((part) => !part || part === "." || part === "..");
  const rows: ReferenceToken[] = [];
  for (const entries of declarations.values()) {
    const row = entries.length === 1 ? entries[0] : undefined;
    if (!immutable(row) || row.exported || tokens[row.start]?.text !== "[") continue;
    const close = pairs.get(row.start);
    if (close === undefined || close !== row.end - 1 && !(close === row.end - 3 && tokens[close + 1]?.text === "as" && tokens[close + 2]?.text === "const")) continue;
    const elements = typescriptCollectionSegments(syntax, row.start + 1, close);
    if (!elements.length || elements.some(([start, end]) => { const value = staticValue(start, end, row.scope); return value === null || !relativeLeaf(value); })) continue;
    const uses = tokens.flatMap((token, index) => token.kind === "identifier" && token.text === row.name && index !== row.index ? [index] : []);
    if (embedded.some((expression) => expression.tokens.some((token) => token.kind === "identifier" && token.text === row.name))) continue;
    if (uses.some((index) => !loops.some((loop) => loop.collection === index) && !(tokens[index - 1]?.text === "..." && tokens[requireTaxonomyValue(tokens[index],"tokens[index]").group]?.text === "[" && [",", "]"].includes(tokens[index + 1]?.text ?? "")))) continue;
    const reader = loops.some((loop) => {
      const item = unique(requireTaxonomyValue(tokens[loop.item],"tokens[loop.item]").text);
      if (requireTaxonomyValue(tokens[loop.collection],"tokens[loop.collection]").text !== row.name || row.end >= loop.index || row.scope !== requireTaxonomyValue(tokens[loop.index],"tokens[loop.index]").scope || !immutable(item) || item.index !== loop.item) return false;
      const joined = (start: number, end: number): boolean => {
        if (!imported(start, "node:path", "join") || tokens[start + 1]?.text !== "(" || pairs.get(start + 1) !== end - 1) return false;
        const arguments_ = typescriptCollectionSegments(syntax, start + 2, end - 1);
        return arguments_.length === 2 && arguments_.every(([left, right]) => right - left === 1) && rootValue(requireTaxonomyValue(arguments_[0],"arguments_[0]")[0]) && requireTaxonomyValue(tokens[requireTaxonomyValue(arguments_[1],"arguments_[1]")[0]],"tokens[arguments_[1][0]]").kind === "identifier" && requireTaxonomyValue(tokens[requireTaxonomyValue(arguments_[1],"arguments_[1]")[0]],"tokens[arguments_[1][0]]").text === item.name;
      };
      for (let index = loop.body + 1; index < loop.end; index++) {
        if (requireTaxonomyValue(tokens[index],"tokens[index]").scope !== loop.body || !imported(index, "node:fs", "readFileSync") || tokens[index + 1]?.text !== "(" || functions.some(([start, end]) => start > loop.body && start <= index && index < end)) continue;
        const close = pairs.get(index + 1), arguments_ = close === undefined ? [] : typescriptCollectionSegments(syntax, index + 2, close);
        if (!arguments_.length) continue;
        const [start, end] = requireTaxonomyValue(arguments_[0],"arguments_[0]");
        if (joined(start, end)) return true;
        const alias = end - start === 1 && requireTaxonomyValue(tokens[start],"tokens[start]").kind === "identifier" ? unique(requireTaxonomyValue(tokens[start],"tokens[start]").text) : undefined;
        if (immutable(alias) && alias.end < index && alias.scope === loop.body && joined(alias.start, alias.end)) return true;
      }
      return false;
    });
    if (reader) for (const [index] of elements) if (requireTaxonomyValue(tokens[index],"tokens[index]").kind === "string") {
      const token = requireTaxonomyValue(tokens[index],"tokens[index]"), value = token.text.slice(1, -1), start = token.start + 1;
      rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "path-collection-for-of"), start, end: token.end - 1, value, physicalTargets: [value] });
    }
  }
  return rows.sort((left, right) => left.start - right.start);
}

/** ▶️ `runVitest(bundleRoot, segments, config)`'s required third argument names a configuration owner
 * resolved relative to `bundleRoot` at runtime (every `📜️script.ts` router's own `cwd`), not to
 * whatever else the call happens to quote first — `segments` is very often a literal array of
 * quoted canonical case paths (e.g. `runVitest(this.root, ["../../🧪️tests/🧩️case/🟦️.ts", …], "../../🧪️tests/🎚️config/🟦️.ts")`), so
 * the generic first-quoted-string scanners this file otherwise uses would misidentify a segment
 * name as the config path. This takes the LAST quoted string in the call instead, matching the
 * parameter's trailing position; a call with no quoted configuration argument yields no token and
 * is rejected by the configuration ownership gate. */
function runVitestConfigArgumentTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const match of content.matchAll(/\brunVitest\s*\(([^;\r\n]*)\)/gu)) {
    if (match.index === undefined) continue;
    const args = match[1];
    if (!args) continue;
    const argsStart = match.index + match[0].indexOf(args);
    const last = [...args.matchAll(/["']([^"']+)["']/gu)].at(-1);
    if (!last || last.index === undefined) continue;
    const start = argsStart + last.index + 1;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "run-vitest-config"), start, end: start + last[1]!.length, value: last[1]! });
  }
  return rows;
}

/** 🧪️ A vitest config's own `test.includeSource` / `test.coverage.include` arrays name the
 * same package's in-source (`import.meta.vitest`) suite files by a path relative to the config
 * file's own directory — the house convention documented inline in every such config ("add new
 * in-source files to includeSource/coverage.include only"). Neither key is a call argument, so the
 * call-scoped scanners above never see them; this extracts every quoted literal that is a direct
 * element of either array. Deliberately narrower than the ordinary `test.include` glob key (a
 * standard vitest option, already populated with genuine test globs in several configs and never
 * previously scanned) — only `includeSource` and `include` nested one level inside a `coverage: {…}`
 * block are matched, so this cannot start tracking an unrelated pre-existing `include` array. A
 * glob literal (e.g. `"*.ts"`, seen paired with a real sibling path in `🎠️kernel`'s config) names no
 * single physical file, so any element containing `*` is skipped rather than treated as a target. */
function vitestConfigIncludeArrayTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  const collect = (match: RegExpMatchArray): void => {
    if (match.index === undefined) return;
    const inner = match[1]!;
    const innerStart = match.index + match[0].indexOf("[") + 1;
    for (const literal of inner.matchAll(/["']([^"']+)["']/gu)) {
      if (literal.index === undefined || literal[1]!.includes("*")) continue;
      const start = innerStart + literal.index + 1;
      rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "vitest-config-include"), start, end: start + literal[1]!.length, value: literal[1]! });
    }
  };
  for (const match of content.matchAll(/\bincludeSource\s*:\s*\[([^\]]*)\]/gu)) collect(match);
  for (const match of content.matchAll(/\bcoverage\s*:\s*\{[^{}]*?\binclude\s*:\s*\[([^\]]*)\]/gu)) collect(match);
  return rows;
}

export function typescriptTokens(path: string, content: string): ReferenceToken[] {
  const rows = regexTokens(content, "typescript", "typescript-path", [
    /(?:\bfrom\s*|\bimport\s*\(|\brequire\s*\(|\bimport\s+)["'\s]*([^"'\s)]+)["']/gu,
    /\b(?:worker|url)\s*\(\s*["']([^"']+)["']/giu,
    /\b(?:[A-Za-z_$][\w$]*(?:Path|File|Filename|Root|Schema|Taxonomy|Config|Entry|Target|Source|Output|Input)[\w$]*|(?:path|file|filename|root|schema|taxonomy|config|entry|target|source|output|input))\s*(?:=|:)\s*["']([^"']+)["']/giu,
    /\b(?:resolve|join|readFileSync|writeFileSync|existsSync|openSync|Bun\.file|policyReadFileSafe)\s*\([^;\r\n]*?["']([^"']+)["']/giu,
  ]);
  rows.push(...runVitestConfigArgumentTokens(content));
  if (basename(path) === "🟦️.ts" && basename(dirname(path)) === "🧪️tests") rows.push(...vitestConfigIncludeArrayTokens(content));
  rows.push(...ticketImportantProseReferenceAuthority(content).map((entry) => ({ ...entry, adapter: "typescript" as const })));
  rows.push(...typescriptLeadingDocumentationReferenceAuthority(content).map((entry) => ({ ...entry, adapter: "typescript" as const })));
  rows.push(...typescriptCommentPathReferenceAuthority(content).map((entry) => ({ ...entry, adapter: "typescript" as const })));
  let catalogLineStart = 0, catalogLineEnd = content.indexOf("\n"), previousCatalogLine = -1;
  for (const match of content.matchAll(/\bimport\.meta\.glob\s*\(/gu)) {
    if (match.index === undefined) continue;
    while (catalogLineEnd >= 0 && catalogLineEnd < match.index) {
      catalogLineStart = catalogLineEnd + 1;
      catalogLineEnd = content.indexOf("\n", catalogLineStart);
    }
    if (previousCatalogLine === catalogLineStart) continue;
    previousCatalogLine = catalogLineStart;
    const end = catalogLineEnd < 0 ? content.length : catalogLineEnd + 1, value = content.slice(catalogLineStart, end);
    const selectors = [...value.matchAll(/["']([^"']*modelDefinitions[^"']*)["']/gu)].map((row) => row[1]);
    if (selectors.length === 0) continue;
    const start = catalogLineStart;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "artifact-catalog-glob"), start, end, value, rewriteKind: "artifact-catalog-glob", rewriteData: { selectors: JSON.stringify(selectors) } });
  }
  for (const match of content.matchAll(/(🖼️assets\/🏗️modelDefinitions\/<modelDefinition>\/\{[\s\S]*?\})/gu)) {
    if (match.index === undefined) continue;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, match.index, "artifact-catalog-comment"), start: match.index, end: match.index + requireTaxonomyValue(match[1],"match[1]").length, value: requireTaxonomyValue(match[1],"required reference value"), rewriteKind: "artifact-catalog-prose", rewriteData: { form: "catalog-grammar" } });
  }
  for (const match of content.matchAll(/(🖼️assets\/🏗️modelDefinitions\/)/gu)) {
    if (match.index === undefined || rows.some((token) => token.start <= match.index! && token.end >= match.index! + requireTaxonomyValue(match[1],"match[1]").length)) continue;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, match.index, "artifact-catalog-marker"), start: match.index, end: match.index + requireTaxonomyValue(match[1],"match[1]").length, value: requireTaxonomyValue(match[1],"required reference value"), rewriteKind: "artifact-catalog-prose", rewriteData: { form: "root-marker" } });
  }
  rows.push(...typescriptPathCollectionReferenceAuthority(content));
  const hasForOfCollection = typescriptCollectionHasForOf(content);
  for (const declaration of content.matchAll(/\bconst\s+((?:[A-Za-z_$][\w$]*(?:Sources|Paths|Files)|paths|sources|files))\s*=\s*\[([\s\S]*?)\]\s*(?:\.map\b|;)/gu)) {
    if (hasForOfCollection) continue;
    if (declaration.index === undefined || !new RegExp(`\\b${declaration[1]}\\b[\\s\\S]*?\\.map\\([\\s\\S]*?\\b(?:policyReadFileSafe|readFileSync|Bun\\.file)\\b`, "u").test(content.slice(declaration.index))) continue;
    const fragmentStart = declaration.index + declaration[0].indexOf(requireTaxonomyValue(declaration[2],"declaration[2]"));
    for (const token of regexTokens(requireTaxonomyValue(declaration[2],"declaration[2]"), "typescript", "path-collection", [/["']([^"']+)["']/gu])) rows.push({ ...token, start: fragmentStart + token.start, end: fragmentStart + token.end, structuredLocation: lineLocation(content, fragmentStart + token.start, "path-collection") });
  }
  return rows;
}
