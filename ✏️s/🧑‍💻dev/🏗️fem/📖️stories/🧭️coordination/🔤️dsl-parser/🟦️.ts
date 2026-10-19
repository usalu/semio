//#region 🔖️Model
/** 🗂️ One parsed fem DSL document: the banner dialect/version, top-level scalars, `name { … }` statement blocks and `name [cols] { rows }` tables. */
export type FemDslDocument = {
  readonly dialect: string;
  readonly version: string;
  readonly scalars: Readonly<Record<string, string>>;
  readonly blocks: Readonly<Record<string, readonly FemDslStatement[]>>;
  readonly tables: Readonly<Record<string, readonly Readonly<Record<string, unknown>>[]>>;
};

/** 📄️ One line of a statement block: an optional leading keyword (`beam`, `nodal`, `area`, …) plus its `key=value` fields. */
export type FemDslStatement = { readonly keyword: string | undefined; readonly fields: Readonly<Record<string, unknown>> };

//#endregion 🔖️Model

//#region 🔖️Tokenizer
const CLOSERS: Readonly<Record<string, string>> = { "[": "]", "{": "}" };

/** ✂️ Splits DSL text into column tokens: a quoted string (re-emitted as JSON so the reader can tell it from a bare word), a balanced `[ … ]` or `{ … }` group (newlines included), or a bare word. Never splits inside quotes or a group. */
function tokenize(text: string): string[] {
  const tokens: string[] = [];
  let index = 0;
  while (index < text.length) {
    const char = text[index]!;
    if (char === " " || char === "\t" || char === "\n" || char === "\r") {
      index += 1;
      continue;
    }
    if (char === '"') {
      let end = index + 1;
      let value = "";
      while (end < text.length && text[end] !== '"') {
        if (text[end] === "\\" && end + 1 < text.length) {
          value += text[end + 1];
          end += 2;
          continue;
        }
        value += text[end];
        end += 1;
      }
      tokens.push(JSON.stringify(value));
      index = end + 1;
      continue;
    }
    const closer = CLOSERS[char];
    if (closer !== undefined) {
      let depth = 0;
      let end = index;
      while (end < text.length) {
        if (text[end] === char) depth += 1;
        else if (text[end] === closer) {
          depth -= 1;
          if (depth === 0) break;
        }
        end += 1;
      }
      tokens.push(text.slice(index, end + 1));
      index = end + 1;
      continue;
    }
    let end = index;
    while (end < text.length && !" \t\n\r".includes(text[end]!)) end += 1;
    tokens.push(text.slice(index, end));
    index = end;
  }
  return tokens;
}

//#endregion 🔖️Tokenizer

//#region 🔖️Values
const NUMBER_PATTERN = /^[-+]?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?$/;

const QUANTITY_PATTERN = /^([-+]?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?)([A-Za-z°µ][A-Za-z0-9/^·]*)$/;

const COORDINATE_PATTERN = /^[-+]?\d+(?:\.\d+)?(?:,[-+]?\d+(?:\.\d+)?)+$/;

const TEXTUAL_COLUMN_TYPES: ReadonlySet<string> = new Set(["TEXT", "ID", "BOOL"]);

/** 🔤️ `some-key` → `someKey`, the record key every projection below reads. */
function camel(key: string): string {
  return key.replace(/-([a-z0-9])/g, (_, letter: string) => letter.toUpperCase());
}

/** 🧩️ One `key=value key=value …` run (possibly multi-line) as a typed record; quoted values keep their spaces. */
function parseAssignments(text: string): Record<string, unknown> {
  const record: Record<string, unknown> = {};
  for (const token of tokenize(text)) {
    const equals = token.indexOf("=");
    if (equals <= 0) continue;
    record[camel(token.slice(0, equals))] = coerceFemDslValue(token.slice(equals + 1));
  }
  return record;
}

/** 📄️ A `{ … }` statement run as one record per line — the shape `#[dsl(statements, block)]` fields (`FemLoadCase::loads`) and `elements { … }` both print. */
function parseStatements(body: string): FemDslStatement[] {
  const statements: FemDslStatement[] = [];
  for (const rawLine of body.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (line === "" || line.startsWith("#")) continue;
    const tokens = tokenize(line);
    if (tokens.length === 0) continue;
    const leading = tokens[0]!;
    const keyword = leading.includes("=") ? undefined : leading;
    const fields: Record<string, unknown> = {};
    for (const token of tokens.slice(keyword === undefined ? 0 : 1)) {
      const equals = token.indexOf("=");
      if (equals <= 0) continue;
      fields[camel(token.slice(0, equals))] = coerceFemDslValue(token.slice(equals + 1));
    }
    statements.push({ keyword, fields });
  }
  return statements;
}

/** 🔢️ Coerces one token to its typed value: `_` is the DSL's "absent" marker, quantities drop their unit suffix (`-4m` → `-4`, `210000000000Pa` → `2.1e11`), `x,y` coordinate pairs become number tuples, `[ … ]` becomes a list and `{ … }` a MAP record or a statement list. A `TEXT`/`ID`/`BOOL` column never has its unit stripped, so an id like `chs76` stays a string. */
export function coerceFemDslValue(token: string, columnType?: string): unknown {
  if (token === "_") return undefined;
  if (token.startsWith('"')) return JSON.parse(token) as string;
  if (token.startsWith("[")) {
    const inner = token.slice(1, -1).trim();
    return inner === "" ? [] : tokenize(inner).map((entry) => coerceFemDslValue(entry));
  }
  if (token.startsWith("{")) {
    const inner = token.slice(1, -1);
    return columnType === "MAP" ? parseAssignments(inner) : parseStatements(inner);
  }
  if (token === "true") return true;
  if (token === "false") return false;
  if (NUMBER_PATTERN.test(token)) return Number(token);
  if (COORDINATE_PATTERN.test(token)) return token.split(",").map(Number);
  if (!TEXTUAL_COLUMN_TYPES.has(columnType ?? "")) {
    const quantity = QUANTITY_PATTERN.exec(token);
    if (quantity) return Number(quantity[1]);
  }
  return token;
}

/** 🔤️ Column header `name:TYPE` → the camelCased record key plus the declared type. */
function parseColumnHeader(header: string): { readonly key: string; readonly type: string } {
  const colon = header.lastIndexOf(":");
  return { key: camel(colon > 0 ? header.slice(0, colon) : header), type: colon > 0 ? header.slice(colon + 1) : "TEXT" };
}

//#endregion 🔖️Values

//#region 🔖️Parser
const TABLE_HEADER = /^([a-z0-9-]+)\s*\[([^\]]*)\]\s*\{$/;

const BLOCK_HEADER = /^([a-z0-9-]+)\s*\{$/;

/** 🧮️ Net brace depth a line contributes, ignoring braces inside a quoted string. */
function braceDelta(line: string): number {
  let delta = 0;
  let quoted = false;
  for (const char of line) {
    if (char === '"') quoted = !quoted;
    else if (!quoted && char === "{") delta += 1;
    else if (!quoted && char === "}") delta -= 1;
  }
  return delta;
}

/** 📖️ Parses one `🗣️.dsl.semio` fem document. Throws on a missing/foreign banner rather than returning a half-read document — a story rendering nothing is far harder to diagnose than a thrown fixture error. */
export function parseFemDsl(text: string): FemDslDocument {
  const lines = text.split(/\r?\n/);
  const banner = /^semio\s+(\S+)\s+v(\S+)\s*$/.exec(lines[0] ?? "");
  if (!banner) throw new Error(`[fem-dsl] missing "semio <dialect> v<n>" banner: ${JSON.stringify(lines[0] ?? "")}`);
  const scalars: Record<string, string> = {};
  const blocks: Record<string, FemDslStatement[]> = {};
  const tables: Record<string, Record<string, unknown>[]> = {};

  let index = 1;
  const readBody = (): string => {
    const body: string[] = [];
    let depth = 1;
    while (index < lines.length) {
      const line = lines[index] ?? "";
      index += 1;
      depth += braceDelta(line);
      if (depth === 0) break;
      body.push(line);
    }
    return body.join("\n");
  };

  while (index < lines.length) {
    const line = (lines[index] ?? "").trim();
    index += 1;
    if (line === "" || line.startsWith("#")) continue;

    const table = TABLE_HEADER.exec(line);
    if (table) {
      const columns = table[2]!.trim().split(/\s+/).filter(Boolean).map(parseColumnHeader);
      const tokens = tokenize(readBody());
      const rows: Record<string, unknown>[] = [];
      for (let offset = 0; offset + columns.length <= tokens.length; offset += columns.length) {
        const row: Record<string, unknown> = {};
        columns.forEach((column, position) => {
          row[column.key] = coerceFemDslValue(tokens[offset + position]!, column.type);
        });
        rows.push(row);
      }
      tables[table[1]!] = rows;
      continue;
    }

    const block = BLOCK_HEADER.exec(line);
    if (block) {
      blocks[block[1]!] = parseStatements(readBody());
      continue;
    }

    const equals = line.indexOf("=");
    if (equals > 0) scalars[line.slice(0, equals)] = line.slice(equals + 1);
  }

  return { dialect: banner[1]!, version: banner[2]!, scalars, blocks, tables };
}

//#endregion 🔖️Parser
