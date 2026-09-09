import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioTransform, type SemioTransform } from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../✉️base/🧬️schema/🪆️child/🟦️.ts";
import type { SemioObjectArtifact } from "../🟦️.ts";

export interface SemioObjectDiff {
  /** @state artifact */ transform?: SemioTransform;
  /** @state artifact */ brep?: ArtifactChild | null;
  /** @state artifact */ mesh?: ArtifactChild | null;
  /** @state artifact */ properties?: ArtifactChild | null;
}

/** 🔺️ Parses explicit child replacement, removal and untouched fields. */
export function parseSemioObjectDiff(value: unknown, at = "$"): SemioObjectDiff {
  const row = parseSchemaRecord(value, ["transform", "brep", "mesh", "properties"], at);
  const result: SemioObjectDiff = {};
  if (Object.hasOwn(row, "transform")) result.transform = parseSemioTransform(row.transform, at + ".transform");
  for (const field of ["brep", "mesh", "properties"] as const) {
    if (Object.hasOwn(row, field)) result[field] = row[field] === null ? null : parseSemioChild(row[field], field === "properties" ? "value" : field, at + "." + field);
  }
  return result;
}

/** 🧮️ Applies parent edits while preserving every untouched child identity. */
export function applySemioObjectDiff(base: SemioObjectArtifact, diff: SemioObjectDiff): SemioObjectArtifact {
  const result = { ...base };
  if (diff.transform !== undefined) result.transform = diff.transform;
  for (const field of ["brep", "mesh", "properties"] as const) {
    if (diff[field] === null) delete result[field];
    else if (diff[field] !== undefined) result[field] = diff[field];
  }
  return result;
}
