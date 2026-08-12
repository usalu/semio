/** 🧬️ SemioMeshMutation facet mirror — real facet mirror of the Rust `🦀️component.rs` sibling.
 * Closed, seventeen-variant dispatch derived from this subset's real snapshot fields, checked
 * against SMO's taxonomy+derivation-rules docs. `replacePrimitiveGeometry` is SMO's approved
 * rename of the old `setPrimitiveGeometry`, completed by DKM after SMO wound down (see the Rust
 * sibling's module doc comment). */
import type { SemioMesh, SemioMaterial, SemioTexture, SemioPrimitive, SemioTopology, SemioPoint3, SemioUv, SemioRgba } from "../📸️snapshot/🟦️component";

export type SemioMeshMutation =
  | { mutation: "createMesh"; mesh: SemioMesh }
  | { mutation: "deleteMesh"; id: string }
  | { mutation: "createPrimitive"; meshId: string; primitive: SemioPrimitive }
  | { mutation: "deletePrimitive"; meshId: string; primitiveId: string }
  | { mutation: "setPrimitiveTopology"; meshId: string; primitiveId: string; topology: SemioTopology }
  | { mutation: "replacePrimitiveGeometry"; meshId: string; primitiveId: string; positions: SemioPoint3[]; normals: SemioPoint3[]; uvs: SemioUv[]; colors: SemioRgba[]; indices: number[] }
  | { mutation: "setPrimitiveMaterial"; meshId: string; primitiveId: string; materialId: string | null }
  | { mutation: "createMaterial"; material: SemioMaterial }
  | { mutation: "deleteMaterial"; id: string }
  | { mutation: "changeMaterialBaseColor"; id: string; newBaseColor: SemioRgba }
  | { mutation: "changeMaterialMetallic"; id: string; newMetallic: number }
  | { mutation: "changeMaterialRoughness"; id: string; newRoughness: number }
  | { mutation: "createTexture"; texture: SemioTexture }
  | { mutation: "deleteTexture"; id: string }
  | { mutation: "changeTextureMime"; id: string; newMime: string }
  | { mutation: "replaceTextureBytes"; id: string; newBytes: number[] }
  | { mutation: "moveVertex"; meshId: string; primitiveId: string; vertexIndex: number; newPoint: SemioPoint3 };
