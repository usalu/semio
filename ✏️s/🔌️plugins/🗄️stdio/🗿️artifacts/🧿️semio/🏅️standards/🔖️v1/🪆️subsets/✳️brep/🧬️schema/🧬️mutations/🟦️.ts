/** 🧬️ SemioBrepMutation — real facet mirror of the Rust `🦀️.rs` sibling. Closed,
 * thirteen-variant dispatch: SMO's approved verb table exactly (`create-loop`/`delete-loop`
 * deliberately absent — see the Rust sibling's module doc comment for why). `SemioBrepMutation`
 * carries only `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it serializes with
 * serde's default EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields }
 * }`, confirmed by the committed `🐚create-shell/🧪️tests/*​/🦠️mutation/🔣️.json` fixture
 * (`{"CreateShell":{"id":"s2","faces":[{"face":"f1","orientation":false}]}}`) — NOT the
 * `{ mutation: "...", payload: {...} }` envelope this previously declared, and the previous
 * `import(...)` payload references pointed at `../<leaf>/🦠️mutation/🟦️.ts` files that
 * don't exist (no leaf has a nested `🦠️mutation` TS mirror here — only
 * `../📸️snapshot/🟦️.ts`'s types are real). None of the 13 leaf structs carry
 * `#[serde(rename_all = ...)]` (confirmed by this artifact's own `🦀️.rs` doc comment), so every
 * leaf's own field names are the literal Rust snake_case names verbatim. */
import type { SemioPoint3, BrepCurve, BrepSurface, BrepShellFace, BrepSolidShell } from "../📸️snapshot/🟦️.ts";

export interface CreateVertex {
  id: string;
  point: SemioPoint3;
}

export interface DeleteVertex {
  id: string;
}

export interface CreateEdge {
  id: string;
  start_vertex: string;
  end_vertex: string;
  curve: BrepCurve;
}

export interface DeleteEdge {
  id: string;
}

export interface CreateFace {
  id: string;
  outer_loop: string;
  inner_loops?: string[];
  surface: BrepSurface;
  orientation: boolean;
}

export interface DeleteFace {
  id: string;
}

export interface CreateShell {
  id: string;
  faces?: BrepShellFace[];
}

export interface DeleteShell {
  id: string;
}

export interface CreateSolid {
  id: string;
  shells?: BrepSolidShell[];
}

export interface DeleteSolid {
  id: string;
}

export interface ReplaceCurve {
  edge_id: string;
  new_curve: BrepCurve;
}

export interface ReplaceSurface {
  face_id: string;
  new_surface: BrepSurface;
}

export interface MoveVertex {
  vertex_id: string;
  new_point: SemioPoint3;
}

export type SemioBrepMutation =
  | { CreateVertex: CreateVertex }
  | { DeleteVertex: DeleteVertex }
  | { CreateEdge: CreateEdge }
  | { DeleteEdge: DeleteEdge }
  | { CreateFace: CreateFace }
  | { DeleteFace: DeleteFace }
  | { CreateShell: CreateShell }
  | { DeleteShell: DeleteShell }
  | { CreateSolid: CreateSolid }
  | { DeleteSolid: DeleteSolid }
  | { ReplaceCurve: ReplaceCurve }
  | { ReplaceSurface: ReplaceSurface }
  | { MoveVertex: MoveVertex };
