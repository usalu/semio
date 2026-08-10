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
  /** @state persistent */
  artifact?: CurateArtifact | null;
  /** @state persistent */
  stock?: CurateStockDelta | null;
  /** @state persistent */
  curated?: CurateCuratedDelta | null;
  /** @state local-ui */
  filters?: Filters | null;
  /** @state shared-ui */
  selectedObjectId?: string | null;
  /** @state local-ui */
  locale?: string | null;
  /** @state local-ui */
  contributionsJson?: string | null;
}
