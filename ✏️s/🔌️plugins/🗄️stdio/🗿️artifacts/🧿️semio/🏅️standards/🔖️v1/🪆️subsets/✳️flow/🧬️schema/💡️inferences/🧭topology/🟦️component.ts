/** 🧭 `topology` — one named inference: a Kahn topological-order pass over the flow node/edge graph. */

export interface SemioFlowTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
