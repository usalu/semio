/** 🧬️ Puzzle5d nested schema types (design-parity). */

/** ⚓️ Part root plane policy. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🔗️ Compat row specificity. */
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

/** 🧬️ Puzzle5d diff schema — sparse field delta. */

export interface Puzzle5dDiff {
  /** @state artifact */
  artifact?: Puzzle5dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  domain?: string;
  /** @state artifact */
  label?: string | null;
  /** @state artifact */
  meta?: Puzzle5dMeta;
  /** @state artifact */
  kindCatalogs?: Puzzle5dKindCatalogs | null;
  /** @state artifact */
  kindCompatibility?: Puzzle5dKindCompatibilityList;
  /** @state artifact */
  parts?: Puzzle5dPartsDelta;
  /** @state artifact */
  fasteners?: Puzzle5dFastenersDelta;
  /** @state presence */
  selectedPartIds?: Puzzle5dStringList;
  /** @state presence */
  selectedGripIds?: Puzzle5dStringList;
  /** @state presence */
  selectedFastenerIds?: Puzzle5dStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  camera2dX?: number;
  /** @state config */
  camera2dY?: number;
  /** @state config */
  camera2dZoom?: number;
  /** @state config */
  camera3dPositionX?: number;
  /** @state config */
  camera3dPositionY?: number;
  /** @state config */
  camera3dPositionZ?: number;
  /** @state config */
  camera3dTargetX?: number;
  /** @state config */
  camera3dTargetY?: number;
  /** @state config */
  camera3dTargetZ?: number;
  /** @state config */
  camera3dZoom?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridFactor?: number;
  /** @state config */
  suggestionOffset?: number;
  /** @state config */
  overlapBudget?: number;
  /** @state config */
  fillCount?: number;
  /** @state config */
  brushCandidateIndex?: number;
  /** @state config */
  lodMode?: string;
  /** @state config */
  /** @state config */
  runtimeExtrasJson?: string;
  /** @state artifact */
  hoveredPartId?: string | null;
  /** @state artifact */
  previewSeq?: number;
}

export interface Puzzle5dStringList { values: string[]; }
export interface Puzzle5dPartsDelta { added: Puzzle5dPart[]; removed: string[]; patched: Puzzle5dPartPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dPartPatchEntry { id: string; patch: Puzzle5dPartPatch; }
export interface Puzzle5dPartPatch { replacement?: Puzzle5dPart; }
export interface Puzzle5dPart { id: string; partKind?: string; anchor?: Puzzle5dPartAnchor; [key: string]: unknown; }
export interface Puzzle5dFastenersDelta { added: Puzzle5dFastener[]; removed: string[]; patched: Puzzle5dFastenerPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dFastenerPatchEntry { id: string; patch: Puzzle5dFastenerPatch; }
export interface Puzzle5dFastenerPatch { replacement?: Puzzle5dFastener; }
export interface Puzzle5dFastener { id: string; source?: string; target?: string; gap?: number; shift?: number; rise?: number; rotation?: number; turn?: number; tilt?: number; x?: number; y?: number; [key: string]: unknown; }
export interface Puzzle5dKindCompatibility { source?: string; target?: string; bidirectional?: boolean; important?: boolean; specificity?: Puzzle5dCompatSpecificity; [key: string]: unknown; }
export interface Puzzle5dArtifact { [key: string]: unknown; }
export interface Puzzle5dMeta { [key: string]: unknown; }
export interface Puzzle5dKindCatalogs { [key: string]: unknown; }

export interface Puzzle5dKindCompatibilityList { values: Puzzle5dKindCompatibility[]; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dDiffGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dDiffGuardRefusal(at, why);
};

type puzzlePuzzle5dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dDiffGuardReject(at, "value is not an object");
export const puzzlePuzzle5dDiffGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dDiffGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dDiffGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dDiffGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dDiffGuardNumber(value, at, bounds) : puzzlePuzzle5dDiffGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dStringList(value: unknown, at = "$"): Puzzle5dStringList {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    values: puzzlePuzzle5dDiffGuardArray(row["values"], `${at}.values`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parsePuzzle5dPartPatchEntry(value: unknown, at = "$"): Puzzle5dPartPatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle5dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle5dPartPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle5dFastenerPatchEntry(value: unknown, at = "$"): Puzzle5dFastenerPatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle5dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle5dFastenerPatch(row["patch"], `${at}.patch`),
  };
}
