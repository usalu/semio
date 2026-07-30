// Generated from puzzle3d-default.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const PUZZLE3DDEFAULT_EDGE_PUZZLE3D_ATTRACTION_LINK = "puzzle3d.attraction.link" as const;

export type Puzzle3dDefaultEdgeKindId = "puzzle3d.attraction.link";
export const PUZZLE3DDEFAULT_EDGE_IDS = ["puzzle3d.attraction.link"] as const satisfies readonly Puzzle3dDefaultEdgeKindId[];
export const PUZZLE3DDEFAULT_PORT_VORTEX = "vortex" as const;

export type Puzzle3dDefaultPortKindId = "vortex";
export const PUZZLE3DDEFAULT_PORT_IDS = ["vortex"] as const satisfies readonly Puzzle3dDefaultPortKindId[];
export const PUZZLE3DDEFAULT_WIRE_CABLE_LINK = "cable.link" as const;

export type Puzzle3dDefaultWireKindId = "cable.link";
export const PUZZLE3DDEFAULT_WIRE_IDS = ["cable.link"] as const satisfies readonly Puzzle3dDefaultWireKindId[];

export const PUZZLE3DDEFAULT_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "puzzle3d-default",
  "name": "Puzzle 3D Default",
  "axes": {
    "portModel": "ported",
    "directedness": "directed"
  },
  "portKinds": [
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
      "id": "cable.link",
      "name": "Cable",
      "presentation": {
        "defaultEdgeKind": "puzzle3d.attraction.link"
      }
    }
  ],
  "edgeKinds": [
    {
      "id": "puzzle3d.attraction.link",
      "name": "Attraction"
    }
  ],
  "nodeKinds": []
} as const satisfies GraphManifestDocument;

export function puzzle3d_defaultManifestCatalogBundle(): KindCatalogBundle {
  const doc = PUZZLE3DDEFAULT_MANIFEST_DOCUMENT;
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
