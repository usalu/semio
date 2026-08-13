/** 🧬️ Curate snapshot schema — artifact-lane fields only. */

export type SortDirection = "asc" | "desc";

export interface TableSort {
  columnId: string;
  direction: SortDirection;
}

export interface Filters {
  query: string;
  moduleIds: string[];
  typologyPath: string[];
  minAvailability: number;
  sort?: TableSort | null;
}

export type GeometryRecipe =
  | { kind: "box"; width: number; height: number; depth: number }
  | { kind: "frame"; width: number; height: number; depth: number; profile: number }
  | { kind: "slab"; width: number; depth: number; thickness: number }
  | { kind: "mesh"; positions: number[]; normals: number[]; indices: number[] };

export interface ObjectKind {
  id: string;
  name: string;
  moduleId: string;
  typologyPath: string[];
  availability: number;
  geometry: GeometryRecipe;
}

/** 🧩️ Sourcing-owned overflow half of `ObjectKind` not representable in the composed
 * `s.stdio.semio.kit` subset's `SemioKitType` (id/name/category only). */
export interface ObjectKindExtra {
  id: string;
  typologyPath: string[];
  availability: number;
  geometry: GeometryRecipe;
}

export interface ArtifactChildHandle { childId: string; target: string; }

export interface CuratedItem {
  objectId: string;
  count: number;
}

export interface CurateSnapshot {
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog: ArtifactChildHandle;
  /** @state artifact */
  stockExtra: ObjectKindExtra[];
  /** @state artifact */
  curated: CuratedItem[];
}
