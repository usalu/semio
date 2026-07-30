// Generated from 🛂manifest.jsonwires.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const WIRES_EDGE_WIRES_OWNS = "wires.owns" as const;
export const WIRES_EDGE_WIRES_IS = "wires.is" as const;
export const WIRES_EDGE_WIRES_REFERENCES = "wires.references" as const;
export const WIRES_EDGE_WIRES_HAS = "wires.has" as const;

export type WiresEdgeKindId = "wires.owns" | "wires.is" | "wires.references" | "wires.has";
export const WIRES_EDGE_IDS = ["wires.owns", "wires.is", "wires.references", "wires.has"] as const satisfies readonly WiresEdgeKindId[];

export const WIRES_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "wires",
  "name": "WIRES Mindmap",
  "axes": {
    "portModel": "normal",
    "directedness": "undirected"
  },
  "edgeKinds": [
    {
      "id": "wires.owns",
      "name": "Owns"
    },
    {
      "id": "wires.is",
      "name": "Is"
    },
    {
      "id": "wires.references",
      "name": "References"
    },
    {
      "id": "wires.has",
      "name": "Has"
    }
  ]
} as const satisfies GraphManifestDocument;

export function wiresManifestCatalogBundle(): KindCatalogBundle {
  const doc = WIRES_MANIFEST_DOCUMENT;
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
