/** 🔺️ PngDiff schema facet — mirrors 🦀️component.rs field-for-field. No full-replace slot:
 * IHDR/tRNS/ancillary fields are sparse top-level patches (tri-state `T | null` for the
 * genuinely optional ones — `null` means "removed"); `plte`/`textChunks`/`chunkOrder`/
 * `unknownChunks` are index-keyed removed/modified/added triples. */

import type {
  PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims,
  PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency,
} from '../📸️snapshot/🟦️component.ts';

export interface PngPlteEntryModified { index: number; rgb: PngRgb }
export interface PngPlteEntryAdded { index: number; rgb: PngRgb }

/** Nested inside `PngDiff.plte`'s outer tri-state: absent = unchanged; `null` = the PLTE
 * chunk was removed entirely; present = the palette is present (new or entry-changed). */
export interface PngPlteDiff {
  removed?: number[];
  modified?: PngPlteEntryModified[];
  added?: PngPlteEntryAdded[];
}

/** Sparse per-field patch for one `PngTextChunk`. */
export interface PngTextChunkDiff {
  keyword?: string;
  value?: string;
  compressed?: boolean;
  kind?: PngTextKind;
  languageTag?: string;
  translatedKeyword?: string;
}

export interface PngTextChunkModified { index: number; diff: PngTextChunkDiff }
export interface PngTextChunkAdded { index: number; chunk: PngTextChunk }

export interface PngTextChunksDiff {
  removed?: number[];
  modified?: PngTextChunkModified[];
  added?: PngTextChunkAdded[];
}

export interface PngUnknownChunkModified { index: number; chunk: PngChunk }
export interface PngUnknownChunkAdded { index: number; chunk: PngChunk }

export interface PngUnknownChunksDiff {
  removed?: number[];
  modified?: PngUnknownChunkModified[];
  added?: PngUnknownChunkAdded[];
}

export interface PngChunkOrderModified { index: number; marker: PngChunkMarker }
export interface PngChunkOrderAdded { index: number; marker: PngChunkMarker }

export interface PngChunkOrderDiff {
  removed?: number[];
  modified?: PngChunkOrderModified[];
  added?: PngChunkOrderAdded[];
}

/** 🔺️ Sparse diff for `stdio.png`. Every field present = changed to a value; tri-state
 * fields (`trns`/`gama`/`chrm`/`srgb`/`phys`/`time`/`bkgd`/`plte`) use `null` for "cleared". */
export interface PngDiff {
  width?: number;
  height?: number;
  bitDepth?: number;
  colorType?: PngColorType;
  interlace?: boolean;
  plte?: PngPlteDiff | null;
  trns?: PngTransparency | null;
  gama?: number | null;
  chrm?: PngChromaticities | null;
  srgb?: PngSrgbIntent | null;
  phys?: PngPhysicalDims | null;
  time?: PngTimestamp | null;
  bkgd?: PngBackground | null;
  textChunks?: PngTextChunksDiff;
  pixels?: number[];
  chunkOrder?: PngChunkOrderDiff;
  unknownChunks?: PngUnknownChunksDiff;
}
