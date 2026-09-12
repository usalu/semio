/** 🗂️ Curation owns catalog references, sourcing geometry and curated quantities. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";

export type GeometryRecipe =
  | { kind: "box"; width: number; height: number; depth: number }
  | { kind: "frame"; width: number; height: number; depth: number; profile: number }
  | { kind: "slab"; width: number; depth: number; thickness: number }
  | { kind: "mesh"; positions: number[]; normals: number[]; indices: number[] };
export interface ObjectKindExtra { id: string; name: string; moduleId: string; typologyPath: string[]; availability: number; geometry: GeometryRecipe }
export interface ObjectKind extends ObjectKindExtra {}
export interface CuratedItem { objectId: string; count: number }
export interface CurationArtifact {
  /** @state artifact @child kind=s.stdio.semio */ catalog: ArtifactChild;
  /** @state artifact */ stockExtra: ObjectKindExtra[];
  /** @state artifact */ curated: CuratedItem[];
}

/** 🔤 Admits a required sourcing identifier or label. */
export function parseCurationText(value: unknown, at: string): string {
  if (typeof value !== "string") throw new Error(at + ": text required");
  return value;
}
/** 🔢 Admits a native unsigned 32-bit quantity. */
export function parseCurationCount(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > 4294967295) throw new Error(at + ": uint32 required");
  return value;
}
/** 📋 Admits an ordered declared collection. */
export function parseCurationList<T>(value: unknown, parse: (item: unknown, at: string) => T, at: string): T[] {
  if (!Array.isArray(value)) throw new Error(at + ": array required");
  return value.map((item, index) => parse(item, `${at}[${index}]`));
}
function finite(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(at + ": finite number required");
  return value;
}
/** 🧱 Parses the geometry discriminant at its sourcing recipe owner. */
export function parseGeometryRecipe(value: unknown, at = "$"): GeometryRecipe {
  const row = parseSchemaRecord(value, ["kind", "width", "height", "depth", "profile", "thickness", "positions", "normals", "indices"], at);
  const fields = row.kind === "box" ? ["kind", "width", "height", "depth"] : row.kind === "frame" ? ["kind", "width", "height", "depth", "profile"] : row.kind === "slab" ? ["kind", "width", "depth", "thickness"] : row.kind === "mesh" ? ["kind", "positions", "normals", "indices"] : undefined;
  if (!fields) throw new Error(at + ": unknown geometry kind");
  parseSchemaRecord(row, fields, at);
  if (row.kind === "mesh") return { kind: row.kind, positions: parseCurationList(row.positions, finite, at + ".positions"), normals: parseCurationList(row.normals, finite, at + ".normals"), indices: parseCurationList(row.indices, parseCurationCount, at + ".indices") };
  const width = finite(row.width, at + ".width"), depth = finite(row.depth, at + ".depth");
  if (row.kind === "slab") return { kind: row.kind, width, depth, thickness: finite(row.thickness, at + ".thickness") };
  const height = finite(row.height, at + ".height");
  if (row.kind === "frame") return { kind: row.kind, width, height, depth, profile: finite(row.profile, at + ".profile") };
  return { kind: "box", width, height, depth };
}
/** 🧩 Parses sourcing-owned catalog metadata beside its shared Kit handle. */
export function parseObjectKindExtra(value: unknown, at = "$"): ObjectKindExtra {
  const row = parseSchemaRecord(value, ["id", "name", "moduleId", "typologyPath", "availability", "geometry"], at);
  return { id: parseCurationText(row.id, at + ".id"), name: parseCurationText(row.name, at + ".name"), moduleId: parseCurationText(row.moduleId, at + ".moduleId"), typologyPath: parseCurationList(row.typologyPath, parseCurationText, at + ".typologyPath"), availability: parseCurationCount(row.availability, at + ".availability"), geometry: parseGeometryRecipe(row.geometry, at + ".geometry") };
}
/** 🧺 Admits a persisted selection quantity. */
export function parseCuratedItem(value: unknown, at = "$"): CuratedItem {
  const row = parseSchemaRecord(value, ["objectId", "count"], at);
  return { objectId: parseCurationText(row.objectId, at + ".objectId"), count: parseCurationCount(row.count, at + ".count") };
}
/** 🪪 Admits exactly one persisted curation document. */
export function parseCurationArtifact(value: unknown, at = "$"): CurationArtifact {
  const row = parseSchemaRecord(value, ["catalog", "stockExtra", "curated"], at);
  return { catalog: parseSemioChild(row.catalog, "kit", at + ".catalog"), stockExtra: parseCurationList(row.stockExtra, parseObjectKindExtra, at + ".stockExtra"), curated: parseCurationList(row.curated, parseCuratedItem, at + ".curated") };
}
