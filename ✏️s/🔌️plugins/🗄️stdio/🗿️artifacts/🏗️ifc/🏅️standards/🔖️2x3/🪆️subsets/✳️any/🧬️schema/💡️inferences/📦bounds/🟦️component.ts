/** 📦 `bounds` — the IFC2X3 snapshot's `IFCCARTESIANPOINT`-derived spatial bounding box. */

export interface Ifc2x3Bounds {
  min: [number, number, number];
  max: [number, number, number];
  pointCount: number;
}
