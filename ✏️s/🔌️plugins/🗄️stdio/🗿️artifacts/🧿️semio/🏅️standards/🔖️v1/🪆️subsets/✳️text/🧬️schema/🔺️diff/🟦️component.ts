/** 🔺️ SemioTextDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioTextRunListDiff {
  values: import("../📸️snapshot/🟦️component.ts").SemioTextRun[];
}
export interface SemioTextDiff {
  /** @state persistent */ runs?: SemioTextRunListDiff | null;
}
