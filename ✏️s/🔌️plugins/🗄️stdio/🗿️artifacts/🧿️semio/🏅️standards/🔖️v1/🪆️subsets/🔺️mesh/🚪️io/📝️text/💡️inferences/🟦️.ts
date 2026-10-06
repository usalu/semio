/** 📝️ Text representation for `s.stdio.semio.mesh.inference`. */
export type SemioMeshInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1MeshInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1MeshInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1MeshInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1MeshInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1MeshInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1MeshInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1MeshInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1MeshInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1MeshInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1MeshInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1MeshInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1MeshInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1MeshInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1MeshInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1MeshInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1MeshInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1MeshInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1MeshInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1MeshInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1MeshInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1MeshInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1MeshInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1MeshInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1MeshInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1MeshInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1MeshInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1MeshInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1MeshInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1MeshInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1MeshInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioMeshInferenceText(value: unknown, at = "$"): SemioMeshInferenceText {
  return stdioSemioV1MeshInferenceTextGuardObject(value, `${at}`);
}
