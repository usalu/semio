/** 📸️ Procedure snapshot retains only shared document child identities. */
import { parseProcedureArtifact, type ArtifactChild } from "../🟦️.ts";

export interface ProcedureSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio */ flow: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ text: ArtifactChild;
}

/** 🔎️ Validates a snapshot through the exact Procedure document boundary. */
export function parseProcedureSnapshot(value: unknown, at = "$"): ProcedureSnapshot {
  return parseProcedureArtifact(value, at);
}
