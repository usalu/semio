/** 🔢️ Exact native IEEE scalar identities and ordinary numeric capture. */
import { ValueError } from "../⚠️refusal/🟦️.ts";
export interface Binary64 { readonly bits: bigint }
export interface Binary32 { readonly bits: number }
const view = new DataView(new ArrayBuffer(8));

/** 🛂️ Validate the complete unsigned binary64 word without coercion. */
export function parseBinary64(value: unknown): Binary64 {
  if (value === null || typeof value !== "object" || !("bits" in value) || typeof value.bits !== "bigint" || value.bits < 0n || value.bits > 18446744073709551615n) throw new ValueError("invalidValue","binary64 requires an unsigned 64-bit word");
  return { bits: value.bits };
}
/** 🛂️ Validate the complete unsigned binary32 word without coercion. */
export function parseBinary32(value: unknown): Binary32 {
  if (value === null || typeof value !== "object" || !("bits" in value) || typeof value.bits !== "number" || !Number.isInteger(value.bits) || value.bits < 0 || value.bits > 4294967295) throw new ValueError("invalidValue","binary32 requires an unsigned 32-bit word");
  return { bits: value.bits };
}
/** 🧮️ Capture an ordinary JavaScript numeric value as its owned binary64 word. */
export function binary64(value: number): Binary64 {
  if (typeof value !== "number") throw new ValueError("invalidValue","binary64 numeric input requires a number");
  view.setFloat64(0,value); return { bits: view.getBigUint64(0) };
}
/** 🔢️ Capture a JavaScript numeric value at the native binary32 width. */
export function binary32(value: number): Binary32 {
  if (typeof value !== "number") throw new ValueError("invalidValue","binary32 numeric input requires a number");
  view.setFloat32(0,value); return { bits: view.getUint32(0) };
}
/** 🧮️ Derive a numeric query value; exact identity remains the owned word. */
export function binary64Value(value: Binary64): number { view.setBigUint64(0,parseBinary64(value).bits); return view.getFloat64(0); }
/** 🔢️ Derive a numeric query value at native binary32 width. */
export function binary32Value(value: Binary32): number { view.setUint32(0,parseBinary32(value).bits); return view.getFloat32(0); }
