/** 🧩️ Direct set-pattern TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfColorSpace, PdfDecimal, PdfDictEntry, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPattern, PdfPatternKind, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🟦️.ts';
export interface SetPatternMutation {
  mutation: 'setPattern';
  pattern: PdfPattern;
}
