/** 🧬️ Puzzle5d artifact schema — every field with its state class. */

export interface Puzzle5dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  domain: string;
  /** @state persistent */
  label?: string;
  /** @state persistent */
  meta: Puzzle5dMeta;
  /** @state persistent */
  kindCatalogs?: Puzzle5dKindCatalogs;
  /** @state persistent */
  kindCompatibility: Puzzle5dKindCompatibility[];
  /** @state persistent */
  parts: Puzzle5dPart[];
  /** @state persistent */
  fasteners: Puzzle5dFastener[];
  /** @state shared-ui */
  selectedPartIds: string[];
  /** @state shared-ui */
  selectedGripIds: string[];
  /** @state shared-ui */
  selectedFastenerIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  camera2dX: number;
  /** @state local-ui */
  camera2dY: number;
  /** @state local-ui */
  camera2dZoom: number;
  /** @state local-ui */
  camera3dPositionX: number;
  /** @state local-ui */
  camera3dPositionY: number;
  /** @state local-ui */
  camera3dPositionZ: number;
  /** @state local-ui */
  camera3dTargetX: number;
  /** @state local-ui */
  camera3dTargetY: number;
  /** @state local-ui */
  camera3dTargetZ: number;
  /** @state local-ui */
  camera3dZoom: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  suggestionOffset: number;
  /** @state local-ui */
  overlapBudget: number;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  runtimeExtrasJson: string;
  /** @state preview */
  hoveredPartId?: string;
  /** @state preview */
  previewSeq: number;
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

/** 🗂️ Kind catalogs bundle. */
export interface Puzzle5dKindCatalogs {
  parts?: Puzzle5dCatalogPartKind[];
  grips?: Puzzle5dCatalogGripKind[];
  fasteners?: Puzzle5dCatalogFastenerKind[];
  ropes?: Puzzle5dCatalogRopeKind[];
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
