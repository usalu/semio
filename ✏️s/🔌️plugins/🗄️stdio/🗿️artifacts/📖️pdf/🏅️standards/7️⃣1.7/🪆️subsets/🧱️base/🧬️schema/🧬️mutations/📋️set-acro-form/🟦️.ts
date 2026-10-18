/** 📋️ Direct set-acro-form TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfAcroForm, PdfFormField, PdfFormFieldKind } from '../../📸️snapshot/📝️form/🟦️.ts';
export interface SetAcroFormMutation {
  mutation: 'setAcroForm';
  form?: PdfAcroForm | null;
}
