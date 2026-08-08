/** 🧬️ Puzzle2d snapshot schema — persistent fields only. */

export interface Puzzle2dSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  camera: Puzzle2dCamera;
  /** @state persistent */
  nodes: Puzzle2dNode[];
  /** @state persistent */
  edges: Puzzle2dEdge[];
  /** @state persistent */
  meta: Puzzle2dMeta;
}

