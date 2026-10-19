/** 🧩️ Direct set-pattern TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🖋️content/🟦️.ts';
import type { PdfLineCap, PdfLineJoin } from "../../📸️snapshot/🖊️line-style/🟦️.ts";
import type { PdfPattern, PdfPatternKind } from '../../📸️snapshot/🖼️resource/🌈️shading/🟦️.ts';
export interface SetPatternMutation {
  mutation: 'setPattern';
  pattern: PdfPattern;
  index?: number | null;
}
