/** 🛠️ Direct set-viewer-preferences TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfPageMode, PdfViewerPreferences } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetViewerPreferencesMutation {
  mutation: 'setViewerPreferences';
  preferences?: PdfViewerPreferences | null;
}
