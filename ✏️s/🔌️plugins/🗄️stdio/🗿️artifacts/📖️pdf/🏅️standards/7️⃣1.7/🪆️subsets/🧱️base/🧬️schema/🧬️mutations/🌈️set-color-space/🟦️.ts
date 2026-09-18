/** 🌈️ Direct set-color-space TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfNamedColorSpace, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetColorSpaceMutation {
  mutation: 'setColorSpace';
  colorSpace: PdfNamedColorSpace;
}
