/** 💡️ SemioMeshInference facet mirror — real facet mirror of the Rust `🦀️.rs` sibling.
 * `computedNormals`/`tessellationPreview` are deliberately absent — see the Rust sibling's module
 * doc comment for why (honest omission, not an oversight). Keyed per `"{meshId}:{primitiveId}"`. */
export interface SemioMeshInference {
  aabb: Record<string, import("./📦aabb/🟦️.ts").SemioAabb>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1MeshInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1MeshInferenceGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1MeshInferenceGuardRefusal(at, why);
};

type stdioSemioV1MeshInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1MeshInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1MeshInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1MeshInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1MeshInferenceGuardReject(at, "value is not an object");
export const stdioSemioV1MeshInferenceGuardArray = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1MeshInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1MeshInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1MeshInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1MeshInferenceGuardString = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1MeshInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1MeshInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1MeshInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1MeshInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1MeshInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1MeshInferenceGuardReject(at, "value is not a boolean"));
export const stdioSemioV1MeshInferenceGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1MeshInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1MeshInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1MeshInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1MeshInferenceGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1MeshInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1MeshInferenceGuardNumber(value, at, bounds) : stdioSemioV1MeshInferenceGuardReject(at, "value is not an integer");
export const stdioSemioV1MeshInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1MeshInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1MeshInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1MeshInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioMeshInference(value: unknown, at = "$"): SemioMeshInference {
  const row = stdioSemioV1MeshInferenceGuardObject(value, at);
  return {
    aabb: stdioSemioV1MeshInferenceGuardObject(row["aabb"], `${at}.aabb`),
  };
}
