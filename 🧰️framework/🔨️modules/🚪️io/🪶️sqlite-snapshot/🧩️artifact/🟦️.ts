import { hostContinuations } from "../../../⏳️async/🪃️continuation/🟦️.ts";
import {NativeDecodeControl} from "../../../🌱️value/🛬️decode/🟦️.ts";
/** 🧩️ Owned helpers for handcrafted semantic artifact projections. */
import {ArtifactSqliteTableIndex} from "./🗂️table/🟦️.ts";
export {ArtifactSqliteRowIndex} from "./🗂️row/🟦️.ts";
import { ValueError, sqliteOperation, parseSqliteDatabaseSchemaControlled, validateSqliteDatabaseSchemaControlled, sqliteValueByteLength, type SqliteOperation, type SqliteDatabase, type SqliteDatabaseOptions, type SqliteDatabaseProgress, type SqliteRow, type SqliteValue } from "../🟦️.ts";

/** 🚦️ Semantic entity progress plus physical page progress. */
export interface ArtifactSqliteProgress { readonly phase: SqliteDatabaseProgress["phase"] | "projectSnapshot" | "reconstructSnapshot"; readonly completed: number; readonly total: number }
/** 🛑️ Shared cancellation, bounds and progress for typed snapshot projections. */
export interface ArtifactSqliteOptions extends Omit<SqliteDatabaseOptions, "onProgress"> { readonly onProgress?: (progress: ArtifactSqliteProgress) => void }

/** 🧭️ Binds neutral semantic work to the caller's relational projection progress. */
export function artifactSqliteValueControl(options:ArtifactSqliteOptions,phase:"projectSnapshot"|"reconstructSnapshot",completed:number,total:number):NativeDecodeControl{
  return new NativeDecodeControl(0,()=>{options.onProgress?.({phase,completed,total});return true;},options.signal);
}

/** 🏗️ Bounds explicit domain cells before copying them into authored relational rows. */
export class ArtifactSqliteProjection {
  private readonly rows: SqliteRow[][];
  private count = 0;
  private bytes = 0;
  private completed = false;
  private constructor(private readonly sql: string, private readonly schema: SqliteDatabase, private readonly options: SqliteOperation,private readonly index:ArtifactSqliteTableIndex) { this.rows = schema.tables.map(() => []); }

  /** 🏛️ Accepts only the caller's handcrafted schema and publishes an initial checkpoint. */
  static async create(sql: string, options: ArtifactSqliteOptions = {}): Promise<ArtifactSqliteProjection> {
    const operation=sqliteOperation(options);options=operation;
    await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
    const schema=await parseSqliteDatabaseSchemaControlled(sql,operation),index=await ArtifactSqliteTableIndex.create(schema.tables,operation,true);
    return new ArtifactSqliteProjection(sql,schema,operation,index);
  }

  /** 🧮️ Bound explicitly predicted additional entities before allocating their owned data. */
  checkRowsAdditional(count: number): void {
    if (!Number.isSafeInteger(count) || count < 0 || this.count + count > (this.options.maxRows ?? 1_000_000)) throw new ValueError("workLimit", "Artifact SQLite row limit");
  }

  /** 📏️ Bound explicitly predicted additional semantic bytes against the remaining allowance. */
  checkValueBytesAdditional(bytes: number): void {
    if (!Number.isSafeInteger(bytes) || bytes < 0) throw new ValueError("ownershipLimit", "Artifact SQLite value limit");
    artifactSqliteValueBudget(this.bytes + bytes, this.options);
  }

  /** ⏱️ Cancel a borrowed domain traversal before it owns another entity. */
  async checkpoint(): Promise<void> {
    if (this.completed) throw new ValueError("invariantViolated", "Artifact SQLite projection is completed");
    await artifactSqliteCheckpoint(this.options, "projectSnapshot", this.count, 0);
  }

  /** 🔗️ Inserts supplied semantic cells with an explicit or per-table surrogate identity. */
  async insert(name: string, cells: readonly SqliteValue[], identity?: bigint): Promise<bigint> {
    if (this.completed) throw new ValueError("invariantViolated", "Artifact SQLite projection is completed");
    if (cells.length + 1 > (this.options.maxColumns ?? 1024)) throw new ValueError("workLimit", "Artifact SQLite column limit");
    const index = await this.index.find(name);
    if (index < 0) throw new ValueError("invalidValue", "Artifact SQLite table is not declared");
    const count = this.count + 1;
    if (count > (this.options.maxRows ?? 1_000_000)) throw new ValueError("workLimit", "Artifact SQLite row limit");
    const key = identity ?? BigInt(this.rows[index]!.length + 1);
    if (key < -9223372036854775808n || key > 9223372036854775807n) throw new ValueError("workLimit", "Artifact SQLite row identity exceeds INTEGER width");
    let bytes = this.bytes + 8;
    for (const cell of cells) { if (typeof cell === "number" && Number.isNaN(cell)) throw new ValueError("invalidValue", "NaN is not a semantic SQLite REAL"); bytes += await artifactSqliteValueByteLengthControlled(cell, this.options, "projectSnapshot", bytes); }
    artifactSqliteValueBudget(bytes, this.options);
    if (count % 256 === 0 || bytes - this.bytes > 65_536 || this.options.signal?.aborted) await artifactSqliteCheckpoint(this.options, "projectSnapshot", count, 0);
    const values = await projectionCells(key, cells, this.options);
    this.rows[index]!.push({ rowid: key, values });
    this.count = count;
    this.bytes = bytes;
    return key;
  }

  /** 📤️ Completes the bounded projection under the artifact's declared schema. */
  async finish(): Promise<SqliteDatabase> {
    if (this.completed) throw new ValueError("invariantViolated", "Artifact SQLite projection is completed");
    await artifactSqliteCheckpoint(this.options, "projectSnapshot", 0, 0);
    const work = { units: 0 };
    for (const rows of this.rows) await orderProjectionRows(rows, this.options, work);
    await artifactSqliteCheckpoint(this.options, "projectSnapshot", work.units, work.units);
    const database=await artifactSqliteDatabaseWithSchema(this.schema,this.sql,this.rows,this.options);
    this.completed = true;
    return database;
  }
}

function orderingStep(work: { units: number }): boolean {
  if (!Number.isSafeInteger(++work.units)) throw new ValueError("workLimit", "Artifact SQLite row ordering work overflow");
  return work.units % 256 === 0;
}
async function siftProjectionRows(rows: SqliteRow[], root: number, end: number, options: ArtifactSqliteOptions, work: { units: number }): Promise<void> {
  while (root < Math.floor(end / 2)) {
    if (orderingStep(work)) await artifactSqliteCheckpoint(options, "projectSnapshot", work.units, 0);
    let child = root * 2 + 1;
    if (child + 1 < end && rows[child]!.rowid < rows[child + 1]!.rowid) child++;
    if (rows[root]!.rowid >= rows[child]!.rowid) break;
    const row = rows[root]!;
    rows[root] = rows[child]!;
    rows[child] = row;
    root = child;
  }
}
async function orderProjectionRows(rows: SqliteRow[], options: ArtifactSqliteOptions, work: { units: number }): Promise<void> {
  let ordered = true;
  for (let index = 1; index < rows.length; index++) {
    if (orderingStep(work)) await artifactSqliteCheckpoint(options, "projectSnapshot", work.units, 0);
    if (rows[index - 1]!.rowid > rows[index]!.rowid) ordered = false;
  }
  if (!ordered) {
    for (let root = Math.floor(rows.length / 2) - 1; root >= 0; root--) await siftProjectionRows(rows, root, rows.length, options, work);
    for (let end = rows.length - 1; end > 0; end--) {
      if (orderingStep(work)) await artifactSqliteCheckpoint(options, "projectSnapshot", work.units, 0);
      const row = rows[0]!;
      rows[0] = rows[end]!;
      rows[end] = row;
      await siftProjectionRows(rows, 0, end, options, work);
    }
  }
  for (let index = 1; index < rows.length; index++) {
    if (orderingStep(work)) await artifactSqliteCheckpoint(options, "projectSnapshot", work.units, 0);
    if (rows[index - 1]!.rowid === rows[index]!.rowid) throw new ValueError("invalidValue", "Artifact SQLite row identities must be unique");
  }
}

/** 🔤️ Count Unicode scalar UTF-8 bytes without allocating an encoded copy. */
export function artifactSqliteTextBytes(value: string): number {
  if (typeof value !== "string") throw new ValueError("invalidValue", "Artifact SQLite value must be TEXT");
  return sqliteValueByteLength(value);
}

/** 🧵️ Measure scalar bytes while long text exposes its known Unicode traversal frontier. */
export async function artifactSqliteValueByteLengthControlled(value: SqliteValue, options: ArtifactSqliteOptions, phase: "projectSnapshot" | "reconstructSnapshot", baseBytes = 0): Promise<number> {
  if (typeof value !== "string" || value.length < 16_384) {
    const bytes = sqliteValueByteLength(value);
    artifactSqliteValueBudget(baseBytes + bytes, options);
    return bytes;
  }
  await artifactSqliteCheckpoint(options, phase, 0, value.length);
  artifactSqliteValueBudget(baseBytes + value.length, options);
  let bytes = 0, index = 0, checkpoint = 16_384;
  while (index < value.length) {
    const code = value.charCodeAt(index++);
    if (code >= 0xd800 && code <= 0xdbff) {
      const next = value.charCodeAt(index++);
      if (!(next >= 0xdc00 && next <= 0xdfff)) throw new ValueError("invalidValue", "Invalid SQLite UTF-8");
      bytes += 4;
    } else {
      if (code >= 0xdc00 && code <= 0xdfff) throw new ValueError("invalidValue", "Invalid SQLite UTF-8");
      bytes += code < 0x80 ? 1 : code < 0x800 ? 2 : 3;
    }
    if (index >= checkpoint) {
      artifactSqliteValueBudget(baseBytes + bytes, options);
      await primitiveCheckpoint(options, phase, index, value.length);
      checkpoint = index + 16_384;
    }
  }
  artifactSqliteValueBudget(baseBytes + bytes, options);
  await artifactSqliteCheckpoint(options, phase, value.length, value.length);
  return bytes;
}

/** 🧮️ Bound aggregate semantic cell bytes before allocating entity rows. */
export function artifactSqliteValueBudget(bytes: number, options: ArtifactSqliteOptions): void {
  const limit = options.maxValueBytes ?? 256 * 1024 * 1024;
  if (!Number.isSafeInteger(limit) || limit < 0 || !Number.isSafeInteger(bytes) || bytes > limit) throw new ValueError("ownershipLimit", "Artifact SQLite value limit");
}

/** ⏱️ Publish semantic entity progress and yield to cancellation at bounded intervals. */
export async function artifactSqliteCheckpoint(options: ArtifactSqliteOptions, phase: "projectSnapshot" | "reconstructSnapshot" | "indexTables", completed: number, total: number, yieldToEventLoop = true): Promise<void> {
  cancellation(options);
  options.onProgress?.({ phase, completed, total });
  cancellation(options);
  if (yieldToEventLoop && completed % 256 === 0) await hostContinuations.yieldContinuation();
  cancellation(options);
}

function cancellation(options: ArtifactSqliteOptions): void {
  if (options.signal?.aborted) throw new ValueError("canceled", "Artifact SQLite projection cancelled");
}
async function primitiveCheckpoint(options: ArtifactSqliteOptions, phase: "projectSnapshot" | "reconstructSnapshot", completed: number, total: number): Promise<void> {
  await artifactSqliteCheckpoint(options, phase, completed, total, false);
  await hostContinuations.yieldContinuation();
  cancellation(options);
}
async function projectionCells(key: bigint, cells: readonly SqliteValue[], options: SqliteOperation): Promise<SqliteValue[]> {
  const total = cells.reduce<number>((count, cell) => count + (cell instanceof Uint8Array ? cell.byteLength : 0), 0);
  const values: SqliteValue[] = [key];
  let completed = 0, checkpoint = 65_536;
  if (total >= 65_536) await primitiveCheckpoint(options, "projectSnapshot", 0, total);
  for (const cell of cells) {
    if (!(cell instanceof Uint8Array)) { values.push(cell); continue; }
    const owned = options.allocateBytes(cell.byteLength);
    for (let offset = 0; offset < cell.byteLength;) {
      const end = Math.min(cell.byteLength, offset + checkpoint - completed);
      owned.set(cell.subarray(offset, end), offset);
      completed += end - offset;
      offset = end;
      if (completed === checkpoint) {
        await primitiveCheckpoint(options, "projectSnapshot", completed, total);
        checkpoint += 65_536;
      }
    }
    values.push(owned);
  }
  if (total >= 65_536) await artifactSqliteCheckpoint(options, "projectSnapshot", total, total, false);
  return values;
}

/** 🗂️ Populate each handcrafted table in schema order. */
export async function artifactSqliteDatabase(sql:string,rows:readonly(readonly SqliteRow[])[],options:ArtifactSqliteOptions):Promise<SqliteDatabase>{
 const operation=sqliteOperation(options),schema=await parseSqliteDatabaseSchemaControlled(sql,operation);
 return artifactSqliteDatabaseWithSchema(schema,sql,rows,operation);
}
async function artifactSqliteDatabaseWithSchema(schema:SqliteDatabase,sql:string,rows:readonly(readonly SqliteRow[])[],operation:SqliteOperation):Promise<SqliteDatabase>{
 if(schema.tables.length!==rows.length)throw new ValueError("invalidValue","Artifact SQLite table count mismatch");
 const tables:SqliteDatabase["tables"][number][]=[];for(let index=0;index<schema.tables.length;index++){tables.push({...schema.tables[index]!,rows:rows[index]!});if((index+1)%256===0)await artifactSqliteCheckpoint(operation,"projectSnapshot",index+1,schema.tables.length);}
 const database={tables};await validateSqliteDatabaseSchemaControlled(database,sql,operation);return database;
}

/** 📋️ Locate typed artifact tables after verifying their complete schema and row identities. */
export async function artifactSqliteTables(database: SqliteDatabase, sql: string, options: ArtifactSqliteOptions): Promise<readonly (readonly SqliteRow[])[]> {
  const operation=sqliteOperation(options);options=operation;
  await artifactSqliteCheckpoint(operation, "reconstructSnapshot", 0, 0);
  if(database.tables.length>(operation.maxTables??4096))throw new ValueError("workLimit","Artifact SQLite table limit");
  let bytes=0,rows=0,total=0;
  for(let index=0;index<database.tables.length;index++){total+=database.tables[index]!.rows.length;if(total>(operation.maxRows??1_000_000))throw new ValueError("workLimit","Artifact SQLite row limit");if((index+1)%256===0)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",index+1,database.tables.length);}
  for (const table of database.tables) {
    for (const row of table.rows) {
      for (const value of row.values) { bytes += await artifactSqliteValueByteLengthControlled(value, options, "reconstructSnapshot", bytes); }
      if (++rows % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", rows, total);
    }
  }
  artifactSqliteValueBudget(bytes, options);
  const schema=await validateSqliteDatabaseSchemaControlled(database, sql, options);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", rows, total);
  const index=await ArtifactSqliteTableIndex.create(database.tables,operation);
  const ordered:(readonly SqliteRow[])[]=[];
  for(let at=0;at<schema.tables.length;at++){const found=await index.find(schema.tables[at]!.name);if(found<0)throw new ValueError("invariantViolated","Validated artifact SQLite table is absent");ordered.push(database.tables[found]!.rows);if((at+1)%256===0)await artifactSqliteCheckpoint(operation,"indexTables",at+1,schema.tables.length);}
  return ordered;
}

/** 🔢️ Require a signed INTEGER cell. */
export function artifactSqliteInteger(row: SqliteRow, index: number): bigint {
  const value = row.values[index];
  if (typeof value !== "bigint") throw new ValueError("invalidValue", "Artifact SQLite cell must be INTEGER");
  return value;
}
/** 🔤️ Require a TEXT cell. */
export function artifactSqliteText(row: SqliteRow, index: number): string {
  const value = row.values[index];
  if (typeof value !== "string") throw new ValueError("invalidValue", "Artifact SQLite cell must be TEXT");
  return value;
}
/** 📐️ Decode a finite REAL-affinity numeric scalar. */
export function artifactSqliteReal(row: SqliteRow, index: number): number {
  const value = row.values[index];
  if (typeof value !== "number" && typeof value !== "bigint") throw new ValueError("invalidValue", "Artifact SQLite coordinate must be REAL");
  const number = Number(value);
  if (!Number.isFinite(number)) throw new ValueError("invalidValue", "Artifact SQLite coordinate must be finite");
  return number;
}
/** 🔘️ Decode an explicitly constrained boolean INTEGER. */
export function artifactSqliteBoolean(row: SqliteRow, index: number): boolean {
  const value = artifactSqliteInteger(row, index);
  if (value !== 0n && value !== 1n) throw new ValueError("invalidValue", "Artifact SQLite boolean must be 0 or 1");
  return value === 1n;
}
/** 📍️ Require one identity-1 document row. */
export function artifactSqliteDocument(rows: readonly SqliteRow[]): SqliteRow {
  if (rows.length !== 1 || rows[0]!.rowid !== 1n || artifactSqliteInteger(rows[0]!, 0) !== 1n) throw new ValueError("invalidValue", "Artifact SQLite requires one document with identifier 1");
  return rows[0]!;
}
/** 🔗️ Require a relationship to the identity-1 document. */
export function artifactSqliteDocumentReference(row: SqliteRow, index: number): void {
  if (artifactSqliteInteger(row, index) !== 1n) throw new ValueError("invalidValue", "Artifact SQLite entity has an unknown document");
}
/** 🧮️ Order collection entities by explicit, contiguous ordinals. */
export function artifactSqliteOrderedRows(rows: readonly SqliteRow[], column: number): SqliteRow[] {
  const ordered = new Array<SqliteRow>(rows.length);
  for (const row of rows) {
    const ordinal = artifactSqliteInteger(row, column);
    if (ordinal < 0n || ordinal >= BigInt(rows.length) || ordered[Number(ordinal)] !== undefined) throw new ValueError("invalidValue", "Artifact SQLite ordinals must be contiguous");
    ordered[Number(ordinal)] = row;
  }
  return ordered;
}

/** 🚦️ Bound and cancel ordered relationship reconstruction before allocating its result. */
export async function artifactSqliteOrderedRowsControlled(rows: readonly SqliteRow[], column: number, options: ArtifactSqliteOptions = {}): Promise<SqliteRow[]> {
  if (rows.length > (options.maxRows ?? 1_000_000)) throw new ValueError("workLimit", "Artifact SQLite row limit");
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, rows.length, rows.length >= 256);
  const ordered = new Array<SqliteRow>(rows.length);
  for (let index = 0; index < rows.length; index++) {
    const row = rows[index]!;
    const ordinal = artifactSqliteInteger(row, column);
    if (ordinal < 0n || ordinal >= BigInt(rows.length) || ordered[Number(ordinal)] !== undefined) throw new ValueError("invalidValue", "Artifact SQLite ordinals must be contiguous");
    ordered[Number(ordinal)] = row;
    if ((index + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", index + 1, rows.length);
  }
  return ordered;
}
