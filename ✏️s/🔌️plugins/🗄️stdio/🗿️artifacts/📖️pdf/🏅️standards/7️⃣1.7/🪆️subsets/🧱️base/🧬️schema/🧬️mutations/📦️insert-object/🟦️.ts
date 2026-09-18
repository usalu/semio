/** 📦️ Direct insert-object TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface InsertObjectMutation {
  mutation: 'insertObject';
  id: ObjRef;
  value: PdfObject;
}
