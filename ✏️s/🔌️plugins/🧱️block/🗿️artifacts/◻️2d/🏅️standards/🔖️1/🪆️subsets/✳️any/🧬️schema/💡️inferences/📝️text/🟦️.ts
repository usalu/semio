/** 📝️ Text representation for `block.block2d.inference`. */
export type Block2dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dInferenceTextGuardRefusal(at, why);
};

type blockBlock2dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dInferenceTextGuardReject(at, "value is not an object");
export const blockBlock2dInferenceTextGuardArray = (value: unknown, at: string, bounds: blockBlock2dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dInferenceTextGuardString = (value: unknown, at: string, bounds: blockBlock2dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dInferenceTextGuardReject(at, "value is not a boolean"));
export const blockBlock2dInferenceTextGuardNumber = (value: unknown, at: string, bounds: blockBlock2dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dInferenceTextGuardInteger = (value: unknown, at: string, bounds: blockBlock2dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dInferenceTextGuardNumber(value, at, bounds) : blockBlock2dInferenceTextGuardReject(at, "value is not an integer");
export const blockBlock2dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dInferenceText(value: unknown, at = "$"): Block2dInferenceText {
  return blockBlock2dInferenceTextGuardObject(value, `${at}`);
}
