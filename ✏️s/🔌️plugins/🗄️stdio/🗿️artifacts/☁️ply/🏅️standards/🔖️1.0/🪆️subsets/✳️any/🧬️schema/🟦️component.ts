/** 🧬️ PlyArtifact schema — mirrors PlySnapshot's persistent fields exactly. */
import type { PlyElement, PlyFormat } from './📸️snapshot/🟦️component.ts';

export interface PlyArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ format: PlyFormat;
  /** @state persistent */ comments: string[];
  /** @state persistent */ elements: PlyElement[];
}
