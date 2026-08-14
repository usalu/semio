/** 🧬️ PdfArtifact (1.7) schema — full artifact state, same fields as `PdfSnapshot`
 *  (see `📸️snapshot/🟦️component.ts`). */
import type { ArtifactSource, PdfDictEntry, PdfInfo, PdfIndirectObject, PdfPage } from './📸️snapshot/🟦️component.ts';

export interface PdfArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ declaredVersion: string;
  /** @state artifact */ pages: PdfPage[];
  /** @state artifact */ info: PdfInfo;
  /** @state artifact */ objects: PdfIndirectObject[];
  /** @state artifact */ trailer: PdfDictEntry[];
  /** @state artifact */ source?: ArtifactSource;
}
