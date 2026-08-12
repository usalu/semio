/** 🔺️ SemioGraphDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioGraphNodeListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioGraphNode[];
}
export interface SemioGraphEdgeListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioGraphEdge[];
}
export interface SemioGraphDiff {
  /** @state persistent */ nodes?: SemioGraphNodeListDiff | null;
  /** @state persistent */ edges?: SemioGraphEdgeListDiff | null;
}
