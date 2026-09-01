/** 🧬️ SemioObjectMutation — real facet mirror. Nine variants: move/rotate/scale (domain transform
 * verbs) plus create/delete pairs for the three CHILD slots (brep/mesh/properties).
 * `SemioObjectMutation` carries only `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it
 * serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by the committed
 * `🧱create-brep/🧪️tests/*​/🦠️mutation/🔣️component.json` fixture (`{"CreateBrep":{"child_id":
 * "brep-1","target":{"artifactId":"...","dialect":{...}}}}`) — NOT the `{ mutation: "...",
 * payload: {...} }` envelope this previously declared, and `target` is the full `ArtifactRef`
 * object, never a flattened URI string. None of the 9 leaf structs carry
 * `#[serde(rename_all = ...)]` (confirmed by this artifact's own `🦀️.rs` doc comment), so every
 * leaf's own field names are the literal Rust snake_case names verbatim; `ArtifactRef` itself is
 * declared with camelCase fields in the schema root and keeps that casing where embedded. */
import type { ArtifactRef } from "../🟦️component.ts";

export interface MoveObject {
  translation: { x: number; y: number; z: number };
}

export interface RotateObject {
  rotation: { x: number; y: number; z: number; w: number };
}

export interface ScaleObject {
  scale: { x: number; y: number; z: number };
}

export interface CreateBrep {
  child_id: string;
  target: ArtifactRef;
}

export interface DeleteBrep {}

export interface CreateMesh {
  child_id: string;
  target: ArtifactRef;
}

export interface DeleteMesh {}

export interface CreateProperties {
  child_id: string;
  target: ArtifactRef;
}

export interface DeleteProperties {}

export type SemioObjectMutation =
  | { MoveObject: MoveObject }
  | { RotateObject: RotateObject }
  | { ScaleObject: ScaleObject }
  | { CreateBrep: CreateBrep }
  | { DeleteBrep: DeleteBrep }
  | { CreateMesh: CreateMesh }
  | { DeleteMesh: DeleteMesh }
  | { CreateProperties: CreateProperties }
  | { DeleteProperties: DeleteProperties };
