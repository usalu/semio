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
export type XmlQuote = 'double' | 'single';

export interface XmlDeclaration {
  version: string;
  encoding?: string;
  standalone?: boolean;
  /** 🗣️ Delimiter the pseudo-attributes are written with (XML 1.0 §2.8); absent = `double`. */
  quote?: XmlQuote;
}

export type XmlExternalId =
  | { kind: 'system'; systemId: string }
  | { kind: 'public'; publicId: string; systemId: string };
export type XmlDtdDeclaration = { kind: 'entity'; parameter: boolean; name: string; value: string };
export interface XmlDoctype { prologPosition?: bigint; name: string; externalId?: XmlExternalId; declarations: XmlDtdDeclaration[]; }

/** 📰 Well-formed XML document root. */
export interface XmlDocument {
  root?: XmlNode;
  doctype?: XmlDoctype;
  declaration?: XmlDeclaration;
  prolog: XmlNode[];
  epilog: XmlNode[];
}

/** 🧱️ One attribute in the retained flat XML owner. */
export interface RetainedXmlAttribute { name: string; value: string }

/** 🧩️ One node value in the retained flat XML owner. */
export type RetainedXmlNodeKind =
  | { kind: 'element'; name: string; first_attribute: number; attribute_count: number; first_child: number | null }
  | { kind: 'text'; text: string }
  | { kind: 'cData'; text: string }
  | { kind: 'comment'; text: string }
  | { kind: 'processingInstruction'; target: string; data: string };

/** 🔗️ One retained node and its forward sibling link. */
export interface RetainedXmlNode { nextSibling: number | null; value: RetainedXmlNodeKind }

/** 🪪️ Retained external identifier. */
export type RetainedXmlExternalId =
  | { kind: 'system'; systemId: string }
  | { kind: 'public'; publicId: string; systemId: string };

/** 📜️ Retained DTD declaration. */
export type RetainedXmlDtdDeclaration = { kind: 'entity'; parameter: boolean; name: string; value: string };

/** 🏷️ Retained document type declaration. */
export interface RetainedXmlDoctype { prologPosition: string; name: string; externalId: RetainedXmlExternalId | null; declarations: RetainedXmlDtdDeclaration[] }

/** 🏳️ Retained XML declaration. */
export interface RetainedXmlDeclaration { version: string; encoding: string | null; standalone: boolean | null; quote: XmlQuote }

/** 🗂️ Flat retained XML document authority. */
export interface RetainedXmlDocument {
  nodes: RetainedXmlNode[];
  attributes: RetainedXmlAttribute[];
  prolog: number[];
  epilog: number[];
  root: number | null;
  doctype: RetainedXmlDoctype | null;
  declaration: RetainedXmlDeclaration | null;
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
  for (const key of Object.keys(row)) if (key !== "name" && key !== "value") stdioXml10BaseSnapshotGuardReject(`${at}.${key}`, "unknown XML attribute field");
  return {
    name: stdioXml10BaseSnapshotGuardString(row["name"], `${at}.name`),
    value: stdioXml10BaseSnapshotGuardString(row["value"], `${at}.value`),
  };
}

export function parseXmlDeclaration(value: unknown, at = "$"): XmlDeclaration {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  const version = stdioXml10BaseSnapshotGuardString(row["version"], `${at}.version`);
  const encoding = row["encoding"] === undefined ? undefined : stdioXml10BaseSnapshotGuardString(row["encoding"], `${at}.encoding`);
  const quote = row["quote"] === undefined ? undefined : stdioXml10BaseSnapshotGuardMember<XmlQuote>(row["quote"], `${at}.quote`, ["double", "single"]);
  return {
    version,
    encoding,
    standalone: row["standalone"] === undefined ? undefined : stdioXml10BaseSnapshotGuardBoolean(row["standalone"], `${at}.standalone`),
    quote,
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

export function parseXmlExternalId(value: unknown, at = "$"): XmlExternalId {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  const kind = stdioXml10BaseSnapshotGuardMember(row["kind"], `${at}.kind`, ["system", "public"] as const);
  if (kind === "system") return { kind, systemId: stdioXml10BaseSnapshotGuardString(row["systemId"], `${at}.systemId`) };
  return {
    kind,
    publicId: stdioXml10BaseSnapshotGuardString(row["publicId"], `${at}.publicId`),
    systemId: stdioXml10BaseSnapshotGuardString(row["systemId"], `${at}.systemId`),
  };
}

export function parseXmlNode(value: unknown, at = "$"): XmlNode {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  const kind = stdioXml10BaseSnapshotGuardMember(row["kind"], `${at}.kind`, ["element", "text", "cData", "comment", "processingInstruction"] as const);
  if (kind === "element") return {
    kind,
    name: stdioXml10BaseSnapshotGuardString(row["name"], `${at}.name`),
    attrs: stdioXml10BaseSnapshotGuardArray(row["attrs"], `${at}.attrs`).map((item, index) => parseXmlAttr(item, `${at}.attrs[${index}]`)),
    children: stdioXml10BaseSnapshotGuardArray(row["children"], `${at}.children`).map((item, index) => parseXmlNode(item, `${at}.children[${index}]`)),
  };
  if (kind === "processingInstruction") return {
    kind,
    target: stdioXml10BaseSnapshotGuardString(row["target"], `${at}.target`),
    data: stdioXml10BaseSnapshotGuardString(row["data"], `${at}.data`),
  };
  return { kind, text: stdioXml10BaseSnapshotGuardString(row["text"], `${at}.text`) };
}

export function parseXmlDoctype(value: unknown, at = "$"): XmlDoctype {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  return {
    prologPosition: row["prologPosition"] === undefined ? 0n : xmlOwnedPosition(row["prologPosition"], `${at}.prologPosition`),
    name: stdioXml10BaseSnapshotGuardString(row["name"], `${at}.name`),
    externalId: row["externalId"] === undefined ? undefined : parseXmlExternalId(row["externalId"], `${at}.externalId`),
    declarations: row["declarations"] === undefined ? [] : stdioXml10BaseSnapshotGuardArray(row["declarations"], `${at}.declarations`).map((item, index) => parseXmlDtdDeclaration(item, `${at}.declarations[${index}]`)),
  };
}

export function parseXmlDocument(value: unknown, at = "$"): XmlDocument {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  const document = {
    root: row["root"] === undefined ? undefined : parseXmlNode(row["root"], `${at}.root`),
    doctype: row["doctype"] === undefined ? undefined : parseXmlDoctype(row["doctype"], `${at}.doctype`),
    declaration: row["declaration"] === undefined ? undefined : parseXmlDeclaration(row["declaration"], `${at}.declaration`),
    prolog: row["prolog"] === undefined ? [] : stdioXml10BaseSnapshotGuardArray(row["prolog"], `${at}.prolog`).map((item, index) => parseXmlNode(item, `${at}.prolog[${index}]`)),
    epilog: row["epilog"] === undefined ? [] : stdioXml10BaseSnapshotGuardArray(row["epilog"], `${at}.epilog`).map((item, index) => parseXmlNode(item, `${at}.epilog[${index}]`)),
  };
  return document;
}

const retainedExact = (row: Readonly<Record<string, unknown>>, fields: readonly string[], at: string): void => {
  for (const key of Object.keys(row)) if (!fields.includes(key)) stdioXml10BaseSnapshotGuardReject(`${at}.${key}`, 'unknown retained XML field');
};
const retainedIndex = (value: unknown, at: string): number => stdioXml10BaseSnapshotGuardInteger(value, at, { minimum: 0 });
const retainedNullableIndex = (value: unknown, at: string): number | null => value === null ? null : retainedIndex(value, at);

/** 🚪️ Parses one retained XML node value. */
export function parseRetainedXmlNodeKind(value: unknown, at = '$'): RetainedXmlNodeKind {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  const kind = stdioXml10BaseSnapshotGuardMember(row.kind, `${at}.kind`, ['element', 'text', 'cData', 'comment', 'processingInstruction'] as const);
  if (kind === 'element') {
    retainedExact(row, ['kind', 'name', 'first_attribute', 'attribute_count', 'first_child'], at);
    return { kind, name: stdioXml10BaseSnapshotGuardString(row.name, `${at}.name`), first_attribute: retainedIndex(row.first_attribute, `${at}.first_attribute`), attribute_count: retainedIndex(row.attribute_count, `${at}.attribute_count`), first_child: retainedNullableIndex(row.first_child, `${at}.first_child`) };
  }
  if (kind === 'processingInstruction') {
    retainedExact(row, ['kind', 'target', 'data'], at);
    return { kind, target: stdioXml10BaseSnapshotGuardString(row.target, `${at}.target`), data: stdioXml10BaseSnapshotGuardString(row.data, `${at}.data`) };
  }
  retainedExact(row, ['kind', 'text'], at);
  return { kind, text: stdioXml10BaseSnapshotGuardString(row.text, `${at}.text`) };
}

/** 🚪️ Parses one retained XML node. */
export function parseRetainedXmlNode(value: unknown, at = '$'): RetainedXmlNode {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  retainedExact(row, ['nextSibling', 'value'], at);
  return { nextSibling: retainedNullableIndex(row.nextSibling, `${at}.nextSibling`), value: parseRetainedXmlNodeKind(row.value, `${at}.value`) };
}

/** 🚪️ Parses one retained XML external identifier. */
export function parseRetainedXmlExternalId(value: unknown, at = '$'): RetainedXmlExternalId {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  const kind = stdioXml10BaseSnapshotGuardMember(row.kind, `${at}.kind`, ['system', 'public'] as const);
  if (kind === 'system') {
    retainedExact(row, ['kind', 'systemId'], at);
    return { kind, systemId: stdioXml10BaseSnapshotGuardString(row.systemId, `${at}.systemId`) };
  }
  retainedExact(row, ['kind', 'publicId', 'systemId'], at);
  return { kind, publicId: stdioXml10BaseSnapshotGuardString(row.publicId, `${at}.publicId`), systemId: stdioXml10BaseSnapshotGuardString(row.systemId, `${at}.systemId`) };
}

/** 🚪️ Parses one retained XML DTD declaration. */
export function parseRetainedXmlDtdDeclaration(value: unknown, at = '$'): RetainedXmlDtdDeclaration {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  retainedExact(row, ['kind', 'parameter', 'name', 'value'], at);
  return { kind: stdioXml10BaseSnapshotGuardConstant(row.kind, `${at}.kind`, 'entity'), parameter: stdioXml10BaseSnapshotGuardBoolean(row.parameter, `${at}.parameter`), name: stdioXml10BaseSnapshotGuardString(row.name, `${at}.name`), value: stdioXml10BaseSnapshotGuardString(row.value, `${at}.value`) };
}

/** 🚪️ Parses one retained XML document type. */
export function parseRetainedXmlDoctype(value: unknown, at = '$'): RetainedXmlDoctype {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  retainedExact(row, ['prologPosition', 'name', 'externalId', 'declarations'], at);
  const prologPosition = stdioXml10BaseSnapshotGuardString(row.prologPosition, `${at}.prologPosition`);
  parseXmlPosition(prologPosition, `${at}.prologPosition`);
  return {
    prologPosition,
    name: stdioXml10BaseSnapshotGuardString(row.name, `${at}.name`),
    externalId: row.externalId === null ? null : parseRetainedXmlExternalId(row.externalId, `${at}.externalId`),
    declarations: stdioXml10BaseSnapshotGuardArray(row.declarations, `${at}.declarations`).map((item, index) => parseRetainedXmlDtdDeclaration(item, `${at}.declarations[${index}]`)),
  };
}

/** 🚪️ Parses one retained XML declaration. */
export function parseRetainedXmlDeclaration(value: unknown, at = '$'): RetainedXmlDeclaration {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  retainedExact(row, ['version', 'encoding', 'standalone', 'quote'], at);
  return {
    version: stdioXml10BaseSnapshotGuardString(row.version, `${at}.version`),
    encoding: row.encoding === null ? null : stdioXml10BaseSnapshotGuardString(row.encoding, `${at}.encoding`),
    standalone: row.standalone === null ? null : stdioXml10BaseSnapshotGuardBoolean(row.standalone, `${at}.standalone`),
    quote: stdioXml10BaseSnapshotGuardMember(row.quote, `${at}.quote`, ['double', 'single'] as const),
  };
}

const validateRetainedXmlDocument = (document: RetainedXmlDocument, at: string): void => {
  const incoming = document.nodes.map(() => 0);
  const own = (ordinal: number, ownerAt: string): void => {
    if (ordinal >= incoming.length) stdioXml10BaseSnapshotGuardReject(ownerAt, 'retained XML node ordinal is out of range');
    incoming[ordinal] = (incoming[ordinal] ?? 0) + 1;
  };
  document.prolog.forEach((ordinal, index) => own(ordinal, `${at}.prolog[${index}]`));
  if (document.root !== null) own(document.root, `${at}.root`);
  document.epilog.forEach((ordinal, index) => own(ordinal, `${at}.epilog[${index}]`));
  document.nodes.forEach((node, ordinal) => {
    if (node.nextSibling !== null) {
      if (node.nextSibling <= ordinal) stdioXml10BaseSnapshotGuardReject(`${at}.nodes[${ordinal}].nextSibling`, 'retained XML sibling ordinal is not forward');
      own(node.nextSibling, `${at}.nodes[${ordinal}].nextSibling`);
    }
    if (node.value.kind === 'element') {
      const end = node.value.first_attribute + node.value.attribute_count;
      if (!Number.isSafeInteger(end) || end > document.attributes.length) stdioXml10BaseSnapshotGuardReject(`${at}.nodes[${ordinal}].value`, 'retained XML attribute range is out of bounds');
      if (node.value.first_child !== null) {
        if (node.value.first_child >= ordinal) stdioXml10BaseSnapshotGuardReject(`${at}.nodes[${ordinal}].value.first_child`, 'retained XML child ordinal is not earlier than its parent');
        own(node.value.first_child, `${at}.nodes[${ordinal}].value.first_child`);
      }
    }
  });
  const unowned = incoming.findIndex((count) => count !== 1);
  if (unowned >= 0) stdioXml10BaseSnapshotGuardReject(`${at}.nodes[${unowned}]`, 'retained XML node must have exactly one owner');
};

/** 🚪️ Parses and validates the flat retained XML authority. */
export function parseRetainedXmlDocument(value: unknown, at = '$'): RetainedXmlDocument {
  const row = stdioXml10BaseSnapshotGuardObject(value, at);
  retainedExact(row, ['nodes', 'attributes', 'prolog', 'epilog', 'root', 'doctype', 'declaration'], at);
  const document: RetainedXmlDocument = {
    nodes: stdioXml10BaseSnapshotGuardArray(row.nodes, `${at}.nodes`).map((item, index) => parseRetainedXmlNode(item, `${at}.nodes[${index}]`)),
    attributes: stdioXml10BaseSnapshotGuardArray(row.attributes, `${at}.attributes`).map((item, index) => parseXmlAttr(item, `${at}.attributes[${index}]`)),
    prolog: stdioXml10BaseSnapshotGuardArray(row.prolog, `${at}.prolog`).map((item, index) => retainedIndex(item, `${at}.prolog[${index}]`)),
    epilog: stdioXml10BaseSnapshotGuardArray(row.epilog, `${at}.epilog`).map((item, index) => retainedIndex(item, `${at}.epilog[${index}]`)),
    root: retainedNullableIndex(row.root, `${at}.root`),
    doctype: row.doctype === null ? null : parseRetainedXmlDoctype(row.doctype, `${at}.doctype`),
    declaration: row.declaration === null ? null : parseRetainedXmlDeclaration(row.declaration, `${at}.declaration`),
  };
  validateRetainedXmlDocument(document, at);
  return document;
}

/** 📥️ Flattens one ordinary XML tree into the retained ownership authority. */
export function retainXmlDocument(source: XmlDocument): RetainedXmlDocument {
  const nodes: RetainedXmlNode[] = [], attributes: RetainedXmlAttribute[] = [];
  type Frame = { source: XmlNode; firstAttribute: number; attributeCount: number; childPosition: number; firstChild: number | null; lastChild: number | null };
  const frame = (node: XmlNode): Frame => {
    const firstAttribute = attributes.length, attrs = node.kind === 'element' ? node.attrs : [];
    attributes.push(...attrs.map(({ name, value }) => ({ name, value })));
    return { source: node, firstAttribute, attributeCount: attrs.length, childPosition: 0, firstChild: null, lastChild: null };
  };
  const append = (node: XmlNode): number => {
    const stack: Frame[] = [frame(node)];
    for (;;) {
      const current = stack[stack.length - 1]!;
      if (current.source.kind === 'element' && current.childPosition < current.source.children.length) {
        stack.push(frame(current.source.children[current.childPosition++]!));
        continue;
      }
      const complete = stack.pop()!;
      const value: RetainedXmlNodeKind = complete.source.kind === 'element'
        ? { kind: 'element', name: complete.source.name, first_attribute: complete.firstAttribute, attribute_count: complete.attributeCount, first_child: complete.firstChild }
        : complete.source.kind === 'processingInstruction'
          ? { kind: 'processingInstruction', target: complete.source.target, data: complete.source.data }
          : { kind: complete.source.kind, text: complete.source.text };
      const ordinal = nodes.length;
      nodes.push({ nextSibling: null, value });
      const parent = stack[stack.length - 1];
      if (parent === undefined) return ordinal;
      if (parent.lastChild === null) parent.firstChild = ordinal;
      else nodes[parent.lastChild]!.nextSibling = ordinal;
      parent.lastChild = ordinal;
    }
  };
  const document: RetainedXmlDocument = {
    nodes,
    attributes,
    prolog: source.prolog.map(append),
    epilog: [],
    root: source.root === undefined ? null : append(source.root),
    doctype: source.doctype === undefined ? null : {
      prologPosition: source.doctype.prologPosition?.toString() ?? '0',
      name: source.doctype.name,
      externalId: source.doctype.externalId ?? null,
      declarations: source.doctype.declarations,
    },
    declaration: source.declaration === undefined ? null : {
      version: source.declaration.version,
      encoding: source.declaration.encoding ?? null,
      standalone: source.declaration.standalone ?? null,
      quote: source.declaration.quote ?? 'double',
    },
  };
  document.epilog = source.epilog.map(append);
  validateRetainedXmlDocument(document, '$');
  return document;
}

/** 📤️ Materializes one retained XML owner into the ordinary tree projection. */
export function materializeRetainedXmlDocument(source: RetainedXmlDocument): XmlDocument {
  validateRetainedXmlDocument(source, '$');
  const built: Array<XmlNode | undefined> = source.nodes.map(() => undefined);
  source.nodes.forEach((node, ordinal) => {
    if (node.value.kind === 'element') {
      const attrs = source.attributes.slice(node.value.first_attribute, node.value.first_attribute + node.value.attribute_count).map(({ name, value }) => ({ name, value }));
      const children: XmlNode[] = [];
      let child = node.value.first_child;
      while (child !== null) {
        const value = built[child];
        if (value === undefined) throw new stdioXml10BaseSnapshotGuardRefusal(`$.nodes[${child}]`, 'retained XML child materialization order is invalid');
        children.push(value);
        built[child] = undefined;
        child = source.nodes[child]!.nextSibling;
      }
      built[ordinal] = { kind: 'element', name: node.value.name, attrs, children };
    } else built[ordinal] = { ...node.value };
  });
  const take = (ordinal: number, at: string): XmlNode => {
    const node = built[ordinal];
    if (node === undefined) throw new stdioXml10BaseSnapshotGuardRefusal(at, 'retained XML node was already consumed');
    built[ordinal] = undefined;
    return node;
  };
  const document: XmlDocument = {
    root: source.root === null ? undefined : take(source.root, '$.root'),
    doctype: source.doctype === null ? undefined : {
      prologPosition: parseXmlPosition(source.doctype.prologPosition, '$.doctype.prologPosition'),
      name: source.doctype.name,
      externalId: source.doctype.externalId ?? undefined,
      declarations: source.doctype.declarations,
    },
    declaration: source.declaration === null ? undefined : {
      version: source.declaration.version,
      encoding: source.declaration.encoding ?? undefined,
      standalone: source.declaration.standalone ?? undefined,
      quote: source.declaration.quote,
    },
    prolog: source.prolog.map((ordinal, index) => take(ordinal, `$.prolog[${index}]`)),
    epilog: source.epilog.map((ordinal, index) => take(ordinal, `$.epilog[${index}]`)),
  };
  if (built.some((node) => node !== undefined)) stdioXml10BaseSnapshotGuardReject('$.nodes', 'retained XML materialization left an unowned node');
  return document;
}

/** 🧭️ Exact unsigned64 scalar at the canonical JSON boundary. */
export function parseXmlPosition(value:unknown,at="$" ):bigint{if(typeof value!=="string"||!/^(0|[1-9][0-9]*)$/.test(value)||value.length>20)throw new Error(`${at}: expected canonical unsigned64 decimal text`);const position=BigInt(value);if(position>18446744073709551615n)throw new Error(`${at}: position exceeds unsigned64`);return position;}
/** 📜️ Ordinary XML wire boundary policy, separate from owned logical snapshot admission. */
export function validateXmlDocumentWireBoundary(document:XmlDocument,at="$" ):void{
 if(document.declaration)validateXmlDeclarationWire(document.declaration,`${at}.declaration`);
  for (const [boundary, nodes] of [["prolog", document.prolog], ["epilog", document.epilog]] as const) {
    if (nodes.some((node) => node.kind !== "comment" && node.kind !== "processingInstruction")) stdioXml10BaseSnapshotGuardReject(`${at}.${boundary}`, "boundary contains a non-miscellaneous node");
  }
  if ((document.doctype?.prologPosition ?? 0n) > BigInt(document.prolog.length)) stdioXml10BaseSnapshotGuardReject(`${at}.doctype.prologPosition`, "position exceeds prolog length");
}

/** 📜️ Declaration policy for an ordinary UTF-8 XML wire. */
export function validateXmlDeclarationWire(declaration:XmlDeclaration,at="$" ):void{
 const {version,encoding,quote}=declaration;
  const delimiter = quote === "single" ? "'" : '"';
  if (version.includes(delimiter)) stdioXml10BaseSnapshotGuardReject(`${at}.version`, "value contains its declaration quote delimiter");
  if (!/^1\.[0-9]+$/u.test(version)) stdioXml10BaseSnapshotGuardReject(`${at}.version`, "value is not a valid XML VersionNum");
  if (encoding !== undefined) {
    if (encoding.includes(delimiter)) stdioXml10BaseSnapshotGuardReject(`${at}.encoding`, "value contains its declaration quote delimiter");
    if (!/^[A-Za-z][A-Za-z0-9._-]*$/u.test(encoding)) stdioXml10BaseSnapshotGuardReject(`${at}.encoding`, "value is not a valid XML EncName");
    if (encoding.toLowerCase() !== "utf-8") stdioXml10BaseSnapshotGuardReject(`${at}.encoding`, "value conflicts with the UTF-8 transport");
  }
}

function xmlOwnedPosition(value:unknown,at:string):bigint{if(typeof value!=="bigint"||value<0n||value>18446744073709551615n)throw new Error(`${at}: expected owned unsigned64`);return value;}
