/** 🖋️ Direct insert-content TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🟦️.ts';
export interface InsertContentMutation {
  mutation: 'insertContent';
  index: number;
  at: number;
  content: PdfOp[];
}
