/** 💡️ Stl inference schema — triangle-soup bounding box and triangle count. */

export interface StlBounds {
  min: [number, number, number];
  max: [number, number, number];
  triangleCount: number;
}

export interface StlInference {
  /** @state inferred */
  bounds: StlBounds;
}
