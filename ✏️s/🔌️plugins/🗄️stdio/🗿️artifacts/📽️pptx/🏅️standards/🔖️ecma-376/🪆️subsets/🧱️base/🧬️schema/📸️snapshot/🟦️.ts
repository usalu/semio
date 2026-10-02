import type{XmlDocument,XmlNode,XmlAttr,XmlDoctype,XmlDeclaration,XmlExternalId,XmlDtdDeclaration,XmlQuote}from"../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import{parseXmlDocument,parseXmlNode}from"../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export{parseXmlDocument,parseXmlNode}from"../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export type{XmlDocument,XmlNode,XmlAttr,XmlDoctype,XmlDeclaration,XmlExternalId,XmlDtdDeclaration,XmlQuote}from"../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import type{OpcPackage,OpcPart,OpcRelationship}from"../../../../../../../🎒️zip/📦️opc/🟦️.ts";
export type{OpcPackage,OpcPart,OpcRelationship}from"../../../../../../../🎒️zip/📦️opc/🟦️.ts";
/** 📄️ One authoritative XML-bearing OPC part. */
export interface PptxXmlPart { path: string; contentType: string; document: XmlDocument }
/** 📐️ Shape position and size in EMUs. */
export interface PptxTransform { x: bigint; y: bigint; cx: bigint; cy: bigint }
/** ✍️ One PresentationML text run. */
export interface PptxRun { text: string; bold: boolean; italic: boolean; fontSize?: number }
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

export function parsePptxXmlPart(value: unknown, at: string): PptxXmlPart { const row = pptxGuardObject(value, at); return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), document: parseXmlDocument(row.document, `${at}.document`) }; }
/** 📐️ Parses an exact signed64 decimal JSON boundary. */
export function parsePptxCoordinate(value:unknown,at:string):bigint{if(typeof value!=="string"||!/^(-?[1-9][0-9]*|0)$/u.test(value)||value.length>20)reject(at,"value is not canonical signed64 decimal text");const number=BigInt(value as string);if(number<-(1n<<63n)||number>=(1n<<63n))reject(at,"value exceeds signed64");return number;}
export function parsePptxTransform(value: unknown, at: string): PptxTransform { const row = pptxGuardObject(value, at); return { x: parsePptxCoordinate(row.x, `${at}.x`), y: parsePptxCoordinate(row.y, `${at}.y`), cx: parsePptxCoordinate(row.cx, `${at}.cx`), cy: parsePptxCoordinate(row.cy, `${at}.cy`) }; }
export function parsePptxRun(value: unknown, at: string): PptxRun { const row = pptxGuardObject(value, at); return { text: text(row.text, `${at}.text`), bold: boolean(row.bold, `${at}.bold`), italic: boolean(row.italic, `${at}.italic`), fontSize: row.fontSize === undefined || row.fontSize === null ? undefined : integer(row.fontSize, `${at}.fontSize`, 0, 4294967295) }; }
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
  const relationships: Record<string, OpcRelationship[]> = Object.create(null);
  for (const [owner, entries] of Object.entries(pptxGuardObject(row.relationships, `${at}.relationships`))) relationships[owner] = array(entries, `${at}.relationships.${owner}`).map((item, index) => parseOpcRelationship(item, `${at}.relationships.${owner}[${index}]`));
  return { parts: array(row.parts, `${at}.parts`).map((item, index) => parseOpcPart(item, `${at}.parts[${index}]`)), contentTypes: { defaults: array(contentTypes.defaults, `${at}.contentTypes.defaults`).map((item, index) => pair(item, `${at}.contentTypes.defaults[${index}]`)), overrides: array(contentTypes.overrides, `${at}.contentTypes.overrides`).map((item, index) => pair(item, `${at}.contentTypes.overrides[${index}]`)) }, relationships, comment: text(row.comment, `${at}.comment`) };
}
/** 🚪️ Parses the complete PPTX snapshot through one typed authority. */
export function parsePptxSnapshot(value: unknown, at = '$'): PptxSnapshot {
  const row = pptxGuardObject(value, at), presentation = pptxGuardObject(row.presentation, `${at}.presentation`);
  return { schema: text(row.schema, `${at}.schema`), opc: parseOpcPackage(row.opc, `${at}.opc`), xmlParts: array(row.xmlParts, `${at}.xmlParts`).map((item, index) => parsePptxXmlPart(item, `${at}.xmlParts[${index}]`)), presentation: { slides: array(presentation.slides, `${at}.presentation.slides`).map((item, index) => parsePptxSlide(item, `${at}.presentation.slides[${index}]`)) } };
}
