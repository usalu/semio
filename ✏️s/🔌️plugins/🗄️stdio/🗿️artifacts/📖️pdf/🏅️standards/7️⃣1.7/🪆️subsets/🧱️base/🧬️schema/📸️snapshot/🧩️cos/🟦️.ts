/** 🧩️ PDF 1.7 snapshot types of COS objects, indirect references and stream filters. */
import type { PdfDate } from "../📆️date/🟦️.ts";

export interface PdfDictEntry {
  key: string;
  value: PdfObject;
}

export type PdfObject =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; value: bigint }
  | ({ kind: "real" } & PdfDecimal)
  | { kind: "str"; value: number[] }
  | { kind: "text"; value: string }
  | { kind: "date"; value: PdfDate }
  | { kind: "name"; value: string }
  | { kind: "array"; value: PdfObject[] }
  | { kind: "dict"; value: PdfDictEntry[] }
  | ({ kind: "ref" } & ObjRef)
  | { kind: "stream"; dict: PdfDictEntry[]; data: number[]; filters: PdfStreamFilter[] };

export type PdfStreamFilter =
  | { kind: "flate"; predictor?: PdfPredictor | null }
  | { kind: "lzw"; predictor?: PdfPredictor | null; earlyChange: boolean }
  | { kind: "asciiHex" }
  | { kind: "ascii85" }
  | { kind: "runLength" }
  | { kind: "dct"; colorTransform?: number | null }
  | { kind: "jpx" }
  | { kind: "ccitt"; parameters: PdfCcittParameters }
  | { kind: "jbig2"; globals?: number[] | null }
  | { kind: "crypt"; name?: string | null };

export interface PdfCcittParameters {
  k?: number;
  columns?: number;
  rows?: number;
  blackIs1?: boolean;
  encodedByteAlign?: boolean;
  endOfLine?: boolean;
  endOfBlock?: boolean;
  damagedRowsBeforeError?: number;
}

export interface PdfPredictor {
  predictor: number;
  colors: number;
  bitsPerComponent: number;
  columns: number;
}

export interface ObjRef {
  num: number;
  gen: number;
}

export interface PdfDecimal {
  negative: boolean;
  coefficient: string;
  scale: number;
}
