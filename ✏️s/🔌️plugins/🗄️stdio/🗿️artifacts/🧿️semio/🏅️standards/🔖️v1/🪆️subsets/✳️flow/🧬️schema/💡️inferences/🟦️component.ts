/** 💡️ Semio flow inference schema — node/edge topological order (Kahn's algorithm). */

export interface SemioFlowTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface SemioFlowInference {
  /** @state inferred */
  topology: SemioFlowTopology;
}
