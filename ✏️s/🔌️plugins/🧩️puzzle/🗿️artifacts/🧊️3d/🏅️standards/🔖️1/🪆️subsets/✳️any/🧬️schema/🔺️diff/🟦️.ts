/** 🧬️ Puzzle3d diff schema — sparse field delta. */

export interface Puzzle3dDiff {
  /** @state artifact */
  artifact?: Puzzle3dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  domain?: string;
  /** @state artifact */
  meta?: Puzzle3dMeta;
  /** @state artifact */
  objects?: Puzzle3dObjectsDelta;
  /** @state artifact */
  attractions?: Puzzle3dAttractionsDelta;
  /** @state artifact */
  targetVolumes?: Puzzle3dTargetVolumesDelta;
  /** @state artifact */
  references?: Puzzle3dReferencesDelta;
  /** @state presence */
  selectedObjectIds?: Puzzle3dStringList;
  /** @state presence */
  selectedVortexIds?: Puzzle3dStringList;
  /** @state presence */
  selectedAttractionIds?: Puzzle3dStringList;
  /** @state presence */
  selectedTargetVolumeIds?: Puzzle3dStringList;
  /** @state presence */
  selectedReferenceIds?: Puzzle3dStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  cameraPositionX?: number;
  /** @state config */
  cameraPositionY?: number;
  /** @state config */
  cameraPositionZ?: number;
  /** @state config */
  cameraTargetX?: number;
  /** @state config */
  cameraTargetY?: number;
  /** @state config */
  cameraTargetZ?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  selectionModeDefault?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  gridVisible?: boolean;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridSpacing?: number;
  /** @state config */
  overlapBudget?: number;
  /** @state config */
  fillCount?: number;
  /** @state config */
  brushCandidateIndex?: number;
  /** @state config */
  lodAutomatic?: boolean;
  /** @state config */
  lodDepthVariable?: boolean;
  /** @state config */
  lodManual?: number;
  /** @state config */
  proximityRadius?: number;
  /** @state config */
  /** @state config */
  runtimeExtrasJson?: string;
  /** @state artifact */
  hoveredObjectId?: string | null;
  /** @state artifact */
  hoveredVortexFullId?: string | null;
  /** @state artifact */
  hoveredKindId?: string | null;
  /** @state artifact */
  previewSeq?: number;
}

export interface Puzzle3dStringList { values: string[]; }
export interface Puzzle3dObjectsDelta { added: Puzzle3dObject[]; removed: string[]; patched: Puzzle3dObjectPatchEntry[]; reordered?: string[]; }
export interface Puzzle3dObjectPatchEntry { id: string; patch: Puzzle3dObjectPatch; }
export interface Puzzle3dObjectPatch { replacement?: Puzzle3dObject; }
export interface Puzzle3dAttractionsDelta { added: Puzzle3dAttraction[]; removed: string[]; patched: Puzzle3dAttractionPatchEntry[]; reordered?: string[]; }
export interface Puzzle3dAttractionPatchEntry { id: string; patch: Puzzle3dAttractionPatch; }
export interface Puzzle3dAttractionPatch { replacement?: Puzzle3dAttraction; }
export interface Puzzle3dTargetVolumesDelta { added: Puzzle3dTargetVolume[]; removed: string[]; patched: Puzzle3dTargetVolumePatchEntry[]; reordered?: string[]; }
export interface Puzzle3dTargetVolumePatchEntry { id: string; patch: Puzzle3dTargetVolumePatch; }
export interface Puzzle3dTargetVolumePatch { replacement?: Puzzle3dTargetVolume; }
export interface Puzzle3dTargetVolume { id: string; [key: string]: unknown; }
export interface Puzzle3dReferencesDelta { added: Puzzle3dReference[]; removed: string[]; patched: Puzzle3dReferencePatchEntry[]; reordered?: string[]; }
export interface Puzzle3dReferencePatchEntry { id: string; patch: Puzzle3dReferencePatch; }
export interface Puzzle3dReferencePatch { replacement?: Puzzle3dReference; }
export interface Puzzle3dReference { id: string; [key: string]: unknown; }
export interface Puzzle3dArtifact { [key: string]: unknown; }

export type Puzzle3dObjectAnchor = "fixed" | "derived";
export type Puzzle3dCompatSpecificity = "general" | "object" | "attraction" | "vortex" | "cable";

export interface Puzzle3dVortex {
  id: string;
  vortexKind?: string;
  label?: string;
  position: [number, number, number];
  direction?: [number, number, number];
  radius?: number;
  hidden?: boolean;
  locked?: boolean;
}

export interface Puzzle3dObject {
  id: string;
  label?: string;
  objectKind?: string;
  anchor?: Puzzle3dObjectAnchor;
  origin: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: number | [number, number, number];
  meshUrl?: string;
  vortices?: Puzzle3dVortex[];
  hidden?: boolean;
  locked?: boolean;
}

export interface Puzzle3dAttraction {
  id?: string;
  attracting: string;
  attracted: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}

export interface Puzzle3dRepresentation {
  id: string;
  name: string;
  url: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

export interface Puzzle3dCatalogVortexTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  vortexKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

export interface Puzzle3dCatalogObjectKind {
  id: string;
  name: string;
  label: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle3dRepresentation[];
  vortices?: Puzzle3dCatalogVortexTemplate[];
  attributes?: Array<{ id?: string; key: string; value: string; definition?: string }>;
  authors?: Array<{ id?: string; name: string; email?: string; role?: string; rank?: number }>;
}

export interface Puzzle3dCatalogVortexKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultCableKind?: string;
}

export interface Puzzle3dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle3dCompatSpecificity;
}

export interface Puzzle3dMeta {
  kindCatalogs?: {
    objects?: Puzzle3dCatalogObjectKind[];
    vortices?: Puzzle3dCatalogVortexKind[];
    cables?: unknown[];
    attractions?: unknown[];
  };
  kindCompatibility?: Puzzle3dKindCompatibility[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dDiffGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dDiffGuardRefusal(at, why);
};

type puzzlePuzzle3dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dDiffGuardReject(at, "value is not an object");
export const puzzlePuzzle3dDiffGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dDiffGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dDiffGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dDiffGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dDiffGuardNumber(value, at, bounds) : puzzlePuzzle3dDiffGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dStringList(value: unknown, at = "$"): Puzzle3dStringList {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    values: puzzlePuzzle3dDiffGuardArray(row["values"], `${at}.values`).map((item, index) => puzzlePuzzle3dDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parsePuzzle3dObjectPatchEntry(value: unknown, at = "$"): Puzzle3dObjectPatchEntry {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dObjectPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dAttractionPatchEntry(value: unknown, at = "$"): Puzzle3dAttractionPatchEntry {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dAttractionPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dTargetVolumePatchEntry(value: unknown, at = "$"): Puzzle3dTargetVolumePatchEntry {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dTargetVolumePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dReferencePatchEntry(value: unknown, at = "$"): Puzzle3dReferencePatchEntry {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dReferencePatch(row["patch"], `${at}.patch`),
  };
}
