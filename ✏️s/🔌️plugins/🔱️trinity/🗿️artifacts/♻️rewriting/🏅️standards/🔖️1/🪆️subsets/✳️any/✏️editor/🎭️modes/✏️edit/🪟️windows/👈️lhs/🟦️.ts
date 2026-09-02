/** 👈️ Trinity Rewriting editor — LHS window: typed twin of `render_fixture_graph`'s node-graph scene
 * boundary over the rule's left-hand-side pattern (editable, fixture-driven viewport — no camera
 * override). */

export interface TrinityRewritingEditLhsNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewritingEditLhsEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewritingEditLhsViewModel {
  windowKindId: "trinity-rewriting-edit-lhs";
  bodyKey: "trinity.rewriting.edit.lhs";
  surfaceId: "trinity.rewriting.edit.lhs";
  nodes: TrinityRewritingEditLhsNode[];
  edges: TrinityRewritingEditLhsEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: true;
}

export const TRINITY_REWRITING_EDIT_LHS_WINDOW_KIND_ID = "trinity-rewriting-edit-lhs" as const;
export const TRINITY_REWRITING_EDIT_LHS_BODY_KEY = "trinity.rewriting.edit.lhs" as const;
export const TRINITY_REWRITING_EDIT_LHS_SURFACE_ID = "trinity.rewriting.edit.lhs" as const;
