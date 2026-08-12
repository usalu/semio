/** 💡️ IFC2X3 inference schema — `IFCCARTESIANPOINT`-derived spatial bounding box. */

export interface Ifc2x3Bounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}

export interface Ifc2x3Inference {
  /** @state inferred */
  bounds: Ifc2x3Bounds;
}
