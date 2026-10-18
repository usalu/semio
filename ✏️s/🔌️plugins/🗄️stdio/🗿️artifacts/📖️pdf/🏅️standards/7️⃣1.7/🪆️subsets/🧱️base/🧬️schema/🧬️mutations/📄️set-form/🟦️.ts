/** 📄️ Direct set-form TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfFormXObject, PdfLineCap, PdfLineJoin, PdfTransparencyGroup } from '../../📸️snapshot/🖼️resource/🎚️state/🟦️.ts';
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🖋️content/🟦️.ts';
export interface SetFormMutation {
  mutation: 'setForm';
  form: PdfFormXObject;
  index?: number | null;
}
