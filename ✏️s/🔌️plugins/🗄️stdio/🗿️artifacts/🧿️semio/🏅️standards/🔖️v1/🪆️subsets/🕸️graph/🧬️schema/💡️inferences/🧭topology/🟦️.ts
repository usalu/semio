/** 🧭 `topology` — one named inference: a Kahn topological-order pass over the graph's node/edge structure. */

export interface SemioGraphTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
