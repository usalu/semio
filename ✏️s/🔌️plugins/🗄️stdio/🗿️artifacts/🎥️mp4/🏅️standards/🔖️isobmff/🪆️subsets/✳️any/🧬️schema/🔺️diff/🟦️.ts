/** 🔺️ Mp4Diff — sparse per-field diff. Mirrors 🦀️.rs field-for-field. */
export interface IndexedModified<D> { index: number; diff: D; }
export interface IndexedAdded<T> { index: number; item: T; }
export interface IndexedDiff<T, D> { removed: number[]; modified: IndexedModified<D>[]; added: IndexedAdded<T>[]; }

export interface Mp4SampleDiff { data?: number[]; duration?: number; ctsOffset?: number; sync?: boolean; }
export interface Mp4TrackDiff {
  trackId?: number; timescale?: number; codec?: import("../📸️snapshot/🟦️").Mp4Codec;
  width?: number; height?: number; metadata?: import("../📸️snapshot/🟦️").Mp4TrackMetadata; chunkSampleCounts?: number[]; samples?: IndexedDiff<import("../📸️snapshot/🟦️").Mp4Sample, Mp4SampleDiff>;
}
export interface Mp4Diff {
  ftyp?: import("../📸️snapshot/🟦️").Mp4Ftyp;
  movie?: import("../📸️snapshot/🟦️").Mp4Movie;
  tracks?: IndexedDiff<import("../📸️snapshot/🟦️").Mp4Track, Mp4TrackDiff>;
}
