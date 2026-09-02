/** ➡️ Trinity Rewriting editor — RHS window: typed twin of `render_fixture_graph`'s node-graph scene
 * boundary over the rule's right-hand-side actions (editable, fixture-driven viewport — no camera
 * override). */

export interface TrinityRewritingEditRhsNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewritingEditRhsEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewritingEditRhsViewModel {
  windowKindId: "trinity-rewriting-edit-rhs";
  bodyKey: "trinity.rewriting.edit.rhs";
  surfaceId: "trinity.rewriting.edit.rhs";
  nodes: TrinityRewritingEditRhsNode[];
  edges: TrinityRewritingEditRhsEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: true;
}

export const TRINITY_REWRITING_EDIT_RHS_WINDOW_KIND_ID = "trinity-rewriting-edit-rhs" as const;
export const TRINITY_REWRITING_EDIT_RHS_BODY_KEY = "trinity.rewriting.edit.rhs" as const;
export const TRINITY_REWRITING_EDIT_RHS_SURFACE_ID = "trinity.rewriting.edit.rhs" as const;
