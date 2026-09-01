/** 🧬️ SemioMeshMutation — real facet mirror of the Rust `🦀️component.rs` sibling. Closed,
 * seventeen-variant dispatch derived from this subset's real snapshot fields, checked against
 * SMO's taxonomy+derivation-rules docs. `SemioMeshMutation` carries only `#[derive(dsl::Mutations)]`
 * — no `#[serde(tag = ...)]` — so it serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by the committed
 * `🧱change-material-roughness/🧪️tests/*​/🦠️mutation/🔣️component.json` fixture
 * (`{"ChangeMaterialRoughness":{"id":"mat-a","new_roughness":0.25}}`) — NOT the flat
 * `{ mutation: "...", ...fields }` shape this previously declared. None of the 17 leaf structs
 * carry `#[serde(rename_all = ...)]` (confirmed by this artifact's own `🦀️.rs` doc comment), so
 * every leaf's own field names are the literal Rust snake_case names verbatim. */
import type { SemioMesh, SemioMaterial, SemioTexture, SemioPrimitive, SemioTopology, SemioPoint3, SemioUv, SemioRgba } from "../📸️snapshot/🟦️component";

export interface CreateMesh {
  mesh: SemioMesh;
}

export interface DeleteMesh {
  id: string;
}

export interface CreatePrimitive {
  mesh_id: string;
  primitive: SemioPrimitive;
}

export interface DeletePrimitive {
  mesh_id: string;
  primitive_id: string;
}

export interface SetPrimitiveTopology {
  mesh_id: string;
  primitive_id: string;
  topology: SemioTopology;
}

export interface ReplacePrimitiveGeometry {
  mesh_id: string;
  primitive_id: string;
  positions: SemioPoint3[];
  normals: SemioPoint3[];
  uvs: SemioUv[];
  colors: SemioRgba[];
  indices: number[];
}

export interface SetPrimitiveMaterial {
  mesh_id: string;
  primitive_id: string;
  material_id: string | null;
}

export interface CreateMaterial {
  material: SemioMaterial;
}

export interface DeleteMaterial {
  id: string;
}

export interface ChangeMaterialBaseColor {
  id: string;
  new_base_color: SemioRgba;
}

export interface ChangeMaterialMetallic {
  id: string;
  new_metallic: number;
}

export interface ChangeMaterialRoughness {
  id: string;
  new_roughness: number;
}

export interface CreateTexture {
  texture: SemioTexture;
}

export interface DeleteTexture {
  id: string;
}

export interface ChangeTextureMime {
  id: string;
  new_mime: string;
}

export interface ReplaceTextureBytes {
  id: string;
  new_bytes: number[];
}

export interface MoveVertex {
  mesh_id: string;
  primitive_id: string;
  vertex_index: number;
  new_point: SemioPoint3;
}

export type SemioMeshMutation =
  | { CreateMesh: CreateMesh }
  | { DeleteMesh: DeleteMesh }
  | { CreatePrimitive: CreatePrimitive }
  | { DeletePrimitive: DeletePrimitive }
  | { SetPrimitiveTopology: SetPrimitiveTopology }
  | { ReplacePrimitiveGeometry: ReplacePrimitiveGeometry }
  | { SetPrimitiveMaterial: SetPrimitiveMaterial }
  | { CreateMaterial: CreateMaterial }
  | { DeleteMaterial: DeleteMaterial }
  | { ChangeMaterialBaseColor: ChangeMaterialBaseColor }
  | { ChangeMaterialMetallic: ChangeMaterialMetallic }
  | { ChangeMaterialRoughness: ChangeMaterialRoughness }
  | { CreateTexture: CreateTexture }
  | { DeleteTexture: DeleteTexture }
  | { ChangeTextureMime: ChangeTextureMime }
  | { ReplaceTextureBytes: ReplaceTextureBytes }
  | { MoveVertex: MoveVertex };
