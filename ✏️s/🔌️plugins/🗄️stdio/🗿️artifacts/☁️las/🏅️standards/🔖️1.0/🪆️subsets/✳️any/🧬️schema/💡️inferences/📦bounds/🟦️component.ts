/** 📦 `bounds` — the las snapshot's header-declared bounding box and point count. */

export interface LasBounds {
  minX: number;
  minY: number;
  minZ: number;
  maxX: number;
  maxY: number;
  maxZ: number;
  pointCount: number;
}
