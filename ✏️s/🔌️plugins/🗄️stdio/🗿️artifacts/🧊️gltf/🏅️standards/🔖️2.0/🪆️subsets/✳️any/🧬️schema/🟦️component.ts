/** 🧬️ GltfArtifact schema — full artifact state (same shape as GltfSnapshot). */
import type { GltfDocument, GltfSourceForm } from './📸️snapshot/🟦️component.ts';

export interface GltfArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ document: GltfDocument;
  /** @state persistent */ buffers: number[][];
  /** @state persistent */ sourceForm: GltfSourceForm;
}
