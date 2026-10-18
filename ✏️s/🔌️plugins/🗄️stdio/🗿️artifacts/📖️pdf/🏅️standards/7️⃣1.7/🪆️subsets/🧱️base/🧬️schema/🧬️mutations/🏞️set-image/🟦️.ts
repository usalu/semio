/** 🏞️ Direct set-image TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfImage, PdfImageBody, PdfImageMask } from '../../📸️snapshot/🖼️resource/🟦️.ts';
export interface SetImageMutation {
  mutation: 'setImage';
  image: PdfImage;
  index?: number | null;
}
