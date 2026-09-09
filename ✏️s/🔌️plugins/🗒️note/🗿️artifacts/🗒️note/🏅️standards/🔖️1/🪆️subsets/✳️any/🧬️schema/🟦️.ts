/** 🧬️ Note artifact schema — every field with its state class. */

export interface NoteArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  blocks: NoteBlockNode[];
  /** @state artifact */
  gridVisible?: boolean;
  /** @state artifact */
  gridSpacing?: number;
  /** @state artifact */
  gridSubdivisions?: number;
  /** @state artifact */
  gridOpacity?: number;
  /** @state artifact */
  snapEnabled?: boolean;
  /** @state artifact */
  snapGridSpacing?: number;
  /** @state artifact */
  pencilWidth?: number;
  /** @state artifact */
  eraserRadius?: number;
  /** @state artifact */
  assets: Record<string, NoteImageAsset>;
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
export class noteNoteArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const noteNoteArtifactGuardReject = (at: string, why: string): never => {
  throw new noteNoteArtifactGuardRefusal(at, why);
};

type noteNoteArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type noteNoteArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type noteNoteArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const noteNoteArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : noteNoteArtifactGuardReject(at, "value is not an object");
export const noteNoteArtifactGuardArray = (value: unknown, at: string, bounds: noteNoteArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return noteNoteArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) noteNoteArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) noteNoteArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const noteNoteArtifactGuardString = (value: unknown, at: string, bounds: noteNoteArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return noteNoteArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) noteNoteArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) noteNoteArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) noteNoteArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const noteNoteArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : noteNoteArtifactGuardReject(at, "value is not a boolean"));
export const noteNoteArtifactGuardNumber = (value: unknown, at: string, bounds: noteNoteArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return noteNoteArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) noteNoteArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) noteNoteArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const noteNoteArtifactGuardInteger = (value: unknown, at: string, bounds: noteNoteArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? noteNoteArtifactGuardNumber(value, at, bounds) : noteNoteArtifactGuardReject(at, "value is not an integer");
export const noteNoteArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : noteNoteArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const noteNoteArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : noteNoteArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNoteArtifact(value: unknown, at = "$"): NoteArtifact {
  const row = noteNoteArtifactGuardObject(value, at);
  return {
    schema: noteNoteArtifactGuardString(row["schema"], `${at}.schema`),
    id: noteNoteArtifactGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : noteNoteArtifactGuardString(row["title"], `${at}.title`),
    blocks: noteNoteArtifactGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => parseNoteBlockNode(item, `${at}.blocks[${index}]`)),
    gridVisible: row["gridVisible"] === undefined ? undefined : noteNoteArtifactGuardBoolean(row["gridVisible"], `${at}.gridVisible`),
    gridSpacing: row["gridSpacing"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["gridSpacing"], `${at}.gridSpacing`),
    gridSubdivisions: row["gridSubdivisions"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["gridSubdivisions"], `${at}.gridSubdivisions`),
    gridOpacity: row["gridOpacity"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["gridOpacity"], `${at}.gridOpacity`),
    snapEnabled: row["snapEnabled"] === undefined ? undefined : noteNoteArtifactGuardBoolean(row["snapEnabled"], `${at}.snapEnabled`),
    snapGridSpacing: row["snapGridSpacing"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["snapGridSpacing"], `${at}.snapGridSpacing`),
    pencilWidth: row["pencilWidth"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["pencilWidth"], `${at}.pencilWidth`),
    eraserRadius: row["eraserRadius"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["eraserRadius"], `${at}.eraserRadius`),
    assets: noteNoteArtifactGuardObject(row["assets"], `${at}.assets`),
  };
}

export function parseNoteBlockNode(value: unknown, at = "$"): NoteBlockNode {
  const row = noteNoteArtifactGuardObject(value, at);
  return {
    kind: noteNoteArtifactGuardString(row["kind"], `${at}.kind`),
  };
}

export function parseNoteImageAsset(value: unknown, at = "$"): NoteImageAsset {
  const row = noteNoteArtifactGuardObject(value, at);
  return {
    mime: noteNoteArtifactGuardString(row["mime"], `${at}.mime`),
    data: noteNoteArtifactGuardString(row["data"], `${at}.data`),
    width: row["width"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : noteNoteArtifactGuardNumber(row["height"], `${at}.height`),
  };
}
