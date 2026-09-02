/** 🧬️ DeflateArtifact schema — mirrors DeflateSnapshot's typed RFC1950 fields. */
import type { DeflateLevelHint } from './📸️snapshot/🟦️.ts';

export interface DeflateArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ compressionMethod: number;
  /** @state artifact */ windowBits: number;
  /** @state artifact */ compressionLevelHint: DeflateLevelHint;
  /** @state artifact */ dictId?: number;
  /** @state artifact */ payload: number[];
}
