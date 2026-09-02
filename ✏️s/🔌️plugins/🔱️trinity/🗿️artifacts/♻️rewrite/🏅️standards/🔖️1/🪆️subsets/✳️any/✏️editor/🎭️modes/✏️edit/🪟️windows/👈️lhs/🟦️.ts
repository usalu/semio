/** 👈️ Trinity Rewrite editor — LHS window: typed twin of `render_fixture_graph`'s node-graph scene
 * boundary over the rule's left-hand-side pattern (editable, fixture-driven viewport — no camera
 * override). */

export interface TrinityRewriteEditLhsNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewriteEditLhsEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewriteEditLhsViewModel {
  windowKindId: "trinity-rewrite-edit-lhs";
  bodyKey: "trinity.rewrite.edit.lhs";
  surfaceId: "trinity.rewrite.edit.lhs";
  nodes: TrinityRewriteEditLhsNode[];
  edges: TrinityRewriteEditLhsEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: true;
}

export const TRINITY_REWRITE_EDIT_LHS_WINDOW_KIND_ID = "trinity-rewrite-edit-lhs" as const;
export const TRINITY_REWRITE_EDIT_LHS_BODY_KEY = "trinity.rewrite.edit.lhs" as const;
export const TRINITY_REWRITE_EDIT_LHS_SURFACE_ID = "trinity.rewrite.edit.lhs" as const;
