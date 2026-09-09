/** 📸️ DAG persisted snapshot with one composed graph-content identity. */
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDagArtifact } from "../🟦️.ts";

export interface DagSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio.graph */ content: ArtifactChild;
}

/** 🪪️ Parses the snapshot through the identical persisted artifact contract. */
export function parseDagSnapshot(value: unknown, at = "$"): DagSnapshot {
  return parseDagArtifact(value, at);
}
