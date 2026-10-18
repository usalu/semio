/** 👁️ Direct set-optional-content TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfOptionalContent, PdfOptionalContentGroup } from '../../📸️snapshot/📝️form/🟦️.ts';
export interface SetOptionalContentMutation {
  mutation: 'setOptionalContent';
  content?: PdfOptionalContent | null;
}
