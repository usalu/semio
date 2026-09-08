// 🌳 `HtmlSnapshot.root` is an `HtmlNode`, and `HtmlDiff`/`HtmlNodeDiff` diff that same node tree
// directly -- own types throughout (HTML is not XML; only the general "recursive node tree"
// *structural pattern* is borrowed from svg/xml, per the ticket brief).
import type { HtmlNode, RawTextKind } from '../📸️snapshot/🟦️.ts';
export type { HtmlNode, RawTextKind };

/** 🔺️ Diff for `stdio.html`. `doctype` is tri-state (`null` = cleared, absent = unchanged,
 * present = set). No `snapshot`-shaped full-replace field anywhere -- even a `setSnapshot`
 * mutation's diff is the sparse field-by-field delta below. */
export interface HtmlDiff {
  doctype?: string | null;
  root?: HtmlNodeDiff;
}

/** 🌳 Recursive per-node diff, shaped like the `HtmlNode` it targets. */
export type HtmlNodeDiff =
  | { kind: 'element'; name?: string; attributes?: HtmlAttributesDiff; children?: HtmlChildrenDiff }
  | { kind: 'text'; text?: string }
  | { kind: 'comment'; text?: string }
  | { kind: 'rawText'; parentKind?: RawTextKind; text?: string }
  | { kind: 'replace'; node: HtmlNode };

/** 🏷️ Name-keyed, order-preserving attribute triple. */
export interface HtmlAttributesDiff {
  removed: string[];
  modified: HtmlAttrModified[];
  added: HtmlAttrAdded[];
}

export interface HtmlAttrModified {
  name: string;
  /** absent = the attribute is now valueless. */
  value?: string;
}

export interface HtmlAttrAdded {
  index: number;
  name: string;
  value?: string;
}

/** 🌳 Index-keyed, recursive children triple. */
export interface HtmlChildrenDiff {
  removed: number[];
  modified: HtmlChildModified[];
  added: HtmlChildAdded[];
}

export interface HtmlChildModified {
  index: number;
  diff: HtmlNodeDiff;
}

export interface HtmlChildAdded {
  index: number;
  item: HtmlNode;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioHtml5AnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioHtml5AnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioHtml5AnyDiffGuardRefusal(at, why);
};

type stdioHtml5AnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioHtml5AnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioHtml5AnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioHtml5AnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioHtml5AnyDiffGuardReject(at, "value is not an object");
export const stdioHtml5AnyDiffGuardArray = (value: unknown, at: string, bounds: stdioHtml5AnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioHtml5AnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioHtml5AnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioHtml5AnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioHtml5AnyDiffGuardString = (value: unknown, at: string, bounds: stdioHtml5AnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioHtml5AnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioHtml5AnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioHtml5AnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioHtml5AnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioHtml5AnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioHtml5AnyDiffGuardReject(at, "value is not a boolean"));
export const stdioHtml5AnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioHtml5AnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioHtml5AnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioHtml5AnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioHtml5AnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioHtml5AnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioHtml5AnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioHtml5AnyDiffGuardNumber(value, at, bounds) : stdioHtml5AnyDiffGuardReject(at, "value is not an integer");
export const stdioHtml5AnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioHtml5AnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioHtml5AnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioHtml5AnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHtmlDiff(value: unknown, at = "$"): HtmlDiff {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    doctype: row["doctype"] === undefined ? undefined : parseNullableString(row["doctype"], `${at}.doctype`),
    root: row["root"] === undefined ? undefined : parseHtmlNodeDiff(row["root"], `${at}.root`),
  };
}

export function parseHtmlAttrModified(value: unknown, at = "$"): HtmlAttrModified {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    name: stdioHtml5AnyDiffGuardString(row["name"], `${at}.name`),
    value: row["value"] === undefined ? undefined : parseNullableAttrValue(row["value"], `${at}.value`),
  };
}

export function parseHtmlAttrAdded(value: unknown, at = "$"): HtmlAttrAdded {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    index: stdioHtml5AnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    name: stdioHtml5AnyDiffGuardString(row["name"], `${at}.name`),
    value: row["value"] === undefined ? undefined : parseNullableAttrValue(row["value"], `${at}.value`),
  };
}

export function parseHtmlAttributesDiff(value: unknown, at = "$"): HtmlAttributesDiff {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    removed: stdioHtml5AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioHtml5AnyDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: stdioHtml5AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseHtmlAttrModified(item, `${at}.modified[${index}]`)),
    added: stdioHtml5AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseHtmlAttrAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseHtmlChildModified(value: unknown, at = "$"): HtmlChildModified {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    index: stdioHtml5AnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseHtmlNodeDiff(row["diff"], `${at}.diff`),
  };
}

export function parseHtmlChildrenDiff(value: unknown, at = "$"): HtmlChildrenDiff {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    removed: stdioHtml5AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioHtml5AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: stdioHtml5AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseHtmlChildModified(item, `${at}.modified[${index}]`)),
    added: stdioHtml5AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseHtmlChildAdded(item, `${at}.added[${index}]`)),
  };
}

export interface HtmlElementDiff {
  readonly name?: string;
  readonly attributes?: HtmlAttributesDiff;
  readonly children?: HtmlChildrenDiff;
}

export function parseHtmlElementDiff(value: unknown, at = "$"): HtmlElementDiff {
  const row = stdioHtml5AnyDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioHtml5AnyDiffGuardString(row["name"], `${at}.name`),
    attributes: row["attributes"] === undefined ? undefined : parseHtmlAttributesDiff(row["attributes"], `${at}.attributes`),
    children: row["children"] === undefined ? undefined : parseHtmlChildrenDiff(row["children"], `${at}.children`),
  };
}
