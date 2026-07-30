// Generated from note-🛂manifest.jsonblocks.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";


export const NOTEBLOCKS_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "note-blocks",
  "name": "Note Document Blocks",
  "blockKinds": [
    {
      "id": "text",
      "name": "Text",
      "properties": [
        {
          "name": "paragraphs",
          "kind": "data",
          "valueType": {
            "kind": "text"
          }
        }
      ]
    },
    {
      "id": "image",
      "name": "Image"
    },
    {
      "id": "table",
      "name": "Table"
    },
    {
      "id": "math",
      "name": "Math",
      "properties": [
        {
          "name": "tex",
          "kind": "data",
          "valueType": {
            "kind": "text"
          }
        }
      ]
    },
    {
      "id": "ink",
      "name": "Ink"
    },
    {
      "id": "group",
      "name": "Group"
    }
  ]
} as const satisfies GraphManifestDocument;

export function note_blocksManifestCatalogBundle(): KindCatalogBundle {
  const doc = NOTEBLOCKS_MANIFEST_DOCUMENT;
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
