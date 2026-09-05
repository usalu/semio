/** 🔺️ SemioTextDiff schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export interface SemioTextRunListDiff {
  values: import("../📸️snapshot/🟦️.ts").SemioTextRun[];
}
export interface SemioTextDiff {
  /** @state artifact */ runs?: SemioTextRunListDiff | null;
}
