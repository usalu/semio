/** 🧊 block5d update-part-3d/🦠️mutation — the whole 2-field 3D-projection pose facet atomically (orientation quaternion + scale vector, always edited together in a 3D pose gizmo). */
export interface UpdatePart3d {
  newOrientation?: [number, number, number, number];
  newScale?: [number, number, number];
}
