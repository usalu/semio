/** 📝️ Text-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the DSL-text ENCODING of the same shape. */
export interface SemioObjectSnapshotDsl {
  schema: string;
  transformWire: string;
  brepWire: string;
  meshWire: string;
  propertiesWire: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1ObjectSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1ObjectSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectSnapshotDsl(value: unknown, at = "$"): SemioObjectSnapshotDsl {
  const row = stdioSemioV1ObjectSnapshotTextGuardObject(value, at);
  return {
    schema: stdioSemioV1ObjectSnapshotTextGuardString(row["schema"], `${at}.schema`),
    transformWire: stdioSemioV1ObjectSnapshotTextGuardString(row["transformWire"], `${at}.transformWire`),
    brepWire: stdioSemioV1ObjectSnapshotTextGuardString(row["brepWire"], `${at}.brepWire`),
    meshWire: stdioSemioV1ObjectSnapshotTextGuardString(row["meshWire"], `${at}.meshWire`),
    propertiesWire: stdioSemioV1ObjectSnapshotTextGuardString(row["propertiesWire"], `${at}.propertiesWire`),
  };
}
