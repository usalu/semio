/** 🧰️ The explicit finite scalar and list admission of PDF native JSON that every native JSON converter builds on. */

import type { PdfStreamFilter } from "../../../../../🧬️schema/📸️snapshot/🧩️cos/🟦️.ts";
import { binary64 } from "@semio-tech/framework/value/ieee754";
import type { Binary64 } from "@semio-tech/framework/value/ieee754";

export function record(value: unknown): Record<string, unknown> { if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("PDF native JSON requires an object"); return value as Record<string, unknown>; }

export function array(value: unknown): unknown[] { if (!Array.isArray(value)) throw new Error("PDF native JSON requires an array"); return value; }

export function text(value: unknown): string { if (typeof value !== "string") throw new Error("PDF native JSON requires text"); return value; }

export function boolean(value: unknown): boolean { if (typeof value !== "boolean") throw new Error("PDF native JSON requires a boolean"); return value; }

export function integer(value: unknown): number { if (typeof value !== "number" || !Number.isSafeInteger(value)) throw new Error("PDF native JSON integer cannot be represented exactly"); return value; }

export function integers(value: unknown): number[] { return array(value).map(integer); }

export function real(value: unknown): Binary64 { if (typeof value !== "number" || !Number.isFinite(value)) throw new Error("PDF native JSON REAL must be finite"); return binary64(value); }

export function reals(value: unknown): Binary64[] { return array(value).map(real); }

export function optionalReals(value: unknown): Binary64[] | null { return value == null ? null : reals(value); }

export function three(value: unknown): [Binary64, Binary64, Binary64] { const values = reals(value); if (values.length !== 3) throw new Error("PDF native JSON requires three components"); return [values[0]!, values[1]!, values[2]!]; }

export function four(value: unknown): [Binary64, Binary64, Binary64, Binary64] { const values = reals(value); if (values.length !== 4) throw new Error("PDF native JSON requires four components"); return [values[0]!, values[1]!, values[2]!, values[3]!]; }

export function nine(value: unknown): [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] { const values = reals(value); if (values.length !== 9) throw new Error("PDF native JSON requires nine components"); return [values[0]!, values[1]!, values[2]!, values[3]!, values[4]!, values[5]!, values[6]!, values[7]!, values[8]!]; }

export function filters(value: unknown): PdfStreamFilter[] {
  return array(value).map(item => {
    const row = record(item); const kind = text(row.kind);
    const predictor = (): { predictor: number; colors: number; bitsPerComponent: number; columns: number } | null => { if (row.predictor == null) return null; const value = record(row.predictor); return { predictor: integer(value.predictor), colors: integer(value.colors), bitsPerComponent: integer(value.bitsPerComponent), columns: integer(value.columns) }; };
    switch (kind) {
      case "flate": return { kind, predictor: predictor() };
      case "lzw": return { kind, predictor: predictor(), earlyChange: boolean(row.earlyChange) };
      case "asciiHex": case "ascii85": case "runLength": case "jpx": return { kind };
      case "dct": return { kind, colorTransform: row.colorTransform == null ? null : integer(row.colorTransform) };
      case "jbig2": return { kind, globals: row.globals == null ? null : integers(row.globals) };
      case "crypt": return { kind, name: row.name == null ? null : text(row.name) };
      case "ccitt": { const value = record(row.parameters); return { kind, parameters: { k: integer(value.k ?? 0), columns: integer(value.columns ?? 1728), rows: integer(value.rows ?? 0), blackIs1: boolean(value.blackIs1 ?? false), encodedByteAlign: boolean(value.encodedByteAlign ?? false), endOfLine: boolean(value.endOfLine ?? false), endOfBlock: boolean(value.endOfBlock ?? true), damagedRowsBeforeError: integer(value.damagedRowsBeforeError ?? 0) } }; }
      default: throw new Error("Unknown PDF native JSON filter");
    }
  });
}
