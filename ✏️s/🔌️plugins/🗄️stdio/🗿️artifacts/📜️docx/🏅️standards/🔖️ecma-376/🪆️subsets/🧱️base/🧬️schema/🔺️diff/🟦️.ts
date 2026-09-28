import type { DocxXmlPart, OpcPart, OpcRelationship } from '../📸️snapshot/🟦️.ts';
import { parseXmlDocument } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
import { parseXmlDiff, type XmlDiff } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';

export interface NamedModified<K, D> { key: K; diff: D }
export interface NamedTripleDiff<K, D, T> { removed?: K[]; modified?: NamedModified<K, D>[]; added?: T[]; order?: K[] }
export type DocxOpcCtEntriesDiff = NamedTripleDiff<string, string, [string, string]>;
export interface DocxOpcPartDiff { contentType?: string; bytes?: number[] }
export type DocxOpcPartsDiff = NamedTripleDiff<string, DocxOpcPartDiff, OpcPart>;
export interface DocxOpcRelDiff { relType?: string; target?: string; targetMode?: 'internal' | 'external' }
export type DocxOpcRelListDiff = NamedTripleDiff<string, DocxOpcRelDiff, OpcRelationship>;
export type DocxOpcRelationshipsDiff = NamedTripleDiff<string, DocxOpcRelListDiff, [string, OpcRelationship[]]>;
export interface DocxOpcContentTypesDiff { defaults?: DocxOpcCtEntriesDiff; overrides?: DocxOpcCtEntriesDiff }
export interface DocxOpcDiff { comment?: string; contentTypes?: DocxOpcContentTypesDiff; parts?: DocxOpcPartsDiff; relationships?: DocxOpcRelationshipsDiff }
export interface DocxXmlPartDiff { contentType?: string; document?: XmlDiff }
export type DocxXmlPartsDiff = NamedTripleDiff<string, DocxXmlPartDiff, DocxXmlPart>;
export interface DocxDiff { opc?: DocxOpcDiff; xmlParts?: DocxXmlPartsDiff }

/** 🚪️ A precise position and reason for refusing a malformed DOCX diff. */
export class stdioDocxEcma376BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}
const reject = (at: string, why: string): never => { throw new stdioDocxEcma376BaseDiffGuardRefusal(at, why); };
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, 'value is not an object');
const array = (value: unknown, at: string): readonly unknown[] => Array.isArray(value) ? value : reject(at, 'value is not an array');
const text = (value: unknown, at: string): string => typeof value === 'string' ? value : reject(at, 'value is not a string');
const integer = (value: unknown, at: string, maximum = Number.MAX_SAFE_INTEGER): number => Number.isSafeInteger(value) && (value as number) >= 0 && (value as number) <= maximum ? value as number : reject(at, 'value is not an unsigned integer');
const byte = (value: unknown, at: string): number => integer(value, at, 255);
const optional = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T | undefined => value === undefined ? undefined : parse(value, at);
const targetMode = (value: unknown, at: string): 'internal' | 'external' => { const mode = text(value, at); return mode === 'internal' ? mode : mode === 'external' ? mode : reject(at, 'unknown OPC target mode'); };
const pair = <A, B>(value: unknown, at: string, left: (value: unknown, at: string) => A, right: (value: unknown, at: string) => B): [A, B] => {
  const values = array(value, at);
  if (values.length !== 2) reject(at, 'tuple does not contain exactly two items');
  return [left(values[0], `${at}[0]`), right(values[1], `${at}[1]`)];
};
function named<K, D, T>(value: unknown, at: string, parseKey: (value: unknown, at: string) => K, parseDiff: (value: unknown, at: string) => D, parseItem: (value: unknown, at: string) => T): NamedTripleDiff<K, D, T> {
  const row = object(value, at);
  return {
    removed: optional(row.removed, `${at}.removed`, (items, itemAt) => array(items, itemAt).map((item, index) => parseKey(item, `${itemAt}[${index}]`))),
    modified: optional(row.modified, `${at}.modified`, (items, itemAt) => array(items, itemAt).map((item, index) => { const entryAt = `${itemAt}[${index}]`, entry = object(item, entryAt); return { key: parseKey(entry.key, `${entryAt}.key`), diff: parseDiff(entry.diff, `${entryAt}.diff`) }; })),
    added: optional(row.added, `${at}.added`, (items, itemAt) => array(items, itemAt).map((item, index) => parseItem(item, `${itemAt}[${index}]`))),
    order: optional(row.order, `${at}.order`, (items, itemAt) => array(items, itemAt).map((item, index) => parseKey(item, `${itemAt}[${index}]`))),
  };
}
function parseOpcPart(value: unknown, at: string): OpcPart { const row = object(value, at); return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), bytes: array(row.bytes, `${at}.bytes`).map((item, index) => byte(item, `${at}.bytes[${index}]`)) }; }
function parseOpcRelationship(value: unknown, at: string): OpcRelationship { const row = object(value, at); return { id: text(row.id, `${at}.id`), relType: text(row.relType, `${at}.relType`), target: text(row.target, `${at}.target`), targetMode: targetMode(row.targetMode, `${at}.targetMode`) }; }
function parseOpcPartDiff(value: unknown, at: string): DocxOpcPartDiff { const row = object(value, at); return { contentType: optional(row.contentType, `${at}.contentType`, text), bytes: optional(row.bytes, `${at}.bytes`, (items, itemAt) => array(items, itemAt).map((item, index) => byte(item, `${itemAt}[${index}]`))) }; }
function parseOpcRelDiff(value: unknown, at: string): DocxOpcRelDiff { const row = object(value, at); return { relType: optional(row.relType, `${at}.relType`, text), target: optional(row.target, `${at}.target`, text), targetMode: optional(row.targetMode, `${at}.targetMode`, targetMode) }; }
function parseOpcRelListDiff(value: unknown, at: string): DocxOpcRelListDiff { return named(value, at, text, parseOpcRelDiff, parseOpcRelationship); }
function parseOpcDiff(value: unknown, at: string): DocxOpcDiff {
  const row = object(value, at), parseEntry = (item: unknown, itemAt: string) => pair(item, itemAt, text, text), parseRelationships = (item: unknown, itemAt: string) => pair(item, itemAt, text, (rels, relsAt) => array(rels, relsAt).map((rel, index) => parseOpcRelationship(rel, `${relsAt}[${index}]`)));
  return {
    comment: optional(row.comment, `${at}.comment`, text),
    contentTypes: optional(row.contentTypes, `${at}.contentTypes`, (item, itemAt) => { const entry = object(item, itemAt); return { defaults: optional(entry.defaults, `${itemAt}.defaults`, (part, partAt) => named(part, partAt, text, text, parseEntry)), overrides: optional(entry.overrides, `${itemAt}.overrides`, (part, partAt) => named(part, partAt, text, text, parseEntry)) }; }),
    parts: optional(row.parts, `${at}.parts`, (item, itemAt) => named(item, itemAt, text, parseOpcPartDiff, parseOpcPart)),
    relationships: optional(row.relationships, `${at}.relationships`, (item, itemAt) => named(item, itemAt, text, parseOpcRelListDiff, parseRelationships)),
  };
}
function parseDocxXmlPart(value: unknown, at: string): DocxXmlPart {
  const row = object(value, at);
  return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), document: parseXmlDocument(row.document, `${at}.document`) };
}
function parseDocxXmlPartDiff(value: unknown, at: string): DocxXmlPartDiff {
  const row = object(value, at);
  return { contentType: optional(row.contentType, `${at}.contentType`, text), document: optional(row.document, `${at}.document`, parseXmlDiff) };
}
/** 🚪️ Parses the sparse canonical-DOCX diff without reconstructing a semantic shadow tree. */
export function parseDocxDiff(value: unknown, at = '$'): DocxDiff {
  const row = object(value, at);
  return {
    opc: optional(row.opc, `${at}.opc`, parseOpcDiff),
    xmlParts: optional(row.xmlParts, `${at}.xmlParts`, (item, itemAt) => named(item, itemAt, text, parseDocxXmlPartDiff, parseDocxXmlPart)),
  };
}
