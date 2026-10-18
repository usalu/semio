import type { PdfAdmittedStreamRole } from "./🪪️stream-roles/🟦️.ts";
/** 🧬️ PdfArtifact (1.7) schema — full artifact state, same fields as `PdfSnapshot`
 *  (see `📸️snapshot/🟦️.ts`). */
import type { PdfDictEntry } from './📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfInfo } from './📸️snapshot/📇️metadata/🟦️.ts';
import type { PdfIndirectObject, PdfPage } from './📸️snapshot/📄️document/🟦️.ts';

export interface PdfArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ declaredVersion: string;
  /** @state artifact */ pages: PdfPage[];
  /** @state artifact */ info: PdfInfo;
  /** @state artifact */ objects: PdfIndirectObject[];
  /** @state artifact */ trailer: PdfDictEntry[];
  /** @state artifact */ admittedStreamRoles: PdfAdmittedStreamRole[];
}
