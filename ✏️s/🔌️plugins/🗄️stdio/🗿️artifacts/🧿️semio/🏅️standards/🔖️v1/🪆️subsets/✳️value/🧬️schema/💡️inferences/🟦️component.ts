/** 💡️ Semio value inference schema — real variant census + max depth over the value graph. */

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

export interface SemioValueInference {
  /** @state inferred */
  census: SemioValueCensus;
}
