/** 💡️ Rewriting inference schema — bounds (bounding box + node count) over `ruleLayout`. */

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

export interface RewritingInference {
  /** @derived */
  bounds: RewritingBounds;
}
