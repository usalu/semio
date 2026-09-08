import type { XmlDeclaration, XmlDoctype, XmlNode } from '../📸️snapshot/🟦️.ts';

/** 🔺️ Diff for `stdio.xml`. `declaration`/`doctype` are tri-state (`null` = cleared, absent =
 * unchanged, present = set). */
export interface XmlDiff {
  prolog?: XmlNode[];
  declaration?: XmlDeclaration | null;
  doctype?: XmlDoctype | null;
  root?: XmlNodeDiff;
}

/** 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets. */
export type XmlNodeDiff =
  | { kind: 'element'; name?: string; attributes?: XmlAttributesDiff; children?: XmlChildrenDiff }
  | { kind: 'text'; text?: string }
  | { kind: 'replace'; node?: XmlNode };

/** 🏷️ Name-keyed, order-preserving attribute triple. */
export interface XmlAttributesDiff {
  removed: string[];
  modified: XmlAttrModified[];
  added: XmlAttrAdded[];
}

export interface XmlAttrModified {
  name: string;
  value: string;
}

export interface XmlAttrAdded {
  index: number;
  name: string;
  value: string;
}

/** 🌳 Index-keyed, recursive children triple. */
export interface XmlChildrenDiff {
  removed: number[];
  modified: XmlChildModified[];
  added: XmlChildAdded[];
}

export interface XmlChildModified {
  index: number;
  diff: XmlNodeDiff;
}

export interface XmlChildAdded {
  index: number;
  item: XmlNode;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXml10BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXml10BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioXml10BaseDiffGuardRefusal(at, why);
};

type stdioXml10BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXml10BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXml10BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXml10BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXml10BaseDiffGuardReject(at, "value is not an object");
export const stdioXml10BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioXml10BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXml10BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXml10BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXml10BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXml10BaseDiffGuardString = (value: unknown, at: string, bounds: stdioXml10BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXml10BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXml10BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXml10BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXml10BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXml10BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXml10BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioXml10BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioXml10BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXml10BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXml10BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXml10BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXml10BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioXml10BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXml10BaseDiffGuardNumber(value, at, bounds) : stdioXml10BaseDiffGuardReject(at, "value is not an integer");
export const stdioXml10BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXml10BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXml10BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXml10BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXmlDiff(value: unknown, at = "$"): XmlDiff {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    prolog: row["prolog"] === undefined ? undefined : stdioXml10BaseDiffGuardArray(row["prolog"], `${at}.prolog`).map((item, index) => stdioXml10BaseDiffGuardObject(item, `${at}.prolog[${index}]`)),
    declaration: row["declaration"] === undefined ? undefined : parseNullableXmlDeclaration(row["declaration"], `${at}.declaration`),
    doctype: row["doctype"] === undefined ? undefined : parseNullableString(row["doctype"], `${at}.doctype`),
    root: row["root"] === undefined ? undefined : parseXmlNodeDiff(row["root"], `${at}.root`),
  };
}

export function parseXmlAttrModified(value: unknown, at = "$"): XmlAttrModified {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    name: stdioXml10BaseDiffGuardString(row["name"], `${at}.name`),
    value: stdioXml10BaseDiffGuardString(row["value"], `${at}.value`),
  };
}

export function parseXmlAttrAdded(value: unknown, at = "$"): XmlAttrAdded {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    index: stdioXml10BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    name: stdioXml10BaseDiffGuardString(row["name"], `${at}.name`),
    value: stdioXml10BaseDiffGuardString(row["value"], `${at}.value`),
  };
}

export function parseXmlAttributesDiff(value: unknown, at = "$"): XmlAttributesDiff {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    removed: stdioXml10BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioXml10BaseDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: stdioXml10BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseXmlAttrModified(item, `${at}.modified[${index}]`)),
    added: stdioXml10BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseXmlAttrAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseXmlChildModified(value: unknown, at = "$"): XmlChildModified {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    index: stdioXml10BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseXmlNodeDiff(row["diff"], `${at}.diff`),
  };
}

export function parseXmlChildAdded(value: unknown, at = "$"): XmlChildAdded {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    index: stdioXml10BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    item: stdioXml10BaseDiffGuardObject(row["item"], `${at}.item`),
  };
}

export function parseXmlChildrenDiff(value: unknown, at = "$"): XmlChildrenDiff {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    removed: stdioXml10BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioXml10BaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: stdioXml10BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseXmlChildModified(item, `${at}.modified[${index}]`)),
    added: stdioXml10BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseXmlChildAdded(item, `${at}.added[${index}]`)),
  };
}

export interface XmlElementDiff {
  readonly name?: string;
  readonly attributes?: XmlAttributesDiff;
  readonly children?: XmlChildrenDiff;
}

export function parseXmlElementDiff(value: unknown, at = "$"): XmlElementDiff {
  const row = stdioXml10BaseDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioXml10BaseDiffGuardString(row["name"], `${at}.name`),
    attributes: row["attributes"] === undefined ? undefined : parseXmlAttributesDiff(row["attributes"], `${at}.attributes`),
    children: row["children"] === undefined ? undefined : parseXmlChildrenDiff(row["children"], `${at}.children`),
  };
}
