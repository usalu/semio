import { parseSemioKitSnapshot, type SemioKitSnapshot } from "./📸️snapshot/🟦️.ts";
export type { ArtifactChild, ArtifactLink, SemioKitConnection, SemioKitDesign, SemioKitPiece, SemioKitType } from "./📸️snapshot/🟦️.ts";

export interface SemioKitArtifact extends SemioKitSnapshot {}

/** 🧰️ Parses the Kit artifact through its exact snapshot-owned domain contract. */
export function parseSemioKitArtifact(value: unknown, at = "$"): SemioKitArtifact {
  return parseSemioKitSnapshot(value, at);
}
