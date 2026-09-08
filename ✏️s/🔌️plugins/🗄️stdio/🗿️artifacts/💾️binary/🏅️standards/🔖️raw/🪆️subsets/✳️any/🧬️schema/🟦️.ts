/** 🧬️ BinaryArtifact schema. */
export interface BinaryArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ bytes: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBinaryRawAnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBinaryRawAnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioBinaryRawAnyArtifactGuardRefusal(at, why);
};

type stdioBinaryRawAnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBinaryRawAnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBinaryRawAnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBinaryRawAnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBinaryRawAnyArtifactGuardReject(at, "value is not an object");
export const stdioBinaryRawAnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioBinaryRawAnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBinaryRawAnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBinaryRawAnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBinaryRawAnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBinaryRawAnyArtifactGuardString = (value: unknown, at: string, bounds: stdioBinaryRawAnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBinaryRawAnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBinaryRawAnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBinaryRawAnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBinaryRawAnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBinaryRawAnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBinaryRawAnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioBinaryRawAnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioBinaryRawAnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBinaryRawAnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBinaryRawAnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBinaryRawAnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBinaryRawAnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioBinaryRawAnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBinaryRawAnyArtifactGuardNumber(value, at, bounds) : stdioBinaryRawAnyArtifactGuardReject(at, "value is not an integer");
export const stdioBinaryRawAnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBinaryRawAnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBinaryRawAnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBinaryRawAnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBinaryArtifact(value: unknown, at = "$"): BinaryArtifact {
  const row = stdioBinaryRawAnyArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioBinaryRawAnyArtifactGuardString(row["schema"], `${at}.schema`),
    bytes: row["bytes"] === undefined ? undefined : stdioBinaryRawAnyArtifactGuardString(row["bytes"], `${at}.bytes`),
  };
}
