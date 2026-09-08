/** 🧬️ IfcArtifact schema. */
export interface IfcArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ PLACEHOLDER_TEXT_COLON: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc4AnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc4AnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioIfc4AnyArtifactGuardRefusal(at, why);
};

type stdioIfc4AnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc4AnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc4AnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc4AnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc4AnyArtifactGuardReject(at, "value is not an object");
export const stdioIfc4AnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioIfc4AnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc4AnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc4AnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc4AnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc4AnyArtifactGuardString = (value: unknown, at: string, bounds: stdioIfc4AnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc4AnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc4AnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc4AnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc4AnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc4AnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc4AnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioIfc4AnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioIfc4AnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc4AnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc4AnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc4AnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc4AnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioIfc4AnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc4AnyArtifactGuardNumber(value, at, bounds) : stdioIfc4AnyArtifactGuardReject(at, "value is not an integer");
export const stdioIfc4AnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc4AnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc4AnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc4AnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfcArtifact(value: unknown, at = "$"): IfcArtifact {
  const row = stdioIfc4AnyArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioIfc4AnyArtifactGuardString(row["schema"], `${at}.schema`),
    text: row["text"] === undefined ? undefined : stdioIfc4AnyArtifactGuardString(row["text"], `${at}.text`),
  };
}
