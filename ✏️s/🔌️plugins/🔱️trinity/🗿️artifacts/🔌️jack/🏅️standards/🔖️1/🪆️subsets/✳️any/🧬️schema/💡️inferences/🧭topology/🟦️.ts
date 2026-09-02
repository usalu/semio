/** 🧭 `topology` — one named inference: topological order, per-node depth, and cycle-freedom. */

export interface JackTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
