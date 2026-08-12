/** 💡️ Jack inference schema — topology (topological order, depth, cycle-freedom) over nodes/edges. */

export interface JackTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface JackInference {
  /** @state inferred */
  topology: JackTopology;
}
