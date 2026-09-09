/** 🧬️ Procedure document with independently owned flow and text children. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface ProcedureArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio */ flow: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ text: ArtifactChild;
}

/** 🪪️ Parses the exact document identity boundary shared by all Procedure views. */
export function parseProcedureArtifact(value: unknown, at = "$"): ProcedureArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: document must be an object`);
  const row = value as Record<string, unknown>, keys = ["schema", "flow", "text"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match ProcedureArtifact`);
  if (typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string`);
  return { schema: row.schema, flow: parseArtifactChild(row.flow), text: parseArtifactChild(row.text) };
}
