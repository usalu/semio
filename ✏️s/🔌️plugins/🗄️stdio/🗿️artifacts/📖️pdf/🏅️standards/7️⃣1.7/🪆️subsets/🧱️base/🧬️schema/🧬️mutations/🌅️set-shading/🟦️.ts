/** 🌅️ Direct set-shading TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfObject, PdfPredictor, PdfShading, PdfShadingKind, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetShadingMutation {
  mutation: 'setShading';
  shading: PdfShading;
}
