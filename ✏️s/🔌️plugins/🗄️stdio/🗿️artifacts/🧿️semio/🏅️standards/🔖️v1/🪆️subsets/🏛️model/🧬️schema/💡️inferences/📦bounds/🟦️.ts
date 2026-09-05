/** 📦 `bounds` — the semio model's own position envelope over spatial nodes + elements. */

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
