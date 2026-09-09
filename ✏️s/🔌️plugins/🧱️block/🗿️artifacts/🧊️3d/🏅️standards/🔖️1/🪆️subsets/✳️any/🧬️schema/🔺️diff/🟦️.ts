/** 🧬️ Block3d diff schema — sparse field delta. */

export interface Block3dDiff {
  /** @state artifact */
  artifact?: Block3dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  objectKind?: BlockKindIdentity;
  /** @state artifact */
  representations?: Block3dRepresentationsDelta;
  /** @state artifact */
  vortexKinds?: Block3dVortexKindsDelta;
  /** @state artifact */
  vortices?: Block3dVorticesDelta;
  /** @state artifact */
  compatibility?: Block3dCompatibilityDelta;
  /** @state artifact */
  attributes?: Block3dAttributesDelta;
  /** @state artifact */
  authors?: Block3dAuthorList;
  /** @state artifact */
  camera3d?: BlockCamera3d;
  /** @state artifact */
  meta?: BlockMeta;
}

export interface BlockKindIdentity { [key: string]: unknown; }

export interface BlockRepresentation { [key: string]: unknown; }

export interface Block3dVortexKind { [key: string]: unknown; }

export interface Block3dVortexTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface Block3dWindowView { [key: string]: unknown; }


export interface Block3dStringList {
  values: string[];
}

export interface Block3dAuthorList {
  values: BlockAuthor[];
}

export interface Block3dRepresentationsDelta {
  added: BlockRepresentation[];
  removed: string[];
  patched: Block3dRepresentationsPatchEntry[];
  reordered?: string[];
}

export interface Block3dRepresentationsPatchEntry {
  id: string;
  patch: Block3dRepresentationsPatch;
}

export interface Block3dRepresentationsPatch {
  replacement?: BlockRepresentation;
}

export interface Block3dVortexKindsDelta {
  added: Block3dVortexKind[];
  removed: string[];
  patched: Block3dVortexKindsPatchEntry[];
  reordered?: string[];
}

export interface Block3dVortexKindsPatchEntry {
  id: string;
  patch: Block3dVortexKindsPatch;
}

export interface Block3dVortexKindsPatch {
  replacement?: Block3dVortexKind;
}

export interface Block3dVorticesDelta {
  added: Block3dVortexTemplate[];
  removed: string[];
  patched: Block3dVorticesPatchEntry[];
  reordered?: string[];
}

export interface Block3dVorticesPatchEntry {
  id: string;
  patch: Block3dVorticesPatch;
}

export interface Block3dVorticesPatch {
  replacement?: Block3dVortexTemplate;
}

export interface Block3dCompatibilityDelta {
  added: BlockCompatibilityRule[];
  removed: string[];
  patched: Block3dCompatibilityPatchEntry[];
  reordered?: string[];
}

export interface Block3dCompatibilityPatchEntry {
  id: string;
  patch: Block3dCompatibilityPatch;
}

export interface Block3dCompatibilityPatch {
  replacement?: BlockCompatibilityRule;
}

export interface Block3dAttributesDelta {
  added: BlockAttribute[];
  removed: string[];
  patched: Block3dAttributesPatchEntry[];
  reordered?: string[];
}

export interface Block3dAttributesPatchEntry {
  id: string;
  patch: Block3dAttributesPatch;
}

export interface Block3dAttributesPatch {
  replacement?: BlockAttribute;
}

export interface Block3dWindowsList {
  values: Block3dWindowView[];
}

export interface Block3dArtifact { [key: string]: unknown; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dDiffGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dDiffGuardRefusal(at, why);
};

type blockBlock3dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dDiffGuardReject(at, "value is not an object");
export const blockBlock3dDiffGuardArray = (value: unknown, at: string, bounds: blockBlock3dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dDiffGuardString = (value: unknown, at: string, bounds: blockBlock3dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dDiffGuardReject(at, "value is not a boolean"));
export const blockBlock3dDiffGuardNumber = (value: unknown, at: string, bounds: blockBlock3dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dDiffGuardInteger = (value: unknown, at: string, bounds: blockBlock3dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dDiffGuardNumber(value, at, bounds) : blockBlock3dDiffGuardReject(at, "value is not an integer");
export const blockBlock3dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dStringList(value: unknown, at = "$"): Block3dStringList {
  const row = blockBlock3dDiffGuardObject(value, at);
  return {
    values: blockBlock3dDiffGuardArray(row["values"], `${at}.values`).map((item, index) => blockBlock3dDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseBlock3dRepresentationsPatchEntry(value: unknown, at = "$"): Block3dRepresentationsPatchEntry {
  const row = blockBlock3dDiffGuardObject(value, at);
  return {
    id: blockBlock3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock3dRepresentationsPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock3dVortexKindsPatchEntry(value: unknown, at = "$"): Block3dVortexKindsPatchEntry {
  const row = blockBlock3dDiffGuardObject(value, at);
  return {
    id: blockBlock3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock3dVortexKindsPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock3dVorticesPatchEntry(value: unknown, at = "$"): Block3dVorticesPatchEntry {
  const row = blockBlock3dDiffGuardObject(value, at);
  return {
    id: blockBlock3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock3dVorticesPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock3dCompatibilityPatchEntry(value: unknown, at = "$"): Block3dCompatibilityPatchEntry {
  const row = blockBlock3dDiffGuardObject(value, at);
  return {
    id: blockBlock3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock3dCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBlock3dAttributesPatchEntry(value: unknown, at = "$"): Block3dAttributesPatchEntry {
  const row = blockBlock3dDiffGuardObject(value, at);
  return {
    id: blockBlock3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseBlock3dAttributesPatch(row["patch"], `${at}.patch`),
  };
}
