/** 📝️ Text representation for `stdio.epw` (diff): the hand-rolled `keyword=hex` token line. */
export type EpwDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioEpwEnergyplusAnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioEpwEnergyplusAnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioEpwEnergyplusAnyDiffTextGuardRefusal(at, why);
};

type stdioEpwEnergyplusAnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioEpwEnergyplusAnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioEpwEnergyplusAnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioEpwEnergyplusAnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioEpwEnergyplusAnyDiffTextGuardReject(at, "value is not an object");
export const stdioEpwEnergyplusAnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioEpwEnergyplusAnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioEpwEnergyplusAnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioEpwEnergyplusAnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioEpwEnergyplusAnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioEpwEnergyplusAnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioEpwEnergyplusAnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioEpwEnergyplusAnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioEpwEnergyplusAnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioEpwEnergyplusAnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioEpwEnergyplusAnyDiffTextGuardNumber(value, at, bounds) : stdioEpwEnergyplusAnyDiffTextGuardReject(at, "value is not an integer");
export const stdioEpwEnergyplusAnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioEpwEnergyplusAnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioEpwEnergyplusAnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioEpwEnergyplusAnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEpwDiffText(value: unknown, at = "$"): EpwDiffText {
  return stdioEpwEnergyplusAnyDiffTextGuardObject(value, `${at}`);
}
