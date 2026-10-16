//#region 🟦️TypeScriptDeclarationFacts
/** 📏️ Half-open UTF-16 coordinates in the unchanged source string. */
export interface TypeScriptDeclarationSpan { readonly start: number; readonly end: number; }
/** 🧩️ Unexpanded declaration structure with source-order member spellings. */
export interface TypeScriptDeclarationStructure { readonly form: "object" | "union" | "reference" | "enum" | "class" | "unresolved"; readonly members: readonly string[]; readonly unresolved: string | null; }
/** 🪪️ One declaration occurrence, without a provider or mutation identity claim. */
export interface TypeScriptDeclarationFact { readonly kind: "type" | "interface" | "enum" | "class" | "variable"; readonly name: string; readonly exported: boolean; readonly modulePath: readonly string[]; readonly span: TypeScriptDeclarationSpan; readonly structure: TypeScriptDeclarationStructure; }
/** 🔗️ One named import or re-export before module resolution. */
export interface TypeScriptDeclarationAliasFact { readonly relation: "import" | "reexport"; readonly typeOnly: boolean; readonly imported: string; readonly local: string; readonly moduleSpecifier: string; readonly modulePath: readonly string[]; readonly span: TypeScriptDeclarationSpan; }
/** 🚧️ Closed syntax-coverage reasons owned by the declaration grammar. */
export type TypeScriptDeclarationDiagnosticCode = "parse-error" | "unresolved-conditional-type" | "unresolved-mapped-type" | "unresolved-computed-property" | "unresolved-jsx" | "unresolved-expression" | "unsupported-function-local" | "unsupported-default-or-namespace-import" | "unsupported-import-equals" | "unsupported-export-star" | "unsupported-binding-pattern" | "unsupported-anonymous-default-class" | "unsupported-module-statement" | "unresolved-object-spread" | "unresolved-heritage" | "unsupported-type-node" | "unsupported-class-member-body" | "unsupported-ambient-module-body" | "unsupported-recovery-suffix";
/** 🩺️ Exact syntax coverage evidence, with compiler-independent coordinates. */
export interface TypeScriptDeclarationDiagnostic { readonly code: TypeScriptDeclarationDiagnosticCode; readonly span: TypeScriptDeclarationSpan; }
/** 🧾️ Pure declaration summaries; complete does not mean resolved types or mutation providers. */
export interface TypeScriptDeclarationFacts { readonly completeness: "complete" | "incomplete"; readonly declarations: readonly TypeScriptDeclarationFact[]; readonly aliases: readonly TypeScriptDeclarationAliasFact[]; readonly diagnostics: readonly TypeScriptDeclarationDiagnostic[]; }
/** 🔤️ Owned tokens keep semantic identifier values separate from physical source spans. */
interface TypeScriptDeclarationToken { readonly kind: "identifier" | "string" | "number" | "template" | "regex" | "jsx" | "punctuation"; readonly text: string; readonly value: string; readonly start: number; readonly end: number; readonly lineBreakBefore: boolean; readonly interpolated?: boolean; }
/** 🚧️ A lexical boundary which cannot be proven from the supported grammar. */
class TypeScriptDeclarationRecovery extends Error { constructor(readonly start: number) { super("TypeScript declaration syntax has an unproven recovery suffix"); } }
/** 🛑️ A proven missing required type at a declaration grammar boundary. */
class TypeScriptDeclarationSyntaxError extends Error { constructor(readonly span: TypeScriptDeclarationSpan) { super("TypeScript declaration syntax has an invalid token boundary"); } }
/** 🔬️ Scans literals atomically and tracks lexical goals independently of physical path analysis. */
class TypeScriptDeclarationScanner {
  readonly tokens: TypeScriptDeclarationToken[] = [];
  readonly pairs = new Map<number, number>();
  cursor: number;
  private goal: "operand" | "operator" | "ambiguous" = "operand";
  private member = false;
  private control = false;
  private declarationBody = false;
  private block = false;
  private statement = true;
  private readonly groups: { index: number; close: string; goal: "operand" | "operator" | "ambiguous"; control: boolean }[] = [];
  constructor(readonly source: string, readonly language: "ts" | "tsx", start = 0) { this.cursor = start; }

  private point(index: number): string { const value = this.source.codePointAt(index); if (value === undefined) throw new TypeScriptDeclarationRecovery(index); return String.fromCodePoint(value); }

  private escape(start: number): { end: number; value: string } {
    const character = this.source[start + 1];
    if (character === undefined) throw new TypeScriptDeclarationRecovery(start);
    if (character === "u" || character === "x") {
      let cursor = start + 2, digits = "";
      if (character === "u" && this.source[cursor] === "{") {
        cursor++;
        while (cursor < this.source.length && /[0-9a-f]/iu.test(this.source.charAt(cursor)) && digits.length < 6) digits += this.source[cursor++];
        if (!digits || this.source[cursor] !== "}") throw new TypeScriptDeclarationRecovery(start);
        cursor++;
      } else {
        const length = character === "u" ? 4 : 2;
        digits = this.source.slice(cursor, cursor + length);
        if (digits.length !== length || !/^[0-9a-f]+$/iu.test(digits)) throw new TypeScriptDeclarationRecovery(start);
        cursor += length;
      }
      const value = Number.parseInt(digits, 16);
      if (value > 0x10ffff) throw new TypeScriptDeclarationRecovery(start);
      return { end: cursor, value: String.fromCodePoint(value) };
    }
    if (character === "\r" || character === "\n" || character === "\u2028" || character === "\u2029") return { end: start + (character === "\r" && this.source[start + 2] === "\n" ? 3 : 2), value: "" };
    if (/[1-9]/u.test(character) || character === "0" && /[0-9]/u.test(this.source[start + 2] ?? "")) throw new TypeScriptDeclarationRecovery(start);
    const values: Readonly<Record<string, string>> = { n: "\n", r: "\r", t: "\t", b: "\b", f: "\f", v: "\v", "0": "\0" };
    return { end: start + 2, value: values[character] ?? character };
  }

  private quoted(start: number): { end: number; value: string } {
    const quote = this.source[start];
    let cursor = start + 1, value = "";
    while (cursor < this.source.length) {
      const character = this.source.charAt(cursor);
      if (character === quote) return { end: cursor + 1, value };
      if (/[\r\n\u2028\u2029]/u.test(character)) throw new TypeScriptDeclarationRecovery(start);
      if (character === "\\") { const escaped = this.escape(cursor); cursor = escaped.end; value += escaped.value; }
      else value += this.source[cursor++];
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private embedded(start: number): number {
    const scanner = new TypeScriptDeclarationScanner(this.source, this.language, start);
    scanner.scan(true);
    return scanner.cursor;
  }

  private template(start: number): { end: number; interpolated: boolean } {
    let cursor = start + 1, interpolated = false;
    while (cursor < this.source.length) {
      if (this.source[cursor] === "\u0060") return { end: cursor + 1, interpolated };
      if (this.source[cursor] === "\\") { cursor = this.escape(cursor).end; continue; }
      if (this.source[cursor] === "$" && this.source[cursor + 1] === "{") { interpolated = true; cursor = this.embedded(cursor + 2); continue; }
      cursor++;
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private regex(start: number): number {
    let cursor = start + 1, characterClass = false;
    while (cursor < this.source.length) {
      const character = this.source.charAt(cursor);
      if (/[\r\n\u2028\u2029]/u.test(character)) throw new TypeScriptDeclarationRecovery(start);
      if (character === "\\") { if (cursor + 1 >= this.source.length || /[\r\n\u2028\u2029]/u.test(this.source.charAt(cursor + 1))) throw new TypeScriptDeclarationRecovery(start); cursor += 2; continue; }
      if (character === "[") characterClass = true;
      else if (character === "]") characterClass = false;
      else if (character === "/" && !characterClass) {
        cursor++;
        while (cursor < this.source.length && /[$_\p{ID_Continue}]/u.test(this.point(cursor))) cursor += this.point(cursor).length;
        return cursor;
      }
      cursor++;
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private jsx(start: number): number {
    let cursor = start + 1, name = "";
    if (this.source[cursor] !== ">") {
      const first = String.fromCodePoint(this.source.codePointAt(cursor) ?? 0);
      if (!/[$_\p{ID_Start}]/u.test(first)) throw new TypeScriptDeclarationRecovery(start);
      while (cursor < this.source.length) {
        const character = this.point(cursor);
        if (!/[$_\p{ID_Continue}:.-]/u.test(character)) break;
        name += character; cursor += character.length;
      }
      while (cursor < this.source.length && this.source[cursor] !== ">") {
        if (this.source.startsWith("/>", cursor)) return cursor + 2;
        const character = this.source.charAt(cursor);
        if (character === "'" || character === '"') cursor = this.quoted(cursor).end;
        else if (character === "{") cursor = this.embedded(cursor + 1);
        else if (character === "<") throw new TypeScriptDeclarationRecovery(start);
        else cursor++;
      }
    }
    if (this.source[cursor] !== ">") throw new TypeScriptDeclarationRecovery(start);
    cursor++;
    while (cursor < this.source.length) {
      if (this.source.startsWith("</", cursor)) {
        const nameStart = cursor + 2; cursor = nameStart;
        while (cursor < this.source.length && !/[\s>]/u.test(this.source.charAt(cursor))) cursor++;
        if (this.source.slice(nameStart, cursor) !== name) throw new TypeScriptDeclarationRecovery(start);
        while (cursor < this.source.length && /\s/u.test(this.source.charAt(cursor))) cursor++;
        if (this.source[cursor] !== ">") throw new TypeScriptDeclarationRecovery(start);
        return cursor + 1;
      }
      if (this.source[cursor] === "<") cursor = this.jsx(cursor);
      else if (this.source[cursor] === "{") cursor = this.embedded(cursor + 1);
      else cursor++;
    }
    throw new TypeScriptDeclarationRecovery(start);
  }

  private transition(token: TypeScriptDeclarationToken, index: number): void {
    const text = token.text;
    if (token.kind !== "punctuation") {
      if (token.kind === "identifier" && !this.member) {
        if (["if", "while", "for", "switch", "with", "catch"].includes(text)) { this.control = true; this.goal = "operand"; this.statement = false; return; }
        if (["function", "class", "interface", "namespace", "module", "enum"].includes(text)) this.declarationBody = true;
        if (["return", "throw", "case", "yield", "await", "typeof", "void", "delete", "new", "in", "instanceof"].includes(text)) { this.goal = "operand"; this.statement = false; return; }
        if (["else", "try", "finally", "do"].includes(text)) { this.block = true; this.goal = "operand"; this.statement = true; return; }
        if (["export", "declare", "default", "abstract", "async"].includes(text) && this.statement) return;
      }
      this.member = false; this.goal = "operator"; this.statement = false;
      return;
    }
    if (["(", "[", "{"].includes(text)) {
      const control = text === "(" && this.control;
      const isBlock = text === "{" && (this.block || this.declarationBody || this.statement);
      const closingGoal = text === "{" ? isBlock ? "operand" : this.goal === "operand" ? "operator" : "ambiguous" : control ? "operand" : "operator";
      this.groups.push({ index, close: text === "(" ? ")" : text === "[" ? "]" : "}", goal: closingGoal, control });
      this.control = false; this.block = false;
      if (text === "{") this.declarationBody = false;
      this.goal = "operand"; this.statement = isBlock;
      return;
    }
    if ([")", "]", "}"].includes(text)) {
      const open = this.groups.pop();
      if (!open || open.close !== text) throw new TypeScriptDeclarationRecovery(token.start);
      this.pairs.set(open.index, index); this.pairs.set(index, open.index);
      this.goal = open.goal; this.block = open.control || text === ")"; this.statement = text === "}" && open.goal === "operand";
      return;
    }
    if (text === "." || text === "?.") { this.member = true; this.goal = "operator"; return; }
    if (text === "++" || text === "--") return;
    this.goal = "operand"; this.statement = text === ";";
    if (text === "=>") this.block = true;
  }

  scan(stopAtBrace = false): void {
    const number = /(?:0[xX][0-9a-fA-F](?:_?[0-9a-fA-F])*n?|0[bB][01](?:_?[01])*n?|0[oO][0-7](?:_?[0-7])*n?|(?:[0-9](?:_?[0-9])*(?:\.(?:[0-9](?:_?[0-9])*)?)?|\.[0-9](?:_?[0-9])*)(?:[eE][+-]?[0-9](?:_?[0-9])*)?n?)/y;
    const operators = ["===", "!==", ">>>=", "**=", "&&=", "||=", "??=", "=>", "==", "!=", "<=", ">=", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "&&", "||", "??", "?.", "**", "<<=", ">>=", ">>>", "<<", ">>", "..."];
    let previousEnd = this.cursor;
    while (this.cursor < this.source.length) {
      const start = this.cursor, first = this.source.charAt(start);
      if (/\s/u.test(first)) { this.cursor++; continue; }
      if (this.source.startsWith("//", start) || start === 0 && this.source.startsWith("#!", start)) { while (this.cursor < this.source.length && !/[\r\n\u2028\u2029]/u.test(this.source.charAt(this.cursor))) this.cursor++; continue; }
      if (this.source.startsWith("/*", start)) { const close = this.source.indexOf("*/", start + 2); if (close < 0) throw new TypeScriptDeclarationRecovery(start); this.cursor = close + 2; continue; }
      if (stopAtBrace && first === "}" && !this.groups.length) { this.cursor++; return; }
      let kind: TypeScriptDeclarationToken["kind"] = "punctuation", value = "", text = "", interpolated: boolean | undefined;
      if (first === "'" || first === '"') { const string = this.quoted(start); kind = "string"; value = string.value; this.cursor = string.end; }
      else if (first === "\u0060") { const template = this.template(start); kind = "template"; interpolated = template.interpolated; this.cursor = template.end; }
      else if (first === "/" && this.goal !== "operator") { if (this.goal === "ambiguous") throw new TypeScriptDeclarationRecovery(start); kind = "regex"; this.cursor = this.regex(start); }
      else if (first === "<" && this.language === "tsx" && this.goal === "operand") { kind = "jsx"; this.cursor = this.jsx(start); }
      else if (first === "\\" || /[$_\p{ID_Start}]/u.test(this.point(start)) || first === "#" && /[$_\p{ID_Start}]/u.test(String.fromCodePoint(this.source.codePointAt(start + 1) ?? 0))) {
        kind = "identifier";
        if (first === "#") { value = "#"; this.cursor++; }
        let initial = true;
        while (this.cursor < this.source.length) {
          const escaped = this.source[this.cursor] === "\\";
          if (escaped && this.source[this.cursor + 1] !== "u") throw new TypeScriptDeclarationRecovery(this.cursor);
          const entry = escaped ? this.escape(this.cursor) : { end: this.cursor + this.point(this.cursor).length, value: this.point(this.cursor) };
          if (!(initial ? /[$_\p{ID_Start}]/u : /[$_\u200c\u200d\p{ID_Continue}]/u).test(entry.value)) { if (escaped) throw new TypeScriptDeclarationRecovery(this.cursor); break; }
          value += entry.value; this.cursor = entry.end; initial = false;
        }
        text = this.source.slice(start, this.cursor);
      } else if (/[0-9]/u.test(first) || first === "." && /[0-9]/u.test(this.source[start + 1] ?? "")) {
        number.lastIndex = start; const match = number.exec(this.source);
        if (!match?.[0]) throw new TypeScriptDeclarationRecovery(start);
        kind = "number"; text = value = match[0]; this.cursor += text.length;
      } else {
        text = operators.find((operator) => this.source.startsWith(operator, start)) ?? first;
        if (!/^[{}()[\].,;:?~!+\-*/%<>=&|^]$/u.test(text) && !operators.includes(text)) throw new TypeScriptDeclarationRecovery(start);
        value = text; this.cursor += text.length;
      }
      const token: TypeScriptDeclarationToken = { kind, text, value, start, end: this.cursor, lineBreakBefore: /[\r\n\u2028\u2029]/u.test(this.source.slice(previousEnd, start)), ...(interpolated === undefined ? {} : { interpolated }) };
      const pieces = kind === "punctuation" && /^[<>]{1,3}=?$/u.test(text) && text.length > 1 ? [...text].map((part, offset) => ({ ...token, text: part, value: part, start: start + offset, end: start + offset + 1, lineBreakBefore: offset === 0 && token.lineBreakBefore })) : [token];
      for (const piece of pieces) { const index = this.tokens.length; this.tokens.push(piece); this.transition(piece, index); }
      previousEnd = this.cursor;
    }
    const unclosed = this.groups[0];
    if (stopAtBrace || unclosed) throw new TypeScriptDeclarationRecovery(unclosed ? this.tokens[unclosed.index]?.start ?? this.cursor : this.cursor);
  }
}

/** 🌳️ Parses declaration grammar from owned tokens and retains unsupported syntax explicitly. */
class TypeScriptDeclarationParser {
  readonly declarations: TypeScriptDeclarationFact[] = [];
  readonly aliases: TypeScriptDeclarationAliasFact[] = [];
  readonly diagnostics: TypeScriptDeclarationDiagnostic[] = [];
  readonly parseErrors: TypeScriptDeclarationDiagnostic[] = [];
  constructor(readonly source: string, readonly tokens: readonly TypeScriptDeclarationToken[], readonly pairs: ReadonlyMap<number, number>) {}

  private token(index: number): TypeScriptDeclarationToken { const token = this.tokens[index]; if (!token) throw new TypeScriptDeclarationRecovery(this.source.length); return token; }

  private text(index: number): string { return this.tokens[index]?.text ?? ""; }
  private span(start: number, end: number): TypeScriptDeclarationSpan { return { start: this.tokens[start]?.start ?? this.source.length, end: end > start ? this.token(end - 1).end : this.tokens[start]?.start ?? this.source.length }; }
  private raw(start: number, end: number): string { const span = this.span(start, end); return this.source.slice(span.start, span.end); }
  private diagnose(code: TypeScriptDeclarationDiagnosticCode, start: number, end: number): void { this.diagnostics.push({ code, span: this.span(start, end) }); }
  private close(index: number, limit: number): number { const close = this.pairs.get(index); if (close === undefined || close <= index || close >= limit) throw new TypeScriptDeclarationRecovery(this.tokens[index]?.start ?? this.source.length); return close; }

  private angleEnd(start: number, end: number): number {
    let depth = 0;
    for (let index = start; index < end; index++) {
      if (this.text(index) === "<") depth++;
      else if (this.text(index) === ">" && --depth === 0) return index;
      else if (["(", "[", "{"].includes(this.text(index))) index = this.close(index, end);
    }
    throw new TypeScriptDeclarationRecovery(this.token(start).start);
  }

  private top(start: number, end: number, selected: readonly string[], angles = false): number[] {
    const result: number[] = [];
    for (let index = start; index < end; index++) {
      const text = this.text(index);
      if (selected.includes(text)) result.push(index);
      if (["(", "[", "{"].includes(text)) index = this.close(index, end);
      else if (angles && text === "<") index = this.angleEnd(index, end);
    }
    return result;
  }

  private segments(start: number, end: number, delimiter: string, angles = false): [number, number][] {
    const result: [number, number][] = [];
    let first = start;
    for (const index of this.top(start, end, [delimiter], angles)) { if (first < index) result.push([first, index]); first = index + 1; }
    if (first < end) result.push([first, end]);
    return result;
  }

  private declarationStart(index: number): boolean {
    const text = this.text(index);
    if (["export", "import", "const", "let", "var", "interface", "enum", "namespace", "module", "class", "function", "declare"].includes(text)) return true;
    return text === "type" && this.tokens[index + 1]?.kind === "identifier" && ["=", "<"].includes(this.text(index + 2));
  }

  private ending(start: number, end: number, comma = false): number {
    for (let index = start; index < end; index++) {
      const text = this.text(index);
      if (text === ";" || comma && text === ",") return index;
      if (index > start && this.token(index).lineBreakBefore && this.declarationStart(index) && ![".", "?.", "=", "=>", ":", "?", "+", "-", "*", "/", "|", "&", "||", "&&", "??", ",", "<", ">"].includes(this.text(index - 1))) return index;
      if (["(", "[", "{"].includes(text)) index = this.close(index, end);
    }
    return end;
  }

  private invalid(index: number): never { throw new TypeScriptDeclarationRecovery(this.tokens[index]?.start ?? this.source.length); }

  private typeEnd(start: number, end: number, conditional = true): number {
    let index = this.typeAtom(start, end);
    while (index < end && ["|", "&"].includes(this.text(index))) index = this.typeAtom(index + 1, end);
    if (conditional && this.text(index) === "extends") {
      index = this.typeEnd(index + 1, end, false);
      if (this.text(index) !== "?") this.invalid(index);
      index = this.typeEnd(index + 1, end);
      if (this.text(index) !== ":") this.invalid(index);
      index = this.typeEnd(index + 1, end);
    }
    return index;
  }

  private typeAtom(start: number, end: number): number {
    if (start >= end || [";", ",", "=", ">", ")", "]", "}", ":", "?"].includes(this.text(start))) throw new TypeScriptDeclarationSyntaxError(this.span(start, Math.min(start + 1, this.tokens.length)));
    let index = start, text = this.text(index);
    if (["keyof", "readonly", "unique", "typeof", "infer"].includes(text)) {
      if (text === "infer") {
        if (this.tokens[++index]?.kind !== "identifier") this.invalid(index);
        index++;
        if (this.text(index) === "extends") index = this.typeEnd(index + 1, end, false);
      } else index = this.typeAtom(index + 1, end);
    } else if (text === "(") {
      const close = this.close(index, end);
      if (this.text(close + 1) === "=>") {
        this.parameters(index + 1, close);
        index = this.typeEnd(close + 2, end);
      } else {
        if (this.typeEnd(index + 1, close) !== close) this.invalid(index + 1);
        index = close + 1;
      }
    } else if (text === "<") {
      index = this.typeParameters(index, end);
      if (this.text(index) !== "(") this.invalid(index);
      const close = this.close(index, end);
      this.parameters(index + 1, close);
      if (this.text(close + 1) !== "=>") this.invalid(close + 1);
      index = this.typeEnd(close + 2, end);
    } else if (text === "[") {
      const close = this.close(index, end);
      let cursor = index + 1;
      while (cursor < close) {
        if (this.text(cursor) === "...") cursor++;
        if (this.tokens[cursor]?.kind === "identifier" && [":", "?"].includes(this.text(cursor + 1))) {
          cursor++;
          if (this.text(cursor) === "?") cursor++;
          if (this.text(cursor) !== ":") this.invalid(cursor);
          cursor++;
        }
        cursor = this.typeEnd(cursor, close);
        if (this.text(cursor) === "?") cursor++;
        if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
        if (cursor < close) cursor++;
      }
      index = close + 1;
    } else if (text === "{") {
      const close = this.close(index, end);
      if (this.mapped(index, close + 1)) {
        let cursor = index + 1;
        if (["+", "-"].includes(this.text(cursor))) cursor++;
        if (this.text(cursor) === "readonly") cursor++;
        if (this.text(cursor) !== "[") this.invalid(cursor);
        const bindingClose = this.close(cursor, close);
        if (this.tokens[cursor + 1]?.kind !== "identifier" || this.text(cursor + 2) !== "in") this.invalid(cursor + 1);
        cursor = this.typeEnd(cursor + 3, bindingClose);
        if (this.text(cursor) === "as") cursor = this.typeEnd(cursor + 1, bindingClose);
        if (cursor !== bindingClose) this.invalid(cursor);
        cursor++;
        if (["+", "-"].includes(this.text(cursor))) cursor++;
        if (this.text(cursor) === "?") cursor++;
        if (this.text(cursor) !== ":") this.invalid(cursor);
        cursor = this.typeEnd(cursor + 1, close);
        if (this.text(cursor) === ";") cursor++;
        if (cursor !== close) this.invalid(cursor);
      } else this.members(index + 1, close, "type");
      index = close + 1;
    } else if (text === "-" && this.tokens[index + 1]?.kind === "number") index += 2;
    else if (["string", "number", "template"].includes(this.tokens[index]?.kind ?? "")) index++;
    else if (this.tokens[index]?.kind === "identifier" && !["extends", "implements", "in", "as", "is", "const", "let", "var", "export", "import", "return", "throw", "class", "function", "interface", "enum"].includes(text)) index++;
    else this.invalid(index);
    while (index < end) {
      text = this.text(index);
      if (text === "." && this.tokens[index + 1]?.kind === "identifier") index += 2;
      else if (text === "<") {
        const close = this.angleEnd(index, end);
        let cursor = index + 1;
        if (cursor === close) this.invalid(cursor);
        while (cursor < close) {
          cursor = this.typeEnd(cursor, close);
          if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
          if (cursor < close) cursor++;
        }
        index = close + 1;
      } else if (text === "[") {
        const close = this.close(index, end);
        if (index + 1 < close && this.typeEnd(index + 1, close) !== close) this.invalid(index + 1);
        index = close + 1;
      } else break;
    }
    return index;
  }

  private typeParameters(start: number, end: number): number {
    const close = this.angleEnd(start, end);
    let cursor = start + 1;
    if (cursor === close) this.invalid(cursor);
    while (cursor < close) {
      while (["const", "in", "out"].includes(this.text(cursor)) && this.tokens[cursor + 1]?.kind === "identifier") cursor++;
      if (this.tokens[cursor]?.kind !== "identifier") this.invalid(cursor);
      cursor++;
      if (this.text(cursor) === "extends") cursor = this.typeEnd(cursor + 1, close, false);
      if (this.text(cursor) === "=") cursor = this.typeEnd(cursor + 1, close);
      if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
      if (cursor < close) cursor++;
    }
    return close + 1;
  }

  private parameters(start: number, end: number): [number, number][] {
    const annotations: [number, number][] = [];
    let cursor = start;
    while (cursor < end) {
      while (["public", "private", "protected", "readonly", "override"].includes(this.text(cursor)) && this.tokens[cursor + 1]?.kind === "identifier") cursor++;
      if (this.text(cursor) === "...") cursor++;
      if (this.tokens[cursor]?.kind !== "identifier") throw new TypeScriptDeclarationRecovery(this.tokens[cursor]?.start ?? this.source.length);
      cursor++;
      if (this.text(cursor) === "?") cursor++;
      if (this.text(cursor) === ":") { const first = ++cursor; cursor = this.typeEnd(cursor, end); annotations.push([first, cursor]); }
      if (this.text(cursor) === "=") cursor = this.expressionEnd(cursor + 1, end);
      if (cursor < end && this.text(cursor) !== ",") this.invalid(cursor);
      if (cursor < end) cursor++;
    }
    return annotations;
  }

  private expressionEnd(start: number, end: number, minimum = 0): number {
    if (start >= end) this.invalid(start);
    let index = start, text = this.text(index);
    if (["!", "~", "+", "-", "typeof", "void", "delete", "await", "yield", "++", "--"].includes(text)) index = this.expressionEnd(index + 1, end, 15);
    else if (text === "{") { const close = this.close(index, end); this.members(index + 1, close, "object"); index = close + 1; }
    else if (text === "(" || text === "[") {
      const close = this.close(index, end);
      let cursor = index + 1;
      if (text === "(" && cursor === close) this.invalid(cursor);
      while (cursor < close) {
        if (text === "[" && this.text(cursor) === ",") { cursor++; continue; }
        if (this.text(cursor) === "...") cursor++;
        cursor = this.expressionEnd(cursor, close);
        if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
        if (cursor < close) cursor++;
      }
      index = close + 1;
    } else if (["string", "number", "template", "regex", "jsx"].includes(this.tokens[index]?.kind ?? "") || this.tokens[index]?.kind === "identifier" && !["const", "let", "var", "export", "import", "return", "throw", "class", "function", "interface", "enum", "extends"].includes(text)) index++;
    else throw new TypeScriptDeclarationRecovery(this.tokens[index]?.start ?? this.source.length);
    while (index < end) {
      text = this.text(index);
      if ([".", "?."].includes(text)) {
        if (this.tokens[index + 1]?.kind !== "identifier") throw new TypeScriptDeclarationRecovery(this.token(index).start);
        index += 2; continue;
      }
      if (text === "(" || text === "[") {
        const close = this.close(index, end);
        let cursor = index + 1;
        if (text === "[" && cursor === close) this.invalid(cursor);
        while (cursor < close) {
          if (this.text(cursor) === "...") cursor++;
          cursor = this.expressionEnd(cursor, close);
          if (cursor < close && this.text(cursor) !== ",") this.invalid(cursor);
          if (cursor < close) cursor++;
        }
        index = close + 1; continue;
      }
      if (["!", "++", "--"].includes(text) && !this.token(index).lineBreakBefore) { index++; continue; }
      if (["as", "satisfies"].includes(text) && minimum <= 10) {
        index = this.text(index + 1) === "const" && text === "as" ? index + 2 : this.typeEnd(index + 1, end);
        continue;
      }
      if (text === "?" && minimum <= 1) {
        index = this.expressionEnd(index + 1, end);
        if (this.text(index) !== ":") this.invalid(index);
        index = this.expressionEnd(index + 1, end, 1); continue;
      }
      const precedence = text === "=" ? 0 : ["??", "||"].includes(text) ? 2 : text === "&&" ? 3 : text === "|" ? 4 : text === "^" ? 5 : text === "&" ? 6 : ["==", "!=", "===", "!=="].includes(text) ? 7 : ["<", ">", "in", "instanceof"].includes(text) ? 8 : ["+", "-"].includes(text) ? 11 : ["*", "/", "%"].includes(text) ? 12 : text === "**" ? 13 : -1;
      if (precedence < minimum) break;
      index = this.expressionEnd(index + 1, end, precedence + (text === "=" || text === "**" ? 0 : 1));
    }
    return index;
  }

  private members(start: number, end: number, mode: "object" | "type" | "class" | "enum"): { names: string[]; computed: boolean; spreads: [number, number][]; bodies: [number, number][]; annotations: [number, number][] } {
    const names: string[] = [], spreads: [number, number][] = [], bodies: [number, number][] = [], annotations: [number, number][] = [];
    let computed = false;
    for (let index = start; index < end;) {
      if (this.text(index) === ";" && (mode === "type" || mode === "class")) { index++; continue; }
      const first = index;
      let bodyComplete = false;
      if (this.text(index) === "...") {
        if (mode !== "object") this.invalid(index);
        index = this.expressionEnd(index + 1, end);
        spreads.push([first, index]);
      } else if (mode === "class" && this.text(index) === "static" && this.text(index + 1) === "{") {
        const close = this.close(index + 1, end);
        if (close > index + 2) bodies.push([index + 1, close + 1]);
        index = close + 1; bodyComplete = true;
      } else {
        while (mode !== "enum" && ["public", "private", "protected", "readonly", "abstract", "declare", "override", "static", "accessor", "async"].includes(this.text(index)) && ![":", "?", "!", "=", "(", "<", ";", ",", ""].includes(this.text(index + 1))) index++;
        if (mode !== "enum" && ["get", "set"].includes(this.text(index)) && ![":", "?", "=", "(", "<", ";", ",", ""].includes(this.text(index + 1))) index++;
        if (mode !== "enum" && this.text(index) === "*") index++;
        const name = index, nameless = mode === "type" && ["(", "<"].includes(this.text(index));
        if (this.text(index) === "[") {
          const close = this.close(index, end);
          if ((mode === "type" || mode === "class") && this.tokens[index + 1]?.kind === "identifier" && this.text(index + 2) === ":") this.invalid(index);
          computed = true; index = close + 1;
        }
        else if (!nameless && ["identifier", "string", "number"].includes(this.tokens[index]?.kind ?? "")) {
          if (!(mode === "class" && this.text(index) === "constructor")) names.push(this.raw(index, index + 1));
          index++;
        } else if (!nameless) this.invalid(index);
        if (this.text(index) === "?" && mode !== "enum") index++;
        if (this.text(index) === "!" && mode === "class") index++;
        if (this.text(index) === "<" && mode !== "enum") index = this.typeParameters(index, end);
        if (this.text(index) === "(" && mode !== "enum") {
          const close = this.close(index, end);
          annotations.push(...this.parameters(index + 1, close));
          index = close + 1;
          if (this.text(index) === ":") { const firstType = ++index; index = this.typeEnd(index, end); annotations.push([firstType, index]); }
          if (this.text(index) === "{") {
            if (mode === "type") this.invalid(index);
            const close = this.close(index, end);
            if (close > index + 1) {
              if (mode === "class") bodies.push([index, close + 1]);
              else this.diagnose("unsupported-recovery-suffix", first, close + 1);
            }
            index = close + 1; bodyComplete = true;
          } else if (mode === "object") this.invalid(index);
        } else {
          if (nameless) this.invalid(index);
          if (this.text(index) === ":") {
            if (mode === "enum") this.invalid(index);
            const firstType = ++index;
            index = mode === "object" ? this.expressionEnd(index, end) : this.typeEnd(index, end);
            if (mode !== "object") annotations.push([firstType, index]);
          } else if (mode === "object" && (this.tokens[name]?.kind !== "identifier" || name !== first)) this.invalid(index);
          if (this.text(index) === "=") {
            if (mode === "type" || mode === "object") this.invalid(index);
            index = this.expressionEnd(index + 1, end);
          }
        }
      }
      if (index >= end) break;
      const delimiter = this.text(index);
      if (delimiter === "," && mode !== "class" || delimiter === ";" && (mode === "type" || mode === "class")) { index++; continue; }
      if (mode === "class" && bodyComplete || (mode === "type" || mode === "class") && this.token(index).lineBreakBefore) continue;
      this.invalid(index);
    }
    return { names, computed, spreads, bodies, annotations };
  }

  private reference(start: number, end: number): boolean {
    const primitives = ["any", "unknown", "never", "void", "undefined", "null", "string", "number", "boolean", "bigint", "symbol", "object", "intrinsic", "this", "true", "false", "keyof", "typeof", "infer", "readonly", "unique", "new", "abstract"];
    if (this.tokens[start]?.kind !== "identifier" || primitives.includes(this.text(start))) return false;
    let index = start + 1;
    while (index < end && this.text(index) === "." && this.tokens[index + 1]?.kind === "identifier") index += 2;
    if (index < end && this.text(index) === "<") index = this.angleEnd(index, end) + 1;
    return index === end;
  }

  private conditional(start: number, end: number): { extends: number; question: number; colon: number } | null {
    const positions = this.top(start, end, ["extends", "?", ":"], true);
    const extend = positions.find((index) => this.text(index) === "extends");
    const question = extend === undefined ? undefined : positions.find((index) => index > extend && this.text(index) === "?");
    if (extend === undefined || question === undefined) return null;
    let nested = 0;
    for (const index of positions.filter((index) => index > question)) {
      if (this.text(index) === "?") nested++;
      else if (this.text(index) === ":" && nested-- === 0) return { extends: extend, question, colon: index };
    }
    return null;
  }

  private conditionalParts(start: number, end: number, conditional: { extends: number; question: number; colon: number }): readonly [number, number][] { return [[start, conditional.extends], [conditional.extends + 1, conditional.question], [conditional.question + 1, conditional.colon], [conditional.colon + 1, end]]; }

  private mapped(start: number, end: number): boolean {
    if (this.text(start) !== "{" || this.pairs.get(start) !== end - 1) return false;
    let index = start + 1;
    while (["readonly", "+", "-"].includes(this.text(index))) index++;
    if (this.text(index) !== "[") return false;
    const close = this.close(index, end);
    return this.top(index + 1, close, ["in"])[0] !== undefined;
  }

  private structure(start: number, end: number): TypeScriptDeclarationStructure {
    if (start >= end) return { form: "unresolved", members: [], unresolved: "unsupported-type" };
    if (this.conditional(start, end)) return { form: "unresolved", members: [], unresolved: "conditional" };
    const union = this.segments(start, end, "|", true);
    if (union.length > 1) {
      const unresolved = union.some(([first, last]) => {
        if (this.text(first) === "(" && this.pairs.get(first) === last - 1) { first++; last--; }
        return this.conditional(first, last) !== null || this.mapped(first, last);
      });
      return { form: "union", members: union.map(([first, last]) => this.raw(first, last)), unresolved: unresolved ? "conditional-or-mapped-union-member" : null };
    }
    if (this.mapped(start, end)) return { form: "unresolved", members: [], unresolved: "mapped" };
    if (this.text(start) === "{" && this.pairs.get(start) === end - 1) { const members = this.members(start + 1, end - 1, "type"); return { form: "object", members: members.names, unresolved: members.computed ? "computed-property" : null }; }
    if (this.reference(start, end)) return { form: "reference", members: [this.raw(start, end)], unresolved: null };
    return { form: "unresolved", members: [], unresolved: "unsupported-type" };
  }

  private typeDiagnostics(start: number, end: number, summarized = true, unwrap = false): void {
    if (start >= end) return;
    if (unwrap && this.text(start) === "(" && this.pairs.get(start) === end - 1) { this.typeDiagnostics(start + 1, end - 1, summarized, true); return; }
    const conditional = this.conditional(start, end);
    if (conditional) {
      this.diagnose("unresolved-conditional-type", start, end);
      for (const [first, last] of this.conditionalParts(start, end, conditional)) this.typeDiagnostics(first, last, false, true);
      return;
    }
    const union = this.segments(start, end, "|", true);
    if (union.length > 1) { for (const [first, last] of union) this.typeDiagnostics(first, last, summarized, true); return; }
    if (this.mapped(start, end)) {
      this.diagnose("unresolved-mapped-type", start, end);
      const colon = this.top(start + 1, end - 1, [":"])[0];
      if (colon !== undefined) this.typeDiagnostics(colon + 1, this.text(end - 2) === ";" ? end - 2 : end - 1, false, true);
      return;
    }
    if (this.text(start) === "{" && this.pairs.get(start) === end - 1) {
      const members = this.members(start + 1, end - 1, "type");
      if (members.computed) this.diagnose("unresolved-computed-property", start, end);
      for (const [first, last] of members.annotations) this.typeDiagnostics(first, last, false, true);
      return;
    }
    const reference = this.reference(start, end);
    if (reference) {
      const open = this.top(start, end, ["<"])[0];
      if (open !== undefined) for (const [first, last] of this.segments(open + 1, this.angleEnd(open, end), ",", true)) this.typeDiagnostics(first, last, false, true);
      return;
    }
    if (summarized) this.diagnose("unsupported-type-node", start, end);
    for (let index = start; index < end; index++) {
      if (["(", "[", "{"].includes(this.text(index))) { const close = this.close(index, end); this.typeDiagnostics(index + (this.text(index) === "{" ? 0 : 1), close + (this.text(index) === "{" ? 1 : 0), false, true); index = close; }
    }
  }

  private expression(start: number, end: number): TypeScriptDeclarationStructure {
    const assertions = this.top(start, end, ["as", "satisfies"]);
    let first = start, last = assertions.at(-1) ?? end;
    if (this.text(first) === "<") { const close = this.angleEnd(first, last); first = close + 1; }
    if (first >= last) return { form: "unresolved", members: [], unresolved: "initializer:expression" };
    if (this.text(first) === "{" && this.pairs.get(first) === last - 1) {
      const members = this.members(first + 1, last - 1, "object");
      for (const [left, right] of members.spreads) this.diagnose("unresolved-object-spread", left, right);
      if (members.computed && !members.spreads.length) this.diagnose("unresolved-computed-property", start, end);
      return { form: "object", members: members.names, unresolved: members.spreads.length ? "object-spread" : members.computed ? "computed-property" : null };
    }
    const token = this.token(first), kind = last === first + 1 ? token.kind : "punctuation";
    const unresolved = kind === "template" ? token.interpolated ? "initializer:template-interpolation" : "initializer:template-literal" : kind === "string" ? "initializer:string-literal" : kind === "regex" ? "initializer:regex-literal" : kind === "jsx" ? "initializer:jsx" : "initializer:expression";
    if (kind === "jsx") this.diagnose("unresolved-jsx", start, end);
    return { form: "unresolved", members: [], unresolved };
  }

  private importOrExport(start: number, end: number, modulePath: readonly string[]): number {
    const relation = this.text(start) === "import" ? "import" : "reexport", stop = this.ending(start + 1, end), next = stop + (this.text(stop) === ";" ? 1 : 0);
    if (relation === "import" && this.top(start + 1, stop, ["="])[0] !== undefined) { this.diagnose("unsupported-import-equals", start, next); return next; }
    let cursor = start + 1, typeOnly = false;
    if (this.text(cursor) === "type") { typeOnly = true; cursor++; }
    const defaultName = relation === "import" && this.tokens[cursor]?.kind === "identifier" ? cursor : -1;
    if (defaultName >= 0) cursor = this.text(cursor + 1) === "," ? cursor + 2 : stop;
    const close = this.text(cursor) === "{" ? this.close(cursor, stop) : -1, module = close < 0 ? undefined : this.tokens[close + 2];
    if (close < 0 || this.text(close + 1) !== "from" || module?.kind !== "string" || close + 3 !== stop) { this.diagnose(relation === "import" ? "unsupported-default-or-namespace-import" : "unsupported-export-star", start, next); return next; }
    if (defaultName >= 0) this.diagnose("unsupported-default-or-namespace-import", defaultName, defaultName + 1);
    for (const [first, last] of this.segments(cursor + 1, close, ",")) {
      let name = first, elementTypeOnly = false;
      if (this.text(name) === "type" && name + 1 < last && this.text(name + 1) !== "as") { elementTypeOnly = true; name++; }
      const local = this.text(name + 1) === "as" ? name + 2 : name;
      if (!["identifier", "string"].includes(this.tokens[name]?.kind ?? "") || this.tokens[local]?.kind !== "identifier" || local + 1 !== last) { this.diagnose(relation === "import" ? "unsupported-default-or-namespace-import" : "unsupported-export-star", first, last); continue; }
      this.aliases.push({ relation, typeOnly: typeOnly || elementTypeOnly, imported: this.token(name).value, local: this.token(local).value, moduleSpecifier: module.value, modulePath, span: this.span(first, last) });
    }
    return next;
  }

  private variables(start: number, keyword: number, end: number, modulePath: readonly string[], exported: boolean): number {
    let cursor = keyword + 1;
    while (cursor < end) {
      const stop = this.ending(cursor, end, true), name = this.token(cursor), equals = this.top(cursor, stop, ["="], true)[0];
      if (stop <= cursor) throw new TypeScriptDeclarationRecovery(name?.start ?? this.source.length);
      if (name.kind !== "identifier") this.diagnose("unsupported-binding-pattern", cursor, stop);
      else {
        let header = cursor + 1;
        if (this.text(header) === "!") header++;
        if (this.text(header) === ":") header = this.typeEnd(header + 1, equals ?? stop);
        if (header !== (equals ?? stop)) this.invalid(header);
        let shape: TypeScriptDeclarationStructure;
        if (equals === undefined) shape = { form: "unresolved", members: [], unresolved: "initializer:absent" };
        else if (equals + 1 >= stop) { shape = { form: "unresolved", members: [], unresolved: "initializer:absent" }; this.parseErrors.push({ code: "parse-error", span: this.span(stop, Math.min(stop + 1, end)) }); }
        else shape = this.expression(equals + 1, stop);
        const colon = this.top(cursor + 1, equals ?? stop, [":"], true)[0];
        if (colon !== undefined) this.typeDiagnostics(colon + 1, equals ?? stop, false, true);
        this.declarations.push({ kind: "variable", name: name.value, exported, modulePath, span: this.span(cursor, stop), structure: shape });
        if (["initializer:absent", "initializer:expression", "initializer:template-interpolation"].includes(shape.unresolved ?? "")) this.diagnose("unresolved-expression", cursor, stop);
      }
      if (this.text(stop) !== ",") return stop + (this.text(stop) === ";" ? 1 : 0);
      cursor = stop + 1;
    }
    return cursor;
  }

  private bodyOpen(start: number, end: number): number {
    for (let index = start; index < end; index++) {
      if (this.text(index) === "{" || this.text(index) === ";") return index;
      if (["(", "["].includes(this.text(index))) index = this.close(index, end);
      else if (this.text(index) === "<") index = this.angleEnd(index, end);
    }
    return end;
  }

  parse(start = 0, end = this.tokens.length, modulePath: readonly string[] = []): void {
    for (let cursor = start; cursor < end;) {
      if (this.text(cursor) === ";") { cursor++; continue; }
      const first = cursor;
      if (this.text(cursor) === "import" || this.text(cursor) === "export" && (["{", "*"].includes(this.text(cursor + 1)) || this.text(cursor + 1) === "type" && ["{", "*"].includes(this.text(cursor + 2)))) { cursor = this.importOrExport(cursor, end, modulePath); continue; }
      let exported = false;
      while (["export", "default", "declare", "abstract", "async"].includes(this.text(cursor))) { exported ||= this.text(cursor) === "export"; cursor++; }
      let keyword = this.text(cursor);
      if (keyword === "const" && this.text(cursor + 1) === "enum") { cursor++; keyword = "enum"; }
      if (["namespace", "module", "global"].includes(keyword)) {
        const names: string[] = [];
        let index = keyword === "global" ? cursor : cursor + 1;
        if (!["identifier", "string"].includes(this.tokens[index]?.kind ?? "")) throw new TypeScriptDeclarationRecovery(this.token(first).start);
        names.push(this.raw(index, index + 1)); index++;
        while (this.text(index) === "." && this.tokens[index + 1]?.kind === "identifier") { names.push(this.raw(index + 1, index + 2)); index += 2; }
        if (this.text(index) !== "{") { const stop = this.ending(index, end), next = stop + (this.text(stop) === ";" ? 1 : 0); this.diagnose("unsupported-ambient-module-body", first, next); cursor = next; continue; }
        const close = this.close(index, end); this.parse(index + 1, close, [...modulePath, ...names]); cursor = close + 1; continue;
      }
      if (["const", "let", "var"].includes(keyword)) { cursor = this.variables(first, cursor, end, modulePath, exported); continue; }
      if (keyword === "type" && this.tokens[cursor + 1]?.kind === "identifier") {
        const name = this.token(cursor + 1);
        let equals = cursor + 2;
        if (this.text(equals) === "<") equals = this.typeParameters(equals, end);
        if (this.text(equals) !== "=") throw new TypeScriptDeclarationRecovery(this.token(first).start);
        const stop = this.ending(equals + 1, end), next = stop + (this.text(stop) === ";" ? 1 : 0);
        if (equals + 1 < stop) { const consumed = this.typeEnd(equals + 1, stop); if (consumed !== stop) this.invalid(consumed); }
        const shape = this.structure(equals + 1, stop);
        this.declarations.push({ kind: "type", name: name.value, exported, modulePath, span: this.span(first, next), structure: shape });
        if (equals + 1 === stop) this.parseErrors.push({ code: "parse-error", span: this.span(stop, Math.min(stop + 1, end)) });
        else this.typeDiagnostics(equals + 1, stop);
        cursor = next; continue;
      }
      if (["interface", "enum", "class"].includes(keyword)) {
        const name = this.token(cursor + 1), named = name.kind === "identifier" && !["extends", "implements"].includes(name.text), open = this.bodyOpen(cursor + (named ? 2 : 1), end);
        if (open >= end || this.text(open) !== "{") throw new TypeScriptDeclarationRecovery(this.token(first).start);
        const close = this.close(open, end);
        if (!named) { this.diagnose(keyword === "class" ? "unsupported-anonymous-default-class" : "unsupported-module-statement", first, close + 1); cursor = close + 1; continue; }
        let header = cursor + 2;
        if (this.text(header) === "<" && keyword !== "enum") header = this.typeParameters(header, open);
        if (header !== open && !["extends", "implements"].includes(this.text(header))) this.invalid(header);
        if (header !== open && (keyword === "enum" || header + 1 === open)) this.invalid(header);
        const clauses = this.top(cursor + 2, open, ["extends", "implements"], true);
        for (const [index, clause] of clauses.entries()) this.diagnose("unresolved-heritage", clause, clauses[index + 1] ?? open);
        const members = this.members(open + 1, close, keyword === "interface" ? "type" : keyword as "class" | "enum");
        for (const [left, right] of members.bodies) this.diagnose("unsupported-class-member-body", left, right);
        if (members.computed) this.diagnose("unresolved-computed-property", first, close + 1);
        if (keyword !== "enum") for (const [left, right] of members.annotations) this.typeDiagnostics(left, right, false, true);
        const unresolved = members.bodies.length ? "class-member-body" : clauses.length ? "heritage" : members.computed ? "computed-property" : null;
        this.declarations.push({ kind: keyword as "interface" | "enum" | "class", name: name.value, exported, modulePath, span: this.span(first, close + 1), structure: { form: keyword === "interface" ? "object" : keyword as "enum" | "class", members: members.names, unresolved } });
        cursor = close + 1; continue;
      }
      if (keyword === "function") {
        const open = this.bodyOpen(cursor + 1, end), next = this.text(open) === "{" ? this.close(open, end) + 1 : open + (this.text(open) === ";" ? 1 : 0);
        if (next <= first) throw new TypeScriptDeclarationRecovery(this.token(first).start);
        this.diagnose("unsupported-function-local", first, next); cursor = next; continue;
      }
      const stop = this.ending(cursor, end), next = stop + (this.text(stop) === ";" ? 1 : 0);
      if (next <= first) throw new TypeScriptDeclarationRecovery(this.token(first).start);
      this.diagnose("unsupported-module-statement", first, next); cursor = next;
    }
  }

  result(): TypeScriptDeclarationFacts {
    const diagnostics = [...this.diagnostics.sort((left, right) => left.span.start - right.span.start), ...this.parseErrors];
    return { completeness: diagnostics.length ? "incomplete" : "complete", declarations: this.declarations, aliases: this.aliases, diagnostics };
  }
}

/** 🧭️ Inspects exact source syntax without IO, compiler dependencies, evaluation or provider inference. */
export function inspectTypeScriptDeclarationFacts(source: string, language: "ts" | "tsx"): TypeScriptDeclarationFacts {
  if (language !== "ts" && language !== "tsx") throw new TypeError("TypeScript declaration facts require an explicit ts or tsx language");
  if (typeof source !== "string") throw new TypeError("TypeScript declaration facts require a source string");
  const scanner = new TypeScriptDeclarationScanner(source, language);
  let parser: TypeScriptDeclarationParser | undefined;
  try {
    scanner.scan();
    parser = new TypeScriptDeclarationParser(source, scanner.tokens, scanner.pairs);
    parser.parse();
    return parser.result();
  } catch (error) {
    if (!(error instanceof TypeScriptDeclarationRecovery) && !(error instanceof TypeScriptDeclarationSyntaxError) && !(error instanceof RangeError)) throw error;
    if (!parser) parser = new TypeScriptDeclarationParser(source, [], new Map());
    if (error instanceof TypeScriptDeclarationSyntaxError) parser.parseErrors.push({ code: "parse-error", span: error.span });
    else parser.diagnostics.push({ code: "unsupported-recovery-suffix", span: { start: error instanceof TypeScriptDeclarationRecovery ? error.start : 0, end: source.length } });
    return parser.result();
  }
}
//#endregion 🟦️TypeScriptDeclarationFacts
