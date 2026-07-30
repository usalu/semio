// Generated from draw-layers.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const DRAWLAYERS_LAYER_SHAPE = "shape" as const;
export const DRAWLAYERS_LAYER_PATH = "path" as const;
export const DRAWLAYERS_LAYER_TEXT = "text" as const;
export const DRAWLAYERS_LAYER_IMAGE = "image" as const;
export const DRAWLAYERS_LAYER_GROUP = "group" as const;
export const DRAWLAYERS_LAYER_BOOLEAN = "boolean" as const;
export const DRAWLAYERS_LAYER_TRACE = "trace" as const;

export type DrawLayersLayerKindId = "shape" | "path" | "text" | "image" | "group" | "boolean" | "trace";
export const DRAWLAYERS_LAYER_IDS = ["shape", "path", "text", "image", "group", "boolean", "trace"] as const satisfies readonly DrawLayersLayerKindId[];

export const DRAWLAYERS_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "draw-layers",
  "name": "Draw Document Layers",
  "layerKinds": [
    {
      "id": "shape",
      "name": "Shape",
      "properties": [
        {
          "name": "shapeKind",
          "kind": "data",
          "valueType": {
            "kind": "text"
          }
        }
      ]
    },
    {
      "id": "path",
      "name": "Path"
    },
    {
      "id": "text",
      "name": "Text"
    },
    {
      "id": "image",
      "name": "Image"
    },
    {
      "id": "group",
      "name": "Group"
    },
    {
      "id": "boolean",
      "name": "Boolean"
    },
    {
      "id": "trace",
      "name": "Trace"
    }
  ]
} as const satisfies GraphManifestDocument;

export function draw_layersManifestCatalogBundle(): KindCatalogBundle {
  const doc = DRAWLAYERS_MANIFEST_DOCUMENT;
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
