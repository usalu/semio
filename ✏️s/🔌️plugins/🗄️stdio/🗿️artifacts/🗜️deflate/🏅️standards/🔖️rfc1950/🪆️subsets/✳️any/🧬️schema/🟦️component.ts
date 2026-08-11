/** 🧬️ DeflateArtifact schema — mirrors DeflateSnapshot's typed RFC1950 fields. */
import type { DeflateLevelHint } from './📸️snapshot/🟦️component.ts';

export interface DeflateArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ compressionMethod: number;
  /** @state persistent */ windowBits: number;
  /** @state persistent */ compressionLevelHint: DeflateLevelHint;
  /** @state persistent */ dictId?: number;
  /** @state persistent */ payload: number[];
}
