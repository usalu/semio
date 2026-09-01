/** 🔺️ JpgDiff schema facet — mirrors 🦀️component.rs field-for-field. No full-replace slot:
 * JFIF/SOF/DRI/quality fields are sparse top-level patches (tri-state `T | null` for the
 * genuinely optional ones — `null` means "removed/cleared"); `frame` is a `modify`/`replace`
 * change (a decode-status `undefined`<->present transition is a "kind change"); `quantTables`/
 * `huffmanTables`/`frame.components` are id-keyed removed/modified/added triples; `otherSegments`
 * is an index-keyed triple. */

import type {
  JfifDensityUnits, JfifThumbnail, JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass,
  JpgHuffmanTable, JpgQuantTable, JpgSegment,
} from '../📸️snapshot/🟦️component.ts';

export interface JpgComponentDiff { hSampling?: number; vSampling?: number; quantTableId?: number }
export interface JpgComponentModified { id: number; diff: JpgComponentDiff }
export interface JpgComponentAdded { index: number; item: JpgFrameComponent }
export interface JpgComponentsDiff {
  removed?: number[];
  modified?: JpgComponentModified[];
  added?: JpgComponentAdded[];
}

export interface JpgFrameFieldsDiff {
  precision?: number;
  width?: number;
  height?: number;
  components?: JpgComponentsDiff;
}

/** 🌲️ `frame`'s change shape — `modify` when both base/next have a frame; `replace` on a
 * decode-status "kind change" (`undefined`<->present). */
export type JpgFrameChange =
  | { change: 'modify' } & JpgFrameFieldsDiff
  | { change: 'replace'; frame?: JpgFrameHeader };

export interface JpgQuantTableDiff { precision?: number; values?: number[] }
export interface JpgQuantTableModified { id: number; diff: JpgQuantTableDiff }
export interface JpgQuantTableAdded { index: number; item: JpgQuantTable }
export interface JpgQuantTablesDiff {
  removed?: number[];
  modified?: JpgQuantTableModified[];
  added?: JpgQuantTableAdded[];
}

/** 🔑️ Compound identity for `huffmanTables` — DC id=0 and AC id=0 are different tables. */
export interface JpgHuffmanTableKey { class: JpgHuffmanClass; id: number }
export interface JpgHuffmanTableDiff { bits?: number[]; values?: number[] }
export interface JpgHuffmanTableModified { key: JpgHuffmanTableKey; diff: JpgHuffmanTableDiff }
export interface JpgHuffmanTableAdded { index: number; item: JpgHuffmanTable }
export interface JpgHuffmanTablesDiff {
  removed?: JpgHuffmanTableKey[];
  modified?: JpgHuffmanTableModified[];
  added?: JpgHuffmanTableAdded[];
}

export interface JpgSegmentDiff { marker?: number; data?: number[] }
export interface JpgSegmentModified { index: number; diff: JpgSegmentDiff }
export interface JpgSegmentAdded { index: number; item: JpgSegment }
export interface JpgOtherSegmentsDiff {
  removed?: number[];
  modified?: JpgSegmentModified[];
  added?: JpgSegmentAdded[];
}

/** 🔺️ Sparse diff for `stdio.jpg`. Every field present = changed to a value; tri-state fields
 * (`reEncodeQuality`/`jfifThumbnail`/`restartInterval`) use `null` for "cleared". */
export interface JpgDiff {
  width?: number;
  height?: number;
  pixels?: number[];
  reEncodeQuality?: number | null;
  jfifVersion?: [number, number];
  jfifDensityUnits?: JfifDensityUnits;
  jfifXDensity?: number;
  jfifYDensity?: number;
  jfifThumbnail?: JfifThumbnail | null;
  frame?: JpgFrameChange;
  sofMarker?: number;
  arithmetic?: boolean;
  quantTables?: JpgQuantTablesDiff;
  huffmanTables?: JpgHuffmanTablesDiff;
  restartInterval?: number | null;
  otherSegments?: JpgOtherSegmentsDiff;
}
