/** 💡️ IFC4 inference schema — `IFCCARTESIANPOINT`-derived spatial bounding box. */

export interface IfcBounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}

export interface IfcInference {
  /** @state inferred */
  bounds: IfcBounds;
}
