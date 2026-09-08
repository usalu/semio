/** 🧬️ TsvArtifact schema facet — full artifact state, mirrors TsvSnapshot field-for-field. */
import type { TsvLineEnding } from './📸️snapshot/🟦️.ts';

export interface TsvArtifact {
  schema: string;
  records: string[][];
  trailingNewline: boolean;
  lineEnding: TsvLineEnding;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnyArtifactGuardRefusal(at, why);
};

type stdioTsvIanaAnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnyArtifactGuardReject(at, "value is not an object");
export const stdioTsvIanaAnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnyArtifactGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnyArtifactGuardNumber(value, at, bounds) : stdioTsvIanaAnyArtifactGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvArtifact(value: unknown, at = "$"): TsvArtifact {
  const row = stdioTsvIanaAnyArtifactGuardObject(value, at);
  return {
    schema: stdioTsvIanaAnyArtifactGuardString(row["schema"], `${at}.schema`),
    records: stdioTsvIanaAnyArtifactGuardArray(row["records"], `${at}.records`).map((item, index) => stdioTsvIanaAnyArtifactGuardArray(item, `${at}.records[${index}]`).map((item, index) => stdioTsvIanaAnyArtifactGuardString(item, `${at}.records[${index}][${index}]`))),
    trailingNewline: stdioTsvIanaAnyArtifactGuardBoolean(row["trailingNewline"], `${at}.trailingNewline`),
    lineEnding: stdioTsvIanaAnyArtifactGuardMember(row["lineEnding"], `${at}.lineEnding`, ["lf", "crlf"] as const),
  };
}
