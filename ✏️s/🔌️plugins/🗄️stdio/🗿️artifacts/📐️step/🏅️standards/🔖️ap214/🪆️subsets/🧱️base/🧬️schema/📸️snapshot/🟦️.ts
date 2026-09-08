/** 🧬️ StepSnapshot schema — typed ISO 10303-21 HEADER triple + id-keyed entity graph, mirroring
 * the Rust `StepSnapshot` shape 1:1. Matches serde's default externally-tagged representation for
 * `StepValue` (unit variants serialize as a bare camelCase string; tuple/struct variants as a
 * single-key object). */

/** 🔤️ One typed Part-21 argument value. */
export type StepValue =
  | 'unset'
  | 'derived'
  | { integer: number }
  | { real: number }
  | { string: string }
  | { enum: string }
  | { reference: number }
  | { aggregate: StepValue[] }
  | { typedValue: { typeName: string; value: StepValue } };

/** 📇️ `FILE_DESCRIPTION(description, implementation_level)`. */
export interface StepFileDescription {
  description: string[];
  implementationLevel: string;
}

/** 📇️ `FILE_NAME(name, timestamp, author, organization, preprocessor_version,
 * originating_system, authorization)`. */
export interface StepFileName {
  name: string;
  timestamp: string;
  author: string[];
  organization: string[];
  preprocessorVersion: string;
  originatingSystem: string;
  authorization: string;
}

/** 📇️ `FILE_SCHEMA(schemas)`. */
export interface StepFileSchema {
  schemas: string[];
}

/** 📇️ The full typed `HEADER;` section. */
export interface StepHeader {
  fileDescription: StepFileDescription;
  fileName: StepFileName;
  fileSchema: StepFileSchema;
}

/** 🧩️ An additional type record on a genuinely complex Part-21 instance
 * (`#N=(TYPE1(...)TYPE2(...))`) — rare, spec-legal, never dropped. */
export interface StepComplexType {
  name: string;
  args: StepValue[];
}

/** 🧩️ One `#N = TYPE(args...)` instance — id-keyed identity, positional argument list. */
export interface StepEntity {
  id: number;
  name: string;
  args: StepValue[];
  complex?: StepComplexType[];
}

/** 📸️ Persisted `stdio.step` snapshot. */
export interface StepSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ header: StepHeader;
  /** @state artifact */ entities: StepEntity[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStepAp214BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStepAp214BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioStepAp214BaseSnapshotGuardRefusal(at, why);
};

type stdioStepAp214BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStepAp214BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStepAp214BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStepAp214BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStepAp214BaseSnapshotGuardReject(at, "value is not an object");
export const stdioStepAp214BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStepAp214BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStepAp214BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStepAp214BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStepAp214BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStepAp214BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStepAp214BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStepAp214BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStepAp214BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStepAp214BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStepAp214BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioStepAp214BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStepAp214BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStepAp214BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStepAp214BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStepAp214BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioStepAp214BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStepAp214BaseSnapshotGuardNumber(value, at, bounds) : stdioStepAp214BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioStepAp214BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStepAp214BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStepAp214BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStepAp214BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStepSnapshot(value: unknown, at = "$"): StepSnapshot {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioStepAp214BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    header: row["header"] === undefined ? undefined : parseStepHeader(row["header"], `${at}.header`),
    entities: row["entities"] === undefined ? undefined : stdioStepAp214BaseSnapshotGuardArray(row["entities"], `${at}.entities`).map((item, index) => parseStepEntity(item, `${at}.entities[${index}]`)),
  };
}

export function parseStepFileDescription(value: unknown, at = "$"): StepFileDescription {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    description: stdioStepAp214BaseSnapshotGuardArray(row["description"], `${at}.description`).map((item, index) => stdioStepAp214BaseSnapshotGuardString(item, `${at}.description[${index}]`)),
    implementationLevel: stdioStepAp214BaseSnapshotGuardString(row["implementationLevel"], `${at}.implementationLevel`),
  };
}

export function parseStepFileName(value: unknown, at = "$"): StepFileName {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    name: stdioStepAp214BaseSnapshotGuardString(row["name"], `${at}.name`),
    timestamp: stdioStepAp214BaseSnapshotGuardString(row["timestamp"], `${at}.timestamp`),
    author: stdioStepAp214BaseSnapshotGuardArray(row["author"], `${at}.author`).map((item, index) => stdioStepAp214BaseSnapshotGuardString(item, `${at}.author[${index}]`)),
    organization: stdioStepAp214BaseSnapshotGuardArray(row["organization"], `${at}.organization`).map((item, index) => stdioStepAp214BaseSnapshotGuardString(item, `${at}.organization[${index}]`)),
    preprocessorVersion: stdioStepAp214BaseSnapshotGuardString(row["preprocessorVersion"], `${at}.preprocessorVersion`),
    originatingSystem: stdioStepAp214BaseSnapshotGuardString(row["originatingSystem"], `${at}.originatingSystem`),
    authorization: stdioStepAp214BaseSnapshotGuardString(row["authorization"], `${at}.authorization`),
  };
}

export function parseStepFileSchema(value: unknown, at = "$"): StepFileSchema {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    schemas: stdioStepAp214BaseSnapshotGuardArray(row["schemas"], `${at}.schemas`).map((item, index) => stdioStepAp214BaseSnapshotGuardString(item, `${at}.schemas[${index}]`)),
  };
}

export function parseStepHeader(value: unknown, at = "$"): StepHeader {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    fileDescription: parseStepFileDescription(row["fileDescription"], `${at}.fileDescription`),
    fileName: parseStepFileName(row["fileName"], `${at}.fileName`),
    fileSchema: parseStepFileSchema(row["fileSchema"], `${at}.fileSchema`),
  };
}

export function parseStepComplexType(value: unknown, at = "$"): StepComplexType {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    name: stdioStepAp214BaseSnapshotGuardString(row["name"], `${at}.name`),
    args: stdioStepAp214BaseSnapshotGuardArray(row["args"], `${at}.args`).map((item, index) => parseStepValue(item, `${at}.args[${index}]`)),
  };
}

export function parseStepEntity(value: unknown, at = "$"): StepEntity {
  const row = stdioStepAp214BaseSnapshotGuardObject(value, at);
  return {
    id: stdioStepAp214BaseSnapshotGuardInteger(row["id"], `${at}.id`, {"minimum": 0}),
    name: stdioStepAp214BaseSnapshotGuardString(row["name"], `${at}.name`),
    args: stdioStepAp214BaseSnapshotGuardArray(row["args"], `${at}.args`).map((item, index) => parseStepValue(item, `${at}.args[${index}]`)),
    complex: row["complex"] === undefined ? undefined : stdioStepAp214BaseSnapshotGuardArray(row["complex"], `${at}.complex`).map((item, index) => parseStepComplexType(item, `${at}.complex[${index}]`)),
  };
}
