import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioTransform, type SemioTransform } from "../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../✉️base/🧬️schema/🪆️child/🟦️.ts";
export type { SemioTransform } from "../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
export type { ArtifactChild } from "../../✉️base/🧬️schema/🪆️child/🟦️.ts";

export interface SemioObjectArtifact {
  /** @state artifact */ schema: "stdio.semio.object";
  /** @state artifact */ transform: SemioTransform;
  /** @state artifact @child kind=s.stdio.semio */ brep?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ mesh?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ properties?: ArtifactChild;
}

/** 📦️ Parses the Object parent and its exact owned child references. */
export function parseSemioObjectArtifact(value: unknown, at = "$"): SemioObjectArtifact {
  const row = parseSchemaRecord(value, ["schema", "transform", "brep", "mesh", "properties"], at);
  if (row.schema !== "stdio.semio.object") throw new Error(at + ".schema: Object schema required");
  const result: SemioObjectArtifact = { schema: row.schema, transform: parseSemioTransform(row.transform, at + ".transform") };
  if (Object.hasOwn(row, "brep")) result.brep = parseSemioChild(row.brep, "brep", at + ".brep");
  if (Object.hasOwn(row, "mesh")) result.mesh = parseSemioChild(row.mesh, "mesh", at + ".mesh");
  if (Object.hasOwn(row, "properties")) result.properties = parseSemioChild(row.properties, "value", at + ".properties");
  return result;
}
