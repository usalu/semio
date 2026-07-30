// Generated from 🛂manifest.jsonrewrite-rhs.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const REWRITERHS_NODE_REWRITE_SET = "rewrite.set" as const;
export const REWRITERHS_NODE_REWRITE_PARAMETER = "rewrite.parameter" as const;
export const REWRITERHS_NODE_REWRITE_CREATE = "rewrite.create" as const;
export const REWRITERHS_NODE_REWRITE_DELETE = "rewrite.delete" as const;
export const REWRITERHS_NODE_REWRITE_MERGE = "rewrite.merge" as const;

export type RewriteRhsNodeKindId = "rewrite.set" | "rewrite.parameter" | "rewrite.create" | "rewrite.delete" | "rewrite.merge";
export const REWRITERHS_NODE_IDS = ["rewrite.set", "rewrite.parameter", "rewrite.create", "rewrite.delete", "rewrite.merge"] as const satisfies readonly RewriteRhsNodeKindId[];
export const REWRITERHS_EDGE_EDGE_FLOW = "edge.flow" as const;

export type RewriteRhsEdgeKindId = "edge.flow";
export const REWRITERHS_EDGE_IDS = ["edge.flow"] as const satisfies readonly RewriteRhsEdgeKindId[];
export const REWRITERHS_PORT_PORT = "port" as const;

export type RewriteRhsPortKindId = "port";
export const REWRITERHS_PORT_IDS = ["port"] as const satisfies readonly RewriteRhsPortKindId[];
export const REWRITERHS_WIRE_WIRE_FLOW = "wire.flow" as const;

export type RewriteRhsWireKindId = "wire.flow";
export const REWRITERHS_WIRE_IDS = ["wire.flow"] as const satisfies readonly RewriteRhsWireKindId[];

export const REWRITERHS_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "rewrite-rhs",
  "name": "Rewrite RHS",
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
    }
  ],
  "nodeKinds": [
    {
      "id": "rewrite.set",
      "name": "Set",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(150 52% 42%)",
        "icon": "emoji:✏️",
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
    },
    {
      "id": "rewrite.parameter",
      "name": "Parameter",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(280 52% 52%)",
        "icon": "emoji:🎛️",
        "handles": [
          {
            "handleKind": "port",
            "angle": 0,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "rewrite.create",
      "name": "Create",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(95 52% 42%)",
        "icon": "emoji:➕",
        "handles": [
          {
            "handleKind": "port",
            "angle": 0,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "rewrite.delete",
      "name": "Delete",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(4 58% 50%)",
        "icon": "emoji:➖",
        "handles": [
          {
            "handleKind": "port",
            "angle": 0,
            "radius": 3
          }
        ]
      }
    },
    {
      "id": "rewrite.merge",
      "name": "Merge",
      "ports": [
        "port"
      ],
      "presentation": {
        "color": "hsl(200 52% 48%)",
        "icon": "emoji:🔀",
        "handles": [
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

export function rewrite_rhsManifestCatalogBundle(): KindCatalogBundle {
  const doc = REWRITERHS_MANIFEST_DOCUMENT;
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
