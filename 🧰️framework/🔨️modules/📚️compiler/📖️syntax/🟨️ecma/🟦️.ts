import type { EcmaSourceSpan, EcmaToken, EcmaPattern, EcmaExpression, EcmaStatement } from "./🧬️schema/🟦️.ts";
export type { EcmaSourceSpan, EcmaToken, EcmaPattern, EcmaExpression, EcmaStatement } from "./🧬️schema/🟦️.ts";

/** 🧵️ Decodes strict ECMA quoted literals without executing source. */
export function ecmaStringValue(token: EcmaToken | undefined, checkCancellation?: () => void): string | null {
  if (token?.kind !== "string") return null;
  const text = token.text, quote = text[0], limit = text.length - 1;
  if (limit < 1 || !["'", '"'].includes(quote!) || text[limit] !== quote) return null;
  const simple: Readonly<Record<string, string>> = { n: "\n", r: "\r", t: "\t", b: "\b", f: "\f", v: "\v", "0": "\0" };
  let value = "", checkpoint = 1024;
  for (let index = 1; index < limit; index++) {
    if (index >= checkpoint) { checkCancellation?.(); checkpoint = index + 1024; }
    const char = text[index]!;
    if (char === quote || char === "\n" || char === "\r") return null;
    if (char !== "\\") { value += char; continue; }
    const next = text[++index];
    if (index >= limit) return null;
    if (next === "\n" || next === "\u2028" || next === "\u2029") continue;
    if (next === "\r") { if (text[index + 1] === "\n") index++; continue; }
    if (/\d/u.test(next!)) {
      if (next !== "0" || /\d/u.test(text[index + 1] ?? "")) return null;
      value += "\0"; continue;
    }
    if (Object.hasOwn(simple, next!)) { value += simple[next!]!; continue; }
    if (next === "x" || next === "u") {
      if (next === "u" && text[index + 1] === "{") {
        const end = text.indexOf("}", index + 2), hex = text.slice(index + 2, end);
        if (end < 0 || end >= limit || !/^[0-9a-f]{1,6}$/iu.test(hex)) return null;
        const point = parseInt(hex, 16);
        if (point > 0x10ffff) return null;
        value += String.fromCodePoint(point); index = end; continue;
      }
      const width = next === "x" ? 2 : 4, hex = text.slice(index + 1, index + 1 + width);
      if (index + width >= limit || hex.length !== width || !/^[0-9a-f]+$/iu.test(hex)) return null;
      value += String.fromCharCode(parseInt(hex, 16)); index += width; continue;
    }
    value += next;
  }
  return value;
}

/** 🔤️ Retains original Unicode identifier and escape spans. */
function ecmaIdentifierEnd(source: string, start: number, checkpoint?: (index: number) => void): number {
  let index = start, first = true;
  while (index < source.length) {
    checkpoint?.(index);
    let value = String.fromCodePoint(source.codePointAt(index)!), width = value.length;
    if (value === "\\" && source[index + 1] === "u") {
      const escaped = source.slice(index).match(/^\\u(?:\{([0-9a-f]{1,6})\}|([0-9a-f]{4}))/iu);
      if (!escaped) break;
      const point = parseInt(escaped[1] ?? escaped[2]!, 16);
      if (point > 0x10ffff) break;
      value = String.fromCodePoint(point); width = escaped[0].length;
    }
    if (!(first ? /[$_\p{ID_Start}]/u : /[$_\u200c\u200d\p{ID_Continue}]/u).test(value)) break;
    index += width; first = false;
  }
  return index;
}

/** 🧱️ Tokenizes source with original UTF16 ranges and independently bounded template expressions. */
export function ecmaTokens(content: string, offset = 0, checkCancellation?: () => void): readonly EcmaToken[] {
  if (!Number.isSafeInteger(offset) || offset < 0 || !Number.isSafeInteger(offset + content.length)) throw Error("Invalid ECMA source offset");
  const operators = [">>>=", "**=", "===", "!==", "??=", "&&=", "||=", "<<=", ">>=", ">>>", "...", "=>", "?.", "??", "&&", "||", "<=", ">=", "==", "!=", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "**", "<<", ">>"];
  const control = new Set(["if", "while", "for", "with", "switch", "catch"]);
  const prefix = new Set(["return", "throw", "case", "yield", "await", "delete", "void", "typeof", "new", "in", "instanceof", "of", "else", "do"]);
  let nextCheckpoint = 0;
  const checkpoint = (index: number): void => { if (index >= nextCheckpoint) { checkCancellation?.(); nextCheckpoint = index + 1024; } };
  const scan = (initial: number, interpolation: boolean): { tokens: EcmaToken[]; index: number; closed: boolean; valid: boolean } => {
    const tokens: EcmaToken[] = [], parens: boolean[] = [], braces: boolean[] = [];
    let index = initial, regexAllowed = true, closedParenRegex = false;
    const fail = (start: number) => ({ tokens: [...tokens, { kind: "invalid" as const, text: content.slice(start), start: offset + start, end: offset + content.length }], index: content.length, closed: false, valid: false });
    const emit = (kind: EcmaToken["kind"], start: number, expressions?: readonly EcmaSourceSpan[]): void => {
      const text = content.slice(start, index), previous = tokens.at(-1)?.text;
      tokens.push({ kind, text, start: offset + start, end: offset + index, ...(expressions === undefined ? {} : { expressions }) });
      if (kind === "identifier") regexAllowed = prefix.has(text);
      else if (kind !== "punctuation") regexAllowed = false;
      else if (text === "(") {
        const functionAt = tokens.slice(Math.max(0, tokens.length - 5), -1).findLastIndex(token => token.text === "function");
        const head = functionAt < 0 ? -1 : Math.max(0, tokens.length - 5) + functionAt;
        const tail = head < 0 ? [] : tokens.slice(head + 1, -1);
        const before = tokens[head - (tokens[head - 1]?.text === "async" ? 2 : 1)]?.text;
        const declaration = head >= 0 && tail.filter(token => token.text !== "*").length <= 1 && tail.every(token => token.text === "*" || token.kind === "identifier") && (before === undefined || [";", "{", "}", "export", "default"].includes(before));
        parens.push(control.has(previous ?? "") || previous === "await" && tokens.at(-3)?.text === "for" || declaration); regexAllowed = true;
      }
      else if (text === ")") regexAllowed = closedParenRegex = parens.pop() ?? false;
      else if (text === "{") { braces.push(previous === undefined || previous === ")" && closedParenRegex || [";", "else", "try", "finally", "do"].includes(previous)); regexAllowed = true; }
      else if (text === "}") regexAllowed = braces.pop() ?? false;
      else regexAllowed = !["]", ".", "?.", "++", "--"].includes(text);
    };
    while (index < content.length) {
      checkCancellation?.();
      const start = index, char = content[index]!, next = content[index + 1];
      if (/\s/u.test(char)) { index++; continue; }
      if (index === 0 && char === "#" && next === "!" || char === "/" && next === "/") {
        index += 2;
        while (index < content.length && !/[\n\r\u2028\u2029]/u.test(content[index]!)) { checkpoint(index); index++; }
        continue;
      }
      if (char === "/" && next === "*") {
        index += 2;
        while (index < content.length && !content.startsWith("*/", index)) { checkpoint(index); index++; }
        if (index >= content.length) return fail(start);
        index += 2; continue;
      }
      if (interpolation && char === "}" && braces.length === 0) return { tokens, index, closed: true, valid: true };
      if (char === "'" || char === '"') {
        const quote = char; index++;
        let closed = false;
        while (index < content.length) {
          checkpoint(index);
          const value = content[index++]!;
          if (value === quote) { closed = true; break; }
          if (value === "\n" || value === "\r") return fail(start);
          if (value === "\\") { if (content[index] === "\r" && content[index + 1] === "\n") index++; index++; }
        }
        const token: EcmaToken = { kind: "string", text: content.slice(start, index), start: offset + start, end: offset + index };
        if (!closed || ecmaStringValue(token, checkCancellation) === null) return fail(start);
        emit("string", start); continue;
      }
      if (char === "`") {
        const expressions: EcmaSourceSpan[] = []; index++;
        let closed = false;
        while (index < content.length) {
          checkpoint(index);
          const value = content[index]!;
          if (value === "`") { index++; closed = true; break; }
          if (value === "\\") { index += content[index + 1] === "\r" && content[index + 2] === "\n" ? 3 : 2; continue; }
          if (value === "$" && content[index + 1] === "{") {
            const expressionStart = index + 2, expression = scan(expressionStart, true);
            if (!expression.valid || !expression.closed) return fail(start);
            expressions.push({ text: content.slice(expressionStart, expression.index), start: offset + expressionStart, end: offset + expression.index });
            index = expression.index + 1; continue;
          }
          index++;
        }
        if (!closed) return fail(start);
        emit("template", start, expressions); continue;
      }
      if (char === "/" && next !== "=" && regexAllowed) {
        index++; let bracket = false, closed = false;
        while (index < content.length) {
          checkpoint(index);
          const value = content[index++]!;
          if (/[\n\r\u2028\u2029]/u.test(value)) return fail(start);
          if (value === "\\") { index++; continue; }
          if (value === "[") bracket = true;
          else if (value === "]") bracket = false;
          else if (value === "/" && !bracket) { closed = true; break; }
        }
        if (!closed) return fail(start);
        while (index < content.length && /[\p{ID_Continue}$]/u.test(String.fromCodePoint(content.codePointAt(index)!))) index += String.fromCodePoint(content.codePointAt(index)!).length;
        emit("regex", start); continue;
      }
      const identifierEnd = ecmaIdentifierEnd(content, index, checkpoint);
      if (identifierEnd > index) { index = identifierEnd; emit("identifier", start); continue; }
      if (char === "\\") return fail(start);
      if (/\d/u.test(char) || char === "." && /\d/u.test(next ?? "")) {
        let valid = true, integral = true;
        const digits = (pattern: RegExp): number => {
          let count = 0;
          while (index < content.length && (pattern.test(content[index]!) || content[index] === "_")) {
            checkpoint(index);
            if (content[index] === "_" && (!count || !pattern.test(content[index - 1]!) || !pattern.test(content[index + 1] ?? ""))) valid = false;
            else if (content[index] !== "_") count++;
            index++;
          }
          return count;
        };
        if (char === "0" && /[xbo]/iu.test(next ?? "")) {
          index += 2;
          const radix = next!.toLowerCase(), pattern = radix === "x" ? /[\da-f]/iu : radix === "b" ? /[01]/u : /[0-7]/u;
          valid = digits(pattern) > 0 && valid;
        } else {
          const count = digits(/\d/u);
          if (char === "0" && /\d/u.test(next ?? "")) valid = false;
          if (content[index] === ".") { integral = false; index++; if (!digits(/\d/u) && !count) valid = false; }
          if (/[eE]/u.test(content[index] ?? "")) { integral = false; index++; if (/[+-]/u.test(content[index] ?? "")) index++; valid = digits(/\d/u) > 0 && valid; }
        }
        if (content[index] === "n") { valid &&= integral; index++; }
        if (!valid || content[index] === "\\" || content[index] === "_" || content[index] && /[$\p{ID_Continue}]/u.test(String.fromCodePoint(content.codePointAt(index)!))) return fail(start);
        emit("number", start); continue;
      }
      const operator = operators.find(value => content.startsWith(value, index));
      index += operator?.length ?? String.fromCodePoint(content.codePointAt(index)!).length;
      emit("punctuation", start);
    }
    return { tokens, index, closed: !interpolation, valid: !interpolation };
  };
  const result = scan(0, false);
  return [...result.tokens, { kind: "eof", text: "", start: offset + content.length, end: offset + content.length }];
}

/** 🆔️ Decodes an original identifier spelling without losing its source range. */
export function ecmaIdentifierValue(token: EcmaToken | undefined, checkCancellation?: () => void): string | null {
  if (token?.kind !== "identifier" || ecmaIdentifierEnd(token.text, 0, index => { if (index % 1024 === 0) checkCancellation?.(); }) !== token.text.length || !token.text.length) return null;
  let value = "", index = 0, checkpoint = 0;
  while (index < token.text.length) {
    if (index >= checkpoint) { checkCancellation?.(); checkpoint = index + 1024; }
    if (token.text[index] !== "\\") { const point = String.fromCodePoint(token.text.codePointAt(index)!); value += point; index += point.length; continue; }
    const escaped = /\\u(?:\{([0-9a-f]{1,6})\}|([0-9a-f]{4}))/iy;
    escaped.lastIndex = index;
    const match = escaped.exec(token.text); if (!match) return null;
    value += String.fromCodePoint(parseInt(match[1] ?? match[2]!, 16)); index = escaped.lastIndex;
  }
  return value;
}

/** 🌳️ Parses closed source syntax into first-party nodes with original UTF16 ranges. */
class EcmaParser {
  private index = 0;
  constructor(private readonly tokens: readonly EcmaToken[], private readonly checkCancellation?: () => void, private readonly source = "", private readonly sourceOffset = 0) {}
  private lineBreak(start: number, end: number): boolean { return /[\n\r\u2028\u2029]/u.test(this.source.slice(start - this.sourceOffset, end - this.sourceOffset)); }
  private node(row: Omit<EcmaExpression, "start" | "end">, start: number, end = this.tokens[Math.max(0, this.index - 1)]?.end ?? start): EcmaExpression { return { ...row, start, end }; }
  private statementNode(row: Omit<EcmaStatement, "start" | "end">, start: number): EcmaStatement { return { ...row, start, end: this.tokens[Math.max(0, this.index - 1)]?.end ?? start }; }
  private peek(offset = 0): EcmaToken { return this.tokens[this.index + offset] ?? this.tokens.at(-1)!; }
  private consume(text: string): boolean { if (this.peek().text !== text) return false; this.take(); return true; }
  private take(): EcmaToken { this.checkCancellation?.(); return this.tokens[this.index++] ?? this.tokens.at(-1)!; }
  private matching(open: number, left: string, right: string): number {
    let depth = 0;
    for (let index = open; index < this.tokens.length; index++) {
      this.checkCancellation?.();
      if (this.tokens[index]!.text === left) depth++;
      else if (this.tokens[index]!.text === right && --depth === 0) return index;
    }
    return -1;
  }
  private chunks(tokens: readonly EcmaToken[]): readonly (readonly EcmaToken[])[] {
    const rows: EcmaToken[][] = [[]];
    let round = 0, square = 0, curly = 0, angle = 0;
    for (const token of tokens) {
      this.checkCancellation?.();
      if (token.text === "(" ) round++; else if (token.text === ")") round--;
      else if (token.text === "[") square++; else if (token.text === "]") square--;
      else if (token.text === "{") curly++; else if (token.text === "}") curly--;
      else if (token.text === "<") angle++; else if (token.text === ">") angle = Math.max(0, angle - 1);
      if (token.text === "," && round === 0 && square === 0 && curly === 0 && angle === 0) rows.push([]);
      else rows.at(-1)!.push(token);
    }
    return rows.filter((row) => row.length > 0);
  }
  private pattern(tokens: readonly EcmaToken[]): EcmaPattern | null {
    let values = [...tokens];
    if (values[0]?.text === "...") values = values.slice(1);
    const assignment = values.findIndex((token) => token.text === "=");
    const defaults = assignment >= 0;
    if (defaults) values = values.slice(0, assignment);
    const range = { start: tokens[0]?.start ?? this.peek().start, end: tokens.at(-1)?.end ?? this.peek().start };
    if (values[0]?.kind === "identifier") return values.length === 1 || values[1]?.text === ":" && values.length > 2 || values[1]?.text === "?" && (values.length === 2 || values[2]?.text === ":" && values.length > 3) ? { ...range, names: [values[0].text], defaults, destructured: false } : null;
    if (!(["{", "["].includes(values[0]?.text ?? "") && ["}", "]"].includes(values.at(-1)?.text ?? ""))) return null;
    const names: string[] = [];
    for (const row of this.chunks(values.slice(1, -1))) {
      const identifiers = row.filter((token) => token.kind === "identifier" && token.text !== "type");
      const name = identifiers.at(-1)?.text;
      if (!name) return null;
      names.push(name);
    }
    const members = this.chunks(values.slice(1, -1));
    const objectBindings = values[0]?.text === "{" && members.every((row) => row.length === 1 && row[0]?.kind === "identifier" || row.length === 3 && row[0]?.kind === "identifier" && row[1]?.text === ":" && row[2]?.kind === "identifier") ? members.map((row) => ({ imported: row[0]!.text, local: row.at(-1)!.text })) : undefined;
    return { ...range, names, defaults: defaults || values.some((token) => token.text === "="), destructured: true, objectBindings };
  }
  private parameters(tokens: readonly EcmaToken[]): readonly EcmaPattern[] | null {
    const rows: EcmaPattern[] = [];
    for (const chunk of this.chunks(tokens)) {
      if (chunk.some((token) => token.text === "@")) return null;
      const pattern = this.pattern(chunk);
      if (!pattern) return null;
      rows.push(pattern);
    }
    return rows;
  }
  private expressionList(end: string): readonly EcmaExpression[] | null {
    const rows: EcmaExpression[] = [];
    if (this.consume(end)) return rows;
    while (this.peek().kind !== "eof") {
      const start = this.peek().start, spread = this.consume("...");
      const expression = this.expression();
      if (!expression) return null;
      rows.push(spread ? this.node({ kind: "spread", object: expression }, start) : expression);
      if (this.consume(end)) return rows;
      if (!this.consume(",")) return null;
      if (this.consume(end)) return rows;
    }
    return null;
  }
  private primary(): EcmaExpression | null {
    const start = this.peek().start, value = this.primaryValue();
    return value ? this.node(value, start) : null;
  }
  private primaryValue(): Omit<EcmaExpression, "start" | "end"> | null {
    const token = this.peek();
    if (token.text === "async" && !this.lineBreak(token.end, this.peek(1).start) && (this.peek(1).text === "(" && this.tokens[this.matching(this.index + 1, "(", ")") + 1]?.text === "=>" || this.peek(1).kind === "identifier" && this.peek(2).text === "=>" || this.peek(1).text === "function")) {
      this.take(); const value = this.primaryValue();
      return value && ["arrow", "function"].includes(value.kind) ? { ...value, async: true } : null;
    }
    if (token.text === "function") {
      this.take(); const name = this.peek().kind === "identifier" ? this.take().text : undefined;
      if (!this.consume("(")) return null;
      const close = this.matching(this.index - 1, "(", ")"); if (close < 0) return null;
      const parameters = this.parameters(this.tokens.slice(this.index, close)); if (!parameters) return null;
      while (this.index <= close) this.take();
      const body = this.block();
      return body ? { kind: "function", name, parameters, body } : null;
    }
    if (token.text === "await" || token.text === "!" || token.text === "+" || token.text === "-" || token.text === "delete" || token.text === "yield") {
      this.take(); const object = this.expression(9); return object ? { kind: "unary", operator: token.text, object } : null;
    }
    if (token.text === "new") {
      this.take(); let callee = this.primary();
      while (callee && this.consume(".")) {
        const property = this.take(); if (property.kind !== "identifier") return null;
        callee = this.node({ kind: "member", object: callee, property: property.text }, callee.start);
      }
      if (!callee || !this.consume("(")) return null;
      const args = this.expressionList(")");
      return args ? { kind: "new", callee, arguments: args } : null;
    }
    if (token.text === "(") {
      const close = this.matching(this.index, "(", ")");
      if (close > this.index && this.tokens[close + 1]?.text === "=>") {
        const parameters = this.parameters(this.tokens.slice(this.index + 1, close));
        if (!parameters) return null;
        while (this.index <= close + 1) this.take();
        const body = this.peek().text === "{" ? this.block() : this.expression();
        return body ? { kind: "arrow", parameters, body } : null;
      }
      this.take(); const expression = this.expression();
      if (!expression || !this.consume(")")) return null;
      return { kind: "parenthesized", object: expression };
    }
    if (token.text === "[") {
      this.take(); const elements = this.expressionList("]");
      return elements ? { kind: "array", elements } : null;
    }
    if (token.text === "{") {
      this.take(); const properties: { key?: EcmaExpression; value: EcmaExpression; computed?: boolean }[] = [];
      if (this.consume("}")) return { kind: "object", properties };
      while (this.peek().kind !== "eof") {
        const start = this.peek().start;
        if (this.consume("...")) {
          const value = this.expression(); if (!value) return null;
          properties.push({ value: this.node({ kind: "spread", object: value }, start) });
        } else {
          let key: EcmaExpression | undefined, computed = false;
          if (this.consume("[")) { computed = true; key = this.expression() ?? undefined; if (!key || !this.consume("]")) return null; }
          else {
            const name = this.take();
            if (!["identifier", "string", "number"].includes(name.kind)) return null;
            key = this.node({ kind: name.kind === "identifier" ? "identifier" : "literal", name: name.kind === "identifier" ? name.text : undefined, value: name.text }, name.start, name.end);
          }
          if (this.consume(":")) {
            const value = this.expression(); if (!value) return null;
            properties.push({ key, value, computed });
          } else if (key.kind === "identifier") properties.push({ key, value: key, computed });
          else return null;
        }
        if (this.consume("}")) return { kind: "object", properties };
        if (!this.consume(",")) return null;
        if (this.consume("}")) return { kind: "object", properties };
      }
      return null;
    }
    if (token.kind === "template") {
      this.take();
      const expressions: EcmaExpression[] = [];
      for (const source of token.expressions ?? []) {
        const parser = new EcmaParser(ecmaSemanticTokens(source.text, source.start, this.checkCancellation), this.checkCancellation, source.text, source.start), expression = parser.expression();
        if (!expression || parser.peek().kind !== "eof") return null;
        expressions.push(expression);
      }
      return { kind: "template", value: token.text, expressions };
    }
    if (["string", "number", "regex"].includes(token.kind) || ["true", "false", "null", "undefined"].includes(token.text)) { this.take(); return { kind: token.kind === "regex" ? "regex" : "literal", value: token.text }; }
    if (token.kind !== "identifier") return null;
    this.take();
    if (this.consume("=>")) {
      const body = this.peek().text === "{" ? this.block() : this.expression();
      return body ? { kind: "arrow", parameters: [{ start: token.start, end: token.end, names: [token.text], defaults: false, destructured: false }], body } : null;
    }
    return { kind: "identifier", name: token.text };
  }
  expression(minimum = 0): EcmaExpression | null {
    const start = this.peek().start, value = this.expressionValue(minimum);
    return value ? this.node(value, start) : null;
  }
  private annotation(): string | null {
    const start = this.peek().start, stack: string[] = [], pairs: Readonly<Record<string, string>> = { "{": "}", "[": "]", "(": ")", "<": ">" };
    let consumed = false;
    while (this.peek().kind !== "eof") {
      const token = this.peek();
      if (!stack.length && [",", ")", ";", "=", "=>", "as", "satisfies", "+", "-", "*", "/", "===", "!==", "&&", "||", "??"].includes(token.text)) break;
      if (pairs[token.text]) stack.push(pairs[token.text]!);
      else if (["}", "]", ")", ">"].includes(token.text)) { if (stack.pop() !== token.text) return null; }
      else if (!["identifier", "string", "number"].includes(token.kind) && ![":", ";", "?", "|", "&", ".", ",", "=>"].includes(token.text)) return null;
      this.take(); consumed = true;
    }
    return consumed && !stack.length ? this.source.slice(start - this.sourceOffset, this.tokens[this.index - 1]!.end - this.sourceOffset) : null;
  }
  private expressionValue(minimum: number): EcmaExpression | null {
    let left = this.primary();
    if (!left) return null;
    while (true) {
      if (this.peek().text === "." || this.peek().text === "?.") {
        const optional = this.take().text === "?.";
        const property = this.take(); if (property.kind !== "identifier") return null;
        left = this.node({ kind: "member", object: left, property: property.text, optional }, left.start); continue;
      }
      if (this.consume("[")) {
        const property = this.expression(); if (!property || !this.consume("]")) return null;
        left = this.node({ kind: "member", object: left, property }, left.start); continue;
      }
      if (this.consume("(")) {
        const args = this.expressionList(")"); if (!args) return null;
        left = this.node({ kind: "call", callee: left, arguments: args }, left.start); continue;
      }
      if (this.consume("!")) { left = this.node({ kind: "nonnull", object: left }, left.start); continue; }
      if (minimum < 8 && ["as", "satisfies"].includes(this.peek().text)) {
        const operator = this.take().text, value = this.annotation(); if (value === null) return null;
        left = this.node({ kind: "assertion", operator, value, object: left }, left.start); continue;
      }
      const precedence: Readonly<Record<string, number>> = { "=": 1, "??=": 1, "+=": 1, "-=": 1, "*=": 1, "/=": 1, "??": 3, "||": 4, "&&": 5, "===": 6, "!==": 6, "==": 6, "!=": 6, "<": 7, "<=": 7, ">": 7, ">=": 7, "+": 8, "-": 8, "*": 9, "/": 9, "%": 9 };
      const operator = this.peek().text, rank = precedence[operator] ?? 0;
      if (rank <= minimum) break;
      this.take(); const right = this.expression(rank - (rank === 1 ? 1 : 0));
      if (!right) return null;
      left = this.node({ kind: rank === 1 ? "assignment" : "binary", operator, left, right }, left.start);
    }
    if (minimum === 0 && this.consume("?")) {
      const whenTrue = this.expression();
      if (!whenTrue || !this.consume(":")) return null;
      const whenFalse = this.expression();
      if (!whenFalse) return null;
      left = this.node({ kind: "conditional", condition: left, whenTrue, whenFalse }, left.start);
    }
    return left;
  }
  private variable(kind: string): Omit<EcmaStatement, "start" | "end"> | null {
    this.take(); const declarations: { pattern: EcmaPattern; initializer: EcmaExpression }[] = [];
    while (true) {
      const start = this.index;
      let round = 0, square = 0, curly = 0;
      while (this.peek().kind !== "eof") {
        const text = this.peek().text;
        if (text === "(" ) round++; else if (text === ")") round--;
        else if (text === "[") square++; else if (text === "]") square--;
        else if (text === "{") curly++; else if (text === "}") curly--;
        if (text === "=" && round === 0 && square === 0 && curly === 0) break;
        this.take();
      }
      if (!this.consume("=")) return null;
      const pattern = this.pattern(this.tokens.slice(start, this.index - 1)), initializer = this.expression();
      if (!pattern || !initializer) return null;
      declarations.push({ pattern, initializer });
      if (!this.consume(",")) break;
    }
    this.consume(";");
    return { kind, declarations };
  }
  private importStatement(): Omit<EcmaStatement, "start" | "end"> | null {
    this.take();
    if (this.peek().kind === "string") { this.take(); this.consume(";"); return { kind: "import", imports: [] }; }
    const rows: { local: string; imported: string; runtime: boolean; module: string }[] = [];
    const clauseType = this.consume("type");
    if (this.peek().kind === "identifier" && this.peek().text !== "from") {
      const local = this.take().text; rows.push({ local, imported: "default", runtime: !clauseType, module: "" }); this.consume(",");
    }
    if (this.consume("*")) {
      if (!this.consume("as") || this.peek().kind !== "identifier") return null;
      const local = this.take().text; rows.push({ local, imported: "*", runtime: !clauseType, module: "" });
    } else if (this.consume("{")) {
      while (!this.consume("}")) {
        const typeOnly = this.consume("type");
        const imported = this.take(); if (imported.kind !== "identifier") return null;
        let local = imported.text;
        if (this.consume("as")) { const alias = this.take(); if (alias.kind !== "identifier") return null; local = alias.text; }
        rows.push({ local, imported: imported.text, runtime: !clauseType && !typeOnly, module: "" });
        if (!this.consume(",") && this.peek().text !== "}") return null;
      }
    }
    if (!this.consume("from") || this.peek().kind !== "string") return null;
    const module = ecmaStringValue(this.take(), this.checkCancellation); if (module === null) return null;
    this.consume(";");
    return { kind: "import", imports: rows.map((row) => ({ ...row, module })) };
  }
  private classStatement(): Omit<EcmaStatement, "start" | "end"> | null {
    this.take(); const name = this.take();
    if (name.kind !== "identifier" || !this.consume("extends")) return null;
    const base = this.expression(10);
    if (!base || !this.consume("{")) return null;
    const methods: EcmaStatement[] = [];
    while (!this.consume("}")) {
      if (this.peek().text === "@" || this.peek().kind === "eof") return null;
      this.consume("async");
      const method = this.take(); if (method.text !== "run" || !this.consume("(")) return null;
      const close = this.matching(this.index - 1, "(", ")"); if (close < 0) return null;
      const parameters = this.parameters(this.tokens.slice(this.index, close)); if (!parameters) return null;
      this.index = close + 1;
      if (this.consume(":")) while (this.peek().text !== "{" && this.peek().kind !== "eof") this.take();
      const body = this.block(); if (!body) return null;
      methods.push(this.statementNode({ kind: "method", name: "run", parameters, body }, method.start));
    }
    return methods.length === 1 ? { kind: "class", name: name.text, base, body: methods } : null;
  }
  private functionStatement(): Omit<EcmaStatement, "start" | "end"> | null {
    this.take(); const name = this.take();
    if (name.kind !== "identifier" || !this.consume("(")) return null;
    const close = this.matching(this.index - 1, "(", ")"); if (close < 0) return null;
    const parameters = this.parameters(this.tokens.slice(this.index, close)); if (!parameters) return null;
    this.index = close + 1;
    if (this.consume(":")) while (this.peek().text !== "{" && this.peek().kind !== "eof") this.take();
    const body = this.block();
    return body ? { kind: "function", name: name.text, parameters, body } : null;
  }
  private ifStatement(): Omit<EcmaStatement, "start" | "end"> | null {
    this.take(); if (!this.consume("(")) return null;
    const expression = this.expression(); if (!expression || !this.consume(")")) return null;
    const then = this.statement(); if (!then) return null;
    const otherwise = this.consume("else") ? this.statement() ?? undefined : undefined;
    return { kind: "if", expression, then, otherwise };
  }
  private forStatement(): Omit<EcmaStatement, "start" | "end"> | null {
    this.take(); if (!this.consume("(")) return null;
    const kind = this.take(); if (kind.text !== "const") return null;
    const start = this.index;
    while (this.peek().text !== "of" && this.peek().kind !== "eof") this.take();
    const initializer = this.pattern(this.tokens.slice(start, this.index));
    if (!initializer || !this.consume("of")) return null;
    const iterable = this.expression(); if (!iterable || !this.consume(")")) return null;
    const statement = this.statement();
    return statement ? { kind: "for", initializer, iterable, statement } : null;
  }
  private block(): readonly EcmaStatement[] | null {
    if (!this.consume("{")) return null;
    const rows: EcmaStatement[] = [];
    while (true) {
      while (this.consume(";")) {}
      if (this.consume("}")) break;
      const row = this.statement(); if (!row) return null;
      rows.push(row);
    }
    return rows;
  }
  private statement(): EcmaStatement | null {
    while (this.consume(";")) {}
    const start = this.peek().start, value = this.statementValue();
    return value ? this.statementNode(value, start) : null;
  }
  private statementValue(): Omit<EcmaStatement, "start" | "end"> | null {
    if (this.peek().kind === "eof" || this.peek().text === "}") return null;
    if (["while", "do", "switch", "try", "catch", "finally", "break", "continue", "with", "debugger", "interface", "type"].includes(this.peek().text)) return null;
    if (this.peek().text === "import" && this.peek(1).text !== "(") return this.importStatement();
    if (this.consume("export")) { const statement = this.statement(); return statement ? { kind: "export", statement } : null; }
    if (this.peek().text === "async" && this.peek(1).text === "function" && !this.lineBreak(this.peek().end, this.peek(1).start)) {
      this.take(); const value = this.functionStatement(); return value ? { ...value, async: true } : null;
    }
    if (this.peek().text === "function") return this.functionStatement();
    if (this.peek().text === "@") return null;
    if (this.peek().text === "class") return this.classStatement();
    if (["const", "let", "var"].includes(this.peek().text)) return this.variable(this.peek().text);
    if (this.peek().text === "if") return this.ifStatement();
    if (this.peek().text === "for") return this.forStatement();
    if (this.peek().text === "{") { const body = this.block(); return body ? { kind: "block", body } : null; }
    if (this.peek().text === "return") {
      const keyword = this.take();
      if (this.consume(";") || this.peek().kind === "eof" || this.peek().text === "}" || this.lineBreak(keyword.end, this.peek().start)) return { kind: "return" };
      const expression = this.expression(); if (!expression) return null;
      this.consume(";"); return { kind: "return", expression };
    }
    if (this.peek().text === "throw") {
      const keyword = this.take();
      if (this.lineBreak(keyword.end, this.peek().start)) return null;
      const expression = this.expression(); if (!expression) return null;
      this.consume(";"); return { kind: "throw", expression };
    }
    const expression = this.expression(); if (!expression) return null;
    this.consume(";"); return { kind: "expression", expression };
  }
  program(): readonly EcmaStatement[] | null {
    const rows: EcmaStatement[] = [];
    while (this.peek().kind !== "eof") {
      while (this.consume(";")) {}
      if (this.peek().kind === "eof") break;
      this.checkCancellation?.();
      const row = this.statement(); if (!row) return null;
      rows.push(row);
    }
    return rows;
  }
}

function ecmaSemanticTokens(content: string, offset = 0, checkCancellation?: () => void): readonly EcmaToken[] {
  return ecmaTokens(content, offset, checkCancellation).map(token => token.kind === "identifier" ? { ...token, text: ecmaIdentifierValue(token, checkCancellation)! } : token);
}

/** 🌲️ Returns a complete closed syntax tree, or unresolved syntax without partial authority. */
export function ecmaProgram(content: string, checkCancellation?: () => void): readonly EcmaStatement[] | null {
  const tokens = ecmaSemanticTokens(content, 0, checkCancellation);
  if (tokens.some(token => token.kind === "invalid")) return null;
  return new EcmaParser(tokens, checkCancellation, content).program();
}
