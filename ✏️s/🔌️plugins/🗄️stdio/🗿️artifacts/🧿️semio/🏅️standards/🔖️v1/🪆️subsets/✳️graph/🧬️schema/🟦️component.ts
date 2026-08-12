/** 🧬️ SemioGraphArtifact schema — full artifact state, mirrors `SemioGraphSnapshot` field for
 * field. */
export interface SemioGraphArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ nodes: import("./📸️snapshot/🟦️component.ts").SemioGraphNode[];
  /** @state persistent */ edges: import("./📸️snapshot/🟦️component.ts").SemioGraphEdge[];
}
