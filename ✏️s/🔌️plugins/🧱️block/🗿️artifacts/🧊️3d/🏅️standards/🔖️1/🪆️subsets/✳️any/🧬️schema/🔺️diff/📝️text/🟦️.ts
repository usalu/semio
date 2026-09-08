/** 📝️ Text representation for `block.block3d.diff`. */
export type Block3dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dDiffTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dDiffTextGuardRefusal(at, why);
};

type blockBlock3dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dDiffTextGuardReject(at, "value is not an object");
export const blockBlock3dDiffTextGuardArray = (value: unknown, at: string, bounds: blockBlock3dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dDiffTextGuardString = (value: unknown, at: string, bounds: blockBlock3dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dDiffTextGuardReject(at, "value is not a boolean"));
export const blockBlock3dDiffTextGuardNumber = (value: unknown, at: string, bounds: blockBlock3dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dDiffTextGuardInteger = (value: unknown, at: string, bounds: blockBlock3dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dDiffTextGuardNumber(value, at, bounds) : blockBlock3dDiffTextGuardReject(at, "value is not an integer");
export const blockBlock3dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dDiffText(value: unknown, at = "$"): Block3dDiffText {
  return blockBlock3dDiffTextGuardObject(value, `${at}`);
}
