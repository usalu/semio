/** 🧬️ SemioGraphArtifact schema — full artifact state, mirrors `SemioGraphSnapshot` field for
 * field. */
export interface SemioGraphArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: import("./📸️snapshot/🟦️.ts").SemioGraphNode[];
  /** @state artifact */ edges: import("./📸️snapshot/🟦️.ts").SemioGraphEdge[];
}
