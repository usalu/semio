/** 🔁️ Direct replace-content TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🖋️content/🟦️.ts';
import type { PdfLineCap, PdfLineJoin } from '../../📸️snapshot/🖼️resource/🎚️state/🟦️.ts';
export interface ReplaceContentMutation {
  mutation: 'replaceContent';
  index: number;
  at: number;
  op: PdfOp;
}
