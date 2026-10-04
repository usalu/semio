/** 🧬️ SemioModelMutation union mirror — named-variant vocabulary, discriminated on `mutation`. */

import type { SemioModelSnapshot, SpatialNode, SemioModelElement, ModelRelation, SpatialKind, ElementClass, GeometryRef, PropertySet, RelationKind, SemioTransform } from "../📸️snapshot/🟦️.ts";
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';

export type SemioModelMutation =
  | { mutation: "setSnapshot"; snapshot: SemioModelSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: "insertSpatialNode"; node: SpatialNode }
  | { mutation: "removeSpatialNode"; id: string }
  | { mutation: "setSpatialNode"; id: string; kind?: SpatialKind; name?: string; parentId?: string | null; placement?: SemioTransform }
  | { mutation: "insertElement"; element: SemioModelElement }
  | { mutation: "removeElement"; id: string }
  | { mutation: "setElement"; id: string; class?: ElementClass; placement?: SemioTransform; geometry?: GeometryRef; spatialId?: string | null; psets?: PropertySet[] }
  | { mutation: "insertRelation"; relation: ModelRelation }
  | { mutation: "removeRelation"; id: string }
  | { mutation: "setRelation"; id: string; kind?: RelationKind; from?: string; to?: string };
