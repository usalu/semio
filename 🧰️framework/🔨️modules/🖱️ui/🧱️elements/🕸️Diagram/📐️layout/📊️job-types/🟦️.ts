import type { Edge } from "@xyflow/react";
import type { DiagramPagedStore } from "../🗃️paged-store/🟦️.ts";
import { DIAGRAM_LAYOUT_MAX_INPUT_ITEMS } from "../📜️wire/🟦️.ts";

export interface DiagramLayoutWork {
  readonly deadline: number;
  readonly fuel: number;
  readonly generation: number;
}

export type DiagramLayoutJobStatus = "running" | "complete" | "cancelled" | "fault";

export interface DiagramLayoutPreview {
  readonly generation: number;
  readonly positions: readonly DiagramLayoutPosition[];
  readonly sequence: number;
}

export interface DiagramLayoutPosition {
  readonly index: number;
  readonly x: number;
  readonly y: number;
}

export interface DiagramLayoutStepResult {
  readonly consumed: number;
  readonly stage: DiagramLayoutStage;
  readonly status: DiagramLayoutJobStatus;
}

export type DiagramLayoutStage = "admit-nodes" | "sort-nodes" | "index-nodes" | "admit-edges" | "sort-edges" | "build-graph" | "assign-ranks" | "crossing" | "sort-crossing" | "measure-ranks" | "position-ranks" | "coordinates" | "project" | "project-edges" | "complete";
export type DiagramLayoutCloseStage = "previews" | "edges" | "nodes" | "spares" | "indices" | "captures" | "scalars" | "complete";

export interface DiagramLayoutNodeRuntime {
  barycenterCount: number;
  barycenterSum: number;
  cross: number;
  depth: number;
  height: number;
  id: string;
  indegree: number;
  order: number;
  outgoingHead: number;
  outgoingTail: number;
  processed: boolean;
  rank: number;
  sourceIndex: number;
  width: number;
  x: number;
  y: number;
}

export interface DiagramLayoutEdgeRuntime {
  id: string;
  source: number;
  sourceId: string;
  sourceIndex: number;
  target: number;
  targetId: string;
}

export interface DiagramLayoutLookup {
  done: boolean;
  high: number;
  low: number;
  result?: number;
  value: string;
}

export interface DiagramLayoutPendingEdge {
  captured: Edge;
  inputIndex: number;
  sourceLookup: DiagramLayoutLookup;
  targetLookup?: DiagramLayoutLookup;
}

export interface DiagramLayoutOwnedSource<Value> {
  readonly length: number;
  get(index: number): Value | undefined;
}

export function asDiagramLayoutSource<Value>(values: readonly Value[] | DiagramLayoutOwnedSource<Value>): DiagramLayoutOwnedSource<Value> {
  if ("get" in values && typeof values.get === "function") return values;
  const array = values as readonly Value[];
  return { get: (index) => array[index], length: array.length };
}

export interface DiagramLayoutMerge<Value> {
  left: number;
  leftCursor: number;
  middle: number;
  right: number;
  rightCursor: number;
  source: DiagramPagedStore<Value>;
  target: DiagramPagedStore<Value>;
  width: number;
}

export const diagramLayoutLimits = Object.freeze({ maxEdges: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, maxNodes: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, previewNodes: 128 });
export const diagramLayoutFrame = Object.freeze({ fuel: 16_384, milliseconds: 6 });
export const diagramLayoutPageSize = 128;
