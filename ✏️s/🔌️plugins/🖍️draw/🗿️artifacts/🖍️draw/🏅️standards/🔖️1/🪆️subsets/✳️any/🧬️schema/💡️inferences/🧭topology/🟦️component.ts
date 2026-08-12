/** 🧭 `topology` — one named inference: a pre-order traversal of the layer tree's structural nesting. */

export interface DrawTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
