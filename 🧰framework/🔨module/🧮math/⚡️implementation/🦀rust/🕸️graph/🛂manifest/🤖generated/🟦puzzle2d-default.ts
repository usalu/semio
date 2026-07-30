// Generated from puzzle2d-🛂manifest.jsondefault.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const PUZZLE2DDEFAULT_EDGE_EDGE_LINK = "edge.link" as const;

export type Puzzle2dDefaultEdgeKindId = "edge.link";
export const PUZZLE2DDEFAULT_EDGE_IDS = ["edge.link"] as const satisfies readonly Puzzle2dDefaultEdgeKindId[];
export const PUZZLE2DDEFAULT_PORT_PORT = "port" as const;

export type Puzzle2dDefaultPortKindId = "port";
export const PUZZLE2DDEFAULT_PORT_IDS = ["port"] as const satisfies readonly Puzzle2dDefaultPortKindId[];
export const PUZZLE2DDEFAULT_WIRE_WIRE_LINK = "wire.link" as const;

export type Puzzle2dDefaultWireKindId = "wire.link";
export const PUZZLE2DDEFAULT_WIRE_IDS = ["wire.link"] as const satisfies readonly Puzzle2dDefaultWireKindId[];

export const PUZZLE2DDEFAULT_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "puzzle2d-default",
  "name": "Puzzle 2D Default",
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
    }
  ],
  "wireKinds": [
    {
      "id": "wire.link",
      "name": "Link wire",
      "presentation": {
        "defaultEdgeKind": "edge.link"
      }
    }
  ],
  "edgeKinds": [
    {
      "id": "edge.link",
      "name": "Link edge"
    }
  ],
  "nodeKinds": [],
  "edgeTips": []
} as const satisfies GraphManifestDocument;

export function puzzle2d_defaultManifestCatalogBundle(): KindCatalogBundle {
  const doc = PUZZLE2DDEFAULT_MANIFEST_DOCUMENT;
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
