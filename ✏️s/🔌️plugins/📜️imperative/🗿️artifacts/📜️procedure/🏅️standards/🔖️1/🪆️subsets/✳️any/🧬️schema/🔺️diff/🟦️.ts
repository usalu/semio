/** 🧬️ Imperative diff schema — sparse field delta over the artifact. */

export interface ProcedureDiff {
  /** @state artifact */ artifact?: ProcedureArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact */ path?: ProcedurePathDelta;
  /** @state artifact */ seed?: Record<string, unknown>;
}

export interface ProcedureStringList {
  values: string[];
}

export interface ProcedurePathRef {
  owner?: string;
  slot?: string;
}

export interface ProcedurePathDelta {
  pathRef: ProcedurePathRef;
  steps: ProcedureStepsDelta;
}

export interface ProcedureStepsDelta {
  added: ProcedureStep[];
  removed: string[];
  patched: ProcedureStepPatchEntry[];
  reordered?: string[];
}

export interface ProcedureStepPatchEntry {
  id: string;
  patch: Record<string, unknown>;
}

export interface ProcedurePath {
  steps: ProcedureStep[];
}

export interface ProcedureStep {
  id: string;
  kind: string;
  params?: Record<string, unknown>;
  bodies?: Record<string, ProcedurePath>;
}

export interface ProcedureArtifact {
  schema: string;
  path: ProcedurePath;
  seed: Record<string, unknown>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class imperativeImperativeDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const imperativeImperativeDiffGuardReject = (at: string, why: string): never => {
  throw new imperativeImperativeDiffGuardRefusal(at, why);
};

type imperativeImperativeDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type imperativeImperativeDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type imperativeImperativeDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const imperativeImperativeDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : imperativeImperativeDiffGuardReject(at, "value is not an object");
export const imperativeImperativeDiffGuardArray = (value: unknown, at: string, bounds: imperativeImperativeDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return imperativeImperativeDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) imperativeImperativeDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) imperativeImperativeDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const imperativeImperativeDiffGuardString = (value: unknown, at: string, bounds: imperativeImperativeDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return imperativeImperativeDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) imperativeImperativeDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) imperativeImperativeDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) imperativeImperativeDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const imperativeImperativeDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : imperativeImperativeDiffGuardReject(at, "value is not a boolean"));
export const imperativeImperativeDiffGuardNumber = (value: unknown, at: string, bounds: imperativeImperativeDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return imperativeImperativeDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) imperativeImperativeDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) imperativeImperativeDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const imperativeImperativeDiffGuardInteger = (value: unknown, at: string, bounds: imperativeImperativeDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? imperativeImperativeDiffGuardNumber(value, at, bounds) : imperativeImperativeDiffGuardReject(at, "value is not an integer");
export const imperativeImperativeDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : imperativeImperativeDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const imperativeImperativeDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : imperativeImperativeDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcedureDiff(value: unknown, at = "$"): ProcedureDiff {
  const row = imperativeImperativeDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === undefined ? undefined : imperativeImperativeDiffGuardObject(row["artifact"], `${at}.artifact`),
    schema: row["schema"] === undefined ? undefined : imperativeImperativeDiffGuardString(row["schema"], `${at}.schema`),
    path: row["path"] === undefined ? undefined : parseProcedurePathDelta(row["path"], `${at}.path`),
    seed: row["seed"] === undefined ? undefined : imperativeImperativeDiffGuardObject(row["seed"], `${at}.seed`),
  };
}

export function parseProcedureStringList(value: unknown, at = "$"): ProcedureStringList {
  const row = imperativeImperativeDiffGuardObject(value, at);
  return {
    values: imperativeImperativeDiffGuardArray(row["values"], `${at}.values`).map((item, index) => imperativeImperativeDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseProcedurePathRef(value: unknown, at = "$"): ProcedurePathRef {
  const row = imperativeImperativeDiffGuardObject(value, at);
  return {
    owner: row["owner"] === undefined ? undefined : imperativeImperativeDiffGuardString(row["owner"], `${at}.owner`),
    slot: row["slot"] === undefined ? undefined : imperativeImperativeDiffGuardString(row["slot"], `${at}.slot`),
  };
}

export function parseProcedurePathDelta(value: unknown, at = "$"): ProcedurePathDelta {
  const row = imperativeImperativeDiffGuardObject(value, at);
  return {
    pathRef: parseProcedurePathRef(row["pathRef"], `${at}.pathRef`),
    steps: parseProcedureStepsDelta(row["steps"], `${at}.steps`),
  };
}

export function parseProcedureStepPatchEntry(value: unknown, at = "$"): ProcedureStepPatchEntry {
  const row = imperativeImperativeDiffGuardObject(value, at);
  return {
    id: imperativeImperativeDiffGuardString(row["id"], `${at}.id`),
    patch: imperativeImperativeDiffGuardObject(row["patch"], `${at}.patch`),
  };
}
