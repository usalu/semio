/** 🏷️ XML attribute pair. */
export interface XmlAttr {
  name: string;
  value: string;
}

/** 🌳 XML node: element, text, CDATA, comment, or processing instruction. */
export type XmlNode =
  | { kind: 'element'; name: string; attrs: XmlAttr[]; children: XmlNode[] }
  | { kind: 'text'; text: string }
  | { kind: 'cData'; text: string }
  | { kind: 'comment'; text: string }
  | { kind: 'processingInstruction'; target: string; data: string };

/** 🏳️ Typed `<?xml version="1.0" encoding="..." standalone="..."?>` declaration. */
export interface XmlDeclaration {
  version: string;
  encoding?: string;
  standalone?: boolean;
}

export type XmlExternalId =
  | { kind: 'system'; systemId: string }
  | { kind: 'public'; publicId: string; systemId: string };
export type XmlDtdDeclaration = { kind: 'entity'; parameter: boolean; name: string; value: string };
export interface XmlDoctype { name: string; externalId?: XmlExternalId; declarations: XmlDtdDeclaration[]; }

/** 📰 Well-formed XML document root. */
export interface XmlDocument {
  root?: XmlNode;
  doctype?: XmlDoctype;
  declaration?: XmlDeclaration;
  prolog: XmlNode[];
}

/** 📸️ Persisted `stdio.xml` snapshot. */
export interface XmlSnapshot {
  schema: string;
  doc: XmlDocument;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXml10BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXml10BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioXml10BaseSnapshotGuardRefusal(at, why);
};

type stdioXml10BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXml10BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXml10BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXml10BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXml10BaseSnapshotGuardReject(at, "value is not an object");
export const stdioXml10BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioXml10BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXml10BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXml10BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXml10BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXml10BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioXml10BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXml10BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXml10BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXml10BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXml10BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXml10BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXml10BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioXml10BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioXml10BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXml10BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXml10BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXml10BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXml10BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioXml10BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXml10BaseSnapshotGuardNumber(value, at, bounds) : stdioXml10BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioXml10BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXml10BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXml10BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXml10BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXmlSnapshot(value: unknown, at = "$"): XmlSnapshot {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioXml10BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    doc: parseXmlDocument(row["doc"], `${at}.doc`),
  };
}

export function parseXmlAttr(value: unknown, at = "$"): XmlAttr {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    name: stdioXml10BaseSnapshotGuardString(row["name"], `${at}.name`),
    value: stdioXml10BaseSnapshotGuardString(row["value"], `${at}.value`),
  };
}

export function parseXmlDeclaration(value: unknown, at = "$"): XmlDeclaration {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    version: stdioXml10BaseSnapshotGuardString(row["version"], `${at}.version`),
    encoding: row["encoding"] === undefined ? undefined : stdioXml10BaseSnapshotGuardString(row["encoding"], `${at}.encoding`),
    standalone: row["standalone"] === undefined ? undefined : stdioXml10BaseSnapshotGuardBoolean(row["standalone"], `${at}.standalone`),
  };
}

export function parseXmlDtdDeclaration(value: unknown, at = "$"): XmlDtdDeclaration {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    kind: stdioXml10BaseSnapshotGuardConstant(row["kind"], `${at}.kind`, "entity"),
    parameter: stdioXml10BaseSnapshotGuardBoolean(row["parameter"], `${at}.parameter`),
    name: stdioXml10BaseSnapshotGuardString(row["name"], `${at}.name`),
    value: stdioXml10BaseSnapshotGuardString(row["value"], `${at}.value`),
  };
}

export function parseXmlDoctype(value: unknown, at = "$"): XmlDoctype {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    name: stdioXml10BaseSnapshotGuardString(row["name"], `${at}.name`),
    externalId: row["externalId"] === undefined ? undefined : parseXmlExternalId(row["externalId"], `${at}.externalId`),
    declarations: row["declarations"] === undefined ? undefined : stdioXml10BaseSnapshotGuardArray(row["declarations"], `${at}.declarations`).map((item, index) => parseXmlDtdDeclaration(item, `${at}.declarations[${index}]`)),
  };
}

export function parseXmlDocument(value: unknown, at = "$"): XmlDocument {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    root: row["root"] === undefined ? undefined : parseXmlNode(row["root"], `${at}.root`),
    doctype: row["doctype"] === undefined ? undefined : parseXmlDoctype(row["doctype"], `${at}.doctype`),
    declaration: row["declaration"] === undefined ? undefined : parseXmlDeclaration(row["declaration"], `${at}.declaration`),
    prolog: stdioXml10BaseSnapshotGuardArray(row["prolog"], `${at}.prolog`).map((item, index) => parseXmlNode(item, `${at}.prolog[${index}]`)),
  };
}
