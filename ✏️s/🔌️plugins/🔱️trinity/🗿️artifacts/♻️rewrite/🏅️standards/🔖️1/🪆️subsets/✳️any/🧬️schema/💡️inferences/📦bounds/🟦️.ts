/** 📦 `bounds` — one named inference: the 2d bounding box + node count of `ruleLayout`. */

export interface RewriteBoundingBox {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export interface RewriteBounds {
  boundingBox: RewriteBoundingBox;
  nodeCount: number;
}
