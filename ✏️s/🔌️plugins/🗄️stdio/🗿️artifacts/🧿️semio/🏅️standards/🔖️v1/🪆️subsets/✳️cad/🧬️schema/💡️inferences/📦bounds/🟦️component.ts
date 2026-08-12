/** 📦 `bounds` — the semio cad snapshot's entity-derived planar bounding box. */

export interface SemioPoint2 {
  x: number;
  y: number;
}

export interface SemioCadBounds {
  min: SemioPoint2;
  max: SemioPoint2;
  entityCount: number;
}
