/** 🔁️ Direct replace-content TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🟦️.ts';
export interface ReplaceContentMutation {
  mutation: 'replaceContent';
  index: number;
  at: number;
  op: PdfOp;
}
