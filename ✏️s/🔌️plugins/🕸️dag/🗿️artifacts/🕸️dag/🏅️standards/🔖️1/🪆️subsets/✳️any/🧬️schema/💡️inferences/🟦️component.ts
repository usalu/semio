/** 💡️ Dag inference schema — topology (topological order + depth + cycle-freedom) over nodes/edges. */

export interface DagTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface DagInference {
  /** @state inferred */
  topology: DagTopology;
}
