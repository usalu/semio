/** 🔺️ PdfDiff (1.7) — handcrafted sparse diff mirroring the Rust `PdfDiff` shape 1:1. `pages` is
 *  an index-keyed triple of flat `PdfPageDiff` patches; `objects` is an `ObjRef`-keyed (the
 *  `(id,gen)` pair) triple of recursive `PdfValueDiff` patches mirroring `PdfObject`'s shape
 *  (`Replace` on node-KIND change, direct field/collection diff when the kind is stable); `trailer`
 *  reuses `PdfDictDiff` verbatim (trailer is itself a Dict-shaped structure). No
 *  `snapshot?: PdfSnapshot` full-replace slot anywhere. */

import type { ObjRef, PdfDecimal, PdfDictEntry, PdfInfo, PdfObject, PdfPage, PdfStreamFilter } from '../📸️snapshot/🟦️component.ts';

/** 📄️ Sparse per-field patch for one `PdfPage` (weak entity -- flat fields only). `cropBox` is
 *  tri-state: absent = unchanged, `null` = cleared, a box = set. */
export interface PdfPageDiff {
  mediaBox?: [number, number, number, number];
  cropBox?: [number, number, number, number] | null;
  rotate?: number;
  text?: string;
}

export interface PdfPageModified {
  index: number;
  diff: PdfPageDiff;
}
export interface PdfPageAdded {
  index: number;
  page: PdfPage;
}
/** 📦️ Index-keyed `pages` triple. */
export interface PdfPagesDiff {
  removed?: number[];
  modified?: PdfPageModified[];
  added?: PdfPageAdded[];
}

export interface PdfDictModified {
  key: string;
  diff: PdfValueDiff;
}
export interface PdfDictAdded {
  index: number;
  key: string;
  item: PdfObject;
}
/** 📦️ Name-keyed `Dict`/`Stream.dict`/`trailer` triple (reused verbatim for the top-level
 *  `PdfDiff.trailer` field -- same shape, same semantics). */
export interface PdfDictDiff {
  removed?: string[];
  modified?: PdfDictModified[];
  added?: PdfDictAdded[];
}

export interface PdfArrayModified {
  index: number;
  diff: PdfValueDiff;
}
export interface PdfArrayAdded {
  index: number;
  item: PdfObject;
}
/** 📦️ Index-keyed `Array` triple. */
export interface PdfArrayDiff {
  removed?: number[];
  modified?: PdfArrayModified[];
  added?: PdfArrayAdded[];
}

/** 🔺️ Recursive logical diff mirroring `PdfObject`'s shape. */
export type PdfValueDiff =
  | { kind: 'replace'; value: PdfObject }
  | { kind: 'bool'; value: boolean }
  | { kind: 'int'; value: number }
  | { kind: 'real'; value: PdfDecimal }
  | { kind: 'str'; value: number[] }
  | { kind: 'name'; value: string }
  | { kind: 'ref'; value: ObjRef }
  | { kind: 'array'; diff: PdfArrayDiff }
  | { kind: 'dict'; diff: PdfDictDiff }
  | { kind: 'stream'; dict?: PdfDictDiff; data?: number[]; filters?: PdfStreamFilter[] };

export interface PdfObjectModified {
  id: ObjRef;
  diff: PdfValueDiff;
}
export interface PdfObjectAdded {
  index: number;
  id: ObjRef;
  value: PdfObject;
}
/** 📦️ `(id,gen)`-keyed `objects` triple. */
export interface PdfObjectsDiff {
  removed?: ObjRef[];
  modified?: PdfObjectModified[];
  added?: PdfObjectAdded[];
}

/** 🧭️ One step of a `NodePath`-style address into ONE object's `PdfObject` tree (used by
 *  `setDictEntry`/`removeDictEntry` mutations). Only `path == []` can address a `Stream`'s dict
 *  (a raw Stream can only ever be an indirect object's OWN top-level value per ISO 32000-1). */
export type PdfPathSegment =
  | { kind: 'arrayIndex'; index: number }
  | { kind: 'dictKey'; key: string };

/** 🔺️ Diff for `stdio.pdf.1.7`. `schema` is an identity field and never appears here. `info` is
 *  a WEAK value struct (whole-value replaced, never sub-diffed). */
export interface PdfDiff {
  /** @state artifact */ declaredVersion?: string;
  /** @state artifact */ info?: PdfInfo;
  /** @state artifact */ pages?: PdfPagesDiff;
  /** @state artifact */ objects?: PdfObjectsDiff;
  /** @state artifact */ trailer?: PdfDictDiff;
}
