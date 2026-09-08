/** 🧬️ Logical ECMA-376 PresentationML diff schema. */
import type { PptxXmlPart } from '../📸️snapshot/🟦️.ts';
export interface PptxDiff {
  opc?: unknown;
  xmlParts?: PptxXmlPart[];
  presentation?: unknown;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPptxEcma376BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPptxEcma376BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioPptxEcma376BaseDiffGuardRefusal(at, why);
};

type stdioPptxEcma376BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPptxEcma376BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPptxEcma376BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPptxEcma376BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPptxEcma376BaseDiffGuardReject(at, "value is not an object");
export const stdioPptxEcma376BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPptxEcma376BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPptxEcma376BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPptxEcma376BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPptxEcma376BaseDiffGuardString = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPptxEcma376BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPptxEcma376BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPptxEcma376BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPptxEcma376BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPptxEcma376BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPptxEcma376BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioPptxEcma376BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPptxEcma376BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPptxEcma376BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPptxEcma376BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPptxEcma376BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioPptxEcma376BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPptxEcma376BaseDiffGuardNumber(value, at, bounds) : stdioPptxEcma376BaseDiffGuardReject(at, "value is not an integer");
export const stdioPptxEcma376BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPptxEcma376BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPptxEcma376BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPptxEcma376BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePptxDiff(value: unknown, at = "$"): PptxDiff {
  const row = stdioPptxEcma376BaseDiffGuardObject(value, at);
  return {
    opc: row["opc"] === undefined ? undefined : stdioPptxEcma376BaseDiffGuardObject(row["opc"], `${at}.opc`),
    xmlParts: row["xmlParts"] === undefined ? undefined : stdioPptxEcma376BaseDiffGuardArray(row["xmlParts"], `${at}.xmlParts`).map((item, index) => stdioPptxEcma376BaseDiffGuardObject(item, `${at}.xmlParts[${index}]`)),
    presentation: row["presentation"] === undefined ? undefined : stdioPptxEcma376BaseDiffGuardObject(row["presentation"], `${at}.presentation`),
  };
}
