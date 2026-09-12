import { parseCurationArtifact, type CurationArtifact } from "../🟦️.ts";
/** 📸 Curation snapshots contain exactly the persisted artifact fields. */
export interface CurationSnapshot extends CurationArtifact {}
/** 🪪 Uses the document owner's exact admission for captured snapshots. */
export function parseCurationSnapshot(value: unknown, at = "$"): CurationSnapshot { return parseCurationArtifact(value, at); }
