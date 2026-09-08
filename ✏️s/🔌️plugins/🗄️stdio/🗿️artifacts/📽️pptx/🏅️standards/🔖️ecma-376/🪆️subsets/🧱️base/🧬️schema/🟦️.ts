/** 🧬️ Logical ECMA-376 PresentationML artifact schema. */
import type { OpcPackage, PptxPresentation, PptxXmlPart } from './📸️snapshot/🟦️.ts';
export interface PptxArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: PptxXmlPart[];
  /** @state artifact */ presentation: PptxPresentation;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPptxEcma376BaseArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPptxEcma376BaseArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioPptxEcma376BaseArtifactGuardRefusal(at, why);
};

type stdioPptxEcma376BaseArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPptxEcma376BaseArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPptxEcma376BaseArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPptxEcma376BaseArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPptxEcma376BaseArtifactGuardReject(at, "value is not an object");
export const stdioPptxEcma376BaseArtifactGuardArray = (value: unknown, at: string, bounds: stdioPptxEcma376BaseArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPptxEcma376BaseArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPptxEcma376BaseArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPptxEcma376BaseArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPptxEcma376BaseArtifactGuardString = (value: unknown, at: string, bounds: stdioPptxEcma376BaseArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPptxEcma376BaseArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPptxEcma376BaseArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPptxEcma376BaseArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPptxEcma376BaseArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPptxEcma376BaseArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPptxEcma376BaseArtifactGuardReject(at, "value is not a boolean"));
export const stdioPptxEcma376BaseArtifactGuardNumber = (value: unknown, at: string, bounds: stdioPptxEcma376BaseArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPptxEcma376BaseArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPptxEcma376BaseArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPptxEcma376BaseArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPptxEcma376BaseArtifactGuardInteger = (value: unknown, at: string, bounds: stdioPptxEcma376BaseArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPptxEcma376BaseArtifactGuardNumber(value, at, bounds) : stdioPptxEcma376BaseArtifactGuardReject(at, "value is not an integer");
export const stdioPptxEcma376BaseArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPptxEcma376BaseArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPptxEcma376BaseArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPptxEcma376BaseArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePptxArtifact(value: unknown, at = "$"): PptxArtifact {
  const row = stdioPptxEcma376BaseArtifactGuardObject(value, at);
  return {
    schema: stdioPptxEcma376BaseArtifactGuardString(row["schema"], `${at}.schema`),
    opc: stdioPptxEcma376BaseArtifactGuardObject(row["opc"], `${at}.opc`),
    xmlParts: stdioPptxEcma376BaseArtifactGuardArray(row["xmlParts"], `${at}.xmlParts`).map((item, index) => stdioPptxEcma376BaseArtifactGuardObject(item, `${at}.xmlParts[${index}]`)),
    presentation: stdioPptxEcma376BaseArtifactGuardObject(row["presentation"], `${at}.presentation`),
  };
}
