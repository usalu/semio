/** 📝️ Text-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the DIFF-TEXT ENCODING of the same shape. */
export interface SemioGraphDiffDsl {
  nodesWire?: string;
  edgesWire?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphDiffTextGuardRefusal(at, why);
};

type stdioSemioV1GraphDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphDiffTextGuardReject(at, "value is not an object");
export const stdioSemioV1GraphDiffTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphDiffTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphDiffTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphDiffTextGuardNumber(value, at, bounds) : stdioSemioV1GraphDiffTextGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioGraphDiffDsl(value: unknown, at = "$"): SemioGraphDiffDsl {
  const row = stdioSemioV1GraphDiffTextGuardObject(value, at);
  return {
    nodesWire: row["nodesWire"] === undefined ? undefined : stdioSemioV1GraphDiffTextGuardString(row["nodesWire"], `${at}.nodesWire`),
    edgesWire: row["edgesWire"] === undefined ? undefined : stdioSemioV1GraphDiffTextGuardString(row["edgesWire"], `${at}.edgesWire`),
  };
}
