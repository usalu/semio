/** ➡️ Trinity Rewrite editor — RHS window: typed twin of `render_fixture_graph`'s node-graph scene
 * boundary over the rule's right-hand-side actions (editable, fixture-driven viewport — no camera
 * override). */

export interface TrinityRewriteEditRhsNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewriteEditRhsEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewriteEditRhsViewModel {
  windowKindId: "trinity-rewrite-edit-rhs";
  bodyKey: "trinity.rewrite.edit.rhs";
  surfaceId: "trinity.rewrite.edit.rhs";
  nodes: TrinityRewriteEditRhsNode[];
  edges: TrinityRewriteEditRhsEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: true;
}

export const TRINITY_REWRITE_EDIT_RHS_WINDOW_KIND_ID = "trinity-rewrite-edit-rhs" as const;
export const TRINITY_REWRITE_EDIT_RHS_BODY_KEY = "trinity.rewrite.edit.rhs" as const;
export const TRINITY_REWRITE_EDIT_RHS_SURFACE_ID = "trinity.rewrite.edit.rhs" as const;
