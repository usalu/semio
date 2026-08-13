import type { LineEnding } from './📸️snapshot/🟦️component.ts';

/** 🧬️ TxtArtifact schema. */
export interface TxtArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ lines: string[];
  /** @state artifact */ trailingNewline: boolean;
  /** @state artifact */ lineEnding: LineEnding;
}
