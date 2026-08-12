/** 💡️ Semio kit inference schema — catalog census over the kit's own collections. */

export interface SemioKitEntries {
  typeCount: number;
  designCount: number;
  pieceCount: number;
  connectionCount: number;
  objectCount: number;
  modelCount: number;
  hasProperties: boolean;
  representationCount: number;
}

export interface SemioKitInference {
  /** @state inferred */
  entries: SemioKitEntries;
}
