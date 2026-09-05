/** 💡️ SemioMeshInference facet mirror — real facet mirror of the Rust `🦀️.rs` sibling.
 * `computedNormals`/`tessellationPreview` are deliberately absent — see the Rust sibling's module
 * doc comment for why (honest omission, not an oversight). Keyed per `"{meshId}:{primitiveId}"`. */
export interface SemioMeshInference {
  aabb: Record<string, import("./📦aabb/🟦️.ts").SemioAabb>;
}
