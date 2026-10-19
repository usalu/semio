/** 🧬️ Dag durable document with exact owned-child coordinates. */
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import { requireArtifactDialect, type ArtifactDialect, parseArtifactChild, type ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";

const CONTENT_DIALECT: ArtifactDialect = { artifactKind: "s.stdio.semio", standard: "v1", subset: "graph" };

/** 🪆️ Admits the content child by its declared graph dialect, independent of the plugin that owns that dialect. */
export function parseDagContentChild(value: unknown, at = "$"): ArtifactChild {
  const child = parseArtifactChild(value);
  requireArtifactDialect(child.target.dialect, CONTENT_DIALECT, at);
  return child;
}

export interface DagArtifact {
  /** @state artifact */ schema: "dag.dag";
  /** @state artifact @child kind=s.stdio.semio */ content: ArtifactChild;
}

/** 🪪️ Validates the document marker, declared fields and each exact child dialect. */
export function parseDagArtifact(value: unknown, at = "$"): DagArtifact {
  const row = parseSchemaRecord(value, ["schema", "content"], at);
  if (row.schema !== "dag.dag") throw new Error(`${at}.schema: invalid Dag marker`);
  return { schema: row.schema, content: parseDagContentChild(row.content, `${at}.content`) };
}
