/** 🧬️ SemioModelSnapshot schema mirror — spatial hierarchy + elements + relations, from ifc/4.
 * Real interfaces matching the Rust `🦀️.rs` sibling's serde shape (camelCase, tagged
 * unions on `kind`/`mutation` discriminants). Source of truth stays the Rust file. */

export interface SemioTransform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}

export type SpatialKind = "site" | "building" | "storey" | "space";

export interface SpatialNode {
  id: string;
  kind: SpatialKind;
  name: string;
  parentId?: string | null;
  placement: SemioTransform;
}

export type ElementClass =
  | { kind: "wall" }
  | { kind: "slab" }
  | { kind: "column" }
  | { kind: "beam" }
  | { kind: "door" }
  | { kind: "window" }
  | { kind: "roof" }
  | { kind: "stair" }
  | { kind: "furniture" }
  | { kind: "other"; name: string };

export type GeometryRef =
  | { kind: "none" }
  | { kind: "brep"; brepId: string }
  | { kind: "mesh"; meshId: string };

export type PsetValue =
  | { kind: "text"; value: string }
  | { kind: "number"; value: number }
  | { kind: "boolean"; value: boolean };

export interface Property {
  key: string;
  value: PsetValue;
}

export interface PropertySet {
  name: string;
  properties: Property[];
}

export interface SemioModelElement {
  id: string;
  class: ElementClass;
  placement: SemioTransform;
  geometry: GeometryRef;
  spatialId?: string | null;
  psets: PropertySet[];
}

export type RelationKind =
  | { kind: "aggregates" }
  | { kind: "containedIn" }
  | { kind: "connectsTo" }
  | { kind: "fillsVoid" }
  | { kind: "voidsElement" }
  | { kind: "other"; label: string };

export interface ModelRelation {
  id: string;
  kind: RelationKind;
  from: string;
  to: string;
}

export interface SemioModelSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ spatial: SpatialNode[];
  /** @state artifact */ elements: SemioModelElement[];
  /** @state artifact */ relations: ModelRelation[];
}
