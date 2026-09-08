/** 🧬️ Note diff schema — sparse field delta. */

export interface NoteDiff {
  /** @state artifact */
  artifact?: NoteArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  blocks?: NoteBlocksDelta;
  /** @state artifact */
  gridVisible?: boolean | null;
  /** @state artifact */
  gridSpacing?: number | null;
  /** @state artifact */
  gridSubdivisions?: number | null;
  /** @state artifact */
  gridOpacity?: number | null;
  /** @state artifact */
  snapEnabled?: boolean | null;
  /** @state artifact */
  snapGridSpacing?: number | null;
  /** @state artifact */
  pencilWidth?: number | null;
  /** @state artifact */
  eraserRadius?: number | null;
  /** @state artifact */
  assets?: NoteAssetsDelta;
  /** @state presence */
  selectedBlockIds?: NoteStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  /** @state artifact */
  hoveredBlockId?: string | null;
}

export interface NoteArtifact {
  schema: string;
  id: string;
  title?: string;
  blocks: NoteBlockNode[];
  gridVisible?: boolean;
  gridSpacing?: number;
  gridSubdivisions?: number;
  gridOpacity?: number;
  snapEnabled?: boolean;
  snapGridSpacing?: number;
  pencilWidth?: number;
  eraserRadius?: number;
  assets: Record<string, NoteImageAsset>;
  selectedBlockIds: string[];
  activeUtilityId: string;
  engagementInput: string;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  hoveredBlockId?: string;
}

export interface NoteAssetsDelta {
  entries: Record<string, NoteImageAsset | null>;
}

export interface NoteStringList {
  values: string[];
}

export interface NoteBlocksDelta {
  added: NoteBlockNode[];
  removed: string[];
  patched: NoteBlockPatchEntry[];
  reordered?: string[];
}

export interface NoteBlockPatchEntry {
  id: string;
  patch: NoteBlockPatch;
}

export interface NoteBlockPatch {
  blockJson?: string;
}

export interface NoteBlockNode {
  kind: string;
  [key: string]: unknown;
}

export interface NoteImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class noteNoteDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const noteNoteDiffGuardReject = (at: string, why: string): never => {
  throw new noteNoteDiffGuardRefusal(at, why);
};

type noteNoteDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type noteNoteDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type noteNoteDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const noteNoteDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : noteNoteDiffGuardReject(at, "value is not an object");
export const noteNoteDiffGuardArray = (value: unknown, at: string, bounds: noteNoteDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return noteNoteDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) noteNoteDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) noteNoteDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const noteNoteDiffGuardString = (value: unknown, at: string, bounds: noteNoteDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return noteNoteDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) noteNoteDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) noteNoteDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) noteNoteDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const noteNoteDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : noteNoteDiffGuardReject(at, "value is not a boolean"));
export const noteNoteDiffGuardNumber = (value: unknown, at: string, bounds: noteNoteDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return noteNoteDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) noteNoteDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) noteNoteDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const noteNoteDiffGuardInteger = (value: unknown, at: string, bounds: noteNoteDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? noteNoteDiffGuardNumber(value, at, bounds) : noteNoteDiffGuardReject(at, "value is not an integer");
export const noteNoteDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : noteNoteDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const noteNoteDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : noteNoteDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNoteAssetsDelta(value: unknown, at = "$"): NoteAssetsDelta {
  const row = noteNoteDiffGuardObject(value, at);
  return {
    entries: noteNoteDiffGuardObject(row["entries"], `${at}.entries`),
  };
}

export function parseNoteStringList(value: unknown, at = "$"): NoteStringList {
  const row = noteNoteDiffGuardObject(value, at);
  return {
    values: noteNoteDiffGuardArray(row["values"], `${at}.values`).map((item, index) => noteNoteDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseNoteBlockPatchEntry(value: unknown, at = "$"): NoteBlockPatchEntry {
  const row = noteNoteDiffGuardObject(value, at);
  return {
    id: noteNoteDiffGuardString(row["id"], `${at}.id`),
    patch: parseNoteBlockPatch(row["patch"], `${at}.patch`),
  };
}

export function parseNoteBlockPatch(value: unknown, at = "$"): NoteBlockPatch {
  const row = noteNoteDiffGuardObject(value, at);
  return {
    blockJson: row["blockJson"] === undefined ? undefined : noteNoteDiffGuardString(row["blockJson"], `${at}.blockJson`),
  };
}

export function parseNoteArtifact(value: unknown, at = "$"): NoteArtifact {
  return noteNoteDiffGuardObject(value, `${at}`);
}
