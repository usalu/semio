import type { LineEnding } from './📸️snapshot/🟦️component.ts';

/** 🧬️ TxtArtifact schema. */
export interface TxtArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ lines: string[];
  /** @state persistent */ trailingNewline: boolean;
  /** @state persistent */ lineEnding: LineEnding;
}
