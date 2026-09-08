/** 🧬️ HtmlArtifact schema — full artifact state, mirrors `HtmlSnapshot` field for field. See the
 * sibling `📸️snapshot/🟦️.ts` for the canonical `HtmlNode`/`HtmlAttr`/`RawTextKind` shapes. */
import type { HtmlNode } from './📸️snapshot/🟦️.ts';
export type { HtmlNode };

export interface HtmlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ doctype?: string;
  /** @state artifact */ root: HtmlNode;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioHtml5AnyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioHtml5AnyArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioHtml5AnyArtifactGuardRefusal(at, why);
};

type stdioHtml5AnyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioHtml5AnyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioHtml5AnyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioHtml5AnyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioHtml5AnyArtifactGuardReject(at, "value is not an object");
export const stdioHtml5AnyArtifactGuardArray = (value: unknown, at: string, bounds: stdioHtml5AnyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioHtml5AnyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioHtml5AnyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioHtml5AnyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioHtml5AnyArtifactGuardString = (value: unknown, at: string, bounds: stdioHtml5AnyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioHtml5AnyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioHtml5AnyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioHtml5AnyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioHtml5AnyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioHtml5AnyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioHtml5AnyArtifactGuardReject(at, "value is not a boolean"));
export const stdioHtml5AnyArtifactGuardNumber = (value: unknown, at: string, bounds: stdioHtml5AnyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioHtml5AnyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioHtml5AnyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioHtml5AnyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioHtml5AnyArtifactGuardInteger = (value: unknown, at: string, bounds: stdioHtml5AnyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioHtml5AnyArtifactGuardNumber(value, at, bounds) : stdioHtml5AnyArtifactGuardReject(at, "value is not an integer");
export const stdioHtml5AnyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioHtml5AnyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioHtml5AnyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioHtml5AnyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHtmlArtifact(value: unknown, at = "$"): HtmlArtifact {
  const row = stdioHtml5AnyArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : stdioHtml5AnyArtifactGuardString(row["schema"], `${at}.schema`),
    entries: row["entries"] === undefined ? undefined : stdioHtml5AnyArtifactGuardArray(row["entries"], `${at}.entries`).map((item, index) => stdioHtml5AnyArtifactGuardObject(item, `${at}.entries[${index}]`)),
  };
}
