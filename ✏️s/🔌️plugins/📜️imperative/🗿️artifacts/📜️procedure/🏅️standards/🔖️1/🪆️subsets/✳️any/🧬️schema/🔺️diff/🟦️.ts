/** 🔺️ Procedure durable delta with exact child-slot replacement. */
import { parseArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseProcedureArtifact, type ProcedureArtifact, type ArtifactChild } from "../🟦️.ts";

export interface ProcedureDiff {
  /** @state artifact */ artifact: ProcedureArtifact | null;
  /** @state artifact */ schema: string | null;
  /** @state artifact @child kind=s.stdio.semio */ flow: ArtifactChild | null;
  /** @state artifact @child kind=s.stdio.semio */ text: ArtifactChild | null;
}

/** 🧮️ Resolves omitted delta fields to the native unchanged null value. */
export function parseProcedureDiff(value: unknown, at = "$"): ProcedureDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: delta must be an object`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !["artifact", "schema", "flow", "text"].includes(key))) throw new Error(`${at}: fields do not match ProcedureDiff`);
  if (row.schema != null && typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string or null`);
  return {
    artifact: row.artifact == null ? null : parseProcedureArtifact(row.artifact, `${at}.artifact`),
    schema: (row.schema ?? null) as string | null,
    flow: row.flow == null ? null : parseArtifactChild(row.flow),
    text: row.text == null ? null : parseArtifactChild(row.text),
  };
}
