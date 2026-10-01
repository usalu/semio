/** 🧩️ Owned helpers for handcrafted semantic artifact projections. */
import { parseSqliteDatabaseSchema, validateSqliteDatabaseSchema, sqliteValueByteLength, type SqliteDatabase, type SqliteDatabaseOptions, type SqliteDatabaseProgress, type SqliteRow, type SqliteValue } from "../🟦️.ts";

/** 🚦️ Semantic entity progress plus physical page progress. */
export interface ArtifactSqliteProgress { readonly phase: SqliteDatabaseProgress["phase"] | "projectSnapshot" | "reconstructSnapshot"; readonly completed: number; readonly total: number }
/** 🛑️ Shared cancellation, bounds and progress for typed snapshot projections. */
export interface ArtifactSqliteOptions extends Omit<SqliteDatabaseOptions, "onProgress"> { readonly onProgress?: (progress: ArtifactSqliteProgress) => void }

/** 🏗️ Bounds explicit domain cells before copying them into authored relational rows. */
export class ArtifactSqliteProjection {
  private readonly rows: SqliteRow[][];
  private count = 0;
  private bytes = 0;
  private completed = false;
  private constructor(private readonly sql: string, private readonly schema: SqliteDatabase, private readonly options: ArtifactSqliteOptions) { this.rows = schema.tables.map(() => []); }

  /** 🏛️ Accepts only the caller's handcrafted schema and publishes an initial checkpoint. */
  static async create(sql: string, options: ArtifactSqliteOptions = {}): Promise<ArtifactSqliteProjection> {
    await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
    return new ArtifactSqliteProjection(sql, parseSqliteDatabaseSchema(sql, options), options);
  }

  /** 🧮️ Bound explicitly predicted additional entities before allocating their owned data. */
  checkRowsAdditional(count: number): void {
    if (!Number.isSafeInteger(count) || count < 0 || this.count + count > (this.options.maxRows ?? 1_000_000)) throw new Error("Artifact SQLite row limit");
  }

  /** 📏️ Bound explicitly predicted additional semantic bytes against the remaining allowance. */
  checkValueBytesAdditional(bytes: number): void {
    if (!Number.isSafeInteger(bytes) || bytes < 0) throw new Error("Artifact SQLite value limit");
    artifactSqliteValueBudget(this.bytes + bytes, this.options);
  }

  /** ⏱️ Cancel a borrowed domain traversal before it owns another entity. */
  async checkpoint(): Promise<void> {
    if (this.completed) throw new Error("Artifact SQLite projection is completed");
    await artifactSqliteCheckpoint(this.options, "projectSnapshot", this.count, 0);
  }

  /** 🔗️ Inserts supplied semantic cells with an explicit or per-table surrogate identity. */
  async insert(name: string, cells: readonly SqliteValue[], identity?: bigint): Promise<bigint> {
    if (this.completed) throw new Error("Artifact SQLite projection is completed");
    if (cells.length + 1 > (this.options.maxColumns ?? 1024)) throw new Error("Artifact SQLite column limit");
    const index = this.schema.tables.findIndex(table => table.name === name);
    if (index < 0) throw new Error("Artifact SQLite table is not declared");
    const count = this.count + 1;
    if (count > (this.options.maxRows ?? 1_000_000)) throw new Error("Artifact SQLite row limit");
    const key = identity ?? BigInt(this.rows[index]!.length + 1);
    if (key < -9223372036854775808n || key > 9223372036854775807n) throw new Error("Artifact SQLite row identity exceeds INTEGER width");
    let bytes = this.bytes + 8;
    for (const cell of cells) { if (typeof cell === "number" && Number.isNaN(cell)) throw new Error("NaN is not a semantic SQLite REAL"); bytes += sqliteValueByteLength(cell); artifactSqliteValueBudget(bytes, this.options); }
    artifactSqliteValueBudget(bytes, this.options);
    if (count % 256 === 0 || bytes - this.bytes > 65_536 || this.options.signal?.aborted) await artifactSqliteCheckpoint(this.options, "projectSnapshot", count, 0);
    this.rows[index]!.push({ rowid: key, values: [key, ...cells.map(cell => cell instanceof Uint8Array ? cell.slice() : cell)] });
    this.count = count;
    this.bytes = bytes;
    return key;
  }

  /** 📤️ Completes the bounded projection under the artifact's declared schema. */
  async finish(): Promise<SqliteDatabase> {
    if (this.completed) throw new Error("Artifact SQLite projection is completed");
    await artifactSqliteCheckpoint(this.options, "projectSnapshot", this.count, this.count);
    const database = artifactSqliteDatabase(this.sql, this.rows, this.options);
    this.completed = true;
    return database;
  }
}

/** 🔤️ Count Unicode scalar UTF-8 bytes without allocating an encoded copy. */
export function artifactSqliteTextBytes(value: string): number {
  if (typeof value !== "string") throw new Error("Artifact SQLite value must be TEXT");
  return sqliteValueByteLength(value);
}

/** 🧮️ Bound aggregate semantic cell bytes before allocating entity rows. */
export function artifactSqliteValueBudget(bytes: number, options: ArtifactSqliteOptions): void {
  const limit = options.maxValueBytes ?? 256 * 1024 * 1024;
  if (!Number.isSafeInteger(limit) || limit < 0 || !Number.isSafeInteger(bytes) || bytes > limit) throw new Error("Artifact SQLite value limit");
}

/** ⏱️ Publish semantic entity progress and yield to cancellation at bounded intervals. */
export async function artifactSqliteCheckpoint(options: ArtifactSqliteOptions, phase: "projectSnapshot" | "reconstructSnapshot", completed: number, total: number, yieldToEventLoop = true): Promise<void> {
  const cancelled = (): void => {
    if (options.signal?.aborted) { const error = new Error("Artifact SQLite projection cancelled"); error.name = "AbortError"; throw error; }
  };
  cancelled();
  options.onProgress?.({ phase, completed, total });
  cancelled();
  if (yieldToEventLoop && completed % 256 === 0) await new Promise<void>((resolve) => setTimeout(resolve, 0));
  cancelled();
}

/** 🗂️ Populate each handcrafted table in schema order. */
export function artifactSqliteDatabase(sql: string, rows: readonly (readonly SqliteRow[])[], options: ArtifactSqliteOptions): SqliteDatabase {
  const schema = parseSqliteDatabaseSchema(sql, options);
  if (schema.tables.length !== rows.length) throw new Error("Artifact SQLite table count mismatch");
  const database = { tables: schema.tables.map((table, index) => ({ ...table, rows: rows[index]! })) };
  validateSqliteDatabaseSchema(database, sql, options);
  return database;
}

/** 📋️ Locate typed artifact tables after verifying their complete schema and row identities. */
export async function artifactSqliteTables(database: SqliteDatabase, sql: string, options: ArtifactSqliteOptions): Promise<readonly (readonly SqliteRow[])[]> {
  let bytes = 0;
  let rows = 0;
  const total = database.tables.reduce((count, table) => count + table.rows.length, 0);
  if (total > (options.maxRows ?? 1_000_000)) throw new Error("Artifact SQLite row limit");
  for (const table of database.tables) {
    for (const row of table.rows) {
      for (const value of row.values) { bytes += sqliteValueByteLength(value); artifactSqliteValueBudget(bytes, options); }
      if (++rows % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", rows, total);
    }
  }
  artifactSqliteValueBudget(bytes, options);
  validateSqliteDatabaseSchema(database, sql, options);
  return parseSqliteDatabaseSchema(sql, options).tables.map((table) => database.tables.find((item) => item.name.toLowerCase() === table.name.toLowerCase())!.rows);
}

/** 🔢️ Require a signed INTEGER cell. */
export function artifactSqliteInteger(row: SqliteRow, index: number): bigint {
  const value = row.values[index];
  if (typeof value !== "bigint") throw new Error("Artifact SQLite cell must be INTEGER");
  return value;
}
/** 🔤️ Require a TEXT cell. */
export function artifactSqliteText(row: SqliteRow, index: number): string {
  const value = row.values[index];
  if (typeof value !== "string") throw new Error("Artifact SQLite cell must be TEXT");
  return value;
}
/** 📐️ Decode a finite REAL-affinity numeric scalar. */
export function artifactSqliteReal(row: SqliteRow, index: number): number {
  const value = row.values[index];
  if (typeof value !== "number" && typeof value !== "bigint") throw new Error("Artifact SQLite coordinate must be REAL");
  const number = Number(value);
  if (!Number.isFinite(number)) throw new Error("Artifact SQLite coordinate must be finite");
  return number;
}
/** 🔘️ Decode an explicitly constrained boolean INTEGER. */
export function artifactSqliteBoolean(row: SqliteRow, index: number): boolean {
  const value = artifactSqliteInteger(row, index);
  if (value !== 0n && value !== 1n) throw new Error("Artifact SQLite boolean must be 0 or 1");
  return value === 1n;
}
/** 📍️ Require one identity-1 document row. */
export function artifactSqliteDocument(rows: readonly SqliteRow[]): SqliteRow {
  if (rows.length !== 1 || rows[0]!.rowid !== 1n || artifactSqliteInteger(rows[0]!, 0) !== 1n) throw new Error("Artifact SQLite requires one document with identifier 1");
  return rows[0]!;
}
/** 🔗️ Require a relationship to the identity-1 document. */
export function artifactSqliteDocumentReference(row: SqliteRow, index: number): void {
  if (artifactSqliteInteger(row, index) !== 1n) throw new Error("Artifact SQLite entity has an unknown document");
}
/** 🧮️ Order collection entities by explicit, contiguous ordinals. */
export function artifactSqliteOrderedRows(rows: readonly SqliteRow[], column: number): SqliteRow[] {
  const ordered = new Array<SqliteRow>(rows.length);
  for (const row of rows) {
    const ordinal = artifactSqliteInteger(row, column);
    if (ordinal < 0n || ordinal >= BigInt(rows.length) || ordered[Number(ordinal)] !== undefined) throw new Error("Artifact SQLite ordinals must be contiguous");
    ordered[Number(ordinal)] = row;
  }
  return ordered;
}

/** 🚦️ Bound and cancel ordered relationship reconstruction before allocating its result. */
export async function artifactSqliteOrderedRowsControlled(rows: readonly SqliteRow[], column: number, options: ArtifactSqliteOptions = {}): Promise<SqliteRow[]> {
  if (rows.length > (options.maxRows ?? 1_000_000)) throw new Error("Artifact SQLite row limit");
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, rows.length, rows.length >= 256);
  const ordered = new Array<SqliteRow>(rows.length);
  for (let index = 0; index < rows.length; index++) {
    const row = rows[index]!;
    const ordinal = artifactSqliteInteger(row, column);
    if (ordinal < 0n || ordinal >= BigInt(rows.length) || ordered[Number(ordinal)] !== undefined) throw new Error("Artifact SQLite ordinals must be contiguous");
    ordered[Number(ordinal)] = row;
    if ((index + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", index + 1, rows.length);
  }
  return ordered;
}
