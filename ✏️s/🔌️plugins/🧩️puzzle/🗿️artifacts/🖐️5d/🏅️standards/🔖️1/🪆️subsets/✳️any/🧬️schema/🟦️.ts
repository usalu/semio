/** 🧬️ Puzzle5d artifact and persisted snapshot share one exact canonical model. */
export * from "./📸️snapshot/🟦️.ts";
import { parsePuzzle5dSnapshot, type Puzzle5dSnapshot } from "./📸️snapshot/🟦️.ts";
export interface Puzzle5dArtifact extends Puzzle5dSnapshot {}
/** 🚪️ Admit the same persisted fields through the artifact public facade. */
export function parsePuzzle5dArtifact(value: unknown, at = "$"): Puzzle5dArtifact { return parsePuzzle5dSnapshot(value, at); }
