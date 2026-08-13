/** 🔺️ SemioTableDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioTableColumnListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioTableColumn[];
}
export interface SemioTableRowListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioTableRow[];
}
export interface SemioTableDiff {
  /** @state artifact */ columns?: SemioTableColumnListDiff | null;
  /** @state artifact */ rows?: SemioTableRowListDiff | null;
}
