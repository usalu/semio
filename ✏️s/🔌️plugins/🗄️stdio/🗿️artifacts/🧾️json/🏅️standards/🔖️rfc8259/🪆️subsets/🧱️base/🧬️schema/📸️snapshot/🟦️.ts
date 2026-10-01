/** 🧬️ JsonSnapshot schema — own JsonValue model, insertion-order-preserving, lexeme-preserving. */
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

export interface JsonSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ value: JsonValue;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJsonRfc8259BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJsonRfc8259BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioJsonRfc8259BaseSnapshotGuardRefusal(at, why);
};

type stdioJsonRfc8259BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJsonRfc8259BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJsonRfc8259BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJsonRfc8259BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJsonRfc8259BaseSnapshotGuardReject(at, "value is not an object");
export const stdioJsonRfc8259BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJsonRfc8259BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJsonRfc8259BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJsonRfc8259BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJsonRfc8259BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJsonRfc8259BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJsonRfc8259BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJsonRfc8259BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJsonRfc8259BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJsonRfc8259BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJsonRfc8259BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioJsonRfc8259BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJsonRfc8259BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJsonRfc8259BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJsonRfc8259BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJsonRfc8259BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJsonRfc8259BaseSnapshotGuardNumber(value, at, bounds) : stdioJsonRfc8259BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioJsonRfc8259BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJsonRfc8259BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJsonRfc8259BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJsonRfc8259BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJsonSnapshot(value: unknown, at = "$"): JsonSnapshot {
  const row = stdioJsonRfc8259BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioJsonRfc8259BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    value: parseJsonValue(row["value"], `${at}.value`),
  };
}

export function parseJsonMember(value: unknown, at = "$"): JsonMember {
  const row = stdioJsonRfc8259BaseSnapshotGuardObject(value, at);
  return {
    key: stdioJsonRfc8259BaseSnapshotGuardString(row["key"], `${at}.key`),
    value: parseJsonValue(row["value"], `${at}.value`),
  };
}

/** 🧾️ Validate the owned JSON value union while retaining exact number lexemes and member order. */
export function parseJsonValue(value: unknown, at = "$"): JsonValue {
  const row = stdioJsonRfc8259BaseSnapshotGuardObject(value, at);
  const kind = stdioJsonRfc8259BaseSnapshotGuardMember(row["kind"], `${at}.kind`, ["null", "bool", "number", "string", "array", "object"] as const);
  switch (kind) {
    case "null": return { kind };
    case "bool": return { kind, value: stdioJsonRfc8259BaseSnapshotGuardBoolean(row["value"], `${at}.value`) };
    case "number": return { kind, lexeme: stdioJsonRfc8259BaseSnapshotGuardString(row["lexeme"], `${at}.lexeme`) };
    case "string": return { kind, value: stdioJsonRfc8259BaseSnapshotGuardString(row["value"], `${at}.value`) };
    case "array": return { kind, items: stdioJsonRfc8259BaseSnapshotGuardArray(row["items"], `${at}.items`).map((item, index) => parseJsonValue(item, `${at}.items[${index}]`)) };
    case "object": return { kind, members: stdioJsonRfc8259BaseSnapshotGuardArray(row["members"], `${at}.members`).map((item, index) => parseJsonMember(item, `${at}.members[${index}]`)) };
  }
}
