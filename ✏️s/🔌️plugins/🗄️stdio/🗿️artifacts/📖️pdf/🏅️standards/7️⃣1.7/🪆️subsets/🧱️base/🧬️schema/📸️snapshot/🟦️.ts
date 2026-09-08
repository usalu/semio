/** 🧬️ PdfSnapshot (1.7) schema — logical object-graph model mirroring Rust 1:1.
 *  `objects` is the full semantic indirect-object graph; `pages` is the
 *  resolved, editable view; `trailer` is the trailer dictionary (same shape as a `Dict`). */

/** 🔗️ An indirect-object reference `N G R` -- also the `objects` collection's diff key. */
export interface ObjRef {
  num: number;
  gen: number;
}

/** 🧩 One `key`/`value` pair of a PDF dictionary (order-preserving array, not a map). */
export interface PdfDictEntry {
  key: string;
  value: PdfObject;
}

export interface PdfDecimal {
  negative: boolean;
  coefficient: string;
  scale: number;
}

export interface PdfPredictor {
  predictor: number;
  colors: number;
  bitsPerComponent: number;
  columns: number;
}

export type PdfStreamFilter =
  | { kind: 'flate'; predictor?: PdfPredictor }
  | { kind: 'asciiHex' }
  | { kind: 'ascii85' }
  | { kind: 'runLength' };

/** 🎯 Parsed PDF COS objects. Stream data is decoded; filters are typed logical concepts. */
export type PdfObject =
  | { kind: 'null' }
  | { kind: 'bool'; value: boolean }
  | { kind: 'int'; value: number }
  | { kind: 'real'; value: PdfDecimal }
  | { kind: 'str'; value: number[] }
  | { kind: 'name'; value: string }
  | { kind: 'array'; value: PdfObject[] }
  | { kind: 'dict'; value: PdfDictEntry[] }
  | { kind: 'ref'; value: ObjRef }
  | { kind: 'stream'; dict: PdfDictEntry[]; data: number[]; filters: PdfStreamFilter[] };

/** 🗄️ One `N G obj ... endobj` indirect object, keyed by `id`. */
export interface PdfIndirectObject {
  id: ObjRef;
  value: PdfObject;
}

/** 📄️ One resolved page -- inherited `/Resources`/`/MediaBox`/`/CropBox`/`/Rotate` already
 *  applied, text already extracted from its content stream(s). */
export interface PdfPage {
  mediaBox: [number, number, number, number];
  cropBox?: [number, number, number, number];
  rotate: number;
  text: string;
}

/** 📇️ Document `/Info` dictionary. */
export interface PdfInfo {
  title?: string;
  author?: string;
  subject?: string;
  keywords?: string;
  creator?: string;
  producer?: string;
}

/** 🧬️ `stdio.pdf` (1.7) persistent snapshot. */
export interface PdfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ declaredVersion: string;
  /** @state artifact */ pages: PdfPage[];
  /** @state artifact */ info: PdfInfo;
  /** @state artifact */ objects: PdfIndirectObject[];
  /** @state artifact */ trailer: PdfDictEntry[];
}
