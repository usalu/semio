/** 📝️ Text representation for `stdio.epw` (snapshot): the real EnergyPlus Weather File body. */
export type EpwSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioEpwEnergyplusAnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioEpwEnergyplusAnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioEpwEnergyplusAnySnapshotTextGuardRefusal(at, why);
};

type stdioEpwEnergyplusAnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioEpwEnergyplusAnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioEpwEnergyplusAnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioEpwEnergyplusAnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioEpwEnergyplusAnySnapshotTextGuardReject(at, "value is not an object");
export const stdioEpwEnergyplusAnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioEpwEnergyplusAnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioEpwEnergyplusAnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioEpwEnergyplusAnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioEpwEnergyplusAnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioEpwEnergyplusAnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioEpwEnergyplusAnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioEpwEnergyplusAnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioEpwEnergyplusAnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioEpwEnergyplusAnySnapshotTextGuardNumber(value, at, bounds) : stdioEpwEnergyplusAnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioEpwEnergyplusAnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioEpwEnergyplusAnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioEpwEnergyplusAnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEpwSnapshotText(value: unknown, at = "$"): EpwSnapshotText {
  return stdioEpwEnergyplusAnySnapshotTextGuardObject(value, `${at}`);
}
