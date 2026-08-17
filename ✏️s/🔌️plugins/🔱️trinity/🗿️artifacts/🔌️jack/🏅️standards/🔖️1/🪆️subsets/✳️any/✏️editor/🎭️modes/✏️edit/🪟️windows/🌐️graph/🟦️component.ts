/** 🌐️ Trinity Jack editor — Nakagin Graph window: typed twin of `🦀️component.rs`'s node-graph
 * render boundary + LOD control, mirroring the framework's own `NodeGraphNodeRecord`/
 * `NodeGraphEdgeRecord`/`NodeGraphViewport` shapes rather than importing them (no cross-package TS
 * import, per this taxonomy's per-component twin convention). */

export interface TrinityJackEditGraphNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TrinityJackEditGraphEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

export interface TrinityJackEditGraphViewport {
  x: number;
  y: number;
  zoom: number;
}

/** 🧱️ The Graph window's typed view-model — `lodJson` mirrors `trinity_lod_json_for_window`'s
 * `{automatic: true} | {automatic: false, forcedLabel: string}` payload, kept as an opaque JSON
 * string on both sides. */
export interface TrinityJackEditGraphViewModel {
  windowKindId: "trinity-jack-edit-graph";
  bodyKey: "trinity.jack.edit.graph";
  surfaceId: "trinity.jack.edit.graph";
  windowId: string;
  nodes: TrinityJackEditGraphNode[];
  edges: TrinityJackEditGraphEdge[];
  viewport: TrinityJackEditGraphViewport;
  lodJson?: string;
}

export const TRINITY_JACK_EDIT_GRAPH_WINDOW_KIND_ID = "trinity-jack-edit-graph" as const;
export const TRINITY_JACK_EDIT_GRAPH_BODY_KEY = "trinity.jack.edit.graph" as const;
export const TRINITY_JACK_EDIT_GRAPH_SURFACE_ID = "trinity.jack.edit.graph" as const;
