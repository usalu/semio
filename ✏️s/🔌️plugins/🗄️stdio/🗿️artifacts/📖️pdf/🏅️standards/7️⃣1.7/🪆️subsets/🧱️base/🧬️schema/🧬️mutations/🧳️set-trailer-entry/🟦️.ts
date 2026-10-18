/** 🧳️ Direct set-trailer-entry TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
export interface SetTrailerEntryMutation {
  mutation: 'setTrailerEntry';
  key: string;
  value: PdfObject;
  index?: number | null;
}
