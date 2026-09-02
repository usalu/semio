/** 📦 `bounds` — one named inference: the 2d bounding box + node count of `ruleLayout`. */

export interface RewritingBoundingBox {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export interface RewritingBounds {
  boundingBox: RewritingBoundingBox;
  nodeCount: number;
}
