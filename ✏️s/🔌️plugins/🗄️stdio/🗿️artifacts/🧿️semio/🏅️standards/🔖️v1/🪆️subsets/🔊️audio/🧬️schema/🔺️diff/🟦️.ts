/** 🧬️ SemioAudioDiff schema. */
export interface SemioAudioDiff {
  schema?: string;
  bytes?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AudioDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AudioDiffGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AudioDiffGuardRefusal(at, why);
};

type stdioSemioV1AudioDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AudioDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AudioDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AudioDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AudioDiffGuardReject(at, "value is not an object");
export const stdioSemioV1AudioDiffGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AudioDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AudioDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AudioDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AudioDiffGuardString = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AudioDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AudioDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AudioDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AudioDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AudioDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AudioDiffGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AudioDiffGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AudioDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AudioDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AudioDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AudioDiffGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AudioDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AudioDiffGuardNumber(value, at, bounds) : stdioSemioV1AudioDiffGuardReject(at, "value is not an integer");
export const stdioSemioV1AudioDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AudioDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AudioDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AudioDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAudioDiff(value: unknown, at = "$"): SemioAudioDiff {
  const row = stdioSemioV1AudioDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioSemioV1AudioDiffGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioSemioV1AudioDiffGuardString(row["bytes"], `${at}.bytes`),
  };
}
