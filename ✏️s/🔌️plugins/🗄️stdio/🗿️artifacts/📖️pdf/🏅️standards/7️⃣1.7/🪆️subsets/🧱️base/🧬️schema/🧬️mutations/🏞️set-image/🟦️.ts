/** 🏞️ Direct set-image TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfImage, PdfImageBody, PdfImageMask, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetImageMutation {
  mutation: 'setImage';
  image: PdfImage;
  index?: number | null;
}
