/** 🧬️ SemioModelMutation union mirror — named-variant vocabulary, discriminated on `mutation`. */

import type { SemioModelSnapshot, SpatialNode, SemioModelElement, ModelRelation, SpatialKind, ElementClass, GeometryRef, PropertySet, RelationKind, SemioTransform } from "../📸️snapshot/🟦️.ts";

export type SemioModelMutation =
  | { mutation: "insertSpatialNode"; node: SpatialNode; at?: number }
  | { mutation: "removeSpatialNode"; id: string }
  | { mutation: "setSpatialNode"; id: string; kind?: SpatialKind; name?: string; parentId?: string | null; placement?: SemioTransform }
  | { mutation: "insertElement"; element: SemioModelElement; at?: number }
  | { mutation: "removeElement"; id: string }
  | { mutation: "setElement"; id: string; class?: ElementClass; placement?: SemioTransform; geometry?: GeometryRef; spatialId?: string | null; psets?: PropertySet[] }
  | { mutation: "insertRelation"; relation: ModelRelation; at?: number }
  | { mutation: "removeRelation"; id: string }
  | { mutation: "setRelation"; id: string; kind?: RelationKind; from?: string; to?: string }
  | { mutation: "dragElements"; targets: string[]; offset: [number, number, number] }
  | { mutation: "rotateElements"; targets: string[]; axis: [number, number, number]; angle: number }
  | { mutation: "scaleElements"; targets: string[]; factors: [number, number, number] };
