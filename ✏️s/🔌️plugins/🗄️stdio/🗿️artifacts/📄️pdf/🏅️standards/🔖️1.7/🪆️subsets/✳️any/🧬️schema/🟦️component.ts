/** 🧬️ PdfArtifact (1.7) schema — full artifact state, same fields as `PdfSnapshot`
 *  (see `📸️snapshot/🟦️component.ts`). */
import type { PdfDictEntry, PdfInfo, PdfIndirectObject, PdfPage } from './📸️snapshot/🟦️component.ts';

export interface PdfArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ declaredVersion: string;
  /** @state persistent */ pages: PdfPage[];
  /** @state persistent */ info: PdfInfo;
  /** @state persistent */ objects: PdfIndirectObject[];
  /** @state persistent */ trailer: PdfDictEntry[];
}
