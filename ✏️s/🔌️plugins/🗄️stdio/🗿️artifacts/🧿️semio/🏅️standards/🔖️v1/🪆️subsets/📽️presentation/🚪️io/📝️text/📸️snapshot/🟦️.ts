/** Text-envelope shape descriptor. */
export interface SnapshotTextEnvelope { header: "schema stdio.semio.presentation.snapshot"; hexPayload: string; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1PresentationSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1PresentationSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1PresentationSnapshotTextGuardRefusal(at, why);
};

type stdioSemioV1PresentationSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1PresentationSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1PresentationSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1PresentationSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1PresentationSnapshotTextGuardReject(at, "value is not an object");
export const stdioSemioV1PresentationSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1PresentationSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1PresentationSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1PresentationSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1PresentationSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1PresentationSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1PresentationSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1PresentationSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1PresentationSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1PresentationSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1PresentationSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1PresentationSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1PresentationSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1PresentationSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1PresentationSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1PresentationSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1PresentationSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1PresentationSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1PresentationSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1PresentationSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1PresentationSnapshotTextGuardNumber(value, at, bounds) : stdioSemioV1PresentationSnapshotTextGuardReject(at, "value is not an integer");
export const stdioSemioV1PresentationSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1PresentationSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1PresentationSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1PresentationSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioPresentationSnapshotTextEnvelope {
  readonly header: "schema stdio.semio.presentation.snapshot";
  readonly hexPayload: string;
}

export function parseSemioPresentationSnapshotTextEnvelope(value: unknown, at = "$"): SemioPresentationSnapshotTextEnvelope {
  const row = stdioSemioV1PresentationSnapshotTextGuardObject(value, at);
  return {
    header: stdioSemioV1PresentationSnapshotTextGuardConstant(row["header"], `${at}.header`, "schema stdio.semio.presentation.snapshot"),
    hexPayload: stdioSemioV1PresentationSnapshotTextGuardString(row["hexPayload"], `${at}.hexPayload`, {"pattern": "^[0-9a-f]*$"}),
  };
}
