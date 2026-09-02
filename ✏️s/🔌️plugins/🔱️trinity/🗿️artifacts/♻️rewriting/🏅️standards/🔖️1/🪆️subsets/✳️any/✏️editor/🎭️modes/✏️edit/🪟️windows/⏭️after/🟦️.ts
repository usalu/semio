/** ⏭️ Trinity Rewriting editor — After window: typed twin of `render_fixture_graph`'s node-graph
 * scene boundary over `after_fixture_json` — the rule-applied result graph, read-only (`editable:
 * false` on the Rust call site), fixture-driven viewport. */

export interface TrinityRewritingEditAfterNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityRewritingEditAfterEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityRewritingEditAfterViewModel {
  windowKindId: "trinity-rewriting-edit-after";
  bodyKey: "trinity.rewriting.edit.after";
  surfaceId: "trinity.rewriting.edit.after";
  nodes: TrinityRewritingEditAfterNode[];
  edges: TrinityRewritingEditAfterEdge[];
  viewport: { x: number; y: number; zoom: number };
  lodJson?: string;
  editable: false;
}

export const TRINITY_REWRITING_EDIT_AFTER_WINDOW_KIND_ID = "trinity-rewriting-edit-after" as const;
export const TRINITY_REWRITING_EDIT_AFTER_BODY_KEY = "trinity.rewriting.edit.after" as const;
export const TRINITY_REWRITING_EDIT_AFTER_SURFACE_ID = "trinity.rewriting.edit.after" as const;
