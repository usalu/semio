/** 📦 `bounds` — the IFC4 snapshot's `IFCCARTESIANPOINT`-derived spatial bounding box. */

export interface IfcBounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}
