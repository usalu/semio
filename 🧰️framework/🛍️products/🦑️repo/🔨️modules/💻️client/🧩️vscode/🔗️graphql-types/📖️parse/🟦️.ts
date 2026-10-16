/** 📖️ Owned reader of GraphQL schema documents (SDL) and operation documents, producing the model the type projection reads.
 *
 * The grammar is the subset the served schema `🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql` and the extension operations use:
 * definitions, fields with arguments, lists, non-null, unions, interfaces, enums, inputs, variables and selection sets with inline fragments.
 * Anything else is refused with the position it appears at. See {@link ../📽️projection/🟦️.ts | the projection} and {@link ../🏃️execution/🟦️.ts | the generator commands}.
 */

//#region 🧾️Model
export type TypeRef = { readonly kind: "named"; readonly name: string } | { readonly kind: "list"; readonly of: TypeRef } | { readonly kind: "nonNull"; readonly of: TypeRef };

export type ArgumentDefinition = { readonly name: string; readonly type: TypeRef; readonly hasDefault: boolean };

export type FieldDefinition = { readonly name: string; readonly arguments: readonly ArgumentDefinition[]; readonly type: TypeRef };

export type TypeDefinition =
  | { readonly kind: "scalar"; readonly name: string }
  | { readonly kind: "enum"; readonly name: string; readonly values: readonly string[] }
  | { readonly kind: "union"; readonly name: string; readonly members: readonly string[] }
  | { readonly kind: "object" | "interface"; readonly name: string; readonly interfaces: readonly string[]; readonly fields: readonly FieldDefinition[] }
  | { readonly kind: "input"; readonly name: string; readonly fields: readonly FieldDefinition[] };

export type SchemaModel = { readonly types: ReadonlyMap<string, TypeDefinition>; readonly roots: { readonly query?: string; readonly mutation?: string; readonly subscription?: string } };

export type VariableDefinition = { readonly name: string; readonly type: TypeRef; readonly hasDefault: boolean };

export type Selection =
  | { readonly kind: "field"; readonly name: string; readonly alias?: string; readonly selections?: readonly Selection[] }
  | { readonly kind: "inline"; readonly typeCondition?: string; readonly selections: readonly Selection[] };

export type OperationModel = { readonly kind: "query" | "mutation" | "subscription"; readonly name: string; readonly variables: readonly VariableDefinition[]; readonly selections: readonly Selection[] };
//#endregion 🧾️Model

//#region 🔤️Lexer
type Token = { readonly kind: "name" | "punct" | "value" | "end"; readonly text: string; readonly line: number; readonly column: number };

const PUNCTUATION = new Set(["{", "}", "(", ")", "[", "]", ":", "!", "=", "|", "&", "@", "$", ","]);

function tokenize(source: string): Token[] {
  const tokens: Token[] = [];
  let index = 0, line = 1, column = 1;
  const advance = (count: number): void => {
    for (let step = 0; step < count; step++) {
      if (source[index] === "\n") { line++; column = 1; } else column++;
      index++;
    }
  };
  while (index < source.length) {
    const char = source[index]!;
    if (/\s/u.test(char) || char === "﻿") { advance(1); continue; }
    if (char === "#") { while (index < source.length && source[index] !== "\n") advance(1); continue; }
    const start = { line, column };
    if (source.startsWith('"""', index)) {
      const end = source.indexOf('"""', index + 3);
      if (end < 0) throw new Error(`GraphQL ${start.line}:${start.column}: unterminated block string`);
      tokens.push({ kind: "value", text: source.slice(index, end + 3), ...start }); advance(end + 3 - index); continue;
    }
    if (char === '"') {
      let end = index + 1;
      while (end < source.length && source[end] !== '"') end += source[end] === "\\" ? 2 : 1;
      if (end >= source.length) throw new Error(`GraphQL ${start.line}:${start.column}: unterminated string`);
      tokens.push({ kind: "value", text: source.slice(index, end + 1), ...start }); advance(end + 1 - index); continue;
    }
    if (source.startsWith("...", index)) { tokens.push({ kind: "punct", text: "...", ...start }); advance(3); continue; }
    if (PUNCTUATION.has(char)) { tokens.push({ kind: "punct", text: char, ...start }); advance(1); continue; }
    const name = /^[_A-Za-z][_0-9A-Za-z]*/u.exec(source.slice(index));
    if (name) { tokens.push({ kind: "name", text: name[0], ...start }); advance(name[0].length); continue; }
    const number = /^-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?/u.exec(source.slice(index));
    if (number) { tokens.push({ kind: "value", text: number[0], ...start }); advance(number[0].length); continue; }
    throw new Error(`GraphQL ${start.line}:${start.column}: unexpected character ${JSON.stringify(char)}`);
  }
  tokens.push({ kind: "end", text: "", line, column });
  return tokens;
}

class Cursor {
  private index = 0;
  constructor(private readonly tokens: readonly Token[]) {}
  peek(): Token { return this.tokens[this.index]!; }
  at(text: string): boolean { const token = this.peek(); return token.kind !== "value" && token.text === text; }
  next(): Token { return this.tokens[this.index++]!; }
  eat(text: string): boolean { if (this.at(text)) { this.index++; return true; } return false; }
  fail(message: string): never { const token = this.peek(); throw new Error(`GraphQL ${token.line}:${token.column}: ${message} (found ${token.kind === "end" ? "end of document" : JSON.stringify(token.text)})`); }
  expect(text: string): Token { if (!this.at(text)) this.fail(`expected ${JSON.stringify(text)}`); return this.next(); }
  name(): string { if (this.peek().kind !== "name") this.fail("expected a name"); return this.next().text; }
  done(): boolean { return this.peek().kind === "end"; }
  skipCommas(): void {
    while (this.eat(","));
  }
}
//#endregion 🔤️Lexer

//#region 🧩️Shared
function parseTypeRef(cursor: Cursor): TypeRef {
  let type: TypeRef;
  if (cursor.eat("[")) { type = { kind: "list", of: parseTypeRef(cursor) }; cursor.expect("]"); } else type = { kind: "named", name: cursor.name() };
  return cursor.eat("!") ? { kind: "nonNull", of: type } : type;
}

function skipValue(cursor: Cursor): void {
  const token = cursor.peek();
  if (token.kind === "value") { cursor.next(); return; }
  if (cursor.eat("$")) { cursor.name(); return; }
  if (cursor.eat("[")) { while (!cursor.at("]")) { skipValue(cursor); cursor.skipCommas(); } cursor.expect("]"); return; }
  if (cursor.eat("{")) { while (!cursor.at("}")) { cursor.name(); cursor.expect(":"); skipValue(cursor); cursor.skipCommas(); } cursor.expect("}"); return; }
  if (token.kind === "name") { cursor.next(); return; }
  cursor.fail("expected a value");
}

function skipDescription(cursor: Cursor): void {
  while (cursor.peek().kind === "value" && cursor.peek().text.startsWith('"')) cursor.next();
}

function skipDirectives(cursor: Cursor): void {
  while (cursor.eat("@")) {
    cursor.name();
    if (cursor.eat("(")) { while (!cursor.at(")")) { cursor.name(); cursor.expect(":"); skipValue(cursor); cursor.skipCommas(); } cursor.expect(")"); }
  }
}
//#endregion 🧩️Shared

//#region 📜️Schema
function parseArguments(cursor: Cursor): ArgumentDefinition[] {
  const parsed: ArgumentDefinition[] = [];
  if (!cursor.eat("(")) return parsed;
  while (!cursor.at(")")) {
    skipDescription(cursor);
    const name = cursor.name(); cursor.expect(":");
    const type = parseTypeRef(cursor);
    const hasDefault = cursor.eat("=");
    if (hasDefault) skipValue(cursor);
    skipDirectives(cursor); cursor.skipCommas();
    parsed.push({ name, type, hasDefault });
  }
  cursor.expect(")");
  return parsed;
}

function parseFields(cursor: Cursor, allowArguments: boolean): FieldDefinition[] {
  const fields: FieldDefinition[] = [];
  cursor.expect("{");
  while (!cursor.at("}")) {
    skipDescription(cursor);
    const name = cursor.name();
    const args = allowArguments ? parseArguments(cursor) : [];
    cursor.expect(":");
    const type = parseTypeRef(cursor);
    if (!allowArguments && cursor.eat("=")) skipValue(cursor);
    skipDirectives(cursor); cursor.skipCommas();
    fields.push({ name, arguments: args, type });
  }
  cursor.expect("}");
  return fields;
}

/** 📜️ Reads one schema document. Duplicate definitions and unknown definition keywords are refused. */
export function parseSchema(sdl: string): SchemaModel {
  const cursor = new Cursor(tokenize(sdl));
  const types = new Map<string, TypeDefinition>();
  const roots: { query?: string; mutation?: string; subscription?: string } = {};
  const add = (definition: TypeDefinition): void => {
    if (types.has(definition.name)) throw new Error(`GraphQL schema: duplicate definition of ${definition.name}`);
    types.set(definition.name, definition);
  };
  while (!cursor.done()) {
    skipDescription(cursor);
    const keyword = cursor.name();
    if (keyword === "schema") {
      cursor.expect("{");
      while (!cursor.at("}")) { const operation = cursor.name() as "query" | "mutation" | "subscription"; cursor.expect(":"); roots[operation] = cursor.name(); cursor.skipCommas(); }
      cursor.expect("}");
    } else if (keyword === "scalar") add({ kind: "scalar", name: cursor.name() });
    else if (keyword === "enum") {
      const name = cursor.name(); const values: string[] = [];
      cursor.expect("{");
      while (!cursor.at("}")) { skipDescription(cursor); values.push(cursor.name()); skipDirectives(cursor); cursor.skipCommas(); }
      cursor.expect("}"); add({ kind: "enum", name, values });
    } else if (keyword === "union") {
      const name = cursor.name(); const members: string[] = [];
      cursor.expect("="); cursor.eat("|");
      members.push(cursor.name());
      while (cursor.eat("|")) members.push(cursor.name());
      add({ kind: "union", name, members });
    } else if (keyword === "type" || keyword === "interface") {
      const name = cursor.name(); const interfaces: string[] = [];
      if (cursor.peek().text === "implements") { cursor.next(); cursor.eat("&"); interfaces.push(cursor.name()); while (cursor.eat("&")) interfaces.push(cursor.name()); }
      add({ kind: keyword === "type" ? "object" : "interface", name, interfaces, fields: parseFields(cursor, true) });
    } else if (keyword === "input") add({ kind: "input", name: cursor.name(), fields: parseFields(cursor, false) });
    else cursor.fail(`unsupported schema definition ${JSON.stringify(keyword)}`);
  }
  return { types, roots: { query: "Query", mutation: "Mutation", ...roots } };
}
//#endregion 📜️Schema

//#region 🕸️Operation
function parseSelections(cursor: Cursor): Selection[] {
  const selections: Selection[] = [];
  cursor.expect("{");
  while (!cursor.at("}")) {
    if (cursor.eat("...")) {
      if (cursor.peek().text !== "on" && cursor.peek().kind === "name") cursor.fail("fragment spreads are not supported; inline the fields");
      let typeCondition: string | undefined;
      if (cursor.peek().text === "on") { cursor.next(); typeCondition = cursor.name(); }
      skipDirectives(cursor);
      selections.push({ kind: "inline", typeCondition, selections: parseSelections(cursor) });
    } else {
      let name = cursor.name(), alias: string | undefined;
      if (cursor.eat(":")) { alias = name; name = cursor.name(); }
      if (cursor.eat("(")) { while (!cursor.at(")")) { cursor.name(); cursor.expect(":"); skipValue(cursor); cursor.skipCommas(); } cursor.expect(")"); }
      skipDirectives(cursor);
      selections.push({ kind: "field", name, alias, selections: cursor.at("{") ? parseSelections(cursor) : undefined });
    }
    cursor.skipCommas();
  }
  cursor.expect("}");
  return selections;
}

/** 🕸️ Reads one operation document that holds exactly one named query, mutation or subscription. */
export function parseOperation(source: string): OperationModel {
  const cursor = new Cursor(tokenize(source));
  const keyword = cursor.name();
  if (keyword !== "query" && keyword !== "mutation" && keyword !== "subscription") return cursor.fail("expected query, mutation or subscription");
  const name = cursor.name();
  const variables: VariableDefinition[] = [];
  if (cursor.eat("(")) {
    while (!cursor.at(")")) {
      cursor.expect("$");
      const variable = cursor.name(); cursor.expect(":");
      const type = parseTypeRef(cursor);
      const hasDefault = cursor.eat("=");
      if (hasDefault) skipValue(cursor);
      skipDirectives(cursor); cursor.skipCommas();
      variables.push({ name: variable, type, hasDefault });
    }
    cursor.expect(")");
  }
  skipDirectives(cursor);
  const selections = parseSelections(cursor);
  if (!cursor.done()) cursor.fail("an operation document holds exactly one operation");
  return { kind: keyword, name, variables, selections };
}
//#endregion 🕸️Operation
