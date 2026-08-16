/** 🕸️ DAG editor — the main window: typed twin of `🦀️component.rs`'s node-graph render boundary.
 * Mirrors the framework's own `NodeGraphNodeRecord`/`NodeGraphEdgeRecord`/`NodeGraphViewport` shapes
 * (`🧰️framework/🔨️modules/🔺️mesh/🟦️component.ts`) rather than importing them, matching this
 * taxonomy's per-component TS twin convention (no cross-package TS import). */

/** 🕹️ One node-graph port (input or output), by id only — kind is implied by which array it's in. */
export interface DagPlayMainPort {
  id: string;
}

/** 🕸️ One live node-graph node, projected off `DagNodeSpec` via `document_to_workflow`. */
export interface DagPlayMainNode {
  id: string;
  label?: string;
  x: number;
  y: number;
  width: number;
  height: number;
  inputs: DagPlayMainPort[];
  outputs: DagPlayMainPort[];
}

/** 🕸️ One live node-graph edge, projected off `DagFixtureEdge` via `document_to_workflow`. */
export interface DagPlayMainEdge {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
  label?: string;
}

/** 📷️ The free/live viewport camera — mirrors `DagCamera`/`DagConfig.camera{X,Y,Zoom}`. */
export interface DagPlayMainViewport {
  x: number;
  y: number;
  zoom: number;
}

/** 🧱️ The main window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (`DagSnapshot` + `DagCamera`) and output (an editable `NodeGraphScene`). */
export interface DagPlayMainViewModel {
  windowKindId: "dag-main";
  bodyKey: "dag.play.main";
  surfaceId: "dag.play.main";
  nodes: DagPlayMainNode[];
  edges: DagPlayMainEdge[];
  viewport: DagPlayMainViewport;
  editable: true;
}

export const DAG_PLAY_WINDOW_MAIN = "dag-main" as const;
export const DAG_PLAY_BODY_MAIN = "dag.play.main" as const;
export const DAG_PLAY_SURFACE_MAIN = "dag.play.main" as const;
