/** 📋️ Direct set-acro-form TypeScript payload. */
import type { ObjRef, PdfAcroForm, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfFormField, PdfFormFieldKind, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetAcroFormMutation {
  mutation: 'setAcroForm';
  form?: PdfAcroForm | null;
}
