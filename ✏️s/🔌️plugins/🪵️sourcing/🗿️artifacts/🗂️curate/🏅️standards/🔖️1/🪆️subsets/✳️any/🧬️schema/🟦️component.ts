/** 🧬️ Curate artifact schema — every field with its state class. */

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

export interface CurateArtifact {
  /** @state persistent */
  stock: ObjectKind[];
  /** @state persistent */
  curated: CuratedItem[];
  /** @state local-ui */
  filters: Filters;
  /** @state shared-ui */
  selectedObjectId?: string | null;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  contributionsJson: string;
}
