/** 🧬️ Dag durable document with exact owned-child coordinates. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";

export interface DagArtifact {
  /** @state artifact */ schema: "dag.dag";
  /** @state artifact @child kind=s.stdio.semio */ content: ArtifactChild;
}

/** 🪪️ Validates the document marker, declared fields and each exact child dialect. */
export function parseDagArtifact(value: unknown, at = "$"): DagArtifact {
  const row = parseSchemaRecord(value, ["schema", "content"], at);
  if (row.schema !== "dag.dag") throw new Error(`${at}.schema: invalid Dag marker`);
  return { schema: row.schema, content: parseSemioChild(row.content, "graph", `${at}.content`) };
}
