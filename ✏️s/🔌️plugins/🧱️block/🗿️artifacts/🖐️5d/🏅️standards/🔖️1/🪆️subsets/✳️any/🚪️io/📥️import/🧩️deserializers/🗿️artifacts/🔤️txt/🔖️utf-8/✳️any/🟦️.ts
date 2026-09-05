/** 🚪️ block5d ← txt — TypeScript mirror of the sibling `🦀️.rs` leaf's `from_dsl_text`.
 *
 * A real reader for this subset's own `.semio` DSL snapshot text, driven by the SAME field tables the
 * `🔣️json` export leaf declares — one schema declaration per subset, never two. Scope is exactly the
 * document shape the snapshot grammar emits: a `semio <envelope>.dsl v1` preamble, top-level
 * `key=value` scalars, `name { … }` blocks of `key=value` pairs, and `name [col:TYPE …] { … }`
 * tables whose rows are whitespace-separated `TEXT`/`REF`/`NUM`/`BOOL`/`ANG`/`CRD`/`DIR`/`LIST`/
 * `TABLE` values. It is NOT a general `dsl::parse` port and deliberately throws on anything outside
 * that shape rather than guessing.
 *
 * The matching PRINTER is not ported — `printDsl` has no TypeScript twin yet (see `📓️w3-io.md`).
 */

import type { Block5dSnapshot } from "../../../../../../../🧬️schema/📸️snapshot/🟦️";
import { type BlockJsonField, BLOCK5D_SNAPSHOT_FIELDS } from "../../../../../../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";

/** 📄️ The `.semio` text preamble every `block.block5d` document opens with. */
export const BLOCK_DSL_PREAMBLE = "semio block.block5d.dsl ";

/** 🐍️ `camelCase` json key → the `kebab-case` name the DSL grammar prints for it. The two
 * exceptions are the members whose Rust field name carries a digit-led segment (`radius_2d`,
 * `part_2d`) that `rename_all = "camelCase"` collapses but the DSL keeps hyphenated. */
const DSL_NAME_OVERRIDES: Readonly<Record<string, string>> = { radius2d: "radius-2d", radius3d: "radius-3d", "2d": "part-2d", "3d": "part-3d" };

function dslName(key: string): string {
  return DSL_NAME_OVERRIDES[key] ?? key.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
}

// #region 🪙️Tokenizer
export type Token = { text: string; quoted: boolean };

/** 🪙️ Splits one DSL body line into quoted-string-aware tokens, keeping `[`/`]` as their own. */
function tokenize(line: string): Token[] {
  const tokens: Token[] = [];
  let index = 0;
  while (index < line.length) {
    const character = line[index];
    if (character === " " || character === "\t") {
      index += 1;
    } else if (character === '"') {
      let text = "";
      index += 1;
      while (index < line.length && line[index] !== '"') {
        if (line[index] === "\\") {
          index += 1;
          const escaped = line[index];
          text += escaped === "n" ? "\n" : escaped === "t" ? "\t" : escaped === "r" ? "\r" : escaped;
        } else {
          text += line[index];
        }
        index += 1;
      }
      index += 1;
      tokens.push({ text, quoted: true });
    } else if (character === "[" || character === "]") {
      tokens.push({ text: character, quoted: false });
      index += 1;
    } else {
      let text = "";
      while (index < line.length && !" \t[]".includes(line[index])) {
        text += line[index];
        index += 1;
      }
      tokens.push({ text, quoted: false });
    }
  }
  return tokens;
}
// #endregion 🪙️Tokenizer

// #region 🔢️Scalars
function numbers(text: string): number[] {
  return text.split(",").map((part) => Number(part));
}

/** 🕳️ The DSL's positional `None` marker: an unquoted `_` in a table cell (`TokenKind::Placeholder`).
 * Keyed optionals inside a block are omitted outright instead, never written as `_`. */
export function isDslPlaceholder(token: Token | undefined): boolean {
  return token !== undefined && !token.quoted && token.text === "_";
}

/** 🔢️ One scalar token → the json value its declared field kind calls for. */
function scalar(token: Token, field: BlockJsonField): unknown {
  switch (field.kind) {
    case "text":
    case "optionalText":
      return token.text;
    case "bool":
      return token.text === "true";
    case "float":
    case "optionalFloat":
      return Number(token.text.endsWith("rad") ? token.text.slice(0, -3) : token.text);
    case "floatTuple":
    case "optionalFloatTuple":
      return numbers(token.text.startsWith("@") || token.text.startsWith("^") ? token.text.slice(1) : token.text);
    default:
      throw new Error(`block5d dsl: field \`${field.key}\` is not a scalar`);
  }
}
// #endregion 🔢️Scalars

// #region 📥️Reader
function readBlockBody(lines: string[], fields: readonly BlockJsonField[]): Record<string, unknown> {
  const record: Record<string, unknown> = {};
  const byDslName = new Map(fields.map((field) => [dslName(field.key), field]));
  for (const line of lines) {
    for (const [, name, raw] of line.matchAll(/([A-Za-z0-9-]+)=("(?:[^"\\]|\\.)*"|[^\s]*)/g)) {
      const field = byDslName.get(name);
      if (!field) throw new Error(`block5d dsl: unknown member \`${name}\``);
      record[field.key] = scalar(tokenize(raw)[0] ?? { text: "", quoted: true }, field);
    }
  }
  for (const field of fields) {
    if (field.key in record) continue;
    if (field.kind === "text") record[field.key] = "";
    else if (field.kind === "float") record[field.key] = 0;
    else if (field.kind === "bool") record[field.key] = false;
    else if (field.kind === "floatTuple") record[field.key] = [];
    else if (field.kind === "textList") record[field.key] = [];
    else if (field.kind === "table") record[field.key] = [];
  }
  return record;
}

function readTableRow(tokens: Token[], columns: readonly BlockJsonField[]): Record<string, unknown> {
  const row: Record<string, unknown> = {};
  let index = 0;
  for (const column of columns) {
    if (column.kind === "textList" || column.kind === "table") {
      if (tokens[index]?.text !== "[") throw new Error(`block5d dsl: expected \`[\` for column \`${column.key}\``);
      index += 1;
      const items: string[] = [];
      while (index < tokens.length && tokens[index].text !== "]") {
        if (column.kind === "table") throw new Error(`block5d dsl: nested table rows are outside this reader's scope`);
        items.push(tokens[index].text);
        index += 1;
      }
      index += 1;
      row[column.key] = items;
    } else if (isDslPlaceholder(tokens[index])) {
      index += 1;
    } else {
      row[column.key] = scalar(tokens[index], column);
      index += 1;
    }
  }
  return row;
}

/** 🔤️ Parses `.semio` DSL snapshot text into this subset's snapshot. */
export function block5dFromDslText(text: string): Block5dSnapshot {
  if (!text.startsWith(BLOCK_DSL_PREAMBLE)) throw new Error(`block5d dsl: missing \`${BLOCK_DSL_PREAMBLE}\` preamble`);
  const lines = text.split("\n").slice(1);
  const byDslName = new Map(BLOCK5D_SNAPSHOT_FIELDS.map((field) => [dslName(field.key), field]));
  const record: Record<string, unknown> = {};
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index].trim();
    if (line.length === 0) continue;
    const opener = line.match(/^([a-z0-9-]+)\s*(\[[^\]]*\])?\s*\{$/);
    if (opener) {
      const field = byDslName.get(opener[1]);
      if (!field) throw new Error(`block5d dsl: unknown member \`${opener[1]}\``);
      const body: string[] = [];
      index += 1;
      while (index < lines.length && lines[index].trim() !== "}") {
        body.push(lines[index]);
        index += 1;
      }
      if (field.kind === "record") {
        record[field.key] = readBlockBody(body, field.fields);
      } else if (field.kind === "table") {
        const header = (opener[2] ?? "[]").slice(1, -1).trim();
        const names = header.length === 0 ? [] : header.split(/\s+/).map((column) => column.split(":")[0]);
        const columns = names.map((name) => {
          const column = field.fields.find((candidate) => dslName(candidate.key) === name);
          if (!column) throw new Error(`block5d dsl: unknown column \`${name}\` in \`${opener[1]}\``);
          return column;
        });
        record[field.key] = body.filter((row) => row.trim().length > 0).map((row) => readTableRow(tokenize(row), columns));
      } else {
        throw new Error(`block5d dsl: \`${opener[1]}\` is not a block or table member`);
      }
      continue;
    }
    const scalarLine = line.match(/^([a-z0-9-]+)=(.*)$/);
    if (!scalarLine) throw new Error(`block5d dsl: unparsable line \`${line}\``);
    const field = byDslName.get(scalarLine[1]);
    if (!field) throw new Error(`block5d dsl: unknown member \`${scalarLine[1]}\``);
    record[field.key] = scalar(tokenize(scalarLine[2])[0] ?? { text: "", quoted: true }, field);
  }
  for (const field of BLOCK5D_SNAPSHOT_FIELDS) {
    if (field.key in record) continue;
    if (field.kind === "table") record[field.key] = [];
    else if (field.kind === "record") record[field.key] = readBlockBody([], field.fields);
    else if (field.kind === "text") record[field.key] = "";
  }
  return record as unknown as Block5dSnapshot;
}
// #endregion 📥️Reader
