/** 🔑️ Direct set-dict-entry TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfPathSegment } from '../../🔺️diff/🟦️.ts';
export interface SetDictEntryMutation {
  mutation: 'setDictEntry';
  id: ObjRef;
  path: PdfPathSegment[];
  key: string;
  value: PdfObject;
  index?: number | null;
}
