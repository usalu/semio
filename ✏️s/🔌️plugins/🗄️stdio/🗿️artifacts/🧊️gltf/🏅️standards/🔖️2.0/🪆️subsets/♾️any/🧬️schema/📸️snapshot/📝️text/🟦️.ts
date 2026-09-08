/** 📝️ Text representation for `stdio.gltf` (snapshot). */
export type GltfSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGltf20AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGltf20AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioGltf20AnySnapshotTextGuardRefusal(at, why);
};

type stdioGltf20AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGltf20AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGltf20AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGltf20AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGltf20AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioGltf20AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioGltf20AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGltf20AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGltf20AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGltf20AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGltf20AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioGltf20AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGltf20AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGltf20AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGltf20AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGltf20AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGltf20AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGltf20AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioGltf20AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioGltf20AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGltf20AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGltf20AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGltf20AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGltf20AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioGltf20AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGltf20AnySnapshotTextGuardNumber(value, at, bounds) : stdioGltf20AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioGltf20AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGltf20AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGltf20AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGltf20AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGltfSnapshotText(value: unknown, at = "$"): GltfSnapshotText {
  return stdioGltf20AnySnapshotTextGuardObject(value, `${at}`);
}
