/** 🧬️ PdfArtifact (1.4) schema — full artifact state, same fields as `PdfSnapshot`. */
import type { PageDoc } from './📸️snapshot/🟦️component.ts';

export interface PdfArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ page: PageDoc;
}
