/** 🧬️ Playbook document fields reference the canonical composed `flow` child that holds the steps. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface PlaybookArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title: string | null;
  /** @state artifact @child kind=s.stdio.semio */
  flow: ArtifactChild;
}

/** 🪪️ Validates exact document fields, including the required nullable native title. */
export function parsePlaybookArtifact(value: unknown, at = "$"): PlaybookArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a Playbook document`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "id", "version", "title", "flow"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key)) || ["schema", "id", "version"].some((key) => typeof row[key] !== "string") || (row.title !== null && typeof row.title !== "string")) throw new Error(`${at}: invalid Playbook document fields`);
  return { schema: row.schema as string, id: row.id as string, version: row.version as string, title: row.title as string | null, flow: parseArtifactChild(row.flow) };
}
