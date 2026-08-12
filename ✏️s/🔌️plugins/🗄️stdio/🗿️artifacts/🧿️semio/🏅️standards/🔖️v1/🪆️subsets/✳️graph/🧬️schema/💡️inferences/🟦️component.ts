/** 💡️ Semio graph inference schema — node/edge topological order (Kahn's algorithm). */

export interface SemioGraphTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface SemioGraphInference {
  /** @state inferred */
  topology: SemioGraphTopology;
}
