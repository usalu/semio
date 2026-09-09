/** 🧬️ DAG artifact with one composed graph-content identity. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface DagArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio.graph */ content: ArtifactChild;
}

const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value must be an object`);
  return value as Record<string, unknown>;
};

const exact = (row: Readonly<Record<string, unknown>>, keys: readonly string[], at: string): void => {
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match DagArtifact`);
};

/** 🪪️ Parses the exact durable DAG document boundary and refuses embedded graph payloads. */
export function parseDagArtifact(value: unknown, at = "$"): DagArtifact {
  const row = object(value, at);
  exact(row, ["schema", "content"], at);
  if (typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string`);
  return { schema: row.schema, content: parseArtifactChild(row.content) };
}
