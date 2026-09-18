/** 🔧️ Direct set-object-value TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetObjectValueMutation {
  mutation: 'setObjectValue';
  id: ObjRef;
  value: PdfObject;
}
