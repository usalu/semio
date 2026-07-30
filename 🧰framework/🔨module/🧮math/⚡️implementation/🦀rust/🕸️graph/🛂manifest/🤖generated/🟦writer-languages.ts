// Generated from writer-🛂manifest.jsonlanguages.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const WRITERLANGUAGES_LANGUAGE_JACK = "jack" as const;
export const WRITERLANGUAGES_LANGUAGE_WIRE = "wire" as const;
export const WRITERLANGUAGES_LANGUAGE_PLAINTEXT = "plaintext" as const;
export const WRITERLANGUAGES_LANGUAGE_MARKDOWN = "markdown" as const;

export type WriterLanguagesLanguageKindId = "jack" | "wire" | "plaintext" | "markdown";
export const WRITERLANGUAGES_LANGUAGE_IDS = ["jack", "wire", "plaintext", "markdown"] as const satisfies readonly WriterLanguagesLanguageKindId[];

export const WRITERLANGUAGES_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "writer-languages",
  "name": "Writer Languages",
  "languageKinds": [
    {
      "id": "jack",
      "name": "Jack",
      "properties": [
        {
          "name": "grammarModule",
          "kind": "data",
          "valueType": "text"
        }
      ]
    },
    {
      "id": "wire",
      "name": "Wire",
      "properties": [
        {
          "name": "grammarModule",
          "kind": "data",
          "valueType": "text"
        }
      ]
    },
    {
      "id": "plaintext",
      "name": "Plain Text"
    },
    {
      "id": "markdown",
      "name": "Markdown"
    }
  ]
} as const satisfies GraphManifestDocument;

export function writer_languagesManifestCatalogBundle(): KindCatalogBundle {
  const doc = WRITERLANGUAGES_MANIFEST_DOCUMENT;
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
