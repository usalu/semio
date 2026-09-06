/** 🔤️ `s.stdio.txt@utf-8` → remodeling snapshot, fidelity `Exact` — a real `.semio` DSL reader.
 *
 *  The txt rendition of a remodeling document IS its DSL text: the exact bytes
 *  `📚️examples/**​/🖼️assets/🗣️.dsl.semio` carries. This reader is spec-driven — every key is the
 *  `kebab-case` rename of the Rust field name, re-derived from `RecordSpec` rather than
 *  transcribed, so a field rename cannot desynchronise it from the JSON reader.
 *
 *  🚧 Declared boundary, exercised against `📚️examples/🎬️demo`: the grammar surface implemented is
 *  the one the snapshot printer emits for this document — the `semio <envelope> v1` preamble,
 *  `key=value` scalars (bare lexemes, quoted text, `#[dsl(unit)]` suffixes such as `5mm` /
 *  `2m` / `1.5m/s`), `name { … }` blocks, `name={ … }` map literals and
 *  `name [col:TYPE …] { … }` tables whose cells are scalars. Two things are deliberately NOT
 *  implemented and refuse loudly rather than guessing: a table cell that is itself a `TABLE` or
 *  `BLOCK` (no committed asset exercises one — `streams.frames` and `streams.source` only ever
 *  appear in a zero-row table), and the `_` positional placeholder lexeme. There is no DSL
 *  *printer* here either: `dsl::print`'s layout rules (block/table selection, column-header
 *  synthesis, unit re-suffixing, per-type number lexemes) live in the framework's `🗣️dsl` module
 *  and have no TypeScript twin anywhere in the repo.
 */

import {
  REMODELING_SNAPSHOT_SPEC,
  kebabOf,
  type FieldSpec,
  type RecordSpec,
  type RemodelingSnapshot,
  type ValueSpec,
} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";

/** 🚫 A DSL refusal carrying the 1-based line it was raised on. */
export class RemodelingDslError extends Error {
  constructor(
    readonly line: number,
    message: string,
  ) {
    super(`line ${line}: ${message}`);
    this.name = "RemodelingDslError";
  }
}

//#region 🔖️Lexer
type TokenKind = "word" | "string" | "open-brace" | "close-brace" | "open-bracket" | "close-bracket" | "equals" | "colon";

interface Token {
  kind: TokenKind;
  text: string;
  line: number;
}

const PUNCTUATION: Record<string, TokenKind> = { "{": "open-brace", "}": "close-brace", "[": "open-bracket", "]": "close-bracket", "=": "equals", ":": "colon" };

/** 🔍 Splits DSL body text into punctuation, bare lexemes and quoted text. */
export function lexDsl(text: string): Token[] {
  const tokens: Token[] = [];
  let line = 1;
  let index = 0;
  while (index < text.length) {
    const character = text[index];
    if (character === "\n") {
      line += 1;
      index += 1;
      continue;
    }
    if (character === " " || character === "\t" || character === "\r") {
      index += 1;
      continue;
    }
    if (character in PUNCTUATION) {
      tokens.push({ kind: PUNCTUATION[character], text: character, line });
      index += 1;
      continue;
    }
    if (character === '"') {
      let value = "";
      index += 1;
      while (index < text.length && text[index] !== '"') {
        if (text[index] === "\\") {
          const escape = text[index + 1];
          value += escape === "n" ? "\n" : escape === "t" ? "\t" : escape === "r" ? "\r" : escape;
          index += 2;
          continue;
        }
        value += text[index];
        index += 1;
      }
      if (index >= text.length) throw new RemodelingDslError(line, "unterminated quoted text");
      index += 1;
      tokens.push({ kind: "string", text: value, line });
      continue;
    }
    let start = index;
    while (index < text.length && !(text[index] in PUNCTUATION) && !/[\s"]/.test(text[index])) index += 1;
    if (index === start) throw new RemodelingDslError(line, `unexpected character ${JSON.stringify(character)}`);
    tokens.push({ kind: "word", text: text.slice(start, index), line });
  }
  return tokens;
}
//#endregion 🔖️Lexer

//#region 🔖️Parser
interface DslTable {
  columns: { name: string; type: string }[];
  rows: Token[][];
  line: number;
}

interface DslNode {
  scalars: Map<string, Token>;
  blocks: Map<string, DslNode>;
  tables: Map<string, DslTable>;
  maps: Map<string, Map<string, DslNode>>;
  line: number;
}

const emptyNode = (line: number): DslNode => ({ scalars: new Map(), blocks: new Map(), tables: new Map(), maps: new Map(), line });

class Cursor {
  index = 0;
  constructor(readonly tokens: Token[]) {}
  peek(offset = 0): Token | undefined {
    return this.tokens[this.index + offset];
  }
  next(): Token {
    const token = this.tokens[this.index];
    if (token === undefined) throw new RemodelingDslError(this.tokens[this.tokens.length - 1]?.line ?? 1, "unexpected end of document");
    this.index += 1;
    return token;
  }
  expect(kind: TokenKind): Token {
    const token = this.next();
    if (token.kind !== kind) throw new RemodelingDslError(token.line, `expected ${kind}, got ${token.kind} ${JSON.stringify(token.text)}`);
    return token;
  }
}

const readTable = (cursor: Cursor, line: number): DslTable => {
  const columns: { name: string; type: string }[] = [];
  cursor.expect("open-bracket");
  while (cursor.peek()?.kind !== "close-bracket") {
    const name = cursor.expect("word").text;
    cursor.expect("colon");
    columns.push({ name, type: cursor.expect("word").text });
  }
  cursor.expect("close-bracket");
  cursor.expect("open-brace");
  const rows: Token[][] = [];
  while (cursor.peek()?.kind !== "close-brace") {
    const row: Token[] = [];
    for (const column of columns) {
      if (column.type === "TABLE" || column.type === "BLOCK") throw new RemodelingDslError(cursor.peek()?.line ?? line, `table column "${column.name}" is a nested ${column.type}, which this reader does not implement`);
      const cell = cursor.next();
      if (cell.kind !== "word" && cell.kind !== "string") throw new RemodelingDslError(cell.line, `expected a scalar cell for column "${column.name}", got ${cell.kind}`);
      if (cell.kind === "word" && cell.text === "_") throw new RemodelingDslError(cell.line, "the positional `_` placeholder is not implemented by this reader");
      row.push(cell);
    }
    rows.push(row);
  }
  cursor.expect("close-brace");
  return { columns, rows, line };
};

/** 🌳 Parses one `{ … }` body into scalars, blocks, tables and map literals. */
function readNode(cursor: Cursor, line: number, terminated: boolean): DslNode {
  const node = emptyNode(line);
  while (cursor.index < cursor.tokens.length) {
    const head = cursor.peek()!;
    if (head.kind === "close-brace") {
      if (!terminated) throw new RemodelingDslError(head.line, "unbalanced closing brace");
      cursor.next();
      return node;
    }
    const key = cursor.expect("word").text;
    const follow = cursor.peek();
    if (follow?.kind === "equals") {
      cursor.next();
      const value = cursor.next();
      if (value.kind === "open-brace") {
        const entries = new Map<string, DslNode>();
        while (cursor.peek()?.kind !== "close-brace") {
          const entryKey = cursor.next();
          if (entryKey.kind !== "word" && entryKey.kind !== "string") throw new RemodelingDslError(entryKey.line, `expected a map key for "${key}", got ${entryKey.kind}`);
          cursor.expect("open-brace");
          entries.set(entryKey.text, readNode(cursor, entryKey.line, true));
        }
        cursor.expect("close-brace");
        node.maps.set(key, entries);
        continue;
      }
      if (value.kind !== "word" && value.kind !== "string") throw new RemodelingDslError(value.line, `expected a value for "${key}", got ${value.kind}`);
      node.scalars.set(key, value);
      continue;
    }
    if (follow?.kind === "open-brace") {
      cursor.next();
      node.blocks.set(key, readNode(cursor, follow.line, true));
      continue;
    }
    if (follow?.kind === "open-bracket") {
      node.tables.set(key, readTable(cursor, follow.line));
      continue;
    }
    throw new RemodelingDslError(head.line, `"${key}" is followed by ${follow?.kind ?? "end of document"}, which starts neither a value, a block nor a table`);
  }
  if (terminated) throw new RemodelingDslError(line, "unterminated block");
  return node;
}
//#endregion 🔖️Parser

//#region 🔖️Binder
const UNIT_SUFFIX = /(mm|m\/s|m|deg|rad|s|ms|px|%)$/;

const scalarNumber = (token: Token): number => {
  const raw = token.text.replace(UNIT_SUFFIX, "");
  const value = Number(raw);
  if (!Number.isFinite(value)) throw new RemodelingDslError(token.line, `expected a number, got ${JSON.stringify(token.text)}`);
  return value;
};

const bindScalar = (token: Token, spec: ValueSpec, key: string): unknown => {
  switch (spec.k) {
    case "text":
      return token.text;
    case "bool":
      if (token.text !== "true" && token.text !== "false") throw new RemodelingDslError(token.line, `expected a boolean for "${key}", got ${JSON.stringify(token.text)}`);
      return token.text === "true";
    case "uint":
    case "int":
      return scalarNumber(token);
    case "f64":
      return scalarNumber(token);
    case "f32":
      return Math.fround(scalarNumber(token));
    case "enum":
      if (!spec.of.includes(token.text)) throw new RemodelingDslError(token.line, `expected one of ${spec.of.join(" | ")} for "${key}", got ${JSON.stringify(token.text)}`);
      return token.text;
    case "tuple": {
      const parts = token.text.split(",").map((part) => Number(part.replace(UNIT_SUFFIX, "")));
      if (parts.length !== spec.len || parts.some((part) => !Number.isFinite(part))) throw new RemodelingDslError(token.line, `expected ${spec.len} comma-separated numbers for "${key}", got ${JSON.stringify(token.text)}`);
      return spec.w === 32 ? parts.map(Math.fround) : parts;
    }
    case "opt":
      return bindScalar(token, spec.of, key);
    default:
      throw new RemodelingDslError(token.line, `"${key}" is a ${spec.k}, which cannot be written as a DSL scalar`);
  }
};

const inner = (spec: ValueSpec): ValueSpec => (spec.k === "opt" ? spec.of : spec);

/** 🧩️ `store::os_io::ArtifactRef`'s compact DSL lexeme, `<artifact-id>!<kind>@<standard>/<subset>`. */
const bindArtifactRef = (token: Token): Record<string, unknown> => {
  const bang = token.text.indexOf("!");
  const at = token.text.indexOf("@", bang + 1);
  const slash = token.text.indexOf("/", at + 1);
  if (bang < 0 || at < 0 || slash < 0) throw new RemodelingDslError(token.line, `expected an artifact reference "<id>!<kind>@<standard>/<subset>", got ${JSON.stringify(token.text)}`);
  return {
    artifactId: token.text.slice(0, bang),
    dialect: { artifactKind: token.text.slice(bang + 1, at), standard: token.text.slice(at + 1, slash), subset: token.text.slice(slash + 1) },
  };
};

const dslKeyOf = (field: FieldSpec): string => field.dslKey ?? kebabOf(field.name);

const bindRow = (row: Token[], table: DslTable, spec: RecordSpec): Record<string, unknown> => {
  const out: Record<string, unknown> = {};
  for (const field of spec.fields) out[camelKey(field.name)] = field.dflt();
  table.columns.forEach((column, index) => {
    const field = spec.fields.find((candidate) => dslKeyOf(candidate) === column.name);
    if (field === undefined) throw new RemodelingDslError(row[index].line, `unknown column "${column.name}" for ${spec.title}`);
    out[camelKey(field.name)] = bindScalar(row[index], field.spec, column.name);
  });
  return out;
};

const camelKey = (snake: string): string => snake.split("_").map((part, index) => (index === 0 ? part : part.charAt(0).toUpperCase() + part.slice(1))).join("");

/** 🧱 Binds one parsed node onto a record spec; an absent key falls to its Rust default. */
export function bindRecord(node: DslNode, spec: RecordSpec): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  const consumed = new Set<string>();
  for (const field of spec.fields) {
    const key = dslKeyOf(field);
    const shape = inner(field.spec);
    consumed.add(key);
    if (shape.k === "rec" && node.blocks.has(key)) {
      out[camelKey(field.name)] = bindRecord(node.blocks.get(key)!, shape.of());
      continue;
    }
    if (shape.k === "rec" && shape.of().title === "ArtifactRef" && node.scalars.has(key)) {
      out[camelKey(field.name)] = bindArtifactRef(node.scalars.get(key)!);
      continue;
    }
    if (shape.k === "list" && node.tables.has(key)) {
      const table = node.tables.get(key)!;
      const item = inner(shape.of);
      if (item.k !== "rec") throw new RemodelingDslError(table.line, `"${key}" is a table of ${item.k}, which this reader does not implement`);
      out[camelKey(field.name)] = table.rows.map((row) => bindRow(row, table, item.of()));
      continue;
    }
    if (shape.k === "map" && node.maps.has(key)) {
      const entries = node.maps.get(key)!;
      const value = inner(shape.of);
      if (value.k !== "rec") throw new RemodelingDslError(node.line, `"${key}" is a map of ${value.k}, which this reader does not implement`);
      const bound: Record<string, unknown> = {};
      for (const [entryKey, entryNode] of [...entries.entries()].sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))) bound[entryKey] = bindRecord(entryNode, value.of());
      out[camelKey(field.name)] = bound;
      continue;
    }
    if (node.scalars.has(key)) {
      out[camelKey(field.name)] = bindScalar(node.scalars.get(key)!, field.spec, key);
      continue;
    }
    out[camelKey(field.name)] = field.dflt();
  }
  for (const key of [...node.scalars.keys(), ...node.blocks.keys(), ...node.tables.keys(), ...node.maps.keys()])
    if (!consumed.has(key)) throw new RemodelingDslError(node.line, `unknown key "${key}" for ${spec.title}`);
  return out;
}
//#endregion 🔖️Binder

//#region 🔖️Entry
export const REMODELING_DSL_ENVELOPE = "semio remodeling.remodeling.dsl v1";

/** 📄️ Parses a `.dsl.semio` remodeling document into a validated `RemodelingSnapshot`. */
export function remodelingSnapshotFromDslText(text: string): RemodelingSnapshot {
  const newline = text.indexOf("\n");
  const preamble = (newline < 0 ? text : text.slice(0, newline)).trim();
  if (preamble !== REMODELING_DSL_ENVELOPE) throw new RemodelingDslError(1, `expected the preamble ${JSON.stringify(REMODELING_DSL_ENVELOPE)}, got ${JSON.stringify(preamble)}`);
  const body = newline < 0 ? "" : text.slice(newline + 1);
  const cursor = new Cursor(lexDsl(body));
  const node = readNode(cursor, 2, false);
  return bindRecord(node, REMODELING_SNAPSHOT_SPEC) as unknown as RemodelingSnapshot;
}
//#endregion 🔖️Entry
