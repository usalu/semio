import type { XmlDocument } from '../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';

/** 🧬️ Full logical SVG artifact state. */
export interface SvgArtifact {
  schema: string;
  doc: XmlDocument;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSvg11BaseArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSvg11BaseArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioSvg11BaseArtifactGuardRefusal(at, why);
};

type stdioSvg11BaseArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSvg11BaseArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSvg11BaseArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSvg11BaseArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSvg11BaseArtifactGuardReject(at, "value is not an object");
export const stdioSvg11BaseArtifactGuardArray = (value: unknown, at: string, bounds: stdioSvg11BaseArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSvg11BaseArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSvg11BaseArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSvg11BaseArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSvg11BaseArtifactGuardString = (value: unknown, at: string, bounds: stdioSvg11BaseArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSvg11BaseArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSvg11BaseArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSvg11BaseArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSvg11BaseArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSvg11BaseArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSvg11BaseArtifactGuardReject(at, "value is not a boolean"));
export const stdioSvg11BaseArtifactGuardNumber = (value: unknown, at: string, bounds: stdioSvg11BaseArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSvg11BaseArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSvg11BaseArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSvg11BaseArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSvg11BaseArtifactGuardInteger = (value: unknown, at: string, bounds: stdioSvg11BaseArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSvg11BaseArtifactGuardNumber(value, at, bounds) : stdioSvg11BaseArtifactGuardReject(at, "value is not an integer");
export const stdioSvg11BaseArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSvg11BaseArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSvg11BaseArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSvg11BaseArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSvgArtifact(value: unknown, at = "$"): SvgArtifact {
  const row = stdioSvg11BaseArtifactGuardObject(value, at);
  return {
    schema: stdioSvg11BaseArtifactGuardString(row["schema"], `${at}.schema`),
    doc: stdioSvg11BaseArtifactGuardObject(row["doc"], `${at}.doc`),
  };
}
