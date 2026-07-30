// Generated from flow-dag.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const FLOWDAG_NODE_COMPUTATION = "computation" as const;
export const FLOWDAG_NODE_SLIDER = "slider" as const;
export const FLOWDAG_NODE_SELECT = "select" as const;
export const FLOWDAG_NODE_SCREEN = "screen" as const;
export const FLOWDAG_NODE_NOTE = "note" as const;
export const FLOWDAG_NODE_IMAGE = "image" as const;
export const FLOWDAG_NODE_PREVIEW = "preview" as const;
export const FLOWDAG_NODE_ACTION = "action" as const;
export const FLOWDAG_NODE_EXPORT = "export" as const;
export const FLOWDAG_NODE_CLUSTER = "cluster" as const;
export const FLOWDAG_NODE_APP_INSTANCE = "appInstance" as const;

export type FlowDagNodeKindId = "computation" | "slider" | "select" | "screen" | "note" | "image" | "preview" | "action" | "export" | "cluster" | "appInstance";
export const FLOWDAG_NODE_IDS = ["computation", "slider", "select", "screen", "note", "image", "preview", "action", "export", "cluster", "appInstance"] as const satisfies readonly FlowDagNodeKindId[];

export const FLOWDAG_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "flow-dag",
  "name": "Flow DAG",
  "axes": {
    "portModel": "ported",
    "directedness": "directed"
  },
  "nodeKinds": [
    {
      "id": "computation",
      "name": "Computation"
    },
    {
      "id": "slider",
      "name": "Slider"
    },
    {
      "id": "select",
      "name": "Select"
    },
    {
      "id": "screen",
      "name": "Screen"
    },
    {
      "id": "note",
      "name": "Note"
    },
    {
      "id": "image",
      "name": "Image"
    },
    {
      "id": "preview",
      "name": "Preview"
    },
    {
      "id": "action",
      "name": "Action"
    },
    {
      "id": "export",
      "name": "Export"
    },
    {
      "id": "cluster",
      "name": "Cluster"
    },
    {
      "id": "appInstance",
      "name": "App Instance"
    }
  ]
} as const satisfies GraphManifestDocument;

export function flow_dagManifestCatalogBundle(): KindCatalogBundle {
  const doc = FLOWDAG_MANIFEST_DOCUMENT;
  return {
    handles: doc.portKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      color: String((row.presentation as { color?: string })?.color ?? "hsl(215 52% 48%)"),
      defaultWireKind: (row.presentation as { defaultWireKind?: string })?.defaultWireKind,
    })),
    wires: doc.wireKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      defaultEdgeKind: (row.presentation as { defaultEdgeKind?: string })?.defaultEdgeKind,
    })),
    nodes: doc.nodeKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      color: (row.presentation as { color?: string })?.color,
      stroke: (row.presentation as { stroke?: string })?.stroke,
      icon: (row.presentation as { icon?: string })?.icon,
      handles: (row.presentation as { handles?: readonly { handleKind: string; angle: number; radius?: number }[] })?.handles,
    })),
    edges: doc.edgeKinds?.map((row) => ({
      id: row.id,
      name: row.name ?? row.id,
      color: (row.presentation as { color?: string })?.color,
      stroke: (row.presentation as { stroke?: string | number })?.stroke as string | undefined,
      pattern: (row.presentation as { pattern?: string })?.pattern,
      shape: (row.presentation as { shape?: "bezier" | "line" })?.shape,
      sourceTip: (row.presentation as { sourceTip?: string })?.sourceTip,
      targetTip: (row.presentation as { targetTip?: string })?.targetTip,
      directed: (row.presentation as { directed?: boolean })?.directed,
    })),
    edgeTips: doc.edgeTips as KindCatalogBundle["edgeTips"],
  };
}
