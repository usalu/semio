/** 📦 `bounds` — the STEP AP214 snapshot's `CARTESIAN_POINT`-derived spatial bounding box. */

export interface StepBounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}
