/** 📝️ Text representation for `block.block2d.diff`. */
export type Block2dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dDiffTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dDiffTextGuardRefusal(at, why);
};

type blockBlock2dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dDiffTextGuardReject(at, "value is not an object");
export const blockBlock2dDiffTextGuardArray = (value: unknown, at: string, bounds: blockBlock2dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dDiffTextGuardString = (value: unknown, at: string, bounds: blockBlock2dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dDiffTextGuardReject(at, "value is not a boolean"));
export const blockBlock2dDiffTextGuardNumber = (value: unknown, at: string, bounds: blockBlock2dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dDiffTextGuardInteger = (value: unknown, at: string, bounds: blockBlock2dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dDiffTextGuardNumber(value, at, bounds) : blockBlock2dDiffTextGuardReject(at, "value is not an integer");
export const blockBlock2dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dDiffText(value: unknown, at = "$"): Block2dDiffText {
  return blockBlock2dDiffTextGuardObject(value, `${at}`);
}
