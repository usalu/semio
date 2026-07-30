// Generated from concrete-forest.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const CONCRETEFOREST_NODE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT = "Hexagonal Cut Concrete Forest Left" as const;
export const CONCRETEFOREST_NODE_HEXAGONAL_CUT_CONCRETE_FOREST_RIGHT = "Hexagonal Cut Concrete Forest Right" as const;

export type ConcreteForestNodeKindId = "Hexagonal Cut Concrete Forest Left" | "Hexagonal Cut Concrete Forest Right";
export const CONCRETEFOREST_NODE_IDS = ["Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Right"] as const satisfies readonly ConcreteForestNodeKindId[];
export const CONCRETEFOREST_EDGE_PUZZLE3D_ATTRACTION_LINK = "puzzle3d.attraction.link" as const;

export type ConcreteForestEdgeKindId = "puzzle3d.attraction.link";
export const CONCRETEFOREST_EDGE_IDS = ["puzzle3d.attraction.link"] as const satisfies readonly ConcreteForestEdgeKindId[];
export const CONCRETEFOREST_PORT_BL = "b-l" as const;
export const CONCRETEFOREST_PORT_BLM = "b-l-m" as const;
export const CONCRETEFOREST_PORT_BS = "b-s" as const;
export const CONCRETEFOREST_PORT_BSM = "b-s-m" as const;
export const CONCRETEFOREST_PORT_CB = "c-b" as const;
export const CONCRETEFOREST_PORT_CT = "c-t" as const;

export type ConcreteForestPortKindId = "b-l" | "b-l-m" | "b-s" | "b-s-m" | "c-b" | "c-t";
export const CONCRETEFOREST_PORT_IDS = ["b-l", "b-l-m", "b-s", "b-s-m", "c-b", "c-t"] as const satisfies readonly ConcreteForestPortKindId[];
export const CONCRETEFOREST_WIRE_CABLE_LINK = "cable.link" as const;

export type ConcreteForestWireKindId = "cable.link";
export const CONCRETEFOREST_WIRE_IDS = ["cable.link"] as const satisfies readonly ConcreteForestWireKindId[];

export const CONCRETEFOREST_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "concrete-forest",
  "name": "Concrete Forest",
  "axes": {
    "portModel": "ported",
    "directedness": "directed"
  },
  "portKinds": [
    {
      "id": "b-l",
      "name": "b-l",
      "presentation": {
        "color": "hsl(206 52% 48%)",
        "defaultWireKind": "cable.link"
      }
    },
    {
      "id": "b-l-m",
      "name": "b-l-m",
      "presentation": {
        "color": "hsl(290 52% 48%)",
        "defaultWireKind": "cable.link"
      }
    },
    {
      "id": "b-s",
      "name": "b-s",
      "presentation": {
        "color": "hsl(55 52% 48%)",
        "defaultWireKind": "cable.link"
      }
    },
    {
      "id": "b-s-m",
      "name": "b-s-m",
      "presentation": {
        "color": "hsl(124 52% 48%)",
        "defaultWireKind": "cable.link"
      }
    },
    {
      "id": "c-b",
      "name": "c-b",
      "presentation": {
        "color": "hsl(37 52% 48%)",
        "defaultWireKind": "cable.link"
      }
    },
    {
      "id": "c-t",
      "name": "c-t",
      "presentation": {
        "color": "hsl(169 52% 48%)",
        "defaultWireKind": "cable.link"
      }
    }
  ],
  "wireKinds": [
    {
      "id": "cable.link",
      "name": "Link",
      "presentation": {
        "defaultEdgeKind": "puzzle3d.attraction.link"
      }
    }
  ],
  "edgeKinds": [
    {
      "id": "puzzle3d.attraction.link",
      "name": "Link"
    }
  ],
  "nodeKinds": [
    {
      "id": "Hexagonal Cut Concrete Forest Left",
      "name": "Hexagonal Cut Concrete Forest Left",
      "presentation": {
        "meshUrl": "/mesh/hexagonal-cut-concrete-forest-left.glb",
        "handles": [
          {
            "handleKind": "b-l",
            "angle": -1.5707963267948966,
            "radius": 0.36
          },
          {
            "handleKind": "b-l-m",
            "angle": -0.9995976625058433,
            "radius": 0.36
          },
          {
            "handleKind": "b-l",
            "angle": -0.42839899821678995,
            "radius": 0.36
          },
          {
            "handleKind": "b-s-m",
            "angle": 0.14279966607226324,
            "radius": 0.36
          },
          {
            "handleKind": "b-s",
            "angle": 0.7139983303613167,
            "radius": 0.36
          },
          {
            "handleKind": "b-s-m",
            "angle": 1.28519699465037,
            "radius": 0.36
          },
          {
            "handleKind": "b-s",
            "angle": 1.856395658939423,
            "radius": 0.36
          },
          {
            "handleKind": "c-b",
            "angle": 2.4275943232284765,
            "radius": 0.36
          },
          {
            "handleKind": "c-t",
            "angle": 2.99879298751753,
            "radius": 0.36
          },
          {
            "handleKind": "c-b",
            "angle": 3.569991651806583,
            "radius": 0.36
          },
          {
            "handleKind": "c-t",
            "angle": 4.141190316095637,
            "radius": 0.36
          }
        ]
      }
    },
    {
      "id": "Hexagonal Cut Concrete Forest Right",
      "name": "Hexagonal Cut Concrete Forest Right",
      "presentation": {
        "meshUrl": "/mesh/hexagonal-cut-concrete-forest-right.glb",
        "handles": [
          {
            "handleKind": "b-l",
            "angle": -1.5707963267948966,
            "radius": 0.36
          },
          {
            "handleKind": "b-l-m",
            "angle": -0.9995976625058433,
            "radius": 0.36
          },
          {
            "handleKind": "b-l",
            "angle": -0.42839899821678995,
            "radius": 0.36
          },
          {
            "handleKind": "b-s-m",
            "angle": 0.14279966607226324,
            "radius": 0.36
          },
          {
            "handleKind": "b-s-m",
            "angle": 0.7139983303613167,
            "radius": 0.36
          },
          {
            "handleKind": "b-s",
            "angle": 1.28519699465037,
            "radius": 0.36
          },
          {
            "handleKind": "b-s-m",
            "angle": 1.856395658939423,
            "radius": 0.36
          },
          {
            "handleKind": "c-b",
            "angle": 2.4275943232284765,
            "radius": 0.36
          },
          {
            "handleKind": "c-t",
            "angle": 2.99879298751753,
            "radius": 0.36
          },
          {
            "handleKind": "c-b",
            "angle": 3.569991651806583,
            "radius": 0.36
          },
          {
            "handleKind": "c-t",
            "angle": 4.141190316095637,
            "radius": 0.36
          }
        ]
      }
    }
  ],
  "edgeTips": []
} as const satisfies GraphManifestDocument;

export function concrete_forestManifestCatalogBundle(): KindCatalogBundle {
  const doc = CONCRETEFOREST_MANIFEST_DOCUMENT;
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
