/** 🧬️ SemioMeshMutation schema — real mirror of `🦀️component.rs`. Discriminated union on the
 * `mutation` tag (`#[serde(tag = "mutation", rename_all = "camelCase")]`). */
import type { SemioMesh, SemioMaterial, SemioTexture, SemioPrimitive, SemioTopology, SemioPoint3, SemioUv, SemioRgba, SemioMeshSnapshot } from "../📸️snapshot/🟦️component";

export type SemioMeshMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioMeshSnapshot }
  | { mutation: "addMesh"; mesh: SemioMesh }
  | { mutation: "removeMesh"; id: string }
  | { mutation: "addPrimitive"; meshId: string; primitive: SemioPrimitive }
  | { mutation: "removePrimitive"; meshId: string; primitiveId: string }
  | { mutation: "setPrimitiveTopology"; meshId: string; primitiveId: string; topology: SemioTopology }
  | { mutation: "setPrimitiveGeometry"; meshId: string; primitiveId: string; positions: SemioPoint3[]; normals: SemioPoint3[]; uvs: SemioUv[]; colors: SemioRgba[]; indices: number[] }
  | { mutation: "setPrimitiveMaterial"; meshId: string; primitiveId: string; materialId: string | null }
  | { mutation: "addMaterial"; material: SemioMaterial }
  | { mutation: "removeMaterial"; id: string }
  | { mutation: "setMaterialBaseColor"; id: string; baseColor: SemioRgba }
  | { mutation: "setMaterialPbr"; id: string; metallic: number; roughness: number }
  | { mutation: "addTexture"; texture: SemioTexture }
  | { mutation: "removeTexture"; id: string }
  | { mutation: "setTextureBytes"; id: string; mime: string; bytes: number[] };
