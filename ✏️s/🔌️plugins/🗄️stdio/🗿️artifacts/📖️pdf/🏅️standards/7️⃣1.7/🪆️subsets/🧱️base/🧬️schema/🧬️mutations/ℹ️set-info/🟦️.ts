/** ℹ️ Direct set-info TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfDate, PdfInfo } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetInfoMutation {
  mutation: 'setInfo';
  info: PdfInfo;
}
