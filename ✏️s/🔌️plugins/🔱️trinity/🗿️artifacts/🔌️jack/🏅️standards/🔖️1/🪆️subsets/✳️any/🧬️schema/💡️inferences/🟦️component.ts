/** 💡️ Jack inference schema — topology (topological order, depth, cycle-freedom) over nodes/edges,
 *  and flat-position (each node's flattened `(u, v)` position, BFS-walked from root_node_id). */

export interface JackTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface JackFlatPositionUv {
  u: number;
  v: number;
}

export interface JackFlatPosition {
  positions: Record<string, JackFlatPositionUv>;
}

export interface JackInference {
  /** @state inferred */
  topology: JackTopology;
  /** @state inferred */
  flatPosition: JackFlatPosition;
}
