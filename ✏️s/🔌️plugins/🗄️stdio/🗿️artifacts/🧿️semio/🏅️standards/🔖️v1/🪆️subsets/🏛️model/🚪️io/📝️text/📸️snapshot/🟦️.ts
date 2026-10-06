/** 📝️ Text representation for `stdio.semio.model` (snapshot): the `semio_format` preamble line
 * plus the hex-encoded compact-JSON snapshot body. */
export type SemioModelSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ModelSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ModelSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ModelSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1ModelSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ModelSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ModelSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ModelSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ModelSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1ModelSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ModelSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ModelSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ModelSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ModelSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ModelSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ModelSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ModelSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ModelSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ModelSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ModelSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ModelSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ModelSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ModelSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ModelSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ModelSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ModelSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ModelSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ModelSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ModelSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ModelSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1ModelSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ModelSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ModelSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ModelSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ModelSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioModelSnapshotText(value: unknown, at = "$"): SemioModelSnapshotText {
  const row = stdioSemioV1ModelSnapshotTextGuardObject(value, at);
  return {
    preamble: row["preamble"] === undefined ? undefined : stdioSemioV1ModelSnapshotTextGuardString(row["preamble"], `${at}.preamble`),
    body: row["body"] === undefined ? undefined : stdioSemioV1ModelSnapshotTextGuardString(row["body"], `${at}.body`, {"pattern": "^([0-9a-f]{2})*$"}),
  };
}
