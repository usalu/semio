/** 📸️ Jack persisted snapshot with one composed graph-content identity. */
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseJackArtifact, type Camera, type Manifest } from "../🟦️.ts";

export interface JackSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ name: string;
  /** @state artifact */ manifestId?: string;
  /** @state artifact */ manifest: Manifest;
  /** @state artifact */ camera: Camera;
  /** @state artifact @child kind=s.stdio.semio.graph */ content: ArtifactChild;
  /** @state artifact */ rootNodeId?: string;
}

/** 🪪️ Parses the snapshot through the identical persisted artifact contract. */
export function parseJackSnapshot(value: unknown, at = "$"): JackSnapshot {
  return parseJackArtifact(value, at);
}
