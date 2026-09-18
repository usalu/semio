/** ℹ️ Direct set-info TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDate, PdfDecimal, PdfDictEntry, PdfInfo, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetInfoMutation {
  mutation: 'setInfo';
  info: PdfInfo;
}
