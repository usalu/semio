// stdio.avi diff 📝️text facet — same shape as ../🟦️.ts.
/** 🔺️ AviDiff — sparse per-field diff. Mirrors 🦀️.rs field-for-field. */
export interface IndexedModified<D> { index: number; diff: D; }
export interface IndexedAdded<T> { index: number; item: T; }
export interface IndexedDiff<T, D> { removed: number[]; modified: IndexedModified<D>[]; added: IndexedAdded<T>[]; }

export interface AviChunkDiff { data?: number[]; keyframe?: boolean; }
export interface AviStreamDiff {
  strh?: import("../📸️snapshot/🟦️.ts").AviStreamHeader;
  strf?: import("../📸️snapshot/🟦️.ts").AviStreamFormat;
  chunks?: IndexedDiff<import("../📸️snapshot/🟦️.ts").AviChunk, AviChunkDiff>;
}
export interface AviDiff {
  mainHeader?: import("../📸️snapshot/🟦️.ts").AviMainHeader;
  streams?: IndexedDiff<import("../📸️snapshot/🟦️.ts").AviStream, AviStreamDiff>;
  idx1Present?: boolean;
  unknownChunks?: IndexedDiff<import("../📸️snapshot/🟦️.ts").RiffChunk, import("../📸️snapshot/🟦️.ts").RiffChunk>;
}
