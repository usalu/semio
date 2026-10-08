import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseCurationCount, parseCurationList, parseCurationText, parseCuratedItem, parseObjectKindExtra, type CuratedItem, type ObjectKindExtra } from "../🟦️.ts";
export interface CurationObjectKindExtraPatchEntry { extra: ObjectKindExtra }
export interface CurationCuratedPatchEntry { count?: number | null }
export interface CurationListRemoval { id: string; index: number }
export interface CurationListRelocation { id: string; from: number; to: number }
export interface CurationListInsertion<T> { index: number; row: T }
export interface CurationListModification<P> { id: string; patch: P }
export interface CurationListDelta<T, P> { removed?: CurationListRemoval[]; inserted?: CurationListInsertion<T>[]; moved?: CurationListRelocation[]; modified?: CurationListModification<P>[] }
export type CurationStockExtraDelta = CurationListDelta<ObjectKindExtra, CurationObjectKindExtraPatchEntry>;
export type CurationCuratedDelta = CurationListDelta<CuratedItem, CurationCuratedPatchEntry>;
export interface CurationDiff {
  /** @state artifact @child kind=s.stdio.semio */ catalog?: ArtifactChild | null;
  /** @state artifact */ stockExtra?: CurationStockExtraDelta | null;
  /** @state artifact */ curated?: CurationCuratedDelta | null;
}
/** 🩹 Admits an exact curated quantity patch. */
export function parseCurationCuratedPatchEntry(value: unknown, at = "$"): CurationCuratedPatchEntry {
  const row = parseSchemaRecord(value, ["count"], at);
  return { ...(Object.hasOwn(row, "count") ? { count: row.count === null ? null : parseCurationCount(row.count, at + ".count") } : {}) };
}
function stockPatch(value: unknown, at: string): CurationObjectKindExtraPatchEntry {
  const row = parseSchemaRecord(value, ["extra"], at);
  return { extra: parseObjectKindExtra(row.extra, at + ".extra") };
}
function delta<T, P>(value: unknown, parse: (value: unknown, at: string) => T, patch: (value: unknown, at: string) => P, at: string): CurationListDelta<T, P> {
  const row = parseSchemaRecord(value, ["removed", "inserted", "moved", "modified"], at);
  const removal = (entry: unknown, where: string): CurationListRemoval => {
    const item = parseSchemaRecord(entry, ["id", "index"], where);
    return { id: parseCurationText(item.id, where + ".id"), index: parseCurationCount(item.index, where + ".index") };
  };
  const insertion = (entry: unknown, where: string): CurationListInsertion<T> => {
    const item = parseSchemaRecord(entry, ["index", "row"], where);
    return { index: parseCurationCount(item.index, where + ".index"), row: parse(item.row, where + ".row") };
  };
  const relocation = (entry: unknown, where: string): CurationListRelocation => {
    const item = parseSchemaRecord(entry, ["id", "from", "to"], where);
    return { id: parseCurationText(item.id, where + ".id"), from: parseCurationCount(item.from, where + ".from"), to: parseCurationCount(item.to, where + ".to") };
  };
  const modification = (entry: unknown, where: string): CurationListModification<P> => {
    const item = parseSchemaRecord(entry, ["id", "patch"], where);
    return { id: parseCurationText(item.id, where + ".id"), patch: patch(item.patch, where + ".patch") };
  };
  return {
    ...(Object.hasOwn(row, "removed") ? { removed: parseCurationList(row.removed, removal, at + ".removed") } : {}),
    ...(Object.hasOwn(row, "inserted") ? { inserted: parseCurationList(row.inserted, insertion, at + ".inserted") } : {}),
    ...(Object.hasOwn(row, "moved") ? { moved: parseCurationList(row.moved, relocation, at + ".moved") } : {}),
    ...(Object.hasOwn(row, "modified") ? { modified: parseCurationList(row.modified, modification, at + ".modified") } : {}),
  };
}
/** 🔺 Admits sparse document edits without adding absent fields. */
export function parseCurationDiff(value: unknown, at = "$"): CurationDiff {
  const row = parseSchemaRecord(value, ["catalog", "stockExtra", "curated"], at);
  return {
    ...(Object.hasOwn(row, "catalog") ? { catalog: row.catalog === null ? null : parseSemioChild(row.catalog, "kit", at + ".catalog") } : {}),
    ...(Object.hasOwn(row, "stockExtra") ? { stockExtra: row.stockExtra === null ? null : delta(row.stockExtra, parseObjectKindExtra, stockPatch, at + ".stockExtra") } : {}),
    ...(Object.hasOwn(row, "curated") ? { curated: row.curated === null ? null : delta(row.curated, parseCuratedItem, parseCurationCuratedPatchEntry, at + ".curated") } : {}),
  };
}
