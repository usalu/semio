// Generated from puzzle5d-🛂manifest.jsondefault.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const PUZZLE5DDEFAULT_EDGE_EDGE_LINK = "edge.link" as const;
export const PUZZLE5DDEFAULT_EDGE_ATTRACTION_LINK = "attraction.link" as const;

export type Puzzle5dDefaultEdgeKindId = "edge.link" | "attraction.link";
export const PUZZLE5DDEFAULT_EDGE_IDS = ["edge.link", "attraction.link"] as const satisfies readonly Puzzle5dDefaultEdgeKindId[];
export const PUZZLE5DDEFAULT_PORT_PORT = "port" as const;
export const PUZZLE5DDEFAULT_PORT_VORTEX = "vortex" as const;

export type Puzzle5dDefaultPortKindId = "port" | "vortex";
export const PUZZLE5DDEFAULT_PORT_IDS = ["port", "vortex"] as const satisfies readonly Puzzle5dDefaultPortKindId[];
export const PUZZLE5DDEFAULT_WIRE_WIRE_LINK = "wire.link" as const;
export const PUZZLE5DDEFAULT_WIRE_CABLE_LINK = "cable.link" as const;

export type Puzzle5dDefaultWireKindId = "wire.link" | "cable.link";
export const PUZZLE5DDEFAULT_WIRE_IDS = ["wire.link", "cable.link"] as const satisfies readonly Puzzle5dDefaultWireKindId[];

export const PUZZLE5DDEFAULT_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "puzzle5d-default",
  "name": "Puzzle 5D Default",
  "axes": {
    "portModel": "ported",
    "directedness": "directed"
  },
  "portKinds": [
    {
      "id": "port",
      "name": "Port",
      "presentation": {
        "color": "var(--muted-foreground)",
        "defaultWireKind": "wire.link"
      }
    },
    {
      "id": "vortex",
      "name": "Vortex",
      "presentation": {
        "defaultWireKind": "cable.link"
      }
    }
  ],
  "wireKinds": [
    {
      "id": "wire.link",
      "name": "Link wire",
      "presentation": {
        "defaultEdgeKind": "edge.link"
      }
    },
    {
      "id": "cable.link",
      "name": "Cable",
      "presentation": {
        "defaultEdgeKind": "attraction.link"
      }
    }
  ],
  "edgeKinds": [
    {
      "id": "edge.link",
      "name": "Link edge"
    },
    {
      "id": "attraction.link",
      "name": "Attraction"
    }
  ],
  "nodeKinds": [],
  "kindCompatibility": [
    {
      "source": "port",
      "target": "port",
      "bidirectional": true
    },
    {
      "source": "vortex",
      "target": "vortex",
      "bidirectional": true
    }
  ]
} as const satisfies GraphManifestDocument;

export function puzzle5d_defaultManifestCatalogBundle(): KindCatalogBundle {
  const doc = PUZZLE5DDEFAULT_MANIFEST_DOCUMENT;
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
