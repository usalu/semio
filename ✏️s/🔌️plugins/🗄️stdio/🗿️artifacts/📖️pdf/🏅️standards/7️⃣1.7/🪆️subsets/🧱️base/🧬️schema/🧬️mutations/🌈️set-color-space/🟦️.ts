/** 🌈️ Direct set-color-space TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfNamedColorSpace } from '../../📸️snapshot/📄️document/🟦️.ts';
export interface SetColorSpaceMutation {
  mutation: 'setColorSpace';
  colorSpace: PdfNamedColorSpace;
  index?: number | null;
}
