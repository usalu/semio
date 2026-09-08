import type { LineEnding } from './📸️snapshot/🟦️.ts';

/** 🧬️ TxtArtifact schema. */
export interface TxtArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ lines: string[];
  /** @state artifact */ trailingNewline: boolean;
  /** @state artifact */ lineEnding: LineEnding;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTxtUtf8AnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTxtUtf8AnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioTxtUtf8AnyArtifactGuardRefusal(at, why);
};

type stdioTxtUtf8AnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTxtUtf8AnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTxtUtf8AnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTxtUtf8AnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTxtUtf8AnyArtifactGuardReject(at, "value is not an object");
export const stdioTxtUtf8AnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioTxtUtf8AnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTxtUtf8AnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTxtUtf8AnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTxtUtf8AnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTxtUtf8AnyArtifactGuardString = (value: unknown, at: string, bounds: stdioTxtUtf8AnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTxtUtf8AnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTxtUtf8AnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTxtUtf8AnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTxtUtf8AnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTxtUtf8AnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTxtUtf8AnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioTxtUtf8AnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioTxtUtf8AnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTxtUtf8AnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTxtUtf8AnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTxtUtf8AnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTxtUtf8AnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioTxtUtf8AnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTxtUtf8AnyArtifactGuardNumber(value, at, bounds) : stdioTxtUtf8AnyArtifactGuardReject(at, "value is not an integer");
export const stdioTxtUtf8AnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTxtUtf8AnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTxtUtf8AnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTxtUtf8AnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTxtArtifact(value: unknown, at = "$"): TxtArtifact {
  const row = stdioTxtUtf8AnyArtifactGuardObject(value, at);
  return {
    schema: stdioTxtUtf8AnyArtifactGuardString(row["schema"], `${at}.schema`),
    lines: stdioTxtUtf8AnyArtifactGuardArray(row["lines"], `${at}.lines`).map((item, index) => stdioTxtUtf8AnyArtifactGuardString(item, `${at}.lines[${index}]`)),
    trailingNewline: stdioTxtUtf8AnyArtifactGuardBoolean(row["trailingNewline"], `${at}.trailingNewline`),
    lineEnding: stdioTxtUtf8AnyArtifactGuardMember(row["lineEnding"], `${at}.lineEnding`, ["lf", "crLf"] as const),
  };
}
