/** 🔺️ AviDiff — sparse per-field diff. Mirrors 🦀️.rs field-for-field. */
export interface IndexedModified<D> { index: number; diff: D; }
export interface IndexedAdded<T> { index: number; item: T; }
export interface IndexedDiff<T, D> { removed: number[]; modified: IndexedModified<D>[]; added: IndexedAdded<T>[]; }

export interface AviChunkDiff { data?: number[]; keyframe?: boolean; }
export interface AviStreamDiff {
  strh?: import("../📸️snapshot/🟦️").AviStreamHeader;
  strf?: import("../📸️snapshot/🟦️").AviStreamFormat;
  chunks?: IndexedDiff<import("../📸️snapshot/🟦️").AviChunk, AviChunkDiff>;
}
export interface AviDiff {
  mainHeader?: import("../📸️snapshot/🟦️").AviMainHeader;
  streams?: IndexedDiff<import("../📸️snapshot/🟦️").AviStream, AviStreamDiff>;
  idx1Present?: boolean;
  unknownChunks?: IndexedDiff<import("../📸️snapshot/🟦️").RiffChunk, import("../📸️snapshot/🟦️").RiffChunk>;
}
