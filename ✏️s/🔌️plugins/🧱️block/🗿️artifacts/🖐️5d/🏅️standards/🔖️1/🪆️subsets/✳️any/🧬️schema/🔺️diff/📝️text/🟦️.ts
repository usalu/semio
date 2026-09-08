/** 📝️ Text representation for `block.block5d.diff`. */
export type Block5dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dDiffTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dDiffTextGuardRefusal(at, why);
};

type blockBlock5dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dDiffTextGuardReject(at, "value is not an object");
export const blockBlock5dDiffTextGuardArray = (value: unknown, at: string, bounds: blockBlock5dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dDiffTextGuardString = (value: unknown, at: string, bounds: blockBlock5dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dDiffTextGuardReject(at, "value is not a boolean"));
export const blockBlock5dDiffTextGuardNumber = (value: unknown, at: string, bounds: blockBlock5dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dDiffTextGuardInteger = (value: unknown, at: string, bounds: blockBlock5dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dDiffTextGuardNumber(value, at, bounds) : blockBlock5dDiffTextGuardReject(at, "value is not an integer");
export const blockBlock5dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dDiffText(value: unknown, at = "$"): Block5dDiffText {
  return blockBlock5dDiffTextGuardObject(value, `${at}`);
}
