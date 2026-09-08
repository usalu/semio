/** 📝️ Text representation for `block.block3d.inference`. */
export type Block3dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dInferenceTextGuardRefusal(at, why);
};

type blockBlock3dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dInferenceTextGuardReject(at, "value is not an object");
export const blockBlock3dInferenceTextGuardArray = (value: unknown, at: string, bounds: blockBlock3dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dInferenceTextGuardString = (value: unknown, at: string, bounds: blockBlock3dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dInferenceTextGuardReject(at, "value is not a boolean"));
export const blockBlock3dInferenceTextGuardNumber = (value: unknown, at: string, bounds: blockBlock3dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dInferenceTextGuardInteger = (value: unknown, at: string, bounds: blockBlock3dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dInferenceTextGuardNumber(value, at, bounds) : blockBlock3dInferenceTextGuardReject(at, "value is not an integer");
export const blockBlock3dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dInferenceText(value: unknown, at = "$"): Block3dInferenceText {
  return blockBlock3dInferenceTextGuardObject(value, `${at}`);
}
