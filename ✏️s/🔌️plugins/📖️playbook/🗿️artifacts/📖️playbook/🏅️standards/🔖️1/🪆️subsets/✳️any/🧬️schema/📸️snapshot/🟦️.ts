/** 📸️ Playbook persisted snapshot shares the document's exact field contract. */
import { parsePlaybookArtifact } from "../🟦️.ts";
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface PlaybookSnapshot {
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

/** 🔎️ Decodes a persisted Playbook document with canonical child identities. */
export function parsePlaybookSnapshot(value: unknown, at = "$"): PlaybookSnapshot {
  return parsePlaybookArtifact(value, at);
}
