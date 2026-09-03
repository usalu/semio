/** 💡️ Imperative inference schema — topology (depth-first order + nesting depth + cycle-freedom) over the Path/Step tree. */

export interface ProcedureTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface ProcedureInference {
  /** @derived */
  topology: ProcedureTopology;
}
