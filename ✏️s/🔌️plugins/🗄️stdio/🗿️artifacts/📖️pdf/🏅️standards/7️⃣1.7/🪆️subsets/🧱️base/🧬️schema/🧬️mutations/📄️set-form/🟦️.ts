/** 📄️ Direct set-form TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFormXObject, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup } from '../../📸️snapshot/🟦️.ts';
export interface SetFormMutation {
  mutation: 'setForm';
  form: PdfFormXObject;
}
