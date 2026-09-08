import type { MdBlock } from './📸️snapshot/🟦️.ts';

/** 🧬️ Full `stdio.md` artifact state. */
export interface MdArtifact {
  schema: string;
  blocks: MdBlock[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnyArtifactGuardRefusal(at, why);
};

type stdioMdCommonmarkAnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnyArtifactGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnyArtifactGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnyArtifactGuardNumber(value, at, bounds) : stdioMdCommonmarkAnyArtifactGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdArtifact(value: unknown, at = "$"): MdArtifact {
  const row = stdioMdCommonmarkAnyArtifactGuardObject(value, at);
  return {
    schema: stdioMdCommonmarkAnyArtifactGuardString(row["schema"], `${at}.schema`),
    blocks: stdioMdCommonmarkAnyArtifactGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => stdioMdCommonmarkAnyArtifactGuardObject(item, `${at}.blocks[${index}]`)),
  };
}
