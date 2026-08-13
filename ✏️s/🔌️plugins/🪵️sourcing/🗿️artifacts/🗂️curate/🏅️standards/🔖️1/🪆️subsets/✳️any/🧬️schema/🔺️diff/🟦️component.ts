/** 🧬️ Curate diff schema — sparse field delta. */

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

export interface CuratedItem {
  objectId: string;
  count: number;
}

export interface CurateStringList {
  values: string[];
}

export interface CurateObjectKindPatchEntry {
  id: string;
  kind: ObjectKind;
}

export interface CurateStockDelta {
  added?: ObjectKind[];
  removed?: string[];
  patched?: CurateObjectKindPatchEntry[];
  reordered?: string[];
}

export interface CurateCuratedPatchEntry {
  objectId: string;
  count?: number;
}

export interface CurateCuratedDelta {
  added?: CuratedItem[];
  removed?: string[];
  patched?: CurateCuratedPatchEntry[];
  reordered?: string[];
}

export interface CurateDiff {
  /** @state artifact */
  artifact?: CurateArtifact | null;
  /** @state artifact */
  stock?: CurateStockDelta | null;
  /** @state artifact */
  curated?: CurateCuratedDelta | null;
  /** @state config */
  filters?: Filters | null;
  /** @state presence */
  selectedObjectId?: string | null;
  /** @state config */
  locale?: string | null;
  /** @state config */
  contributionsJson?: string | null;
}
