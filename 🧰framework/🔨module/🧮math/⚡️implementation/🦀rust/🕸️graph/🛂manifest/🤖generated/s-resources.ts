// Generated from s-resources.manifest.json

import type { GraphManifestDocument, KindCatalogBundle } from "./types.js";

export const SRESOURCES_DESCRIPTOR_R2D_NOTE = "2d.note" as const;
export const SRESOURCES_DESCRIPTOR_R2D_DRAWING = "2d.drawing" as const;
export const SRESOURCES_DESCRIPTOR_R2D_RASTER = "2d.raster" as const;
export const SRESOURCES_DESCRIPTOR_R2D_MAP = "2d.map" as const;
export const SRESOURCES_DESCRIPTOR_R2D_PROCEDURAL = "2d.procedural" as const;
export const SRESOURCES_DESCRIPTOR_R2D_SHOOTING = "2d.shooting" as const;
export const SRESOURCES_DESCRIPTOR_R2D_PUZZLE = "2d.puzzle" as const;
export const SRESOURCES_DESCRIPTOR_R3D_PUZZLE = "3d.puzzle" as const;
export const SRESOURCES_DESCRIPTOR_R5D_PUZZLE = "5d.puzzle" as const;
export const SRESOURCES_DESCRIPTOR_R3D_PROCEDURAL = "3d.procedural" as const;
export const SRESOURCES_DESCRIPTOR_R3D_PROCESS = "3d.process" as const;
export const SRESOURCES_DESCRIPTOR_R3D_CAD = "3d.cad" as const;
export const SRESOURCES_DESCRIPTOR_COMPUTATION_FLOW = "computation.flow" as const;
export const SRESOURCES_DESCRIPTOR_GRAPH_TRINITY = "graph.trinity" as const;
export const SRESOURCES_DESCRIPTOR_GRAPH_DAG = "graph.dag" as const;
export const SRESOURCES_DESCRIPTOR_TEXT_DOCUMENT = "text.document" as const;
export const SRESOURCES_DESCRIPTOR_FORM_DICTIONARY = "form.dictionary" as const;
export const SRESOURCES_DESCRIPTOR_KIT_COMPOSE = "kit.compose" as const;
export const SRESOURCES_DESCRIPTOR_ANIMATE_PRESENT_DECK = "animate.present.deck" as const;
export const SRESOURCES_DESCRIPTOR_R3D_MESH = "3d.mesh" as const;
export const SRESOURCES_DESCRIPTOR_CATALOGUE_KINDS = "catalogue.kinds" as const;
export const SRESOURCES_DESCRIPTOR_R3D_LOWPOLY = "3d.lowpoly" as const;
export const SRESOURCES_DESCRIPTOR_COMPUTATION_SEQUENCE = "computation.sequence" as const;
export const SRESOURCES_DESCRIPTOR_R2D_LAYOUT = "2d.layout" as const;
export const SRESOURCES_DESCRIPTOR_COMPUTATION_IMPERATIVE = "computation.imperative" as const;
export const SRESOURCES_DESCRIPTOR_VCS_DOCUMENT = "vcs.document" as const;
export const SRESOURCES_DESCRIPTOR_PARAMETER_VALUE = "parameter.value" as const;
export const SRESOURCES_DESCRIPTOR_CATALOGUE_SOURCING = "catalogue.sourcing" as const;

export type SResourcesDescriptorKindId = "2d.note" | "2d.drawing" | "2d.raster" | "2d.map" | "2d.procedural" | "2d.shooting" | "2d.puzzle" | "3d.puzzle" | "5d.puzzle" | "3d.procedural" | "3d.process" | "3d.cad" | "computation.flow" | "graph.trinity" | "graph.dag" | "text.document" | "form.dictionary" | "kit.compose" | "animate.present.deck" | "3d.mesh" | "catalogue.kinds" | "3d.lowpoly" | "computation.sequence" | "2d.layout" | "computation.imperative" | "vcs.document" | "parameter.value" | "catalogue.sourcing";
export const SRESOURCES_DESCRIPTOR_IDS = ["2d.note", "2d.drawing", "2d.raster", "2d.map", "2d.procedural", "2d.shooting", "2d.puzzle", "3d.puzzle", "5d.puzzle", "3d.procedural", "3d.process", "3d.cad", "computation.flow", "graph.trinity", "graph.dag", "text.document", "form.dictionary", "kit.compose", "animate.present.deck", "3d.mesh", "catalogue.kinds", "3d.lowpoly", "computation.sequence", "2d.layout", "computation.imperative", "vcs.document", "parameter.value", "catalogue.sourcing"] as const satisfies readonly SResourcesDescriptorKindId[];

export const SRESOURCES_MANIFEST_DOCUMENT = {
  "schema": "manifest",
  "id": "s-resources",
  "name": "S Resource Kinds",
  "descriptorKinds": [
    {
      "id": "2d.note",
      "name": "2D Note",
      "presentation": {
        "sourceFormat": "note.document",
        "componentKind": "note",
        "dimension": "2d"
      }
    },
    {
      "id": "2d.drawing",
      "name": "2D Drawing",
      "presentation": {
        "sourceFormat": "draw.document",
        "componentKind": "draw",
        "dimension": "2d"
      }
    },
    {
      "id": "2d.raster",
      "name": "2D Raster",
      "presentation": {
        "sourceFormat": "raster.document",
        "componentKind": "raster",
        "dimension": "2d"
      }
    },
    {
      "id": "2d.map",
      "name": "2D Map",
      "presentation": {
        "sourceFormat": "gis.map",
        "componentKind": "gismap",
        "dimension": "2d"
      }
    },
    {
      "id": "2d.procedural",
      "name": "2D Procedural",
      "presentation": {
        "sourceFormat": "procedural.2d",
        "componentKind": "puzzle2d",
        "dimension": "2d"
      }
    },
    {
      "id": "2d.shooting",
      "name": "2D Shooting",
      "presentation": {
        "sourceFormat": "shooting.scene",
        "componentKind": "shooting",
        "dimension": "2d"
      }
    },
    {
      "id": "2d.puzzle",
      "name": "2D Puzzle",
      "presentation": {
        "sourceFormat": "puzzle.2d",
        "componentKind": "puzzle2d",
        "dimension": "2d"
      }
    },
    {
      "id": "3d.puzzle",
      "name": "3D Puzzle",
      "presentation": {
        "sourceFormat": "puzzle.3d",
        "componentKind": "puzzle3d",
        "dimension": "3d"
      }
    },
    {
      "id": "5d.puzzle",
      "name": "5D Puzzle",
      "presentation": {
        "sourceFormat": "puzzle.5d",
        "componentKind": "puzzle5d",
        "dimension": "5d"
      }
    },
    {
      "id": "3d.procedural",
      "name": "3D Procedural",
      "presentation": {
        "sourceFormat": "procedural.3d",
        "componentKind": "puzzle3d",
        "dimension": "3d"
      }
    },
    {
      "id": "3d.process",
      "name": "3D Process",
      "presentation": {
        "sourceFormat": "process.3d",
        "componentKind": "puzzle3d",
        "dimension": "3d"
      }
    },
    {
      "id": "3d.cad",
      "name": "3D CAD",
      "presentation": {
        "sourceFormat": "cad.scene",
        "componentKind": "cad",
        "dimension": "3d"
      }
    },
    {
      "id": "computation.flow",
      "name": "Flow",
      "presentation": {
        "sourceFormat": "flow.document",
        "componentKind": "flow",
        "dimension": "graph"
      }
    },
    {
      "id": "graph.trinity",
      "name": "Trinity Graph",
      "presentation": {
        "sourceFormat": "trinity.graph",
        "componentKind": "trinity",
        "dimension": "graph"
      }
    },
    {
      "id": "graph.dag",
      "name": "DAG",
      "presentation": {
        "sourceFormat": "flow.dag",
        "componentKind": "dag",
        "dimension": "graph"
      }
    },
    {
      "id": "text.document",
      "name": "Text Document",
      "presentation": {
        "sourceFormat": "writer.document",
        "componentKind": "writer",
        "dimension": "text"
      }
    },
    {
      "id": "form.dictionary",
      "name": "Form Dictionary",
      "presentation": {
        "sourceFormat": "forms.dictionary",
        "componentKind": "forms",
        "dimension": "data"
      }
    },
    {
      "id": "kit.compose",
      "name": "Compose Kit",
      "presentation": {
        "sourceFormat": "compose.kit",
        "componentKind": "virtualFileSystem",
        "dimension": "kit"
      }
    },
    {
      "id": "animate.present.deck",
      "name": "Animate Present Deck",
      "presentation": {
        "sourceFormat": "animate.present.deck",
        "componentKind": "panel",
        "dimension": "2d"
      }
    },
    {
      "id": "3d.mesh",
      "name": "3D Mesh",
      "presentation": {
        "sourceFormat": "mesh.reference",
        "componentKind": "mesh",
        "dimension": "3d"
      }
    },
    {
      "id": "catalogue.kinds",
      "name": "Kind Catalogue",
      "presentation": {
        "sourceFormat": "catalogue.kinds",
        "componentKind": "catalogue",
        "dimension": "data"
      }
    },
    {
      "id": "3d.lowpoly",
      "name": "3D Lowpoly",
      "presentation": {
        "sourceFormat": "lowpoly.fixture",
        "componentKind": "lowpoly",
        "dimension": "3d"
      }
    },
    {
      "id": "computation.sequence",
      "name": "Sequence",
      "presentation": {
        "sourceFormat": "sequence.fixture",
        "componentKind": "sequence",
        "dimension": "graph"
      }
    },
    {
      "id": "2d.layout",
      "name": "Layout",
      "presentation": {
        "sourceFormat": "layout.fixture",
        "componentKind": "layout",
        "dimension": "2d"
      }
    },
    {
      "id": "computation.imperative",
      "name": "Imperative",
      "presentation": {
        "sourceFormat": "imperative.document",
        "componentKind": "imperative",
        "dimension": "graph"
      }
    },
    {
      "id": "vcs.document",
      "name": "VCS Document",
      "presentation": {
        "sourceFormat": "vcs.demo",
        "componentKind": "vcs",
        "dimension": "data"
      }
    },
    {
      "id": "parameter.value",
      "name": "Parameter",
      "presentation": {
        "sourceFormat": "parameter.value",
        "componentKind": "parameter",
        "dimension": "data"
      }
    },
    {
      "id": "catalogue.sourcing",
      "name": "Sourcing Curation",
      "presentation": {
        "sourceFormat": "sourcing.curate",
        "componentKind": "catalogue",
        "dimension": "data"
      }
    }
  ]
} as const satisfies GraphManifestDocument;

export function s_resourcesManifestCatalogBundle(): KindCatalogBundle {
  const doc = SRESOURCES_MANIFEST_DOCUMENT;
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
