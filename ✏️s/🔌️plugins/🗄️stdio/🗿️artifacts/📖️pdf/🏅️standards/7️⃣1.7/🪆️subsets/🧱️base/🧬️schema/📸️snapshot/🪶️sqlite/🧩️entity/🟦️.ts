/** 🧩️ Explicit PDF entity ownership, bounded copies and relationship reconstruction. */
import { ArtifactSqliteProjection, artifactSqliteCheckpoint, artifactSqliteTables, artifactSqliteInteger, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { encodeIeee754Cells, readBinary64, ieee754IsNull, type Ieee754Column, type Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type { SqliteDatabase, SqliteRow, SqliteValue } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export type { ArtifactSqliteOptions, SqliteDatabase, SqliteRow, SqliteValue, Binary64 };
export type PdfCell = SqliteValue | Binary64;
export type PdfNumberColumns = (table: string) => readonly Ieee754Column[];
const noNumbers: PdfNumberColumns = () => [];

/** 🏗️ Insert only explicitly supplied fields into the PDF owner's handwritten tables. */
export class PdfProjection {
  private count = 0;
  private constructor(private readonly inner: ArtifactSqliteProjection, readonly options: ArtifactSqliteOptions, private readonly numbers: PdfNumberColumns) {}
  static async create(sql: string, options: ArtifactSqliteOptions = {}, numbers: PdfNumberColumns = noNumbers): Promise<PdfProjection> {
    if ((options.maxRows ?? 1_000_000) < 1) throw new Error("PDF SQLite metadata row limit");
    return new PdfProjection(await ArtifactSqliteProjection.create(sql, options), options, numbers);
  }
  checkRowsAdditional(count: number): void { this.inner.checkRowsAdditional(count + 1); }
  async bytes(value: readonly number[]): Promise<Uint8Array> {
    this.inner.checkValueBytesAdditional(value.length);
    if (value.length > 65_536 || this.options.signal?.aborted) await artifactSqliteCheckpoint(this.options, "projectSnapshot", this.count, 0);
    for (let index = 0; index < value.length; index++) { const octet = value[index]; if (!Number.isInteger(octet) || octet! < 0 || octet! > 255) throw new Error("PDF octet exceeds byte width"); if (index > 0 && index % 256 === 0) await artifactSqliteCheckpoint(this.options, "projectSnapshot", this.count, 0); }
    return Uint8Array.from(value);
  }
  async insert(table: string, cells: readonly PdfCell[], identity?: bigint): Promise<bigint> {
    this.checkRowsAdditional(1);
    const encoded = encodeIeee754Cells([identity ?? 0n, ...cells], this.numbers(table), this.options.maxColumns);
    const key = await this.inner.insert(table, encoded.slice(1), identity);
    this.count++;
    return key;
  }
  async finish(): Promise<SqliteDatabase> { return this.inner.finish(); }
}

/** 🗂️ Verify exact entities and consume each relationship once without recursive traversal. */
export class PdfReader {
  private readonly tables = new Map<string, Map<bigint, SqliteRow>>();
  private readonly used = new Set<SqliteRow>();
  private readonly relations = new Map<string, Map<bigint, SqliteRow[]>>();
  private copiedBytes = 0;
  private copyUnits = 0;
  private constructor(database: SqliteDatabase, readonly options: ArtifactSqliteOptions, private readonly numbers: PdfNumberColumns, private readonly total: number) {
    for (const table of database.tables) {
      const rows = new Map<bigint, SqliteRow>();
      for (const row of table.rows) { if (row.rowid <= 0n || artifactSqliteInteger(row, 0) !== row.rowid || rows.has(row.rowid)) throw new Error("PDF entity identities must be positive and unique"); rows.set(row.rowid, row); }
      this.tables.set(table.name, rows);
    }
  }
  static async create(database: SqliteDatabase, sql: string, options: ArtifactSqliteOptions = {}, numbers: PdfNumberColumns = noNumbers): Promise<PdfReader> {
    await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, 0);
    const total = database.tables.reduce((count, table) => count + table.rows.length, 0);
    if (total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("PDF SQLite metadata row limit");
    await artifactSqliteTables(database, sql, options);
    return new PdfReader(database, options, numbers, total);
  }
  has(table: string, key: bigint): boolean { return this.tables.get(table)?.has(key) ?? false; }
  async take(table: string, key: bigint, width: number): Promise<SqliteRow> {
    const row = this.tables.get(table)?.get(key);
    if (!row || this.used.has(row) || row.values.length !== width + 2 * this.numbers(table).length) throw new Error("PDF entity is missing, multiply owned or has a different width");
    this.used.add(row);
    if (this.used.size % 256 === 0 || this.options.signal?.aborted) await artifactSqliteCheckpoint(this.options, "reconstructSnapshot", this.used.size, this.total);
    return row;
  }
  async children(table: string, parentColumn: number, ordinalColumn: number, parent: bigint, roleColumn?: number, role?: string): Promise<readonly SqliteRow[]> {
    const signature = `${table}/${parentColumn}/${ordinalColumn}/${roleColumn ?? ""}/${role ?? ""}`;
    let relation = this.relations.get(signature);
    if (!relation) {
      relation = new Map();
      let index = 0;
      for (const row of this.tables.get(table)?.values() ?? []) {
        if (roleColumn !== undefined && artifactSqliteText(row, roleColumn) !== role) continue;
        const owner = artifactSqliteInteger(row, parentColumn);
        const rows = relation.get(owner) ?? [];
        rows.push(row); relation.set(owner, rows);
        if (++index % 256 === 0) await artifactSqliteCheckpoint(this.options, "reconstructSnapshot", this.used.size, this.total);
      }
      for (const [owner, rows] of relation) {
        const ordered: SqliteRow[] = new Array(rows.length);
        for (const row of rows) { const ordinal = artifactSqliteInteger(row, ordinalColumn); if (ordinal < 0n || ordinal >= BigInt(rows.length) || ordered[Number(ordinal)]) throw new Error("PDF relation ordinals must be unique and contiguous"); ordered[Number(ordinal)] = row; }
        relation.set(owner, ordered);
      }
      this.relations.set(signature, relation);
    }
    return relation.get(parent) ?? [];
  }
  private async account(bytes: number): Promise<void> {
    this.copiedBytes += bytes;
    artifactSqliteValueBudget(this.copiedBytes, this.options);
    if (++this.copyUnits % 256 === 0 || bytes > 65_536 || this.options.signal?.aborted) await artifactSqliteCheckpoint(this.options, "reconstructSnapshot", this.used.size, this.total);
  }
  async text(row: SqliteRow, column: number): Promise<string> { const value = artifactSqliteText(row, column); await this.account(artifactSqliteTextBytes(value)); return value; }
  async optionalText(row: SqliteRow, column: number): Promise<string | null> { return row.values[column] === null ? null : this.text(row, column); }
  async bytes(row: SqliteRow, column: number): Promise<number[]> { const value = row.values[column]; if (!(value instanceof Uint8Array)) throw new Error("PDF intrinsic octets must be BLOB"); await this.account(value.length); return Array.from(value); }
  real(table: string, row: SqliteRow, column: number): Binary64 { return readBinary64(row, column, this.numbers(table)); }
  isNull(table: string, row: SqliteRow, column: number): boolean { return this.numbers(table).some(item => item.index === column) ? ieee754IsNull(row, column, this.numbers(table)) : row.values[column] === null; }
  nullExcept(table: string, row: SqliteRow, start: number, end: number, allowed: readonly number[]): void { for (let index = start; index < end; index++) if (!allowed.includes(index) && !this.isNull(table, row, index)) throw new Error("PDF variant contains an unrelated scalar or IEEE word"); }
  async finish(): Promise<void> { if (this.used.size !== this.total) throw new Error("PDF SQLite contains unowned entities or relationships"); await artifactSqliteCheckpoint(this.options, "reconstructSnapshot", this.total, this.total); }
}

/** 🔢️ Check exact native integer widths before SQLite ownership copies. */
export function pdfInteger(value: number | bigint, bits = 32, signed = false): bigint {
  if (typeof value === "number" && !Number.isSafeInteger(value)) throw new Error("PDF integer is not an exact scalar");
  const integer = BigInt(value);
  const minimum = signed ? -(1n << BigInt(bits - 1)) : 0n;
  const maximum = signed ? (1n << BigInt(bits - 1)) - 1n : (1n << BigInt(bits)) - 1n;
  if (integer < minimum || integer > maximum) throw new Error("PDF integer exceeds native width");
  return integer;
}
/** 📏️ Read exact native integer coordinates without unsafe widening. */
export function pdfNumber(row: SqliteRow, column: number, bits = 32, signed = false): number { return Number(pdfInteger(artifactSqliteInteger(row, column), bits, signed)); }
/** 🔘️ Read a boolean's authored INTEGER representation. */
export function pdfBoolean(row: SqliteRow, column: number): boolean { const value = artifactSqliteInteger(row, column); if (value !== 0n && value !== 1n) throw new Error("PDF boolean must be zero or one"); return value === 1n; }
/** 🧱️ Reject scalar payloads that do not belong to the declared variant. */
export function pdfNullExcept(row: SqliteRow, start: number, end: number, allowed: readonly number[]): void { for (let index = start; index < end; index++) if (!allowed.includes(index) && row.values[index] !== null) throw new Error("PDF variant contains an unrelated scalar"); }
