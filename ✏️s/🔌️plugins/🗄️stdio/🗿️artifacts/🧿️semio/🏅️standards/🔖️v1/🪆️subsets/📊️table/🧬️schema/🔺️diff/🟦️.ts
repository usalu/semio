/** 🔺️ SemioTableDiff schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export interface SemioTableColumnListDiff {
  values: import("../📸️snapshot/🟦️.ts").SemioTableColumn[];
}
export interface SemioTableRowListDiff {
  values: import("../📸️snapshot/🟦️.ts").SemioTableRow[];
}
export interface SemioTableDiff {
  /** @state artifact */ columns?: SemioTableColumnListDiff | null;
  /** @state artifact */ rows?: SemioTableRowListDiff | null;
}
