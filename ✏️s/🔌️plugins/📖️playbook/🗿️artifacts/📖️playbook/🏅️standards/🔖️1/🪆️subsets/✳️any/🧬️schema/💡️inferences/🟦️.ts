/** 💡️ Playbook inference schema — topology (step/block dependency order). */

export interface PlaybookTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PlaybookInference {
  /** @derived */
  topology: PlaybookTopology;
}
