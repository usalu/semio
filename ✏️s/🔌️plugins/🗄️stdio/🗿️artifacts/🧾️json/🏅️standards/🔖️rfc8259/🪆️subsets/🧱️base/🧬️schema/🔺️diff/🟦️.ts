/** 🔺️ JsonDiff — recursive diff mirroring JsonValue's shape. No full-replace slot: `value` is a
 *  sparse `JsonValueDiff | undefined`, `undefined` meaning no change. Mirrors
 *  `../📸️snapshot/🟦️.ts`'s `JsonValue`/`JsonMember` (restated here, not imported, to
 *  keep each facet leaf self-contained). */
export interface JsonMember {
  key: string;
  value: JsonValue;
}
export type JsonValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "number"; lexeme: string }
  | { kind: "string"; value: string }
  | { kind: "array"; items: JsonValue[] }
  | { kind: "object"; members: JsonMember[] };

export type JsonValueDiff =
  | { kind: "replace"; value: JsonValue }
  | { kind: "bool"; value: boolean }
  | { kind: "number"; lexeme: string }
  | { kind: "string"; value: string }
  | { kind: "array"; diff: JsonArrayDiff }
  | { kind: "object"; diff: JsonObjectDiff };

export interface JsonArrayModified {
  index: number;
  diff: JsonValueDiff;
}
export interface JsonArrayAdded {
  index: number;
  item: JsonValue;
}
export interface JsonArrayDiff {
  removed?: number[];
  modified?: JsonArrayModified[];
  added?: JsonArrayAdded[];
}

export interface JsonObjectModified {
  key: string;
  diff: JsonValueDiff;
}
export interface JsonObjectAdded {
  index: number;
  key: string;
  item: JsonValue;
}
export interface JsonObjectDiff {
  removed?: string[];
  modified?: JsonObjectModified[];
  added?: JsonObjectAdded[];
}

export interface JsonDiff {
  /** @state artifact */ value?: JsonValueDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJsonRfc8259BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJsonRfc8259BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioJsonRfc8259BaseDiffGuardRefusal(at, why);
};

type stdioJsonRfc8259BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJsonRfc8259BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJsonRfc8259BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJsonRfc8259BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJsonRfc8259BaseDiffGuardReject(at, "value is not an object");
export const stdioJsonRfc8259BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJsonRfc8259BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJsonRfc8259BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJsonRfc8259BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJsonRfc8259BaseDiffGuardString = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJsonRfc8259BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJsonRfc8259BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJsonRfc8259BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJsonRfc8259BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJsonRfc8259BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJsonRfc8259BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioJsonRfc8259BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJsonRfc8259BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJsonRfc8259BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJsonRfc8259BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJsonRfc8259BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJsonRfc8259BaseDiffGuardNumber(value, at, bounds) : stdioJsonRfc8259BaseDiffGuardReject(at, "value is not an integer");
export const stdioJsonRfc8259BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJsonRfc8259BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJsonRfc8259BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJsonRfc8259BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJsonDiff(value: unknown, at = "$"): JsonDiff {
  const row = stdioJsonRfc8259BaseDiffGuardObject(value, at);
  return {
    value: row["value"] === undefined ? undefined : parseJsonValueDiff(row["value"], `${at}.value`),
  };
}

export function parseJsonValue(value: unknown, at = "$"): JsonValue {
  return parseJsonValue(value, `${at}`);
}
