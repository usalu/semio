// 🌳 `SvgSnapshot.doc` wraps an `XmlDocument`, and `SvgDiff`/`SvgNodeDiff` diff that same node
// tree directly, per the plan's spec-mandated-reuse rule -- svg embeds xml's NODE model (real
// import, the canonical shape), but declares its own DIFF types below.
import type { XmlDeclaration, XmlDoctype, XmlNode } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
export type { XmlDeclaration, XmlDoctype, XmlNode };

/** 🔺️ Diff for `stdio.svg`. `declaration`/`doctype` are tri-state (`null` = cleared, absent =
 * unchanged, present = set). No `snapshot`-shaped full-replace field anywhere -- even a
 * `setSnapshot` mutation's diff is the sparse field-by-field delta below. */
export interface SvgDiff {
  prolog?: XmlNode[];
  declaration?: XmlDeclaration | null;
  doctype?: XmlDoctype | null;
  root?: SvgNodeDiff;
}

/** 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets. */
export type SvgNodeDiff =
  | { kind: 'element'; name?: string; attributes?: SvgAttributesDiff; children?: SvgChildrenDiff }
  | { kind: 'text'; text?: string }
  | { kind: 'replace'; node?: XmlNode };

/** 🏷️ Name-keyed, order-preserving attribute triple. */
export interface SvgAttributesDiff {
  removed: string[];
  modified: SvgAttrModified[];
  added: SvgAttrAdded[];
}

export interface SvgAttrModified {
  name: string;
  value: string;
}

export interface SvgAttrAdded {
  index: number;
  name: string;
  value: string;
}

/** 🌳 Index-keyed, recursive children triple. */
export interface SvgChildrenDiff {
  removed: number[];
  modified: SvgChildModified[];
  added: SvgChildAdded[];
}

export interface SvgChildModified {
  index: number;
  diff: SvgNodeDiff;
}

export interface SvgChildAdded {
  index: number;
  item: XmlNode;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSvg11BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSvg11BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioSvg11BaseDiffGuardRefusal(at, why);
};

type stdioSvg11BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSvg11BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSvg11BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSvg11BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSvg11BaseDiffGuardReject(at, "value is not an object");
export const stdioSvg11BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioSvg11BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSvg11BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSvg11BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSvg11BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSvg11BaseDiffGuardString = (value: unknown, at: string, bounds: stdioSvg11BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSvg11BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSvg11BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSvg11BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSvg11BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSvg11BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSvg11BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioSvg11BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioSvg11BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSvg11BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSvg11BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSvg11BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSvg11BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioSvg11BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSvg11BaseDiffGuardNumber(value, at, bounds) : stdioSvg11BaseDiffGuardReject(at, "value is not an integer");
export const stdioSvg11BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSvg11BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSvg11BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSvg11BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSvgDiff(value: unknown, at = "$"): SvgDiff {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    prolog: row["prolog"] === undefined ? undefined : stdioSvg11BaseDiffGuardArray(row["prolog"], `${at}.prolog`).map((item, index) => parseXmlNode(item, `${at}.prolog[${index}]`)),
    declaration: row["declaration"] === undefined ? undefined : parseNullableXmlDeclaration(row["declaration"], `${at}.declaration`),
    doctype: row["doctype"] === undefined ? undefined : parseNullableString(row["doctype"], `${at}.doctype`),
    root: row["root"] === undefined ? undefined : parseSvgNodeDiff(row["root"], `${at}.root`),
  };
}

export interface XmlAttr {
  readonly name: string;
  readonly value: string;
}

export function parseXmlAttr(value: unknown, at = "$"): XmlAttr {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    name: stdioSvg11BaseDiffGuardString(row["name"], `${at}.name`),
    value: stdioSvg11BaseDiffGuardString(row["value"], `${at}.value`),
  };
}

export type XmlNode = Readonly<Record<string, unknown>>;

export function parseXmlNode(value: unknown, at = "$"): XmlNode {
  return stdioSvg11BaseDiffGuardObject(value, `${at}`);
}

export interface XmlDeclaration {
  readonly version: string;
  readonly encoding?: string;
  readonly standalone?: boolean;
}

export function parseXmlDeclaration(value: unknown, at = "$"): XmlDeclaration {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    version: stdioSvg11BaseDiffGuardString(row["version"], `${at}.version`),
    encoding: row["encoding"] === undefined ? undefined : stdioSvg11BaseDiffGuardString(row["encoding"], `${at}.encoding`),
    standalone: row["standalone"] === undefined ? undefined : stdioSvg11BaseDiffGuardBoolean(row["standalone"], `${at}.standalone`),
  };
}

export function parseSvgAttrModified(value: unknown, at = "$"): SvgAttrModified {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    name: stdioSvg11BaseDiffGuardString(row["name"], `${at}.name`),
    value: stdioSvg11BaseDiffGuardString(row["value"], `${at}.value`),
  };
}

export function parseSvgAttrAdded(value: unknown, at = "$"): SvgAttrAdded {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    index: stdioSvg11BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    name: stdioSvg11BaseDiffGuardString(row["name"], `${at}.name`),
    value: stdioSvg11BaseDiffGuardString(row["value"], `${at}.value`),
  };
}

export function parseSvgAttributesDiff(value: unknown, at = "$"): SvgAttributesDiff {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    removed: stdioSvg11BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioSvg11BaseDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: stdioSvg11BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseSvgAttrModified(item, `${at}.modified[${index}]`)),
    added: stdioSvg11BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseSvgAttrAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseSvgChildModified(value: unknown, at = "$"): SvgChildModified {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    index: stdioSvg11BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseSvgNodeDiff(row["diff"], `${at}.diff`),
  };
}

export function parseSvgChildAdded(value: unknown, at = "$"): SvgChildAdded {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    index: stdioSvg11BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    item: parseXmlNode(row["item"], `${at}.item`),
  };
}

export function parseSvgChildrenDiff(value: unknown, at = "$"): SvgChildrenDiff {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    removed: stdioSvg11BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioSvg11BaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: stdioSvg11BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseSvgChildModified(item, `${at}.modified[${index}]`)),
    added: stdioSvg11BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseSvgChildAdded(item, `${at}.added[${index}]`)),
  };
}

export interface SvgElementDiff {
  readonly name?: string;
  readonly attributes?: SvgAttributesDiff;
  readonly children?: SvgChildrenDiff;
}

export function parseSvgElementDiff(value: unknown, at = "$"): SvgElementDiff {
  const row = stdioSvg11BaseDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioSvg11BaseDiffGuardString(row["name"], `${at}.name`),
    attributes: row["attributes"] === undefined ? undefined : parseSvgAttributesDiff(row["attributes"], `${at}.attributes`),
    children: row["children"] === undefined ? undefined : parseSvgChildrenDiff(row["children"], `${at}.children`),
  };
}
