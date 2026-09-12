import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseCurationArtifact, parseCurationCount, parseCurationList, parseCurationText, parseCuratedItem, parseObjectKindExtra, type CurationArtifact, type CuratedItem, type ObjectKindExtra } from "../🟦️.ts";
export interface CurationObjectKindExtraPatchEntry { id: string; extra: ObjectKindExtra }
export interface CurationCuratedPatchEntry { objectId: string; count?: number | null }
export interface CurationStockExtraDelta { added?: ObjectKindExtra[]; removed?: string[]; patched?: CurationObjectKindExtraPatchEntry[]; reordered?: string[] | null }
export interface CurationCuratedDelta { added?: CuratedItem[]; removed?: string[]; patched?: CurationCuratedPatchEntry[]; reordered?: string[] | null }
export interface CurationDiff {
  /** @state artifact */ artifact?: CurationArtifact | null;
  /** @state artifact @child kind=s.stdio.semio */ catalog?: ArtifactChild | null;
  /** @state artifact */ stockExtra?: CurationStockExtraDelta | null;
  /** @state artifact */ curated?: CurationCuratedDelta | null;
}
/** 🩹 Admits an exact curated quantity patch. */
export function parseCurationCuratedPatchEntry(value: unknown, at = "$"): CurationCuratedPatchEntry {
  const row = parseSchemaRecord(value, ["objectId", "count"], at);
  return { objectId: parseCurationText(row.objectId, at + ".objectId"), ...(Object.hasOwn(row, "count") ? { count: row.count === null ? null : parseCurationCount(row.count, at + ".count") } : {}) };
}
function stockPatch(value: unknown, at: string): CurationObjectKindExtraPatchEntry {
  const row = parseSchemaRecord(value, ["id", "extra"], at);
  return { id: parseCurationText(row.id, at + ".id"), extra: parseObjectKindExtra(row.extra, at + ".extra") };
}
function delta<T, P>(value: unknown, parse: (value: unknown, at: string) => T, patch: (value: unknown, at: string) => P, at: string): { added?: T[]; removed?: string[]; patched?: P[]; reordered?: string[] | null } {
  const row = parseSchemaRecord(value, ["added", "removed", "patched", "reordered"], at);
  return {
    ...(Object.hasOwn(row, "added") ? { added: parseCurationList(row.added, parse, at + ".added") } : {}),
    ...(Object.hasOwn(row, "removed") ? { removed: parseCurationList(row.removed, parseCurationText, at + ".removed") } : {}),
    ...(Object.hasOwn(row, "patched") ? { patched: parseCurationList(row.patched, patch, at + ".patched") } : {}),
    ...(Object.hasOwn(row, "reordered") ? { reordered: row.reordered === null ? null : parseCurationList(row.reordered, parseCurationText, at + ".reordered") } : {}),
  };
}
/** 🔺 Admits sparse document edits without adding absent fields. */
export function parseCurationDiff(value: unknown, at = "$"): CurationDiff {
  const row = parseSchemaRecord(value, ["artifact", "catalog", "stockExtra", "curated"], at);
  return {
    ...(Object.hasOwn(row, "artifact") ? { artifact: row.artifact === null ? null : parseCurationArtifact(row.artifact, at + ".artifact") } : {}),
    ...(Object.hasOwn(row, "catalog") ? { catalog: row.catalog === null ? null : parseSemioChild(row.catalog, "kit", at + ".catalog") } : {}),
    ...(Object.hasOwn(row, "stockExtra") ? { stockExtra: row.stockExtra === null ? null : delta(row.stockExtra, parseObjectKindExtra, stockPatch, at + ".stockExtra") } : {}),
    ...(Object.hasOwn(row, "curated") ? { curated: row.curated === null ? null : delta(row.curated, parseCuratedItem, parseCurationCuratedPatchEntry, at + ".curated") } : {}),
  };
}
