/** ✏️ Direct set-page-content TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🟦️.ts';
export interface SetPageContentMutation {
  mutation: 'setPageContent';
  index: number;
  content: PdfOp[];
}
