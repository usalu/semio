/** 💡️ Semio cad inference schema — entity-derived planar bounding box. */

/** Mirrors the shared `engine::geometry::SemioPoint2` reused by the Rust side. */
export interface SemioPoint2 {
  x: number;
  y: number;
}

export interface SemioCadBounds {
  min: SemioPoint2;
  max: SemioPoint2;
  entityCount: number;
}

export interface SemioCadInference {
  /** @state inferred */
  bounds: SemioCadBounds;
}
