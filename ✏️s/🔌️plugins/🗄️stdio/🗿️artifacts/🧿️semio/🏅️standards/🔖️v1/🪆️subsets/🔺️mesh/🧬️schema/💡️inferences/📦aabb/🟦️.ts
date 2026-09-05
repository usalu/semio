/** 📦 real facet mirror of `MeshAabb`'s `InferredField::Value`. Keyed by `"{meshId}:{primitiveId}"` — see the Rust sibling's `aabb_key`. */
export interface SemioAabb {
  min: { x: number; y: number; z: number };
  max: { x: number; y: number; z: number };
}
