/** 🧬️ Curation diff schema — sparse field delta. */
import type { CurationArtifact } from "../🟦️.ts";

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

export interface CurationStringList {
  values: string[];
}

/** 🧩️ Sourcing-owned overflow half of `ObjectKind` not representable in the composed
 * `s.stdio.semio.kit` subset's `SemioKitType` (id/name/category only). */
export interface ObjectKindExtra {
  id: string;
  name: string;
  moduleId: string;
  typologyPath: string[];
  availability: number;
  geometry: GeometryRecipe;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

export interface CurationObjectKindExtraPatchEntry {
  id: string;
  extra: ObjectKindExtra;
}

export interface CurationStockExtraDelta {
  added?: ObjectKindExtra[];
  removed?: string[];
  patched?: CurationObjectKindExtraPatchEntry[];
  reordered?: string[];
}

export interface CurationCuratedPatchEntry {
  objectId: string;
  count?: number;
}

export interface CurationCuratedDelta {
  added?: CuratedItem[];
  removed?: string[];
  patched?: CurationCuratedPatchEntry[];
  reordered?: string[];
}

export interface CurationDiff {
  /** @state artifact */
  artifact?: CurationArtifact | null;
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog?: ArtifactChildHandle | null;
  /** @state artifact */
  stockExtra?: CurationStockExtraDelta | null;
  /** @state artifact */
  curated?: CurationCuratedDelta | null;
  /** @state config */
  filters?: Filters | null;
  /** @state config */
  locale?: string | null;
  /** @state config */
  contributionsJson?: string | null;
}
