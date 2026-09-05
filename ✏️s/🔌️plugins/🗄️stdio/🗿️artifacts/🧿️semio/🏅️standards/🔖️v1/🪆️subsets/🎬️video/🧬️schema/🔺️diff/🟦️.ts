/** 🔺️ SemioVideoDiff — sparse, handcrafted. `streams` is an index-keyed triple
 * (removed/modified/added) over `SemioVideoStreamDiff`; a modified stream's own `samples` field
 * is, recursively, the same index-keyed triple shape over `SemioVideoSampleDiff`. Mirrors
 * `🔺️diff/🦀️.rs` field for field. */
export interface IndexModified<D> {
  index: number;
  diff: D;
}
export interface IndexAdded<T> {
  index: number;
  item: T;
}
export interface IndexedTripleDiff<D, T> {
  removed: number[];
  modified: IndexModified<D>[];
  added: IndexAdded<T>[];
}

export interface SemioVideoSampleDiff {
  pts?: number;
  key?: boolean;
  /** hex-encoded opaque bytes, whole-value replace only */
  data?: number[];
}

export interface SemioVideoStreamDiff {
  kind?: import("../📸️snapshot/🟦️.ts").SemioVideoStreamKind;
  codec?: string;
  width?: number;
  height?: number;
  rate?: import("../📸️snapshot/🟦️.ts").SemioRational;
  samples?: IndexedTripleDiff<SemioVideoSampleDiff, import("../📸️snapshot/🟦️.ts").SemioVideoSample>;
}

export type SemioVideoStreamsDiff = IndexedTripleDiff<SemioVideoStreamDiff, import("../📸️snapshot/🟦️.ts").SemioVideoStream>;

export interface SemioVideoDiff {
  streams?: SemioVideoStreamsDiff;
}
