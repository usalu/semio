// Generated from 🛂manifest.jsonrewrite-lhs.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const REWRITELHS_NODE_REWRITE_MATCH = "rewrite.match" as const;
export const REWRITELHS_NODE_REWRITE_WHERE = "rewrite.where" as const;

export type RewriteLhsNodeKindId = "rewrite.match" | "rewrite.where";
export const REWRITELHS_NODE_IDS = ["rewrite.match", "rewrite.where"] as const satisfies readonly RewriteLhsNodeKindId[];
export const REWRITELHS_EDGE_EDGE_FLOW = "edge.flow" as const;
export const REWRITELHS_EDGE_EDGE_PATTERN = "edge.pattern" as const;

export type RewriteLhsEdgeKindId = "edge.flow" | "edge.pattern";
export const REWRITELHS_EDGE_IDS = ["edge.flow", "edge.pattern"] as const satisfies readonly RewriteLhsEdgeKindId[];
export const REWRITELHS_PORT_PORT = "port" as const;

export type RewriteLhsPortKindId = "port";
export const REWRITELHS_PORT_IDS = ["port"] as const satisfies readonly RewriteLhsPortKindId[];
export const REWRITELHS_WIRE_WIRE_FLOW = "wire.flow" as const;

export type RewriteLhsWireKindId = "wire.flow";
export const REWRITELHS_WIRE_IDS = ["wire.flow"] as const satisfies readonly RewriteLhsWireKindId[];

export const REWRITELHS_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "rewrite-lhs",
  "name": "Rewrite LHS",
  "axes": {
    "portModel": "ported",
    "directedness": "directed"
  },
  "portKinds": [
    {
      "id": "port",
      "name": "Port",
      "direction": "out",
      "properties": []
    }
  ],
  "wireKinds": [
    {
      "id": "wire.flow",
      "name": "Flow",
      "presentation": {
        "defaultEdgeKind": "edge.flow"
      }
    }
  ],
  "edgeKinds": [
    {
      "id": "edge.flow",
      "name": "Flow",
      "presentation": {
        "directed": true,
        "targetTip": "filled-arrow"
      }
    },
    {
      "id": "edge.pattern",
      "name": "Pattern",
      "presentation": {
        "directed": true,
        "targetTip": "filled-arrow"
      }
    }
  ],
  "nodeKinds": [
    {
      "id": "rewrite.match",
      "name": "Match",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(210 58% 48%)",
        "icon": "emoji:🎯",
        "handles": [
          {
            "handleKind": "port",
            "angle": 0,
            "radius": 3
          },
          {
            "handleKind": "port",
            "angle": 3.141592653589793,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "rewrite.where",
      "name": "Where",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(42 58% 48%)",
        "icon": "emoji:🔍",
        "handles": [
          {
            "handleKind": "port",
            "angle": 3.141592653589793,
            "radius": 3
          },
          {
            "handleKind": "port",
            "angle": 0,
            "radius": 3
          }
        ]
      }
    }
  ],
  "edgeTips": []
} as const satisfies GraphManifestDocument;

export function rewrite_lhsManifestCatalogBundle(): KindCatalogBundle {
  const doc = REWRITELHS_MANIFEST_DOCUMENT;
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
