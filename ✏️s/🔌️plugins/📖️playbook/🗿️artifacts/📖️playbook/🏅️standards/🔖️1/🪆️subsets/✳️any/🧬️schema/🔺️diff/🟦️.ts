/** 🔺️ Playbook sparse document delta references OS Store child identities. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

import { parsePlaybookArtifact, type PlaybookArtifact } from "../🟦️.ts";

export interface PlaybookDiff {
  /** @state artifact */
  artifact?: PlaybookArtifact | null;
  /** @state artifact */
  schema?: string | null;
  /** @state artifact */
  id?: string | null;
  /** @state artifact */
  version?: string | null;
  /** @state artifact */
  title?: string | null;
  /** @state artifact @child kind=s.stdio.semio */
  document?: ArtifactChild | null;
  /** @state artifact @child kind=s.stdio.semio */
  flow?: ArtifactChild | null;
}

/** 🔺️ Decodes the native sparse slots with exact owner and child validation. */
export function parsePlaybookDiff(value: unknown, at = "$"): PlaybookDiff {
  parseDslValue(value);
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a Playbook diff`);
  const row = value as Record<string, unknown>;
  const strings = ["schema", "id", "version", "title"];
  if (Object.keys(row).some((key) => ![...strings, "artifact", "document", "flow"].includes(key)) || strings.some((key) => row[key] != null && typeof row[key] !== "string")) throw new Error(`${at}: invalid Playbook diff fields`);
  return { artifact: row.artifact == null ? null : parsePlaybookArtifact(row.artifact), schema: row.schema as string ?? null, id: row.id as string ?? null, version: row.version as string ?? null, title: row.title as string ?? null, document: row.document == null ? null : parseArtifactChild(row.document), flow: row.flow == null ? null : parseArtifactChild(row.flow) };
}
