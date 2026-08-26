/** 🔺️ TiffDiff schema facet — mirrors 🦀️component.rs field-for-field. No full-replace slot:
 * `ifds` is an index-keyed removed/modified/added triple; within a modified IFD, `entries` is
 * a TAG-ID-keyed triple (`tag`, not array index — a `TiffTag` is a weak value, so
 * modified/added carry the whole new tag). */

import type { TiffByteOrder, TiffFieldType, TiffIfd, TiffValues } from '../📸️snapshot/🟦️component.ts';

export interface TiffTagModified { tag: number; kind: TiffFieldType; values: TiffValues }
export interface TiffTagAdded { tag: number; kind: TiffFieldType; values: TiffValues }

/** Tag-id-keyed `entries` triple for one IFD. */
export interface TiffTagsDiff {
  removed?: number[];
  modified?: TiffTagModified[];
  added?: TiffTagAdded[];
}

/** One IFD's own delta: the recursive tag triple plus a whole-value slot for its raw strip payload. */
export interface TiffIfdDiff { entries: TiffTagsDiff; pixels?: number[] }
export interface TiffIfdModified { index: number; diff: TiffIfdDiff }
export interface TiffIfdAdded { index: number; ifd: TiffIfd }

/** Index-keyed `ifds` triple (TIFF's IFD chain is positional). */
export interface TiffIfdsDiff {
  removed?: number[];
  modified?: TiffIfdModified[];
  added?: TiffIfdAdded[];
}

/** 🔺️ Sparse diff for `stdio.tiff`. Every field present = changed to a value. No tri-state
 * fields at this level — `byteOrder`/`pixels` are always-present scalars, `ifds` is the only
 * collection. */
export interface TiffDiff {
  byteOrder?: TiffByteOrder;
  ifds?: TiffIfdsDiff;
  pixels?: number[];
}
