/** 🧬️ PlyArtifact schema — mirrors PlySnapshot's persistent fields exactly. */
import type { PlyElement, PlyFormat } from './📸️snapshot/🟦️component.ts';

export interface PlyArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ format: PlyFormat;
  /** @state artifact */ comments: string[];
  /** @state artifact */ elements: PlyElement[];
}
