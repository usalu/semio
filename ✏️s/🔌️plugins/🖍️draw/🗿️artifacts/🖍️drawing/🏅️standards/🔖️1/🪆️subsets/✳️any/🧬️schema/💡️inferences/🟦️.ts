/** 💡️ Drawing inference schema — layer-tree topology (pre-order + nesting depth). */

export interface DrawingTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface DrawingInference {
  /** @derived */
  topology: DrawingTopology;
}
