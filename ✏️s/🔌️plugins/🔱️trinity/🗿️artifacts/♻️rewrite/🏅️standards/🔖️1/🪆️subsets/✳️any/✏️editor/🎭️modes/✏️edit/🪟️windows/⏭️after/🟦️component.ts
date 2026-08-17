/** ⏭️ Trinity Rewrite editor — After window: typed twin of `render_fixture_graph`'s node-graph
 * scene boundary over `after_fixture_json` — the rule-applied result graph, read-only (`editable:
 * false` on the Rust call site), fixture-driven viewport. */

export interface TrinityRewriteEditAfterNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewriteEditAfterEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewriteEditAfterViewModel {
  windowKindId: "trinity-rewrite-edit-after";
  bodyKey: "trinity.rewrite.edit.after";
  surfaceId: "trinity.rewrite.edit.after";
  nodes: TrinityRewriteEditAfterNode[];
  edges: TrinityRewriteEditAfterEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: false;
}

export const TRINITY_REWRITE_EDIT_AFTER_WINDOW_KIND_ID = "trinity-rewrite-edit-after" as const;
export const TRINITY_REWRITE_EDIT_AFTER_BODY_KEY = "trinity.rewrite.edit.after" as const;
export const TRINITY_REWRITE_EDIT_AFTER_SURFACE_ID = "trinity.rewrite.edit.after" as const;
