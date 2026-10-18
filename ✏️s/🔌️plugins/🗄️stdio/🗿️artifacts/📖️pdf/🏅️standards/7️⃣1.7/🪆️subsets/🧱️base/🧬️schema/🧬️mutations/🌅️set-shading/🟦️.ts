/** 🌅️ Direct set-shading TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfShading, PdfShadingKind } from '../../📸️snapshot/🖼️resource/🌈️shading/🟦️.ts';
export interface SetShadingMutation {
  mutation: 'setShading';
  shading: PdfShading;
  index?: number | null;
}
