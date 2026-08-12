/** 🔺️ SemioKitDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. Each field is
 * present only when this diff touches it. */
export interface SemioKitDiff {
  types?: { id: string; name: string; category: string }[];
  designs?: unknown[];
  objects?: { childId: string; target: string }[];
  models?: { childId: string; target: string }[];
  properties?: { childId: string; target: string } | null;
  representations?: unknown[];
}
