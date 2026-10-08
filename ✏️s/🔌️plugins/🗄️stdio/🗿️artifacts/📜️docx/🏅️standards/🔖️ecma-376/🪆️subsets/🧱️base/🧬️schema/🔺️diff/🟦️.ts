import type { DocxXmlPart, OpcPart, OpcRelationship } from '../📸️snapshot/🟦️.ts';
import { parseRetainedXmlDocument } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
import { parseXmlDiff, type XmlDiff } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts';

export interface OpcContentTypeRow { name: string; contentType: string }
export interface OpcOwnerRow { owner: string; relationships: OpcRelationship[] }
export interface Removal { id: string; index: number }
export interface Insertion<R> { index: number; row: R }
export interface Relocation { id: string; from: number; to: number }
export interface Modification<Q> { id: string; patch: Q }
export interface ListDelta<R, Q> { removed?: Removal[]; inserted?: Insertion<R>[]; moved?: Relocation[]; modified?: Modification<Q>[] }
export interface DocxOpcCtEntryPatch { contentType?: string }
export interface DocxOpcOwnerPatch { relationships?: DocxOpcRelListDiff }
export type DocxOpcCtEntriesDiff = ListDelta<OpcContentTypeRow, DocxOpcCtEntryPatch>;
export interface DocxOpcPartDiff { contentType?: string; bytes?: number[] }
export type DocxOpcPartsDiff = ListDelta<OpcPart, DocxOpcPartDiff>;
export interface DocxOpcRelDiff { relType?: string; target?: string; targetMode?: 'internal' | 'external' }
export type DocxOpcRelListDiff = ListDelta<OpcRelationship, DocxOpcRelDiff>;
export type DocxOpcRelationshipsDiff = ListDelta<OpcOwnerRow, DocxOpcOwnerPatch>;
export interface DocxOpcContentTypesDiff { defaults?: DocxOpcCtEntriesDiff; overrides?: DocxOpcCtEntriesDiff }
export interface DocxOpcDiff { comment?: string; contentTypes?: DocxOpcContentTypesDiff; parts?: DocxOpcPartsDiff; relationships?: DocxOpcRelationshipsDiff }
export interface DocxXmlPartDiff { contentType?: string; document?: XmlDiff }
export type DocxXmlPartsDiff = ListDelta<DocxXmlPart, DocxXmlPartDiff>;
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
function delta<R, Q>(value: unknown, at: string, parseId: (value: unknown, at: string) => string, parseRow: (value: unknown, at: string) => R, parsePatch: (value: unknown, at: string) => Q): ListDelta<R, Q> {
  const row = object(value, at), list = <T>(items: unknown, itemAt: string, parse: (entry: Readonly<Record<string, unknown>>, entryAt: string) => T): T[] => array(items, itemAt).map((item, index) => parse(object(item, `${itemAt}[${index}]`), `${itemAt}[${index}]`));
  return {
    removed: optional(row.removed, `${at}.removed`, (items, itemAt) => list(items, itemAt, (entry, entryAt) => ({ id: parseId(entry.id, `${entryAt}.id`), index: integer(entry.index, `${entryAt}.index`) }))),
    inserted: optional(row.inserted, `${at}.inserted`, (items, itemAt) => list(items, itemAt, (entry, entryAt) => ({ index: integer(entry.index, `${entryAt}.index`), row: parseRow(entry.row, `${entryAt}.row`) }))),
    moved: optional(row.moved, `${at}.moved`, (items, itemAt) => list(items, itemAt, (entry, entryAt) => ({ id: parseId(entry.id, `${entryAt}.id`), from: integer(entry.from, `${entryAt}.from`), to: integer(entry.to, `${entryAt}.to`) }))),
    modified: optional(row.modified, `${at}.modified`, (items, itemAt) => list(items, itemAt, (entry, entryAt) => ({ id: parseId(entry.id, `${entryAt}.id`), patch: parsePatch(entry.patch, `${entryAt}.patch`) }))),
  };
}
function parseOpcPart(value: unknown, at: string): OpcPart { const row = object(value, at); return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), bytes: array(row.bytes, `${at}.bytes`).map((item, index) => byte(item, `${at}.bytes[${index}]`)) }; }
function parseOpcRelationship(value: unknown, at: string): OpcRelationship { const row = object(value, at); return { id: text(row.id, `${at}.id`), relType: text(row.relType, `${at}.relType`), target: text(row.target, `${at}.target`), targetMode: targetMode(row.targetMode, `${at}.targetMode`) }; }
function parseOpcPartDiff(value: unknown, at: string): DocxOpcPartDiff { const row = object(value, at); return { contentType: optional(row.contentType, `${at}.contentType`, text), bytes: optional(row.bytes, `${at}.bytes`, (items, itemAt) => array(items, itemAt).map((item, index) => byte(item, `${itemAt}[${index}]`))) }; }
function parseOpcRelDiff(value: unknown, at: string): DocxOpcRelDiff { const row = object(value, at); return { relType: optional(row.relType, `${at}.relType`, text), target: optional(row.target, `${at}.target`, text), targetMode: optional(row.targetMode, `${at}.targetMode`, targetMode) }; }
function parseOpcRelListDiff(value: unknown, at: string): DocxOpcRelListDiff { return delta(value, at, text, parseOpcRelationship, parseOpcRelDiff); }
function parseOpcOwnerPatch(value: unknown, at: string): DocxOpcOwnerPatch { const row = object(value, at); return { relationships: optional(row.relationships, `${at}.relationships`, parseOpcRelListDiff) }; }
function parseOpcCtEntryPatch(value: unknown, at: string): DocxOpcCtEntryPatch { const row = object(value, at); return { contentType: optional(row.contentType, `${at}.contentType`, text) }; }
function parseOpcDiff(value: unknown, at: string): DocxOpcDiff {
  const row = object(value, at), parseEntry = (item: unknown, itemAt: string): OpcContentTypeRow => { const entry = object(item, itemAt); return { name: text(entry.name, `${itemAt}.name`), contentType: text(entry.contentType, `${itemAt}.contentType`) }; }, parseRelationships = (item: unknown, itemAt: string): OpcOwnerRow => { const entry = object(item, itemAt); return { owner: text(entry.owner, `${itemAt}.owner`), relationships: array(entry.relationships, `${itemAt}.relationships`).map((rel, index) => parseOpcRelationship(rel, `${itemAt}.relationships[${index}]`)) }; };
  return {
    comment: optional(row.comment, `${at}.comment`, text),
    contentTypes: optional(row.contentTypes, `${at}.contentTypes`, (item, itemAt) => { const entry = object(item, itemAt); return { defaults: optional(entry.defaults, `${itemAt}.defaults`, (part, partAt) => delta(part, partAt, text, parseEntry, parseOpcCtEntryPatch)), overrides: optional(entry.overrides, `${itemAt}.overrides`, (part, partAt) => delta(part, partAt, text, parseEntry, parseOpcCtEntryPatch)) }; }),
    parts: optional(row.parts, `${at}.parts`, (item, itemAt) => delta(item, itemAt, text, parseOpcPart, parseOpcPartDiff)),
    relationships: optional(row.relationships, `${at}.relationships`, (item, itemAt) => delta(item, itemAt, text, parseRelationships, parseOpcOwnerPatch)),
  };
}
function parseDocxXmlPart(value: unknown, at: string): DocxXmlPart {
  const row = object(value, at);
  return { path: text(row.path, `${at}.path`), contentType: text(row.contentType, `${at}.contentType`), document: parseRetainedXmlDocument(row.document, `${at}.document`) };
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
    xmlParts: optional(row.xmlParts, `${at}.xmlParts`, (item, itemAt) => delta(item, itemAt, text, parseDocxXmlPart, parseDocxXmlPartDiff)),
  };
}
