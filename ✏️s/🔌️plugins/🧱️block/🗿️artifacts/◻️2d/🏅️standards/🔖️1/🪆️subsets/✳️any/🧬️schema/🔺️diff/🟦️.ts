/** 🧬️ Block2d diff schema — sparse field delta. */

export interface Block2dDiff {
  /** @state artifact */
  artifact?: Block2dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  nodeKind?: BlockKindIdentity;
  /** @state artifact */
  presentation?: Block2dPresentation;
  /** @state artifact */
  handleKinds?: Block2dHandleKindsDelta;
  /** @state artifact */
  handles?: Block2dHandlesDelta;
  /** @state artifact */
  compatibility?: Block2dCompatibilityDelta;
  /** @state artifact */
  attributes?: Block2dAttributesDelta;
  /** @state artifact */
  authors?: Block2dAuthorList;
  /** @state artifact */
  camera2d?: BlockCamera2d;
  /** @state artifact */
  meta?: BlockMeta;
}

export interface BlockKindIdentity { [key: string]: unknown; }

export interface Block2dPresentation { [key: string]: unknown; }

export interface Block2dHandleKind { [key: string]: unknown; }

export interface Block2dHandleTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera2d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface Block2dStringList {
  values: string[];
}

export interface Block2dAuthorList {
  values: BlockAuthor[];
}

export interface Block2dHandleKindsDelta {
  added: Block2dHandleKind[];
  removed: string[];
  patched: Block2dHandleKindsPatchEntry[];
  reordered?: string[];
}

export interface Block2dHandleKindsPatchEntry {
  id: string;
  patch: Block2dHandleKindsPatch;
}

export interface Block2dHandleKindsPatch {
  replacement?: Block2dHandleKind;
}

export interface Block2dHandlesDelta {
  added: Block2dHandleTemplate[];
  removed: string[];
  patched: Block2dHandlesPatchEntry[];
  reordered?: string[];
}

export interface Block2dHandlesPatchEntry {
  id: string;
  patch: Block2dHandlesPatch;
}

export interface Block2dHandlesPatch {
  replacement?: Block2dHandleTemplate;
}

export interface Block2dCompatibilityDelta {
  added: BlockCompatibilityRule[];
  removed: string[];
  patched: Block2dCompatibilityPatchEntry[];
  reordered?: string[];
}

export interface Block2dCompatibilityPatchEntry {
  id: string;
  patch: Block2dCompatibilityPatch;
}

export interface Block2dCompatibilityPatch {
  replacement?: BlockCompatibilityRule;
}

export interface Block2dAttributesDelta {
  added: BlockAttribute[];
  removed: string[];
  patched: Block2dAttributesPatchEntry[];
  reordered?: string[];
}

export interface Block2dAttributesPatchEntry {
  id: string;
  patch: Block2dAttributesPatch;
}

export interface Block2dAttributesPatch {
  replacement?: BlockAttribute;
}

export interface Block2dArtifact { [key: string]: unknown; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dDiffGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dDiffGuardRefusal(at, why);
};

type blockBlock2dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dDiffGuardReject(at, "value is not an object");
export const blockBlock2dDiffGuardArray = (value: unknown, at: string, bounds: blockBlock2dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dDiffGuardString = (value: unknown, at: string, bounds: blockBlock2dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dDiffGuardReject(at, "value is not a boolean"));
export const blockBlock2dDiffGuardNumber = (value: unknown, at: string, bounds: blockBlock2dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dDiffGuardInteger = (value: unknown, at: string, bounds: blockBlock2dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dDiffGuardNumber(value, at, bounds) : blockBlock2dDiffGuardReject(at, "value is not an integer");
export const blockBlock2dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dStringList(value: unknown, at = "$"): Block2dStringList {
  const row = blockBlock2dDiffGuardObject(value, at);
  return {
    values: blockBlock2dDiffGuardArray(row["values"], `${at}.values`).map((item, index) => blockBlock2dDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseBlock2dHandleKindsPatchEntry(value: unknown, at = "$"): Block2dHandleKindsPatchEntry {
  const row = blockBlock2dDiffGuardObject(value, at);
  return {
    id: blockBlock2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock2dHandleKindsPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock2dHandlesPatchEntry(value: unknown, at = "$"): Block2dHandlesPatchEntry {
  const row = blockBlock2dDiffGuardObject(value, at);
  return {
    id: blockBlock2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock2dHandlesPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock2dCompatibilityPatchEntry(value: unknown, at = "$"): Block2dCompatibilityPatchEntry {
  const row = blockBlock2dDiffGuardObject(value, at);
  return {
    id: blockBlock2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock2dCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock2dAttributesPatchEntry(value: unknown, at = "$"): Block2dAttributesPatchEntry {
  const row = blockBlock2dDiffGuardObject(value, at);
  return {
    id: blockBlock2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock2dAttributesPatch(row["patch"], `${at}.patch`),
  };
}
