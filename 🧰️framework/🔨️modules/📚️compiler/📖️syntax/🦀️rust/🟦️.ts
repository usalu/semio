/** 👁️ Exact Rust item visibility. */
export type RustStructuralVisibility = "private" | "pub" | `pub(${string})`;

/** 🔤️ Lexical Rust token categories. */
type RustTokenKind = "identifier" | "string" | "number" | "punctuation";

/** 🧱️ Source token with character offsets. */
export interface RustToken {
  readonly kind: RustTokenKind;
  readonly text: string;
  readonly start: number;
  readonly end: number;
}

/** 🏷️ Consecutive item attributes. */
export interface RustAttributes {
  readonly ranges: readonly (readonly [number, number])[];
  readonly next: number;
}

/** 🔍️ Parsed item visibility. */
export interface RustVisibility {
  readonly value: RustStructuralVisibility;
  readonly next: number;
}

/** 🔤️ Recognizes a Rust identifier code point without interpreting comments or literal contents. */
function rustIdentifierPart(character: string): boolean {
  return character === "_" || /[\p{L}\p{N}]/u.test(character);
}

/** 🧵️ Decodes the identity-bearing value of one normal, byte, or raw Rust string token. */
export function rustStringValue(token: RustToken | undefined): string | null {
  if (!token || token.kind !== "string") return null;
  let text = token.text;
  if (text.startsWith("b") && !text.startsWith("br")) text = text.slice(1);
  if (text.startsWith("br") || text.startsWith("r")) {
    const quote = text.indexOf('"');
    if (quote < 0) return null;
    const hashes = text.slice(text.startsWith("br") ? 2 : 1, quote).length;
    return text.slice(quote + 1, text.length - hashes - 1);
  }
  if (!text.startsWith('"') || !text.endsWith('"')) return null;
  const body = text.slice(1, -1), simple: Readonly<Record<string, string>> = { n: "\n", r: "\r", t: "\t", "0": "\0", "\\": "\\", '"': '"', "'": "'" };
  let value = "";
  for (let index = 0; index < body.length; index++) {
    if (body[index] !== "\\") { value += body[index]; continue; }
    const next = body[++index];
    if (next === undefined) return null;
    if (next in simple) { value += simple[next]; continue; }
    if (next === "x" && /^[0-9a-f]{2}$/iu.test(body.slice(index + 1, index + 3))) { value += String.fromCharCode(parseInt(body.slice(index + 1, index + 3), 16)); index += 2; continue; }
    if (next === "u" && body[index + 1] === "{") {
      const close = body.indexOf("}", index + 2), hex = body.slice(index + 2, close).replaceAll("_", "");
      if (close < 0 || !/^[0-9a-f]{1,6}$/iu.test(hex)) return null;
      const point = parseInt(hex, 16);
      if (point > 0x10ffff || (point >= 0xd800 && point <= 0xdfff)) return null;
      value += String.fromCodePoint(point); index = close; continue;
    }
    if (next === "\n" || (next === "\r" && body[index + 1] === "\n")) { while (/\s/u.test(body[index + 1] ?? "")) index++; continue; }
    return null;
  }
  return value;
}

/** 🧱️ Tokenizes Rust with atomic literals and nested comments. */
export function rustTokens(source: string): RustToken[] {
  const tokens: RustToken[] = [];
  const punctuation = ["::", "=>", "->", "..=", "...", "..", "&&", "||", "<=", ">=", "==", "!=", "<<=", ">>=", "<<", ">>"];
  let index = 0;
  while (index < source.length) {
    const start = index;
    const character = source[index]!;
    if (/\s/u.test(character)) {
      index += 1;
      continue;
    }
    if (source.startsWith("//", index)) {
      index = source.indexOf("\n", index + 2);
      if (index < 0) break;
      continue;
    }
    if (source.startsWith("/*", index)) {
      let depth = 1;
      index += 2;
      while (index < source.length && depth > 0) {
        if (source.startsWith("/*", index)) {
          depth += 1;
          index += 2;
        } else if (source.startsWith("*/", index)) {
          depth -= 1;
          index += 2;
        } else index += 1;
      }
      continue;
    }
    const rawPrefix = source.startsWith("br", index) ? 2 : source.startsWith("r", index) ? 1 : 0;
    if (rawPrefix > 0) {
      let cursor = index + rawPrefix;
      while (source[cursor] === "#") cursor += 1;
      if (source[cursor] === '"') {
        const hashes = cursor - index - rawPrefix;
        const suffix = `"${"#".repeat(hashes)}`;
        const close = source.indexOf(suffix, cursor + 1);
        index = close < 0 ? source.length : close + suffix.length;
        tokens.push({ kind: "string", text: source.slice(start, index), start, end: index });
        continue;
      }
    }
    if (character === '"' || (character === "b" && source[index + 1] === '"')) {
      index += character === "b" ? 2 : 1;
      while (index < source.length) {
        if (source[index] === "\\") index += 2;
        else if (source[index] === '"') {
          index += 1;
          break;
        } else index += 1;
      }
      tokens.push({ kind: "string", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (character === "'") {
      let cursor = index + 1;
      if (source[cursor] === "\\") {
        const selector = source[cursor + 1];
        cursor += 2;
        if (selector === "x") cursor += 2;
        else if (selector === "u" && source[cursor] === "{") {
          const close = source.indexOf("}", cursor);
          cursor = close < 0 ? source.length : close + 1;
        }
      } else if (cursor < source.length) cursor += String.fromCodePoint(source.codePointAt(cursor)!).length;
      if (source[cursor] === "'") {
        index = cursor + 1;
        tokens.push({ kind: "string", text: source.slice(start, index), start, end: index });
        continue;
      }
    }
    if (source.startsWith("r#", index) && source[index + 2] && rustIdentifierPart(source[index + 2]!) && !/[0-9]/u.test(source[index + 2]!)) {
      index += 2;
      while (index < source.length) {
        const next = String.fromCodePoint(source.codePointAt(index)!);
        if (!rustIdentifierPart(next)) break;
        index += next.length;
      }
      tokens.push({ kind: "identifier", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (rustIdentifierPart(character) && !/[0-9]/u.test(character)) {
      index += character.length;
      while (index < source.length) {
        const next = String.fromCodePoint(source.codePointAt(index)!);
        if (!rustIdentifierPart(next)) break;
        index += next.length;
      }
      tokens.push({ kind: "identifier", text: source.slice(start, index), start, end: index });
      continue;
    }
    if (/[0-9]/u.test(character)) {
      index += 1;
      while (index < source.length && /[\p{L}\p{N}_]/u.test(source[index]!)) index += 1;
      if (source[index] === "." && source[index + 1] !== "." && !/[\p{L}_]/u.test(source[index + 1] ?? "")) {
        index += 1;
        while (index < source.length && /[\p{L}\p{N}_]/u.test(source[index]!)) index += 1;
      }
      tokens.push({ kind: "number", text: source.slice(start, index), start, end: index });
      continue;
    }
    const operator = punctuation.find((candidate) => source.startsWith(candidate, index));
    index += operator?.length ?? 1;
    tokens.push({ kind: "punctuation", text: operator ?? character, start, end: index });
  }
  return tokens;
}

/** 🧩️ Pairs Rust delimiter tokens so all structural scans can skip nested syntax exactly. */
export function rustTokenPairs(tokens: readonly RustToken[]): ReadonlyMap<number, number> {
  const pairs = new Map<number, number>();
  const stack: { readonly index: number; readonly token: string }[] = [];
  const closeFor: Readonly<Record<string, string>> = { "(": ")", "[": "]", "{": "}" };
  for (let index = 0; index < tokens.length; index += 1) {
    const text = tokens[index]!.text;
    if (Object.hasOwn(closeFor, text)) stack.push({ index, token: text });
    else if (text === ")" || text === "]" || text === "}") {
      const open = stack.at(-1);
      if (open && closeFor[open.token] === text) {
        stack.pop();
        pairs.set(open.index, index);
        pairs.set(index, open.index);
      }
    }
  }
  return pairs;
}

/** 🔑️ Normalizes identifier symbols while preserving raw keyword token spelling. */
export function rustIdentifierSymbol(token: RustToken | undefined): string | null {
  return token?.kind === "identifier" ? token.text.replace(/^r#/u, "") : null;
}

/** 🪆️ Lexical scope of a compile input. */
export type RustCompileScope = Readonly<{ kind: "module"; modulePath: readonly string[] } | { kind: "local-block"; startOffset: number; endOffset: number; compilerAttributes?: readonly ("test" | "doc")[] }>;

/** ✨️ Source origin of a finite local macro expansion. */
export type RustCompileExpansion = Readonly<{ kind: "local-macro"; macro: string; definitionLine: number; invocationLine: number; definitionOffset: number; templateOffset: number; invocationOffset: number; scope: RustCompileScope }>;

/** 📎️ Authored compile input with lexical provenance. */
export type RustCompileReference = Readonly<{ kind: "include" | "include_str" | "include_bytes" | "path"; path: string; line: number; base?: "manifest" | "generated"; modulePath?: readonly string[]; inlineBase?: string; directory?: true; expansion?: RustCompileExpansion }>;

/** 🧷️ Discovers authored compile inputs in all configurations and macro templates; unsupported expressions fail closed. */
/** 📤️ A token generator defers its emitted include expression to the consuming compilation. */
export type RustGeneratedTokenOutput = Readonly<{ macro: string; start: number; end: number; line: number; origin: Readonly<{ provider: "quote"; symbol: "quote"; kind: "direct-import" | "qualified"; importStart?: number; importEnd?: number }>; inputs: readonly Readonly<{ kind: "include" | "include_str" | "include_bytes"; expression: string; start: number; end: number; line: number }>[] }>;

/** 🪪️ Retains proven token-output syntax separately from the current crate's authored inputs. */
function rustGeneratedTokenOutputs(source: string, tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): readonly Readonly<{ first: number; close: number; output: RustGeneratedTokenOutput }>[] {
  const imports = new Map<string, Readonly<{ start: number; end: number }>>(), ambiguous = new Set<string>();
  const enclosing = (index: number) => [...pairs].some(([open, close]) => open < index && close > index && tokens[open]?.text === "{");
  let qualifiedClosed = true;
  for (let index = 0; index < tokens.length; index++) {
    if ((tokens[index]?.text === "mod" || tokens[index]?.text === "extern" && tokens[index + 1]?.text === "crate") && rustIdentifierSymbol(tokens[index + (tokens[index]?.text === "mod" ? 1 : 2)]) === "quote") qualifiedClosed = false;
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    if (end < 0) continue;
    const body = tokens.slice(index + 1, end), direct = body.length === 3 || body.length === 5 && body[3]?.text === "as";
    const fromQuote = body[0]?.text === "quote" && body[1]?.text === "::" && body[2]?.text === "quote", alias = direct && fromQuote ? rustIdentifierSymbol(body.at(-1)) : null;
    const attributes = [...pairs].some(([open, close]) => close === index - 1 && tokens[open]?.text === "[" && tokens[open - 1]?.text === "#");
    if (direct && fromQuote && alias && !enclosing(index) && !attributes && tokens[index - 1]?.text !== "pub") {
      if (imports.has(alias)) ambiguous.add(alias);
      else imports.set(alias, { start: tokens[index]!.start, end: tokens[end]!.end });
    } else if (body.some(token => token.text === "*")) { qualifiedClosed = false; ambiguous.add("*"); }
    else for (const token of body) if (rustIdentifierSymbol(token) === "quote") { qualifiedClosed = false; ambiguous.add("quote"); }
    index = end;
  }
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    if (end < 0) continue;
    for (const [alias, origin] of imports) if (tokens[index]!.start !== origin.start && tokens.slice(index + 1, end).some(token => rustIdentifierSymbol(token) === alias)) ambiguous.add(alias);
    index = end;
  }
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text === "macro_rules" && tokens[index + 1]?.text === "!") { const name = rustIdentifierSymbol(tokens[index + 2]); if (name) ambiguous.add(name); }
    if (tokens[index]?.text === "macro" || tokens[index]?.text === "mod") { const name = rustIdentifierSymbol(tokens[index + 1]); if (name) ambiguous.add(name); }
  }
  const ranges: Readonly<{ first: number; close: number; output: RustGeneratedTokenOutput }>[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index + 1]?.text !== "!") continue;
    let first = index;
    while (first >= 2 && tokens[first - 1]?.text === "::" && tokens[first - 2]?.kind === "identifier") first -= 2;
    const name = tokens.slice(first, index + 1).map(token => token.text).join(""), close = pairs.get(index + 2);
    if (close === undefined || name !== "quote::quote" && !imports.has(name) && name !== "quote") continue;
    const inputs: RustGeneratedTokenOutput["inputs"][number][] = [];
    for (let at = index + 3; at < close; at++) {
      const kind = rustIdentifierSymbol(tokens[at]);
      if (!["include", "include_str", "include_bytes"].includes(kind ?? "") || tokens[at + 1]?.text !== "!") continue;
      const end = pairs.get(at + 2);
      if (end === undefined || end > close) throw Error("Unsupported Rust compile token-output delimiter");
      inputs.push({ kind: kind as RustGeneratedTokenOutput["inputs"][number]["kind"], expression: source.slice(tokens[at + 2]!.end, tokens[end]!.start), start: tokens[at]!.start, end: tokens[end]!.end, line: source.slice(0, tokens[at]!.start).split("\n").length });
      at = end;
    }
    if (!inputs.length) continue;
    const qualified = name === "quote::quote", imported = imports.get(name);
    if (!qualifiedClosed || ambiguous.has("*") || ambiguous.has(name) || ambiguous.has("quote") || !qualified && !imported) throw Error("Unsupported Rust compile token-output origin: " + name);
    const origin: RustGeneratedTokenOutput["origin"] = { provider: "quote", symbol: "quote", kind: qualified ? "qualified" : "direct-import", ...(imported && !qualified ? { importStart: imported.start, importEnd: imported.end } : {}) };
    ranges.push({ first, close, output: { macro: name, start: tokens[first]!.start, end: tokens[close]!.end, line: source.slice(0, tokens[first]!.start).split("\n").length, origin, inputs } });
    index = close;
  }
  return ranges;
}

export function inspectRustCompileReferences(source: string, onGeneratedTokens?: (output: RustGeneratedTokenOutput) => void): readonly RustCompileReference[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), references: RustCompileReference[] = [];
  const generated = rustGeneratedTokenOutputs(source, tokens, pairs);
  if (generated.length && !onGeneratedTokens) throw Error("Unsupported Rust compile token output requires an explicit generated-input observer");
  for (const range of generated) onGeneratedTokens!(range.output);
  type Input = Readonly<{ path: string; base?: "manifest" | "generated"; directory?: true }>;
  const literal = (start: number, end: number, bindings: ReadonlyMap<string, Input> = new Map()): Input | null => {
    if (end === start + 2 && tokens[start]?.text === "$" && tokens[start + 1]?.kind === "identifier") return bindings.get(rustIdentifierSymbol(tokens[start + 1])!) ?? null;
    if (end === start + 1 && tokens[start]?.kind === "string") { const path = rustStringValue(tokens[start]); return path === null ? null : { path }; }
    if (tokens[start + 1]?.text !== "!" || pairs.get(start + 2) !== end - 1) return null;
    if (rustIdentifierSymbol(tokens[start]) === "env" && end === start + 5) {
      const name = rustStringValue(tokens[start + 3]);
      return name === "CARGO_MANIFEST_DIR" ? { path: "", base: "manifest" } : name === "OUT_DIR" ? { path: "", base: "generated" } : null;
    }
    if (rustIdentifierSymbol(tokens[start]) !== "concat") return null;
    const parts = rustTokenSegments(tokens, pairs, start + 3, end - 1, ",").map(([a, b]) => literal(a, b, bindings));
    if (!parts.length || parts.some((part, index) => part === null || index > 0 && part.base)) return null;
    const values = parts as Input[];
    return { path: values.map((part) => part.path).join(""), ...(values[0]!.base ? { base: values[0]!.base } : {}) };
  };
  const lineAt = (offset: number): number => source.slice(0, offset).split("\n").length;
  const reserved = new Set(["_", "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro", "override", "priv", "try", "typeof", "unsized", "virtual", "yield"]);
  const identifier = (index: number): boolean => tokens[index]?.kind === "identifier" && !reserved.has(tokens[index]!.text);
  const pattern = (index: number): boolean => identifier(index) || tokens[index]?.text === "_";
  const pathHead = (index: number): boolean => identifier(index) || ["self", "Self", "super", "crate"].includes(tokens[index]?.text ?? "");
  const segments = (start: number, end: number, trailing: boolean): readonly (readonly [number, number])[] | null => {
    const values: (readonly [number, number])[] = [];
    let first = start;
    for (let cursor = start; cursor < end; cursor++) {
      const close = pairs.get(cursor);
      if (close !== undefined && close > cursor) { cursor = close; continue; }
      if (tokens[cursor]?.text !== ",") continue;
      if (cursor === first) return null;
      values.push([first, cursor]); first = cursor + 1;
    }
    if (first < end) values.push([first, end]);
    else if (first > start && !trailing) return null;
    return values;
  };
  const closureBody = (start: number, end: number): number | null => {
    let cursor = tokens[start]?.text === "move" ? start + 1 : start;
    if (tokens[cursor]?.text === "||") return cursor + 1;
    if (tokens[cursor]?.text !== "|") return null;
    const first = ++cursor;
    while (cursor < end && tokens[cursor]?.text !== "|") {
      const close = pairs.get(cursor);
      if (close !== undefined && close > cursor) cursor = close;
      cursor++;
    }
    if (cursor === end) return null;
    const parameters = segments(first, cursor, true);
    if (!parameters) return null;
    for (const [a, b] of parameters) {
      if (!pattern(a)) return null;
      let at = a + 1;
      if (at === b) continue;
      if (tokens[at++]?.text !== ":") return null;
      if (tokens[at]?.text === "&") { at++; if (tokens[at]?.text === "mut") at++; }
      if (tokens[at]?.text === "[" && pairs.get(at) === b - 1 && b === at + 3 && pathHead(at + 1)) continue;
      if (!pathHead(at++)) return null;
      while (at < b && tokens[at]?.text === "::" && identifier(at + 1)) at += 2;
      if (at !== b) return null;
    }
    return cursor + 1;
  };
  const argumentsAt = (start: number, end: number): readonly (readonly [number, number])[] | null => {
    const args: (readonly [number, number])[] = [];
    let cursor = start;
    while (cursor < end) {
      const first = cursor, closure = ["|", "||", "move"].includes(tokens[first]?.text ?? "");
      if (closure) {
        const body = closureBody(first, end);
        if (body === null || body === end || tokens[body]?.text === "->") return null;
        cursor = body;
      }
      while (cursor < end && tokens[cursor]?.text !== ",") {
        const close = pairs.get(cursor);
        if (close !== undefined && close > cursor) cursor = close;
        cursor++;
      }
      if (cursor === first) return null;
      args.push([first, cursor]);
      if (cursor < end && ++cursor === end) return null;
    }
    return args;
  };
  const priorities: Readonly<Record<string, number>> = { "||": 1, "&&": 2, "==": 3, "!=": 3, "<": 4, "<=": 4, ">": 4, ">=": 4, "|": 5, "^": 6, "&": 7, "<<": 8, ">>": 8, "+": 9, "-": 9, "*": 10, "/": 10, "%": 10 };
  const expression = (start: number, end: number, minimum = 0): number | null => {
    let cursor = start;
    let compared = false;
    const token = tokens[cursor];
    if (!token || cursor === end) return null;
    if (["|", "||", "move"].includes(token.text)) {
      const body = closureBody(cursor, end);
      return body === null || tokens[body]?.text === "->" ? null : expression(body, end);
    }
    if (["&", "&&", "!", "-", "*"].includes(token.text)) {
      const value = expression(cursor + (["&", "&&"].includes(token.text) && tokens[cursor + 1]?.text === "mut" ? 2 : 1), end, 11);
      if (value === null) return null;
      cursor = value;
    } else if (token.text === "{") {
      const close = pairs.get(cursor);
      if (close === undefined || close >= end) return null;
      cursor++;
      while (cursor < close) {
        if (tokens[cursor]?.text === "let") {
          const equal = rustFindTopLevel(tokens, pairs, cursor + 1, close, new Set(["="]));
          if (equal < 0) return null;
          const first = cursor + (tokens[cursor + 1]?.text === "mut" ? 2 : 1);
          if (equal !== first + 1 || !pattern(first)) {
            if (tokens[first]?.text !== "(" || pairs.get(first) !== equal - 1) return null;
            const patterns = segments(first + 1, equal - 1, true);
            if (!patterns?.length || patterns.some(([a, b]) => b !== a + 1 || !pattern(a))) return null;
          }
          const value = expression(equal + 1, close);
          if (value === null || tokens[value]?.text !== ";") return null;
          cursor = value + 1;
        } else {
          const value = expression(cursor, close);
          if (value === null) return null;
          if (value === close) { cursor = value; break; }
          if (tokens[value]?.text !== ";") return null;
          cursor = value + 1;
        }
      }
      cursor = close + 1;
    } else if (token.text === "(" || token.text === "[") {
      const close = pairs.get(cursor), values = close === undefined ? null : segments(cursor + 1, close, true);
      if (close === undefined || close >= end || values === null || values.some(([a, b]) => expression(a, b) !== b)) return null;
      cursor = close + 1;
    } else if (["identifier", "number", "string"].includes(token.kind)) {
      const path = pathHead(cursor);
      if (token.kind === "identifier" && !path && !["true", "false"].includes(token.text)) return null;
      cursor++;
      while (path && tokens[cursor]?.text === "::" && identifier(cursor + 1)) cursor += 2;
      if (path && tokens[cursor]?.text === "!" && pairs.has(cursor + 1)) cursor = pairs.get(cursor + 1)! + 1;
      else if (path && tokens[cursor]?.text === "{") {
        const close = pairs.get(cursor), fields = close === undefined ? null : segments(cursor + 1, close, true);
        if (close === undefined || close >= end || fields === null || fields.some(([a, b]) => !identifier(a) || b !== a + 1 && (tokens[a + 1]?.text !== ":" || expression(a + 2, b) !== b))) return null;
        cursor = close + 1;
      }
    } else return null;
    while (cursor < end) {
      if (tokens[cursor]?.text === "(" || tokens[cursor]?.text === "[") {
        const close = pairs.get(cursor), values = close === undefined ? null : segments(cursor + 1, close, true);
        if (close === undefined || close >= end || values === null || values.some(([a, b]) => expression(a, b) !== b) || tokens[cursor]?.text === "[" && values.length !== 1) return null;
        cursor = close + 1; continue;
      }
      if (tokens[cursor]?.text === "." && (identifier(cursor + 1) || tokens[cursor + 1]?.kind === "number" && /^\d+(?:\.\d+)*$/u.test(tokens[cursor + 1]!.text))) { cursor += 2; continue; }
      const operator = tokens[cursor]?.text ?? "", priority = Object.hasOwn(priorities, operator) ? priorities[operator]! : 0;
      if (!priority || priority < minimum) break;
      if (priority === 3 || priority === 4) { if (compared) return null; compared = true; }
      else if (priority < 3) compared = false;
      const value = expression(cursor + 1, end, priority + 1);
      if (value === null) return null;
      cursor = value;
    }
    return cursor;
  };
  const fragment = (start: number, end: number, kind: string): boolean => {
    if (kind === "ident") return end === start + 1 && tokens[start]?.kind === "identifier" && tokens[start]?.text !== "_";
    if (kind === "literal") return end === start + 1 && (["string", "number"].includes(tokens[start]?.kind ?? "") || ["true", "false"].includes(tokens[start]?.text ?? ""));
    if (kind === "tt") return end === start + 1 || pairs.get(start) === end - 1;
    return expression(start, end) === end;
  };
  const templates: { name: string; start: number; end: number; definition: number; parameters: readonly (readonly [number, number])[]; required: ReadonlySet<string>; scopeEnd: number; scope?: RustCompileScope; hasInputs: boolean; bindings: { values: ReadonlyMap<string, Input>; invocation: number }[]; supported: boolean }[] = [];
  for (let index = 0; index < tokens.length; index++) {
    const emitted = generated.find(range => range.first <= index && index <= range.close);
    if (emitted) { index = emitted.close; continue; }
    const attributes = rustAttributes(tokens, pairs, index, tokens.length), definition = attributes.next;
    if (tokens[definition]?.text !== "macro_rules" || tokens[definition + 1]?.text !== "!") continue;
    const name = rustIdentifierSymbol(tokens[definition + 2]), body = definition + 3, close = pairs.get(body), matcher = body + 1, matcherClose = pairs.get(matcher);
    if (!name || close === undefined) continue;
    const transcriber = (matcherClose ?? body) + 2, transcriberClose = pairs.get(transcriber);
    const segmentsForParameters = matcherClose === undefined ? null : segments(matcher + 1, matcherClose, false), parameters = segmentsForParameters ?? [];
    const required = new Set<string>();
    let hasInputs = false;
    for (let at = transcriber + 1; at < (transcriberClose ?? close); at++) if (["include", "include_str", "include_bytes"].includes(rustIdentifierSymbol(tokens[at]) ?? "") && tokens[at + 1]?.text === "!") {
      hasInputs = true;
      const end = pairs.get(at + 2);
      if (end !== undefined) for (let parameter = at + 3; parameter < end; parameter++) if (tokens[parameter]?.text === "$" && tokens[parameter + 1]?.kind === "identifier") required.add(rustIdentifierSymbol(tokens[parameter + 1])!);
    }
    const simple = segmentsForParameters !== null && matcherClose !== undefined && tokens[matcherClose + 1]?.text === "=>" && transcriberClose !== undefined && (transcriberClose + 1 === close || transcriberClose + 2 === close && tokens[transcriberClose + 1]?.text === ";") && parameters.every(([a, b]) => b === a + 4 && tokens[a]?.text === "$" && tokens[a + 1]?.kind === "identifier" && tokens[a + 2]?.text === ":" && ["literal", "expr", "tt", "ident"].includes(tokens[a + 3]?.text ?? "")) && new Set(parameters.map(([a]) => rustIdentifierSymbol(tokens[a + 1]))).size === parameters.length && !rustMetadataAttributes(tokens, pairs, attributes).items.some((item) => !["allow", "warn", "deny", "forbid", "expect", "doc", "rustfmt::skip"].includes(item.name)) && !tokens.slice(transcriber + 1, transcriberClose).some((token, at, body) => token.text === "macro_rules" || rustIdentifierSymbol(token) === name && body[at + 1]?.text === "!");
    const scopeEnd = hasInputs ? [...pairs].filter(([a, b]) => a < definition && b > close && tokens[a]?.text === "{").sort((left, right) => right[0] - left[0])[0]?.[1] ?? tokens.length : close;
    templates.push({ name, start: body, end: close, definition, parameters, required, scopeEnd, hasInputs, bindings: [], supported: simple });
    index = definition;
  }
  for (const template of templates) {
    if (!template.hasInputs) continue;
    if (templates.filter((candidate) => candidate.name === template.name).length > 1) template.supported = false;
    if (template.supported) for (let invocation = 0; invocation < template.scopeEnd; invocation++) {
      const { name, parameters } = template;
      if (invocation > template.end) {
        const attributes = rustAttributes(tokens, pairs, invocation, template.scopeEnd);
        if (rustMetadataAttributes(tokens, pairs, attributes).items.some((item) => !["allow", "warn", "deny", "forbid", "expect", "doc", "path", "rustfmt::skip"].includes(item.name))) template.supported = false;
      }
      if (invocation > template.end && tokens[invocation]?.text === "mod" && tokens[invocation + 1]?.kind === "identifier" && tokens[rustFindTopLevel(tokens, pairs, invocation + 2, template.scopeEnd, new Set([";", "{"]))]?.text === ";" || invocation > template.end && rustIdentifierSymbol(tokens[invocation]) === "include" && tokens[invocation + 1]?.text === "!") template.supported = false;
      if (rustIdentifierSymbol(tokens[invocation]) !== name || tokens[invocation + 1]?.text !== "!") continue;
      if (tokens[invocation - 1]?.text === "macro_rules") continue;
      if (invocation < template.end || tokens[invocation - 1]?.text === "::" || templates.some((owner) => invocation > owner.start && invocation < owner.end) || [...pairs].some(([a, b]) => a < invocation && b > invocation && tokens[a - 1]?.text === "!")) { template.supported = false; continue; }
      const invocationClose = pairs.get(invocation + 2);
      if (invocationClose === undefined) { template.supported = false; continue; }
      const args = argumentsAt(invocation + 3, invocationClose);
      if (!args || args.length !== parameters.length) { template.supported = false; continue; }
      const bound = new Map<string, Input>();
      for (let at = 0; at < parameters.length; at++) {
        const [a] = parameters[at]!, [x, y] = args[at]!, parameter = rustIdentifierSymbol(tokens[a + 1])!;
        if (!fragment(x, y, tokens[a + 3]!.text)) { template.supported = false; continue; }
        if (template.required.has(parameter)) {
          const value = literal(x, y);
          if (!value) template.supported = false;
          else bound.set(parameter, value);
        }
      }
      if (bound.size === template.required.size) template.bindings.push({ values: bound, invocation });
    }
  }
  const add = (kind: RustCompileReference["kind"], input: Input | null, offset: number, modulePath: readonly string[] = [], inlineBase?: string, expansion?: RustCompileExpansion): void => {
    const line = lineAt(offset);
    if (!input || !input.path && !input.directory || input.base && !input.path.startsWith("/") || input.base === "generated" && input.path.split(/[\\/]/u).includes("..")) throw new Error(`Unsupported Rust compile expression at line ${line}: ${kind}`);
    references.push({ kind, ...input, line, ...(modulePath.length ? { modulePath } : {}), ...(kind === "path" && inlineBase !== undefined ? { inlineBase } : {}), ...(expansion ? { expansion } : {}) });
  };
  const visit = (start: number, end: number, modulePath: readonly string[], inlineBase?: string, scope: RustCompileScope | null = { kind: "module", modulePath }): void => {
    for (let index = start; index < end; index++) {
      const emitted = generated.find(range => range.first === index);
      if (emitted) { index = emitted.close; continue; }
      const inner = tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[", innerClose = inner ? pairs.get(index + 2) : undefined;
      if (inner && (innerClose === undefined || innerClose >= end)) throw new Error(`Unsupported Rust compile attribute delimiter at line ${lineAt(tokens[index]!.start)}`);
      const attributes = inner ? { ranges: [[index + 3, innerClose!]] as readonly (readonly [number, number])[], next: innerClose! + 1 } : rustAttributes(tokens, pairs, index, end), visibility = rustVisibility(tokens, pairs, attributes.next), token = tokens[visibility.next];
      for (const [a, b] of attributes.ranges) {
        const docs = rustMetadataAttributes(tokens, pairs, { ranges: [[a, b]], next: attributes.next }).items.filter((item) => item.name === "doc");
        for (let at = a; at < b; at++) if (["include", "include_str", "include_bytes"].includes(rustIdentifierSymbol(tokens[at]) ?? "") && tokens[at + 1]?.text === "!") {
          if (!docs.some((doc) => at >= doc.start && at < doc.end) || [...pairs].some(([open, close]) => open < at && close > at && tokens[open - 1]?.text === "!" && rustIdentifierSymbol(tokens[open - 2]) !== null && !["concat", "include", "include_str", "include_bytes", "env"].includes(rustIdentifierSymbol(tokens[open - 2])!))) throw new Error(`Unsupported Rust compile attribute expression at line ${lineAt(tokens[at]!.start)}`);
        }
        visit(a, b, modulePath, inlineBase, null);
      }
      if (inner) { index = innerClose!; continue; }
      if (token?.text === "macro_rules") {
        const template = templates.find((candidate) => candidate.definition === visibility.next);
        if (template && scope) template.scope = scope;
        else if (template) template.supported = false;
      }
      if (token?.text === "fn" && tokens[visibility.next + 1]?.kind === "identifier" && tokens[visibility.next + 2]?.text === "(") {
        const parameterClose = pairs.get(visibility.next + 2), boundary = parameterClose === undefined ? -1 : rustFindTopLevel(tokens, pairs, parameterClose + 1, end, new Set([";", "{"]));
        let depth = 0;
        const signature = parameterClose !== undefined && boundary >= 0 && tokens.slice(parameterClose + 1, boundary).every((token) => {
          if (token.text === "<") depth++;
          if (token.text === ">") depth--;
          return depth >= 0 && (token.kind === "identifier" || ["->", "::", "<", ">", ",", "&", "'", "[", "]"].includes(token.text));
        }) && depth === 0;
        const close = boundary < 0 ? undefined : pairs.get(boundary);
        if (close !== undefined) {
          const inertAttributes = attributes.ranges.every(([start, end]) => tokens[start]?.text === "test" && end === start + 1 || tokens[start]?.text === "doc" && end === start + 3 && tokens[start + 1]?.text === "=" && tokens[start + 2]?.kind === "string");
          const compilerAttributes = attributes.ranges.map(([start]) => tokens[start]!.text as "test" | "doc");
          const plain = scope?.kind === "module" && signature && inertAttributes && !["async", "unsafe", "const", "extern"].includes(tokens[visibility.next - 1]?.text ?? "") && tokens[visibility.next - 2]?.text !== "extern";
          visit(boundary + 1, close, modulePath, inlineBase, plain ? { kind: "local-block", startOffset: tokens[boundary]!.start, endOffset: tokens[close]!.end, ...(compilerAttributes.length ? { compilerAttributes } : {}) } : null);
          index = close; continue;
        }
      }
      if (token?.text === "mod" && tokens[visibility.next + 1]?.kind === "identifier") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
        if (boundary >= 0) {
          const paths = rustPathAttributes(tokens, pairs, attributes);
          if (tokens[boundary]?.text === "{") {
            const close = pairs.get(boundary);
            if (close === undefined) throw new Error("Unsupported Rust compile module body");
            if (attributes.ranges.some(([start, end]) => tokens[start]?.text === "cfg_attr" && rustPathAttributes(tokens, pairs, { ranges: [[start, end]], next: attributes.next }).length)) throw new Error("Unsupported Rust compile conditional inline module mount");
            for (const path of paths) add("path", path.path === null ? null : { path: path.path, directory: true }, path.offset, modulePath, inlineBase);
            if (paths.length > 1) throw new Error("Unsupported Rust compile ambiguous inline module mount");
            const mount = paths[0]?.path, name = rustIdentifierSymbol(tokens[visibility.next + 1])!;
            if (mount !== undefined && mount !== null && (mount.startsWith("/") || /^[A-Za-z]:/u.test(mount) || mount.includes("\\"))) throw new Error("Unsupported Rust compile nonportable inline module mount");
            const childBase = inlineBase !== undefined ? `${inlineBase}/${mount ?? name}` : modulePath.length === 0 && mount !== undefined && mount !== null ? mount || "." : undefined;
            visit(boundary + 1, close, [...modulePath, name], childBase);
            index = close;
          } else {
            for (const path of paths) add("path", path.path === null ? null : { path: path.path }, path.offset, modulePath, inlineBase);
            index = boundary;
          }
          continue;
        }
      }
      if (attributes.next > index) { index = attributes.next - 1; continue; }
      const current = tokens[index]!;
      const compileMacro = rustIdentifierSymbol(current);
      if (["include", "include_str", "include_bytes"].includes(compileMacro ?? "") && tokens[index + 1]?.text === "!") {
        const close = pairs.get(index + 2);
        if (close === undefined) throw new Error("Unsupported Rust compile delimiter");
        const end = tokens[close - 1]?.text === "," ? close - 1 : close, input = literal(index + 3, end), template = templates.find((template) => index > template.start && index < template.end);
        if (compileMacro !== "include" && template?.supported && template.scope && template.bindings.length) for (const binding of template.bindings) {
          const definitionOffset = tokens[template.definition]!.start, invocationOffset = tokens[binding.invocation]!.start;
          add(compileMacro as RustCompileReference["kind"], literal(index + 3, end, binding.values), current.start, modulePath, undefined, { kind: "local-macro", macro: template.name, definitionLine: lineAt(definitionOffset), invocationLine: lineAt(invocationOffset), definitionOffset, templateOffset: current.start, invocationOffset, scope: template.scope });
        } else add(compileMacro as RustCompileReference["kind"], input, current.start, modulePath);
        index = close;
        continue;
      }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, modulePath, inlineBase, null); index = close; }
    }
  };
  visit(0, tokens.length, []);
  return references;
}

/** 📝️ Renders one token range in a deterministic compact Rust spelling. */
export function rustTokenText(tokens: readonly RustToken[], start: number, end: number): string {
  return tokens.slice(start, end).map((token) => token.text).join(" ")
    .replace(/\s*::\s*/gu, "::")
    .replace(/\s*([<>(){}\[\],;:.!])\s*/gu, "$1")
    .replace(/\s+/gu, " ")
    .trim();
}

/** 🧩️ Splits a token range only at delimiters outside paired nested syntax. */
export function rustTokenSegments(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, delimiter: string): (readonly [number, number])[] {
  const segments: (readonly [number, number])[] = [];
  let segmentStart = start;
  for (let index = start; index < end; index += 1) {
    const pair = pairs.get(index);
    if (pair !== undefined && pair > index) {
      index = pair;
      continue;
    }
    if (tokens[index]!.text !== delimiter) continue;
    if (segmentStart < index) segments.push([segmentStart, index]);
    segmentStart = index + 1;
  }
  if (segmentStart < end) segments.push([segmentStart, end]);
  return segments;
}

/** 🏷️ Reads consecutive outer attributes attached to one Rust item. */
export function rustAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): RustAttributes {
  const ranges: (readonly [number, number])[] = [];
  let index = start;
  while (index + 1 < end && tokens[index]!.text === "#" && tokens[index + 1]!.text === "[") {
    const close = pairs.get(index + 1);
    if (close === undefined || close >= end) break;
    ranges.push([index + 2, close]);
    index = close + 1;
  }
  return { ranges, next: index };
}

/** 👁️ Reads Rust item visibility without flattening restricted `pub(...)` scopes. */
export function rustVisibility(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number): RustVisibility {
  if (tokens[start]?.text !== "pub") return { value: "private", next: start };
  if (tokens[start + 1]?.text !== "(") return { value: "pub", next: start + 1 };
  const close = pairs.get(start + 1);
  if (close === undefined) return { value: "pub", next: start + 1 };
  return { value: `pub(${rustTokenText(tokens, start + 2, close)})`, next: close + 1 };
}

/** 🧭️ Extracts only direct or cfg_attr module path metadata, retaining every authored configuration. */
export function rustPathAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: RustAttributes): readonly Readonly<{ path: string | null; offset: number }>[] {
  const paths: { path: string | null; offset: number }[] = [];
  const visit = (start: number, end: number): void => {
    if (tokens[start]?.text === "path") {
      paths.push({ path: end === start + 3 && tokens[start + 1]?.text === "=" ? rustStringValue(tokens[start + 2]) : null, offset: tokens[start]!.start });
      return;
    }
    if (tokens[start]?.text !== "cfg_attr" || tokens[start + 1]?.text !== "(") return;
    const close = pairs.get(start + 1);
    if (close === undefined || close + 1 !== end) return;
    for (const [a, b] of rustTokenSegments(tokens, pairs, start + 2, close, ",").slice(1)) visit(a, b);
  };
  for (const [start, end] of attributes.ranges) visit(start, end);
  return paths;
}

/** ⏩️ Finds a top-level token while skipping every paired nested group. */
export function rustFindTopLevel(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, wanted: ReadonlySet<string>): number {
  for (let index = start; index < end; index += 1) {
    const pair = pairs.get(index);
    if (pair !== undefined && pair > index) {
      if (wanted.has(tokens[index]!.text)) return index;
      index = pair;
      continue;
    }
    if (wanted.has(tokens[index]!.text)) return index;
  }
  return -1;
}

/** 🏷️ Attribute origin and conditional state. */
export interface RustMetadataAttributeFact { readonly name: string; readonly start: number; readonly end: number; readonly conditional: boolean; }

/** 🧭️ Parses one exact nongeneric Rust path from already-tokenized source. */
export function rustMetadataPath(tokens: readonly RustToken[], start: number, end: number, absolute: boolean): string | null {
  let index = start, prefix = "";
  if (tokens[index]?.text === "::") { prefix = "::"; index += 1; }
  else if (absolute) return null;
  const parts: string[] = [];
  while (tokens[index]?.kind === "identifier") {
    parts.push(tokens[index]!.text);
    index += 1;
    if (tokens[index]?.text !== "::") break;
    index += 1;
    if (tokens[index]?.kind !== "identifier") return null;
  }
  return parts.length > 0 && index === end ? `${prefix}${parts.join("::")}` : null;
}

/** 🧭️ Reads one attribute name and its optional parenthesized argument range. */
export function rustMetadataAttributeHead(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): { readonly name: string; readonly arguments: readonly [number, number] | null } | null {
  const name = rustMetadataPath(tokens, start, end, false);
  if (name) return { name, arguments: null };
  let index = start;
  const parts: string[] = [];
  while (tokens[index]?.kind === "identifier") {
    parts.push(tokens[index]!.text);
    index += 1;
    if (tokens[index]?.text !== "::") break;
    index += 1;
  }
  if (parts.length > 0 && tokens[index]?.text === "=" && index + 1 < end) return { name: parts.join("::"), arguments: null };
  if (parts.length === 0 || tokens[index]?.text !== "(") return null;
  const close = pairs.get(index);
  if (close === undefined || close !== end - 1) return null;
  return { name: parts.join("::"), arguments: [index + 1, close] };
}

/** 🧭️ Flattens direct and `cfg_attr` metadata while retaining conditional evidence. */
export function rustMetadataAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: RustAttributes): { readonly items: readonly RustMetadataAttributeFact[]; readonly conditional: boolean } {
  const items: RustMetadataAttributeFact[] = [];
  let conditional = false;
  const add = (start: number, end: number, inheritedConditional: boolean): void => {
    const head = rustMetadataAttributeHead(tokens, pairs, start, end);
    if (!head) return;
    const name = head.name;
    if (name === "cfg") { conditional = true; return; }
    if (name === "cfg_attr") {
      conditional = true;
      if (!head.arguments) return;
      const segments = rustTokenSegments(tokens, pairs, head.arguments[0], head.arguments[1], ",");
      for (const [attributeStart, attributeEnd] of segments.slice(1)) add(attributeStart, attributeEnd, true);
      return;
    }
    items.push({ name, start, end, conditional: inheritedConditional });
  };
  for (const [start, end] of attributes.ranges) add(start, end, false);
  return { items, conditional };
}

/** 🧭️ Extracts exact derive paths only from a complete `derive(...)` metadata item. */
export function rustMetadataDerives(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: readonly RustMetadataAttributeFact[]): readonly string[] {
  const derives: string[] = [];
  for (const attribute of attributes.filter((item) => item.name === "derive")) {
    const head = rustMetadataAttributeHead(tokens, pairs, attribute.start, attribute.end);
    if (!head?.arguments) continue;
    const paths = rustTokenSegments(tokens, pairs, head.arguments[0], head.arguments[1], ",").map(([start, end]) => rustMetadataPath(tokens, start, end, false));
    if (paths.length > 0 && paths.every((path): path is string => path !== null)) derives.push(...paths);
  }
  return derives;
}
