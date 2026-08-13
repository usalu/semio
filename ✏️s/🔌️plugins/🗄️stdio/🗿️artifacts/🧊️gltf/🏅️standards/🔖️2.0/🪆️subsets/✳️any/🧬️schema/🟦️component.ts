/** 🧬️ GltfArtifact schema — full artifact state (same shape as GltfSnapshot). */
import type { GltfDocument, GltfSourceForm } from './📸️snapshot/🟦️component.ts';

export interface GltfArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ document: GltfDocument;
  /** @state artifact */ buffers: number[][];
  /** @state artifact */ sourceForm: GltfSourceForm;
}
