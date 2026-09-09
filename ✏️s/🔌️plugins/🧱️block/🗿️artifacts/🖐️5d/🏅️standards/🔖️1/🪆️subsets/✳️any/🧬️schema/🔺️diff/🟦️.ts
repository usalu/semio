/** 🧬️ Block5d diff schema — sparse field delta. */

export interface Block5dDiff {
  /** @state artifact */
  artifact?: Block5dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  partKind?: BlockKindIdentity;
  /** @state artifact */
  part2d?: Block5dPart2d;
  /** @state artifact */
  part3d?: Block5dPart3d;
  /** @state artifact */
  representations?: Block5dRepresentationsDelta;
  /** @state artifact */
  gripKinds?: Block5dGripKindsDelta;
  /** @state artifact */
  grips?: Block5dGripsDelta;
  /** @state artifact */
  compatibility?: Block5dCompatibilityDelta;
  /** @state artifact */
  attributes?: Block5dAttributesDelta;
  /** @state artifact */
  authors?: Block5dAuthorList;
  /** @state artifact */
  camera2d?: BlockCamera2d;
  /** @state artifact */
  camera3d?: BlockCamera3d;
  /** @state artifact */
  meta?: BlockMeta;
}

export interface BlockKindIdentity { [key: string]: unknown; }

export interface Block5dPart2d { [key: string]: unknown; }

export interface Block5dPart3d { [key: string]: unknown; }

export interface BlockRepresentation { [key: string]: unknown; }

export interface Block5dGripKind { [key: string]: unknown; }

export interface Block5dGripTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera2d { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface Block5dStringList {
  values: string[];
}

export interface Block5dAuthorList {
  values: BlockAuthor[];
}

export interface Block5dRepresentationsDelta {
  added: BlockRepresentation[];
  removed: string[];
  patched: Block5dRepresentationsPatchEntry[];
  reordered?: string[];
}

export interface Block5dRepresentationsPatchEntry {
  id: string;
  patch: Block5dRepresentationsPatch;
}

export interface Block5dRepresentationsPatch {
  replacement?: BlockRepresentation;
}

export interface Block5dGripKindsDelta {
  added: Block5dGripKind[];
  removed: string[];
  patched: Block5dGripKindsPatchEntry[];
  reordered?: string[];
}

export interface Block5dGripKindsPatchEntry {
  id: string;
  patch: Block5dGripKindsPatch;
}

export interface Block5dGripKindsPatch {
  replacement?: Block5dGripKind;
}

export interface Block5dGripsDelta {
  added: Block5dGripTemplate[];
  removed: string[];
  patched: Block5dGripsPatchEntry[];
  reordered?: string[];
}

export interface Block5dGripsPatchEntry {
  id: string;
  patch: Block5dGripsPatch;
}

export interface Block5dGripsPatch {
  replacement?: Block5dGripTemplate;
}

export interface Block5dCompatibilityDelta {
  added: BlockCompatibilityRule[];
  removed: string[];
  patched: Block5dCompatibilityPatchEntry[];
  reordered?: string[];
}

export interface Block5dCompatibilityPatchEntry {
  id: string;
  patch: Block5dCompatibilityPatch;
}

export interface Block5dCompatibilityPatch {
  replacement?: BlockCompatibilityRule;
}

export interface Block5dAttributesDelta {
  added: BlockAttribute[];
  removed: string[];
  patched: Block5dAttributesPatchEntry[];
  reordered?: string[];
}

export interface Block5dAttributesPatchEntry {
  id: string;
  patch: Block5dAttributesPatch;
}

export interface Block5dAttributesPatch {
  replacement?: BlockAttribute;
}

export interface Block5dArtifact { [key: string]: unknown; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dDiffGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dDiffGuardRefusal(at, why);
};

type blockBlock5dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dDiffGuardReject(at, "value is not an object");
export const blockBlock5dDiffGuardArray = (value: unknown, at: string, bounds: blockBlock5dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dDiffGuardString = (value: unknown, at: string, bounds: blockBlock5dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dDiffGuardReject(at, "value is not a boolean"));
export const blockBlock5dDiffGuardNumber = (value: unknown, at: string, bounds: blockBlock5dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dDiffGuardInteger = (value: unknown, at: string, bounds: blockBlock5dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dDiffGuardNumber(value, at, bounds) : blockBlock5dDiffGuardReject(at, "value is not an integer");
export const blockBlock5dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dStringList(value: unknown, at = "$"): Block5dStringList {
  const row = blockBlock5dDiffGuardObject(value, at);
  return {
    values: blockBlock5dDiffGuardArray(row["values"], `${at}.values`).map((item, index) => blockBlock5dDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseBlock5dRepresentationsPatchEntry(value: unknown, at = "$"): Block5dRepresentationsPatchEntry {
  const row = blockBlock5dDiffGuardObject(value, at);
  return {
    id: blockBlock5dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock5dRepresentationsPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock5dGripKindsPatchEntry(value: unknown, at = "$"): Block5dGripKindsPatchEntry {
  const row = blockBlock5dDiffGuardObject(value, at);
  return {
    id: blockBlock5dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock5dGripKindsPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock5dGripsPatchEntry(value: unknown, at = "$"): Block5dGripsPatchEntry {
  const row = blockBlock5dDiffGuardObject(value, at);
  return {
    id: blockBlock5dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock5dGripsPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock5dCompatibilityPatchEntry(value: unknown, at = "$"): Block5dCompatibilityPatchEntry {
  const row = blockBlock5dDiffGuardObject(value, at);
  return {
    id: blockBlock5dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock5dCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock5dAttributesPatchEntry(value: unknown, at = "$"): Block5dAttributesPatchEntry {
  const row = blockBlock5dDiffGuardObject(value, at);
  return {
    id: blockBlock5dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock5dAttributesPatch(row["patch"], `${at}.patch`),
  };
}
