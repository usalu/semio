/** 📦️ One losslessly retained OPC package part. */
export interface OpcPart { path: string; contentType: string; bytes: number[] }
/** 🔗️ One OPC relationship. */
export interface OpcRelationship { id: string; relType: string; target: string; targetMode: 'internal' | 'external' }
/** 📦️ The complete OPC container, including the ZIP comment. */
export interface OpcPackage {
  parts: OpcPart[];
  contentTypes: { defaults: [string, string][]; overrides: [string, string][] };
  relationships: Record<string, OpcRelationship[]>;
  comment: string;
}
/** 🏷️ One XML attribute. */
export interface XmlAttr { name: string; value: string }
/** 🌳 One logical XML node. */
export type XmlNode =
  | { kind: 'element'; name: string; attrs: XmlAttr[]; children: XmlNode[] }
  | { kind: 'text' | 'cData' | 'comment'; text: string }
  | { kind: 'processingInstruction'; target: string; data: string };
/** 🪪 One XML doctype external identifier. */
export type XmlExternalId =
  | { kind: 'system'; systemId: string }
  | { kind: 'public'; publicId: string; systemId: string };
/** 📜️ One retained DTD declaration. */
export type XmlDtdDeclaration = { kind: 'entity'; parameter: boolean; name: string; value: string };
/** 🏷️ One XML doctype. */
export interface XmlDoctype { prologPosition?: number; name: string; externalId?: XmlExternalId | null; declarations: XmlDtdDeclaration[] }
/** 🏳️ One XML declaration. */
export type XmlQuote = 'double' | 'single';
export interface XmlDeclaration { version: string; encoding?: string | null; standalone?: boolean | null; quote?: XmlQuote }
/** 📰 One complete logical XML document. */
export interface XmlDocument { root?: XmlNode | null; doctype?: XmlDoctype | null; declaration?: XmlDeclaration | null; prolog: XmlNode[]; epilog: XmlNode[] }
/** 📄️ One authoritative XML-bearing OPC part. */
export interface PptxXmlPart { path: string; contentType: string; document: XmlDocument }
/** 📐️ Shape position and size in EMUs. */
export interface PptxTransform { x: number; y: number; cx: number; cy: number }
/** ✍️ One PresentationML text run. */
export interface PptxRun { text: string; bold: boolean; italic: boolean; fontSize?: number | null }
/** 📄️ One PresentationML paragraph. */
export interface PptxParagraph { runs: PptxRun[] }
/** 🖼️ One typed or losslessly retained slide shape. */
export type PptxShape =
  | { shapeKind: 'textBox'; textFrame: PptxParagraph[]; position: PptxTransform }
  | { shapeKind: 'picture'; blipRelId: string; position: PptxTransform }
  | { shapeKind: 'placeholder'; kind: string; textFrame: PptxParagraph[]; position: PptxTransform }
  | { shapeKind: 'other'; node: XmlNode };
/** 🎞️ One ordered slide. */
export interface PptxSlide { shapes: PptxShape[] }
/** 📽️ The typed semantic presentation view. */
export interface PptxPresentation { slides: PptxSlide[] }
/** 🧬️ Complete lossless PPTX snapshot. */
export interface PptxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: PptxXmlPart[];
  /** @state artifact */ presentation: PptxPresentation;
}

/** 🚪️ A precise position and reason for refusing a malformed PPTX snapshot. */
export class stdioPptxEcma376BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}
function reject(at: string, why: string): never { throw new stdioPptxEcma376BaseSnapshotGuardRefusal(at, why); }
export const pptxGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, 'value is not an object');
const array = (value: unknown, at: string): readonly unknown[] => Array.isArray(value) ? value : reject(at, 'value is not an array');
const text = (value: unknown, at: string): string => typeof value === 'string' ? value : reject(at, 'value is not a string');
const boolean = (value: unknown, at: string): boolean => typeof value === 'boolean' ? value : reject(at, 'value is not a boolean');
const integer = (value: unknown, at: string, minimum = Number.MIN_SAFE_INTEGER, maximum = Number.MAX_SAFE_INTEGER): number => Number.isSafeInteger(value) && (value as number) >= minimum && (value as number) <= maximum ? value as number : reject(at, 'value is not an integer');
const nullableText = (value: unknown, at: string): string | null => value === null ? null : text(value, at);
const nullableBoolean = (value: unknown, at: string): boolean | null => value === null ? null : boolean(value, at);

export function parseXmlNode(value: unknown, at: string): XmlNode {
  const row = pptxGuardObject(value, at), kind = text(row.kind, `${at}.kind`);
  if (kind === 'element') return { kind, name: text(row.name, `${at}.name`), attrs: array(row.attrs, `${at}.attrs`).map((item, index) => { const attrAt = `${at}.attrs[${index}]`, attr = pptxGuardObject(item, attrAt); if (attr.prologPosition !== undefined) reject(`${attrAt}.prologPosition`, 'field belongs to XML doctype'); return { name: text(attr.name, `${attrAt}.name`), value: text(attr.value, `${attrAt}.value`) }; }), children: array(row.children, `${at}.children`).map((item, index) => parseXmlNode(item, `${at}.children[${index}]`)) };
  if (kind === 'text' || kind === 'cData' || kind === 'comment') return { kind, text: text(row.text, `${at}.text`) };
  if (kind === 'processingInstruction') return { kind, target: text(row.target, `${at}.target`), data: text(row.data, `${at}.data`) };
  return reject(`${at}.kind`, `unknown XML node kind ${kind}`);
}
function parseXmlExternalId(value: unknown, at: string): XmlExternalId {
  const row = pptxGuardObject(value, at), kind = text(row.kind, `${at}.kind`);
  if (kind === 'system') return { kind, systemId: text(row.systemId, `${at}.systemId`) };
  if (kind === 'public') return { kind, publicId: text(row.publicId, `${at}.publicId`), systemId: text(row.systemId, `${at}.systemId`) };
  return reject(`${at}.kind`, `unknown XML external identifier kind ${kind}`);
}
function parseXmlDoctype(value: unknown, at: string): XmlDoctype {
  const row = pptxGuardObject(value, at);
  return {
    prologPosition: row.prologPosition === undefined ? 0 : integer(row.prologPosition, `${at}.prologPosition`, 0),
    name: text(row.name, `${at}.name`),
    externalId: row.externalId === undefined ? undefined : row.externalId === null ? null : parseXmlExternalId(row.externalId, `${at}.externalId`),
    declarations: row.declarations === undefined ? [] : array(row.declarations, `${at}.declarations`).map((item, index) => { const declarationAt = `${at}.declarations[${index}]`, declaration = pptxGuardObject(item, declarationAt), kind = text(declaration.kind, `${declarationAt}.kind`); if (kind !== 'entity') reject(`${declarationAt}.kind`, `unknown XML DTD declaration kind ${kind}`); return { kind, parameter: boolean(declaration.parameter, `${declarationAt}.parameter`), name: text(declaration.name, `${declarationAt}.name`), value: text(declaration.value, `${declarationAt}.value`) }; }),
  };
}
function parseXmlDeclaration(value: unknown, at: string): XmlDeclaration {
  const row = pptxGuardObject(value, at), quote = row.quote === undefined ? undefined : text(row.quote, `${at}.quote`);
  if (quote !== undefined && quote !== 'double' && quote !== 'single') reject(`${at}.quote`, 'unknown XML declaration quote');
  const version = text(row.version, `${at}.version`), encoding = row.encoding === undefined ? undefined : nullableText(row.encoding, `${at}.encoding`), delimiter = quote === 'single' ? "'" : '"';
  if (version.includes(delimiter) || !/^1\.[0-9]+$/u.test(version)) reject(`${at}.version`, 'invalid XML declaration version');
  if (encoding !== undefined && encoding !== null) {
    if (encoding.includes(delimiter) || !/^[A-Za-z][A-Za-z0-9._-]*$/u.test(encoding)) reject(`${at}.encoding`, 'invalid XML declaration encoding');
    if (encoding.toLowerCase() !== 'utf-8') reject(`${at}.encoding`, 'encoding conflicts with the UTF-8 transport');
  }
  return { version, encoding, standalone: row.standalone === undefined ? undefined : nullableBoolean(row.standalone, `${at}.standalone`), quote };
}
export function parseXmlDocument(value: unknown, at: string): XmlDocument {
  const row = pptxGuardObject(value, at);
  const document: XmlDocument = {
    root: row.root === undefined ? undefined : row.root === null ? null : parseXmlNode(row.root, `${at}.root`),
    doctype: row.doctype === undefined ? undefined : row.doctype === null ? null : parseXmlDoctype(row.doctype, `${at}.doctype`),
    declaration: row.declaration === undefined ? undefined : row.declaration === null ? null : parseXmlDeclaration(row.declaration, `${at}.declaration`),
    prolog: row.prolog === undefined ? [] : array(row.prolog, `${at}.prolog`).map((item, index) => parseXmlNode(item, `${at}.prolog[${index}]`)),
    epilog: row.epilog === undefined ? [] : array(row.epilog, `${at}.epilog`).map((item, index) => parseXmlNode(item, `${at}.epilog[${index}]`)),
  };
  for (const [boundary, nodes] of [['prolog', document.prolog], ['epilog', document.epilog]] as const) if (nodes.some((node) => node.kind !== 'comment' && node.kind !== 'processingInstruction')) reject(`${at}.${boundary}`, 'boundary contains a non-miscellaneous node');
  if ((document.doctype?.prologPosition ?? 0) > document.prolog.length) reject(`${at}.doctype.prologPosition`, 'position exceeds prolog length');
  return document;
}
export function parsePptxXmlPart(value: unknown, at: string): PptxXmlPart { const row = pptxGuardObject(value, at); return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), document: parseXmlDocument(row.document, `${at}.document`) }; }
export function parsePptxTransform(value: unknown, at: string): PptxTransform { const row = pptxGuardObject(value, at); return { x: integer(row.x, `${at}.x`), y: integer(row.y, `${at}.y`), cx: integer(row.cx, `${at}.cx`), cy: integer(row.cy, `${at}.cy`) }; }
export function parsePptxRun(value: unknown, at: string): PptxRun { const row = pptxGuardObject(value, at); return { text: text(row.text, `${at}.text`), bold: boolean(row.bold, `${at}.bold`), italic: boolean(row.italic, `${at}.italic`), fontSize: row.fontSize === undefined ? undefined : row.fontSize === null ? null : integer(row.fontSize, `${at}.fontSize`, 0, 4294967295) }; }
export function parsePptxParagraph(value: unknown, at: string): PptxParagraph { const row = pptxGuardObject(value, at); return { runs: array(row.runs, `${at}.runs`).map((item, index) => parsePptxRun(item, `${at}.runs[${index}]`)) }; }
export function parsePptxShape(value: unknown, at: string): PptxShape {
  const row = pptxGuardObject(value, at), shapeKind = text(row.shapeKind, `${at}.shapeKind`);
  if (shapeKind === 'textBox') return { shapeKind, textFrame: array(row.textFrame, `${at}.textFrame`).map((item, index) => parsePptxParagraph(item, `${at}.textFrame[${index}]`)), position: parsePptxTransform(row.position, `${at}.position`) };
  if (shapeKind === 'picture') return { shapeKind, blipRelId: text(row.blipRelId, `${at}.blipRelId`), position: parsePptxTransform(row.position, `${at}.position`) };
  if (shapeKind === 'placeholder') return { shapeKind, kind: text(row.kind, `${at}.kind`), textFrame: array(row.textFrame, `${at}.textFrame`).map((item, index) => parsePptxParagraph(item, `${at}.textFrame[${index}]`)), position: parsePptxTransform(row.position, `${at}.position`) };
  if (shapeKind === 'other') return { shapeKind, node: parseXmlNode(row.node, `${at}.node`) };
  return reject(`${at}.shapeKind`, `unknown PPTX shape kind ${shapeKind}`);
}
export function parsePptxSlide(value: unknown, at: string): PptxSlide { const row = pptxGuardObject(value, at); return { shapes: array(row.shapes, `${at}.shapes`).map((item, index) => parsePptxShape(item, `${at}.shapes[${index}]`)) }; }
export function parseOpcPart(value: unknown, at: string): OpcPart { const row = pptxGuardObject(value, at); return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), bytes: array(row.bytes, `${at}.bytes`).map((item, index) => integer(item, `${at}.bytes[${index}]`, 0, 255)) }; }
export function parseOpcRelationship(value: unknown, at: string): OpcRelationship { const row = pptxGuardObject(value, at), targetMode = text(row.targetMode, `${at}.targetMode`); if (targetMode !== 'internal' && targetMode !== 'external') reject(`${at}.targetMode`, 'unknown OPC target mode'); return { id: text(row.id, `${at}.id`), relType: text(row.relType, `${at}.relType`), target: text(row.target, `${at}.target`), targetMode }; }
function parseOpcPackage(value: unknown, at: string): OpcPackage {
  const row = pptxGuardObject(value, at), contentTypes = pptxGuardObject(row.contentTypes, `${at}.contentTypes`), pair = (value: unknown, pairAt: string): [string, string] => { const values = array(value, pairAt); if (values.length !== 2) reject(pairAt, 'tuple does not contain exactly two items'); return [text(values[0], `${pairAt}[0]`), text(values[1], `${pairAt}[1]`)]; };
  const relationships: Record<string, OpcRelationship[]> = {};
  for (const [owner, entries] of Object.entries(pptxGuardObject(row.relationships, `${at}.relationships`))) relationships[owner] = array(entries, `${at}.relationships.${owner}`).map((item, index) => parseOpcRelationship(item, `${at}.relationships.${owner}[${index}]`));
  return { parts: array(row.parts, `${at}.parts`).map((item, index) => parseOpcPart(item, `${at}.parts[${index}]`)), contentTypes: { defaults: array(contentTypes.defaults, `${at}.contentTypes.defaults`).map((item, index) => pair(item, `${at}.contentTypes.defaults[${index}]`)), overrides: array(contentTypes.overrides, `${at}.contentTypes.overrides`).map((item, index) => pair(item, `${at}.contentTypes.overrides[${index}]`)) }, relationships, comment: text(row.comment, `${at}.comment`) };
}
/** 🚪️ Parses the complete PPTX snapshot through one typed authority. */
export function parsePptxSnapshot(value: unknown, at = '$'): PptxSnapshot {
  const row = pptxGuardObject(value, at), presentation = pptxGuardObject(row.presentation, `${at}.presentation`);
  return { schema: text(row.schema, `${at}.schema`), opc: parseOpcPackage(row.opc, `${at}.opc`), xmlParts: array(row.xmlParts, `${at}.xmlParts`).map((item, index) => parsePptxXmlPart(item, `${at}.xmlParts[${index}]`)), presentation: { slides: array(presentation.slides, `${at}.presentation.slides`).map((item, index) => parsePptxSlide(item, `${at}.presentation.slides[${index}]`)) } };
}
