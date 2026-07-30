/** Generated graph manifest shared types */

export interface GraphManifestPropertyDef {
  readonly name: string;
  readonly kind: "data" | "derived";
  readonly valueType?: unknown;
  readonly expr?: string;
}

export interface GraphManifestKindRow {
  readonly id: string;
  readonly name?: string;
  readonly properties?: readonly GraphManifestPropertyDef[];
  readonly ports?: readonly string[];
  readonly presentation?: Readonly<Record<string, unknown>>;
}

export interface GraphManifestDocument {
  readonly schema: "manifest";
  readonly id: string;
  readonly name?: string;
  readonly axes?: { readonly portModel?: "normal" | "ported"; readonly directedness?: "directed" | "undirected" };
  readonly nodeKinds?: readonly GraphManifestKindRow[];
  readonly edgeKinds?: readonly GraphManifestKindRow[];
  readonly portKinds?: readonly GraphManifestKindRow[];
  readonly wireKinds?: readonly GraphManifestKindRow[];
  readonly layerKinds?: readonly GraphManifestKindRow[];
  readonly languageKinds?: readonly GraphManifestKindRow[];
  readonly surfaceKinds?: readonly GraphManifestKindRow[];
  readonly windowKinds?: readonly GraphManifestKindRow[];
  readonly fileNodeKinds?: readonly GraphManifestKindRow[];
  readonly descriptorKinds?: readonly GraphManifestKindRow[];
  readonly edgeTips?: readonly Record<string, unknown>[];
  readonly kindCompatibility?: readonly Record<string, unknown>[];
}

export interface HandleKind {
  readonly color: string;
  readonly defaultWireKind?: string;
  readonly id: string;
  readonly name: string;
}

export interface WireKind {
  readonly defaultEdgeKind?: string;
  readonly id: string;
  readonly name: string;
}

export interface NodeKindHandleTemplate {
  readonly handleKind: string;
  readonly angle: number;
  readonly radius?: number;
}

export interface NodeKind {
  readonly color?: string;
  readonly defaultHandleKind?: string;
  readonly icon?: string;
  readonly id: string;
  readonly name: string;
  readonly stroke?: string;
  readonly handles?: readonly NodeKindHandleTemplate[];
}

export interface EdgeTip {
  readonly filled?: boolean;
  readonly geometry?: "arrow" | "fine-arrow" | "diamond" | "circle" | "bar";
  readonly id: string;
  readonly scale?: number;
}

export interface EdgeKind {
  readonly color?: string;
  readonly directed?: boolean;
  readonly id: string;
  readonly name: string;
  readonly pattern?: string;
  readonly shape?: "bezier" | "line";
  readonly sourceTip?: string;
  readonly stroke?: string;
  readonly targetTip?: string;
}

export interface KindCatalogBundle {
  readonly edgeTips?: readonly EdgeTip[];
  readonly edges?: readonly EdgeKind[];
  readonly handles?: readonly HandleKind[];
  readonly nodes?: readonly NodeKind[];
  readonly wires?: readonly WireKind[];
}

export const MANIFEST_IDS = ["draw-layers", "flow-dag", "note-blocks", "concrete-forest", "puzzle2d-default", "puzzle3d-default", "puzzle5d-default", "wires", "s-resources", "nakagin", "rewrite-lhs", "rewrite-rhs", "writer-languages"] as const;
export type ManifestId = (typeof MANIFEST_IDS)[number];

export function mergeManifestCatalogBundles(...bundles: readonly KindCatalogBundle[]): KindCatalogBundle {
  function mergedSlice<T extends { id: string }>(slices: readonly (readonly T[] | undefined)[]): readonly T[] | undefined {
    const byId = new Map<string, T>();
    let any = false;
    for (const slice of slices) {
      if (!slice) continue;
      any = true;
      for (const row of slice) {
        byId.set(row.id, row);
      }
    }
    if (!any) return undefined;
    return [...byId.values()].sort((left, right) => left.id.localeCompare(right.id));
  }
  return {
    edgeTips: mergedSlice(bundles.map((bundle) => bundle.edgeTips)),
    edges: mergedSlice(bundles.map((bundle) => bundle.edges)),
    handles: mergedSlice(bundles.map((bundle) => bundle.handles)),
    nodes: mergedSlice(bundles.map((bundle) => bundle.nodes)),
    wires: mergedSlice(bundles.map((bundle) => bundle.wires)),
  };
}
