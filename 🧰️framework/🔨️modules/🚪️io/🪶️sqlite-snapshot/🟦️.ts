/** 🪶️ Dependency-free relational SQLite 3 physical engine. @see https://sqlite.org/fileformat.html */
export type SqliteValue = null | bigint | number | string | Uint8Array;
/** 🧾️ Semantic cells in declared column order, including resolved INTEGER PRIMARY KEY aliases. */
export interface SqliteRow { readonly rowid: bigint; readonly values: readonly SqliteValue[] }
/** 🗃️ A handcrafted rowid-table schema and its semantic rows. */
export interface SqliteTable { readonly name: string; readonly sql: string; readonly rows: readonly SqliteRow[] }
/** 🗄️ A standalone relational database containing no opaque snapshot carrier. */
export interface SqliteDatabase { readonly tables: readonly SqliteTable[] }
/** 📈️ Progress at physical database page boundaries. */
export interface SqliteDatabaseProgress { readonly phase: "readPages" | "writePages"; readonly completed: number; readonly total: number }
/** 🛑️ Cancellation and aggregate allocation limits for relational transfer. */
export interface SqliteDatabaseOptions {
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: SqliteDatabaseProgress) => void;
  readonly maxFileBytes?: number;
  readonly maxValueBytes?: number;
  readonly maxSchemaBytes?: number;
  readonly maxRows?: number;
  readonly maxColumns?: number;
  readonly maxTables?: number;
  readonly maxPages?: number;
}

const PAGE_SIZE = 4096;
const APPLICATION_ID = 0x534d534e;
const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });
const magic = encoder.encode("SQLite format 3\0");
const INT_MIN = -(1n << 63n);
const INT_MAX = (1n << 63n) - 1n;

function invalid(reason: string): never { throw new Error(`Invalid relational SQLite database: ${reason}`); }
function cancelled(options: SqliteDatabaseOptions): void {
  if (options.signal?.aborted) {
    const error = new Error("Relational SQLite transfer cancelled");
    error.name = "AbortError";
    throw error;
  }
}
function limits(options: SqliteDatabaseOptions) {
  const value = { file: options.maxFileBytes ?? 272 * 1024 * 1024, data: options.maxValueBytes ?? 256 * 1024 * 1024, schema: options.maxSchemaBytes ?? 4 * 1024 * 1024, rows: options.maxRows ?? 1_000_000, columns: options.maxColumns ?? 1024, tables: options.maxTables ?? 4096, pages: options.maxPages ?? 1_000_000 };
  for (const limit of Object.values(value)) if (!Number.isSafeInteger(limit) || limit < 0) invalid("resource limit");
  return value;
}
async function cooperate(options: SqliteDatabaseOptions, step: number): Promise<void> {
  cancelled(options);
  if (step % 64 === 0) await new Promise<void>((resolve) => setTimeout(resolve, 0));
  cancelled(options);
}
async function checkpoint(options: SqliteDatabaseOptions, phase: "readPages" | "writePages", completed: number, total: number): Promise<void> {
  cancelled(options);
  options.onProgress?.({ phase, completed, total });
  await cooperate(options, completed);
}
function text(bytes: Uint8Array): string {
  try { return decoder.decode(bytes); } catch { return invalid("UTF-8"); }
}
function textByteLength(value: string): number {
  let bytes = 0;
  for (let index = 0; index < value.length; index++) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const next = value.charCodeAt(++index);
      if (!(next >= 0xdc00 && next <= 0xdfff)) invalid("UTF-8");
      bytes += 4;
    } else {
      if (code >= 0xdc00 && code <= 0xdfff) invalid("UTF-8");
      bytes += code < 0x80 ? 1 : code < 0x800 ? 2 : 3;
    }
  }
  return bytes;
}
function utf8(value: string): Uint8Array {
  textByteLength(value);
  return encoder.encode(value);
}

/** 🧮️ Count validated semantic scalar bytes without allocating encoded copies. */
export function sqliteValueByteLength(value: SqliteValue): number {
  if (value === null) return 0;
  if (typeof value === "string") return textByteLength(value);
  if (typeof value === "bigint") { if (value < INT_MIN || value > INT_MAX) invalid("signed64 integer"); return 8; }
  if (typeof value === "number") { if (Number.isNaN(value)) invalid("NaN REAL"); return 8; }
  if (!(value instanceof Uint8Array)) invalid("cell type");
  return value.length;
}
function lower(value: string): string { return value.replace(/[A-Z]/g, (letter) => String.fromCharCode(letter.charCodeAt(0) + 32)); }
function varint(value: bigint): Uint8Array {
  const unsigned = BigInt.asUintN(64, value);
  if (value < 0n || unsigned >= 1n << 56n) {
    const result = new Uint8Array(9);
    result[8] = Number(unsigned & 255n);
    let remaining = unsigned >> 8n;
    for (let i = 7; i >= 0; i--) { result[i] = Number(remaining & 127n) | 128; remaining >>= 7n; }
    return result;
  }
  let remaining = unsigned;
  const result = [Number(remaining & 127n)];
  while ((remaining >>= 7n) > 0n) result.unshift(Number(remaining & 127n) | 128);
  return Uint8Array.from(result);
}
function join(parts: readonly Uint8Array[]): Uint8Array {
  const bytes = new Uint8Array(parts.reduce((sum, part) => sum + part.length, 0));
  let offset = 0;
  for (const part of parts) { bytes.set(part, offset); offset += part.length; }
  return bytes;
}
function localPayload(length: number, usable: number): number {
  if (length <= usable - 35) return length;
  const minimum = Math.floor(((usable - 12) * 32) / 255) - 23;
  const candidate = minimum + ((length - minimum) % (usable - 4));
  return candidate <= usable - 35 ? candidate : minimum;
}

interface Token { value: string; quoted: boolean; literal: boolean; start: number; end: number }
interface Column { name: string; affinity: "integer" | "real" | "text" | "blob" | "numeric"; notNull: boolean; integerType: boolean }
interface TableSchema { columns: Column[]; alias?: number }
function tokens(sql: string): Token[] {
  const result: Token[] = [];
  const pattern = /\s+|--[^\n]*|\/\*[\s\S]*?\*\/|"(?:[^"]|"")*"|`(?:[^`]|``)*`|\[[^\]]*\]|'(?:[^']|'')*'|[A-Za-z_][A-Za-z_0-9]*|(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?|[(),;.+\-*\/%=<>!|&~]/gy;
  let offset = 0;
  while (offset < sql.length) {
    pattern.lastIndex = offset;
    const match = pattern.exec(sql);
    if (!match) invalid("schema SQL token");
    let value = match[0];
    offset = pattern.lastIndex;
    if (/^\s+$/.test(value) || value.startsWith("--") || value.startsWith("/*")) continue;
    const quoted = value.startsWith('"') || value.startsWith("`") || value.startsWith("[");
    const literal = value.startsWith("'");
    if (value.startsWith('"')) value = value.slice(1, -1).replace(/""/g, '"');
    else if (value.startsWith("`")) value = value.slice(1, -1).replace(/``/g, "`");
    else if (value.startsWith("[")) value = value.slice(1, -1);
    result.push({ value, quoted, literal, start: match.index, end: pattern.lastIndex });
  }
  if (result.at(-1)?.value === ";") result.pop();
  return result;
}
function keyword(token: Token | undefined, word: string): boolean { return token !== undefined && !token.quoted && !token.literal && lower(token.value) === word; }
function identifier(token: Token | undefined): string {
  if (!token || token.literal || (!token.quoted && !/^[A-Za-z_][A-Za-z_0-9]*$/.test(token.value))) invalid("schema identifier");
  return token.value;
}
function topLevelPair(part: readonly Token[], first: string, second: string): boolean {
  let depth = 0;
  for (let i = 0; i < part.length; i++) {
    const token = part[i]!;
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    else if (!token.quoted && !token.literal && token.value === ")") depth--;
    else if (depth === 0 && keyword(token, first) && keyword(part[i + 1], second)) return true;
  }
  return false;
}
function schema(name: string, sql: string, maxColumns: number): TableSchema {
  const all = tokens(sql);
  if (!keyword(all[0], "create") || !keyword(all[1], "table") || lower(identifier(all[2])) !== lower(name) || all[3]?.value !== "(") invalid("schema CREATE TABLE");
  if (all.some((token) => ["unique", "without", "autoincrement", "generated", "desc"].some((word) => keyword(token, word)))) invalid("unsupported indexed or generated schema");
  const groups: Token[][] = [];
  let group: Token[] = [];
  let depth = 1;
  let offset = 4;
  for (; offset < all.length; offset++) {
    const token = all[offset]!;
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    if (!token.quoted && !token.literal && token.value === ")") depth--;
    if (depth === 0) { if (group.length) groups.push(group); break; }
    if (depth === 1 && token.value === "," && !token.quoted && !token.literal) { groups.push(group); group = []; }
    else group.push(token);
  }
  if (depth !== 0 || offset !== all.length - 1 || !groups.length) invalid("schema parentheses");
  const columns: Column[] = [];
  let alias: number | undefined;
  let primaryColumn: string | undefined;
  const constraints = ["primary", "not", "null", "check", "default", "collate", "references", "constraint"];
  for (const source of groups) {
    let part = source;
    if (keyword(part[0], "constraint")) { identifier(part[1]); part = part.slice(2); }
    if (keyword(part[0], "foreign") || keyword(part[0], "check")) continue;
    if (keyword(part[0], "primary")) {
      if (!keyword(part[1], "key") || part[2]?.value !== "(" || part[4]?.value !== ")" || part.length !== 5 || primaryColumn !== undefined) invalid("unsupported primary key schema");
      primaryColumn = identifier(part[3]);
      continue;
    }
    const columnName = identifier(part[0]);
    if (columns.some((column) => lower(column.name) === lower(columnName))) invalid("schema duplicate column");
    const boundary = part.findIndex((token, index) => index > 0 && constraints.some((word) => keyword(token, word)));
    const typeTokens = part.slice(1, boundary < 0 ? part.length : boundary);
    const typeName = typeTokens.map((token) => lower(token.value)).join(" ");
    const affinity = typeName.includes("int") ? "integer" : /char|clob|text/.test(typeName) ? "text" : typeName === "" || typeName.includes("blob") ? "blob" : /real|floa|doub/.test(typeName) ? "real" : "numeric";
    const notNull = topLevelPair(part, "not", "null");
    const primary = topLevelPair(part, "primary", "key");
    if (primary) {
      if (typeTokens.length !== 1 || typeTokens[0]!.quoted || !keyword(typeTokens[0], "integer") || alias !== undefined || primaryColumn !== undefined) invalid("unsupported non-INTEGER primary key");
      alias = columns.length;
    }
    columns.push({ name: columnName, affinity, notNull, integerType: typeTokens.length === 1 && keyword(typeTokens[0], "integer") });
    if (columns.length > maxColumns) invalid("column limit");
  }
  if (primaryColumn !== undefined) {
    const index = columns.findIndex((column) => lower(column.name) === lower(primaryColumn!));
    if (index < 0 || alias !== undefined || !columns[index]!.integerType) invalid("unsupported non-INTEGER primary key");
    alias = index;
  }
  if (!columns.length) invalid("schema empty table");
  return { columns, alias };
}

/** 🏛️ Parse handcrafted CREATE TABLE statements into empty semantic tables without filesystem access. */
export function parseSqliteDatabaseSchema(sql: string, options: SqliteDatabaseOptions = {}): SqliteDatabase {
  cancelled(options);
  const limit = limits(options);
  if (textByteLength(sql) > limit.schema) invalid("schema limit");
  const statements: Token[][] = [];
  let statement: Token[] = [];
  for (const token of tokens(sql)) {
    if (token.value === ";" && !token.quoted && !token.literal) { if (statement.length) statements.push(statement); statement = []; }
    else statement.push(token);
  }
  if (statement.length) statements.push(statement);
  if (statements.length > limit.tables) invalid("table limit");
  const names = new Set<string>();
  let bytes = 0;
  return { tables: statements.map((statement) => {
    const name = identifier(statement[2]);
    const source = sql.slice(statement[0]!.start, statement.at(-1)!.end);
    if (names.has(lower(name)) || lower(name).startsWith("sqlite_")) invalid("schema table name");
    names.add(lower(name));
    bytes += textByteLength(name) + textByteLength(source);
    if (bytes > limit.schema) invalid("schema limit");
    schema(name, source, limit.columns);
    return { name, sql: source, rows: [] };
  }) };
}

/** 🔎️ Require the handcrafted semantic schema while accepting SQL whitespace, case and identifier quoting. */
export function validateSqliteDatabaseSchema(database: SqliteDatabase, sql: string, options: SqliteDatabaseOptions = {}): void {
  const limit = limits(options);
  const expected = parseSqliteDatabaseSchema(sql, options);
  if (database.tables.length !== expected.tables.length) invalid("artifact table count");
  const names = new Set<string>();
  let rows = 0;
  for (const table of database.tables) {
    const name = lower(table.name);
    if (names.has(name)) invalid("artifact duplicate table");
    names.add(name);
    const definition = expected.tables.find((item) => lower(item.name) === name);
    if (!definition) invalid("artifact unknown table");
    schema(table.name, table.sql, limit.columns);
    if (!equivalentSchema(table.sql, definition.sql)) invalid("artifact table schema mismatch");
    const columns = schema(table.name, definition.sql, limit.columns);
    const ids = new Set<bigint>();
    for (const row of table.rows) {
      if (++rows > limit.rows) invalid("row limit");
      if (row.values.length !== columns.columns.length) invalid("column count");
      if (typeof row.rowid !== "bigint" || row.rowid < INT_MIN || row.rowid > INT_MAX || ids.has(row.rowid)) invalid("rowid range or duplicate");
      ids.add(row.rowid);
      if (columns.alias !== undefined && row.values[columns.alias] !== row.rowid) invalid("primary key alias");
    }
  }
}

function equivalentSchema(actual: string, expected: string): boolean {
  const trusted = tokens(expected);
  const received = tokens(actual);
  if (trusted.length !== received.length) return false;
  const identifiers = new Set<number>([2]);
  const columnNames = new Set<string>();
  let depth = 1;
  let groupStart = true;
  for (let index = 4; index < trusted.length; index++) {
    const token = trusted[index]!;
    if (depth === 1 && groupStart) {
      if (keyword(token, "constraint")) { identifiers.add(index + 1); index++; continue; }
      groupStart = false;
      if (!["primary", "foreign", "check"].some((word) => keyword(token, word))) { identifiers.add(index); columnNames.add(lower(token.value)); }
    }
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    else if (!token.quoted && !token.literal && token.value === ")") depth--;
    else if (depth === 1 && token.value === "," && !token.quoted && !token.literal) groupStart = true;
    if (keyword(token, "references")) {
      identifiers.add(index + 1);
      if (trusted[index + 2]?.value === "(" && !trusted[index + 2]?.quoted) {
        for (let next = index + 3; next < trusted.length && trusted[next]!.value !== ")"; next += 2) identifiers.add(next);
      }
    }
  }
  depth = 0;
  const syntax = ["not", "null", "is", "in", "between", "and", "or", "like", "glob", "match", "regexp", "escape", "case", "when", "then", "else", "end", "collate", "as"];
  for (let index = 0; index < trusted.length; index++) {
    const token = trusted[index]!;
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    else if (!token.quoted && !token.literal && token.value === ")") depth--;
    else if (depth > 1 && !token.literal && columnNames.has(lower(token.value)) && (token.quoted || !syntax.some((word) => keyword(token, word)))) identifiers.add(index);
    if (keyword(token, "collate")) identifiers.add(index + 1);
  }
  return trusted.every((token, index) => {
    const other = received[index]!;
    if (token.literal || other.literal) return token.literal === other.literal && token.value === other.value;
    return lower(token.value) === lower(other.value) && (identifiers.has(index) || token.quoted === other.quoted);
  });
}

function encodedField(value: SqliteValue): { serial: bigint; data: Uint8Array } {
  if (value === null) return { serial: 0n, data: new Uint8Array() };
  if (typeof value === "bigint") {
    if (value < INT_MIN || value > INT_MAX) invalid("signed64 integer");
    if (value === 0n || value === 1n) return { serial: value + 8n, data: new Uint8Array() };
    const widths = [1, 2, 3, 4, 6, 8];
    const width = widths.find((size) => value >= -(1n << BigInt(size * 8 - 1)) && value < 1n << BigInt(size * 8 - 1))!;
    const bytes = new Uint8Array(width);
    let unsigned = BigInt.asUintN(width * 8, value);
    for (let i = width - 1; i >= 0; i--) { bytes[i] = Number(unsigned & 255n); unsigned >>= 8n; }
    return { serial: BigInt(widths.indexOf(width) + 1), data: bytes };
  }
  if (typeof value === "number") {
    if (Number.isNaN(value)) invalid("NaN REAL");
    const data = new Uint8Array(8);
    new DataView(data.buffer).setFloat64(0, value);
    return { serial: 7n, data };
  }
  if (typeof value === "string") { const data = utf8(value); return { serial: BigInt(data.length) * 2n + 13n, data }; }
  if (!(value instanceof Uint8Array)) invalid("cell type");
  return { serial: BigInt(value.length) * 2n + 12n, data: value };
}
function encodedRecord(fields: readonly { serial: bigint; data: Uint8Array }[]): Uint8Array {
  const serials = join(fields.map((field) => varint(field.serial)));
  let length = serials.length + 1;
  while (length !== serials.length + varint(BigInt(length)).length) length = serials.length + varint(BigInt(length)).length;
  return join([varint(BigInt(length)), serials, ...fields.map((field) => field.data)]);
}
interface Cell { key: bigint; bytes: Uint8Array }
interface TreeNode { page: number; maximum: bigint }

async function preflightDatabase(database: SqliteDatabase, options: SqliteDatabaseOptions, limit: ReturnType<typeof limits>): Promise<TableSchema[]> {
  const definitions: TableSchema[] = [];
  const names = new Set<string>();
  let dataBytes = 0;
  let schemaBytes = 0;
  let rows = 0;
  for (const table of database.tables) {
    if (!table.name || lower(table.name).startsWith("sqlite_") || names.has(lower(table.name))) invalid("schema table name");
    names.add(lower(table.name));
    schemaBytes += textByteLength(table.sql) + textByteLength(table.name);
    if (schemaBytes > limit.schema) invalid("schema limit");
    const definition = schema(table.name, table.sql, limit.columns);
    definitions.push(definition);
    let previous: bigint | undefined;
    for (const row of table.rows) {
      if (++rows > limit.rows) invalid("row limit");
      if (typeof row.rowid !== "bigint" || row.rowid < INT_MIN || row.rowid > INT_MAX || (previous !== undefined && row.rowid <= previous)) invalid("rowid order or range");
      previous = row.rowid;
      if (row.values.length !== definition.columns.length) invalid("column count");
      for (let index = 0; index < row.values.length; index++) {
        const value = row.values[index]!;
        if (index === definition.alias && (typeof value !== "bigint" || value !== row.rowid)) invalid("primary key alias");
        if (value === null && definition.columns[index]!.notNull) invalid("NOT NULL cell");
        dataBytes += sqliteValueByteLength(value);
        if (dataBytes > limit.data) invalid("value limit");
      }
      await cooperate(options, rows);
    }
    await cooperate(options, definitions.length);
  }
  return definitions;
}

/** 📤️ Export handcrafted semantic tables as an integrity-valid standalone SQLite file. */
export async function exportSqliteDatabase(value: SqliteDatabase, options: SqliteDatabaseOptions = {}): Promise<Uint8Array> {
  cancelled(options);
  const limit = limits(options);
  if (value.tables.length > limit.tables) invalid("table limit");
  const definitions = await preflightDatabase(value, options, limit);
  const pages: Uint8Array[] = [];
  const allocate = (): number => {
    if (pages.length + 1 > limit.pages || (pages.length + 1) * PAGE_SIZE > limit.file) invalid("page or file limit");
    pages.push(new Uint8Array(PAGE_SIZE));
    return pages.length;
  };
  allocate();
  const roots = value.tables.map(() => allocate());
  let rowCount = 0;
  const cell = async (rowid: bigint, payload: Uint8Array): Promise<Cell> => {
    const local = localPayload(payload.length, PAGE_SIZE);
    let first = 0;
    let previous = 0;
    for (let offset = local; offset < payload.length; offset += PAGE_SIZE - 4) {
      const page = allocate();
      if (!first) first = page;
      if (previous) new DataView(pages[previous - 1]!.buffer).setUint32(0, page);
      pages[page - 1]!.set(payload.subarray(offset, offset + PAGE_SIZE - 4), 4);
      previous = page;
      await cooperate(options, page);
    }
    const pointer = new Uint8Array(first ? 4 : 0);
    if (first) new DataView(pointer.buffer).setUint32(0, first);
    return { key: rowid, bytes: join([varint(BigInt(payload.length)), varint(rowid), payload.subarray(0, local), pointer]) };
  };
  const writeLeaf = (page: number, cells: readonly Cell[]): TreeNode => {
    const bytes = pages[page - 1]!;
    const view = new DataView(bytes.buffer);
    const header = page === 1 ? 100 : 0;
    bytes[header] = 13;
    view.setUint16(header + 3, cells.length);
    let cursor = PAGE_SIZE;
    for (let i = 0; i < cells.length; i++) { cursor -= cells[i]!.bytes.length; bytes.set(cells[i]!.bytes, cursor); view.setUint16(header + 8 + i * 2, cursor); }
    view.setUint16(header + 5, cursor);
    return { page, maximum: cells.at(-1)?.key ?? 0n };
  };
  const writeInterior = (page: number, children: readonly TreeNode[]): TreeNode => {
    const bytes = pages[page - 1]!;
    const view = new DataView(bytes.buffer);
    const header = page === 1 ? 100 : 0;
    bytes[header] = 5;
    view.setUint16(header + 3, children.length - 1);
    view.setUint32(header + 8, children.at(-1)!.page);
    let cursor = PAGE_SIZE;
    for (let i = 0; i < children.length - 1; i++) {
      const key = varint(children[i]!.maximum);
      cursor -= key.length + 4;
      view.setUint32(cursor, children[i]!.page);
      bytes.set(key, cursor + 4);
      view.setUint16(header + 12 + i * 2, cursor);
    }
    view.setUint16(header + 5, cursor);
    return { page, maximum: children.at(-1)!.maximum };
  };
  const tree = async (root: number, cells: readonly Cell[]): Promise<void> => {
    const total = cells.reduce((sum, item) => sum + item.bytes.length + 2, 0);
    if (total <= PAGE_SIZE - 8 - (root === 1 ? 100 : 0)) { writeLeaf(root, cells); return; }
    let nodes: TreeNode[] = [];
    let group: Cell[] = [];
    let used = 8;
    for (const item of cells) {
      if (used + item.bytes.length + 2 > PAGE_SIZE) { nodes.push(writeLeaf(allocate(), group)); group = []; used = 8; }
      group.push(item);
      used += item.bytes.length + 2;
    }
    if (group.length) nodes.push(writeLeaf(allocate(), group));
    const rootBranches = Math.floor((PAGE_SIZE - 12 - (root === 1 ? 100 : 0)) / 15) + 1;
    const branches = Math.floor((PAGE_SIZE - 12) / 15) + 1;
    while (nodes.length > rootBranches) {
      const next: TreeNode[] = [];
      let offset = 0;
      while (offset < nodes.length) {
        let count = Math.min(branches, nodes.length - offset);
        if (nodes.length - offset - count === 1) count--;
        next.push(writeInterior(allocate(), nodes.slice(offset, offset + count)));
        offset += count;
        await cooperate(options, pages.length);
      }
      nodes = next;
    }
    writeInterior(root, nodes);
  };
  const schemaCells: Cell[] = [];
  for (let tableIndex = 0; tableIndex < value.tables.length; tableIndex++) {
    const table = value.tables[tableIndex]!;
    const definition = definitions[tableIndex]!;
    const cells: Cell[] = [];
    for (const row of table.rows) {
      rowCount++;
      const fields = row.values.map((value, index) => encodedField(index === definition.alias ? null : value));
      cells.push(await cell(row.rowid, encodedRecord(fields)));
      await cooperate(options, rowCount);
    }
    await tree(roots[tableIndex]!, cells);
    schemaCells.push(await cell(BigInt(tableIndex + 1), encodedRecord(["table", table.name, table.name, BigInt(roots[tableIndex]!), table.sql].map(encodedField))));
  }
  await tree(1, schemaCells);
  const first = pages[0]!;
  const header = new DataView(first.buffer);
  first.set(magic);
  header.setUint16(16, PAGE_SIZE);
  first.set([1, 1, 0, 64, 32, 32], 18);
  for (const [offset, integer] of [[24, 1], [28, pages.length], [40, 1], [44, 4], [56, 1], [60, 1], [68, APPLICATION_ID], [92, 1], [96, 3046000]]) header.setUint32(offset!, integer!);
  const bytes = new Uint8Array(pages.length * PAGE_SIZE);
  for (let i = 0; i < pages.length; i++) { bytes.set(pages[i]!, i * PAGE_SIZE); await checkpoint(options, "writePages", i + 1, pages.length); }
  return bytes;
}

class Reader {
  readonly view: DataView;
  readonly pageSize: number;
  readonly usable: number;
  readonly pages: number;
  readonly seen = new Set<number>();
  readonly limit: ReturnType<typeof limits>;
  dataBytes = 0;
  schemaBytes = 0;
  rowCount = 0;
  constructor(readonly bytes: Uint8Array, readonly options: SqliteDatabaseOptions) {
    cancelled(options);
    this.limit = limits(options);
    if (bytes.length > this.limit.file) invalid("file limit");
    if (bytes.length < 100 || !magic.every((byte, i) => bytes[i] === byte)) invalid("file header");
    this.view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const size = this.view.getUint16(16);
    this.pageSize = size === 1 ? 65536 : size;
    if (this.pageSize < 512 || this.pageSize > 65536 || (this.pageSize & (this.pageSize - 1)) !== 0) invalid("page size");
    if (bytes.length % this.pageSize !== 0) invalid("truncated page");
    this.pages = bytes.length / this.pageSize;
    if (this.pages > this.limit.pages) invalid("page limit");
    this.usable = this.pageSize - bytes[20]!;
    if (this.usable < 480 || bytes[18] !== 1 || bytes[19] !== 1 || bytes[21] !== 64 || bytes[22] !== 32 || bytes[23] !== 32) invalid("page or journal header");
    if (this.view.getUint32(44) !== 4 || this.view.getUint32(56) !== 1 || this.view.getUint32(60) !== 1 || this.view.getUint32(68) !== APPLICATION_ID) invalid("identity, version, schema format or UTF-8 encoding");
    if (bytes.subarray(72, 92).some((byte) => byte !== 0)) invalid("reserved header bytes");
    const declared = this.view.getUint32(28);
    if (declared && this.view.getUint32(24) === this.view.getUint32(92) && declared !== this.pages) invalid("database page count");
  }
  claim(page: number): number {
    if (!Number.isSafeInteger(page) || page < 1 || page > this.pages) invalid("page reference");
    if (this.seen.has(page)) invalid("cyclic or aliased page");
    this.seen.add(page);
    return (page - 1) * this.pageSize;
  }
  variable(bytes: Uint8Array, offset: number, end: number): { value: bigint; next: number } {
    let value = 0n;
    for (let i = 0; i < 9; i++) {
      if (offset >= end) invalid("truncated varint");
      const byte = bytes[offset++]!;
      value = (value << BigInt(i === 8 ? 8 : 7)) | BigInt(i === 8 ? byte : byte & 127);
      if (i === 8 || byte < 128) return { value, next: offset };
    }
    return invalid("varint");
  }
  integer(value: bigint): number { if (value > BigInt(Number.MAX_SAFE_INTEGER)) invalid("integer limit"); return Number(value); }
  fields(payload: Uint8Array, count: number): SqliteValue[] {
    const header = this.variable(payload, 0, payload.length);
    const headerEnd = this.integer(header.value);
    if (headerEnd < header.next || headerEnd > payload.length) invalid("record header");
    const serials: number[] = [];
    let offset = header.next;
    while (offset < headerEnd) {
      if (serials.length >= count) invalid("column count");
      const serial = this.variable(payload, offset, headerEnd);
      serials.push(this.integer(serial.value));
      offset = serial.next;
    }
    if (serials.length !== count) invalid("column count");
    offset = headerEnd;
    const values: SqliteValue[] = [];
    for (const serial of serials) {
      const length = serial >= 12 ? Math.floor((serial - 12) / 2) : [0, 1, 2, 3, 4, 6, 8, 8, 0, 0][serial];
      if (length === undefined || offset + length > payload.length) invalid("serial type or cell length");
      if (serial === 0) values.push(null);
      else if (serial === 8 || serial === 9) values.push(BigInt(serial - 8));
      else if (serial >= 12) values.push(serial % 2 ? text(payload.subarray(offset, offset + length)) : payload.subarray(offset, offset + length));
      else if (serial === 7) {
        const value = new DataView(payload.buffer, payload.byteOffset + offset, 8).getFloat64(0);
        if (Number.isNaN(value)) invalid("NaN REAL");
        values.push(value);
      } else {
        let value = 0n;
        for (let i = 0; i < length; i++) value = (value << 8n) | BigInt(payload[offset + i]!);
        if ((payload[offset]! & 128) !== 0) value -= 1n << BigInt(length * 8);
        values.push(value);
      }
      offset += length;
    }
    if (offset !== payload.length) invalid("record trailing bytes");
    return values;
  }
  async *table(root: number, count: number, schemaRows: boolean): AsyncGenerator<SqliteRow> {
    const pending: { page: number; lower?: bigint; upper?: bigint }[] = [{ page: root }];
    while (pending.length) {
      const node = pending.pop()!;
      const start = this.claim(node.page);
      await checkpoint(this.options, "readPages", this.seen.size, this.pages);
      const end = start + this.usable;
      const header = start + (node.page === 1 ? 100 : 0);
      const type = this.bytes[header];
      if (type !== 5 && type !== 13) invalid("unsupported index or table B-tree page");
      const headerSize = type === 5 ? 12 : 8;
      if (header + headerSize > end) invalid("page header");
      const cells = this.view.getUint16(header + 3);
      const content = this.view.getUint16(header + 5) || 65536;
      const pointerEnd = header + headerSize + cells * 2;
      if (content > this.usable || start + content < pointerEnd || this.bytes[header + 7]! > 60) invalid("cell pointer array");
      const ranges: [number, number][] = [];
      let free = this.view.getUint16(header + 1);
      while (free) {
        if (free < content || free + 4 > this.usable) invalid("freeblock pointer");
        const size = this.view.getUint16(start + free + 2);
        const next = this.view.getUint16(start + free);
        if (size < 4 || free + size > this.usable || (next && next < free + size)) invalid("freeblock chain");
        ranges.push([start + free, start + free + size]);
        free = next;
      }
      const children: typeof pending = [];
      let previousKey = node.lower;
      for (let i = 0; i < cells; i++) {
        const pointer = this.view.getUint16(header + headerSize + i * 2);
        const cell = start + pointer;
        if (pointer < content || cell >= end) invalid("cell pointer");
        let cursor = cell;
        if (type === 5) {
          if (cursor + 4 > end) invalid("interior cell");
          const separator = this.variable(this.bytes, cursor + 4, end);
          const key = BigInt.asIntN(64, separator.value);
          if ((previousKey !== undefined && key <= previousKey) || (node.upper !== undefined && key > node.upper)) invalid("interior rowid order or bounds");
          children.push({ page: this.view.getUint32(cursor), lower: previousKey, upper: key });
          previousKey = key;
          ranges.push([cell, separator.next]);
          continue;
        }
        const size = this.variable(this.bytes, cursor, end);
        if (!schemaRows && this.rowCount >= this.limit.rows) invalid("row limit");
        const length = this.integer(size.value);
        const rowid = this.variable(this.bytes, size.next, end);
        const key = BigInt.asIntN(64, rowid.value);
        if ((previousKey !== undefined && key <= previousKey) || (node.upper !== undefined && key > node.upper)) invalid("leaf rowid order or bounds");
        previousKey = key;
        cursor = rowid.next;
        const budget = schemaRows ? this.limit.schema + count * 9 + 64 : this.limit.data - this.dataBytes + count * 9 + 64;
        if (length > budget || length > this.bytes.length) invalid("schema or value limit");
        const local = localPayload(length, this.usable);
        const overflow = length > local;
        if (cursor + local + (overflow ? 4 : 0) > end) invalid("truncated cell payload");
        ranges.push([cell, cursor + local + (overflow ? 4 : 0)]);
        const payload = new Uint8Array(length);
        payload.set(this.bytes.subarray(cursor, cursor + local));
        let written = local;
        let next = overflow ? this.view.getUint32(cursor + local) : 0;
        while (written < length) {
          const overflowStart = this.claim(next);
          await checkpoint(this.options, "readPages", this.seen.size, this.pages);
          next = this.view.getUint32(overflowStart);
          const chunk = Math.min(this.usable - 4, length - written);
          payload.set(this.bytes.subarray(overflowStart + 4, overflowStart + 4 + chunk), written);
          written += chunk;
        }
        if (next) invalid("overflow trailing page");
        yield { rowid: key, values: this.fields(payload, count) };
      }
      ranges.sort((a, b) => a[0] - b[0]);
      let previous = start + content;
      let fragmented = 0;
      for (const [begin, finish] of ranges) {
        if (begin < previous) invalid("overlapping cells");
        const gap = begin - previous;
        if (gap > 3) invalid("untracked freeblock");
        fragmented += gap;
        previous = finish;
      }
      const tail = end - previous;
      if (tail > 3 || fragmented + tail !== this.bytes[header + 7]) invalid("fragmented cell bytes");
      if (type === 5) {
        children.push({ page: this.view.getUint32(header + 8), lower: previousKey, upper: node.upper });
        for (let i = children.length - 1; i >= 0; i--) pending.push(children[i]!);
      }
    }
  }
}

/** 📥️ Read semantic table rows without access to any native artifact codec. */
export async function importSqliteDatabase(bytes: Uint8Array, options: SqliteDatabaseOptions = {}): Promise<SqliteDatabase> {
  const reader = new Reader(bytes, options);
  const definitions: { name: string; sql: string; root: number; schema: TableSchema }[] = [];
  const names = new Set<string>();
  for await (const row of reader.table(1, 5, true)) {
    const [kind, name, tableName, root, sql] = row.values;
    if (kind !== "table") invalid("unsupported index, view or trigger schema");
    if (typeof name !== "string" || typeof tableName !== "string" || lower(name) !== lower(tableName) || typeof root !== "bigint" || root < 1n || root > BigInt(reader.pages) || typeof sql !== "string" || lower(name).startsWith("sqlite_") || names.has(lower(name))) invalid("schema row types or identifiers");
    names.add(lower(name));
    if (definitions.length + 1 > reader.limit.tables) invalid("table limit");
    reader.schemaBytes += utf8(name).length + utf8(sql).length;
    if (reader.schemaBytes > reader.limit.schema) invalid("schema limit");
    definitions.push({ name, sql, root: Number(root), schema: schema(name, sql, reader.limit.columns) });
  }
  const tables: SqliteTable[] = [];
  for (const definition of definitions) {
    const rows: SqliteRow[] = [];
    for await (const row of reader.table(definition.root, definition.schema.columns.length, false)) {
      if (++reader.rowCount > reader.limit.rows) invalid("row limit");
      const values = row.values.map((cell, index) => {
        const column = definition.schema.columns[index]!;
        if (index === definition.schema.alias) {
          if (cell !== null) invalid("primary key alias storage");
          cell = row.rowid;
        } else if (column.affinity === "real" && typeof cell === "bigint") cell = Number(cell);
        if (cell === null && column.notNull) invalid("NOT NULL cell");
        reader.dataBytes += cell === null ? 0 : typeof cell === "bigint" || typeof cell === "number" ? 8 : typeof cell === "string" ? utf8(cell).length : cell.length;
        if (reader.dataBytes > reader.limit.data) invalid("value limit");
        return cell;
      });
      rows.push({ rowid: row.rowid, values });
      await cooperate(options, reader.rowCount);
    }
    tables.push({ name: definition.name, sql: definition.sql, rows });
  }
  await checkpoint(options, "readPages", reader.pages, reader.pages);
  return { tables };
}
