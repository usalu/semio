/** 📊️ Trinity Jack editor — Results window: typed twin of `🦀️.rs`'s `render()` boundary.
 * A query result is EITHER a node-graph (when the last query returned a graph fixture) OR a table
 * (columns/rows) — the Rust side branches on `QueryResultKind`, so the TS twin mirrors that as a
 * discriminated union rather than one shape with optional fields for both. */

export interface TrinityJackEditResultsTable {
  kind: "table";
  columnsJson: string;
  rowsJson: string;
}

export interface TrinityJackEditResultsGraph {
  kind: "graph";
  nodes: { id: string; label?: string; x: number; y: number; width: number; height: number }[];
  edges: { id: string; sourceNodeId: string; sourcePortId: string; targetNodeId: string; targetPortId: string }[];
  viewport: { x: number; y: number; zoom: number };
}

/** 🧱️ The Results window's typed view-model. */
export type TrinityJackEditResultsViewModel = {
  windowKindId: "trinity-jack-edit-results";
  bodyKey: "trinity.jack.edit.results";
  surfaceId: "trinity.jack.edit.results";
} & (TrinityJackEditResultsTable | TrinityJackEditResultsGraph);

export const TRINITY_JACK_EDIT_RESULTS_WINDOW_KIND_ID = "trinity-jack-edit-results" as const;
export const TRINITY_JACK_EDIT_RESULTS_BODY_KEY = "trinity.jack.edit.results" as const;
export const TRINITY_JACK_EDIT_RESULTS_SURFACE_ID = "trinity.jack.edit.results" as const;
