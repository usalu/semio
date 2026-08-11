/** 🔺️ SemioModelDiff schema mirror — sparse per-field diff, id-keyed triples (removed/modified/added)
 * over spatial/elements/relations. Real interfaces matching the Rust `🦀️component.rs` sibling. */

import type { SemioTransform, SpatialKind, ElementClass, GeometryRef, PropertySet, RelationKind } from "../📸️snapshot/🟦️component.ts";

export interface SpatialNodeDiff {
  kind?: SpatialKind;
  name?: string;
  parentId?: string | null;
  placement?: SemioTransform;
}

export interface SemioModelElementDiff {
  class?: ElementClass;
  placement?: SemioTransform;
  geometry?: GeometryRef;
  spatialId?: string | null;
  psets?: PropertySet[];
}

export interface ModelRelationDiff {
  kind?: RelationKind;
  from?: string;
  to?: string;
}

export interface NamedModified<D> {
  key: string;
  diff: D;
}

export interface NamedTripleDiff<D, T> {
  removed: string[];
  modified: NamedModified<D>[];
  added: T[];
}

export interface SemioModelDiff {
  spatial?: NamedTripleDiff<SpatialNodeDiff, unknown>;
  elements?: NamedTripleDiff<SemioModelElementDiff, unknown>;
  relations?: NamedTripleDiff<ModelRelationDiff, unknown>;
}
