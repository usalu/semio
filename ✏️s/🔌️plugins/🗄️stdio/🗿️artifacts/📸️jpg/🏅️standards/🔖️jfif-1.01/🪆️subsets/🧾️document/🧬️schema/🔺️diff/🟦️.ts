/** 🔺️ Sparse owned JPEG content edits. */
import type { JfifDensityUnits, JfifThumbnail, JpgSegment } from '../📸️snapshot/🟦️.ts';

export interface JpgSegmentDiff { marker?: number; data?: number[] }
export interface JpgSegmentModified { index: number; diff: JpgSegmentDiff }
export interface JpgSegmentAdded { index: number; item: JpgSegment }
export interface JpgOtherSegmentsDiff {
  removed?: number[];
  modified?: JpgSegmentModified[];
  added?: JpgSegmentAdded[];
}

/** 🔺️ Sparse owned diff with explicit null clearing for optional values. */
export interface JpgDiff {
  width?: number;
  height?: number;
  pixels?: number[];
  jfifVersion?: [number, number];
  jfifDensityUnits?: JfifDensityUnits;
  jfifXDensity?: number;
  jfifYDensity?: number;
  jfifThumbnail?: JfifThumbnail | null;
  otherSegments?: JpgOtherSegmentsDiff;
}
