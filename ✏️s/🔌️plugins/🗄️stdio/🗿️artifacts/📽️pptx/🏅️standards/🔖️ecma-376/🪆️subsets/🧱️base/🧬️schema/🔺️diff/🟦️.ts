import type {
  OpcPart,
  OpcRelationship,
  PptxParagraph,
  PptxRun,
  PptxShape,
  PptxSlide,
  PptxTransform,
  PptxXmlPart,
} from '../📸️snapshot/🟦️.ts';
import { parseOpcPart, parseOpcRelationship, parsePptxParagraph, parsePptxRun, parsePptxShape, parsePptxSlide, parsePptxTransform, parsePptxXmlPart } from '../📸️snapshot/🟦️.ts';

export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface IndexedTripleDiff<D, T> { removed?: number[]; modified?: IndexModified<D>[]; added?: IndexAdded<T>[] }
export interface NamedModified<K, D> { key: K; diff: D }
export interface NamedTripleDiff<K, D, T> { removed?: K[]; modified?: NamedModified<K, D>[]; added?: T[] }
export type PptxSlidesDiff = IndexedTripleDiff<PptxSlideDiff, PptxSlide>;
export type PptxShapesDiff = IndexedTripleDiff<PptxShapeDiff, PptxShape>;
export type PptxParagraphsDiff = IndexedTripleDiff<PptxParagraphDiff, PptxParagraph>;
export type PptxRunsDiff = IndexedTripleDiff<PptxRunDiff, PptxRun>;
export interface PptxSlideDiff { shapes?: PptxShapesDiff }
export type PptxShapeDiff =
  | { shapeKind: 'textBox'; textFrame?: PptxParagraphsDiff; position?: PptxTransform }
  | { shapeKind: 'picture'; blipRelId?: string; position?: PptxTransform }
  | { shapeKind: 'placeholder'; kind?: string; textFrame?: PptxParagraphsDiff; position?: PptxTransform }
  | { shapeKind: 'replace'; shape: PptxShape };
export interface PptxParagraphDiff { runs?: PptxRunsDiff }
export interface PptxRunDiff { text?: string; bold?: boolean; italic?: boolean; fontSize?: number | null }
export interface PptxPresentationDiff { slides?: PptxSlidesDiff }
export type PptxOpcCtEntriesDiff = NamedTripleDiff<string, string, [string, string]>;
export interface PptxOpcPartDiff { contentType?: string; bytes?: number[] }
export type PptxOpcPartsDiff = NamedTripleDiff<string, PptxOpcPartDiff, OpcPart>;
export interface PptxOpcRelDiff { relType?: string; target?: string; targetMode?: 'internal' | 'external' }
export type PptxOpcRelListDiff = NamedTripleDiff<string, PptxOpcRelDiff, OpcRelationship>;
export type PptxOpcRelationshipsDiff = NamedTripleDiff<string, PptxOpcRelListDiff, [string, OpcRelationship[]]>;
export interface PptxOpcContentTypesDiff { defaults?: PptxOpcCtEntriesDiff; overrides?: PptxOpcCtEntriesDiff }
export interface PptxOpcDiff { comment?: string; contentTypes?: PptxOpcContentTypesDiff; parts?: PptxOpcPartsDiff; relationships?: PptxOpcRelationshipsDiff }
export interface PptxDiff { opc?: PptxOpcDiff; presentation?: PptxPresentationDiff; xmlParts?: PptxXmlPart[] }

/** 🚪️ A precise position and reason for refusing a malformed PPTX diff. */
export class stdioPptxEcma376BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}
const reject = (at: string, why: string): never => { throw new stdioPptxEcma376BaseDiffGuardRefusal(at, why); };
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, 'value is not an object');
const array = (value: unknown, at: string): readonly unknown[] => Array.isArray(value) ? value : reject(at, 'value is not an array');
const text = (value: unknown, at: string): string => typeof value === 'string' ? value : reject(at, 'value is not a string');
const boolean = (value: unknown, at: string): boolean => typeof value === 'boolean' ? value : reject(at, 'value is not a boolean');
const integer = (value: unknown, at: string, minimum = Number.MIN_SAFE_INTEGER, maximum = Number.MAX_SAFE_INTEGER): number => Number.isSafeInteger(value) && (value as number) >= minimum && (value as number) <= maximum ? value as number : reject(at, 'value is not an integer');
const byte = (value: unknown, at: string): number => integer(value, at, 0, 255);
const optional = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T | undefined => value === undefined ? undefined : parse(value, at);
const targetMode = (value: unknown, at: string): 'internal' | 'external' => { const mode = text(value, at); return mode === 'internal' ? mode : mode === 'external' ? mode : reject(at, 'unknown OPC target mode'); };
const pair = <A, B>(value: unknown, at: string, left: (value: unknown, at: string) => A, right: (value: unknown, at: string) => B): [A, B] => {
  const values = array(value, at);
  if (values.length !== 2) reject(at, 'tuple does not contain exactly two items');
  return [left(values[0], `${at}[0]`), right(values[1], `${at}[1]`)];
};
function indexed<D, T>(value: unknown, at: string, parseDiff: (value: unknown, at: string) => D, parseItem: (value: unknown, at: string) => T): IndexedTripleDiff<D, T> {
  const row = object(value, at);
  return {
    removed: optional(row.removed, `${at}.removed`, (items, itemAt) => array(items, itemAt).map((item, index) => integer(item, `${itemAt}[${index}]`, 0))),
    modified: optional(row.modified, `${at}.modified`, (items, itemAt) => array(items, itemAt).map((item, index) => { const entryAt = `${itemAt}[${index}]`, entry = object(item, entryAt); return { index: integer(entry.index, `${entryAt}.index`, 0), diff: parseDiff(entry.diff, `${entryAt}.diff`) }; })),
    added: optional(row.added, `${at}.added`, (items, itemAt) => array(items, itemAt).map((item, index) => { const entryAt = `${itemAt}[${index}]`, entry = object(item, entryAt); return { index: integer(entry.index, `${entryAt}.index`, 0), item: parseItem(entry.item, `${entryAt}.item`) }; })),
  };
}
function named<K, D, T>(value: unknown, at: string, parseKey: (value: unknown, at: string) => K, parseDiff: (value: unknown, at: string) => D, parseItem: (value: unknown, at: string) => T): NamedTripleDiff<K, D, T> {
  const row = object(value, at);
  return {
    removed: optional(row.removed, `${at}.removed`, (items, itemAt) => array(items, itemAt).map((item, index) => parseKey(item, `${itemAt}[${index}]`))),
    modified: optional(row.modified, `${at}.modified`, (items, itemAt) => array(items, itemAt).map((item, index) => { const entryAt = `${itemAt}[${index}]`, entry = object(item, entryAt); return { key: parseKey(entry.key, `${entryAt}.key`), diff: parseDiff(entry.diff, `${entryAt}.diff`) }; })),
    added: optional(row.added, `${at}.added`, (items, itemAt) => array(items, itemAt).map((item, index) => parseItem(item, `${itemAt}[${index}]`))),
  };
}
function parseRunDiff(value: unknown, at: string): PptxRunDiff { const row = object(value, at); return { text: optional(row.text, `${at}.text`, text), bold: optional(row.bold, `${at}.bold`, boolean), italic: optional(row.italic, `${at}.italic`, boolean), fontSize: row.fontSize === undefined ? undefined : row.fontSize === null ? null : integer(row.fontSize, `${at}.fontSize`, 0, 4294967295) }; }
function parseParagraphDiff(value: unknown, at: string): PptxParagraphDiff { const row = object(value, at); return { runs: optional(row.runs, `${at}.runs`, (item, itemAt) => indexed(item, itemAt, parseRunDiff, parsePptxRun)) }; }
function parseShapeDiff(value: unknown, at: string): PptxShapeDiff {
  const row = object(value, at), shapeKind = text(row.shapeKind, `${at}.shapeKind`);
  if (shapeKind === 'textBox') return { shapeKind, textFrame: optional(row.textFrame, `${at}.textFrame`, (item, itemAt) => indexed(item, itemAt, parseParagraphDiff, parsePptxParagraph)), position: optional(row.position, `${at}.position`, parsePptxTransform) };
  if (shapeKind === 'picture') return { shapeKind, blipRelId: optional(row.blipRelId, `${at}.blipRelId`, text), position: optional(row.position, `${at}.position`, parsePptxTransform) };
  if (shapeKind === 'placeholder') return { shapeKind, kind: optional(row.kind, `${at}.kind`, text), textFrame: optional(row.textFrame, `${at}.textFrame`, (item, itemAt) => indexed(item, itemAt, parseParagraphDiff, parsePptxParagraph)), position: optional(row.position, `${at}.position`, parsePptxTransform) };
  if (shapeKind === 'replace') return { shapeKind, shape: parsePptxShape(row.shape, `${at}.shape`) };
  return reject(`${at}.shapeKind`, `unknown PPTX shape diff kind ${shapeKind}`);
}
function parseSlideDiff(value: unknown, at: string): PptxSlideDiff { const row = object(value, at); return { shapes: optional(row.shapes, `${at}.shapes`, (item, itemAt) => indexed(item, itemAt, parseShapeDiff, parsePptxShape)) }; }
function parseOpcPartDiff(value: unknown, at: string): PptxOpcPartDiff { const row = object(value, at); return { contentType: optional(row.contentType, `${at}.contentType`, text), bytes: optional(row.bytes, `${at}.bytes`, (items, itemAt) => array(items, itemAt).map((item, index) => byte(item, `${itemAt}[${index}]`))) }; }
function parseOpcRelDiff(value: unknown, at: string): PptxOpcRelDiff { const row = object(value, at); return { relType: optional(row.relType, `${at}.relType`, text), target: optional(row.target, `${at}.target`, text), targetMode: optional(row.targetMode, `${at}.targetMode`, targetMode) }; }
function parseOpcRelListDiff(value: unknown, at: string): PptxOpcRelListDiff { return named(value, at, text, parseOpcRelDiff, parseOpcRelationship); }
function parseOpcDiff(value: unknown, at: string): PptxOpcDiff {
  const row = object(value, at), parseEntry = (item: unknown, itemAt: string) => pair(item, itemAt, text, text), parseRelationships = (item: unknown, itemAt: string) => pair(item, itemAt, text, (rels, relsAt) => array(rels, relsAt).map((rel, index) => parseOpcRelationship(rel, `${relsAt}[${index}]`)));
  return {
    comment: optional(row.comment, `${at}.comment`, text),
    contentTypes: optional(row.contentTypes, `${at}.contentTypes`, (item, itemAt) => { const entry = object(item, itemAt); return { defaults: optional(entry.defaults, `${itemAt}.defaults`, (part, partAt) => named(part, partAt, text, text, parseEntry)), overrides: optional(entry.overrides, `${itemAt}.overrides`, (part, partAt) => named(part, partAt, text, text, parseEntry)) }; }),
    parts: optional(row.parts, `${at}.parts`, (item, itemAt) => named(item, itemAt, text, parseOpcPartDiff, parseOpcPart)),
    relationships: optional(row.relationships, `${at}.relationships`, (item, itemAt) => named(item, itemAt, text, parseOpcRelListDiff, parseRelationships)),
  };
}
/** 🚪️ Parses the complete sparse PPTX diff without collapsing OPC, XML, presentation, or shape details. */
export function parsePptxDiff(value: unknown, at = '$'): PptxDiff {
  const row = object(value, at);
  return {
    opc: optional(row.opc, `${at}.opc`, parseOpcDiff),
    presentation: optional(row.presentation, `${at}.presentation`, (item, itemAt) => { const entry = object(item, itemAt); return { slides: optional(entry.slides, `${itemAt}.slides`, (part, partAt) => indexed(part, partAt, parseSlideDiff, parsePptxSlide)) }; }),
    xmlParts: optional(row.xmlParts, `${at}.xmlParts`, (items, itemAt) => array(items, itemAt).map((item, index) => parsePptxXmlPart(item, `${itemAt}[${index}]`))),
  };
}
