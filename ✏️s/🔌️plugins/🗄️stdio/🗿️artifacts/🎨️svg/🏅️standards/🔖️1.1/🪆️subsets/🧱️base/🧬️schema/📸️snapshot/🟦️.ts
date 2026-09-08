import type { XmlDocument } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';

/** 📸️ Persisted logical SVG document. */
export interface SvgSnapshot {
  schema: string;
  doc: XmlDocument;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSvg11BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSvg11BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioSvg11BaseSnapshotGuardRefusal(at, why);
};

type stdioSvg11BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSvg11BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSvg11BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSvg11BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSvg11BaseSnapshotGuardReject(at, "value is not an object");
export const stdioSvg11BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioSvg11BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSvg11BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSvg11BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSvg11BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSvg11BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioSvg11BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSvg11BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSvg11BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSvg11BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSvg11BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSvg11BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSvg11BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioSvg11BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioSvg11BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSvg11BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSvg11BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSvg11BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSvg11BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioSvg11BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSvg11BaseSnapshotGuardNumber(value, at, bounds) : stdioSvg11BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioSvg11BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSvg11BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSvg11BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSvg11BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSvgSnapshot(value: unknown, at = "$"): SvgSnapshot {
  const row = stdioSvg11BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioSvg11BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    doc: stdioSvg11BaseSnapshotGuardObject(row["doc"], `${at}.doc`),
  };
}
