/** 🧬️ SemioDrawingMutation — mirrors all 17 of the real Rust 🧬️mutations/🦀️.rs's SMO-approved
 * verb-dispatch variants (📌️important.md's binding vocabulary ruling; the pre-rewrite
 * `setSnapshot`/`setCanvasSize`/… setter vocabulary is banned/superseded, see that file's own doc
 * comment — there is no `SetSnapshot` variant at all). `NodePath` addresses a scene-graph node
 * (`DrawNode` carries no stable id): `layer` selects `layers[layer]`, `path` is a chain of child
 * indices from that layer's root (`path == []` = the root itself).
 * `SemioDrawingMutation` carries only `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so
 * it serializes with serde's default EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": {
 * ...leaf-struct-fields } }`, confirmed by the committed
 * `<kind>/🧪️tests/*​/🦠️mutation/🔣️component.json` fixtures (e.g. `{"UngroupNode":{"at":
 * {"layer":0,"path":[2]}}}`, `{"ReplaceFill":{"style_name":"primary","new_fill":{...}}}`) — NOT
 * the `{ mutation: "...", payload: {...} }` envelope (mixing camelCase AND kebab-case tags) this
 * previously declared. None of the leaf structs carry `#[serde(rename_all = ...)]` (confirmed by
 * this artifact's own `🦀️.rs` doc comment), so every leaf's own field names are the literal Rust
 * snake_case names verbatim. */
import type { DrawLayer, DrawNode, PathSegment, Rgba, SemioPoint2 } from "../📸️snapshot/🟦️component";

export type SemioPoint3 = { x: number; y: number; z: number };
export type SemioQuaternion = { x: number; y: number; z: number; w: number };
export type SemioTransform = { translation: SemioPoint3; rotation: SemioQuaternion; scale: SemioPoint3 };

export interface NodePath {
  layer: number;
  path: number[];
}

export interface CreateLayer {
  index: number;
  layer: DrawLayer;
}

export interface DeleteLayer {
  id: string;
}

export interface CreateNode {
  parent: NodePath;
  index: number;
  node: DrawNode;
}

export interface DeleteNode {
  at: NodePath;
}

export interface MoveNode {
  at: NodePath;
  new_origin: SemioPoint2;
}

export interface DragNodes {
  ats: NodePath[];
  offset: SemioPoint2;
}

export interface RotateNode {
  at: NodePath;
  new_rotation: SemioQuaternion;
}

export interface ScaleNode {
  at: NodePath;
  new_scale: SemioPoint3;
}

export interface ReorderNodes {
  parent: NodePath;
  from: number;
  to: number;
}

export interface GroupNodes {
  parent: NodePath;
  indices: number[];
  transform: SemioTransform;
}

export interface UngroupNode {
  at: NodePath;
}

export interface FlattenNode {
  at: NodePath;
}

export interface UnflattenNode {
  at: NodePath;
  original: DrawNode;
}

export interface ReplacePath {
  at: NodePath;
  new_segments: PathSegment[];
}

export interface ReplaceFill {
  style_name: string;
  new_fill: Rgba | null;
}

export interface ChangeStrokeColor {
  style_name: string;
  new_color: Rgba | null;
}

export interface ChangeStrokeWidth {
  style_name: string;
  new_width: number | null;
}

export type SemioDrawingMutation =
  | { CreateLayer: CreateLayer }
  | { DeleteLayer: DeleteLayer }
  | { CreateNode: CreateNode }
  | { DeleteNode: DeleteNode }
  | { MoveNode: MoveNode }
  | { DragNodes: DragNodes }
  | { RotateNode: RotateNode }
  | { ScaleNode: ScaleNode }
  | { ReorderNodes: ReorderNodes }
  | { GroupNodes: GroupNodes }
  | { UngroupNode: UngroupNode }
  | { FlattenNode: FlattenNode }
  | { UnflattenNode: UnflattenNode }
  | { ReplacePath: ReplacePath }
  | { ReplaceFill: ReplaceFill }
  | { ChangeStrokeColor: ChangeStrokeColor }
  | { ChangeStrokeWidth: ChangeStrokeWidth };
