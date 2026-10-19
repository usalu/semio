/** 🔺️ Dag sparse durable delta; omitted slots are untouched. */
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import type { ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";
import { parseDagArtifact, parseDagContentChild, type DagArtifact } from "../🟦️.ts";

export interface DagDiff {
  /** @state artifact */ schema?: "dag.dag";
  /** @state artifact @child kind=s.stdio.semio */ content?: ArtifactChild;
}

/** 🔎️ Keeps absent, set and clear operations distinct at the wire boundary. */
export function parseDagDiff(value: unknown, at = "$"): DagDiff {
  const row = parseSchemaRecord(value, ["schema", "content"], at);
  const diff: DagDiff = {};
  if (Object.hasOwn(row, "schema")) {
    if (row.schema !== "dag.dag") throw new Error(`${at}.schema: invalid Dag marker`);
    diff.schema = row.schema;
  }
  if (Object.hasOwn(row, "content")) diff.content = parseDagContentChild(row.content, `${at}.content`);
  return diff;
}

/** 🩹️ Applies only the declared document edits and preserves untouched child references. */
export function applyDagDiff(base: DagArtifact, value: unknown): DagArtifact {
  parseDagArtifact(base);
  const diff = parseDagDiff(value);
  const next = { ...base, ...diff };
  return next;
}
