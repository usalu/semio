/** 📦 `bounds` — the dxf snapshot's entity-derived 3D bounding box. */

export interface DxfBounds {
  min: [number, number, number];
  max: [number, number, number];
  entityCount: number;
}
