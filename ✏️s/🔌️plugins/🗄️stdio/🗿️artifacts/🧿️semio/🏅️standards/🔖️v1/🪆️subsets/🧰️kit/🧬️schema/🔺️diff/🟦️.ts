import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactLink, type ArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseSemioKitDesign, parseSemioKitSnapshot, parseSemioKitType, type SemioKitDesign, type SemioKitSnapshot, type SemioKitType } from "../📸️snapshot/🟦️.ts";

export interface SemioKitList<T> { values: T[] }
export interface SemioKitDiff {
  types?: SemioKitList<SemioKitType>;
  designs?: SemioKitList<SemioKitDesign>;
  objects?: SemioKitList<ArtifactChild>;
  models?: SemioKitList<ArtifactChild>;
  properties?: ArtifactChild | null;
  representations?: SemioKitList<ArtifactLink>;
}

function list<T>(value: unknown, parse: (entry: unknown, at: string) => T, at: string): SemioKitList<T> {
  const row = parseSchemaRecord(value, ["values"], at);
  if (!Array.isArray(row.values)) throw new Error(at + ".values: array required");
  return { values: row.values.map((entry, index) => parse(entry, `${at}.values[${index}]`)) };
}

/** 🔺️ Parses sparse whole-list replacements and the nullable properties replacement. */
export function parseSemioKitDiff(value: unknown, at = "$"): SemioKitDiff {
  const row = parseSchemaRecord(value, ["types", "designs", "objects", "models", "properties", "representations"], at);
  const result: SemioKitDiff = {};
  if (Object.hasOwn(row, "types")) result.types = list(row.types, parseSemioKitType, at + ".types");
  if (Object.hasOwn(row, "designs")) result.designs = list(row.designs, parseSemioKitDesign, at + ".designs");
  if (Object.hasOwn(row, "objects")) result.objects = list(row.objects, (entry, field) => parseSemioChild(entry, "object", field), at + ".objects");
  if (Object.hasOwn(row, "models")) result.models = list(row.models, (entry, field) => parseSemioChild(entry, "model", field), at + ".models");
  if (Object.hasOwn(row, "properties")) result.properties = row.properties === null ? null : parseSemioChild(row.properties, "value", at + ".properties");
  if (Object.hasOwn(row, "representations")) result.representations = list(row.representations, (entry) => parseArtifactLink(entry), at + ".representations");
  return result;
}

/** 🧮️ Applies every present replacement and validates the resulting Kit document. */
export function applySemioKitDiff(base: SemioKitSnapshot, diff: SemioKitDiff): SemioKitSnapshot {
  const result: SemioKitSnapshot = { ...base };
  if (diff.types !== undefined) result.types = diff.types.values;
  if (diff.designs !== undefined) result.designs = diff.designs.values;
  if (diff.objects !== undefined) result.objects = diff.objects.values;
  if (diff.models !== undefined) result.models = diff.models.values;
  if (diff.properties === null) delete result.properties;
  else if (diff.properties !== undefined) result.properties = diff.properties;
  if (diff.representations !== undefined) result.representations = diff.representations.values;
  return parseSemioKitSnapshot(result);
}
