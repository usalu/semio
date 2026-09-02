/** 🧭 `topology` — one named inference: depth-first execution-order stats over the Path/Step tree. */

export interface ImperativeTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
