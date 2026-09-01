/** 🕸️ DAG viewer — the main window: typed twin of `🦀️component.rs`'s read-only node-graph render
 * boundary. Mirrors the framework's own `NodeGraphNodeRecord`/`NodeGraphEdgeRecord`/
 * `NodeGraphViewport` shapes (`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`) rather than importing
 * them, matching this taxonomy's per-component TS twin convention (no cross-package TS import). No
 * mutation-shaped fields (no drag/add-node payloads), matching the viewer's `ViewEmit`-only contract. */

/** 🕹️ One node-graph port (input or output), by id only — kind is implied by which array it's in. */
export interface DagViewMainPort {
  id: string;
}

/** 🕸️ One read-only node-graph node, projected off `DagNodeSpec` via `document_to_workflow`. */
export interface DagViewMainNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
  inputs: DagViewMainPort[];
  outputs: DagViewMainPort[];
}

/** 🕸️ One read-only node-graph edge, projected off `DagFixtureEdge` via `document_to_workflow`. */
export interface DagViewMainEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
  label?: string;
}

/** 🧱️ The main window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `DagSnapshot`, no runtime/config state: a viewer has none of those) and output (a
 * non-editable `NodeGraphScene` at a fixed default camera). */
export interface DagViewMainViewModel {
  windowKindId: "dag-view-main";
  bodyKey: "dag.view.main";
  surfaceId: "dag.view.main";
  nodes: DagViewMainNode[];
  edges: DagViewMainEdge[];
  editable: false;
}

export const DAG_VIEW_WINDOW_MAIN = "dag-view-main" as const;
export const DAG_VIEW_BODY_MAIN = "dag.view.main" as const;
export const DAG_VIEW_SURFACE_MAIN = "dag.view.main" as const;
