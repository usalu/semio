/** 🧬️ PdfSnapshot (1.7) schema — real object-graph model mirroring the Rust `PdfSnapshot` shape
 *  1:1. `objects` is the FULL raw indirect-object graph (lossless retention); `pages` is the
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

/** 🎯 A parsed PDF object -- the full COS object grammar (ISO 32000-1 §7.3), including streams.
 *  `str` is a byte string (not necessarily UTF-8), hence `number[]`. `stream.rawFilter` present
 *  means `data` is still filter-encoded verbatim (an unsupported filter we deliberately don't
 *  decode); absent means `data` has already been fully filter-decoded. */
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
  | { kind: 'stream'; dict: PdfDictEntry[]; data: number[]; rawFilter?: string };

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
