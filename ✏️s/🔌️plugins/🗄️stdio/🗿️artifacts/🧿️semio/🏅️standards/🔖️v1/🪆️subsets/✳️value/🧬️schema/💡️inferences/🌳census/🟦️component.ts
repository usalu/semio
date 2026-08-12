/** 🌳 `census` — the semio value graph's own recursive variant census + max depth. */

export interface SemioValueCensus {
  nullCount: number;
  boolCount: number;
  intCount: number;
  floatCount: number;
  strCount: number;
  bytesCount: number;
  listCount: number;
  mapCount: number;
  refCount: number;
  nodeCount: number;
  maxDepth: number;
}
