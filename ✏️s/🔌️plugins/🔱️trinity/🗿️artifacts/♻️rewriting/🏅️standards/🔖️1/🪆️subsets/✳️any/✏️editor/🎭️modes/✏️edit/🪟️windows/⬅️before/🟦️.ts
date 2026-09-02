/** ⬅️ Trinity Rewriting editor — Before window: typed twin of `render_fixture_graph`'s node-graph
 * scene boundary over `before_fixture_json`, editable, with the viewport driven by the pane's own
 * live camera (`cfg.before_pane_camera`) rather than the fixture's own layout — unlike LHS/RHS. */

export interface TrinityRewritingEditBeforeNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewritingEditBeforeEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewritingEditBeforeViewModel {
  windowKindId: "trinity-rewriting-edit-before";
  bodyKey: "trinity.rewriting.edit.before";
  surfaceId: "trinity.rewriting.edit.before";
  nodes: TrinityRewritingEditBeforeNode[];
  edges: TrinityRewritingEditBeforeEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: true;
}

export const TRINITY_REWRITING_EDIT_BEFORE_WINDOW_KIND_ID = "trinity-rewriting-edit-before" as const;
export const TRINITY_REWRITING_EDIT_BEFORE_BODY_KEY = "trinity.rewriting.edit.before" as const;
export const TRINITY_REWRITING_EDIT_BEFORE_SURFACE_ID = "trinity.rewriting.edit.before" as const;
