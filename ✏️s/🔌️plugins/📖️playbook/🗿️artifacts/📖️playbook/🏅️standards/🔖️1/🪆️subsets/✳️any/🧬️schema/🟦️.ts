/** 🧬️ Playbook document fields reference their canonical composed child owners. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDslValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

export interface PlaybookArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title: string | null;
  /** @state artifact */
  document: ArtifactChild;
  /** @state artifact */
  flow: ArtifactChild;
}

/** 🪪️ Validates exact document fields, including the required nullable native title. */
export function parsePlaybookArtifact(value: unknown, at = "$"): PlaybookArtifact {
  parseDslValue(value);
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a Playbook document`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "id", "version", "title", "document", "flow"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key)) || ["schema", "id", "version"].some((key) => typeof row[key] !== "string") || (row.title !== null && typeof row.title !== "string")) throw new Error(`${at}: invalid Playbook document fields`);
  return { schema: row.schema as string, id: row.id as string, version: row.version as string, title: row.title as string | null, document: parseArtifactChild(row.document), flow: parseArtifactChild(row.flow) };
}
