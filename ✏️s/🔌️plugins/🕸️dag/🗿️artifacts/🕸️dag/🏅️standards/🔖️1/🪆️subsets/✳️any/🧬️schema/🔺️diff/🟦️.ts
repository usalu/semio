/** 🔺️ DAG sparse durable delta with whole child-handle replacement. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface DagDiff {
  /** @state artifact */ schema: string | null;
  /** @state artifact @child kind=s.stdio.semio.graph */ content: ArtifactChild | null;
}

/** 🪪️ Parses the exact native DagDiff carrier and refuses legacy graph deltas. */
export function parseDagDiff(value: unknown, at = "$"): DagDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: diff must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "content"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match DagDiff`);
  if (row.schema !== null && typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string or null`);
  return { schema: row.schema as string | null, content: row.content === null ? null : parseArtifactChild(row.content) };
}
