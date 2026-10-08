import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactLink, type ArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseSemioKitConnection, parseSemioKitDesign, parseSemioKitPiece, parseSemioKitSnapshot, parseSemioKitType, type SemioKitConnection, type SemioKitDesign, type SemioKitPiece, type SemioKitSnapshot, type SemioKitType } from "../📸️snapshot/🟦️.ts";

export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface Replace<T> { value: T }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[] }
export interface SemioKitTypeDiff { name?: string; category?: string }
export interface SemioKitDesignDiff { name?: string; pieces?: SemioKitPiece[]; connections?: SemioKitConnection[] }
export interface SemioKitLinkDiff { pin?: ArtifactLink["pin"] }
export interface SemioKitDiff {
  types?: IndexedTripleDiff<SemioKitTypeDiff, SemioKitType>;
  designs?: IndexedTripleDiff<SemioKitDesignDiff, SemioKitDesign>;
  objects?: IndexedTripleDiff<Replace<ArtifactChild>, ArtifactChild>;
  models?: IndexedTripleDiff<Replace<ArtifactChild>, ArtifactChild>;
  properties?: ArtifactChild | null;
  representations?: IndexedTripleDiff<SemioKitLinkDiff, ArtifactLink>;
}

function index(value: unknown, at: string): number {
  if (!Number.isSafeInteger(value) || (value as number) < 0) throw new Error(at + ": non-negative integer required");
  return value as number;
}

function triple<D, T>(value: unknown, parseRow: (entry: unknown, at: string) => D, parseItem: (entry: unknown, at: string) => T, at: string): IndexedTripleDiff<D, T> {
  const row = parseSchemaRecord(value, ["removed", "modified", "added"], at);
  const removed = row.removed === undefined ? [] : (Array.isArray(row.removed) ? row.removed.map((entry, position) => index(entry, `${at}.removed[${position}]`)) : (() => { throw new Error(at + ".removed: array required"); })());
  const modified = row.modified === undefined ? [] : (Array.isArray(row.modified) ? row.modified.map((entry, position) => { const field = `${at}.modified[${position}]`; const item = parseSchemaRecord(entry, ["index", "diff"], field); return { index: index(item.index, field + ".index"), diff: parseRow(item.diff, field + ".diff") }; }) : (() => { throw new Error(at + ".modified: array required"); })());
  const added = row.added === undefined ? [] : (Array.isArray(row.added) ? row.added.map((entry, position) => { const field = `${at}.added[${position}]`; const item = parseSchemaRecord(entry, ["index", "item"], field); return { index: index(item.index, field + ".index"), item: parseItem(item.item, field + ".item") }; }) : (() => { throw new Error(at + ".added: array required"); })());
  return { removed, modified, added };
}

function typeDiff(value: unknown, at: string): SemioKitTypeDiff {
  const row = parseSchemaRecord(value, ["name", "category"], at);
  const result: SemioKitTypeDiff = {};
  if (row.name !== undefined) result.name = parseSemioKitType({ id: "x", name: row.name, category: "x" }, at).name;
  if (row.category !== undefined) result.category = parseSemioKitType({ id: "x", name: "x", category: row.category }, at).category;
  return result;
}

function designDiff(value: unknown, at: string): SemioKitDesignDiff {
  const row = parseSchemaRecord(value, ["name", "pieces", "connections"], at);
  const result: SemioKitDesignDiff = {};
  if (row.name !== undefined) result.name = parseSemioKitDesign({ id: "x", name: row.name, pieces: [], connections: [] }, at).name;
  if (row.pieces !== undefined) { if (!Array.isArray(row.pieces)) throw new Error(at + ".pieces: array required"); result.pieces = row.pieces.map((entry, position) => parseSemioKitPiece(entry, `${at}.pieces[${position}]`)); }
  if (row.connections !== undefined) { if (!Array.isArray(row.connections)) throw new Error(at + ".connections: array required"); result.connections = row.connections.map((entry, position) => parseSemioKitConnection(entry, `${at}.connections[${position}]`)); }
  return result;
}

function replaceChild(subset: "object" | "model") {
  return (value: unknown, at: string): Replace<ArtifactChild> => ({ value: parseSemioChild(parseSchemaRecord(value, ["value"], at).value, subset, at + ".value") });
}

function linkDiff(value: unknown, at: string): SemioKitLinkDiff {
  const row = parseSchemaRecord(value, ["pin"], at);
  return row.pin === undefined ? {} : { pin: parseArtifactLink({ target: { artifactId: "x", dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "mesh" } }, pin: row.pin, role: "x" }).pin };
}

/** 🔺️ Parses the sparse keyed-row diff: per collection removed base indices, modified rows and added rows with their final index. */
export function parseSemioKitDiff(value: unknown, at = "$"): SemioKitDiff {
  const row = parseSchemaRecord(value, ["types", "designs", "objects", "models", "properties", "representations"], at);
  const result: SemioKitDiff = {};
  if (Object.hasOwn(row, "types")) result.types = triple(row.types, typeDiff, parseSemioKitType, at + ".types");
  if (Object.hasOwn(row, "designs")) result.designs = triple(row.designs, designDiff, parseSemioKitDesign, at + ".designs");
  if (Object.hasOwn(row, "objects")) result.objects = triple(row.objects, replaceChild("object"), (entry, field) => parseSemioChild(entry, "object", field), at + ".objects");
  if (Object.hasOwn(row, "models")) result.models = triple(row.models, replaceChild("model"), (entry, field) => parseSemioChild(entry, "model", field), at + ".models");
  if (Object.hasOwn(row, "properties")) result.properties = row.properties === null ? null : parseSemioChild(row.properties, "value", at + ".properties");
  if (Object.hasOwn(row, "representations")) result.representations = triple(row.representations, linkDiff, (entry) => parseArtifactLink(entry), at + ".representations");
  return result;
}

function applyTriple<D, T>(base: readonly T[], diff: IndexedTripleDiff<D, T>, applyRow: (row: T, change: D) => T): T[] {
  const rows = base.map((row) => row);
  for (const { index: at, diff: change } of diff.modified) {
    if (at >= rows.length) throw new Error(`modified index ${at} is outside ${rows.length} row(s)`);
    rows[at] = applyRow(rows[at]!, change);
  }
  const survivors = rows.filter((_, position) => !diff.removed.includes(position));
  for (const { index: at, item } of [...diff.added].sort((left, right) => left.index - right.index)) survivors.splice(Math.min(at, survivors.length), 0, item);
  return survivors;
}

/** 🧮️ Applies every present keyed-row change and validates the resulting Kit document. */
export function applySemioKitDiff(base: SemioKitSnapshot, diff: SemioKitDiff): SemioKitSnapshot {
  const result: SemioKitSnapshot = { ...base };
  if (diff.types !== undefined) result.types = applyTriple(base.types, diff.types, (row, change) => ({ ...row, ...change }));
  if (diff.designs !== undefined) result.designs = applyTriple(base.designs, diff.designs, (row, change) => ({ ...row, ...change }));
  if (diff.objects !== undefined) result.objects = applyTriple(base.objects, diff.objects, (_, change) => change.value);
  if (diff.models !== undefined) result.models = applyTriple(base.models, diff.models, (_, change) => change.value);
  if (diff.properties === null) delete result.properties;
  else if (diff.properties !== undefined) result.properties = diff.properties;
  if (diff.representations !== undefined) result.representations = applyTriple(base.representations, diff.representations, (row, change) => ({ ...row, ...change }));
  return parseSemioKitSnapshot(result);
}
