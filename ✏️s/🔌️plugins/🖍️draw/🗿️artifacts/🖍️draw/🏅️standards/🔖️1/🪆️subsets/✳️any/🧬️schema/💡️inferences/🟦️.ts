/** 💡️ Draw inference schema — layer-tree topology (pre-order + nesting depth). */

export interface DrawTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface DrawInference {
  /** @derived */
  topology: DrawTopology;
}
