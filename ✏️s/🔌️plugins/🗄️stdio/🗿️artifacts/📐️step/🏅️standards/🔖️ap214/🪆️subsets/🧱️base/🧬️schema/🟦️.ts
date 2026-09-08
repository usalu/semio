/** 🧬️ StepArtifact schema. */
export interface StepArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ PLACEHOLDER_TEXT_COLON: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStepAp214BaseArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStepAp214BaseArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioStepAp214BaseArtifactGuardRefusal(at, why);
};

type stdioStepAp214BaseArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStepAp214BaseArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStepAp214BaseArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStepAp214BaseArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStepAp214BaseArtifactGuardReject(at, "value is not an object");
export const stdioStepAp214BaseArtifactGuardArray = (value: unknown, at: string, bounds: stdioStepAp214BaseArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStepAp214BaseArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStepAp214BaseArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStepAp214BaseArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStepAp214BaseArtifactGuardString = (value: unknown, at: string, bounds: stdioStepAp214BaseArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStepAp214BaseArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStepAp214BaseArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStepAp214BaseArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStepAp214BaseArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStepAp214BaseArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStepAp214BaseArtifactGuardReject(at, "value is not a boolean"));
export const stdioStepAp214BaseArtifactGuardNumber = (value: unknown, at: string, bounds: stdioStepAp214BaseArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStepAp214BaseArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStepAp214BaseArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStepAp214BaseArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStepAp214BaseArtifactGuardInteger = (value: unknown, at: string, bounds: stdioStepAp214BaseArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStepAp214BaseArtifactGuardNumber(value, at, bounds) : stdioStepAp214BaseArtifactGuardReject(at, "value is not an integer");
export const stdioStepAp214BaseArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStepAp214BaseArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStepAp214BaseArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStepAp214BaseArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStepArtifact(value: unknown, at = "$"): StepArtifact {
  const row = stdioStepAp214BaseArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioStepAp214BaseArtifactGuardString(row["schema"], `${at}.schema`),
    text: row["text"] === undefined ? undefined : stdioStepAp214BaseArtifactGuardString(row["text"], `${at}.text`),
  };
}
