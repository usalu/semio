/** 🔢️ Exact owned IEEE identities and literal-column SQLite companions. */
import type { SqliteRow, SqliteValue } from "../🟦️.ts";

export interface Binary64 { readonly bits: bigint }
export interface Binary32 { readonly bits: number }
export interface Ieee754Column { readonly index: number; readonly width: 32 | 64 }
export type Ieee754Cell = SqliteValue | Binary64 | Binary32;
const view = new DataView(new ArrayBuffer(8));

/** 🛂️ Validate the complete unsigned binary64 word without coercion. */
export function parseBinary64(value: unknown): Binary64 {
  if (value === null || typeof value !== "object" || !("bits" in value) || typeof value.bits !== "bigint" || value.bits < 0n || value.bits > 18446744073709551615n) throw new Error("binary64 requires an unsigned 64-bit word");
  return { bits: value.bits };
}
/** 🛂️ Validate the complete unsigned binary32 word without coercion. */
export function parseBinary32(value: unknown): Binary32 {
  if (value === null || typeof value !== "object" || !("bits" in value) || typeof value.bits !== "number" || !Number.isInteger(value.bits) || value.bits < 0 || value.bits > 4294967295) throw new Error("binary32 requires an unsigned 32-bit word");
  return { bits: value.bits };
}
/** 🧮️ Capture an ordinary JavaScript numeric value as its owned binary64 word. */
export function binary64(value: number): Binary64 {
  if (typeof value !== "number") throw new Error("binary64 numeric input requires a number");
  view.setFloat64(0,value); return { bits: view.getBigUint64(0) };
}
/** 🔢️ Capture a JavaScript numeric value at the native binary32 width. */
export function binary32(value: number): Binary32 {
  if (typeof value !== "number") throw new Error("binary32 numeric input requires a number");
  view.setFloat32(0,value); return { bits: view.getUint32(0) };
}
/** 🧮️ Derive a numeric query value; exact identity remains the owned word. */
export function binary64Value(value: Binary64): number { view.setBigUint64(0,parseBinary64(value).bits); return view.getFloat64(0); }
/** 🔢️ Derive a numeric query value at native binary32 width. */
export function binary32Value(value: Binary32): number { view.setUint32(0,parseBinary32(value).bits); return view.getFloat32(0); }
function classify(bits: bigint, width: 32 | 64): string {
  const fraction = width === 64 ? 52n : 23n;
  const exponentMask = width === 64 ? 2047n : 255n;
  if (((bits >> fraction) & exponentMask) !== exponentMask) return "finite";
  if ((bits & ((1n << fraction) - 1n)) !== 0n) return "nan";
  return (bits >> BigInt(width - 1)) === 0n ? "positiveInfinity" : "negativeInfinity";
}
/** 🧮️ Count one authored scalar's query and companion data before row allocation. */
export function ieee754CellByteLength(value:Binary64|Binary32,width:32|64):number{
  const bits=width===64?parseBinary64(value).bits:BigInt(parseBinary32(value).bits);
  const kind=classify(bits,width);
  return(kind==="nan"?8:16)+kind.length;
}
function checkColumns(size: number, columns: readonly Ieee754Column[]): void {
  const seen = new Set<number>();
  for (const column of columns) {
    if (!Number.isInteger(column.index) || column.index < 1 || column.index >= size || (column.width !== 32 && column.width !== 64) || seen.has(column.index)) throw new Error("invalid authored IEEE column position or width");
    seen.add(column.index);
  }
}
/** 📤️ Append named companions at caller-authored positions, preserving optional absence. */
export function encodeIeee754Cells(cells: readonly Ieee754Cell[], columns: readonly Ieee754Column[], maxColumns = 1024): SqliteValue[] {
  checkColumns(cells.length,columns);
  if (cells.length + columns.length * 2 > maxColumns) throw new Error("IEEE SQLite column limit");
  const result = cells.map(value => value !== null && typeof value === "object" && !(value instanceof Uint8Array) ? null : value) as SqliteValue[];
  const selected = new Set(columns.map(column => column.index));
  for (const [index,value] of cells.entries()) if (value !== null && typeof value === "object" && !(value instanceof Uint8Array) && !selected.has(index)) throw new Error("owned scalar has no authored IEEE column");
  for (const column of columns) {
    const value = cells[column.index];
    if (value === null) { result.push(null,null); continue; }
    const bits = column.width === 64 ? parseBinary64(value).bits : BigInt(parseBinary32(value).bits);
    const kind = classify(bits,column.width);
    result[column.index] = kind === "nan" ? null : column.width === 64 ? binary64Value({ bits }) : binary32Value({ bits: Number(bits) });
    result.push(column.width === 64 ? BigInt.asIntN(64,bits) : bits,kind);
  }
  return result;
}
function read(row: SqliteRow, index: number, columns: readonly Ieee754Column[], width: 32 | 64): bigint {
  const original = row.values.length - columns.length * 2;
  checkColumns(original,columns);
  const slot = columns.findIndex(column => column.index === index && column.width === width);
  if (slot < 0) throw new Error("scalar field has no authored IEEE width");
  const position = original + slot * 2;
  const bits = row.values[position];
  if (typeof bits !== "bigint" || (width === 32 && (bits < 0n || bits > 4294967295n)) || (width === 64 && (bits < -9223372036854775808n || bits > 9223372036854775807n))) throw new Error("invalid IEEE integer companion");
  const word = width === 64 ? BigInt.asUintN(64,bits) : bits;
  const kind = classify(word,width);
  if (row.values[position + 1] !== kind) throw new Error("IEEE numeric class disagrees with native bits");
  const value = row.values[index];
  const query = width === 64 ? binary64Value({ bits: word }) : binary32Value({ bits: Number(word) });
  if (kind === "nan" ? value !== null : (typeof value !== "number" && typeof value !== "bigint") || Number(value) !== query) throw new Error("query REAL disagrees with native IEEE bits");
  return word;
}
/** 📥️ Restore binary64 identity independently of JavaScript NaN canonicalization. */
export function readBinary64(row: SqliteRow, index: number, columns: readonly Ieee754Column[]): Binary64 { return { bits: read(row,index,columns,64) }; }
/** 📥️ Restore binary32 identity without widening through a JavaScript number. */
export function readBinary32(row: SqliteRow, index: number, columns: readonly Ieee754Column[]): Binary32 { return { bits: Number(read(row,index,columns,32)) }; }
/** 🫥️ Distinguish absent optional identity from NaN's NULL numeric query. */
export function ieee754IsNull(row: SqliteRow,index: number,columns: readonly Ieee754Column[]): boolean {
  const original = row.values.length - columns.length * 2;
  checkColumns(original,columns);
  const slot = columns.findIndex(column => column.index === index);
  if (slot < 0) throw new Error("scalar field has no authored IEEE column");
  return row.values[index] === null && row.values[original + slot * 2] === null && row.values[original + slot * 2 + 1] === null;
}
