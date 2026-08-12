/** 💡️ Rewrite inference schema — bounds (bounding box + node count) over `ruleLayout`. */

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

export interface RewriteInference {
  /** @state inferred */
  bounds: RewriteBounds;
}
