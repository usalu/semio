/** 💡️ Semio envelope inference schema — the wrapped subset's dispatch tag/ordinal. */

export interface SemioKind {
  tag: string;
  ordinal: number;
}

export interface SemioInference {
  /** @derived */
  kind: SemioKind;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1BaseInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1BaseInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1BaseInferenceGuardRefusal(at, why);
};

type stdioSemioV1BaseInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1BaseInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1BaseInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1BaseInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1BaseInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1BaseInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1BaseInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1BaseInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1BaseInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1BaseInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1BaseInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1BaseInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1BaseInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1BaseInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1BaseInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1BaseInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1BaseInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1BaseInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1BaseInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1BaseInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1BaseInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1BaseInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1BaseInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1BaseInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1BaseInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1BaseInferenceGuardNumber(value, at, bounds) : stdioSemioV1BaseInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1BaseInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1BaseInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1BaseInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1BaseInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioInference(value: unknown, at = "$"): SemioInference {
  const row = stdioSemioV1BaseInferenceGuardObject(value, at);
  return {
    kind: parseSemioKind(row["kind"], `${at}.kind`),
  };
}

export function parseSemioKind(value: unknown, at = "$"): SemioKind {
  const row = stdioSemioV1BaseInferenceGuardObject(value, at);
  return {
    tag: stdioSemioV1BaseInferenceGuardString(row["tag"], `${at}.tag`),
    ordinal: stdioSemioV1BaseInferenceGuardInteger(row["ordinal"], `${at}.ordinal`, {"minimum": 0}),
  };
}
