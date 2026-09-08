/** 🧬️ SemioDocumentArtifact — full artifact state, mirrors `SemioDocumentSnapshot` field for
 * field (see `📸️snapshot/🟦️.ts` for the real per-field shapes). */
import type { DocBlock, DocImage, DocStyle } from "./📸️snapshot/🟦️";

export interface SemioDocumentArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ styles: DocStyle[];
  /** @state artifact */ images: DocImage[];
  /** @state artifact */ blocks: DocBlock[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DocumentArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DocumentArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DocumentArtifactGuardRefusal(at, why);
};

type stdioSemioV1DocumentArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DocumentArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DocumentArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DocumentArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DocumentArtifactGuardReject(at, "value is not an object");
export const stdioSemioV1DocumentArtifactGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DocumentArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DocumentArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DocumentArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DocumentArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DocumentArtifactGuardString = (value: unknown, at: string, bounds: stdioSemioV1DocumentArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DocumentArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DocumentArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DocumentArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DocumentArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DocumentArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DocumentArtifactGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DocumentArtifactGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DocumentArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DocumentArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DocumentArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DocumentArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DocumentArtifactGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DocumentArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DocumentArtifactGuardNumber(value, at, bounds) : stdioSemioV1DocumentArtifactGuardReject(at, "value is not an integer");
export const stdioSemioV1DocumentArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DocumentArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DocumentArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DocumentArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioDocumentArtifact(value: unknown, at = "$"): SemioDocumentArtifact {
  const row = stdioSemioV1DocumentArtifactGuardObject(value, at);
  return {
    schema: stdioSemioV1DocumentArtifactGuardString(row["schema"], `${at}.schema`),
    styles: stdioSemioV1DocumentArtifactGuardArray(row["styles"], `${at}.styles`).map((item, index) => stdioSemioV1DocumentArtifactGuardObject(item, `${at}.styles[${index}]`)),
    images: stdioSemioV1DocumentArtifactGuardArray(row["images"], `${at}.images`).map((item, index) => stdioSemioV1DocumentArtifactGuardObject(item, `${at}.images[${index}]`)),
    blocks: stdioSemioV1DocumentArtifactGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => stdioSemioV1DocumentArtifactGuardObject(item, `${at}.blocks[${index}]`)),
  };
}
