/** 🔺️ SemioObjectDiff schema — real facet mirror of the Rust `🦀️.rs` sibling. Each field
 * is present only when this diff touches it; `brep`/`mesh`/`properties` carry `null` to mean
 * "clear the slot" vs. absent-from-the-diff to mean "untouched". */
export interface SemioObjectDiff {
  transform?: { translation: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number; w: number }; scale: { x: number; y: number; z: number } };
  brep?: { childId: string; target: string } | null;
  mesh?: { childId: string; target: string } | null;
  properties?: { childId: string; target: string } | null;
}
