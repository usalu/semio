/** 💡️ Json inference schema — document outline (node count, max depth, root kind). */

export interface JsonOutline {
  nodeCount: number;
  maxDepth: number;
  rootKind: string;
}

export interface JsonInference {
  /** @derived */
  outline: JsonOutline;
}
