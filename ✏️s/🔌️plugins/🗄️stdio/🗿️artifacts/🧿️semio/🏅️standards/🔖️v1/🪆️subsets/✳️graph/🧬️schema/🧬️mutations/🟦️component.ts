/** 🧬️ SemioGraphMutation — real facet mirror of the Rust `🦀️component.rs` sibling. Closed,
 * eleven-variant dispatch. `SemioGraphMutation` carries only `#[derive(dsl::Mutations)]` — no
 * `#[serde(tag = ...)]` — so it serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by the committed
 * `🔚remove-node-port/🧪️tests/*​/🦠️mutation/🔣️component.json` fixture (`{"RemoveNodePort":
 * {"node_id":{"value":"a"},"index":1}}`) — NOT the `{ mutation: "...", payload: {...} }` envelope
 * this previously declared, and the previous `import(...)` payload references pointed at
 * `../<leaf>/🦠️mutation/🟦️component.ts` files that don't exist (no leaf has a nested `🦠️mutation`
 * TS mirror here — only `../📸️snapshot/🟦️component.ts`'s types are real). None of the 11 leaf
 * structs carry `#[serde(rename_all = ...)]` (confirmed by this artifact's own `🦀️.rs` doc
 * comment), so every leaf's own field names are the literal Rust snake_case names verbatim. */
import type { GraphNodeId, GraphEdgeId, SemioGraphPort } from "../📸️snapshot/🟦️component.ts";

export type SemioPoint2 = { x: number; y: number };

export interface SemioValueEntry {
  key: string;
  value: unknown;
}

export interface CreateNode {
  id: GraphNodeId;
  kind?: string;
  label?: string;
  position?: SemioPoint2;
  ports?: SemioGraphPort[];
  properties?: SemioValueEntry[];
}

export interface DeleteNode {
  id: GraphNodeId;
}

export interface ChangeNodeKind {
  id: GraphNodeId;
  new_kind: string;
}

export interface ChangeNodeLabel {
  id: GraphNodeId;
  new_label: string;
}

export interface MoveNode {
  id: GraphNodeId;
  new_position: SemioPoint2;
}

export interface AddNodePort {
  node_id: GraphNodeId;
  index: number;
  port: SemioGraphPort;
}

export interface RemoveNodePort {
  node_id: GraphNodeId;
  index: number;
}

export interface AddNodeProperty {
  node_id: GraphNodeId;
  index: number;
  property: SemioValueEntry;
}

export interface RemoveNodeProperty {
  node_id: GraphNodeId;
  index: number;
}

export interface CreateEdge {
  id: GraphEdgeId;
  source: GraphNodeId;
  target: GraphNodeId;
  kind?: string;
  label?: string;
}

export interface DeleteEdge {
  id: GraphEdgeId;
}

export type SemioGraphMutation =
  | { CreateNode: CreateNode }
  | { DeleteNode: DeleteNode }
  | { ChangeNodeKind: ChangeNodeKind }
  | { ChangeNodeLabel: ChangeNodeLabel }
  | { MoveNode: MoveNode }
  | { AddNodePort: AddNodePort }
  | { RemoveNodePort: RemoveNodePort }
  | { AddNodeProperty: AddNodeProperty }
  | { RemoveNodeProperty: RemoveNodeProperty }
  | { CreateEdge: CreateEdge }
  | { DeleteEdge: DeleteEdge };
