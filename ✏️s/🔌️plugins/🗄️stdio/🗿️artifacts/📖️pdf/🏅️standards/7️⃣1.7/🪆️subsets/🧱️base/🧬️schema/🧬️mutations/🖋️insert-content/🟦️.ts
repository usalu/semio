/** 🖋️ Direct insert-content TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🖋️content/🟦️.ts';
import type { PdfLineCap, PdfLineJoin } from "../../📸️snapshot/🖊️line-style/🟦️.ts";
export interface InsertContentMutation {
  mutation: 'insertContent';
  index: number;
  at: number;
  content: PdfOp[];
}
