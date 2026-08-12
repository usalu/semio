/** 🔺️ SemioTableDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioTableColumnListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioTableColumn[];
}
export interface SemioTableRowListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioTableRow[];
}
export interface SemioTableDiff {
  /** @state persistent */ columns?: SemioTableColumnListDiff | null;
  /** @state persistent */ rows?: SemioTableRowListDiff | null;
}
