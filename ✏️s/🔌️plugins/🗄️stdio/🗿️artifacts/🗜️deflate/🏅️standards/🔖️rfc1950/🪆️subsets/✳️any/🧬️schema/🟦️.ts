/** 🧬️ DeflateArtifact schema — mirrors DeflateSnapshot's typed RFC1950 fields. */
import type { DeflateLevelHint } from './📸️snapshot/🟦️.ts';

export interface DeflateArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ compressionMethod: number;
  /** @state artifact */ windowBits: number;
  /** @state artifact */ compressionLevelHint: DeflateLevelHint;
  /** @state artifact */ dictId?: number;
  /** @state artifact */ payload: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDeflateRfc1950AnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDeflateRfc1950AnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioDeflateRfc1950AnyArtifactGuardRefusal(at, why);
};

type stdioDeflateRfc1950AnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDeflateRfc1950AnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDeflateRfc1950AnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDeflateRfc1950AnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDeflateRfc1950AnyArtifactGuardReject(at, "value is not an object");
export const stdioDeflateRfc1950AnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDeflateRfc1950AnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDeflateRfc1950AnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDeflateRfc1950AnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDeflateRfc1950AnyArtifactGuardString = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDeflateRfc1950AnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDeflateRfc1950AnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDeflateRfc1950AnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDeflateRfc1950AnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDeflateRfc1950AnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDeflateRfc1950AnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioDeflateRfc1950AnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDeflateRfc1950AnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDeflateRfc1950AnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDeflateRfc1950AnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDeflateRfc1950AnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDeflateRfc1950AnyArtifactGuardNumber(value, at, bounds) : stdioDeflateRfc1950AnyArtifactGuardReject(at, "value is not an integer");
export const stdioDeflateRfc1950AnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDeflateRfc1950AnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDeflateRfc1950AnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDeflateRfc1950AnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDeflateArtifact(value: unknown, at = "$"): DeflateArtifact {
  const row = stdioDeflateRfc1950AnyArtifactGuardObject(value, at);
  return {
    schema: stdioDeflateRfc1950AnyArtifactGuardString(row["schema"], `${at}.schema`),
    compressionMethod: stdioDeflateRfc1950AnyArtifactGuardInteger(row["compressionMethod"], `${at}.compressionMethod`, {"minimum": 0, "maximum": 15}),
    windowBits: stdioDeflateRfc1950AnyArtifactGuardInteger(row["windowBits"], `${at}.windowBits`, {"minimum": 0, "maximum": 15}),
    compressionLevelHint: stdioDeflateRfc1950AnyArtifactGuardMember(row["compressionLevelHint"], `${at}.compressionLevelHint`, ["fastest", "fast", "default", "maximum"] as const),
    dictId: row["dictId"] === undefined ? undefined : stdioDeflateRfc1950AnyArtifactGuardInteger(row["dictId"], `${at}.dictId`, {"minimum": 0, "maximum": 4294967295}),
    payload: stdioDeflateRfc1950AnyArtifactGuardArray(row["payload"], `${at}.payload`).map((item, index) => stdioDeflateRfc1950AnyArtifactGuardInteger(item, `${at}.payload[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}
