/** 💡️ STEP AP214 inference schema — `CARTESIAN_POINT`-derived spatial bounding box. */

export interface StepBounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}

export interface StepInference {
  /** @derived */
  bounds: StepBounds;
}
