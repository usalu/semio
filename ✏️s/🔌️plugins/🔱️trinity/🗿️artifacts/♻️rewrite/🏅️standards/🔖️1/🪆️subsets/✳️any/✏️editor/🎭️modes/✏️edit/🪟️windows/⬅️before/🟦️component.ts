/** ⬅️ Trinity Rewrite editor — Before window: typed twin of `render_fixture_graph`'s node-graph
 * scene boundary over `before_fixture_json`, editable, with the viewport driven by the pane's own
 * live camera (`cfg.before_pane_camera`) rather than the fixture's own layout — unlike LHS/RHS. */

export interface TrinityRewriteEditBeforeNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewriteEditBeforeEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewriteEditBeforeViewModel {
  windowKindId: "trinity-rewrite-edit-before";
  bodyKey: "trinity.rewrite.edit.before";
  surfaceId: "trinity.rewrite.edit.before";
  nodes: TrinityRewriteEditBeforeNode[];
  edges: TrinityRewriteEditBeforeEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: true;
}

export const TRINITY_REWRITE_EDIT_BEFORE_WINDOW_KIND_ID = "trinity-rewrite-edit-before" as const;
export const TRINITY_REWRITE_EDIT_BEFORE_BODY_KEY = "trinity.rewrite.edit.before" as const;
export const TRINITY_REWRITE_EDIT_BEFORE_SURFACE_ID = "trinity.rewrite.edit.before" as const;
