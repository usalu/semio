/** 📝️ Text representation for `s.stdio.semio.flow.inference`. */
export type SemioFlowInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1FlowInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1FlowInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1FlowInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1FlowInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1FlowInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1FlowInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1FlowInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1FlowInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1FlowInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1FlowInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1FlowInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1FlowInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1FlowInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1FlowInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1FlowInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1FlowInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1FlowInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1FlowInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1FlowInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1FlowInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1FlowInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1FlowInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1FlowInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1FlowInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1FlowInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1FlowInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1FlowInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1FlowInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1FlowInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1FlowInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1FlowInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioFlowInferenceText(value: unknown, at = "$"): SemioFlowInferenceText {
  return stdioSemioV1FlowInferenceTextGuardObject(value, `${at}`);
}
