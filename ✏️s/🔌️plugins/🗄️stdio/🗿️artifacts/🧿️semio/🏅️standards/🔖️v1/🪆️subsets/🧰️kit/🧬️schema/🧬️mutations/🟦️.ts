/** 🧬️ SemioKitMutation — real facet mirror. Fifteen variants covering both composition primitives
 * (create/delete for owned children, bind/unbind/change for the one LINK slot) and domain
 * vocabulary (add/remove/rename for types, add/remove/edit for designs). `SemioKitMutation` is
 * externally tagged by variant name, snake_case payload fields — no `#[serde(rename_all)]` on the
 * enum or any of its 15 leaf structs (confirmed by this artifact's own `🦀️.rs` doc comment) — so it
 * serializes as `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, NOT the
 * `{ mutation: "...", payload: {...} }` envelope this previously declared. `target`/`pin` carry the
 * full `ArtifactRef`/link-pin union objects, never flattened strings. */
import type { ArtifactRef, ArtifactLinkRef, SemioKitPiece, SemioKitConnection } from "../📸️snapshot/🟦️.ts";

export interface CreateObject {
  child_id: string;
  target: ArtifactRef;
}

export interface DeleteObject {
  child_id: string;
}

export interface CreateModel {
  child_id: string;
  target: ArtifactRef;
}

export interface DeleteModel {
  child_id: string;
}

export interface CreateProperties {
  child_id: string;
  target: ArtifactRef;
}

export interface DeleteProperties {}

export interface BindRepresentation {
  target: ArtifactRef;
  pin: ArtifactLinkRef["pin"];
  role: string;
}

export interface UnbindRepresentation {
  index: number;
}

export interface ChangeRepresentationPin {
  index: number;
  pin: ArtifactLinkRef["pin"];
}

export interface AddType {
  id: string;
  name: string;
  category: string;
}

export interface RemoveType {
  id: string;
}

export interface RenameType {
  id: string;
  new_name: string;
}

export interface AddDesign {
  id: string;
  name: string;
}

export interface RemoveDesign {
  id: string;
}

export interface EditDesign {
  id: string;
  pieces: SemioKitPiece[];
  connections: SemioKitConnection[];
}

export type SemioKitMutation =
  | { CreateObject: CreateObject }
  | { DeleteObject: DeleteObject }
  | { CreateModel: CreateModel }
  | { DeleteModel: DeleteModel }
  | { CreateProperties: CreateProperties }
  | { DeleteProperties: DeleteProperties }
  | { BindRepresentation: BindRepresentation }
  | { UnbindRepresentation: UnbindRepresentation }
  | { ChangeRepresentationPin: ChangeRepresentationPin }
  | { AddType: AddType }
  | { RemoveType: RemoveType }
  | { RenameType: RenameType }
  | { AddDesign: AddDesign }
  | { RemoveDesign: RemoveDesign }
  | { EditDesign: EditDesign };
