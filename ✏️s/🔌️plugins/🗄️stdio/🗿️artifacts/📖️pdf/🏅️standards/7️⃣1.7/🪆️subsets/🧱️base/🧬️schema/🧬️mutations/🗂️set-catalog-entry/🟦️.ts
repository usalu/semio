/** 🗂️ Direct set-catalog-entry TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetCatalogEntryMutation {
  mutation: 'setCatalogEntry';
  key: string;
  value: PdfObject;
}
