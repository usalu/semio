/** 🔺️ SemioGraphDiff schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export interface SemioGraphNodeListDiff {
  values: import("../📸️snapshot/🟦️.ts").SemioGraphNode[];
}
export interface SemioGraphEdgeListDiff {
  values: import("../📸️snapshot/🟦️.ts").SemioGraphEdge[];
}
export interface SemioGraphDiff {
  /** @state artifact */ nodes?: SemioGraphNodeListDiff | null;
  /** @state artifact */ edges?: SemioGraphEdgeListDiff | null;
}
