// stdio.avi diff 📝️text facet — same shape as ../🟦️.ts.
/** 🔺️ AviDiff — sparse per-field diff. Mirrors 🦀️.rs field-for-field. */
export interface IndexedModified<D> { index: number; diff: D; }
export interface IndexedAdded<T> { index: number; item: T; }
export interface IndexedDiff<T, D> { removed: number[]; modified: IndexedModified<D>[]; added: IndexedAdded<T>[]; }

export interface AviChunkDiff { data?: number[]; keyframe?: boolean; }
export interface AviStreamDiff {
  strh?: import("./🟦️").AviStreamHeader;
  strf?: import("./🟦️").AviStreamFormat;
  chunks?: IndexedDiff<import("./🟦️").AviChunk, AviChunkDiff>;
}
export interface AviDiff {
  mainHeader?: import("./🟦️").AviMainHeader;
  streams?: IndexedDiff<import("./🟦️").AviStream, AviStreamDiff>;
  idx1Present?: boolean;
  unknownChunks?: IndexedDiff<import("./🟦️").RiffChunk, import("./🟦️").RiffChunk>;
}
