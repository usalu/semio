/** 🛠️ Direct set-viewer-preferences TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPageMode, PdfPredictor, PdfStreamFilter, PdfViewerPreferences } from '../../📸️snapshot/🟦️.ts';
export interface SetViewerPreferencesMutation {
  mutation: 'setViewerPreferences';
  preferences?: PdfViewerPreferences | null;
}
