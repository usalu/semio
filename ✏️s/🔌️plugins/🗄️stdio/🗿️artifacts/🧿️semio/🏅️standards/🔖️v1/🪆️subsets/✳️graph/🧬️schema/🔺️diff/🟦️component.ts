/** 🔺️ SemioGraphDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioGraphNodeListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioGraphNode[];
}
export interface SemioGraphEdgeListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioGraphEdge[];
}
export interface SemioGraphDiff {
  /** @state artifact */ nodes?: SemioGraphNodeListDiff | null;
  /** @state artifact */ edges?: SemioGraphEdgeListDiff | null;
}
