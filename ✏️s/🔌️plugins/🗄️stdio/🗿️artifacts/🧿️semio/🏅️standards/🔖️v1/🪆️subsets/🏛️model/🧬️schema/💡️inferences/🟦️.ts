/** 💡️ Semio model inference schema — position envelope over spatial nodes + elements. */

export interface SemioPoint3 {
  x: number;
  y: number;
  z: number;
}

export interface SemioModelBounds {
  min: SemioPoint3;
  max: SemioPoint3;
  entityCount: number;
}

export interface SemioModelInference {
  /** @derived */
  bounds: SemioModelBounds;
}
