/** 🧭 `topology` — one named inference: depth-first execution-order stats over the Path/Step tree. */

export interface ProcedureTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
