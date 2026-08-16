/** 🌐️ Trinity Jack viewer — Graph window: typed twin of `🦀️component.rs`'s view-model. Read-only
 * mirror of the node-graph scene payload `render()` produces — no query text, no LOD toggle, no
 * selection-shaped fields, matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One node in the rendered graph, mirroring `NodeGraphNodeRecord`'s shape. */
export interface TrinityJackViewGraphNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

/** 👁️ One edge in the rendered graph, mirroring `NodeGraphEdgeRecord`'s shape. */
export interface TrinityJackViewGraphEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

/** 👁️ The Graph window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `JackSnapshot`, no runtime/config/query state: a viewer has none of those). */
export interface TrinityJackViewGraphViewModel {
  windowKindId: "trinity-jack-view-graph";
  bodyKey: "trinity.jack.view.graph";
  surfaceId: "trinity.jack.view.graph";
  nodes: TrinityJackViewGraphNode[];
  edges: TrinityJackViewGraphEdge[];
}

export const TRINITY_JACK_VIEW_GRAPH_WINDOW_KIND_ID = "trinity-jack-view-graph" as const;
export const TRINITY_JACK_VIEW_GRAPH_BODY_KEY = "trinity.jack.view.graph" as const;
export const TRINITY_JACK_VIEW_GRAPH_SURFACE_ID = "trinity.jack.view.graph" as const;
