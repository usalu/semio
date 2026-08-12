/** 🧭 `topology` — one named inference: execution-order topology stats over the node/edge graph. */

export interface DagTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
