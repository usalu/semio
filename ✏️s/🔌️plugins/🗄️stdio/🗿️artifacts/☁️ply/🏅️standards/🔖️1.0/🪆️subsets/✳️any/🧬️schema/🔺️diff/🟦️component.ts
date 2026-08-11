/** 🔺️ PlyDiff schema — handcrafted sparse diff mirroring the Rust `PlyDiff` shape 1:1. Two
 * collection levels nest: `elements` (name-keyed) → each modified element's `rows` (index-keyed). */

import type { PlyElement, PlyFormat, PlyProperty, PlyRow, PlyValue } from '../📸️snapshot/🟦️component.ts';

/** 🔣️ One changed cell, keyed by the owning element's property NAME. */
export interface PlyRowFieldChange {
  name: string;
  value: PlyValue;
}

/** 🔺️ Sparse per-property patch for one row. */
export interface PlyRowDiff {
  fields?: PlyRowFieldChange[];
}

/** 📦️ One `rows.modified[]` entity — `index` is the row's position in BASE. */
export interface PlyRowModified {
  index: number;
  diff: PlyRowDiff;
}

/** 📦️ One `rows.added[]` entity — `index` is the row's position in the FINAL sequence. */
export interface PlyRowAdded {
  index: number;
  row: PlyRow;
}

/** 🔺️ Index-keyed removed/modified/added triple over one element's `rows`. */
export interface PlyRowsDiff {
  removed?: number[];
  modified?: PlyRowModified[];
  added?: PlyRowAdded[];
}

/** 🔺️ Sparse per-field patch for one element. `properties` is a weak value-list — whole-vec
 * replaced, never sub-diffed. */
export interface PlyElementDiff {
  properties?: PlyProperty[];
  rows?: PlyRowsDiff;
}

/** 📦️ One `elements.modified[]` entity — `name` is the element's identity. */
export interface PlyElementModified {
  name: string;
  diff: PlyElementDiff;
}

/** 📦️ One `elements.added[]` entity — `index` is the position in the FINAL sequence. */
export interface PlyElementAdded {
  index: number;
  element: PlyElement;
}

/** 🔺️ Sparse name-keyed `elements` triple. */
export interface PlyElementsDiff {
  removed?: string[];
  modified?: PlyElementModified[];
  added?: PlyElementAdded[];
}

/** 🔺️ Diff for `stdio.ply`. `schema` is an identity field and never appears here. */
export interface PlyDiff {
  format?: PlyFormat;
  comments?: string[];
  elements?: PlyElementsDiff;
}
