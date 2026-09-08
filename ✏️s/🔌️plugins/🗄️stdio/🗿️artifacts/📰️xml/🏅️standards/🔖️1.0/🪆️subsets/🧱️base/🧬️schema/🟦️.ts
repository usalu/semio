/** 🧬️ XmlArtifact schema — full persisted state. */
import type { XmlDocument } from './📸️snapshot/🟦️.ts';

export interface XmlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ doc: XmlDocument;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXml10BaseArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXml10BaseArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioXml10BaseArtifactGuardRefusal(at, why);
};

type stdioXml10BaseArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXml10BaseArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXml10BaseArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXml10BaseArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXml10BaseArtifactGuardReject(at, "value is not an object");
export const stdioXml10BaseArtifactGuardArray = (value: unknown, at: string, bounds: stdioXml10BaseArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXml10BaseArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXml10BaseArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXml10BaseArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXml10BaseArtifactGuardString = (value: unknown, at: string, bounds: stdioXml10BaseArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXml10BaseArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXml10BaseArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXml10BaseArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXml10BaseArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXml10BaseArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXml10BaseArtifactGuardReject(at, "value is not a boolean"));
export const stdioXml10BaseArtifactGuardNumber = (value: unknown, at: string, bounds: stdioXml10BaseArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXml10BaseArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXml10BaseArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXml10BaseArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXml10BaseArtifactGuardInteger = (value: unknown, at: string, bounds: stdioXml10BaseArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXml10BaseArtifactGuardNumber(value, at, bounds) : stdioXml10BaseArtifactGuardReject(at, "value is not an integer");
export const stdioXml10BaseArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXml10BaseArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXml10BaseArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXml10BaseArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXmlArtifact(value: unknown, at = "$"): XmlArtifact {
  const row = stdioXml10BaseArtifactGuardObject(value, at);
  return {
    schema: stdioXml10BaseArtifactGuardString(row["schema"], `${at}.schema`),
    doc: stdioXml10BaseArtifactGuardObject(row["doc"], `${at}.doc`),
  };
}
