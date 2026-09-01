/** 🧬️ Puzzle5d snapshot schema — artifact-lane fields only. */

/** 🪪️ Composed-child handle — mirrors stdio's `s.stdio.semio.kit` cross-language convention. */
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

export interface Puzzle5dSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  domain: string;
  /** @state artifact */
  label?: string;
  /** @state artifact */
  meta: Puzzle5dMeta;
  /** @state artifact @child kind=s.stdio.semio.kit */
  kindCatalogs?: ArtifactChildHandle;
  /** @state artifact */
  kindCatalogsExtra?: Puzzle5dKindCatalogsExtra;
  /** @state artifact */
  kindCompatibility: Puzzle5dKindCompatibility[];
  /** @state artifact */
  parts: Puzzle5dPart[];
  /** @state artifact */
  fasteners: Puzzle5dFastener[];
}



/** ⚓️ Part root plane policy. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🔗️ Compat row specificity. */
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

/** 🏷️ Part-kind attribute. */
export interface Puzzle5dAttribute {
  id?: string;
  key?: string;
  value?: string;
  definition?: string;
}

/** ✍️ Part-kind author. */
export interface Puzzle5dAuthor {
  id?: string;
  name?: string;
  email?: string;
  role?: string;
  rank?: number;
}

/** 🖼️ Part-kind representation. */
export interface Puzzle5dRepresentation {
  id?: string;
  name?: string;
  url?: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

/** 🌱️ Grip template on a part-kind. */
export interface Puzzle5dGripTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  gripKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

/** 🧱️ Part-kind catalog row. */
export interface Puzzle5dCatalogPartKind {
  id: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle5dRepresentation[];
  grips?: Puzzle5dGripTemplate[];
  attributes?: Puzzle5dAttribute[];
  authors?: Puzzle5dAuthor[];
}

/** 🔘️ Grip-kind catalog row. */
export interface Puzzle5dCatalogGripKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultRopeKind?: string;
}

/** 🔗️ Fastener-kind catalog row. */
export interface Puzzle5dCatalogFastenerKind {
  id: string;
  name?: string;
  label?: string;
}

/** 🧵️ Rope-kind catalog row. */
export interface Puzzle5dCatalogRopeKind {
  id: string;
  name?: string;
  label?: string;
  defaultFastenerKind?: string;
}

/** 🗂️ Kind catalogs bundle — still the `replace-kind-catalogs` mutation payload shape; the snapshot
 * itself now carries the composed `kindCatalogs`/`kindCatalogsExtra` pair below instead. */
export interface Puzzle5dKindCatalogs {
  parts?: Puzzle5dCatalogPartKind[];
  grips?: Puzzle5dCatalogGripKind[];
  fasteners?: Puzzle5dCatalogFastenerKind[];
  ropes?: Puzzle5dCatalogRopeKind[];
}

/** 🧩️ Puzzle5d-owned overflow for one part-kind row — everything the composed `SemioKitType`
 * (`id`/`name`/`category`) cannot represent. */
export interface Puzzle5dCatalogPartKindExtra {
  id: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle5dRepresentation[];
  grips?: Puzzle5dGripTemplate[];
  attributes?: Puzzle5dAttribute[];
  authors?: Puzzle5dAuthor[];
}

/** 🧩️ Puzzle5d-owned overflow for one grip-kind row. */
export interface Puzzle5dCatalogGripKindExtra {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultRopeKind?: string;
}

/** 🧩️ Puzzle5d-owned overflow for one fastener-kind row. */
export interface Puzzle5dCatalogFastenerKindExtra {
  id: string;
  name?: string;
  label?: string;
}

/** 🧩️ Puzzle5d-owned overflow for one rope-kind row. */
export interface Puzzle5dCatalogRopeKindExtra {
  id: string;
  name?: string;
  label?: string;
  defaultFastenerKind?: string;
}

/** 🗂️ Puzzle5d-owned overflow half of the kind-catalogs bundle, sibling to the composed
 * `kindCatalogs` child. */
export interface Puzzle5dKindCatalogsExtra {
  parts?: Puzzle5dCatalogPartKindExtra[];
  grips?: Puzzle5dCatalogGripKindExtra[];
  fasteners?: Puzzle5dCatalogFastenerKindExtra[];
  ropes?: Puzzle5dCatalogRopeKindExtra[];
}

/** 🔗️ Kind compatibility row. */
export interface Puzzle5dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle5dCompatSpecificity;
}

/** 📝️ Meta. */
export interface Puzzle5dMeta {
  description?: string;
}

/** 🧱️ Part. */
export interface Puzzle5dPart {
  id: string;
  partKind?: string;
  anchor?: Puzzle5dPartAnchor;
  "2d"?: Record<string, unknown>;
  "3d"?: Record<string, unknown>;
  grips?: Record<string, unknown>[];
}

/** 🔗️ Fastener with eight transform params. */
export interface Puzzle5dFastener {
  id: string;
  source: string;
  target: string;
  fastenerKind?: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}
